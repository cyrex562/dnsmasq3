use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct MxSrvRecord {
    // char *name, *target;
    pub name: String,
    pub target: String,
    pub issrv: i32,
    pub srvport: u16,
    pub priority: i32,
    pub weight: i32,
    pub offset: usize,
    // struct mx_srv_record *next;
}
