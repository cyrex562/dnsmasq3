mod pt_host_agent_service;

use log::{debug, error, info};
use pt_grpc::prism_tank_grpc::pt_controller_client::PtControllerClient;
use pt_grpc::prism_tank_grpc::pt_host_agent_server::PtHostAgentServer;
use pt_grpc::prism_tank_grpc::{
    HeartbeatReply, HeartbeatRequest, RegisterReply, RegisterRequest, UnregisterReply,
    UnregisterRequest,
};
use pt_host_agent_service::PtHostAgentService;

use std::sync::mpsc::channel;
use std::time::Duration;
use std::{env, net::SocketAddr, thread};
use tokio::runtime;
use tokio::runtime::{Builder, Runtime};
use tonic::transport::Server;
use uuid::Uuid;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

static HEARTBEAT_INTERVAL_VAR: &str = "HEARTBEAT_INTVL";
static GRPC_LISTEN_ADDR_VAR: &str = "GRPC_LISTEN_ADDR";
static TRACKER_CONN_STR_VAR: &str = "TRACKER_CONN_STR";

pub struct ControllerClient {
    client: PtControllerClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl ControllerClient {
    pub fn connect<D>(dst: D) -> Result<Self, String>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let mut rt = match Builder::new().basic_scheduler().enable_all().build() {
            Ok(val) => val,
            Err(e) => return Err(format!("failed to create runtime: {}", e)),
        };
        let client = match rt.block_on(PtControllerClient::connect(dst)) {
            Ok(val) => val,
            Err(e) => return Err(format!("failed to start client: {}", e)),
        };

        Ok(ControllerClient { rt, client })
    }

    pub fn heartbeat(
        &mut self,
        request: impl tonic::IntoRequest<HeartbeatRequest>,
    ) -> Result<tonic::Response<HeartbeatReply>, tonic::Status> {
        self.rt.block_on(self.client.heartbeat(request))
    }

    pub fn register(
        &mut self,
        request: impl tonic::IntoRequest<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterReply>, tonic::Status> {
        self.rt.block_on(self.client.register(request))
    }

    pub fn unregister(
        &mut self,
        request: impl tonic::IntoRequest<UnregisterRequest>,
    ) -> Result<tonic::Response<UnregisterReply>, tonic::Status> {
        self.rt.block_on(self.client.unregister(request))
    }
}

/// main method
fn main() {
    match pt_util::init_logger() {
        Ok(_val) => println!("logger initialized"),
        Err(e) => panic!("failed to init logger: {}", e),
    };

    info!("initializing");

    let host_agent_uuid = Uuid::new_v4();
    let host_agent_id_str: String = host_agent_uuid.to_hyphenated().to_string();
    info!("host agent ID: {}", host_agent_id_str);

    // get and parse tracker server address
    let tracker_conn_str = match env::var(TRACKER_CONN_STR_VAR) {
        Ok(val) => val,
        Err(e) => panic!(
            "failed to get tracker server conn str env var: {}: {}",
            TRACKER_CONN_STR_VAR, e
        ),
    };

    let child_tracker_conn_str = tracker_conn_str.clone();

    // get and parse heartbeat interval
    let heartbeat_intvl_str = match env::var(HEARTBEAT_INTERVAL_VAR) {
        Ok(val) => val,
        Err(e) => panic!(
            "failed to get heartbeat interval env var {}: {}",
            HEARTBEAT_INTERVAL_VAR, e
        ),
    };
    let heartbeat_intvl: u64 = heartbeat_intvl_str.parse::<u64>().unwrap();

    let grpc_listen_addr = match env::var(GRPC_LISTEN_ADDR_VAR) {
        Ok(val) => val,
        Err(e) => panic!(
            "failed to get grpc listen addr from env var {}: {}",
            GRPC_LISTEN_ADDR_VAR, e
        ),
    };

    let listen_addr: SocketAddr = match grpc_listen_addr.parse() {
        Ok(val) => val,
        Err(e) => panic!(
            "failed to parse address port combination specified in env var \"{}\": {}",
            grpc_listen_addr, e
        ),
    };

    // init redis
    // let redis_conn_str = match pt_util::get_redis_conn_str() {
    //     Ok(val) => val,
    //     Err(e) => panic!("failed to get redis connection string: {}", e),
    // };

    // let redis_conn = match pt_util::init_redis_conn(redis_conn_str) {
    //     Ok(val) => val,
    //     Err(e) => panic!("failed to init connection to redis: {}", e),
    // };

    // init service
    let pt_host_agent = PtHostAgentService::new();
    let rt = match runtime::Builder::new()
        .threaded_scheduler()
        .max_threads(4)
        .build()
    {
        Ok(val) => val,
        Err(e) => panic!("failed to create runtime: {}", e),
    };
    let server_future = Server::builder()
        .add_service(PtHostAgentServer::new(pt_host_agent))
        .serve(listen_addr);
    info!("starting grpc command listener");
    let _join_handle = rt.spawn(server_future);

    let (hb_exit_tx, hb_exit_rx) = channel::<bool>();

    // register with tracker
    info!("registering with tracker at {}", tracker_conn_str);
    // setup rpc client
    let mut client = match ControllerClient::connect(tracker_conn_str) {
        Ok(val) => val,
        Err(e) => panic!(format!("failed to connect to tracker server: {}", e)),
    };
    let reg_msg = RegisterRequest {
        host_agent_id: host_agent_id_str.clone(),
        host_agent_address: grpc_listen_addr,
    };
    let request = tonic::Request::new(reg_msg);

    let response = match client.register(request) {
        Ok(val) => val,
        Err(e) => panic!(format!("failed to register with tracker: {}", e)),
    };
    debug!("registration response: {:?}", response);

    let host_id_copy = host_agent_id_str.clone();

    // init heartbeat thread
    let heartbeat_child = thread::spawn(move || -> Result<(), String> {
        let mut child_client = match ControllerClient::connect(child_tracker_conn_str) {
            Ok(val) => val,
            Err(e) => return Err(format!("failed to connect to tracker server: {}", e)),
        };

        loop {
            // check for exit flag on channel
            let exit_flag = hb_exit_rx.try_recv().unwrap();
            if exit_flag {
                debug!("exit flag is true, exiting thread");
                return Ok(());
            }

            // send heartbeat RPC call
            debug!("sending heartbeat");
            let request = tonic::Request::new(HeartbeatRequest {
                host_agent_id: host_agent_id_str.clone(),
            });
            let response = match child_client.heartbeat(request) {
                Ok(val) => val,
                Err(e) => return Err(format!("request to tracker server failed: {}", e)),
            };

            debug!("heartbeat response: {:?}", response);

            // sleep between heartbeats
            debug!("sleeping between heartbeats");
            thread::sleep(Duration::from_secs(heartbeat_intvl));
        }
    });

    info!("waiting to shut down");
    rt.shutdown_timeout(Duration::from_secs(5));

    match hb_exit_tx.send(true) {
        Ok(v) => debug!("exit signal sent: {:?}", v),
        Err(e) => error!("failed to send exit signal: {:?}", e),
    };

    let _res = heartbeat_child.join();

    info!("un-registering with tracker");

    let response = match client.unregister(tonic::Request::new(UnregisterRequest {
        host_agent_id: host_id_copy,
    })) {
        Ok(val) => val,
        Err(e) => panic!(format!("failed to unregister with tracker: {}", e)),
    };
    debug!("un-registration response: {:?}", response);

    info!("exiting");
}

// END OF FILE
