use ::function_name::named;
use dotenv::dotenv;
use pt_controller::{client::ControllerClient, pt_controller_service::PtControllerService};
use pt_grpc::prism_tank_grpc::{
    pt_controller_server::PtControllerServer, EchoRequest, HeartbeatRequest, RegisterRequest,
    ResultCode, UnregisterRequest,
};


use pt_util::{get_env_var_str, get_env_var_u64, time_secs};
use std::{net::SocketAddr, time};
use tokio::runtime::Runtime;
use tonic::{self, transport::Server, Request};


// static ADDRESS: &str = "127.0.0.1";
// static PORT: &str = "50002";
// static REDIS_CONN_STR: &str = "redis://127.0.0.1:6379";
// static REDIS_CONN_STR: &str = "redis://10.0.0.19:6379";
// static SHUTDOWN_WAIT: time::Duration = time::Duration::from_secs(1);

fn create_controller_svc(rt: &Runtime,
                         address: &str,
                         port: &str,
                         redis_conn_str: &str)
                         -> Result<(), String> {
    let controller_addr: SocketAddr = match format!("{}:{}", address, port).parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => {
            return Err(format!("failed to convert controller listen address \
                                string to SocketAddr: {}",
                               e))
        }
    };

    let controller = PtControllerService::new(redis_conn_str.to_string());
    let controller_svc = Server::builder().add_service(PtControllerServer::new(controller))
                                          .serve(controller_addr);
    let _join_handle = rt.spawn(controller_svc);
    Ok(())
}

#[test]
#[named]
fn test_controller_echo() -> Result<(), String> {
    dotenv().ok();
    let grpc_addr_str = get_env_var_str("GRPC_ADDR")?;
    let grpc_port_str = get_env_var_str("GRPC_PORT")?;
    let shutdown_wait = get_env_var_u64("SHUTDOWN_WAIT")?;
    let redis_conn_str = get_env_var_str("REDIS_CONN_STR")?;
    dotenv().ok();
    let rt = Runtime::new().expect("failed to obtain a new runtime object");
    let echo_val: String = "test".into();
    match create_controller_svc(&rt,
                                grpc_addr_str.as_str(),
                                grpc_port_str.as_str(),
                                redis_conn_str.as_str())
    {
        Ok(_v) => println!("{}: controller service created", function_name!()),
        Err(e) => return Err(format!("failed to create controller service: {}", e)),
    };

    let mut client =
        ControllerClient::connect(format!("http://{}:{}", grpc_addr_str, grpc_port_str))?;

    let request = tonic::Request::new(EchoRequest { value: echo_val.clone() });
    let response = match client.echo(request) {
        Ok(v) => v,
        Err(e) => return Err(format!("echo request failed: {:?}", e)),
    };

    println!("{}: full response: {:?}", function_name!(), response);
    let response_inner = response.into_inner();
    assert_eq!(echo_val, response_inner.value);
    rt.shutdown_timeout(time::Duration::from_secs(shutdown_wait));
    Ok(())
}

#[test]
#[named]
fn test_controller_register() -> Result<(), String> {
    dotenv().ok();
    let grpc_addr_str = get_env_var_str("GRPC_ADDR")?;
    let grpc_port_str = get_env_var_str("GRPC_PORT")?;
    let shutdown_wait = get_env_var_u64("SHUTDOWN_WAIT")?;
    let redis_conn_str = get_env_var_str("REDIS_CONN_STR")?;

    let rt = Runtime::new().expect("failed to obtain a new runtime object");
    match create_controller_svc(&rt,
                                grpc_addr_str.as_str(),
                                grpc_port_str.as_str(),
                                redis_conn_str.as_str())
    {
        Ok(_v) => println!("{}: controller service created", function_name!()),
        Err(e) => return Err(format!("failed to create controller service: {}", e)),
    };

    let mut client =
        ControllerClient::connect(format!("http://{}:{}", grpc_addr_str, grpc_port_str))?;

    let request = tonic::Request::new(RegisterRequest { host_agent_id: "test".into(),
                                                        host_agent_address: "test".into() });
    let response = match client.register(request) {
        Ok(v) => v,
        Err(e) => return Err(format!("register request failed: {:?}", e)),
    };
    println!("{}: full response: {:?}", function_name!(), response);
    let http_headers = response.metadata().clone().into_headers();
    println!("{}: http headers: {:?}", function_name!(), http_headers);
    let status = http_headers.get("grpc-status").unwrap();
    assert_eq!(status, "0");

    let response_inner = response.into_inner();
    println!("{}: response result: {:?}",
             function_name!(),
             response_inner.result);
    assert_eq!(response_inner.result, ResultCode::Ok as i32);

    let mut redis_conn = pt_util::init_redis_conn(redis_conn_str)?;
    pt_util::delete_redis_key(&mut redis_conn,
                              format!("pt:ha_tracker:host_agents:{}", "test"))?;

    rt.shutdown_timeout(time_secs!(shutdown_wait));

    Ok(())
}

#[test]
#[named]
fn test_controller_unregister() -> Result<(), String> {
    dotenv().ok();
    let grpc_addr_str = get_env_var_str("GRPC_ADDR")?;
    let grpc_port_str = get_env_var_str("GRPC_PORT")?;
    let shutdown_wait = get_env_var_u64("SHUTDOWN_WAIT")?;
    let redis_conn_str = get_env_var_str("REDIS_CONN_STR")?;

    let rt = Runtime::new().expect("failed to obtain a new runtime object");
    match create_controller_svc(&rt,
                                grpc_addr_str.as_str(),
                                grpc_port_str.as_str(),
                                redis_conn_str.as_str())
    {
        Ok(_v) => println!("{}: controller service created", function_name!()),
        Err(e) => return Err(format!("failed to create controller service: {}", e)),
    };

    let mut client =
        ControllerClient::connect(format!("http://{}:{}", grpc_addr_str, grpc_port_str))?;

    let reg_request = tonic::Request::new(RegisterRequest { host_agent_id: "test".into(),
                                                            host_agent_address: "test".into() });
    let reg_response = match client.register(reg_request) {
        Ok(v) => v,
        Err(e) => return Err(format!("register request failed: {:?}", e)),
    };
    println!("{}: register response: {:?}",
             function_name!(),
             reg_response);

    let unreg_request = tonic::Request::new(UnregisterRequest { host_agent_id: "test".into() });
    let unreg_response = match client.unregister(unreg_request) {
        Ok(v) => v,
        Err(e) => return Err(format!("unregister request failed: {:?}", e)),
    };

    println!("{}: unregister response: {:?}",
             function_name!(),
             unreg_response);
    let response_inner = unreg_response.into_inner();
    assert_eq!(response_inner.result, ResultCode::Ok as i32);

    rt.shutdown_timeout(time_secs!(shutdown_wait));

    Ok(())
}

#[test]
#[named]
fn test_controller_heartbeat() -> Result<(), String> {
    dotenv().ok();
    let grpc_addr_str = get_env_var_str("GRPC_ADDR")?;
    let grpc_port_str = get_env_var_str("GRPC_PORT")?;
    let shutdown_wait = get_env_var_u64("SHUTDOWN_WAIT")?;
    let redis_conn_str = get_env_var_str("REDIS_CONN_STR")?;
    let rt: Runtime = Runtime::new().expect("failed to obtain a new runtime object");
    match create_controller_svc(&rt,
                                grpc_addr_str.as_str(),
                                grpc_port_str.as_str(),
                                redis_conn_str.as_str())
    {
        Ok(_v) => println!("controller service created"),
        Err(e) => return Err(format!("failed to create controller service: {}", e)),
    };

    let mut client: ControllerClient =
        ControllerClient::connect(format!("http://{}:{}", grpc_addr_str, grpc_port_str))?;

    let reg_request = tonic::Request::new(RegisterRequest { host_agent_id: "test".into(),
                                                            host_agent_address: "test".into() });
    let reg_response = match client.register(reg_request) {
        Ok(v) => v,
        Err(e) => return Err(format!("register request failed: {:?}", e)),
    };
    println!("{}: register response: {:?}",
             function_name!(),
             reg_response);

    let hb_request: Request<HeartbeatRequest> =
        tonic::Request::new(HeartbeatRequest { host_agent_id: "test".into() });
    let hb_response = match client.heartbeat(hb_request) {
        Ok(v) => v,
        Err(e) => return Err(format!("heartbeat request failed: {:?}", e)),
    };

    println!("{}: heartbeat response: {:?}",
             function_name!(),
             hb_response);
    let response_inner = hb_response.into_inner();
    assert_eq!(response_inner.result, ResultCode::Ok as i32);

    let unreg_request = tonic::Request::new(UnregisterRequest { host_agent_id: "test".into() });
    let unreg_response = match client.unregister(unreg_request) {
        Ok(v) => v,
        Err(e) => return Err(format!("unregister request failed: {:?}", e)),
    };

    println!("{}: unregister response: {:?}",
             function_name!(),
             unreg_response);
    let response_inner = unreg_response.into_inner();
    assert_eq!(response_inner.result, ResultCode::Ok as i32);

    rt.shutdown_timeout(time_secs!(shutdown_wait));

    Ok(())
}
