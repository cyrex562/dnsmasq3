use crate::server::Server;

#[derive(Default, Debug, Clone)]
pub struct RandFd {
    // struct server *serv;
    pub serv: Server,
    // fd: i32;
    pub fd: i32,
    // unsigned short refcount; /* refcount == 0xffff means overflow record. */
    pub refcount: u16,
}
