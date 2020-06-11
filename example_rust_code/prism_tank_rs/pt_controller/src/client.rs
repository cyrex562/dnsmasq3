use pt_grpc::prism_tank_grpc::pt_controller_client::PtControllerClient;
use pt_grpc::prism_tank_grpc::{EchoRequest, EchoReply,  RegisterReply, RegisterRequest, UnregisterRequest, UnregisterReply, HeartbeatRequest, HeartbeatReply};
use tokio::runtime::{Runtime, Builder};

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;


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
            let mut rt = match Builder::new()
                .basic_scheduler()
                .enable_all()
                .build() {
                    Ok(val) => val,
                    Err(e) => return Err(format!("failed to create client builder: {:?}", e)),
                };
            
            let client = match rt.block_on(PtControllerClient::connect(dst)) {
                Ok(val) => val,
                Err(e) => return Err(format!("failed to connect to Controller Server: {:?}", e))
            };
            Ok(ControllerClient{rt, client})
        }
    pub fn echo(
        &mut self,
        request: impl tonic::IntoRequest<EchoRequest>,
    )-> Result<tonic::Response<EchoReply>, tonic::Status> {
        self.rt.block_on(self.client.echo(request))
    }

    pub fn register(
        &mut self,
        request: impl tonic::IntoRequest<RegisterRequest>,
    )-> Result<tonic::Response<RegisterReply>, tonic::Status> {
        self.rt.block_on(self.client.register(request))
    }

    pub fn unregister(
        &mut self,
        request: impl tonic::IntoRequest<UnregisterRequest>,
    )-> Result<tonic::Response<UnregisterReply>, tonic::Status> {
        self.rt.block_on(self.client.unregister(request))
    }

    pub fn heartbeat(
        &mut self,
        request: impl tonic::IntoRequest<HeartbeatRequest>,
    )-> Result<tonic::Response<HeartbeatReply>, tonic::Status> {
        self.rt.block_on(self.client.heartbeat(request))
    }
}