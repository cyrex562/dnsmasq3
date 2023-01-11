use crate::all_addr::AllAddr;
use crate::mysockaddr::MySockAddr;

#[derive(Default, Debug, Clone)]
pub struct FrecSrc {
    // union mysockaddr source; union all_addr dest; unsigned int iface,
    pub source: MySockAddr,
    pub dest: AllAddr,
    pub iface: u32,
    // log_id; fd: i32;
    pub lod_id: i32,
    pub fd: i32,
    // unsigned short orig_id; struct frec_src * next;
    pub orig_id: u16,
}
