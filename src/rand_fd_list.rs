use crate::rand_fd::RandFd;

#[derive(Default,Debug,Clone)]
pub struct RandFdList {
  // struct randfd *rfd;
  rfd: Vec<RandFd>,
    // struct randfd_list *next;
}
