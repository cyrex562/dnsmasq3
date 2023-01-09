use crate::rand_fd::randfd;

#[derive(Default,Debug,Clone)]
pub struct randfd_list {
  // struct randfd *rfd;
  rfd: *mut randfd,
    // struct randfd_list *next;
}
