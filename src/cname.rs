use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct Cname {
    // int ttl, flag;
    pub ttl: i32,
    pub flag: i32,
    // char *alias, *target;
    pub alias: String,
    pub target: String,
    // struct cname *next, *targetp;
}
