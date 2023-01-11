use libc::{c_char, IF_NAMESIZE};

#[derive(Default, Debug, Clone)]
pub struct DhcpBridge {
    pub iface:String,
    // char iface[IF_NAMESIZE];
    // struct dhcp_bridge *alias, *next;
}
