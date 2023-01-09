use libc::c_char;
use crate::addrlist::addrlist;

#[derive(Default, Debug, Clone)]
pub struct auth_name_list {
    // char *name;
    pub name: *mut c_char,
    // flags: i32;
    pub flags: i32,
    // struct auth_name_list *next;
}

#[derive(Default, Debug, Clone)]
pub struct auth_zone {
    // char *domain;
    pub domain: *mut c_char,
    pub interface_names: *mut auth_name_list,
    // struct addrlist *subnet;
    pub subnet: *mut addrlist,
    // struct addrlist *exclude;
    pub exclude: *mut addrlist,
    // struct auth_zone *next;
}
