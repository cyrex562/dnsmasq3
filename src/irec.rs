use libc::c_char;
use crate::mysockaddr::mysockaddr;

#[derive(Default, Debug, Clone)]
pub struct irec {
    // union mysockaddr addr;
    pub addr: mysockaddr,
    pub netmask: in_addr,
    /* only valid for IPv4 */
    // int tftp_ok, dhcp_ok, mtu, done, warned, dad, dns_auth, index, multicast_done, found, label;
    pub tftp_ok: i32,
    pub dhcp_ok: i32,
    pub mtu: i32,
    pub done: i32,
    pub warned: i32,
    pub dad: i32,
    pub dns_auth: i32,
    pub index: i32,
    pub multicast_done: i32,
    pub found: i32,
    pub label: i32,
    // char *name;
    pub name: *mut c_char,
    // struct irec *next;
}
