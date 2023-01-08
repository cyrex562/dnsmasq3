use libc::time_t;

#[derive(Default, Debug,Clone)]
pub struct AppContext {
    pub arps: Vec<arp_record>,
    pub last: time_t,

}

impl AppContext {
    pub fn new() -> Self {
        Self {
            arps: vec![],
            last: 0,
            ..Default::default()
        }
    }
}
