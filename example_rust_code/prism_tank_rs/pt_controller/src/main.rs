mod pt_controller_service;


use log::info;
use pt_controller_service::PtControllerService;
use pt_grpc::prism_tank_grpc::pt_controller_server::PtControllerServer;

use std::{env, net::SocketAddr, time::Duration};
use tokio::runtime;

use tonic::transport::Server;

fn main() {
    match pt_util::init_logger() {
        Ok(val) => val,
        Err(e) => panic!("failed to init logger: {}", e),
    };

    info!("initializing");

    let grpc_listen_addr = match env::var("GRPC_LISTEN_ADDR") {
        Ok(val) => val,
        Err(e) => {
            panic!("failed to get grpc listen addr from env var {}: {}",
                   "GRPC_LISTEN_ADDR", e)
        }
    };

    let addr: SocketAddr = match grpc_listen_addr.parse() {
        Ok(val) => val,
        Err(e) => {
            panic!("failed to parse address port combination specified in env var \"{}\": {}",
                   grpc_listen_addr, e)
        }
    };

    let redis_conn_str = match pt_util::get_redis_conn_str() {
        Ok(val) => val,
        Err(e) => panic!("failed to get redis connection string: {}", e),
    };

    let tracker_svc = PtControllerService::new(redis_conn_str);

    let rt = match runtime::Builder::new().threaded_scheduler()
                                          .max_threads(4)
                                          .build()
    {
        Ok(val) => val,
        Err(e) => panic!("failed to create runtime: {}", e),
    };
    let server_future = Server::builder().add_service(PtControllerServer::new(tracker_svc))
                                         .serve(addr);
    info!("starting grpc command listener");
    let _join_handle = rt.spawn(server_future);

    info!("waiting to shut down");
    rt.shutdown_timeout(Duration::from_secs(5));
}
