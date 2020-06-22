mod util;
mod dnsmasq_h;
mod config;
mod ip6addr;
mod metrics;
mod dns_protocol;
mod dhcp_protocol;
mod dnsmasq_sys;
mod dbus;
mod dubs_defs;
mod dhcp6_protocol;
mod radv_protocol;

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
