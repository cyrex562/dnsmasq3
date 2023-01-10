#[derive(Default, Debug, Clone)]
pub struct pcaprec_hdr_s {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub incl_len: u32,
    pub orig_len: u32,
}
