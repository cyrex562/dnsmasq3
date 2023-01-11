use libc::c_char;
use crate::addrlist::AddrList;

#[derive(Default, Debug, Clone)]
pub struct AuthNameList {
    pub name: String,
    pub flags: i32,
    // struct auth_name_list *next;
}

#[derive(Default, Debug, Clone)]
pub struct AuthZone {
    pub domain: String,
    pub interface_names: Vec<AuthNameList>,
    pub subnet: Vec<AddrList>,
    pub exclude: Vec<AddrList>,
    // struct auth_zone *next;
}
