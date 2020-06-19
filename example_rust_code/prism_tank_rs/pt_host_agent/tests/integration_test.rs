/// Host Agent Integration Tests
use ::function_name::named;
use dotenv::dotenv;
use pt_grpc::prism_tank_grpc::{
    pt_host_agent_server::PtHostAgentServer, EchoRequest, GetHostnameReply, GetHostnameRequest,
};
use pt_host_agent::{client::HostAgentClient, pt_host_agent_service::PtHostAgentService};
use pt_util::{get_env_var_str, get_env_var_u64};
use std::{net::SocketAddr, time};
use tokio::runtime::Runtime;
use tonic::transport::Server;

fn create_host_agent_svc(rt: &Runtime,
                         address: &str,
                         port: &str)
                         -> Result<(), String> {
    let host_agent_addr = match format!("{}:{}", address, port).parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => {
            return Err(format!("failed to convert host agent listen address to \
                                SocketAddr: {}",
                               e))
        }
    };

    let host_agent = PtHostAgentService::new();

    let host_agent_svc = Server::builder().add_service(PtHostAgentServer::new(host_agent))
                                          .serve(host_agent_addr);
    rt.spawn(host_agent_svc);
    Ok(())
}

#[test]
#[named]
/// test basic echo function
fn test_host_agent_echo() -> Result<(), String> {
    dotenv().ok();
    let grpc_addr_str = get_env_var_str("GRPC_ADDR")?;
    let grpc_port_str = get_env_var_str("GRPC_PORT")?;
    let shutdown_wait: u64 = get_env_var_u64("SHUTDOWN_WAIT")?;

    // setup runtime
    let rt = Runtime::new().expect("failed to obtain a new runtime object");

    // create and start the host agent
    match create_host_agent_svc(&rt, grpc_addr_str.as_str(), grpc_port_str.as_str()) {
        Ok(_v) => println!("{}: host agent service created", function_name!()),
        Err(e) => return Err(format!("failed to create host agent service: {}", e)),
    };

    // send echo to host agent
    let echo_val: String = "test".into();
    let mut client =
        HostAgentClient::connect(format!("http://{}:{}", grpc_addr_str, grpc_port_str))?;
    let request = tonic::Request::new(EchoRequest { value: echo_val.clone() });
    let response = match client.echo(request) {
        Ok(v) => v,
        Err(e) => return Err(format!("echo request failed: {:?}", e)),
    };
    println!("{}: full response: {:?}", function_name!(), response);
    let response_inner = response.into_inner();
    println!("{}: response message body: {:?}",
             function_name!(),
             response_inner);

    // verify received
    assert_eq!(echo_val, response_inner.value);

    // cleanup
    rt.shutdown_timeout(time::Duration::from_secs(shutdown_wait));

    Ok(())
}

#[test]
#[named]
fn test_get_hostname() -> Result<(), String> {
    dotenv().ok();
    let grpc_addr_str = get_env_var_str("GRPC_ADDR")?;
    let grpc_port_str = get_env_var_str("GRPC_PORT")?;
    let shutdown_wait: u64 = get_env_var_u64("SHUTDOWN_WAIT")?;

    // setup runtime
    let rt = Runtime::new().expect("failed to obtain a new runtime object");

    // create and start the host agent
    match create_host_agent_svc(&rt, grpc_addr_str.as_str(), grpc_port_str.as_str()) {
        Ok(_v) => println!("{}: host agent service created", function_name!()),
        Err(e) => return Err(format!("failed to create host agent service: {}", e)),
    };

    let mut client: HostAgentClient =
        HostAgentClient::connect(format!("http://{}:{}", grpc_addr_str, grpc_port_str))?;
    let request: tonic::Request<GetHostnameRequest> = tonic::Request::new(GetHostnameRequest {});
    let response: tonic::Response<GetHostnameReply> = match client.get_hostname(request) {
        Ok(v) => v,
        Err(e) => return Err(format!("get hostname request failed: {:?}", e)),
    };
    println!("{}: full response: {:?}", function_name!(), response);
    // let meta = response.metadata();
    let http_headers = response.metadata().clone().into_headers();
    println!("{}: http headers: {:?}", function_name!(), http_headers);
    let status = http_headers.get("grpc-status").unwrap();
    assert_eq!(status, "0");

    let response_inner = response.into_inner();
    println!("{}: response message body: {:?}",
             function_name!(),
             response_inner);
    assert!(response_inner.hostname.contains("Static hostname"));

    rt.shutdown_timeout(time::Duration::from_secs(shutdown_wait));

    Ok(())
}
