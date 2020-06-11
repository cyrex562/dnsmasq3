use tonic::{Request, Response, Status};

use log::{debug, info};
use pt_grpc::prism_tank_grpc::pt_controller_server::{PtController};
use pt_grpc::prism_tank_grpc::{EchoRequest, EchoReply, ResultCode, 
    RegisterRequest, RegisterReply, HeartbeatRequest, HeartbeatReply, UnregisterRequest, UnregisterReply};

use chrono::{Utc};



// todo: add redis connection to struct for use in methods

pub struct PtControllerService {
    // redis_conn: redis::Connection,
    redis_conn_str: String,
}

impl PtControllerService  {
    pub fn new(redis_conn_str: String) -> Self { Self {redis_conn_str}}
}

#[tonic::async_trait]
impl PtController for PtControllerService  {
    async fn echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoReply>, Status> {
        info!("got a request {:?}", request);
        
        let reply = EchoReply {
            value: request.into_inner().value
        };
        Ok(Response::new(reply))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterReply>, Status> {
        info!("got a request {:?}", request);

        let mut redis_conn = match pt_util::init_redis_conn(self.redis_conn_str.clone()) {
            Ok(v) => v,
            Err(e) => {
                let msg: String = format!("failed to connect to redis: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg));
            }

        };

        // TODO: check if host_agent_id is already in the list of agent_ids, return a warning if it is

        let inner_msg = request.into_inner();
        let key = format!("pt:ha_tracker:host_agents:{}", inner_msg.host_agent_id);
        let hm_res: redis::RedisResult<String>; 
        hm_res = redis::cmd("HMSET")
                  .arg(key)
                  .arg(&[(String::from("host_agent_address"),  inner_msg.host_agent_address),
                  (String::from("time_last_seen"), Utc::now().timestamp().to_string()),
                  (String::from("state"), "alive".into())])
                  .query(&mut redis_conn);
        match hm_res {
            Ok(v) => debug!("hmset call succeeded: {:?}", v),
            Err(e) => {
                let msg: String = format!("hmset command failed: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg))
            },
        };


        let lpush_res: redis::RedisResult<i32>;
        lpush_res = redis::cmd("LPUSH")
            .arg("pt:ha_tracker:host_agent_ids")
            .arg(inner_msg.host_agent_id)
            .query(&mut redis_conn);
        match lpush_res {
            Ok(v) => {
                debug!("lpush call succeeded: {:?}", v);
                Ok(Response::new(RegisterReply{result: ResultCode::Ok as i32}))
            },
            Err(e) => Err(tonic::Status::new(tonic::Code::Internal, format!("lpush command failed: {}", e)))
        }

    }

    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatReply>, Status> {
        info!("got a request {:?}", request);
        // todo: use heartbeat message to update the status of known host agents

        let mut redis_conn = match pt_util::init_redis_conn(self.redis_conn_str.clone()) {
            Ok(v) => v,
            Err(e) => {
                let msg: String = format!("failed to connect to redis: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg));
            }

        };
        
        let inner_msg = request.into_inner();
        let key = format!("pt:ha_tracker:host_agents:{}", inner_msg.host_agent_id);

        let key_exists = match pt_util::redis_key_exists(&mut redis_conn, key.clone()) {
            Ok(v) => v,
            Err(e) => {
                let msg: String = format!("key exists check failed: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg))
            }
        };

        if !key_exists {
            let msg: String = format!("key {} does not exist in redis db", key);
            return Err(tonic::Status::new(tonic::Code::NotFound, msg));
        }

        let curr_time = Utc::now().timestamp().to_string();
        let hm_res: redis::RedisResult<String>;
        
        hm_res = redis::cmd("HMSET")
           .arg(key)
           .arg(&[
               (String::from("time_last_seen"), curr_time),
               (String::from("state"), "alive".into())
           ])
           .query(&mut redis_conn);
        match hm_res {
            Ok(v) => {
                debug!("hset call succeeded: {:?}", v);
                Ok(Response::new(HeartbeatReply{result: ResultCode::Ok as i32}))
            },
            Err(e) => {
                let msg: String = format!("hset command failed: {}", e);
                Err(tonic::Status::new(tonic::Code::Internal, msg))
            }
        }
            
        // todo: get all host agent ids from list. retrieve the host agent object for each one, compare it to the stale time, and if exceeded, mark it as stale. push the host agent ids back onto the list.
    }

    async fn unregister(
        &self,
        request: Request<UnregisterRequest>,
    ) -> Result<Response<UnregisterReply>, Status> {
        info!("got a request: {:?}", request);

        let mut redis_conn = match pt_util::init_redis_conn(self.redis_conn_str.clone()) {
            Ok(v) => v,
            Err(e) => {
                let msg: String = format!("failed to connect to redis: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg));
            }
        };
        
        let inner_msg = request.into_inner();
        let key = format!("pt:ha_tracker:host_agents:{}", inner_msg.host_agent_id);

        let key_exists = match pt_util::redis_key_exists(&mut redis_conn, key.clone()) {
            Ok(v) => v,
            Err(e) => {
                let msg: String = format!("key exists check failed: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg))
            }
        };

        if !key_exists {
            let msg: String = format!("key \"{}\" does not exist in redis db", key);
            return Err(tonic::Status::new(tonic::Code::NotFound, msg));
        }

        match pt_util::delete_redis_key(&mut redis_conn, key) {
            Ok(_v) => debug!("HA object removed from list"),
            Err(e) => {
                let msg: String = format!("failed to delete redis key: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg))
            }
        }

        let lrem_res: redis::RedisResult<i32>;
        lrem_res = redis::cmd("LREM").arg("pt:ha_tracker:host_agent_ids").arg(-1).arg(inner_msg.host_agent_id).query(&mut redis_conn);
        match lrem_res {
            Ok(v) => {
                debug!("remove from listed succeeded: {:?}", v);
                Ok(Response::new(UnregisterReply{result: ResultCode::Ok as i32}))},
            Err(e) => {
                Err(tonic::Status::new(tonic::Code::Internal, format!("failed to remove host agent id from list of ids: {}", e)))
            },
        }

        // Ok(Response::new(UnregisterReply{result: ResultCode::Ok as i32}))
    }
}