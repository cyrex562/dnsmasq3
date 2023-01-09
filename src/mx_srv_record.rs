use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct mx_srv_record {
    // char *name, *target;
    pub name: *mut c_char,
    pub target: *mut c_char,
    // int issrv, srvport, priority, weight;
    pub issrv: i32,
    pub srvport: i32,
    pub priority: i32,
    pub weight: i32,
    // unsigned offset: i32;
    pub offset: i32,
    // struct mx_srv_record *next;
}
