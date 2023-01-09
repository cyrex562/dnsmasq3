use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct name_list {
    // char *name;
    pub name: *mut c_char,
    // struct name_list *next;
}

#[derive(Default, Debug, Clone)]
pub struct host_record {
    // int ttl, flags;
    pub ttl: i32,
    pub flags: i32,
    pub names: *mut name_list,
    // addr: in_addr;
    pub addr: in_addr,
    // addr6: in6_addr;
    pub in6_addr: addr6,
    // struct host_record *next;
}
