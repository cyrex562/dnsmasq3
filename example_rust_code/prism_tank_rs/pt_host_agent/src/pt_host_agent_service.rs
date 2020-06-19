mod hostname;

use tonic::{Request, Response, Status};
use log::{info};
use pt_grpc::prism_tank_grpc::pt_host_agent_server::{PtHostAgent};
use pt_grpc::prism_tank_grpc::{EchoRequest, EchoReply, GetHostnameRequest, GetHostnameReply, ResultCode};


pub struct PtHostAgentService {

}

impl PtHostAgentService {
    pub fn new() -> Self { Self {  } }
}


#[tonic::async_trait]
impl PtHostAgent for PtHostAgentService {
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

    async fn get_hostname(
        &self,
        request: Request<GetHostnameRequest>,
    ) -> Result<Response<GetHostnameReply>, Status> {
        info!("got a request {:?}", request);

        let hostname = match hostname::get_hostname() {
            Ok(v) => v,
            Err(e) => {
                let msg = format!("failed to get hostname: {}", e);
                return Err(tonic::Status::new(tonic::Code::Internal, msg))
            }
        };

        let reply = GetHostnameReply {
            hostname,
            result: ResultCode::Ok as i32
        };
        Ok(Response::new(reply))
    }
}