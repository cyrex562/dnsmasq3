use crate::all_addr::AllAddr;

#[derive(Default, Debug, Clone)]
pub struct BogusAddr {
    // int is6, prefix;
    pub is6: i32,
    pub prefix: i32,
    // union all_addr addr;
    pub addr: AllAddr,
    // struct bogus_addr *next;
}
