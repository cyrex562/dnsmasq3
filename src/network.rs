#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:17"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "72:1"]
    pub type __intmax_t = libc::c_long;
    #[c2rust::src_loc = "73:1"]
    pub type __uintmax_t = libc::c_ulong;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "149:1"]
    pub type __ino64_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "180:1"]
    pub type __blkcnt64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/types.h:17"]
pub mod sys_types_h {
    #[c2rust::src_loc = "49:1"]
    pub type ino_t = __ino64_t;
    #[c2rust::src_loc = "59:1"]
    pub type dev_t = __dev_t;
    #[c2rust::src_loc = "87:1"]
    pub type off_t = __off64_t;
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    use super::types_h::{__ino64_t, __dev_t, __off64_t, __pid_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:17"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src =
  "/usr/lib/llvm-10/lib/clang/10.0.0/include/stddef.h:17"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "89:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_timespec.h:17"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:17"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/socket.h:17"]
pub mod socket_h {
    #[c2rust::src_loc = "33:1"]
    pub type socklen_t = __socklen_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "257:8"]
    pub struct msghdr {
        pub msg_name: *mut libc::c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut iovec,
        pub msg_iovlen: size_t,
        pub msg_control: *mut libc::c_void,
        pub msg_controllen: size_t,
        pub msg_flags: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "275:8"]
    pub struct cmsghdr {
        pub cmsg_len: size_t,
        pub cmsg_level: libc::c_int,
        pub cmsg_type: libc::c_int,
        pub __cmsg_data: [libc::c_uchar; 0],
    }
    #[inline]
    #[c2rust::src_loc = "311:1"]
    pub unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: *mut msghdr,
                                           mut __cmsg: *mut cmsghdr)
     -> *mut cmsghdr {
        if (*__cmsg).cmsg_len <
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            return 0 as *mut cmsghdr
        }
        __cmsg =
            (__cmsg as
                 *mut libc::c_uchar).offset(((*__cmsg).cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                 &
                                                 !(::std::mem::size_of::<size_t>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
                                                as isize) as *mut cmsghdr;
        if __cmsg.offset(1 as libc::c_int as isize) as *mut libc::c_uchar >
               ((*__mhdr).msg_control as
                    *mut libc::c_uchar).offset((*__mhdr).msg_controllen as
                                                   isize) ||
               (__cmsg as
                    *mut libc::c_uchar).offset(((*__cmsg).cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                    &
                                                    !(::std::mem::size_of::<size_t>()
                                                          as
                                                          libc::c_ulong).wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                                                   as isize) >
                   ((*__mhdr).msg_control as
                        *mut libc::c_uchar).offset((*__mhdr).msg_controllen as
                                                       isize) {
            return 0 as *mut cmsghdr
        }
        return __cmsg;
    }
    #[c2rust::src_loc = "53:9"]
    pub const PF_INET6: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "104:9"]
    pub const AF_INET6: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const PF_INET: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const AF_INET: libc::c_int = PF_INET;
    use super::types_h::__socklen_t;
    use super::sockaddr_h::sa_family_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/socket_type.h:17"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
    #[c2rust::src_loc = "31:9"]
    pub const SOCK_DGRAM_0: libc::c_int = SOCK_DGRAM as libc::c_int;
    #[c2rust::src_loc = "28:9"]
    pub const SOCK_STREAM_0: libc::c_int = SOCK_STREAM as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/sockaddr.h:17"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/socket.h:17"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::{sockaddr, socklen_t};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::un_h::sockaddr_un;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn getsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int, __optval: *mut libc::c_void,
                          __optlen: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/un.h:17"]
pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:8"]
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [libc::c_char; 108],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:17"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[c2rust::src_loc = "40:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "289:8"]
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "537:8"]
    pub struct in6_pktinfo {
        pub ipi6_addr: in6_addr,
        pub ipi6_ifindex: libc::c_uint,
    }
    #[c2rust::src_loc = "65:9"]
    pub const IPPROTO_IPV6_0: libc::c_int = IPPROTO_IPV6 as libc::c_int;
    #[c2rust::src_loc = "43:9"]
    pub const IPPROTO_IP_0: libc::c_int = IPPROTO_IP as libc::c_int;
    #[c2rust::src_loc = "190:9"]
    pub const INADDR_ANY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const IPPROTO_TCP_0: libc::c_int = IPPROTO_TCP as libc::c_int;
    #[c2rust::src_loc = "234:9"]
    pub const INET6_ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "228:30"]
        pub static in6addr_any: in6_addr;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:17"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint32_t, __uint16_t, __uint8_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/in.h:17"]
pub mod bits_in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "157:8"]
    pub struct in_pktinfo {
        pub ipi_ifindex: libc::c_int,
        pub ipi_spec_dst: in_addr,
        pub ipi_addr: in_addr,
    }
    #[c2rust::src_loc = "231:9"]
    pub const IPV6_UNICAST_IF: libc::c_int = 76 as libc::c_int;
    #[c2rust::src_loc = "129:9"]
    pub const IP_UNICAST_IF: libc::c_int = 50 as libc::c_int;
    #[c2rust::src_loc = "191:9"]
    pub const IPV6_V6ONLY: libc::c_int = 26 as libc::c_int;
    #[c2rust::src_loc = "169:9"]
    pub const IPV6_2292PKTINFO: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "201:9"]
    pub const IPV6_RECVPKTINFO: libc::c_int = 49 as libc::c_int;
    #[c2rust::src_loc = "202:9"]
    pub const IPV6_PKTINFO: libc::c_int = 50 as libc::c_int;
    #[c2rust::src_loc = "81:9"]
    pub const IP_PKTINFO: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "173:9"]
    pub const IPV6_2292PKTOPTIONS: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "82:9"]
    pub const IP_PKTOPTIONS: libc::c_int = 9 as libc::c_int;
    #[c2rust::src_loc = "185:9"]
    pub const IPV6_JOIN_GROUP: libc::c_int = 20 as libc::c_int;
    use super::in_h::in_addr;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dnsmasq.h:17"]
pub mod dnsmasq_h {
    #[c2rust::src_loc = "70:1"]
    pub type u32_0 = libc::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "295:7"]
    pub union all_addr {
        pub addr4: in_addr,
        pub addr6: in6_addr,
        pub cname: C2RustUnnamed_8,
        pub key: C2RustUnnamed_7,
        pub ds: C2RustUnnamed_6,
        pub srv: C2RustUnnamed_5,
        pub log: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "322:3"]
    pub struct C2RustUnnamed_4 {
        pub keytag: libc::c_ushort,
        pub algo: libc::c_ushort,
        pub digest: libc::c_ushort,
        pub rcode: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:3"]
    pub struct C2RustUnnamed_5 {
        pub target: *mut blockdata,
        pub targetlen: libc::c_ushort,
        pub srvport: libc::c_ushort,
        pub priority: libc::c_ushort,
        pub weight: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "443:8"]
    pub struct blockdata {
        pub next: *mut blockdata,
        pub key: [libc::c_uchar; 40],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "311:3"]
    pub struct C2RustUnnamed_6 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
        pub digest: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:3"]
    pub struct C2RustUnnamed_7 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub flags: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:3"]
    pub struct C2RustUnnamed_8 {
        pub target: C2RustUnnamed_9,
        pub uid: libc::c_uint,
        pub is_name_ptr: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:5"]
    pub union C2RustUnnamed_9 {
        pub cache: *mut crec,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "448:8"]
    pub struct crec {
        pub next: *mut crec,
        pub prev: *mut crec,
        pub hash_next: *mut crec,
        pub addr: all_addr,
        pub ttd: time_t,
        pub uid: libc::c_uint,
        pub flags: libc::c_uint,
        pub name: C2RustUnnamed_10,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "455:3"]
    pub union C2RustUnnamed_10 {
        pub sname: [libc::c_char; 50],
        pub bname: *mut bigname,
        pub namep: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "438:7"]
    pub union bigname {
        pub name: [libc::c_char; 1025],
        pub next: *mut bigname,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "328:8"]
    pub struct bogus_addr {
        pub addr: in_addr,
        pub next: *mut bogus_addr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "334:8"]
    pub struct doctor {
        pub in_0: in_addr,
        pub end: in_addr,
        pub out: in_addr,
        pub mask: in_addr,
        pub next: *mut doctor,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "339:8"]
    pub struct mx_srv_record {
        pub name: *mut libc::c_char,
        pub target: *mut libc::c_char,
        pub issrv: libc::c_int,
        pub srvport: libc::c_int,
        pub priority: libc::c_int,
        pub weight: libc::c_int,
        pub offset: libc::c_uint,
        pub next: *mut mx_srv_record,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "346:8"]
    pub struct naptr {
        pub name: *mut libc::c_char,
        pub replace: *mut libc::c_char,
        pub regexp: *mut libc::c_char,
        pub services: *mut libc::c_char,
        pub flags: *mut libc::c_char,
        pub order: libc::c_uint,
        pub pref: libc::c_uint,
        pub next: *mut naptr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "362:8"]
    pub struct txt_record {
        pub name: *mut libc::c_char,
        pub txt: *mut libc::c_uchar,
        pub class: libc::c_ushort,
        pub len: libc::c_ushort,
        pub stat: libc::c_int,
        pub next: *mut txt_record,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "370:8"]
    pub struct ptr_record {
        pub name: *mut libc::c_char,
        pub ptr: *mut libc::c_char,
        pub next: *mut ptr_record,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "375:8"]
    pub struct cname {
        pub ttl: libc::c_int,
        pub flag: libc::c_int,
        pub alias: *mut libc::c_char,
        pub target: *mut libc::c_char,
        pub next: *mut cname,
        pub targetp: *mut cname,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "394:8"]
    pub struct addrlist {
        pub addr: all_addr,
        pub flags: libc::c_int,
        pub prefixlen: libc::c_int,
        pub decline_time: time_t,
        pub next: *mut addrlist,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "404:8"]
    pub struct auth_zone {
        pub domain: *mut libc::c_char,
        pub interface_names: *mut auth_name_list,
        pub subnet: *mut addrlist,
        pub exclude: *mut addrlist,
        pub next: *mut auth_zone,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "406:10"]
    pub struct auth_name_list {
        pub name: *mut libc::c_char,
        pub flags: libc::c_int,
        pub next: *mut auth_name_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "419:8"]
    pub struct host_record {
        pub ttl: libc::c_int,
        pub flags: libc::c_int,
        pub names: *mut name_list,
        pub addr: in_addr,
        pub addr6: in6_addr,
        pub next: *mut host_record,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "421:10"]
    pub struct name_list {
        pub name: *mut libc::c_char,
        pub next: *mut name_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "430:8"]
    pub struct interface_name {
        pub name: *mut libc::c_char,
        pub intr: *mut libc::c_char,
        pub family: libc::c_int,
        pub addr: *mut addrlist,
        pub next: *mut interface_name,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "507:7"]
    pub union mysockaddr {
        pub sa: sockaddr,
        pub in_0: sockaddr_in,
        pub in6: sockaddr_in6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "537:8"]
    pub struct serverfd {
        pub fd: libc::c_int,
        pub source_addr: mysockaddr,
        pub interface: [libc::c_char; 17],
        pub ifindex: libc::c_uint,
        pub used: libc::c_uint,
        pub preallocated: libc::c_uint,
        pub next: *mut serverfd,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "545:8"]
    pub struct randfd {
        pub fd: libc::c_int,
        pub refcount: libc::c_ushort,
        pub family: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "550:8"]
    pub struct server {
        pub addr: mysockaddr,
        pub source_addr: mysockaddr,
        pub interface: [libc::c_char; 17],
        pub sfd: *mut serverfd,
        pub domain: *mut libc::c_char,
        pub flags: libc::c_int,
        pub tcpfd: libc::c_int,
        pub edns_pktsz: libc::c_int,
        pub pktsz_reduced: time_t,
        pub queries: libc::c_uint,
        pub failed_queries: libc::c_uint,
        pub uid: u32_0,
        pub next: *mut server,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "564:8"]
    pub struct ipsets {
        pub sets: *mut *mut libc::c_char,
        pub domain: *mut libc::c_char,
        pub next: *mut ipsets,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "570:8"]
    pub struct irec {
        pub addr: mysockaddr,
        pub netmask: in_addr,
        pub tftp_ok: libc::c_int,
        pub dhcp_ok: libc::c_int,
        pub mtu: libc::c_int,
        pub done: libc::c_int,
        pub warned: libc::c_int,
        pub dad: libc::c_int,
        pub dns_auth: libc::c_int,
        pub index: libc::c_int,
        pub multicast_done: libc::c_int,
        pub found: libc::c_int,
        pub label: libc::c_int,
        pub name: *mut libc::c_char,
        pub next: *mut irec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "578:8"]
    pub struct listener {
        pub fd: libc::c_int,
        pub tcpfd: libc::c_int,
        pub tftpfd: libc::c_int,
        pub used: libc::c_int,
        pub addr: mysockaddr,
        pub iface: *mut irec,
        pub next: *mut listener,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "586:8"]
    pub struct iname {
        pub name: *mut libc::c_char,
        pub addr: mysockaddr,
        pub used: libc::c_int,
        pub next: *mut iname,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "594:8"]
    pub struct mysubnet {
        pub addr: mysockaddr,
        pub addr_used: libc::c_int,
        pub mask: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "601:8"]
    pub struct resolvc {
        pub next: *mut resolvc,
        pub is_default: libc::c_int,
        pub logged: libc::c_int,
        pub mtime: time_t,
        pub name: *mut libc::c_char,
        pub wd: libc::c_int,
        pub file: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "619:8"]
    pub struct hostsfile {
        pub next: *mut hostsfile,
        pub flags: libc::c_int,
        pub fname: *mut libc::c_char,
        pub wd: libc::c_int,
        pub index: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "666:8"]
    pub struct frec {
        pub frec_src: frec_src,
        pub sentto: *mut server,
        pub rfd4: *mut randfd,
        pub rfd6: *mut randfd,
        pub new_id: libc::c_ushort,
        pub forwardall: libc::c_int,
        pub flags: libc::c_int,
        pub time: time_t,
        pub hash: [*mut libc::c_uchar; 32],
        pub next: *mut frec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "667:10"]
    pub struct frec_src {
        pub source: mysockaddr,
        pub dest: all_addr,
        pub iface: libc::c_uint,
        pub log_id: libc::c_uint,
        pub fd: libc::c_int,
        pub orig_id: libc::c_ushort,
        pub next: *mut frec_src,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "752:8"]
    pub struct dhcp_netid {
        pub net: *mut libc::c_char,
        pub next: *mut dhcp_netid,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:8"]
    pub struct dhcp_netid_list {
        pub list: *mut dhcp_netid,
        pub next: *mut dhcp_netid_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "762:8"]
    pub struct tag_if {
        pub set: *mut dhcp_netid_list,
        pub tag: *mut dhcp_netid,
        pub next: *mut tag_if,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "768:8"]
    pub struct delay_config {
        pub delay: libc::c_int,
        pub netid: *mut dhcp_netid,
        pub next: *mut delay_config,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "774:8"]
    pub struct hwaddr_config {
        pub hwaddr_len: libc::c_int,
        pub hwaddr_type: libc::c_int,
        pub hwaddr: [libc::c_uchar; 16],
        pub wildcard_mask: libc::c_uint,
        pub next: *mut hwaddr_config,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "781:8"]
    pub struct dhcp_config {
        pub flags: libc::c_uint,
        pub clid_len: libc::c_int,
        pub clid: *mut libc::c_uchar,
        pub hostname: *mut libc::c_char,
        pub domain: *mut libc::c_char,
        pub netid: *mut dhcp_netid_list,
        pub filter: *mut dhcp_netid,
        pub addr6: *mut addrlist,
        pub addr: in_addr,
        pub decline_time: time_t,
        pub lease_time: libc::c_uint,
        pub hwaddr: *mut hwaddr_config,
        pub next: *mut dhcp_config,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "813:8"]
    pub struct dhcp_opt {
        pub opt: libc::c_int,
        pub len: libc::c_int,
        pub flags: libc::c_int,
        pub u: C2RustUnnamed_11,
        pub val: *mut libc::c_uchar,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_opt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "815:3"]
    pub union C2RustUnnamed_11 {
        pub encap: libc::c_int,
        pub wildcard_mask: libc::c_uint,
        pub vendor_class: *mut libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "841:8"]
    pub struct dhcp_boot {
        pub file: *mut libc::c_char,
        pub sname: *mut libc::c_char,
        pub tftp_sname: *mut libc::c_char,
        pub next_server: in_addr,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_boot,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "848:8"]
    pub struct dhcp_match_name {
        pub name: *mut libc::c_char,
        pub wildcard: libc::c_int,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_match_name,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "855:8"]
    pub struct pxe_service {
        pub CSA: libc::c_ushort,
        pub type_0: libc::c_ushort,
        pub menu: *mut libc::c_char,
        pub basename: *mut libc::c_char,
        pub sname: *mut libc::c_char,
        pub server: in_addr,
        pub netid: *mut dhcp_netid,
        pub next: *mut pxe_service,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "872:8"]
    pub struct dhcp_vendor {
        pub len: libc::c_int,
        pub match_type: libc::c_int,
        pub enterprise: libc::c_uint,
        pub data: *mut libc::c_char,
        pub netid: dhcp_netid,
        pub next: *mut dhcp_vendor,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "880:8"]
    pub struct dhcp_pxe_vendor {
        pub data: *mut libc::c_char,
        pub next: *mut dhcp_pxe_vendor,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "885:8"]
    pub struct dhcp_mac {
        pub mask: libc::c_uint,
        pub hwaddr_len: libc::c_int,
        pub hwaddr_type: libc::c_int,
        pub hwaddr: [libc::c_uchar; 16],
        pub netid: dhcp_netid,
        pub next: *mut dhcp_mac,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "893:8"]
    pub struct dhcp_bridge {
        pub iface: [libc::c_char; 16],
        pub alias: *mut dhcp_bridge,
        pub next: *mut dhcp_bridge,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "898:8"]
    pub struct cond_domain {
        pub domain: *mut libc::c_char,
        pub prefix: *mut libc::c_char,
        pub start: in_addr,
        pub end: in_addr,
        pub start6: in6_addr,
        pub end6: in6_addr,
        pub is6: libc::c_int,
        pub indexed: libc::c_int,
        pub next: *mut cond_domain,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "906:8"]
    pub struct ra_interface {
        pub name: *mut libc::c_char,
        pub mtu_name: *mut libc::c_char,
        pub interval: libc::c_int,
        pub lifetime: libc::c_int,
        pub prio: libc::c_int,
        pub mtu: libc::c_int,
        pub next: *mut ra_interface,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "913:8"]
    pub struct dhcp_context {
        pub lease_time: libc::c_uint,
        pub addr_epoch: libc::c_uint,
        pub netmask: in_addr,
        pub broadcast: in_addr,
        pub local: in_addr,
        pub router: in_addr,
        pub start: in_addr,
        pub end: in_addr,
        pub start6: in6_addr,
        pub end6: in6_addr,
        pub local6: in6_addr,
        pub prefix: libc::c_int,
        pub if_index: libc::c_int,
        pub valid: libc::c_uint,
        pub preferred: libc::c_uint,
        pub saved_valid: libc::c_uint,
        pub ra_time: time_t,
        pub ra_short_period_start: time_t,
        pub address_lost_time: time_t,
        pub template_interface: *mut libc::c_char,
        pub flags: libc::c_int,
        pub netid: dhcp_netid,
        pub filter: *mut dhcp_netid,
        pub next: *mut dhcp_context,
        pub current: *mut dhcp_context,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "931:8"]
    pub struct shared_network {
        pub if_index: libc::c_int,
        pub match_addr: in_addr,
        pub shared_addr: in_addr,
        pub match_addr6: in6_addr,
        pub shared_addr6: in6_addr,
        pub next: *mut shared_network,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "962:8"]
    pub struct ping_result {
        pub addr: in_addr,
        pub time: time_t,
        pub hash: libc::c_uint,
        pub next: *mut ping_result,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "969:8"]
    pub struct tftp_file {
        pub refcount: libc::c_int,
        pub fd: libc::c_int,
        pub size: off_t,
        pub dev: dev_t,
        pub inode: ino_t,
        pub filename: [libc::c_char; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "977:8"]
    pub struct tftp_transfer {
        pub sockfd: libc::c_int,
        pub timeout: time_t,
        pub backoff: libc::c_int,
        pub block: libc::c_uint,
        pub blocksize: libc::c_uint,
        pub expansion: libc::c_uint,
        pub offset: off_t,
        pub peer: mysockaddr,
        pub source: all_addr,
        pub if_index: libc::c_int,
        pub opt_blocksize: libc::c_char,
        pub opt_transize: libc::c_char,
        pub netascii: libc::c_char,
        pub carrylf: libc::c_char,
        pub file: *mut tftp_file,
        pub next: *mut tftp_transfer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "991:8"]
    pub struct addr_list {
        pub addr: in_addr,
        pub next: *mut addr_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "996:8"]
    pub struct tftp_prefix {
        pub interface: *mut libc::c_char,
        pub prefix: *mut libc::c_char,
        pub missing: libc::c_int,
        pub next: *mut tftp_prefix,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1003:8"]
    pub struct dhcp_relay {
        pub local: all_addr,
        pub server: all_addr,
        pub interface: *mut libc::c_char,
        pub iface_index: libc::c_int,
        pub current: *mut dhcp_relay,
        pub next: *mut dhcp_relay,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1010:15"]
    pub struct dnsmasq_daemon {
        pub options: [libc::c_uint; 2],
        pub default_resolv: resolvc,
        pub resolv_files: *mut resolvc,
        pub last_resolv: time_t,
        pub servers_file: *mut libc::c_char,
        pub mxnames: *mut mx_srv_record,
        pub naptr: *mut naptr,
        pub txt: *mut txt_record,
        pub rr: *mut txt_record,
        pub ptr: *mut ptr_record,
        pub host_records: *mut host_record,
        pub host_records_tail: *mut host_record,
        pub cnames: *mut cname,
        pub auth_zones: *mut auth_zone,
        pub int_names: *mut interface_name,
        pub mxtarget: *mut libc::c_char,
        pub add_subnet4: *mut mysubnet,
        pub add_subnet6: *mut mysubnet,
        pub lease_file: *mut libc::c_char,
        pub username: *mut libc::c_char,
        pub groupname: *mut libc::c_char,
        pub scriptuser: *mut libc::c_char,
        pub luascript: *mut libc::c_char,
        pub authserver: *mut libc::c_char,
        pub hostmaster: *mut libc::c_char,
        pub authinterface: *mut iname,
        pub secondary_forward_server: *mut name_list,
        pub group_set: libc::c_int,
        pub osport: libc::c_int,
        pub domain_suffix: *mut libc::c_char,
        pub cond_domain: *mut cond_domain,
        pub synth_domains: *mut cond_domain,
        pub runfile: *mut libc::c_char,
        pub lease_change_command: *mut libc::c_char,
        pub if_names: *mut iname,
        pub if_addrs: *mut iname,
        pub if_except: *mut iname,
        pub dhcp_except: *mut iname,
        pub auth_peers: *mut iname,
        pub tftp_interfaces: *mut iname,
        pub bogus_addr: *mut bogus_addr,
        pub ignore_addr: *mut bogus_addr,
        pub servers: *mut server,
        pub ipsets: *mut ipsets,
        pub log_fac: libc::c_int,
        pub log_file: *mut libc::c_char,
        pub max_logs: libc::c_int,
        pub cachesize: libc::c_int,
        pub ftabsize: libc::c_int,
        pub port: libc::c_int,
        pub query_port: libc::c_int,
        pub min_port: libc::c_int,
        pub max_port: libc::c_int,
        pub local_ttl: libc::c_ulong,
        pub neg_ttl: libc::c_ulong,
        pub max_ttl: libc::c_ulong,
        pub min_cache_ttl: libc::c_ulong,
        pub max_cache_ttl: libc::c_ulong,
        pub auth_ttl: libc::c_ulong,
        pub dhcp_ttl: libc::c_ulong,
        pub use_dhcp_ttl: libc::c_ulong,
        pub dns_client_id: *mut libc::c_char,
        pub addn_hosts: *mut hostsfile,
        pub dhcp: *mut dhcp_context,
        pub dhcp6: *mut dhcp_context,
        pub ra_interfaces: *mut ra_interface,
        pub dhcp_conf: *mut dhcp_config,
        pub dhcp_opts: *mut dhcp_opt,
        pub dhcp_match: *mut dhcp_opt,
        pub dhcp_opts6: *mut dhcp_opt,
        pub dhcp_match6: *mut dhcp_opt,
        pub dhcp_name_match: *mut dhcp_match_name,
        pub dhcp_pxe_vendors: *mut dhcp_pxe_vendor,
        pub dhcp_vendors: *mut dhcp_vendor,
        pub dhcp_macs: *mut dhcp_mac,
        pub boot_config: *mut dhcp_boot,
        pub pxe_services: *mut pxe_service,
        pub tag_if: *mut tag_if,
        pub override_relays: *mut addr_list,
        pub relay4: *mut dhcp_relay,
        pub relay6: *mut dhcp_relay,
        pub delay_conf: *mut delay_config,
        pub override_0: libc::c_int,
        pub enable_pxe: libc::c_int,
        pub doing_ra: libc::c_int,
        pub doing_dhcp6: libc::c_int,
        pub dhcp_ignore: *mut dhcp_netid_list,
        pub dhcp_ignore_names: *mut dhcp_netid_list,
        pub dhcp_gen_names: *mut dhcp_netid_list,
        pub force_broadcast: *mut dhcp_netid_list,
        pub bootp_dynamic: *mut dhcp_netid_list,
        pub dhcp_hosts_file: *mut hostsfile,
        pub dhcp_opts_file: *mut hostsfile,
        pub dynamic_dirs: *mut hostsfile,
        pub dhcp_max: libc::c_int,
        pub tftp_max: libc::c_int,
        pub tftp_mtu: libc::c_int,
        pub dhcp_server_port: libc::c_int,
        pub dhcp_client_port: libc::c_int,
        pub start_tftp_port: libc::c_int,
        pub end_tftp_port: libc::c_int,
        pub min_leasetime: libc::c_uint,
        pub doctors: *mut doctor,
        pub edns_pktsz: libc::c_ushort,
        pub tftp_prefix: *mut libc::c_char,
        pub if_prefix: *mut tftp_prefix,
        pub duid_enterprise: libc::c_uint,
        pub duid_config_len: libc::c_uint,
        pub duid_config: *mut libc::c_uchar,
        pub dbus_name: *mut libc::c_char,
        pub ubus_name: *mut libc::c_char,
        pub dump_file: *mut libc::c_char,
        pub dump_mask: libc::c_int,
        pub soa_sn: libc::c_ulong,
        pub soa_refresh: libc::c_ulong,
        pub soa_retry: libc::c_ulong,
        pub soa_expiry: libc::c_ulong,
        pub metrics: [u32_0; 20],
        pub packet: *mut libc::c_char,
        pub packet_buff_sz: libc::c_int,
        pub namebuff: *mut libc::c_char,
        pub frec_list: *mut frec,
        pub free_frec_src: *mut frec_src,
        pub frec_src_count: libc::c_int,
        pub sfds: *mut serverfd,
        pub interfaces: *mut irec,
        pub listeners: *mut listener,
        pub last_server: *mut server,
        pub forwardtime: time_t,
        pub forwardcount: libc::c_int,
        pub srv_save: *mut server,
        pub packet_len: size_t,
        pub rfd_save: *mut randfd,
        pub tcp_pids: [pid_t; 20],
        pub tcp_pipes: [libc::c_int; 20],
        pub pipe_to_parent: libc::c_int,
        pub randomsocks: [randfd; 64],
        pub v6pktinfo: libc::c_int,
        pub interface_addrs: *mut addrlist,
        pub log_id: libc::c_int,
        pub log_display_id: libc::c_int,
        pub log_source_addr: *mut mysockaddr,
        pub dhcpfd: libc::c_int,
        pub helperfd: libc::c_int,
        pub pxefd: libc::c_int,
        pub inotifyfd: libc::c_int,
        pub netlinkfd: libc::c_int,
        pub kernel_version: libc::c_int,
        pub dhcp_packet: iovec,
        pub dhcp_buff: *mut libc::c_char,
        pub dhcp_buff2: *mut libc::c_char,
        pub dhcp_buff3: *mut libc::c_char,
        pub ping_results: *mut ping_result,
        pub lease_stream: *mut FILE,
        pub bridges: *mut dhcp_bridge,
        pub shared_networks: *mut shared_network,
        pub duid_len: libc::c_int,
        pub duid: *mut libc::c_uchar,
        pub outpacket: iovec,
        pub dhcp6fd: libc::c_int,
        pub icmp6fd: libc::c_int,
        pub dbus: *mut libc::c_void,
        pub tftp_trans: *mut tftp_transfer,
        pub tftp_done_trans: *mut tftp_transfer,
        pub addrbuff: *mut libc::c_char,
        pub addrbuff2: *mut libc::c_char,
        pub dumpfd: libc::c_int,
    }
    #[c2rust::src_loc = "208:9"]
    pub const EC_BADNET: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "531:9"]
    pub const SERV_NO_REBIND: libc::c_int = 2048 as libc::c_int;
    #[c2rust::src_loc = "530:9"]
    pub const SERV_USE_RESOLV: libc::c_int = 1024 as libc::c_int;
    #[c2rust::src_loc = "520:9"]
    pub const SERV_NO_ADDR: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "521:9"]
    pub const SERV_LITERAL_ADDRESS: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "527:9"]
    pub const SERV_MARK: libc::c_int = 256 as libc::c_int;
    #[c2rust::src_loc = "519:9"]
    pub const SERV_FROM_RESOLV: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "522:9"]
    pub const SERV_HAS_DOMAIN: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "533:9"]
    pub const SERV_LOOP: libc::c_int = 8192 as libc::c_int;
    #[c2rust::src_loc = "524:9"]
    pub const SERV_FOR_NODOTS: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "514:9"]
    pub const IFACE_TENTATIVE: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "389:9"]
    pub const ADDRLIST_REVONLY: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "515:9"]
    pub const IFACE_DEPRECATED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "516:9"]
    pub const IFACE_PERMANENT: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "388:9"]
    pub const ADDRLIST_IPV6: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "401:9"]
    pub const AUTH6: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "402:9"]
    pub const AUTH4: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "387:9"]
    pub const ADDRLIST_LITERAL: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "171:9"]
    pub const ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6,
                      in_addr_t};
    use super::time_t_h::time_t;
    use super::socket_h::sockaddr;
    use super::sys_types_h::{off_t, dev_t, ino_t, pid_t};
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1172:4"]
        pub static mut dnsmasq_daemon: *mut dnsmasq_daemon;
        #[no_mangle]
        #[c2rust::src_loc = "1247:1"]
        pub fn private_net(addr: in_addr, ban_localhost: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1282:1"]
        pub fn rand16() -> libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "1283:1"]
        pub fn rand32() -> u32_0;
        #[no_mangle]
        #[c2rust::src_loc = "1288:1"]
        pub fn safe_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1289:1"]
        pub fn safe_strncpy(dest: *mut libc::c_char, src: *const libc::c_char,
                            size: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "1291:1"]
        pub fn whine_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1292:1"]
        pub fn sa_len(addr: *mut mysockaddr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1293:1"]
        pub fn sockaddr_isequal(s1: *mut mysockaddr, s2: *mut mysockaddr)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1294:1"]
        pub fn hostname_isequal(a: *const libc::c_char,
                                b: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1304:1"]
        pub fn prettyprint_addr(addr: *mut mysockaddr, buf: *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1313:1"]
        pub fn wildcard_match(wildcard: *const libc::c_char,
                              match_0: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1320:1"]
        pub fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
                   exit_code: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char,
                         _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1348:1"]
        pub fn server_gone(server: *mut server);
        #[no_mangle]
        #[c2rust::src_loc = "1639:1"]
        pub fn loop_send_probes();
        #[no_mangle]
        #[c2rust::src_loc = "1489:1"]
        pub fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                               callback:
                                   Option<unsafe extern "C" fn()
                                              -> libc::c_int>) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1446:1"]
        pub fn lease_find_interfaces(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1559:1"]
        pub fn dhcp_construct_contexts(now: time_t);
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stat.h:17"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:8"]
    pub struct stat64 {
        pub st_dev: __dev_t,
        pub st_ino: __ino64_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt64_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    #[c2rust::src_loc = "38:10"]
    pub const _STAT_VER_LINUX: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const _STAT_VER: libc::c_int = _STAT_VER_LINUX;
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t, __ino64_t, __blkcnt64_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/net/if.h:17"]
pub mod if_h {
    #[c2rust::src_loc = "42:1"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "79:5"]
    pub const IFF_DYNAMIC: C2RustUnnamed_1 = 32768;
    #[c2rust::src_loc = "77:5"]
    pub const IFF_AUTOMEDIA: C2RustUnnamed_1 = 16384;
    #[c2rust::src_loc = "75:5"]
    pub const IFF_PORTSEL: C2RustUnnamed_1 = 8192;
    #[c2rust::src_loc = "72:5"]
    pub const IFF_MULTICAST: C2RustUnnamed_1 = 4096;
    #[c2rust::src_loc = "69:5"]
    pub const IFF_SLAVE: C2RustUnnamed_1 = 2048;
    #[c2rust::src_loc = "67:5"]
    pub const IFF_MASTER: C2RustUnnamed_1 = 1024;
    #[c2rust::src_loc = "64:5"]
    pub const IFF_ALLMULTI: C2RustUnnamed_1 = 512;
    #[c2rust::src_loc = "60:5"]
    pub const IFF_PROMISC: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "58:5"]
    pub const IFF_NOARP: C2RustUnnamed_1 = 128;
    #[c2rust::src_loc = "56:5"]
    pub const IFF_RUNNING: C2RustUnnamed_1 = 64;
    #[c2rust::src_loc = "54:5"]
    pub const IFF_NOTRAILERS: C2RustUnnamed_1 = 32;
    #[c2rust::src_loc = "52:5"]
    pub const IFF_POINTOPOINT: C2RustUnnamed_1 = 16;
    #[c2rust::src_loc = "50:5"]
    pub const IFF_LOOPBACK: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "48:5"]
    pub const IFF_DEBUG: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IFF_BROADCAST: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IFF_UP: C2RustUnnamed_1 = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:8"]
    pub struct ifmap {
        pub mem_start: libc::c_ulong,
        pub mem_end: libc::c_ulong,
        pub base_addr: libc::c_ushort,
        pub irq: libc::c_uchar,
        pub dma: libc::c_uchar,
        pub port: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:8"]
    pub struct ifreq {
        pub ifr_ifrn: C2RustUnnamed_3,
        pub ifr_ifru: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:5"]
    pub union C2RustUnnamed_2 {
        pub ifru_addr: sockaddr,
        pub ifru_dstaddr: sockaddr,
        pub ifru_broadaddr: sockaddr,
        pub ifru_netmask: sockaddr,
        pub ifru_hwaddr: sockaddr,
        pub ifru_flags: libc::c_short,
        pub ifru_ivalue: libc::c_int,
        pub ifru_mtu: libc::c_int,
        pub ifru_map: ifmap,
        pub ifru_slave: [libc::c_char; 16],
        pub ifru_newname: [libc::c_char; 16],
        pub ifru_data: __caddr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:5"]
    pub union C2RustUnnamed_3 {
        pub ifrn_name: [libc::c_char; 16],
    }
    #[c2rust::src_loc = "31:9"]
    pub const IF_NAMESIZE: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "51:10"]
    pub const IFF_LOOPBACK_0: libc::c_int = IFF_LOOPBACK as libc::c_int;
    use super::socket_h::sockaddr;
    use super::types_h::__caddr_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "193:1"]
        pub fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    }
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:17"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    #[c2rust::src_loc = "111:9"]
    pub const _IO_EOF_SEEN: libc::c_int = 0x10 as libc::c_int;
    #[c2rust::src_loc = "114:9"]
    pub const _IO_ERR_SEEN: libc::c_int = 0x20 as libc::c_int;
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:17"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:17"]
pub mod stdlib_h {
    #[c2rust::src_loc = "808:1"]
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char)
     -> libc::c_int {
        return strtol(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char,
                      10 as libc::c_int) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "365:1"]
    pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char)
     -> libc::c_long {
        return strtol(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char,
                      10 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "372:15"]
    pub unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
     -> libc::c_longlong {
        return strtoll(__nptr,
                       NULL as *mut libc::c_void as *mut *mut libc::c_char,
                       10 as libc::c_int);
    }
    use super::stddef_h::NULL;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "117:15"]
        pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
         -> libc::c_double;
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "200:22"]
        pub fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
                       _: libc::c_int) -> libc::c_longlong;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdint.h:17"]
pub mod stdint_h {
    #[c2rust::src_loc = "101:1"]
    pub type intmax_t = __intmax_t;
    #[c2rust::src_loc = "102:1"]
    pub type uintmax_t = __uintmax_t;
    use super::types_h::{__intmax_t, __uintmax_t};
}
#[c2rust::header_src = "/usr/include/inttypes.h:17"]
pub mod inttypes_h {
    #[c2rust::src_loc = "34:1"]
    pub type __gwchar_t = libc::c_int;
    #[inline]
    #[c2rust::src_loc = "323:1"]
    pub unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
                                       mut endptr: *mut *mut libc::c_char,
                                       mut base: libc::c_int) -> intmax_t {
        return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "335:1"]
    pub unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
                                       mut endptr: *mut *mut libc::c_char,
                                       mut base: libc::c_int) -> uintmax_t {
        return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "347:1"]
    pub unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
                                       mut endptr: *mut *mut __gwchar_t,
                                       mut base: libc::c_int) -> intmax_t {
        return __wcstol_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                                       mut endptr: *mut *mut __gwchar_t,
                                       mut base: libc::c_int) -> uintmax_t {
        return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    use super::stdint_h::{intmax_t, uintmax_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "318:1"]
        pub fn __strtol_internal(__nptr: *const libc::c_char,
                                 __endptr: *mut *mut libc::c_char,
                                 __base: libc::c_int, __group: libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "330:1"]
        pub fn __strtoul_internal(__nptr: *const libc::c_char,
                                  __endptr: *mut *mut libc::c_char,
                                  __base: libc::c_int, __group: libc::c_int)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "342:1"]
        pub fn __wcstol_internal(__nptr: *const __gwchar_t,
                                 __endptr: *mut *mut __gwchar_t,
                                 __base: libc::c_int, __group: libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "354:1"]
        pub fn __wcstoul_internal(__nptr: *const __gwchar_t,
                                  __endptr: *mut *mut __gwchar_t,
                                  __base: libc::c_int, __group: libc::c_int)
         -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:17"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "257:26"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "486:1"]
        pub fn getc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "603:1"]
        pub fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                          __delimiter: libc::c_int, __stream: *mut FILE)
         -> __ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "858:1"]
        pub fn __uflow(_: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "859:1"]
        pub fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:17"]
pub mod inet_h {
    use super::in_h::in_addr_t;
    use super::socket_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                         __buf: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                         __buf: *mut libc::c_char, __len: socklen_t)
         -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/stat.h:17"]
pub mod sys_stat_h {
    #[inline]
    #[c2rust::src_loc = "452:1"]
    pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                                  mut __statbuf: *mut stat) -> libc::c_int {
        return __xstat(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "466:1"]
    pub unsafe extern "C" fn fstat(mut __fd: libc::c_int,
                                   mut __statbuf: *mut stat) -> libc::c_int {
        return __fxstat(_STAT_VER, __fd, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "501:1"]
    pub unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                                    mut __statbuf: *mut stat64)
     -> libc::c_int {
        return __xstat64(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "515:1"]
    pub unsafe extern "C" fn fstat64(mut __fd: libc::c_int,
                                     mut __statbuf: *mut stat64)
     -> libc::c_int {
        return __fxstat64(_STAT_VER, __fd, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "473:1"]
    pub unsafe extern "C" fn fstatat(mut __fd: libc::c_int,
                                     mut __filename: *const libc::c_char,
                                     mut __statbuf: *mut stat,
                                     mut __flag: libc::c_int) -> libc::c_int {
        return __fxstatat(_STAT_VER, __fd, __filename, __statbuf, __flag);
    }
    #[inline]
    #[c2rust::src_loc = "522:1"]
    pub unsafe extern "C" fn fstatat64(mut __fd: libc::c_int,
                                       mut __filename: *const libc::c_char,
                                       mut __statbuf: *mut stat64,
                                       mut __flag: libc::c_int)
     -> libc::c_int {
        return __fxstatat64(_STAT_VER, __fd, __filename, __statbuf, __flag);
    }
    #[inline]
    #[c2rust::src_loc = "459:1"]
    pub unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
                                   mut __statbuf: *mut stat) -> libc::c_int {
        return __lxstat(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "508:1"]
    pub unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
                                     mut __statbuf: *mut stat64)
     -> libc::c_int {
        return __lxstat64(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "482:1"]
    pub unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
                                   mut __mode: __mode_t, mut __dev: __dev_t)
     -> libc::c_int {
        return __xmknod(_MKNOD_VER, __path, __mode, &mut __dev);
    }
    #[c2rust::src_loc = "390:10"]
    pub const _MKNOD_VER: libc::c_int = 0 as libc::c_int;
    #[inline]
    #[c2rust::src_loc = "490:1"]
    pub unsafe extern "C" fn mknodat(mut __fd: libc::c_int,
                                     mut __path: *const libc::c_char,
                                     mut __mode: __mode_t, mut __dev: __dev_t)
     -> libc::c_int {
        return __xmknodat(_MKNOD_VER, __fd, __path, __mode, &mut __dev);
    }
    use super::stat_h::{stat, _STAT_VER_LINUX, _STAT_VER, stat64};
    use super::types_h::{__mode_t, __dev_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "409:1"]
        pub fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
                       __stat_buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "406:1"]
        pub fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                        __stat_buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "430:1"]
        pub fn __xstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                         __stat_buf: *mut stat64) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "428:1"]
        pub fn __fxstat64(__ver: libc::c_int, __fildes: libc::c_int,
                          __stat_buf: *mut stat64) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn __fxstatat(__ver: libc::c_int, __fildes: libc::c_int,
                          __filename: *const libc::c_char,
                          __stat_buf: *mut stat, __flag: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "434:1"]
        pub fn __fxstatat64(__ver: libc::c_int, __fildes: libc::c_int,
                            __filename: *const libc::c_char,
                            __stat_buf: *mut stat64, __flag: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "412:1"]
        pub fn __lxstat(__ver: libc::c_int, __filename: *const libc::c_char,
                        __stat_buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn __lxstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                          __stat_buf: *mut stat64) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "438:1"]
        pub fn __xmknod(__ver: libc::c_int, __path: *const libc::c_char,
                        __mode: __mode_t, __dev: *mut __dev_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "441:1"]
        pub fn __xmknodat(__ver: libc::c_int, __fd: libc::c_int,
                          __path: *const libc::c_char, __mode: __mode_t,
                          __dev: *mut __dev_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/ioctl.h:17"]
pub mod ioctl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "64:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "122:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "130:14"]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "137:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "140:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "226:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "336:14"]
        pub fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "397:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:17"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:17"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t)
     -> __uint64_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t)
     -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t)
     -> __uint16_t {
        return __x;
    }
    use super::types_h::{__uint64_t, __uint32_t, __uint16_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:17"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "69:15"]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong &
                     0xff00000000000000 as libc::c_ulonglong) >>
                    56 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000000000 as libc::c_ulonglong) >>
                        40 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff0000000000 as libc::c_ulonglong) >>
                        24 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff00000000 as libc::c_ulonglong) >>
                        8 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
                    |
                    (__bsx as libc::c_ulonglong &
                         0xff0000 as libc::c_ulonglong) << 24 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                        << 40 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                        << 56 as libc::c_int) as __uint64_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::{__uint64_t, __uint32_t, __uint16_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdio.h:17"]
pub mod bits_stdio_h {
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                                     mut __arg: ::std::ffi::VaList)
     -> libc::c_int {
        return vfprintf(stdout, __fmt, __arg.as_va_list());
    }
    #[inline]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE)
     -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(__fp)
               } else {
                   let fresh0 = (*__fp)._IO_read_ptr;
                   (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
                   *(fresh0 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "72:1"]
    pub unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
        return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(stdin)
               } else {
                   let fresh1 = (*stdin)._IO_read_ptr;
                   (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
                   *(fresh1 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE)
     -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(__fp)
               } else {
                   let fresh2 = (*__fp)._IO_read_ptr;
                   (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
                   *(fresh2 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "81:1"]
    pub unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
        return putc(__c, stdout);
    }
    #[inline]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
                                            mut __stream: *mut FILE)
     -> libc::c_int {
        return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
                      libc::c_int as libc::c_long != 0 {
                   __overflow(__stream, __c as libc::c_uchar as libc::c_int)
               } else {
                   let fresh3 = (*__stream)._IO_write_ptr;
                   (*__stream)._IO_write_ptr =
                       (*__stream)._IO_write_ptr.offset(1);
                   *fresh3 = __c as libc::c_char;
                   *fresh3 as libc::c_uchar as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
                                           mut __stream: *mut FILE)
     -> libc::c_int {
        return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
                      libc::c_int as libc::c_long != 0 {
                   __overflow(__stream, __c as libc::c_uchar as libc::c_int)
               } else {
                   let fresh4 = (*__stream)._IO_write_ptr;
                   (*__stream)._IO_write_ptr =
                       (*__stream)._IO_write_ptr.offset(1);
                   *fresh4 = __c as libc::c_char;
                   *fresh4 as libc::c_uchar as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int)
     -> libc::c_int {
        return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as
                      libc::c_int as libc::c_long != 0 {
                   __overflow(stdout, __c as libc::c_uchar as libc::c_int)
               } else {
                   let fresh5 = (*stdout)._IO_write_ptr;
                   (*stdout)._IO_write_ptr =
                       (*stdout)._IO_write_ptr.offset(1);
                   *fresh5 = __c as libc::c_char;
                   *fresh5 as libc::c_uchar as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "117:1"]
    pub unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                                     mut __n: *mut size_t,
                                     mut __stream: *mut FILE) -> __ssize_t {
        return __getdelim(__lineptr, __n, '\n' as i32, __stream);
    }
    #[inline]
    #[c2rust::src_loc = "127:1"]
    pub unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE)
     -> libc::c_int {
        return ((*__stream)._flags & _IO_EOF_SEEN != 0 as libc::c_int) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "134:1"]
    pub unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE)
     -> libc::c_int {
        return ((*__stream)._flags & _IO_ERR_SEEN != 0 as libc::c_int) as
                   libc::c_int;
    }
    use super::internal::__va_list_tag;
    use super::stdio_h::{vfprintf, stdout, getc, stdin, __uflow, putc,
                         __overflow, __getdelim};
    use super::FILE_h::FILE;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    use super::struct_FILE_h::{_IO_EOF_SEEN, _IO_ERR_SEEN};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdlib-float.h:17"]
pub mod stdlib_float_h {
    #[inline]
    #[c2rust::src_loc = "24:1"]
    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char)
     -> libc::c_double {
        return strtod(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char);
    }
    use super::stdlib_h::strtod;
    use super::stddef_h::NULL;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/stdlib-bsearch.h:17"]
pub mod stdlib_bsearch_h {
    #[inline]
    #[c2rust::src_loc = "19:1"]
    pub unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                                     mut __base: *const libc::c_void,
                                     mut __nmemb: size_t, mut __size: size_t,
                                     mut __compar: __compar_fn_t)
     -> *mut libc::c_void {
        let mut __l: size_t = 0;
        let mut __u: size_t = 0;
        let mut __idx: size_t = 0;
        let mut __p = 0 as *const libc::c_void;
        let mut __comparison: libc::c_int = 0;
        __l = 0 as libc::c_int as size_t;
        __u = __nmemb;
        while __l < __u {
            __idx =
                __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as
                                                       libc::c_ulong);
            __p =
                (__base as
                     *const libc::c_char).offset(__idx.wrapping_mul(__size) as
                                                     isize) as
                    *mut libc::c_void;
            __comparison =
                Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                       __p);
            if __comparison < 0 as libc::c_int {
                __u = __idx
            } else if __comparison > 0 as libc::c_int {
                __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else { return __p as *mut libc::c_void }
        }
        return NULL as *mut libc::c_void;
    }
    use super::stddef_h::{size_t, NULL};
    use super::stdlib_h::__compar_fn_t;
}
#[c2rust::header_src = "/usr/include/fcntl.h:17"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:17"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
                   *(*__ctype_tolower_loc()).offset(__c as isize)
               } else { __c };
    }
    #[inline]
    #[c2rust::src_loc = "212:1"]
    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
                   *(*__ctype_toupper_loc()).offset(__c as isize)
               } else { __c };
    }
    use super::types_h::__int32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:17"]
pub mod errno_h {
    #[c2rust::src_loc = "38:10"]
    pub const errno: libc::c_int = *__errno_location();
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/linux/sockios.h:17"]
pub mod sockios_h {
    #[c2rust::src_loc = "57:9"]
    pub const SIOCGIFNAME: libc::c_int = 0x8910 as libc::c_int;
    #[c2rust::src_loc = "74:9"]
    pub const SIOCGIFMTU: libc::c_int = 0x8921 as libc::c_int;
    #[c2rust::src_loc = "60:9"]
    pub const SIOCGIFFLAGS: libc::c_int = 0x8913 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/socket.h:17"]
pub mod asm_generic_socket_h {
    #[c2rust::src_loc = "42:9"]
    pub const SO_BINDTODEVICE: libc::c_int = 25 as libc::c_int;
    #[c2rust::src_loc = "9:9"]
    pub const SOL_SOCKET: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "12:9"]
    pub const SO_REUSEADDR: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno-base.h:17"]
pub mod errno_base_h {
    #[c2rust::src_loc = "17:9"]
    pub const EACCES: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "16:9"]
    pub const ENOMEM: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const EINVAL: libc::c_int = 22 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno.h:17"]
pub mod asm_generic_errno_h {
    #[c2rust::src_loc = "81:9"]
    pub const EADDRINUSE: libc::c_int = 98 as libc::c_int;
    #[c2rust::src_loc = "75:9"]
    pub const ENOPROTOOPT: libc::c_int = 92 as libc::c_int;
    #[c2rust::src_loc = "80:9"]
    pub const EAFNOSUPPORT: libc::c_int = 97 as libc::c_int;
    #[c2rust::src_loc = "76:9"]
    pub const EPROTONOSUPPORT: libc::c_int = 93 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/fcntl-linux.h:17"]
pub mod fcntl_linux_h {
    #[c2rust::src_loc = "62:10"]
    pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
    #[c2rust::src_loc = "171:9"]
    pub const F_SETFL: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "170:9"]
    pub const F_GETFL: libc::c_int = 3 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:17"]
pub mod dns_protocol_h {
    #[c2rust::src_loc = "26:9"]
    pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:17"]
pub mod syslog_h {
    #[c2rust::src_loc = "54:9"]
    pub const LOG_ERR: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const LOG_INFO: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "58:9"]
    pub const LOG_DEBUG: libc::c_int = 7 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/config.h:17"]
pub mod config_h {
    #[c2rust::src_loc = "30:9"]
    pub const SERVERS_LOGGED: libc::c_int = 30 as libc::c_int;
    #[c2rust::src_loc = "31:9"]
    pub const LOCALS_LOGGED: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "21:9"]
    pub const TCP_BACKLOG: libc::c_int = 32 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/netinet/tcp.h:17"]
pub mod tcp_h {
    #[c2rust::src_loc = "62:9"]
    pub const TCP_FASTOPEN: libc::c_int = 23 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/radv-protocol.h:17"]
pub mod radv_protocol_h {
    #[c2rust::src_loc = "18:9"]
    pub const ALL_ROUTERS: [libc::c_char; 8] =
        unsafe {
            *::std::mem::transmute::<&[u8; 8],
                                     &[libc::c_char; 8]>(b"FF02::2\x00")
        };
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dhcp6-protocol.h:17"]
pub mod dhcp6_protocol_h {
    #[c2rust::src_loc = "20:9"]
    pub const ALL_SERVERS: [libc::c_char; 10] =
        unsafe {
            *::std::mem::transmute::<&[u8; 10],
                                     &[libc::c_char; 10]>(b"FF05::1:3\x00")
        };
    #[c2rust::src_loc = "21:9"]
    pub const ALL_RELAY_AGENTS_AND_SERVERS: [libc::c_char; 10] =
        unsafe {
            *::std::mem::transmute::<&[u8; 10],
                                     &[libc::c_char; 10]>(b"FF02::1:2\x00")
        };
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __intmax_t, __uintmax_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __ino64_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __pid_t, __time_t, __blksize_t,
                        __blkcnt_t, __blkcnt64_t, __ssize_t,
                        __syscall_slong_t, __caddr_t, __socklen_t};
pub use self::sys_types_h::{ino_t, dev_t, off_t, pid_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, sockaddr, msghdr, cmsghdr, __cmsg_nxthdr,
                         PF_INET6, AF_INET6, PF_INET, AF_INET};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM,
                              SOCK_DGRAM_0, SOCK_STREAM_0};
pub use self::sockaddr_h::sa_family_t;
pub use self::sys_socket_h::{__CONST_SOCKADDR_ARG, sockaddr_x25, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, getsockopt,
                             setsockopt, listen};
pub use self::un_h::sockaddr_un;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, ipv6_mreq, in6_pktinfo,
                     IPPROTO_IPV6_0, IPPROTO_IP_0, INADDR_ANY, IPPROTO_TCP_0,
                     INET6_ADDRSTRLEN, in6addr_any};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::bits_in_h::{in_pktinfo, IPV6_UNICAST_IF, IP_UNICAST_IF,
                          IPV6_V6ONLY, IPV6_2292PKTINFO, IPV6_RECVPKTINFO,
                          IPV6_PKTINFO, IP_PKTINFO, IPV6_2292PKTOPTIONS,
                          IP_PKTOPTIONS, IPV6_JOIN_GROUP};
pub use self::dnsmasq_h::{u32_0, all_addr, C2RustUnnamed_4, C2RustUnnamed_5,
                          blockdata, C2RustUnnamed_6, C2RustUnnamed_7,
                          C2RustUnnamed_8, C2RustUnnamed_9, crec,
                          C2RustUnnamed_10, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_netid, dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_11, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, EC_BADNET,
                          SERV_NO_REBIND, SERV_USE_RESOLV, SERV_NO_ADDR,
                          SERV_LITERAL_ADDRESS, SERV_MARK, SERV_FROM_RESOLV,
                          SERV_HAS_DOMAIN, SERV_LOOP, SERV_FOR_NODOTS,
                          IFACE_TENTATIVE, ADDRLIST_REVONLY, IFACE_DEPRECATED,
                          IFACE_PERMANENT, ADDRLIST_IPV6, AUTH6, AUTH4,
                          ADDRLIST_LITERAL, ADDRSTRLEN, private_net, rand16,
                          rand32, safe_malloc, safe_strncpy, whine_malloc,
                          sa_len, sockaddr_isequal, hostname_isequal,
                          prettyprint_addr, wildcard_match, die, my_syslog,
                          server_gone, loop_send_probes, iface_enumerate,
                          lease_find_interfaces, dhcp_construct_contexts};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER};
pub use self::if_h::{C2RustUnnamed_1, IFF_DYNAMIC, IFF_AUTOMEDIA, IFF_PORTSEL,
                     IFF_MULTICAST, IFF_SLAVE, IFF_MASTER, IFF_ALLMULTI,
                     IFF_PROMISC, IFF_NOARP, IFF_RUNNING, IFF_NOTRAILERS,
                     IFF_POINTOPOINT, IFF_LOOPBACK, IFF_DEBUG, IFF_BROADCAST,
                     IFF_UP, ifmap, ifreq, C2RustUnnamed_2, C2RustUnnamed_3,
                     IF_NAMESIZE, IFF_LOOPBACK_0, if_nametoindex};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll, free};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
use self::stdio_h::{stdin, stdout, fclose, fopen, sprintf, vfprintf, getc,
                    putc, fgets, __getdelim, __uflow, __overflow};
use self::inet_h::{inet_addr, inet_pton, inet_ntop};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::ioctl_h::ioctl;
use self::string_h::{memset, memcmp, strcpy, strcat, strcmp, strncmp, strchr,
                     strtok, strlen, strerror};
use self::unistd_h::close;
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
use self::fcntl_h::fcntl;
pub use self::ctype_h::{tolower, toupper, __ctype_tolower_loc,
                        __ctype_toupper_loc};
pub use self::errno_h::{errno, __errno_location};
pub use self::sockios_h::{SIOCGIFNAME, SIOCGIFMTU, SIOCGIFFLAGS};
pub use self::asm_generic_socket_h::{SO_BINDTODEVICE, SOL_SOCKET,
                                     SO_REUSEADDR};
pub use self::errno_base_h::{EACCES, ENOMEM, EINVAL};
pub use self::asm_generic_errno_h::{EADDRINUSE, ENOPROTOOPT, EAFNOSUPPORT,
                                    EPROTONOSUPPORT};
pub use self::fcntl_linux_h::{O_NONBLOCK, F_SETFL, F_GETFL};
pub use self::dns_protocol_h::MAXDNAME;
pub use self::syslog_h::{LOG_ERR, LOG_INFO, LOG_WARNING, LOG_DEBUG};
pub use self::config_h::{SERVERS_LOGGED, LOCALS_LOGGED, TCP_BACKLOG};
pub use self::tcp_h::TCP_FASTOPEN;
pub use self::radv_protocol_h::ALL_ROUTERS;
pub use self::dhcp6_protocol_h::{ALL_SERVERS, ALL_RELAY_AGENTS_AND_SERVERS};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "226:8"]
pub struct iface_param {
    pub spare: *mut addrlist,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "914:17"]
pub union C2RustUnnamed_12 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "883:3"]
pub union C2RustUnnamed_13 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in_pktinfo,
}
/* dnsmasq is Copyright (c) 2000-2021 Simon Kelley

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; version 2 dated June, 1991, or
   (at your option) version 3 dated 29 June, 2007.
 
   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.
     
   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
#[no_mangle]
#[c2rust::src_loc = "21:1"]
pub unsafe extern "C" fn indextoname(mut fd: libc::c_int,
                                     mut index: libc::c_int,
                                     mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut ifr =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    if index == 0 as libc::c_int { return 0 as libc::c_int }
    ifr.ifr_ifru.ifru_ivalue = index;
    if ioctl(fd, SIOCGIFNAME as libc::c_ulong, &mut ifr as *mut ifreq) ==
           -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    safe_strncpy(name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 IF_NAMESIZE as size_t);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn iface_check(mut family: libc::c_int,
                                     mut addr: *mut all_addr,
                                     mut name: *mut libc::c_char,
                                     mut auth: *mut libc::c_int)
 -> libc::c_int {
    let mut tmp = 0 as *mut iname;
    let mut ret = 1 as libc::c_int;
    let mut match_addr = 0 as libc::c_int;
    /* Note: have to check all and not bail out early, so that we set the
     "used" flags.

     May be called with family == AF_LOCALto check interface by name only. */
    if !auth.is_null() { *auth = 0 as libc::c_int }
    if !(*dnsmasq_daemon).if_names.is_null() ||
           !(*dnsmasq_daemon).if_addrs.is_null() {
        ret = 0 as libc::c_int;
        tmp = (*dnsmasq_daemon).if_names;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name, name) != 0 {
                (*tmp).used = 1 as libc::c_int;
                ret = (*tmp).used
            }
            tmp = (*tmp).next
        }
        if !addr.is_null() {
            tmp = (*dnsmasq_daemon).if_addrs;
            while !tmp.is_null() {
                if (*tmp).addr.sa.sa_family as libc::c_int == family {
                    if family == AF_INET &&
                           (*tmp).addr.in_0.sin_addr.s_addr ==
                               (*addr).addr4.s_addr {
                        (*tmp).used = 1 as libc::c_int;
                        match_addr = (*tmp).used;
                        ret = match_addr
                    } else if family == AF_INET6 &&
                                  ({
                                       let mut __a =
                                           &mut (*tmp).addr.in6.sin6_addr as
                                               *mut in6_addr as
                                               *const in6_addr;
                                       let mut __b =
                                           &mut (*addr).addr6 as *mut in6_addr
                                               as *const in6_addr;
                                       ((*__a).__in6_u.__u6_addr32[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                            ==
                                            (*__b).__in6_u.__u6_addr32[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                ==
                                                (*__b).__in6_u.__u6_addr32[1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                ==
                                                (*__b).__in6_u.__u6_addr32[2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[3 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                ==
                                                (*__b).__in6_u.__u6_addr32[3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize])
                                           as libc::c_int
                                   }) != 0 {
                        (*tmp).used = 1 as libc::c_int;
                        match_addr = (*tmp).used;
                        ret = match_addr
                    }
                }
                tmp = (*tmp).next
            }
        }
    }
    if match_addr == 0 {
        tmp = (*dnsmasq_daemon).if_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name, name) != 0 {
                ret = 0 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    tmp = (*dnsmasq_daemon).authinterface;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() {
            if strcmp((*tmp).name, name) == 0 as libc::c_int &&
                   ((*tmp).addr.sa.sa_family as libc::c_int ==
                        0 as libc::c_int ||
                        (*tmp).addr.sa.sa_family as libc::c_int == family) {
                break ;
            }
        } else {
            if !addr.is_null() &&
                   (*tmp).addr.sa.sa_family as libc::c_int == AF_INET &&
                   family == AF_INET &&
                   (*tmp).addr.in_0.sin_addr.s_addr == (*addr).addr4.s_addr {
                break ;
            }
            if !addr.is_null() &&
                   (*tmp).addr.sa.sa_family as libc::c_int == AF_INET6 &&
                   family == AF_INET6 &&
                   ({
                        let mut __a =
                            &mut (*tmp).addr.in6.sin6_addr as *mut in6_addr as
                                *const in6_addr;
                        let mut __b =
                            &mut (*addr).addr6 as *mut in6_addr as
                                *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] &&
                             (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] &&
                             (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] &&
                             (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize]) as
                            libc::c_int
                    }) != 0 {
                break ;
            }
        }
        tmp = (*tmp).next
    }
    if !tmp.is_null() && !auth.is_null() {
        *auth = 1 as libc::c_int;
        ret = 1 as libc::c_int
    }
    return ret;
}
/* Fix for problem that the kernel sometimes reports the loopback interface as the
   arrival interface when a packet originates locally, even when sent to address of 
   an interface other than the loopback. Accept packet if it arrived via a loopback 
   interface, even when we're not accepting packets that way, as long as the destination
   address is one we're believing. Interface list must be up-to-date before calling. */
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn loopback_exception(mut fd: libc::c_int,
                                            mut family: libc::c_int,
                                            mut addr: *mut all_addr,
                                            mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut ifr =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut iface = 0 as *mut irec;
    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                 IF_NAMESIZE as size_t);
    if ioctl(fd, SIOCGIFFLAGS as libc::c_ulong, &mut ifr as *mut ifreq) !=
           -(1 as libc::c_int) &&
           ifr.ifr_ifru.ifru_flags as libc::c_int & IFF_LOOPBACK_0 != 0 {
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).addr.sa.sa_family as libc::c_int == family {
                if family == AF_INET {
                    if (*iface).addr.in_0.sin_addr.s_addr ==
                           (*addr).addr4.s_addr {
                        return 1 as libc::c_int
                    }
                } else if ({
                               let mut __a =
                                   &mut (*iface).addr.in6.sin6_addr as
                                       *mut in6_addr as *const in6_addr;
                               let mut __b =
                                   &mut (*addr).addr6 as *mut in6_addr as
                                       *const in6_addr;
                               ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                               usize] ==
                                    (*__b).__in6_u.__u6_addr32[0 as
                                                                   libc::c_int
                                                                   as usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[1 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[2 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[3 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[3 as
                                                                       libc::c_int
                                                                       as
                                                                       usize])
                                   as libc::c_int
                           }) != 0 {
                    return 1 as libc::c_int
                }
            }
            iface = (*iface).next
        }
    }
    return 0 as libc::c_int;
}
/* If we're configured with something like --interface=eth0:0 then we'll listen correctly
   on the relevant address, but the name of the arrival interface, derived from the
   index won't match the config. Check that we found an interface address for the arrival 
   interface: daemon->interfaces must be up-to-date. */
#[no_mangle]
#[c2rust::src_loc = "210:1"]
pub unsafe extern "C" fn label_exception(mut index: libc::c_int,
                                         mut family: libc::c_int,
                                         mut addr: *mut all_addr)
 -> libc::c_int {
    let mut iface = 0 as *mut irec;
    /* labels only supported on IPv4 addresses. */
    if family != AF_INET { return 0 as libc::c_int }
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).index == index &&
               (*iface).addr.sa.sa_family as libc::c_int == AF_INET &&
               (*iface).addr.in_0.sin_addr.s_addr == (*addr).addr4.s_addr {
            return 1 as libc::c_int
        }
        iface = (*iface).next
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn iface_allowed(mut param: *mut iface_param,
                                   mut if_index: libc::c_int,
                                   mut label: *mut libc::c_char,
                                   mut addr: *mut mysockaddr,
                                   mut netmask: in_addr,
                                   mut prefixlen: libc::c_int,
                                   mut iface_flags: libc::c_int)
 -> libc::c_int {
    let mut iface = 0 as *mut irec;
    let mut mtu = 0 as libc::c_int;
    let mut loopback: libc::c_int = 0;
    let mut ifr =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut tftp_ok =
        ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                        libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                                       as usize] &
             (1 as libc::c_uint) <<
                 (40 as libc::c_int as
                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       as
                                                       libc::c_ulong).wrapping_mul(8
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
             != 0) as libc::c_int;
    let mut dhcp_ok = 1 as libc::c_int;
    let mut auth_dns = 0 as libc::c_int;
    let mut is_label = 0 as libc::c_int;
    let mut tmp = 0 as *mut iname;
    if indextoname((*param).fd, if_index, ifr.ifr_ifrn.ifrn_name.as_mut_ptr())
           == 0 ||
           ioctl((*param).fd, SIOCGIFFLAGS as libc::c_ulong,
                 &mut ifr as *mut ifreq) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    loopback = ifr.ifr_ifru.ifru_flags as libc::c_int & IFF_LOOPBACK_0;
    if loopback != 0 { dhcp_ok = 0 as libc::c_int }
    if ioctl((*param).fd, SIOCGIFMTU as libc::c_ulong, &mut ifr as *mut ifreq)
           != -(1 as libc::c_int) {
        mtu = ifr.ifr_ifru.ifru_mtu
    }
    if label.is_null() {
        label = ifr.ifr_ifrn.ifrn_name.as_mut_ptr()
    } else { is_label = strcmp(label, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) }
    /* maintain a list of all addresses on all interfaces for --local-service option */
    if (*dnsmasq_daemon).options[(49 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (49 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut al = 0 as *mut addrlist;
        if !(*param).spare.is_null() {
            al = (*param).spare;
            (*param).spare = (*al).next
        } else {
            al =
                whine_malloc(::std::mem::size_of::<addrlist>() as
                                 libc::c_ulong) as *mut addrlist
        }
        if !al.is_null() {
            (*al).next = (*dnsmasq_daemon).interface_addrs;
            (*dnsmasq_daemon).interface_addrs = al;
            (*al).prefixlen = prefixlen;
            if (*addr).sa.sa_family as libc::c_int == AF_INET {
                (*al).addr.addr4 = (*addr).in_0.sin_addr;
                (*al).flags = 0 as libc::c_int
            } else {
                (*al).addr.addr6 = (*addr).in6.sin6_addr;
                (*al).flags = ADDRLIST_IPV6
            }
        }
    }
    if (*addr).sa.sa_family as libc::c_int != AF_INET6 ||
           ({
                let mut __a =
                    &mut (*addr).in6.sin6_addr as *mut in6_addr as
                        *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) == 0 {
        let mut int_name = 0 as *mut interface_name;
        let mut al_0 = 0 as *mut addrlist;
        let mut zone = 0 as *mut auth_zone;
        let mut name = 0 as *mut auth_name_list;
        /* Find subnets in auth_zones */
        zone = (*dnsmasq_daemon).auth_zones;
        while !zone.is_null() {
            name = (*zone).interface_names;
            while !name.is_null() {
                if wildcard_match((*name).name, label) != 0 {
                    if (*addr).sa.sa_family as libc::c_int == AF_INET &&
                           (*name).flags & AUTH4 != 0 {
                        if !(*param).spare.is_null() {
                            al_0 = (*param).spare;
                            (*param).spare = (*al_0).next
                        } else {
                            al_0 =
                                whine_malloc(::std::mem::size_of::<addrlist>()
                                                 as libc::c_ulong) as
                                    *mut addrlist
                        }
                        if !al_0.is_null() {
                            (*al_0).next = (*zone).subnet;
                            (*zone).subnet = al_0;
                            (*al_0).prefixlen = prefixlen;
                            (*al_0).addr.addr4 = (*addr).in_0.sin_addr;
                            (*al_0).flags = 0 as libc::c_int
                        }
                    }
                    if (*addr).sa.sa_family as libc::c_int == AF_INET6 &&
                           (*name).flags & AUTH6 != 0 {
                        if !(*param).spare.is_null() {
                            al_0 = (*param).spare;
                            (*param).spare = (*al_0).next
                        } else {
                            al_0 =
                                whine_malloc(::std::mem::size_of::<addrlist>()
                                                 as libc::c_ulong) as
                                    *mut addrlist
                        }
                        if !al_0.is_null() {
                            (*al_0).next = (*zone).subnet;
                            (*zone).subnet = al_0;
                            (*al_0).prefixlen = prefixlen;
                            (*al_0).addr.addr6 = (*addr).in6.sin6_addr;
                            (*al_0).flags = ADDRLIST_IPV6
                        }
                    }
                }
                name = (*name).next
            }
            zone = (*zone).next
        }
        /* Update addresses from interface_names. These are a set independent
	 of the set we're listening on. */
        int_name = (*dnsmasq_daemon).int_names;
        while !int_name.is_null() {
            if strncmp(label, (*int_name).intr, IF_NAMESIZE as libc::c_ulong)
                   == 0 as libc::c_int &&
                   ((*addr).sa.sa_family as libc::c_int == (*int_name).family
                        || (*int_name).family == 0 as libc::c_int) {
                if !(*param).spare.is_null() {
                    al_0 = (*param).spare;
                    (*param).spare = (*al_0).next
                } else {
                    al_0 =
                        whine_malloc(::std::mem::size_of::<addrlist>() as
                                         libc::c_ulong) as *mut addrlist
                }
                if !al_0.is_null() {
                    (*al_0).next = (*int_name).addr;
                    (*int_name).addr = al_0;
                    if (*addr).sa.sa_family as libc::c_int == AF_INET {
                        (*al_0).addr.addr4 = (*addr).in_0.sin_addr;
                        (*al_0).flags = 0 as libc::c_int
                    } else {
                        (*al_0).addr.addr6 = (*addr).in6.sin6_addr;
                        (*al_0).flags = ADDRLIST_IPV6;
                        /* Privacy addresses and addresses still undergoing DAD and deprecated addresses
		       don't appear in forward queries, but will in reverse ones. */
                        if iface_flags & IFACE_PERMANENT == 0 ||
                               iface_flags &
                                   (IFACE_DEPRECATED | IFACE_TENTATIVE) != 0 {
                            (*al_0).flags |= ADDRLIST_REVONLY
                        }
                    }
                }
            }
            int_name = (*int_name).next
        }
    }
    /* check whether the interface IP has been added already 
     we call this routine multiple times. */
    iface = (*dnsmasq_daemon).interfaces; /* for garbage collection */
    while !iface.is_null() {
        if sockaddr_isequal(&mut (*iface).addr, addr) != 0 &&
               (*iface).index == if_index {
            (*iface).dad =
                (iface_flags & IFACE_TENTATIVE != 0) as libc::c_int;
            (*iface).found = 1 as libc::c_int;
            (*iface).netmask = netmask;
            return 1 as libc::c_int
        }
        iface = (*iface).next
    }
    /* If we are restricting the set of interfaces to use, make
     sure that loopback interfaces are in that set. */
    if !(*dnsmasq_daemon).if_names.is_null() && loopback != 0 {
        let mut lo = 0 as *mut iname;
        lo = (*dnsmasq_daemon).if_names;
        while !lo.is_null() {
            if !(*lo).name.is_null() &&
                   strcmp((*lo).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) ==
                       0 as libc::c_int {
                break ;
            }
            lo = (*lo).next
        }
        if lo.is_null() &&
               {
                   lo =
                       whine_malloc(::std::mem::size_of::<iname>() as
                                        libc::c_ulong) as *mut iname;
                   !lo.is_null()
               } {
            (*lo).name =
                whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                    as *mut libc::c_char;
            if !(*lo).name.is_null() {
                strcpy((*lo).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
                (*lo).used = 1 as libc::c_int;
                (*lo).next = (*dnsmasq_daemon).if_names;
                (*dnsmasq_daemon).if_names = lo
            } else { free(lo as *mut libc::c_void); }
        }
    }
    if (*addr).sa.sa_family as libc::c_int == AF_INET &&
           iface_check(AF_INET,
                       &mut (*addr).in_0.sin_addr as *mut in_addr as
                           *mut all_addr, label, &mut auth_dns) == 0 {
        return 1 as libc::c_int
    }
    if (*addr).sa.sa_family as libc::c_int == AF_INET6 &&
           iface_check(AF_INET6,
                       &mut (*addr).in6.sin6_addr as *mut in6_addr as
                           *mut all_addr, label, &mut auth_dns) == 0 {
        return 1 as libc::c_int
    }
    /* No DHCP where we're doing auth DNS. */
    if auth_dns != 0 {
        tftp_ok = 0 as libc::c_int;
        dhcp_ok = 0 as libc::c_int
    } else {
        tmp = (*dnsmasq_daemon).dhcp_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 0 as libc::c_int;
                dhcp_ok = 0 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    if !(*dnsmasq_daemon).tftp_interfaces.is_null() {
        /* dedicated tftp interface list */
        tftp_ok = 0 as libc::c_int;
        tmp = (*dnsmasq_daemon).tftp_interfaces;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 1 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    /* add to list */
    iface =
        whine_malloc(::std::mem::size_of::<irec>() as libc::c_ulong) as
            *mut irec; /* dummy */
    if !iface.is_null() {
        (*iface).addr = *addr;
        (*iface).netmask = netmask;
        (*iface).tftp_ok = tftp_ok;
        (*iface).dhcp_ok = dhcp_ok;
        (*iface).dns_auth = auth_dns;
        (*iface).mtu = mtu;
        (*iface).dad = (iface_flags & IFACE_TENTATIVE != 0) as libc::c_int;
        (*iface).found = 1 as libc::c_int;
        (*iface).warned = 0 as libc::c_int;
        (*iface).multicast_done = (*iface).warned;
        (*iface).done = (*iface).multicast_done;
        (*iface).index = if_index;
        (*iface).label = is_label;
        (*iface).name =
            whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                as *mut libc::c_char;
        if !(*iface).name.is_null() {
            strcpy((*iface).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            (*iface).next = (*dnsmasq_daemon).interfaces;
            (*dnsmasq_daemon).interfaces = iface;
            return 1 as libc::c_int
        }
        free(iface as *mut libc::c_void);
    }
    errno = ENOMEM;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "487:1"]
unsafe extern "C" fn iface_allowed_v6(mut local: *mut in6_addr,
                                      mut prefix: libc::c_int,
                                      mut scope: libc::c_int,
                                      mut if_index: libc::c_int,
                                      mut flags: libc::c_int,
                                      mut preferred: libc::c_int,
                                      mut valid: libc::c_int,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut addr = mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut netmask = in_addr{s_addr: 0,};
    netmask.s_addr = 0 as libc::c_int as in_addr_t;
    /* warning */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = AF_INET6 as sa_family_t;
    addr.in6.sin6_addr = *local;
    addr.in6.sin6_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    /* FreeBSD insists this is zero for non-linklocal addresses */
    if ({
            let mut __a = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                 __bswap_32(0xffc00000 as libc::c_uint) ==
                 __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
        }) != 0 {
        addr.in6.sin6_scope_id = if_index as uint32_t
    } else { addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t }
    return iface_allowed(vparam as *mut iface_param, if_index,
                         NULL_0 as *mut libc::c_char, &mut addr, netmask,
                         prefix, flags);
}
#[c2rust::src_loc = "515:1"]
unsafe extern "C" fn iface_allowed_v4(mut local: in_addr,
                                      mut if_index: libc::c_int,
                                      mut label: *mut libc::c_char,
                                      mut netmask: in_addr,
                                      mut broadcast: in_addr,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut addr = mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut prefix: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    /* warning */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in_0.sin_family = AF_INET as sa_family_t;
    addr.in_0.sin_addr = local;
    addr.in_0.sin_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    /* determine prefix length from netmask */
    prefix = 32 as libc::c_int;
    bit = 1 as libc::c_int;
    while bit as libc::c_uint & __bswap_32(netmask.s_addr) ==
              0 as libc::c_int as libc::c_uint && prefix != 0 as libc::c_int {
        bit = bit << 1 as libc::c_int;
        prefix -= 1
    }
    return iface_allowed(vparam as *mut iface_param, if_index, label,
                         &mut addr, netmask, prefix, 0 as libc::c_int);
}
/*
 * Clean old interfaces no longer found.
 */
#[c2rust::src_loc = "540:1"]
unsafe extern "C" fn clean_interfaces() {
    let mut iface = 0 as *mut irec;
    let mut up: *mut *mut irec = &mut (*dnsmasq_daemon).interfaces;
    iface = *up;
    while !iface.is_null() {
        if (*iface).found == 0 && (*iface).done == 0 {
            *up = (*iface).next;
            free((*iface).name as *mut libc::c_void);
            free(iface as *mut libc::c_void);
        } else { up = &mut (*iface).next }
        iface = *up
    };
}
/* * Release listener if no other interface needs it.
 *
 * @return 1 if released, 0 if still required
 */
#[c2rust::src_loc = "564:1"]
unsafe extern "C" fn release_listener(mut l: *mut listener) -> libc::c_int {
    if (*l).used > 1 as libc::c_int {
        let mut iface = 0 as *mut irec;
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).done != 0 &&
                   sockaddr_isequal(&mut (*l).addr, &mut (*iface).addr) != 0 {
                if (*iface).found != 0 {
                    /* update listener to point to active interface instead */
                    if (*(*l).iface).found == 0 { (*l).iface = iface }
                } else { (*l).used -= 1; (*iface).done = 0 as libc::c_int }
            }
            iface = (*iface).next
        }
        /* Someone is still using this listener, skip its deletion */
        if (*l).used > 0 as libc::c_int { return 0 as libc::c_int }
    }
    if (*(*l).iface).done != 0 {
        let mut port: libc::c_int = 0;
        port =
            prettyprint_addr(&mut (*(*l).iface).addr,
                             (*dnsmasq_daemon).addrbuff);
        my_syslog(LOG_DEBUG,
                  b"stopped listening on %s(#%d): %s port %d\x00" as *const u8
                      as *const libc::c_char, (*(*l).iface).name,
                  (*(*l).iface).index, (*dnsmasq_daemon).addrbuff, port);
        /* In case it ever returns */
        (*(*l).iface).done = 0 as libc::c_int
    }
    if (*l).fd != -(1 as libc::c_int) { close((*l).fd); }
    if (*l).tcpfd != -(1 as libc::c_int) { close((*l).tcpfd); }
    if (*l).tftpfd != -(1 as libc::c_int) { close((*l).tftpfd); }
    free(l as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "612:1"]
pub unsafe extern "C" fn enumerate_interfaces(mut reset: libc::c_int)
 -> libc::c_int {
    static mut spare: *mut addrlist = NULL_0 as *mut addrlist;
    static mut done: libc::c_int = 0 as libc::c_int;
    let mut param = iface_param{spare: 0 as *mut addrlist, fd: 0,};
    let mut errsave: libc::c_int = 0;
    let mut ret = 1 as libc::c_int;
    let mut addr = 0 as *mut addrlist;
    let mut tmp = 0 as *mut addrlist;
    let mut intname = 0 as *mut interface_name;
    let mut iface = 0 as *mut irec;
    let mut zone = 0 as *mut auth_zone;
    /* Do this max once per select cycle  - also inhibits netlink socket use
   in TCP child processes. */
    if reset != 0 { done = 0 as libc::c_int; return 1 as libc::c_int }
    if done != 0 { return 1 as libc::c_int }
    done = 1 as libc::c_int;
    param.fd = socket(PF_INET, SOCK_DGRAM_0, 0 as libc::c_int);
    if param.fd == -(1 as libc::c_int) { return 0 as libc::c_int }
    /* Mark interfaces for garbage collection */
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        (*iface).found = 0 as libc::c_int;
        iface = (*iface).next
    }
    /* remove addresses stored against interface_names */
    intname = (*dnsmasq_daemon).int_names;
    while !intname.is_null() {
        addr = (*intname).addr;
        while !addr.is_null() {
            tmp = (*addr).next;
            (*addr).next = spare;
            spare = addr;
            addr = tmp
        }
        (*intname).addr = NULL_0 as *mut addrlist;
        intname = (*intname).next
    }
    /* Remove list of addresses of local interfaces */
    addr = (*dnsmasq_daemon).interface_addrs;
    while !addr.is_null() {
        tmp = (*addr).next;
        (*addr).next = spare;
        spare = addr;
        addr = tmp
    }
    (*dnsmasq_daemon).interface_addrs = NULL_0 as *mut addrlist;
    /* remove addresses stored against auth_zone subnets, but not 
   ones configured as address literals */
    zone = (*dnsmasq_daemon).auth_zones;
    while !zone.is_null() {
        if !(*zone).interface_names.is_null() {
            let mut up = 0 as *mut *mut addrlist;
            up = &mut (*zone).subnet;
            addr = (*zone).subnet;
            while !addr.is_null() {
                tmp = (*addr).next;
                if (*addr).flags & ADDRLIST_LITERAL != 0 {
                    up = &mut (*addr).next
                } else {
                    *up = (*addr).next;
                    (*addr).next = spare;
                    spare = addr
                }
                addr = tmp
            }
        }
        zone = (*zone).next
    }
    param.spare = spare;
    ret =
        iface_enumerate(AF_INET6,
                        &mut param as *mut iface_param as *mut libc::c_void,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut in6_addr,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(iface_allowed_v6
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut in6_addr,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               *mut libc::c_void)
                                                                                          ->
                                                                                              libc::c_int)));
    if ret != 0 {
        ret =
            iface_enumerate(AF_INET,
                            &mut param as *mut iface_param as
                                *mut libc::c_void,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    in_addr,
                                                                                _:
                                                                                    libc::c_int,
                                                                                _:
                                                                                    *mut libc::c_char,
                                                                                _:
                                                                                    in_addr,
                                                                                _:
                                                                                    in_addr,
                                                                                _:
                                                                                    *mut libc::c_void)
                                                               ->
                                                                   libc::c_int>,
                                                    Option<unsafe extern "C" fn()
                                                               ->
                                                                   libc::c_int>>(Some(iface_allowed_v4
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *mut libc::c_char,
                                                                                                               _:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   *mut libc::c_void)
                                                                                              ->
                                                                                                  libc::c_int)))
    }
    errsave = errno;
    close(param.fd);
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (39 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        /* Garbage-collect listeners listening on addresses that no longer exist.
	 Does nothing when not binding interfaces or for listeners on localhost, 
	 since the ->iface field is NULL. Note that this needs the protections
	 against reentrancy, hence it's here.  It also means there's a possibility,
	 in OPT_CLEVERBIND mode, that at listener will just disappear after
	 a call to enumerate_interfaces, this is checked OK on all calls. */
        let mut l = 0 as *mut listener;
        let mut tmp_0 = 0 as *mut listener;
        let mut up_0 = 0 as *mut *mut listener;
        let mut freed = 0 as libc::c_int;
        up_0 = &mut (*dnsmasq_daemon).listeners;
        l = (*dnsmasq_daemon).listeners;
        while !l.is_null() {
            tmp_0 = (*l).next;
            if (*l).iface.is_null() || (*(*l).iface).found != 0 {
                up_0 = &mut (*l).next
            } else if release_listener(l) != 0 {
                *up_0 = tmp_0;
                freed = 1 as libc::c_int
            }
            l = tmp_0
        }
        if freed != 0 { clean_interfaces(); }
    }
    errno = errsave;
    spare = param.spare;
    return ret;
}
/* set NONBLOCK bit on fd: See Stevens 16.6 */
#[no_mangle]
#[c2rust::src_loc = "735:1"]
pub unsafe extern "C" fn fix_fd(mut fd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, F_GETFL);
    if flags == -(1 as libc::c_int) ||
           fcntl(fd, F_SETFL, flags | O_NONBLOCK) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "746:1"]
unsafe extern "C" fn make_sock(mut addr: *mut mysockaddr,
                               mut type_0: libc::c_int,
                               mut dienow: libc::c_int) -> libc::c_int {
    let mut port: libc::c_int = 0;
    let mut errsave: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut family = (*addr).sa.sa_family as libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut opt = 1 as libc::c_int;
    fd = socket(family, type_0, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        port = 0;
        errsave = 0;
        s = 0 as *mut libc::c_char;
        /* No error if the kernel just doesn't support this IP flavour */
        if errno == EPROTONOSUPPORT || errno == EAFNOSUPPORT ||
               errno == EINVAL {
            return -(1 as libc::c_int)
        }
    } else if !(setsockopt(fd, SOL_SOCKET, SO_REUSEADDR,
                           &mut opt as *mut libc::c_int as
                               *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t) ==
                    -(1 as libc::c_int) || fix_fd(fd) == 0) {
        if !(family == AF_INET6 &&
                 setsockopt(fd, IPPROTO_IPV6_0, IPV6_V6ONLY,
                            &mut opt as *mut libc::c_int as
                                *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as socklen_t) ==
                     -(1 as libc::c_int)) {
            rc =
                bind(fd,
                     __CONST_SOCKADDR_ARG{__sockaddr__:
                                              addr as *mut sockaddr,},
                     sa_len(addr) as socklen_t);
            if !(rc == -(1 as libc::c_int)) {
                if type_0 == SOCK_STREAM_0 {
                    let mut qlen = 5 as libc::c_int;
                    setsockopt(fd, IPPROTO_TCP_0, TCP_FASTOPEN,
                               &mut qlen as *mut libc::c_int as
                                   *const libc::c_void,
                               ::std::mem::size_of::<libc::c_int>() as
                                   libc::c_ulong as socklen_t);
                    if listen(fd, TCP_BACKLOG) == -(1 as libc::c_int) {
                        current_block = 4055993212646746884;
                    } else { current_block = 11459959175219260272; }
                } else if family == AF_INET {
                    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (13 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                        if setsockopt(fd, IPPROTO_IP_0, IP_PKTINFO,
                                      &mut opt as *mut libc::c_int as
                                          *const libc::c_void,
                                      ::std::mem::size_of::<libc::c_int>() as
                                          libc::c_ulong as socklen_t) ==
                               -(1 as libc::c_int) {
                            current_block = 4055993212646746884;
                        } else { current_block = 11459959175219260272; }
                    } else { current_block = 11459959175219260272; }
                } else if set_ipv6pktinfo(fd) == 0 {
                    current_block = 4055993212646746884;
                } else { current_block = 11459959175219260272; }
                match current_block {
                    4055993212646746884 => { }
                    _ => { return fd }
                }
            }
        }
    }
    errsave = errno;
    port = prettyprint_addr(addr, (*dnsmasq_daemon).addrbuff);
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 &&
           (*dnsmasq_daemon).options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
        sprintf((*dnsmasq_daemon).addrbuff,
                b"port %d\x00" as *const u8 as *const libc::c_char, port);
    }
    s =
        b"failed to create listening socket for %s: %s\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    if fd != -(1 as libc::c_int) { close(fd); }
    errno = errsave;
    if dienow != 0 {
        /* failure to bind addresses given by --listen-address at this point
	     is OK if we're doing bind-dynamic */
        if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            die(s, (*dnsmasq_daemon).addrbuff, EC_BADNET);
        }
    } else {
        my_syslog(LOG_WARNING, s, (*dnsmasq_daemon).addrbuff,
                  strerror(errno));
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "826:1"]
pub unsafe extern "C" fn set_ipv6pktinfo(mut fd: libc::c_int) -> libc::c_int {
    let mut opt = 1 as libc::c_int;
    /* The API changed around Linux 2.6.14 but the old ABI is still supported:
     handle all combinations of headers and kernel.
     OpenWrt note that this fixes the problem addressed by your very broken patch. */
    (*dnsmasq_daemon).v6pktinfo = IPV6_PKTINFO;
    if setsockopt(fd, IPPROTO_IPV6_0, IPV6_RECVPKTINFO,
                  &mut opt as *mut libc::c_int as *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) != -(1 as libc::c_int) {
        return 1 as libc::c_int
    } else {
        if errno == ENOPROTOOPT &&
               setsockopt(fd, IPPROTO_IPV6_0, IPV6_2292PKTINFO,
                          &mut opt as *mut libc::c_int as *const libc::c_void,
                          ::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong as socklen_t) !=
                   -(1 as libc::c_int) {
            (*dnsmasq_daemon).v6pktinfo = IPV6_2292PKTINFO;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* Find the interface on which a TCP connection arrived, if possible, or zero otherwise. */
#[no_mangle]
#[c2rust::src_loc = "855:1"]
pub unsafe extern "C" fn tcp_interface(mut fd: libc::c_int,
                                       mut af: libc::c_int) -> libc::c_int {
    /* suppress potential unused warning */
    let mut if_index = 0 as libc::c_int;
    let mut opt = 1 as libc::c_int;
    let mut cmptr = 0 as *mut cmsghdr;
    let mut msg =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut len: socklen_t = 0;
    /* use mshdr so that the CMSDG_* macros are available */
    msg.msg_control = (*dnsmasq_daemon).packet as *mut libc::c_void;
    len = (*dnsmasq_daemon).packet_buff_sz as socklen_t;
    msg.msg_controllen = len as size_t;
    /* we overwrote the buffer... */
    (*dnsmasq_daemon).srv_save = NULL_0 as *mut server;
    if af == AF_INET {
        if setsockopt(fd, IPPROTO_IP_0, IP_PKTINFO,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) != -(1 as libc::c_int) &&
               getsockopt(fd, IPPROTO_IP_0, IP_PKTOPTIONS, msg.msg_control,
                          &mut len) != -(1 as libc::c_int) {
            msg.msg_controllen = len as size_t;
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IP_0 &&
                       (*cmptr).cmsg_type == IP_PKTINFO {
                    let mut p = C2RustUnnamed_13{c: 0 as *mut libc::c_uchar,};
                    p.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    if_index = (*p.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
    } else if set_ipv6pktinfo(fd) != 0 &&
                  getsockopt(fd, IPPROTO_IPV6_0, PKTOPTIONS, msg.msg_control,
                             &mut len) != -(1 as libc::c_int) {
        msg.msg_controllen = len as size_t;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        while !cmptr.is_null() {
            if (*cmptr).cmsg_level == IPPROTO_IPV6_0 &&
                   (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
                let mut p_0 = C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
                p_0.c = (*cmptr).__cmsg_data.as_mut_ptr();
                if_index = (*p_0.p).ipi6_ifindex as libc::c_int
            }
            cmptr = __cmsg_nxthdr(&mut msg, cmptr)
        }
    }
    /* Only the RFC-2292 API has the ability to find the interface for TCP connections,
	 it was removed in RFC-3542 !!!! 

	 Fortunately, Linux kept the 2292 ABI when it moved to 3542. The following code always
	 uses the old ABI, and should work with pre- and post-3542 kernel headers */
    /* Linux */
    return if_index;
}
#[c2rust::src_loc = "902:11"]
pub const PKTOPTIONS: libc::c_int = IPV6_2292PKTOPTIONS;
#[c2rust::src_loc = "929:1"]
unsafe extern "C" fn create_listeners(mut addr: *mut mysockaddr,
                                      mut do_tftp: libc::c_int,
                                      mut dienow: libc::c_int)
 -> *mut listener {
    let mut l = NULL_0 as *mut listener;
    let mut fd = -(1 as libc::c_int);
    let mut tcpfd = -(1 as libc::c_int);
    let mut tftpfd = -(1 as libc::c_int);
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        fd = make_sock(addr, SOCK_DGRAM_0, dienow);
        tcpfd = make_sock(addr, SOCK_STREAM_0, dienow)
    }
    if do_tftp != 0 {
        if (*addr).sa.sa_family as libc::c_int == AF_INET {
            /* port must be restored to DNS port for TCP code */
            let mut save = (*addr).in_0.sin_port as libc::c_short;
            (*addr).in_0.sin_port =
                __bswap_16(69 as libc::c_int as __uint16_t);
            tftpfd = make_sock(addr, SOCK_DGRAM_0, dienow);
            (*addr).in_0.sin_port = save as in_port_t
        } else {
            let mut save_0 = (*addr).in6.sin6_port as libc::c_short;
            (*addr).in6.sin6_port =
                __bswap_16(69 as libc::c_int as __uint16_t);
            tftpfd = make_sock(addr, SOCK_DGRAM_0, dienow);
            (*addr).in6.sin6_port = save_0 as in_port_t
        }
    }
    if fd != -(1 as libc::c_int) || tcpfd != -(1 as libc::c_int) ||
           tftpfd != -(1 as libc::c_int) {
        l =
            safe_malloc(::std::mem::size_of::<listener>() as libc::c_ulong) as
                *mut listener;
        (*l).next = NULL_0 as *mut listener;
        (*l).fd = fd;
        (*l).tcpfd = tcpfd;
        (*l).tftpfd = tftpfd;
        (*l).addr = *addr;
        (*l).used = 1 as libc::c_int;
        (*l).iface = NULL_0 as *mut irec
    }
    return l;
}
#[no_mangle]
#[c2rust::src_loc = "978:1"]
pub unsafe extern "C" fn create_wildcard_listeners() {
    let mut addr = mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut l = 0 as *mut listener;
    let mut l6 = 0 as *mut listener;
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in_0.sin_family = AF_INET as sa_family_t;
    addr.in_0.sin_addr.s_addr = INADDR_ANY as in_addr_t;
    addr.in_0.sin_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    l =
        create_listeners(&mut addr,
                         ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                        as usize] &
                              (1 as libc::c_uint) <<
                                  (40 as libc::c_int as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                              != 0) as libc::c_int, 1 as libc::c_int);
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = AF_INET6 as sa_family_t;
    addr.in6.sin6_addr = in6addr_any;
    addr.in6.sin6_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    l6 =
        create_listeners(&mut addr,
                         ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                        as usize] &
                              (1 as libc::c_uint) <<
                                  (40 as libc::c_int as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                              != 0) as libc::c_int, 1 as libc::c_int);
    if !l.is_null() { (*l).next = l6 } else { l = l6 }
    (*dnsmasq_daemon).listeners = l;
}
#[c2rust::src_loc = "1010:1"]
unsafe extern "C" fn find_listener(mut addr: *mut mysockaddr)
 -> *mut listener {
    let mut l = 0 as *mut listener;
    l = (*dnsmasq_daemon).listeners;
    while !l.is_null() {
        if sockaddr_isequal(&mut (*l).addr, addr) != 0 { return l }
        l = (*l).next
    }
    return NULL_0 as *mut listener;
}
#[no_mangle]
#[c2rust::src_loc = "1019:1"]
pub unsafe extern "C" fn create_bound_listeners(mut dienow: libc::c_int) {
    let mut new = 0 as *mut listener;
    let mut iface = 0 as *mut irec;
    let mut if_tmp = 0 as *mut iname;
    let mut existing = 0 as *mut listener;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).done == 0 && (*iface).dad == 0 && (*iface).found != 0 {
            existing = find_listener(&mut (*iface).addr);
            if !existing.is_null() {
                (*iface).done = 1 as libc::c_int;
                (*existing).used += 1
                /* increase usage counter */
            } else {
                new =
                    create_listeners(&mut (*iface).addr, (*iface).tftp_ok,
                                     dienow);
                if !new.is_null() {
                    (*new).iface = iface;
                    (*new).next = (*dnsmasq_daemon).listeners;
                    (*dnsmasq_daemon).listeners = new;
                    (*iface).done = 1 as libc::c_int;
                    /* Don't log the initial set of listen addresses created
               at startup, since this is happening before the logging
               system is initialised and the sign-on printed. */
                    if dienow == 0 {
                        let mut port =
                            prettyprint_addr(&mut (*iface).addr,
                                             (*dnsmasq_daemon).addrbuff);
                        my_syslog(LOG_DEBUG,
                                  b"listening on %s(#%d): %s port %d\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*iface).name, (*iface).index,
                                  (*dnsmasq_daemon).addrbuff, port);
                    }
                }
            }
        }
        iface = (*iface).next
    }
    /* Check for --listen-address options that haven't been used because there's
     no interface with a matching address. These may be valid: eg it's possible
     to listen on 127.0.1.1 even if the loopback interface is 127.0.0.1

     If the address isn't valid the bind() will fail and we'll die() 
     (except in bind-dynamic mode, when we'll complain but keep trying.)

     The resulting listeners have the ->iface field NULL, and this has to be
     handled by the DNS and TFTP code. It disables --localise-queries processing
     (no netmask) and some MTU login the tftp code. */
    if_tmp = (*dnsmasq_daemon).if_addrs;
    while !if_tmp.is_null() {
        if (*if_tmp).used == 0 &&
               {
                   new =
                       create_listeners(&mut (*if_tmp).addr,
                                        ((*dnsmasq_daemon).options[(40 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_ulong))
                                                                       as
                                                                       usize]
                                             &
                                             (1 as libc::c_uint) <<
                                                 (40 as libc::c_int as
                                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                             != 0) as libc::c_int, dienow);
                   !new.is_null()
               } {
            (*new).next = (*dnsmasq_daemon).listeners;
            (*dnsmasq_daemon).listeners = new;
            if dienow == 0 {
                let mut port_0 =
                    prettyprint_addr(&mut (*if_tmp).addr,
                                     (*dnsmasq_daemon).addrbuff);
                my_syslog(LOG_DEBUG,
                          b"listening on %s port %d\x00" as *const u8 as
                              *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                          port_0);
            }
        }
        if_tmp = (*if_tmp).next
    };
}
/* In --bind-interfaces, the only access control is the addresses we're listening on. 
   There's nothing to avoid a query to the address of an internal interface arriving via
   an external interface where we don't want to accept queries, except that in the usual 
   case the addresses of internal interfaces are RFC1918. When bind-interfaces in use, 
   and we listen on an address that looks like it's probably globally routeable, shout.

   The fix is to use --bind-dynamic, which actually checks the arrival interface too.
   Tough if your platform doesn't support this.

   Note that checking the arrival interface is supported in the standard IPv6 API and
   always done, so we don't warn about any IPv6 addresses here.
*/
#[no_mangle]
#[c2rust::src_loc = "1093:1"]
pub unsafe extern "C" fn warn_bound_listeners() {
    let mut iface = 0 as *mut irec;
    let mut advice = 0 as libc::c_int;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).dns_auth == 0 {
            if (*iface).addr.sa.sa_family as libc::c_int == AF_INET {
                if private_net((*iface).addr.in_0.sin_addr, 1 as libc::c_int)
                       == 0 {
                    inet_ntop(AF_INET,
                              &mut (*iface).addr.in_0.sin_addr as *mut in_addr
                                  as *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              ADDRSTRLEN as socklen_t);
                    advice = 1 as libc::c_int;
                    (*iface).warned = advice;
                    my_syslog(LOG_WARNING,
                              b"LOUD WARNING: listening on %s may accept requests via interfaces other than %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).addrbuff, (*iface).name);
                }
            }
        }
        iface = (*iface).next
    }
    if advice != 0 {
        my_syslog(LOG_WARNING,
                  b"LOUD WARNING: use --bind-dynamic rather than --bind-interfaces to avoid DNS amplification attacks via these interface(s)\x00"
                      as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1118:1"]
pub unsafe extern "C" fn warn_wild_labels() {
    let mut iface = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).found != 0 && !(*iface).name.is_null() &&
               (*iface).label != 0 {
            my_syslog(LOG_WARNING,
                      b"warning: using interface %s instead\x00" as *const u8
                          as *const libc::c_char, (*iface).name);
        }
        iface = (*iface).next
    };
}
#[no_mangle]
#[c2rust::src_loc = "1127:1"]
pub unsafe extern "C" fn warn_int_names() {
    let mut intname = 0 as *mut interface_name;
    intname = (*dnsmasq_daemon).int_names;
    while !intname.is_null() {
        if (*intname).addr.is_null() {
            my_syslog(LOG_WARNING,
                      b"warning: no addresses found for interface %s\x00" as
                          *const u8 as *const libc::c_char, (*intname).intr);
        }
        intname = (*intname).next
    };
}
#[no_mangle]
#[c2rust::src_loc = "1136:1"]
pub unsafe extern "C" fn is_dad_listeners() -> libc::c_int {
    let mut iface = 0 as *mut irec;
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).dad != 0 && (*iface).done == 0 {
                return 1 as libc::c_int
            }
            iface = (*iface).next
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1149:1"]
pub unsafe extern "C" fn join_multicast(mut dienow: libc::c_int) {
    let mut iface = 0 as *mut irec;
    let mut tmp = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).addr.sa.sa_family as libc::c_int == AF_INET6 &&
               (*iface).dhcp_ok != 0 && (*iface).multicast_done == 0 {
            /* There's an irec per address but we only want to join for multicast 
	   once per interface. Weed out duplicates. */
            tmp = (*dnsmasq_daemon).interfaces;
            while !tmp.is_null() {
                if (*tmp).multicast_done != 0 &&
                       (*tmp).index == (*iface).index {
                    break ;
                }
                tmp = (*tmp).next
            }
            (*iface).multicast_done = 1 as libc::c_int;
            if tmp.is_null() {
                let mut mreq =
                    ipv6_mreq{ipv6mr_multiaddr:
                                  in6_addr{__in6_u:
                                               C2RustUnnamed{__u6_addr8:
                                                                 [0; 16],},},
                              ipv6mr_interface: 0,};
                let mut err = 0 as libc::c_int;
                mreq.ipv6mr_interface = (*iface).index as libc::c_uint;
                inet_pton(AF_INET6, ALL_RELAY_AGENTS_AND_SERVERS.as_ptr(),
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if ((*dnsmasq_daemon).doing_dhcp6 != 0 ||
                        !(*dnsmasq_daemon).relay6.is_null()) &&
                       setsockopt((*dnsmasq_daemon).dhcp6fd, IPPROTO_IPV6_0,
                                  IPV6_JOIN_GROUP,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = errno
                }
                inet_pton(AF_INET6, ALL_SERVERS.as_ptr(),
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if (*dnsmasq_daemon).doing_dhcp6 != 0 &&
                       setsockopt((*dnsmasq_daemon).dhcp6fd, IPPROTO_IPV6_0,
                                  IPV6_JOIN_GROUP,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = errno
                }
                inet_pton(AF_INET6, ALL_ROUTERS.as_ptr(),
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if (*dnsmasq_daemon).doing_ra != 0 &&
                       setsockopt((*dnsmasq_daemon).icmp6fd, IPPROTO_IPV6_0,
                                  IPV6_JOIN_GROUP,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = errno
                }
                if err != 0 {
                    let mut s =
                        b"interface %s failed to join DHCPv6 multicast group: %s\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char;
                    errno = err;
                    if errno == ENOMEM {
                        my_syslog(LOG_ERR,
                                  b"try increasing /proc/sys/net/core/optmem_max\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                    if dienow != 0 {
                        die(s, (*iface).name, EC_BADNET);
                    } else {
                        my_syslog(LOG_ERR, s, (*iface).name, strerror(errno));
                    }
                }
            }
        }
        iface = (*iface).next
    };
}
/* return a UDP socket bound to a random port, have to cope with straying into
   occupied port nos and reserved ones. */
#[no_mangle]
#[c2rust::src_loc = "1211:1"]
pub unsafe extern "C" fn random_sock(mut family: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = socket(family, SOCK_DGRAM_0, 0 as libc::c_int);
    if fd != -(1 as libc::c_int) {
        let mut addr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut ports_avail =
            ((*dnsmasq_daemon).max_port as libc::c_ushort as libc::c_int -
                 (*dnsmasq_daemon).min_port as libc::c_ushort as libc::c_int +
                 1 as libc::c_int) as libc::c_uint;
        let mut tries =
            if ports_avail < 30 as libc::c_int as libc::c_uint {
                (3 as libc::c_int as libc::c_uint).wrapping_mul(ports_avail)
            } else { 100 as libc::c_int as libc::c_uint } as libc::c_int;
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.sa.sa_family = family as sa_family_t;
        /* don't loop forever if all ports in use. */
        if fix_fd(fd) != 0 {
            loop  {
                let fresh6 = tries;
                tries = tries - 1;
                if !(fresh6 != 0) { break ; }
                let mut port =
                    __bswap_16(((*dnsmasq_daemon).min_port +
                                    rand16() as libc::c_int %
                                        ports_avail as libc::c_ushort as
                                            libc::c_int) as __uint16_t);
                if family == AF_INET {
                    addr.in_0.sin_addr.s_addr = INADDR_ANY as in_addr_t;
                    addr.in_0.sin_port = port
                } else {
                    addr.in6.sin6_addr = in6addr_any;
                    addr.in6.sin6_port = port
                }
                if bind(fd,
                        __CONST_SOCKADDR_ARG{__sockaddr__:
                                                 &mut addr as *mut mysockaddr
                                                     as *mut sockaddr,},
                        sa_len(&mut addr) as socklen_t) == 0 as libc::c_int {
                    return fd
                }
                if errno != EADDRINUSE && errno != EACCES { break ; }
            }
        }
        close(fd);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1262:1"]
pub unsafe extern "C" fn local_bind(mut fd: libc::c_int,
                                    mut addr: *mut mysockaddr,
                                    mut intname: *mut libc::c_char,
                                    mut ifindex: libc::c_uint,
                                    mut is_tcp: libc::c_int) -> libc::c_int {
    let mut addr_copy = *addr;
    let mut port: libc::c_ushort = 0;
    let mut tries = 1 as libc::c_int;
    let mut done = 0 as libc::c_int;
    let mut ports_avail =
        ((*dnsmasq_daemon).max_port as libc::c_ushort as libc::c_int -
             (*dnsmasq_daemon).min_port as libc::c_ushort as libc::c_int +
             1 as libc::c_int) as libc::c_uint;
    if addr_copy.sa.sa_family as libc::c_int == AF_INET {
        port = addr_copy.in_0.sin_port
    } else { port = addr_copy.in6.sin6_port }
    /* cannot set source _port_ for TCP connections. */
    if is_tcp != 0 { port = 0 as libc::c_int as libc::c_ushort }
    /* Bind a random port within the range given by min-port and max-port */
    if port as libc::c_int == 0 as libc::c_int {
        tries =
            if ports_avail < 30 as libc::c_int as libc::c_uint {
                (3 as libc::c_int as libc::c_uint).wrapping_mul(ports_avail)
            } else { 100 as libc::c_int as libc::c_uint } as libc::c_int;
        port =
            __bswap_16(((*dnsmasq_daemon).min_port +
                            rand16() as libc::c_int %
                                ports_avail as libc::c_ushort as libc::c_int)
                           as __uint16_t)
    }
    loop  {
        let fresh7 = tries;
        tries = tries - 1;
        if !(fresh7 != 0) { break ; }
        if addr_copy.sa.sa_family as libc::c_int == AF_INET {
            addr_copy.in_0.sin_port = port
        } else { addr_copy.in6.sin6_port = port }
        if bind(fd,
                __CONST_SOCKADDR_ARG{__sockaddr__:
                                         &mut addr_copy as *mut mysockaddr as
                                             *mut sockaddr,},
                sa_len(&mut addr_copy) as socklen_t) != -(1 as libc::c_int) {
            done = 1 as libc::c_int;
            break ;
        } else {
            if errno != EADDRINUSE && errno != EACCES {
                return 0 as libc::c_int
            }
            port =
                __bswap_16(((*dnsmasq_daemon).min_port +
                                rand16() as libc::c_int %
                                    ports_avail as libc::c_ushort as
                                        libc::c_int) as __uint16_t)
        }
    }
    if done == 0 { return 0 as libc::c_int }
    if is_tcp == 0 && ifindex > 0 as libc::c_int as libc::c_uint {
        if addr_copy.sa.sa_family as libc::c_int == AF_INET {
            let mut ifindex_opt = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IP_0, IP_UNICAST_IF,
                               &mut ifindex_opt as *mut uint32_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as socklen_t) ==
                        0 as libc::c_int) as libc::c_int
        }
        if addr_copy.sa.sa_family as libc::c_int == AF_INET6 {
            let mut ifindex_opt_0 = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IPV6_0, IPV6_UNICAST_IF,
                               &mut ifindex_opt_0 as *mut uint32_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as socklen_t) ==
                        0 as libc::c_int) as libc::c_int
        }
    }
    /* suppress potential unused warning */
    if *intname.offset(0 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int &&
           setsockopt(fd, SOL_SOCKET, SO_BINDTODEVICE,
                      intname as *const libc::c_void,
                      IF_NAMESIZE as socklen_t) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "1335:1"]
unsafe extern "C" fn allocate_sfd(mut addr: *mut mysockaddr,
                                  mut intname: *mut libc::c_char)
 -> *mut serverfd {
    let mut sfd = 0 as *mut serverfd;
    let mut ifindex = 0 as libc::c_int as libc::c_uint;
    let mut errsave: libc::c_int = 0;
    let mut opt = 1 as libc::c_int;
    /* when using random ports, servers which would otherwise use
     the INADDR_ANY/port0 socket have sfd set to NULL */
    if (*dnsmasq_daemon).osport == 0 &&
           *intname.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
        errno =
            0 as
                libc::c_int; /* index == 0 when not binding to an interface */
        if (*addr).sa.sa_family as libc::c_int == AF_INET &&
               (*addr).in_0.sin_addr.s_addr == INADDR_ANY as in_addr_t &&
               (*addr).in_0.sin_port as libc::c_int ==
                   __bswap_16(0 as libc::c_int as __uint16_t) as libc::c_int {
            return NULL_0 as *mut serverfd
        }
        if (*addr).sa.sa_family as libc::c_int == AF_INET6 &&
               memcmp(&mut (*addr).in6.sin6_addr as *mut in6_addr as
                          *const libc::c_void,
                      &in6addr_any as *const in6_addr as *const libc::c_void,
                      ::std::mem::size_of::<in6_addr>() as libc::c_ulong) ==
                   0 as libc::c_int &&
               (*addr).in6.sin6_port as libc::c_int ==
                   __bswap_16(0 as libc::c_int as __uint16_t) as libc::c_int {
            return NULL_0 as *mut serverfd
        }
    }
    if !intname.is_null() &&
           strlen(intname) != 0 as libc::c_int as libc::c_ulong {
        ifindex = if_nametoindex(intname)
    }
    /* may have a suitable one already */
    sfd = (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        if sockaddr_isequal(&mut (*sfd).source_addr, addr) != 0 &&
               strcmp(intname, (*sfd).interface.as_mut_ptr()) ==
                   0 as libc::c_int && ifindex == (*sfd).ifindex {
            return sfd
        }
        sfd = (*sfd).next
    }
    /* need to make a new one. */
    errno = ENOMEM; /* in case malloc fails. */
    sfd =
        whine_malloc(::std::mem::size_of::<serverfd>() as libc::c_ulong) as
            *mut serverfd; /* save error from bind/setsockopt. */
    if sfd.is_null() { return NULL_0 as *mut serverfd }
    (*sfd).fd =
        socket((*addr).sa.sa_family as libc::c_int, SOCK_DGRAM_0,
               0 as libc::c_int);
    if (*sfd).fd == -(1 as libc::c_int) {
        free(sfd as *mut libc::c_void);
        return NULL_0 as *mut serverfd
    }
    if (*addr).sa.sa_family as libc::c_int == AF_INET6 &&
           setsockopt((*sfd).fd, IPPROTO_IPV6_0, IPV6_V6ONLY,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           local_bind((*sfd).fd, addr, intname, ifindex, 0 as libc::c_int) ==
               0 || fix_fd((*sfd).fd) == 0 {
        errsave = errno;
        close((*sfd).fd);
        free(sfd as *mut libc::c_void);
        errno = errsave;
        return NULL_0 as *mut serverfd
    }
    safe_strncpy((*sfd).interface.as_mut_ptr(), intname,
                 ::std::mem::size_of::<[libc::c_char; 17]>() as
                     libc::c_ulong);
    (*sfd).source_addr = *addr;
    (*sfd).next = (*dnsmasq_daemon).sfds;
    (*sfd).ifindex = ifindex;
    (*sfd).preallocated = 0 as libc::c_int as libc::c_uint;
    (*dnsmasq_daemon).sfds = sfd;
    return sfd;
}
/* create upstream sockets during startup, before root is dropped which may be needed
   this allows query_port to be a low port and interface binding */
#[no_mangle]
#[c2rust::src_loc = "1402:1"]
pub unsafe extern "C" fn pre_allocate_sfds() {
    let mut srv = 0 as *mut server;
    let mut sfd = 0 as *mut serverfd;
    if (*dnsmasq_daemon).query_port != 0 as libc::c_int {
        let mut addr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in_0.sin_family = AF_INET as sa_family_t;
        addr.in_0.sin_addr.s_addr = INADDR_ANY as in_addr_t;
        addr.in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        sfd =
            allocate_sfd(&mut addr,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
        if !sfd.is_null() {
            (*sfd).preallocated = 1 as libc::c_int as libc::c_uint
        }
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in6.sin6_family = AF_INET6 as sa_family_t;
        addr.in6.sin6_addr = in6addr_any;
        addr.in6.sin6_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        sfd =
            allocate_sfd(&mut addr,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
        if !sfd.is_null() {
            (*sfd).preallocated = 1 as libc::c_int as libc::c_uint
        }
    }
    srv = (*dnsmasq_daemon).servers;
    while !srv.is_null() {
        if (*srv).flags &
               (SERV_LITERAL_ADDRESS | SERV_NO_ADDR | SERV_USE_RESOLV |
                    SERV_NO_REBIND) == 0 &&
               allocate_sfd(&mut (*srv).source_addr,
                            (*srv).interface.as_mut_ptr()).is_null() &&
               errno != 0 as libc::c_int &&
               (*dnsmasq_daemon).options[(13 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (13 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
            prettyprint_addr(&mut (*srv).source_addr,
                             (*dnsmasq_daemon).namebuff);
            if (*srv).interface[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                strcat((*dnsmasq_daemon).namebuff,
                       b" \x00" as *const u8 as *const libc::c_char);
                strcat((*dnsmasq_daemon).namebuff,
                       (*srv).interface.as_mut_ptr());
            }
            die(b"failed to bind server socket for %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).namebuff, EC_BADNET);
        }
        srv = (*srv).next
    };
}
#[no_mangle]
#[c2rust::src_loc = "1448:1"]
pub unsafe extern "C" fn mark_servers(mut flag: libc::c_int) {
    let mut serv = 0 as *mut server;
    /* mark everything with argument flag */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags & flag != 0 { (*serv).flags |= SERV_MARK }
        /* Give looped servers another chance */
        (*serv).flags &= !SERV_LOOP;
        serv = (*serv).next
    };
}
#[no_mangle]
#[c2rust::src_loc = "1464:1"]
pub unsafe extern "C" fn cleanup_servers() {
    let mut serv = 0 as *mut server;
    let mut tmp = 0 as *mut server;
    let mut up = 0 as *mut *mut server;
    /* unlink and free anything still marked. */
    serv = (*dnsmasq_daemon).servers;
    up = &mut (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        tmp = (*serv).next;
        if (*serv).flags & SERV_MARK != 0 {
            server_gone(serv);
            *up = (*serv).next;
            if !(*serv).domain.is_null() {
                free((*serv).domain as *mut libc::c_void);
            }
            free(serv as *mut libc::c_void);
        } else { up = &mut (*serv).next }
        serv = tmp
    }
    /* Now we have a new set of servers, test for loops. */
    loop_send_probes();
}
#[no_mangle]
#[c2rust::src_loc = "1490:1"]
pub unsafe extern "C" fn add_update_server(mut flags: libc::c_int,
                                           mut addr: *mut mysockaddr,
                                           mut source_addr: *mut mysockaddr,
                                           mut interface: *const libc::c_char,
                                           mut domain: *const libc::c_char) {
    let mut serv = 0 as *mut server;
    let mut next = NULL_0 as *mut server;
    let mut domain_str = NULL_0 as *mut libc::c_char;
    /* See if there is a suitable candidate, and unmark */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags & SERV_MARK != 0 {
            if !domain.is_null() {
                if !((*serv).flags & SERV_HAS_DOMAIN == 0 ||
                         hostname_isequal(domain, (*serv).domain) == 0) {
                    break ;
                }
            } else if !((*serv).flags & SERV_HAS_DOMAIN != 0) { break ; }
        }
        serv = (*serv).next
    }
    if !serv.is_null() {
        domain_str = (*serv).domain;
        next = (*serv).next
    } else {
        serv =
            whine_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
                *mut server;
        if !serv.is_null() {
            /* Not found, create a new one. */
            if !domain.is_null() &&
                   {
                       domain_str =
                           whine_malloc(strlen(domain).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
                               as *mut libc::c_char;
                       domain_str.is_null()
                   } {
                free(serv as *mut libc::c_void);
                serv = NULL_0 as *mut server
            } else {
                let mut s = 0 as *mut server;
                /* Add to the end of the chain, for order */
                if (*dnsmasq_daemon).servers.is_null() {
                    (*dnsmasq_daemon).servers = serv
                } else {
                    s = (*dnsmasq_daemon).servers;
                    while !(*s).next.is_null() { s = (*s).next }
                    (*s).next = serv
                }
                if !domain.is_null() { strcpy(domain_str, domain); }
            }
        }
    }
    if !serv.is_null() {
        memset(serv as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<server>() as libc::c_ulong);
        (*serv).flags = flags;
        (*serv).domain = domain_str;
        (*serv).next = next;
        (*serv).failed_queries = 0 as libc::c_int as libc::c_uint;
        (*serv).queries = (*serv).failed_queries;
        (*serv).uid = rand32();
        if !domain.is_null() { (*serv).flags |= SERV_HAS_DOMAIN }
        if !interface.is_null() {
            safe_strncpy((*serv).interface.as_mut_ptr(), interface,
                         ::std::mem::size_of::<[libc::c_char; 17]>() as
                             libc::c_ulong);
        }
        if !addr.is_null() { (*serv).addr = *addr }
        if !source_addr.is_null() { (*serv).source_addr = *source_addr }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1569:1"]
pub unsafe extern "C" fn check_servers() {
    let mut iface = 0 as *mut irec;
    let mut serv = 0 as *mut server;
    let mut sfd = 0 as *mut serverfd;
    let mut tmp = 0 as *mut serverfd;
    let mut up = 0 as *mut *mut serverfd;
    let mut port = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut locals = 0 as libc::c_int;
    /* interface may be new since startup */
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        enumerate_interfaces(0 as libc::c_int);
    }
    /* don't garbage collect pre-allocated sfds. */
    sfd = (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        (*sfd).used = (*sfd).preallocated;
        sfd = (*sfd).next
    }
    let mut current_block_37: u64;
    count = 0 as libc::c_int;
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags &
               (SERV_LITERAL_ADDRESS | SERV_NO_ADDR | SERV_USE_RESOLV |
                    SERV_NO_REBIND) == 0 {
            /* Init edns_pktsz for newly created server records. */
            if (*serv).edns_pktsz == 0 as libc::c_int {
                (*serv).edns_pktsz =
                    (*dnsmasq_daemon).edns_pktsz as libc::c_int
            }
            port =
                prettyprint_addr(&mut (*serv).addr,
                                 (*dnsmasq_daemon).namebuff);
            /* 0.0.0.0 is nothing, the stack treats it like 127.0.0.1 */
            if (*serv).addr.sa.sa_family as libc::c_int == AF_INET &&
                   (*serv).addr.in_0.sin_addr.s_addr ==
                       0 as libc::c_int as libc::c_uint {
                (*serv).flags |= SERV_MARK;
                current_block_37 = 8515828400728868193;
            } else {
                iface = (*dnsmasq_daemon).interfaces;
                while !iface.is_null() {
                    if sockaddr_isequal(&mut (*serv).addr, &mut (*iface).addr)
                           != 0 {
                        break ;
                    }
                    iface = (*iface).next
                }
                if !iface.is_null() {
                    my_syslog(LOG_WARNING,
                              b"ignoring nameserver %s - local interface\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).namebuff);
                    (*serv).flags |= SERV_MARK;
                    current_block_37 = 8515828400728868193;
                } else if (*serv).sfd.is_null() &&
                              {
                                  (*serv).sfd =
                                      allocate_sfd(&mut (*serv).source_addr,
                                                   (*serv).interface.as_mut_ptr());
                                  (*serv).sfd.is_null()
                              } && errno != 0 as libc::c_int {
                    my_syslog(LOG_WARNING,
                              b"ignoring nameserver %s - cannot make/bind socket: %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).namebuff, strerror(errno));
                    (*serv).flags |= SERV_MARK;
                    current_block_37 = 8515828400728868193;
                } else {
                    if !(*serv).sfd.is_null() {
                        (*(*serv).sfd).used = 1 as libc::c_int as libc::c_uint
                    }
                    current_block_37 = 3437258052017859086;
                }
            }
        } else { current_block_37 = 3437258052017859086; }
        match current_block_37 {
            3437258052017859086 => {
                if (*serv).flags & SERV_NO_REBIND == 0 &&
                       (*serv).flags & SERV_LITERAL_ADDRESS == 0 {
                    count += 1;
                    if !(count > SERVERS_LOGGED) {
                        if (*serv).flags &
                               (SERV_HAS_DOMAIN | SERV_FOR_NODOTS |
                                    SERV_USE_RESOLV) != 0 {
                            let mut s1 = 0 as *mut libc::c_char;
                            let mut s2 = 0 as *mut libc::c_char;
                            let mut s3 =
                                b"\x00" as *const u8 as *const libc::c_char as
                                    *mut libc::c_char;
                            if (*serv).flags & SERV_HAS_DOMAIN == 0 {
                                s1 =
                                    b"unqualified\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 =
                                    b"names\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            } else if strlen((*serv).domain) ==
                                          0 as libc::c_int as libc::c_ulong {
                                s1 =
                                    b"default\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 =
                                    b"\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            } else {
                                s1 =
                                    b"domain\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 = (*serv).domain
                            }
                            if (*serv).flags & SERV_NO_ADDR != 0 {
                                count -= 1;
                                locals += 1;
                                if locals <= LOCALS_LOGGED {
                                    my_syslog(LOG_INFO,
                                              b"using only locally-known addresses for %s %s\x00"
                                                  as *const u8 as
                                                  *const libc::c_char, s1,
                                              s2);
                                }
                            } else if (*serv).flags & SERV_USE_RESOLV != 0 {
                                my_syslog(LOG_INFO,
                                          b"using standard nameservers for %s %s\x00"
                                              as *const u8 as
                                              *const libc::c_char, s1, s2);
                            } else {
                                my_syslog(LOG_INFO,
                                          b"using nameserver %s#%d for %s %s %s\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*dnsmasq_daemon).namebuff, port,
                                          s1, s2, s3);
                            }
                        } else if (*serv).flags & SERV_LOOP != 0 {
                            my_syslog(LOG_INFO,
                                      b"NOT using nameserver %s#%d - query loop detected\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port);
                        } else if (*serv).interface[0 as libc::c_int as usize]
                                      as libc::c_int != 0 as libc::c_int {
                            my_syslog(LOG_INFO,
                                      b"using nameserver %s#%d(via %s)\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port,
                                      (*serv).interface.as_mut_ptr());
                        } else {
                            my_syslog(LOG_INFO,
                                      b"using nameserver %s#%d\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port);
                        }
                    }
                }
            }
            _ => { }
        }
        serv = (*serv).next
    }
    if locals > LOCALS_LOGGED {
        my_syslog(LOG_INFO,
                  b"using %d more local addresses\x00" as *const u8 as
                      *const libc::c_char, locals - LOCALS_LOGGED);
    }
    if count - 1 as libc::c_int > SERVERS_LOGGED {
        my_syslog(LOG_INFO,
                  b"using %d more nameservers\x00" as *const u8 as
                      *const libc::c_char,
                  count - SERVERS_LOGGED - 1 as libc::c_int);
    }
    /* Do we need a socket set? */
    /* Remove unused sfds */
    sfd = (*dnsmasq_daemon).sfds;
    up = &mut (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        tmp = (*sfd).next;
        if (*sfd).used == 0 {
            *up = (*sfd).next;
            close((*sfd).fd);
            free(sfd as *mut libc::c_void);
        } else { up = &mut (*sfd).next }
        sfd = tmp
    }
    cleanup_servers();
}
/* Return zero if no servers found, in that case we keep polling.
   This is a protection against an update-time/write race on resolv.conf */
#[no_mangle]
#[c2rust::src_loc = "1721:1"]
pub unsafe extern "C" fn reload_servers(mut fname: *mut libc::c_char)
 -> libc::c_int {
    let mut f = 0 as *mut FILE;
    let mut line = 0 as *mut libc::c_char;
    let mut gotone = 0 as libc::c_int;
    /* buff happens to be MAXDNAME long... */
    f = fopen(fname, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        my_syslog(LOG_ERR,
                  b"failed to read %s: %s\x00" as *const u8 as
                      *const libc::c_char, fname, strerror(errno));
        return 0 as libc::c_int
    }
    mark_servers(SERV_FROM_RESOLV);
    loop  {
        line = fgets((*dnsmasq_daemon).namebuff, MAXDNAME, f);
        if line.is_null() { break ; }
        let mut addr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut source_addr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut token =
            strtok(line, b" \t\n\r\x00" as *const u8 as *const libc::c_char);
        if token.is_null() { continue ; }
        if strcmp(token,
                  b"nameserver\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int &&
               strcmp(token,
                      b"server\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
            continue ;
        }
        token =
            strtok(NULL_0 as *mut libc::c_char,
                   b" \t\n\r\x00" as *const u8 as *const libc::c_char);
        if token.is_null() { continue ; }
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        memset(&mut source_addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in_0.sin_addr.s_addr = inet_addr(token);
        if addr.in_0.sin_addr.s_addr != -(1 as libc::c_int) as in_addr_t {
            addr.in_0.sin_family = AF_INET as sa_family_t;
            source_addr.in_0.sin_family = addr.in_0.sin_family;
            addr.in_0.sin_port = __bswap_16(53 as libc::c_int as __uint16_t);
            source_addr.in_0.sin_addr.s_addr = INADDR_ANY as in_addr_t;
            source_addr.in_0.sin_port =
                __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
        } else {
            let mut scope_index = 0 as libc::c_int;
            let mut scope_id = strchr(token, '%' as i32);
            if !scope_id.is_null() {
                let fresh8 = scope_id;
                scope_id = scope_id.offset(1);
                *fresh8 = 0 as libc::c_int as libc::c_char;
                scope_index = if_nametoindex(scope_id) as libc::c_int
            }
            if !(inet_pton(AF_INET6, token,
                           &mut addr.in6.sin6_addr as *mut in6_addr as
                               *mut libc::c_void) > 0 as libc::c_int) {
                continue ;
            }
            addr.in6.sin6_family = AF_INET6 as sa_family_t;
            source_addr.in6.sin6_family = addr.in6.sin6_family;
            addr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
            source_addr.in6.sin6_flowinfo = addr.in6.sin6_flowinfo;
            addr.in6.sin6_port = __bswap_16(53 as libc::c_int as __uint16_t);
            addr.in6.sin6_scope_id = scope_index as uint32_t;
            source_addr.in6.sin6_addr = in6addr_any;
            source_addr.in6.sin6_port =
                __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
            source_addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t
        }
        add_update_server(SERV_FROM_RESOLV, &mut addr, &mut source_addr,
                          NULL_0 as *const libc::c_char,
                          NULL_0 as *const libc::c_char);
        gotone = 1 as libc::c_int
    }
    fclose(f);
    cleanup_servers();
    return gotone;
}
/* Called when addresses are added or deleted from an interface */
#[no_mangle]
#[c2rust::src_loc = "1800:1"]
pub unsafe extern "C" fn newaddress(mut now: time_t) {
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (39 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           (*dnsmasq_daemon).options[(49 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (49 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || (*dnsmasq_daemon).doing_dhcp6 != 0 ||
           !(*dnsmasq_daemon).relay6.is_null() ||
           (*dnsmasq_daemon).doing_ra != 0 {
        enumerate_interfaces(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (39 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        create_bound_listeners(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 ||
           !(*dnsmasq_daemon).relay6.is_null() ||
           (*dnsmasq_daemon).doing_ra != 0 {
        join_multicast(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 || (*dnsmasq_daemon).doing_ra != 0 {
        dhcp_construct_contexts(now);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 { lease_find_interfaces(now); };
}
