/* Async event queue */
#[derive(Default, Debug, Clone)]
pub struct event_desc {
    // int event, data, msg_sz;
    pub event: i32,
    pub data: i32,
    pub msg_sz: i32,
}
