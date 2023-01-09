#[derive(Default,Debug,Clone)]
pub struct match_param {
    // int ind, matched;
    pub ind: i32,
    pub matched; i32,

    // struct in_addr netmask, broadcast, addr;
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub addr: in_addr,
}
