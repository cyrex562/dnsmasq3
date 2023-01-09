use crate::mysockaddr::mysockaddr;
/* subnet parameters from command line */
#[derive(Default, Debug, Clone)]
pub struct mysubnet {
    // union mysockaddr addr;
    pub addr: mysockaddr,
    // addr_used: i32;
    pub addr_used: i32,
    // mask: i32;
    pub mask: i32,
}
