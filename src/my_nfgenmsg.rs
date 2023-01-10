#[derive(Default,Debug,Clone)]
pub struct my_nfgenmsg {
    pub nfgen_family: u8,
    pub version: u8,
    pub res_id: u16,
}
