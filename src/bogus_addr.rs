use crate::all_addr::all_addr;

#[derive(Default, Debug, Clone)]
pub struct bogus_addr {
    // int is6, prefix;
    pub is6: i32,
    pub prefix: i32,
    // union all_addr addr;
    pub addr: all_addr,
    // struct bogus_addr *next;
}
