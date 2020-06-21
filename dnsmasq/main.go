mod util;
mod dnsmasq_h;

use std::fmt::Debug;
use log::{debug, error, info};
use std::fmt;


fn main() {
    match util::init_logger {
        Ok(_val) => println!("logger intialized"),
        Err(e) => panic!(
            "failed to init logger: {}", e),

    }
}
