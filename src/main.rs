mod util;
mod defines;
mod dnsmasq_h;
mod config;
mod ip6addr;
mod metrics;
mod dns_protocol;
mod dhcp_protocol;
mod dnsmasq_sys;
mod dhcp6_protocol;
mod radv_protocol;

use log::{debug, error, info};



fn main() {
    match util::init_logger() {
        Ok(()) => (),
        Err(e) => panic!("failed to initialize logger: {:?}", e)
    };
}
