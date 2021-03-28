#[cfg(windows)]
use core::mem;

#[cfg(windows)]
use winapi::shared::inaddr;

#[cfg(unix)]
use libc;

/// Type alias to the Windows version of `in_addr`.
#[cfg(windows)]
#[allow(non_camel_case_types)]
pub type in_addr = inaddr::in_addr;

/// Wrapper around `in_addr` on which `From` is implemented.
#[cfg(windows)]
#[derive(Copy, Clone)]
pub struct InAddr(pub(crate) in_addr);

#[cfg(windows)]
impl From<u32> for InAddr {
    fn from(value: u32) -> Self {
        let un = unsafe {
            let mut un = mem::zeroed::<inaddr::in_addr_S_un>();
            *un.S_addr_mut() = value;
            un
        };
        InAddr(inaddr::in_addr { S_un: un })
    }
}

#[cfg(windows)]
impl From<InAddr> for u32 {
    fn from(InAddr(addr): InAddr) -> u32 {
        unsafe { *addr.S_un.S_addr() }
    }
}
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

impl InAddr {
    pub fn new<T: Into<Self>>(t: T) -> Self {
        t.into()
    }
}

impl From<in_addr> for InAddr {
    fn from(in_addr: in_addr) -> Self {
        InAddr(in_addr)
    }
}

impl From<InAddr> for in_addr {
    fn from(InAddr(addr): InAddr) -> Self {
        addr
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for InAddr {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Into::<std::net::Ipv4Addr>::into(*self).fmt(formatter)
    }
}

#[cfg(feature = "std")]
impl From<std::net::Ipv4Addr> for InAddr {
    fn from(addr: std::net::Ipv4Addr) -> Self {
        Into::<u32>::into(addr).into()
    }
}

#[cfg(feature = "std")]
impl From<InAddr> for std::net::Ipv4Addr {
    fn from(addr: InAddr) -> Self {
        Into::<u32>::into(addr).into()
    }
}
