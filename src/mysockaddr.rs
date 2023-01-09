/* struct sockaddr is not large enough to hold any address,
   and specifically not big enough to hold an IPv6 address.
   Blech. Roll our own. */
use libc::sockaddr;

#[derive(Debug, Clone)]
pub union mysockaddr {
    // struct sockaddr sa;
    pub sa: sockaddr,
    // struct sockaddr_in in;
    pub sa_in: sockaddr_in,
    // struct sockaddr_in6 in6;
    pub sa_in6: sockaddr_in6,
}
