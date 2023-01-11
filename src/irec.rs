use std::net::IpAddr;
use libc::c_char;
use crate::mysockaddr::MySockAddr;

#[derive(Default, Debug, Clone)]
pub struct Irec {
    // union mysockaddr addr;
    pub addr: IpAddr,
    pub netmask: IpAddr,
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
    pub name: String,
    // struct irec *next;
}
