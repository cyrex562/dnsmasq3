mod util;
mod dnsmasq_h;

// #include "config.h"
mod config;
// #include "ip6addr.h"
mod ip6addr;
// #include "metrics.h"
mod metrics;
// #include "dns-protocol.h"
mod dns_protocol;
// #include "dhcp-protocol.h"
mod dhcp_protocol;
// #include "dnsmasq_sys.h"
mod dnsmasq_sys;

// #ifdef HAVE_DHCP6

// #include "dhcp6-protocol.h"
mod dhcp6_protocol;
// #include "radv-protocol.h"
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
