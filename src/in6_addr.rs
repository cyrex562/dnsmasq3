#[cfg(windows)]
use core::mem;

#[cfg(windows)]
use winapi::shared::in6addr;

#[cfg(unix)]
use libc;

/// Type alias to the Windows version of `in_addr`.
#[cfg(windows)]
#[allow(non_camel_case_types)]
pub type in6_addr = in6addr::in6_addr;

/// Wrapper around `in_addr` on which `From` is implemented.
#[cfg(windows)]
#[derive(Copy, Clone)]
pub struct In6Addr(pub(crate) in6_addr);

// #[cfg(windows)]
// impl From<u128> for In6Addr {
//     fn from(value: u128) -> Self {
//         let un = unsafe {
//             let mut un = mem::zeroed::<in6addr::in6_addr_u>();
//             *un.S_addr_mut() = value;
//             un
//         };
//         InAddr(inaddr::in_addr { S_un: un })
//     }
// }

// #[cfg(windows)]
// impl From<InAddr> for u128 {
//     fn from(InAddr(addr): InAddr) -> u128 {
//         unsafe { *addr.S_un.S_addr() }
//     }
// }
/// Type alias to the Unix version of `in_addr`.
#[cfg(unix)]
#[allow(non_camel_case_types)]
pub type in_addr = libc::in_addr;

/// Wrapper around `in_addr` on which `From` is implemented.
#[cfg(unix)]
#[derive(Copy, Clone)]
pub struct InAddr(pub(crate) in_addr);

#[cfg(unix)]
impl From<u32> for InAddr {
    fn from(value: u32) -> Self {
        InAddr(in_addr { s_addr: value })
    }
}

#[cfg(unix)]
impl From<InAddr> for u32 {
    fn from(InAddr(addr): InAddr) -> u32 {
        addr.s_addr
    }
}

impl In6Addr {
    pub fn new<T: Into<Self>>(t: T) -> Self {
        t.into()
    }
}

impl From<in6_addr> for In6Addr {
    fn from(in_addr: in6_addr) -> Self {
        In6Addr(in_addr)
    }
}

impl From<In6Addr> for in6_addr {
    fn from(In6Addr(addr): In6Addr) -> Self {
        addr
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for In6Addr {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Into::<std::net::Ipv6Addr>::into(*self).fmt(formatter)
    }
}

#[cfg(feature = "std")]
impl From<std::net::Ipv6Addr> for In6Addr {
    fn from(addr: std::net::Ipv6Addr) -> Self {

        // Into::<u32>::into(addr).into()
        Into::<u128>::into(addr).into()
    }
}

#[cfg(feature = "std")]
impl From<In6Addr> for std::net::Ipv6Addr {
    fn from(addr: In6Addr) -> Self {
        Into::<u128>::into(addr).into()
    }
}
