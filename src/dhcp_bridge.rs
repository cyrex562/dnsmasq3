use libc::{c_char, IF_NAMESIZE};

#[derive(Default, Debug, Clone)]
pub struct dhcp_bridge {
    pub iface: [c_char; IF_NAMESIZE],
    // char iface[IF_NAMESIZE];
    // struct dhcp_bridge *alias, *next;
}