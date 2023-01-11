/* dns doctor param */
use std::fmt::{Debug, Formatter};
use std::net::IpAddr;
use libc::in_addr;
use crate::util::inaddr_to_string;

#[derive(Clone,Default,Debug)]
pub struct Doctor {
    // struct in_addr in, end, out, mask;
    pub in_address: IpAddr,
    pub end_address: IpAddr,
    pub out_address: IpAddr,
    pub mask_address: IpAddr,
    // struct doctor *next;
}

