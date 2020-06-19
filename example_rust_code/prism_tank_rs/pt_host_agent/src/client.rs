use pt_grpc::prism_tank_grpc::pt_host_agent_client::PtHostAgentClient;
use pt_grpc::prism_tank_grpc::{EchoReply, EchoRequest, GetHostnameReply, GetHostnameRequest};
use tokio::runtime::{Builder, Runtime};

pub struct HostAgentClient {
    client: PtHostAgentClient<tonic::transport::Channel>,
    rt: Runtime,
}

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

impl HostAgentClient {
    pub fn connect<D>(dst: D) -> Result<Self, String>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let mut rt = Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap();

        let client = match rt.block_on(PtHostAgentClient::connect(dst)) {
            Ok(val) => val,
            Err(e) => return Err(format!("failed to setup client: {}", e)),
        };
        Ok(HostAgentClient { rt, client })
    }

    pub fn echo(
        &mut self,
        request: impl tonic::IntoRequest<EchoRequest>,
    ) -> Result<tonic::Response<EchoReply>, tonic::Status> {
        self.rt.block_on(self.client.echo(request))
    }

    pub fn get_hostname(
        &mut self,
        request: impl tonic::IntoRequest<GetHostnameRequest>,
    ) -> Result<tonic::Response<GetHostnameReply>, tonic::Status> {
        self.rt.block_on(self.client.get_hostname(request))
    }
}
