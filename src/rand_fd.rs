use crate::server::server;

#[derive(Default, Debug, Clone)]
pub struct randfd {
    // struct server *serv;
    pub serv: server,
    // fd: i32;
    pub fd: i32,
    // unsigned short refcount; /* refcount == 0xffff means overflow record. */
    pub refcount: u16,
}
