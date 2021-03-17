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
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
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
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::{__ino64_t, __dev_t, __off64_t, __pid_t, __ssize_t};
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
  "/usr/include/x86_64-linux-gnu/bits/types/struct_timeval.h:17"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
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
    use super::socket_h::{sockaddr, socklen_t, msghdr};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::un_h::sockaddr_un;
    use super::sys_types_h::ssize_t;
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
        #[c2rust::src_loc = "173:1"]
        pub fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
                       __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
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
    #[c2rust::src_loc = "57:9"]
    pub const IPPROTO_UDP_0: libc::c_int = IPPROTO_UDP as libc::c_int;
    #[c2rust::src_loc = "190:9"]
    pub const INADDR_ANY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "43:9"]
    pub const IPPROTO_IP_0: libc::c_int = IPPROTO_IP as libc::c_int;
    #[c2rust::src_loc = "192:9"]
    pub const INADDR_BROADCAST: libc::c_uint = 0xffffffff as libc::c_uint;
    #[c2rust::src_loc = "234:9"]
    pub const INET6_ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
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
    #[c2rust::src_loc = "81:9"]
    pub const IP_PKTINFO: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "49:16"]
    pub const IP_TOS: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "107:9"]
    pub const IP_PMTUDISC_DONT: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "84:9"]
    pub const IP_MTU_DISCOVER: libc::c_int = 10 as libc::c_int;
    use super::in_h::in_addr;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dnsmasq.h:17"]
pub mod dnsmasq_h {
    #[c2rust::src_loc = "68:1"]
    pub type u8_0 = libc::c_uchar;
    #[c2rust::src_loc = "69:1"]
    pub type u16_0 = libc::c_ushort;
    #[c2rust::src_loc = "70:1"]
    pub type u32_0 = libc::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "295:7"]
    pub union all_addr {
        pub addr4: in_addr,
        pub addr6: in6_addr,
        pub cname: C2RustUnnamed_9,
        pub key: C2RustUnnamed_8,
        pub ds: C2RustUnnamed_7,
        pub srv: C2RustUnnamed_6,
        pub log: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "322:3"]
    pub struct C2RustUnnamed_5 {
        pub keytag: libc::c_ushort,
        pub algo: libc::c_ushort,
        pub digest: libc::c_ushort,
        pub rcode: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:3"]
    pub struct C2RustUnnamed_6 {
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
    pub struct C2RustUnnamed_7 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
        pub digest: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:3"]
    pub struct C2RustUnnamed_8 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub flags: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:3"]
    pub struct C2RustUnnamed_9 {
        pub target: C2RustUnnamed_10,
        pub uid: libc::c_uint,
        pub is_name_ptr: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:5"]
    pub union C2RustUnnamed_10 {
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
        pub name: C2RustUnnamed_11,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "455:3"]
    pub union C2RustUnnamed_11 {
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
    #[c2rust::src_loc = "720:8"]
    pub struct dhcp_lease {
        pub clid_len: libc::c_int,
        pub clid: *mut libc::c_uchar,
        pub hostname: *mut libc::c_char,
        pub fqdn: *mut libc::c_char,
        pub old_hostname: *mut libc::c_char,
        pub flags: libc::c_int,
        pub expires: time_t,
        pub hwaddr_len: libc::c_int,
        pub hwaddr_type: libc::c_int,
        pub hwaddr: [libc::c_uchar; 16],
        pub addr: in_addr,
        pub override_0: in_addr,
        pub giaddr: in_addr,
        pub extradata: *mut libc::c_uchar,
        pub extradata_len: libc::c_uint,
        pub extradata_size: libc::c_uint,
        pub last_interface: libc::c_int,
        pub new_interface: libc::c_int,
        pub new_prefixlen: libc::c_int,
        pub addr6: in6_addr,
        pub iaid: libc::c_uint,
        pub slaac_address: *mut slaac_address,
        pub vendorclass_count: libc::c_int,
        pub next: *mut dhcp_lease,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "741:10"]
    pub struct slaac_address {
        pub addr: in6_addr,
        pub ping_time: time_t,
        pub backoff: libc::c_int,
        pub next: *mut slaac_address,
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
        pub u: C2RustUnnamed_12,
        pub val: *mut libc::c_uchar,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_opt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "815:3"]
    pub union C2RustUnnamed_12 {
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
    #[c2rust::src_loc = "287:9"]
    pub const MS_DHCP: libc::c_int = LOG_DAEMON;
    #[c2rust::src_loc = "171:9"]
    pub const ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "943:9"]
    pub const CONTEXT_BRDCAST: libc::c_uint =
        (1 as libc::c_uint) << 2 as libc::c_int;
    #[c2rust::src_loc = "942:9"]
    pub const CONTEXT_NETMASK: libc::c_uint =
        (1 as libc::c_uint) << 1 as libc::c_int;
    #[c2rust::src_loc = "944:9"]
    pub const CONTEXT_PROXY: libc::c_uint =
        (1 as libc::c_uint) << 3 as libc::c_int;
    #[c2rust::src_loc = "941:9"]
    pub const CONTEXT_STATIC: libc::c_uint =
        (1 as libc::c_uint) << 0 as libc::c_int;
    #[c2rust::src_loc = "804:9"]
    pub const CONFIG_ADDR: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "805:9"]
    pub const CONFIG_NOCLID: libc::c_int = 128 as libc::c_int;
    #[c2rust::src_loc = "803:9"]
    pub const CONFIG_NAME: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "806:9"]
    pub const CONFIG_FROM_ETHERS: libc::c_int = 256 as libc::c_int;
    #[c2rust::src_loc = "471:9"]
    pub const F_HOSTS: libc::c_uint = (1 as libc::c_uint) << 6 as libc::c_int;
    #[c2rust::src_loc = "472:9"]
    pub const F_IPV4: libc::c_uint = (1 as libc::c_uint) << 7 as libc::c_int;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6};
    use super::time_t_h::time_t;
    use super::socket_h::{sockaddr, msghdr};
    use super::sys_types_h::{off_t, dev_t, ino_t, pid_t, ssize_t};
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::FILE_h::FILE;
    use super::syslog_h::LOG_DAEMON;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1172:4"]
        pub static mut dnsmasq_daemon: *mut dnsmasq_daemon;
        #[no_mangle]
        #[c2rust::src_loc = "1181:1"]
        pub fn cache_find_by_addr(crecp: *mut crec, addr: *mut all_addr,
                                  now: time_t, prot: libc::c_uint)
         -> *mut crec;
        #[no_mangle]
        #[c2rust::src_loc = "1199:1"]
        pub fn cache_get_name(crecp: *mut crec) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1215:1"]
        pub fn get_domain(addr: in_addr) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1285:1"]
        pub fn legal_hostname(name: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1286:1"]
        pub fn canonicalise(in_0: *mut libc::c_char, nomem: *mut libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1289:1"]
        pub fn safe_strncpy(dest: *mut libc::c_char, src: *const libc::c_char,
                            size: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "1291:1"]
        pub fn whine_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1294:1"]
        pub fn hostname_isequal(a: *const libc::c_char,
                                b: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1298:1"]
        pub fn is_same_net(a: in_addr, b: in_addr, mask: in_addr)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1302:1"]
        pub fn retry_send(rc: ssize_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1305:1"]
        pub fn parse_hex(in_0: *mut libc::c_char, out: *mut libc::c_uchar,
                         maxlen: libc::c_int,
                         wildcard_mask: *mut libc::c_uint,
                         mac_type: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1313:1"]
        pub fn wildcard_match(wildcard: *const libc::c_char,
                              match_0: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1314:1"]
        pub fn wildcard_matchn(wildcard: *const libc::c_char,
                               match_0: *const libc::c_char, num: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1320:1"]
        pub fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
                   exit_code: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char,
                         _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1350:1"]
        pub fn send_from(fd: libc::c_int, nowild: libc::c_int,
                         packet: *mut libc::c_char, len: size_t,
                         to: *mut mysockaddr, source: *mut all_addr,
                         iface: libc::c_uint) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1358:1"]
        pub fn indextoname(fd: libc::c_int, index: libc::c_int,
                           name: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn iface_check(family: libc::c_int, addr: *mut all_addr,
                           name: *mut libc::c_char, auth: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1381:1"]
        pub fn fix_fd(fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1415:1"]
        pub fn lease_update_dns(force: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1414:1"]
        pub fn lease_update_file(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1455:1"]
        pub fn dhcp_reply(context: *mut dhcp_context,
                          iface_name: *mut libc::c_char,
                          int_index: libc::c_int, sz: size_t, now: time_t,
                          unicast_dest: libc::c_int, loopback: libc::c_int,
                          is_inform: *mut libc::c_int, pxe: libc::c_int,
                          fallback: in_addr, recvtime: time_t) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "1442:1"]
        pub fn lease_prune(target: *mut dhcp_lease, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1489:1"]
        pub fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                               callback:
                                   Option<unsafe extern "C" fn()
                                              -> libc::c_int>) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1578:1"]
        pub fn recv_dhcp_packet(fd: libc::c_int, msg: *mut msghdr) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "1582:1"]
        pub fn match_netid(check: *mut dhcp_netid, pool: *mut dhcp_netid,
                           tagnotneeded: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1465:1"]
        pub fn icmp_ping(addr: in_addr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1440:1"]
        pub fn lease_find_by_addr(addr: in_addr) -> *mut dhcp_lease;
        #[no_mangle]
        #[c2rust::src_loc = "1441:1"]
        pub fn lease_find_max_addr(context: *mut dhcp_context) -> in_addr;
        #[no_mangle]
        #[c2rust::src_loc = "1583:1"]
        pub fn strip_hostname(hostname: *mut libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dhcp-protocol.h:17"]
pub mod dhcp_protocol_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:8"]
    pub struct dhcp_packet {
        pub op: u8_0,
        pub htype: u8_0,
        pub hlen: u8_0,
        pub hops: u8_0,
        pub xid: u32_0,
        pub secs: u16_0,
        pub flags: u16_0,
        pub ciaddr: in_addr,
        pub yiaddr: in_addr,
        pub siaddr: in_addr,
        pub giaddr: in_addr,
        pub chaddr: [u8_0; 16],
        pub sname: [u8_0; 64],
        pub file: [u8_0; 128],
        pub options: [u8_0; 312],
    }
    #[c2rust::src_loc = "20:9"]
    pub const PXE_PORT: libc::c_int = 4011 as libc::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const BOOTREQUEST: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "27:9"]
    pub const BOOTREPLY: libc::c_int = 2 as libc::c_int;
    use super::dnsmasq_h::{u8_0, u32_0, u16_0};
    use super::in_h::in_addr;
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
#[c2rust::header_src = "/usr/include/ctype.h:17"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_4 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_4 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_4 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_4 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_4 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_4 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_4 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_4 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_4 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_4 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_4 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_4 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_4 = 256;
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
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
#[c2rust::header_src = "/usr/include/net/if_arp.h:17"]
pub mod if_arp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct arpreq {
        pub arp_pa: sockaddr,
        pub arp_ha: sockaddr,
        pub arp_flags: libc::c_int,
        pub arp_netmask: sockaddr,
        pub arp_dev: [libc::c_char; 16],
    }
    #[c2rust::src_loc = "156:9"]
    pub const ATF_COM: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "73:9"]
    pub const ARPHRD_ETHER: libc::c_int = 1 as libc::c_int;
    use super::socket_h::sockaddr;
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:17"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
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
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:17"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t)
     -> __uint16_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t)
     -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t)
     -> __uint64_t {
        return __x;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:17"]
pub mod inet_h {
    use super::in_h::{in_addr_t, in_addr};
    use super::socket_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
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
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
        #[c2rust::src_loc = "226:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "397:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "486:1"]
        pub fn getc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "858:1"]
        pub fn __uflow(_: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "859:1"]
        pub fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "603:1"]
        pub fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                          __delimiter: libc::c_int, __stream: *mut FILE)
         -> __ssize_t;
    }
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
#[c2rust::header_src = "/usr/include/time.h:17"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
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
#[c2rust::header_src = "/usr/include/asm-generic/socket.h:17"]
pub mod asm_generic_socket_h {
    #[c2rust::src_loc = "12:9"]
    pub const SO_REUSEADDR: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "9:9"]
    pub const SOL_SOCKET: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "27:9"]
    pub const SO_REUSEPORT: libc::c_int = 15 as libc::c_int;
    #[c2rust::src_loc = "16:9"]
    pub const SO_BROADCAST: libc::c_int = 6 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno.h:17"]
pub mod asm_generic_errno_h {
    #[c2rust::src_loc = "75:9"]
    pub const ENOPROTOOPT: libc::c_int = 92 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/netinet/ip.h:17"]
pub mod ip_h {
    #[c2rust::src_loc = "202:9"]
    pub const IPTOS_CLASS_CS6: libc::c_int = 0xc0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:17"]
pub mod syslog_h {
    #[c2rust::src_loc = "55:9"]
    pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const LOG_DAEMON: libc::c_int =
        (3 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const LOG_ERR: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const LOG_INFO: libc::c_int = 6 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/linux/sockios.h:17"]
pub mod sockios_h {
    #[c2rust::src_loc = "117:9"]
    pub const SIOCSARP: libc::c_int = 0x8955 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const SIOCGIFADDR: libc::c_int = 0x8915 as libc::c_int;
    #[c2rust::src_loc = "60:9"]
    pub const SIOCGIFFLAGS: libc::c_int = 0x8913 as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const SIOCGSTAMP: libc::c_int = 0x8906 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/sockios.h:17"]
pub mod asm_generic_sockios_h {
    #[c2rust::src_loc = "11:9"]
    pub const SIOCGSTAMP_OLD: libc::c_int = 0x8906 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/config.h:17"]
pub mod config_h {
    #[c2rust::src_loc = "37:9"]
    pub const PING_WAIT: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "38:9"]
    pub const PING_CACHE_TIME: libc::c_int = 30 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const ETHERSFILE: [libc::c_char; 12] =
        unsafe {
            *::std::mem::transmute::<&[u8; 12],
                                     &[libc::c_char; 12]>(b"/etc/ethers\x00")
        };
}
#[c2rust::header_src = "/usr/include/linux/if_ether.h:17"]
pub mod if_ether_h {
    #[c2rust::src_loc = "32:9"]
    pub const ETH_ALEN: libc::c_int = 6 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/net/ethernet.h:17"]
pub mod ethernet_h {
    #[c2rust::src_loc = "60:9"]
    pub const ETHER_ADDR_LEN: libc::c_int = ETH_ALEN;
    use super::if_ether_h::ETH_ALEN;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:17"]
pub mod dns_protocol_h {
    #[c2rust::src_loc = "26:9"]
    pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __intmax_t, __uintmax_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __ino64_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __pid_t, __time_t, __suseconds_t,
                        __blksize_t, __blkcnt_t, __blkcnt64_t, __ssize_t,
                        __syscall_slong_t, __caddr_t, __socklen_t};
pub use self::sys_types_h::{ino_t, dev_t, off_t, pid_t, ssize_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::struct_timeval_h::timeval;
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, sockaddr, msghdr, cmsghdr, __cmsg_nxthdr,
                         PF_INET, AF_INET};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM,
                              SOCK_DGRAM_0};
pub use self::sockaddr_h::sa_family_t;
pub use self::sys_socket_h::{__CONST_SOCKADDR_ARG, sockaddr_x25, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, sendmsg, setsockopt};
pub use self::un_h::sockaddr_un;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, IPPROTO_UDP_0, INADDR_ANY,
                     IPPROTO_IP_0, INADDR_BROADCAST, INET6_ADDRSTRLEN};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::bits_in_h::{in_pktinfo, IP_PKTINFO, IP_TOS, IP_PMTUDISC_DONT,
                          IP_MTU_DISCOVER};
pub use self::dnsmasq_h::{u8_0, u16_0, u32_0, all_addr, C2RustUnnamed_5,
                          C2RustUnnamed_6, blockdata, C2RustUnnamed_7,
                          C2RustUnnamed_8, C2RustUnnamed_9, C2RustUnnamed_10,
                          crec, C2RustUnnamed_11, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_lease, slaac_address, dhcp_netid,
                          dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_12, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, EC_BADNET, MS_DHCP,
                          ADDRSTRLEN, CONTEXT_BRDCAST, CONTEXT_NETMASK,
                          CONTEXT_PROXY, CONTEXT_STATIC, CONFIG_ADDR,
                          CONFIG_NOCLID, CONFIG_NAME, CONFIG_FROM_ETHERS,
                          F_HOSTS, F_IPV4, cache_find_by_addr, cache_get_name,
                          get_domain, legal_hostname, canonicalise,
                          safe_strncpy, whine_malloc, hostname_isequal,
                          is_same_net, retry_send, parse_hex, wildcard_match,
                          wildcard_matchn, die, my_syslog, send_from,
                          indextoname, iface_check, fix_fd, lease_update_dns,
                          lease_update_file, dhcp_reply, lease_prune,
                          iface_enumerate, recv_dhcp_packet, match_netid,
                          icmp_ping, lease_find_by_addr, lease_find_max_addr,
                          strip_hostname};
pub use self::dhcp_protocol_h::{dhcp_packet, PXE_PORT, BOOTREQUEST,
                                BOOTREPLY};
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
pub use self::ctype_h::{C2RustUnnamed_4, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper, tolower,
                        toupper, __ctype_b_loc, __ctype_tolower_loc,
                        __ctype_toupper_loc};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::if_arp_h::{arpreq, ATF_COM, ARPHRD_ETHER};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity,
                                 __uint64_identity};
use self::inet_h::{inet_addr, inet_ntoa, inet_ntop};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::ioctl_h::ioctl;
use self::string_h::{memcpy, memset, memcmp, strcpy, strchr, strlen,
                     strerror};
use self::stdio_h::{stdin, stdout, fclose, fopen, vfprintf, getc, __uflow,
                    putc, __overflow, fgets, __getdelim};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
use self::time_h::difftime;
pub use self::errno_h::{errno, __errno_location};
pub use self::asm_generic_socket_h::{SO_REUSEADDR, SOL_SOCKET, SO_REUSEPORT,
                                     SO_BROADCAST};
pub use self::asm_generic_errno_h::ENOPROTOOPT;
pub use self::ip_h::IPTOS_CLASS_CS6;
pub use self::syslog_h::{LOG_WARNING, LOG_DAEMON, LOG_ERR, LOG_INFO};
pub use self::sockios_h::{SIOCSARP, SIOCGIFADDR, SIOCGIFFLAGS, SIOCGSTAMP};
pub use self::asm_generic_sockios_h::SIOCGSTAMP_OLD;
pub use self::config_h::{PING_WAIT, PING_CACHE_TIME, ETHERSFILE};
pub use self::if_ether_h::ETH_ALEN;
pub use self::ethernet_h::ETHER_ADDR_LEN;
pub use self::dns_protocol_h::MAXDNAME;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "158:3"]
pub union C2RustUnnamed_13 {
    pub align: cmsghdr,
    pub control: [libc::c_char; 32],
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "21:8"]
pub struct iface_param {
    pub current: *mut dhcp_context,
    pub relay: *mut dhcp_relay,
    pub relay_local: in_addr,
    pub ind: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "28:8"]
pub struct match_param {
    pub ind: libc::c_int,
    pub matched: libc::c_int,
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub addr: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "189:4"]
pub union C2RustUnnamed_14 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in_pktinfo,
}
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn make_fd(mut port: libc::c_int) -> libc::c_int {
    let mut fd = socket(PF_INET, SOCK_DGRAM_0, IPPROTO_UDP_0);
    let mut saddr =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut oneopt = 1 as libc::c_int;
    let mut mtu = IP_PMTUDISC_DONT;
    let mut tos = IPTOS_CLASS_CS6;
    if fd == -(1 as libc::c_int) {
        die(b"cannot create DHCP socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            NULL_0 as *mut libc::c_char, EC_BADNET);
    }
    if fix_fd(fd) == 0 ||
           setsockopt(fd, IPPROTO_IP_0, IP_MTU_DISCOVER,
                      &mut mtu as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IP_0, IP_TOS,
                      &mut tos as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IP_0, IP_PKTINFO,
                      &mut oneopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, SOL_SOCKET, SO_BROADCAST,
                      &mut oneopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) {
        die(b"failed to set options on DHCP socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            NULL_0 as *mut libc::c_char, EC_BADNET);
    }
    /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 67. That's OK if they serve different networks.
     Need to set REUSEADDR|REUSEPORT to make this possible.
     Handle the case that REUSEPORT is defined, but the kernel doesn't 
     support it. This handles the introduction of REUSEPORT on Linux. */
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
           != 0 ||
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
               != 0 {
        let mut rc = 0 as libc::c_int;
        rc =
            setsockopt(fd, SOL_SOCKET, SO_REUSEPORT,
                       &mut oneopt as *mut libc::c_int as *const libc::c_void,
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                           as socklen_t);
        if rc == -(1 as libc::c_int) && errno == ENOPROTOOPT {
            rc = 0 as libc::c_int
        }
        if rc != -(1 as libc::c_int) {
            rc =
                setsockopt(fd, SOL_SOCKET, SO_REUSEADDR,
                           &mut oneopt as *mut libc::c_int as
                               *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t)
        }
        if rc == -(1 as libc::c_int) {
            die(b"failed to set SO_REUSE{ADDR|PORT} on DHCP socket: %s\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char,
                NULL_0 as *mut libc::c_char, EC_BADNET);
        }
    }
    memset(&mut saddr as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    saddr.sin_family = AF_INET as sa_family_t;
    saddr.sin_port = __bswap_16(port as __uint16_t);
    saddr.sin_addr.s_addr = INADDR_ANY as in_addr_t;
    if bind(fd,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut saddr as *mut sockaddr_in as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) != 0 {
        die(b"failed to bind DHCP server socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            NULL_0 as *mut libc::c_char, EC_BADNET);
    }
    return fd;
}
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub unsafe extern "C" fn dhcp_init() {
    (*dnsmasq_daemon).dhcpfd = make_fd((*dnsmasq_daemon).dhcp_server_port);
    if (*dnsmasq_daemon).enable_pxe != 0 {
        (*dnsmasq_daemon).pxefd = make_fd(PXE_PORT)
    } else { (*dnsmasq_daemon).pxefd = -(1 as libc::c_int) };
}
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn dhcp_packet(mut now: time_t,
                                     mut pxe_fd: libc::c_int) {
    let mut fd =
        if pxe_fd != 0 {
            (*dnsmasq_daemon).pxefd
        } else { (*dnsmasq_daemon).dhcpfd };
    let mut mess = 0 as *mut dhcp_packet;
    let mut context = 0 as *mut dhcp_context;
    let mut relay = 0 as *mut dhcp_relay;
    let mut is_relay_reply = 0 as libc::c_int;
    let mut tmp = 0 as *mut iname;
    let mut ifr =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut msg =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut dest =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut cmptr = 0 as *mut cmsghdr;
    let mut iov = iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,};
    let mut sz: ssize_t = 0;
    let mut iface_index = 0 as libc::c_int;
    let mut unicast_dest = 0 as libc::c_int;
    let mut is_inform = 0 as libc::c_int;
    let mut loopback = 0 as libc::c_int;
    let mut rcvd_iface_index: libc::c_int = 0;
    let mut iface_addr = in_addr{s_addr: 0,};
    let mut parm =
        iface_param{current: 0 as *mut dhcp_context,
                    relay: 0 as *mut dhcp_relay,
                    relay_local: in_addr{s_addr: 0,},
                    ind: 0,};
    let mut recvtime = now;
    let mut arp_req =
        arpreq{arp_pa: sockaddr{sa_family: 0, sa_data: [0; 14],},
               arp_ha: sockaddr{sa_family: 0, sa_data: [0; 14],},
               arp_flags: 0,
               arp_netmask: sockaddr{sa_family: 0, sa_data: [0; 14],},
               arp_dev: [0; 16],};
    let mut tv = timeval{tv_sec: 0, tv_usec: 0,};
    let mut control_u =
        C2RustUnnamed_13{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    let mut bridge = 0 as *mut dhcp_bridge;
    let mut alias = 0 as *mut dhcp_bridge;
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
    msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
    msg.msg_name = &mut dest as *mut sockaddr_in as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut (*dnsmasq_daemon).dhcp_packet;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    sz = recv_dhcp_packet(fd, &mut msg);
    if sz == -(1 as libc::c_int) as libc::c_long ||
           sz <
               (::std::mem::size_of::<dhcp_packet>() as
                    libc::c_ulong).wrapping_sub(::std::mem::size_of::<[u8_0; 312]>()
                                                    as libc::c_ulong) as
                   ssize_t {
        return
    }
    if ioctl(fd, SIOCGSTAMP as libc::c_ulong, &mut tv as *mut timeval) ==
           0 as libc::c_int {
        recvtime = tv.tv_sec
    }
    if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
       {
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        while !cmptr.is_null() {
            if (*cmptr).cmsg_level == IPPROTO_IP_0 &&
                   (*cmptr).cmsg_type == IP_PKTINFO {
                let mut p = C2RustUnnamed_14{c: 0 as *mut libc::c_uchar,};
                p.c = (*cmptr).__cmsg_data.as_mut_ptr();
                iface_index = (*p.p).ipi_ifindex;
                if (*p.p).ipi_addr.s_addr != INADDR_BROADCAST {
                    unicast_dest = 1 as libc::c_int
                }
            }
            cmptr = __cmsg_nxthdr(&mut msg, cmptr)
        }
    }
    if indextoname((*dnsmasq_daemon).dhcpfd, iface_index,
                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 ||
           ioctl((*dnsmasq_daemon).dhcpfd, SIOCGIFFLAGS as libc::c_ulong,
                 &mut ifr as *mut ifreq) != 0 as libc::c_int {
        return
    }
    mess = (*dnsmasq_daemon).dhcp_packet.iov_base as *mut dhcp_packet;
    loopback =
        ((*mess).giaddr.s_addr == 0 &&
             ifr.ifr_ifru.ifru_flags as libc::c_int & IFF_LOOPBACK_0 != 0) as
            libc::c_int;
    /* ARP fiddling uses original interface even if we pretend to use a different one. */
    safe_strncpy(arp_req.arp_dev.as_mut_ptr(),
                 ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>() as
                     libc::c_ulong);
    /* If the interface on which the DHCP request was received is an
     alias of some other interface (as specified by the
     --bridge-interface option), change ifr.ifr_name so that we look
     for DHCP contexts associated with the aliased interface instead
     of with the aliasing one. */
    rcvd_iface_index = iface_index;
    bridge = (*dnsmasq_daemon).bridges;
    while !bridge.is_null() {
        alias = (*bridge).alias;
        while !alias.is_null() {
            if wildcard_matchn((*alias).iface.as_mut_ptr(),
                               ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                               IF_NAMESIZE) != 0 {
                iface_index =
                    if_nametoindex((*bridge).iface.as_mut_ptr()) as
                        libc::c_int;
                if iface_index == 0 {
                    my_syslog(MS_DHCP | LOG_WARNING,
                              b"unknown interface %s in bridge-interface\x00"
                                  as *const u8 as *const libc::c_char,
                              (*bridge).iface.as_mut_ptr());
                    return
                } else {
                    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                                 (*bridge).iface.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 16]>()
                                     as libc::c_ulong);
                    break ;
                }
            } else { alias = (*alias).next }
        }
        if !alias.is_null() { break ; }
        bridge = (*bridge).next
    }
    relay =
        relay_reply4((*dnsmasq_daemon).dhcp_packet.iov_base as
                         *mut dhcp_packet,
                     ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
    if !relay.is_null() {
        /* Reply from server, using us as relay. */
        rcvd_iface_index = (*relay).iface_index;
        if indextoname((*dnsmasq_daemon).dhcpfd, rcvd_iface_index,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
            return
        }
        is_relay_reply = 1 as libc::c_int;
        iov.iov_len = sz as size_t;
        safe_strncpy(arp_req.arp_dev.as_mut_ptr(),
                     ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 16]>() as
                         libc::c_ulong);
    } else {
        ifr.ifr_ifru.ifru_addr.sa_family = AF_INET as sa_family_t;
        if ioctl((*dnsmasq_daemon).dhcpfd, SIOCGIFADDR as libc::c_ulong,
                 &mut ifr as *mut ifreq) != -(1 as libc::c_int) {
            iface_addr =
                (*(&mut ifr.ifr_ifru.ifru_addr as *mut sockaddr as
                       *mut sockaddr_in)).sin_addr
        } else {
            if iface_check(AF_INET, NULL_0 as *mut all_addr,
                           ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                           NULL_0 as *mut libc::c_int) != 0 {
                my_syslog(MS_DHCP | LOG_WARNING,
                          b"DHCP packet received on %s which has no address\x00"
                              as *const u8 as *const libc::c_char,
                          ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            }
            return
        }
        tmp = (*dnsmasq_daemon).dhcp_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = (*tmp).next
        }
        /* unlinked contexts/relays are marked by context->current == context */
        context = (*dnsmasq_daemon).dhcp;
        while !context.is_null() {
            (*context).current = context;
            context = (*context).next
        }
        relay = (*dnsmasq_daemon).relay4;
        while !relay.is_null() {
            (*relay).current = relay;
            relay = (*relay).next
        }
        parm.current = NULL_0 as *mut dhcp_context;
        parm.relay = NULL_0 as *mut dhcp_relay;
        parm.relay_local.s_addr = 0 as libc::c_int as in_addr_t;
        parm.ind = iface_index;
        if iface_check(AF_INET,
                       &mut iface_addr as *mut in_addr as *mut all_addr,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                       NULL_0 as *mut libc::c_int) == 0 {
            /* If we failed to match the primary address of the interface, see if we've got a --listen-address
	     for a secondary */
            let mut match_0 =
                match_param{ind: 0,
                            matched: 0,
                            netmask: in_addr{s_addr: 0,},
                            broadcast: in_addr{s_addr: 0,},
                            addr: in_addr{s_addr: 0,},};
            match_0.matched = 0 as libc::c_int;
            match_0.ind = iface_index;
            if (*dnsmasq_daemon).if_addrs.is_null() ||
                   iface_enumerate(AF_INET,
                                   &mut match_0 as *mut match_param as
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
                                                                          libc::c_int>>(Some(check_listen_addrs
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
                       == 0 || match_0.matched == 0 {
                return
            }
            iface_addr = match_0.addr;
            /* make sure secondary address gets priority in case
	     there is more than one address on the interface in the same subnet */
            complete_context(match_0.addr, iface_index,
                             NULL_0 as *mut libc::c_char, match_0.netmask,
                             match_0.broadcast,
                             &mut parm as *mut iface_param as
                                 *mut libc::c_void);
        }
        if iface_enumerate(AF_INET,
                           &mut parm as *mut iface_param as *mut libc::c_void,
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
                                                              -> libc::c_int>,
                                                   Option<unsafe extern "C" fn()
                                                              ->
                                                                  libc::c_int>>(Some(complete_context
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
               == 0 {
            return
        }
        /* We're relaying this request */
        if parm.relay_local.s_addr != 0 as libc::c_int as libc::c_uint &&
               relay_upstream4(parm.relay, mess, sz as size_t, iface_index) !=
                   0 {
            return
        }
        /* May have configured relay, but not DHCP server */
        if (*dnsmasq_daemon).dhcp.is_null() {
            return
        } /* lose any expired leases */
        lease_prune(NULL_0 as *mut dhcp_lease, now);
        iov.iov_len =
            dhcp_reply(parm.current, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                       iface_index, sz as size_t, now, unicast_dest, loopback,
                       &mut is_inform, pxe_fd, iface_addr, recvtime);
        lease_update_file(now);
        lease_update_dns(0 as libc::c_int);
        if iov.iov_len == 0 as libc::c_int as libc::c_ulong { return }
    }
    msg.msg_name = &mut dest as *mut sockaddr_in as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    msg.msg_control = NULL_0 as *mut libc::c_void;
    msg.msg_controllen = 0 as libc::c_int as size_t;
    msg.msg_iov = &mut iov;
    iov.iov_base = (*dnsmasq_daemon).dhcp_packet.iov_base;
    /* packet buffer may have moved */
    mess = (*dnsmasq_daemon).dhcp_packet.iov_base as *mut dhcp_packet;
    if pxe_fd != 0 {
        if (*mess).ciaddr.s_addr != 0 as libc::c_int as libc::c_uint {
            dest.sin_addr = (*mess).ciaddr
        }
    } else if (*mess).giaddr.s_addr != 0 && is_relay_reply == 0 {
        /* Send to BOOTP relay  */
        dest.sin_port =
            __bswap_16((*dnsmasq_daemon).dhcp_server_port as __uint16_t);
        dest.sin_addr = (*mess).giaddr
    } else if (*mess).ciaddr.s_addr != 0 {
        /* If the client's idea of its own address tallys with
	 the source address in the request packet, we believe the
	 source port too, and send back to that.  If we're replying 
	 to a DHCPINFORM, trust the source address always. */
        if is_inform == 0 && dest.sin_addr.s_addr != (*mess).ciaddr.s_addr ||
               dest.sin_port as libc::c_int == 0 as libc::c_int ||
               dest.sin_addr.s_addr == 0 as libc::c_int as libc::c_uint ||
               is_relay_reply != 0 {
            dest.sin_port =
                __bswap_16((*dnsmasq_daemon).dhcp_client_port as __uint16_t);
            dest.sin_addr = (*mess).ciaddr
        }
    } else {
        /* fill cmsg for outbound interface (both broadcast & unicast) */
        let mut pkt = 0 as *mut in_pktinfo;
        msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
        msg.msg_controllen =
            ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        pkt = (*cmptr).__cmsg_data.as_mut_ptr() as *mut in_pktinfo;
        (*pkt).ipi_ifindex = rcvd_iface_index;
        (*pkt).ipi_spec_dst.s_addr = 0 as libc::c_int as in_addr_t;
        msg.msg_controllen =
            ((::std::mem::size_of::<in_pktinfo>() as
                  libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                 &
                 !(::std::mem::size_of::<size_t>() as
                       libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong)).wrapping_add((::std::mem::size_of::<cmsghdr>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
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
                                                                                                                              libc::c_ulong));
        (*cmptr).cmsg_len =
            ((::std::mem::size_of::<cmsghdr>() as
                  libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                 &
                 !(::std::mem::size_of::<size_t>() as
                       libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong)).wrapping_add(::std::mem::size_of::<in_pktinfo>()
                                                                                        as
                                                                                        libc::c_ulong);
        (*cmptr).cmsg_level = IPPROTO_IP_0;
        (*cmptr).cmsg_type = IP_PKTINFO;
        if __bswap_16((*mess).flags) as libc::c_int & 0x8000 as libc::c_int !=
               0 || (*mess).hlen as libc::c_int == 0 as libc::c_int ||
               (*mess).hlen as libc::c_ulong >
                   ::std::mem::size_of::<[libc::c_char; 14]>() as
                       libc::c_ulong ||
               (*mess).htype as libc::c_int == 0 as libc::c_int {
            /* broadcast to 255.255.255.255 (or mac address invalid) */
            dest.sin_addr.s_addr = INADDR_BROADCAST;
            dest.sin_port =
                __bswap_16((*dnsmasq_daemon).dhcp_client_port as __uint16_t)
        } else {
            /* unicast to unconfigured client. Inject mac address direct into ARP cache.
          struct sockaddr limits size to 14 bytes. */
            dest.sin_addr = (*mess).yiaddr;
            dest.sin_port =
                __bswap_16((*dnsmasq_daemon).dhcp_client_port as __uint16_t);
            memcpy(&mut arp_req.arp_pa as *mut sockaddr as *mut libc::c_void,
                   &mut dest as *mut sockaddr_in as *const libc::c_void,
                   ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
            arp_req.arp_ha.sa_family = (*mess).htype as sa_family_t;
            memcpy(arp_req.arp_ha.sa_data.as_mut_ptr() as *mut libc::c_void,
                   (*mess).chaddr.as_mut_ptr() as *const libc::c_void,
                   (*mess).hlen as libc::c_ulong);
            /* interface name already copied in */
            arp_req.arp_flags = ATF_COM;
            if ioctl((*dnsmasq_daemon).dhcpfd, SIOCSARP as libc::c_ulong,
                     &mut arp_req as *mut arpreq) == -(1 as libc::c_int) {
                my_syslog(MS_DHCP | LOG_ERR,
                          b"ARP-cache injection failed: %s\x00" as *const u8
                              as *const libc::c_char, strerror(errno));
            }
        }
    }
    while retry_send(sendmsg(fd, &mut msg, 0 as libc::c_int)) != 0 { }
    /* This can fail when, eg, iptables DROPS destination 255.255.255.255 */
    if errno != 0 as libc::c_int {
        my_syslog(MS_DHCP | LOG_WARNING,
                  b"Error sending DHCP packet to %s: %s\x00" as *const u8 as
                      *const libc::c_char, inet_ntoa(dest.sin_addr),
                  strerror(errno));
    };
}
/* check against secondary interface addresses */
#[c2rust::src_loc = "476:1"]
unsafe extern "C" fn check_listen_addrs(mut local: in_addr,
                                        mut if_index: libc::c_int,
                                        mut label: *mut libc::c_char,
                                        mut netmask: in_addr,
                                        mut broadcast: in_addr,
                                        mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut param = vparam as *mut match_param;
    let mut tmp = 0 as *mut iname;
    if if_index == (*param).ind {
        tmp = (*dnsmasq_daemon).if_addrs;
        while !tmp.is_null() {
            if (*tmp).addr.sa.sa_family as libc::c_int == AF_INET &&
                   (*tmp).addr.in_0.sin_addr.s_addr == local.s_addr {
                (*param).matched = 1 as libc::c_int;
                (*param).addr = local;
                (*param).netmask = netmask;
                (*param).broadcast = broadcast;
                break ;
            } else { tmp = (*tmp).next }
        }
    }
    return 1 as libc::c_int;
}
/* This is a complex routine: it gets called with each (address,netmask,broadcast) triple 
   of each interface (and any relay address) and does the  following things:

   1) Discards stuff for interfaces other than the one on which a DHCP packet just arrived.
   2) Fills in any netmask and broadcast addresses which have not been explicitly configured.
   3) Fills in local (this host) and router (this host or relay) addresses.
   4) Links contexts which are valid for hosts directly connected to the arrival interface on ->current.

   Note that the current chain may be superseded later for configured hosts or those coming via gateways. */
#[c2rust::src_loc = "511:1"]
unsafe extern "C" fn guess_range_netmask(mut addr: in_addr,
                                         mut netmask: in_addr) {
    let mut context = 0 as *mut dhcp_context;
    context = (*dnsmasq_daemon).dhcp;
    while !context.is_null() {
        if (*context).flags as libc::c_uint & CONTEXT_NETMASK == 0 &&
               (is_same_net(addr, (*context).start, netmask) != 0 ||
                    is_same_net(addr, (*context).end, netmask) != 0) {
            if (*context).netmask.s_addr != netmask.s_addr &&
                   !(is_same_net(addr, (*context).start, netmask) != 0 &&
                         is_same_net(addr, (*context).end, netmask) != 0) {
                strcpy((*dnsmasq_daemon).dhcp_buff,
                       inet_ntoa((*context).start));
                strcpy((*dnsmasq_daemon).dhcp_buff2,
                       inet_ntoa((*context).end));
                my_syslog(MS_DHCP | LOG_WARNING,
                          b"DHCP range %s -- %s is not consistent with netmask %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).dhcp_buff,
                          (*dnsmasq_daemon).dhcp_buff2, inet_ntoa(netmask));
            }
            (*context).netmask = netmask
        }
        context = (*context).next
    };
}
#[c2rust::src_loc = "533:1"]
unsafe extern "C" fn complete_context(mut local: in_addr,
                                      mut if_index: libc::c_int,
                                      mut label: *mut libc::c_char,
                                      mut netmask: in_addr,
                                      mut broadcast: in_addr,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut context = 0 as *mut dhcp_context;
    let mut relay = 0 as *mut dhcp_relay;
    let mut param = vparam as *mut iface_param;
    let mut share = 0 as *mut shared_network;
    let mut current_block_14: u64;
    share = (*dnsmasq_daemon).shared_networks;
    while !share.is_null() {
        if !((*share).shared_addr.s_addr == 0 as libc::c_int as libc::c_uint)
           {
            if (*share).if_index != 0 as libc::c_int {
                if (*share).if_index != if_index {
                    current_block_14 = 17778012151635330486;
                } else { current_block_14 = 13536709405535804910; }
            } else if (*share).match_addr.s_addr != local.s_addr {
                current_block_14 = 17778012151635330486;
            } else { current_block_14 = 13536709405535804910; }
            match current_block_14 {
                17778012151635330486 => { }
                _ => {
                    context = (*dnsmasq_daemon).dhcp;
                    while !context.is_null() {
                        if (*context).netmask.s_addr !=
                               0 as libc::c_int as libc::c_uint &&
                               is_same_net((*share).shared_addr,
                                           (*context).start,
                                           (*context).netmask) != 0 &&
                               is_same_net((*share).shared_addr,
                                           (*context).end, (*context).netmask)
                                   != 0 {
                            /* link it onto the current chain if we've not seen it before */
                            if (*context).current == context {
                                /* For a shared network, we have no way to guess what the default route should be. */
                                (*context).router.s_addr =
                                    0 as libc::c_int as
                                        in_addr_t; /* Use configured address for Server Identifier */
                                (*context).local = local;
                                (*context).current = (*param).current;
                                (*param).current = context
                            }
                            if (*context).flags as libc::c_uint &
                                   CONTEXT_BRDCAST == 0 {
                                (*context).broadcast.s_addr =
                                    (*context).start.s_addr |
                                        !(*context).netmask.s_addr
                            }
                        }
                        context = (*context).next
                    }
                }
            }
        }
        share = (*share).next
    }
    guess_range_netmask(local, netmask);
    context = (*dnsmasq_daemon).dhcp;
    while !context.is_null() {
        if (*context).netmask.s_addr != 0 as libc::c_int as libc::c_uint &&
               is_same_net(local, (*context).start, (*context).netmask) != 0
               && is_same_net(local, (*context).end, (*context).netmask) != 0
           {
            /* link it onto the current chain if we've not seen it before */
            if if_index == (*param).ind && (*context).current == context {
                (*context).router = local;
                (*context).local = local;
                (*context).current = (*param).current;
                (*param).current = context
            }
            if (*context).flags as libc::c_uint & CONTEXT_BRDCAST == 0 {
                if is_same_net(broadcast, (*context).start,
                               (*context).netmask) != 0 {
                    (*context).broadcast = broadcast
                } else {
                    (*context).broadcast.s_addr =
                        (*context).start.s_addr | !(*context).netmask.s_addr
                }
            }
        }
        context = (*context).next
    }
    relay = (*dnsmasq_daemon).relay4;
    while !relay.is_null() {
        if if_index == (*param).ind &&
               (*relay).local.addr4.s_addr == local.s_addr &&
               (*relay).current == relay &&
               ((*param).relay_local.s_addr ==
                    0 as libc::c_int as libc::c_uint ||
                    (*param).relay_local.s_addr == local.s_addr) {
            (*relay).current = (*param).relay;
            (*param).relay = relay;
            (*param).relay_local = local
        }
        relay = (*relay).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "623:1"]
pub unsafe extern "C" fn address_available(mut context: *mut dhcp_context,
                                           mut taddr: in_addr,
                                           mut netids: *mut dhcp_netid)
 -> *mut dhcp_context {
    /* Check is an address is OK for this network, check all
     possible ranges. Make sure that the address isn't in use
     by the server itself. */
    let mut start: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut addr = __bswap_32(taddr.s_addr);
    let mut tmp = 0 as *mut dhcp_context;
    tmp = context;
    while !tmp.is_null() {
        if taddr.s_addr == (*context).router.s_addr {
            return NULL_0 as *mut dhcp_context
        }
        tmp = (*tmp).current
    }
    tmp = context;
    while !tmp.is_null() {
        start = __bswap_32((*tmp).start.s_addr);
        end = __bswap_32((*tmp).end.s_addr);
        if (*tmp).flags as libc::c_uint & (CONTEXT_STATIC | CONTEXT_PROXY) ==
               0 && addr >= start && addr <= end &&
               match_netid((*tmp).filter, netids, 1 as libc::c_int) != 0 {
            return tmp
        }
        tmp = (*tmp).current
    }
    return NULL_0 as *mut dhcp_context;
}
#[no_mangle]
#[c2rust::src_loc = "653:1"]
pub unsafe extern "C" fn narrow_context(mut context: *mut dhcp_context,
                                        mut taddr: in_addr,
                                        mut netids: *mut dhcp_netid)
 -> *mut dhcp_context {
    /* We start of with a set of possible contexts, all on the current physical interface.
     These are chained on ->current.
     Here we have an address, and return the actual context corresponding to that
     address. Note that none may fit, if the address came a dhcp-host and is outside
     any dhcp-range. In that case we return a static range if possible, or failing that,
     any context on the correct subnet. (If there's more than one, this is a dodgy 
     configuration: maybe there should be a warning.) */
    let mut tmp = 0 as *mut dhcp_context;
    tmp = address_available(context, taddr, netids);
    if tmp.is_null() {
        tmp = context;
        while !tmp.is_null() {
            if match_netid((*tmp).filter, netids, 1 as libc::c_int) != 0 &&
                   is_same_net(taddr, (*tmp).start, (*tmp).netmask) != 0 &&
                   (*tmp).flags as libc::c_uint & CONTEXT_STATIC != 0 {
                break ;
            }
            tmp = (*tmp).current
        }
        if tmp.is_null() {
            tmp = context;
            while !tmp.is_null() {
                if match_netid((*tmp).filter, netids, 1 as libc::c_int) != 0
                       &&
                       is_same_net(taddr, (*tmp).start, (*tmp).netmask) != 0
                       && (*tmp).flags as libc::c_uint & CONTEXT_PROXY == 0 {
                    break ;
                }
                tmp = (*tmp).current
            }
        }
    }
    /* Only one context allowed now */
    if !tmp.is_null() { (*tmp).current = NULL_0 as *mut dhcp_context }
    return tmp;
}
#[no_mangle]
#[c2rust::src_loc = "690:1"]
pub unsafe extern "C" fn config_find_by_address(mut configs: *mut dhcp_config,
                                                mut addr: in_addr)
 -> *mut dhcp_config {
    let mut config = 0 as *mut dhcp_config;
    config = configs;
    while !config.is_null() {
        if (*config).flags & CONFIG_ADDR as libc::c_uint != 0 &&
               (*config).addr.s_addr == addr.s_addr {
            return config
        }
        config = (*config).next
    }
    return NULL_0 as *mut dhcp_config;
}
/* Check if and address is in use by sending ICMP ping.
   This wrapper handles a cache and load-limiting.
   Return is NULL is address in use, or a pointer to a cache entry
   recording that it isn't. */
#[no_mangle]
#[c2rust::src_loc = "705:1"]
pub unsafe extern "C" fn do_icmp_ping(mut now: time_t, mut addr: in_addr,
                                      mut hash: libc::c_uint,
                                      mut loopback: libc::c_int)
 -> *mut ping_result {
    static mut dummy: ping_result =
        ping_result{addr: in_addr{s_addr: 0,},
                    time: 0,
                    hash: 0,
                    next: 0 as *const ping_result as *mut ping_result,};
    let mut r = 0 as *mut ping_result;
    let mut victim = NULL_0 as *mut ping_result;
    let mut count: libc::c_int = 0;
    let mut max =
        (0.6f64 *
             (PING_CACHE_TIME as libc::c_float / PING_WAIT as libc::c_float)
                 as libc::c_double) as libc::c_int;
    /* check if we failed to ping addr sometime in the last
     PING_CACHE_TIME seconds. If so, assume the same situation still exists.
     This avoids problems when a stupid client bangs
     on us repeatedly. As a final check, if we did more
     than 60% of the possible ping checks in the last 
     PING_CACHE_TIME, we are in high-load mode, so don't do any more. */
    count = 0 as libc::c_int; /* old record */
    r = (*dnsmasq_daemon).ping_results;
    while !r.is_null() {
        if difftime(now, (*r).time) >
               PING_CACHE_TIME as libc::c_float as libc::c_double {
            victim = r
        } else { count += 1; if (*r).addr.s_addr == addr.s_addr { return r } }
        r = (*r).next
    }
    /* didn't find cached entry */
    if count >= max ||
           (*dnsmasq_daemon).options[(21 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (21 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || loopback != 0 {
        /* overloaded, or configured not to check, loopback interface, return "not in use" */
        dummy.hash = hash; /* address in use. */
        return &mut dummy
    } else if icmp_ping(addr) != 0 {
        return NULL_0 as *mut ping_result
    } else {
        /* at this point victim may hold an expired record */
        if victim.is_null() {
            victim =
                whine_malloc(::std::mem::size_of::<ping_result>() as
                                 libc::c_ulong) as *mut ping_result;
            if !victim.is_null() {
                (*victim).next = (*dnsmasq_daemon).ping_results;
                (*dnsmasq_daemon).ping_results = victim
            }
        }
        /* record that this address is OK for 30s 
	 without more ping checks */
        if !victim.is_null() {
            (*victim).addr = addr;
            (*victim).time = now;
            (*victim).hash = hash
        }
        return victim
    };
}
#[no_mangle]
#[c2rust::src_loc = "761:1"]
pub unsafe extern "C" fn address_allocate(mut context: *mut dhcp_context,
                                          mut addrp: *mut in_addr,
                                          mut hwaddr: *mut libc::c_uchar,
                                          mut hw_len: libc::c_int,
                                          mut netids: *mut dhcp_netid,
                                          mut now: time_t,
                                          mut loopback: libc::c_int)
 -> libc::c_int {
    /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. */
    let mut start = in_addr{s_addr: 0,};
    let mut addr = in_addr{s_addr: 0,};
    let mut c = 0 as *mut dhcp_context;
    let mut d = 0 as *mut dhcp_context;
    let mut i: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    /* hash hwaddr: use the SDBM hashing algorithm.  Seems to give good
     dispersal even with similarly-valued "strings". */
    j = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < hw_len {
        j =
            (*hwaddr.offset(i as isize) as
                 libc::c_uint).wrapping_add(j <<
                                                6 as
                                                    libc::c_int).wrapping_add(j
                                                                                  <<
                                                                                  16
                                                                                      as
                                                                                      libc::c_int).wrapping_sub(j);
        i += 1
    }
    /* j == 0 is marker */
    if j == 0 as libc::c_int as libc::c_uint {
        j = 1 as libc::c_int as libc::c_uint
    }
    pass = 0 as libc::c_int;
    while pass <= 1 as libc::c_int {
        c = context;
        while !c.is_null() {
            if !((*c).flags as libc::c_uint & (CONTEXT_STATIC | CONTEXT_PROXY)
                     != 0) {
                if !(match_netid((*c).filter, netids, pass) == 0) {
                    if (*dnsmasq_daemon).options[(34 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (34 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 {
                        /* seed is largest extant lease addr in this context */
                        start = lease_find_max_addr(c)
                    } else {
                        /* pick a seed based on hwaddr */
                        start.s_addr =
                            __bswap_32(__bswap_32((*c).start.s_addr).wrapping_add(j.wrapping_add((*c).addr_epoch).wrapping_rem((1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint).wrapping_add(__bswap_32((*c).end.s_addr)).wrapping_sub(__bswap_32((*c).start.s_addr)))))
                    }
                    /* iterate until we find a free address. */
                    addr = start;
                    loop  {
                        /* eliminate addresses in use by the server. */
                        d = context;
                        while !d.is_null() {
                            if addr.s_addr == (*d).router.s_addr { break ; }
                            d = (*d).current
                        }
                        /* Addresses which end in .255 and .0 are broken in Windows even when using 
	       supernetting. ie dhcp-range=192.168.0.1,192.168.1.254,255,255,254.0
	       then 192.168.0.255 is a valid IP address, but not for Windows as it's
	       in the class C range. See  KB281579. We therefore don't allocate these 
	       addresses to avoid hard-to-diagnose problems. Thanks Bill. */
                        if d.is_null() && lease_find_by_addr(addr).is_null()
                               &&
                               config_find_by_address((*dnsmasq_daemon).dhcp_conf,
                                                      addr).is_null() &&
                               (!(__bswap_32(addr.s_addr) &
                                      0xe0000000 as libc::c_uint ==
                                      0xc0000000 as libc::c_uint) ||
                                    __bswap_32(addr.s_addr) &
                                        0xff as libc::c_int as libc::c_uint !=
                                        0xff as libc::c_int as libc::c_uint &&
                                        __bswap_32(addr.s_addr) &
                                            0xff as libc::c_int as
                                                libc::c_uint !=
                                            0 as libc::c_int as libc::c_uint)
                           {
                            /* in consec-ip mode, skip addresses equal to
		   the number of addresses rejected by clients. This
		   should avoid the same client being offered the same
		   address after it has rjected it. */
                            if (*dnsmasq_daemon).options[(34 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (34 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   != 0 && (*c).addr_epoch != 0 {
                                (*c).addr_epoch =
                                    (*c).addr_epoch.wrapping_sub(1)
                            } else {
                                let mut r = 0 as *mut ping_result;
                                r = do_icmp_ping(now, addr, j, loopback);
                                if !r.is_null() {
                                    /* consec-ip mode: we offered this address for another client
			   (different hash) recently, don't offer it to this one. */
                                    if (*dnsmasq_daemon).options[(34 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_ulong))
                                                                     as usize]
                                           &
                                           (1 as libc::c_uint) <<
                                               (34 as libc::c_int as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                           == 0 || (*r).hash == j {
                                        *addrp = addr;
                                        return 1 as libc::c_int
                                    }
                                } else if (*dnsmasq_daemon).options[(34 as
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
                                                  (34 as libc::c_int as
                                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                                              == 0 {
                                    (*c).addr_epoch =
                                        (*c).addr_epoch.wrapping_add(1)
                                }
                            }
                        }
                        addr.s_addr =
                            __bswap_32(__bswap_32(addr.s_addr).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint));
                        if addr.s_addr ==
                               __bswap_32(__bswap_32((*c).end.s_addr).wrapping_add(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint))
                           {
                            addr = (*c).start
                        }
                        if !(addr.s_addr != start.s_addr) { break ; }
                    }
                }
            }
            c = (*c).current
        }
        pass += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "860:1"]
pub unsafe extern "C" fn dhcp_read_ethers() {
    let mut f =
        fopen(ETHERSFILE.as_ptr(),
              b"r\x00" as *const u8 as *const libc::c_char);
    let mut flags: libc::c_uint = 0;
    let mut buff = (*dnsmasq_daemon).namebuff;
    let mut ip = 0 as *mut libc::c_char;
    let mut cp = 0 as *mut libc::c_char;
    let mut addr = in_addr{s_addr: 0,};
    let mut hwaddr: [libc::c_uchar; 6] = [0; 6];
    let mut up = 0 as *mut *mut dhcp_config;
    let mut tmp = 0 as *mut dhcp_config;
    let mut config = 0 as *mut dhcp_config;
    let mut count = 0 as libc::c_int;
    let mut lineno = 0 as libc::c_int;
    /* address in use: perturb address selection so that we are
			   less likely to try this address again. */
    addr.s_addr = 0 as libc::c_int as in_addr_t; /* eliminate warning */
    if f.is_null() {
        my_syslog(MS_DHCP | LOG_ERR,
                  b"failed to read %s: %s\x00" as *const u8 as
                      *const libc::c_char, ETHERSFILE.as_ptr(),
                  strerror(errno));
        return
    }
    /* This can be called again on SIGHUP, so remove entries created last time round. */
    up = &mut (*dnsmasq_daemon).dhcp_conf;
    config = (*dnsmasq_daemon).dhcp_conf;
    while !config.is_null() {
        tmp = (*config).next;
        if (*config).flags & CONFIG_FROM_ETHERS as libc::c_uint != 0 {
            *up = tmp;
            /* cannot have a clid */
            if (*config).flags & CONFIG_NAME as libc::c_uint != 0 {
                free((*config).hostname as *mut libc::c_void);
            }
            free((*config).hwaddr as *mut libc::c_void);
            free(config as *mut libc::c_void);
        } else { up = &mut (*config).next }
        config = tmp
    }
    while !fgets(buff, MAXDNAME, f).is_null() {
        let mut host = NULL_0 as *mut libc::c_char;
        lineno += 1;
        while strlen(buff) > 0 as libc::c_int as libc::c_ulong &&
                  *(*__ctype_b_loc()).offset(*buff.offset(strlen(buff).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                              as isize) as
                                                 libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            *buff.offset(strlen(buff).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                             isize) = 0 as libc::c_int as libc::c_char
        }
        if *buff as libc::c_int == '#' as i32 ||
               *buff as libc::c_int == '+' as i32 ||
               *buff as libc::c_int == 0 as libc::c_int {
            continue ;
        }
        ip = buff;
        while *ip as libc::c_int != 0 &&
                  *(*__ctype_b_loc()).offset(*ip as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      == 0 {
            ip = ip.offset(1)
        }
        while *ip as libc::c_int != 0 &&
                  *(*__ctype_b_loc()).offset(*ip as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            *ip = 0 as libc::c_int as libc::c_char;
            ip = ip.offset(1)
        }
        if *ip == 0 ||
               parse_hex(buff, hwaddr.as_mut_ptr(), ETHER_ADDR_LEN,
                         NULL_0 as *mut libc::c_uint,
                         NULL_0 as *mut libc::c_int) != ETHER_ADDR_LEN {
            my_syslog(MS_DHCP | LOG_ERR,
                      b"bad line at %s line %d\x00" as *const u8 as
                          *const libc::c_char, ETHERSFILE.as_ptr(), lineno);
        } else {
            /* check for name or dotted-quad */
            cp = ip;
            while *cp != 0 {
                if !(*cp as libc::c_int == '.' as i32 ||
                         *cp as libc::c_int >= '0' as i32 &&
                             *cp as libc::c_int <= '9' as i32) {
                    break ;
                }
                cp = cp.offset(1)
            }
            if *cp == 0 {
                addr.s_addr = inet_addr(ip);
                if addr.s_addr == -(1 as libc::c_int) as in_addr_t {
                    my_syslog(MS_DHCP | LOG_ERR,
                              b"bad address at %s line %d\x00" as *const u8 as
                                  *const libc::c_char, ETHERSFILE.as_ptr(),
                              lineno);
                    continue ;
                } else {
                    flags = CONFIG_ADDR as libc::c_uint;
                    config = (*dnsmasq_daemon).dhcp_conf;
                    while !config.is_null() {
                        if (*config).flags & CONFIG_ADDR as libc::c_uint != 0
                               && (*config).addr.s_addr == addr.s_addr {
                            break ;
                        }
                        config = (*config).next
                    }
                }
            } else {
                let mut nomem: libc::c_int = 0;
                host = canonicalise(ip, &mut nomem);
                if host.is_null() || legal_hostname(host) == 0 {
                    if nomem == 0 {
                        my_syslog(MS_DHCP | LOG_ERR,
                                  b"bad name at %s line %d\x00" as *const u8
                                      as *const libc::c_char,
                                  ETHERSFILE.as_ptr(), lineno);
                    }
                    free(host as *mut libc::c_void);
                    continue ;
                } else {
                    flags = CONFIG_NAME as libc::c_uint;
                    config = (*dnsmasq_daemon).dhcp_conf;
                    while !config.is_null() {
                        if (*config).flags & CONFIG_NAME as libc::c_uint != 0
                               &&
                               hostname_isequal((*config).hostname, host) != 0
                           {
                            break ;
                        }
                        config = (*config).next
                    }
                }
            }
            if !config.is_null() &&
                   (*config).flags & CONFIG_FROM_ETHERS as libc::c_uint != 0 {
                my_syslog(MS_DHCP | LOG_ERR,
                          b"ignoring %s line %d, duplicate name or IP address\x00"
                              as *const u8 as *const libc::c_char,
                          ETHERSFILE.as_ptr(), lineno);
            } else {
                if config.is_null() {
                    config = (*dnsmasq_daemon).dhcp_conf;
                    while !config.is_null() {
                        let mut conf_addr = (*config).hwaddr;
                        if !conf_addr.is_null() && (*conf_addr).next.is_null()
                               &&
                               (*conf_addr).wildcard_mask ==
                                   0 as libc::c_int as libc::c_uint &&
                               (*conf_addr).hwaddr_len == ETHER_ADDR_LEN &&
                               ((*conf_addr).hwaddr_type == ARPHRD_ETHER ||
                                    (*conf_addr).hwaddr_type ==
                                        0 as libc::c_int) &&
                               memcmp((*conf_addr).hwaddr.as_mut_ptr() as
                                          *const libc::c_void,
                                      hwaddr.as_mut_ptr() as
                                          *const libc::c_void,
                                      ETHER_ADDR_LEN as libc::c_ulong) ==
                                   0 as libc::c_int {
                            break ;
                        }
                        config = (*config).next
                    }
                    if config.is_null() {
                        config =
                            whine_malloc(::std::mem::size_of::<dhcp_config>()
                                             as libc::c_ulong) as
                                *mut dhcp_config;
                        if config.is_null() { continue ; }
                        (*config).flags = CONFIG_FROM_ETHERS as libc::c_uint;
                        (*config).hwaddr = NULL_0 as *mut hwaddr_config;
                        (*config).domain = NULL_0 as *mut libc::c_char;
                        (*config).netid = NULL_0 as *mut dhcp_netid_list;
                        (*config).next = (*dnsmasq_daemon).dhcp_conf;
                        (*dnsmasq_daemon).dhcp_conf = config
                    }
                    (*config).flags |= flags;
                    if flags & CONFIG_NAME as libc::c_uint != 0 {
                        (*config).hostname = host;
                        host = NULL_0 as *mut libc::c_char
                    }
                    if flags & CONFIG_ADDR as libc::c_uint != 0 {
                        (*config).addr = addr
                    }
                }
                (*config).flags |= CONFIG_NOCLID as libc::c_uint;
                if (*config).hwaddr.is_null() {
                    (*config).hwaddr =
                        whine_malloc(::std::mem::size_of::<hwaddr_config>() as
                                         libc::c_ulong) as *mut hwaddr_config
                }
                if !(*config).hwaddr.is_null() {
                    memcpy((*(*config).hwaddr).hwaddr.as_mut_ptr() as
                               *mut libc::c_void,
                           hwaddr.as_mut_ptr() as *const libc::c_void,
                           ETHER_ADDR_LEN as libc::c_ulong);
                    (*(*config).hwaddr).hwaddr_len = ETHER_ADDR_LEN;
                    (*(*config).hwaddr).hwaddr_type = ARPHRD_ETHER;
                    (*(*config).hwaddr).wildcard_mask =
                        0 as libc::c_int as libc::c_uint;
                    (*(*config).hwaddr).next = NULL_0 as *mut hwaddr_config
                }
                count += 1;
                free(host as *mut libc::c_void);
            }
        }
    }
    fclose(f);
    my_syslog(MS_DHCP | LOG_INFO,
              b"read %s - %d addresses\x00" as *const u8 as
                  *const libc::c_char, ETHERSFILE.as_ptr(), count);
}
/* If we've not found a hostname any other way, try and see if there's one in /etc/hosts
   for this address. If it has a domain part, that must match the set domain and
   it gets stripped. The set of legal domain names is bigger than the set of legal hostnames
   so check here that the domain name is legal as a hostname. 
   NOTE: we're only allowed to overwrite daemon->dhcp_buff if we succeed. */
#[no_mangle]
#[c2rust::src_loc = "1027:1"]
pub unsafe extern "C" fn host_from_dns(mut addr: in_addr)
 -> *mut libc::c_char {
    let mut lookup = 0 as *mut crec; /* DNS disabled. */
    if (*dnsmasq_daemon).port == 0 as libc::c_int {
        return NULL_0 as *mut libc::c_char
    }
    lookup =
        cache_find_by_addr(NULL_0 as *mut crec,
                           &mut addr as *mut in_addr as *mut all_addr,
                           0 as libc::c_int as time_t, F_IPV4);
    if !lookup.is_null() && (*lookup).flags & F_HOSTS != 0 {
        let mut dot = 0 as *mut libc::c_char;
        let mut hostname = cache_get_name(lookup);
        dot = strchr(hostname, '.' as i32);
        if !dot.is_null() &&
               strlen(dot.offset(1 as libc::c_int as isize)) !=
                   0 as libc::c_int as libc::c_ulong {
            let mut d2 = get_domain(addr);
            if d2.is_null() ||
                   hostname_isequal(dot.offset(1 as libc::c_int as isize), d2)
                       == 0 {
                return NULL_0 as *mut libc::c_char
            }
            /* wrong domain */
        }
        if legal_hostname(hostname) == 0 {
            return NULL_0 as *mut libc::c_char
        }
        safe_strncpy((*dnsmasq_daemon).dhcp_buff, hostname,
                     256 as libc::c_int as size_t);
        strip_hostname((*dnsmasq_daemon).dhcp_buff);
        return (*dnsmasq_daemon).dhcp_buff
    }
    return NULL_0 as *mut libc::c_char;
}
#[c2rust::src_loc = "1060:1"]
unsafe extern "C" fn relay_upstream4(mut relay: *mut dhcp_relay,
                                     mut mess: *mut dhcp_packet,
                                     mut sz: size_t,
                                     mut iface_index: libc::c_int)
 -> libc::c_int {
    /* ->local is same value for all relays on ->current chain */
    let mut from = all_addr{addr4: in_addr{s_addr: 0,},};
    if (*mess).op as libc::c_int != BOOTREQUEST { return 0 as libc::c_int }
    /* source address == relay address */
    from.addr4 = (*relay).local.addr4;
    /* already gatewayed ? */
    if (*mess).giaddr.s_addr != 0 {
        /* if so check if by us, to stomp on loops. */
        if (*mess).giaddr.s_addr == (*relay).local.addr4.s_addr {
            return 1 as libc::c_int
        }
    } else {
        /* plug in our address */
        (*mess).giaddr.s_addr = (*relay).local.addr4.s_addr
    }
    let fresh6 = (*mess).hops;
    (*mess).hops = (*mess).hops.wrapping_add(1);
    if fresh6 as libc::c_int > 20 as libc::c_int { return 1 as libc::c_int }
    while !relay.is_null() {
        let mut to =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        to.sa.sa_family = AF_INET as sa_family_t;
        to.in_0.sin_addr = (*relay).server.addr4;
        to.in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).dhcp_server_port as __uint16_t);
        send_from((*dnsmasq_daemon).dhcpfd, 0 as libc::c_int,
                  mess as *mut libc::c_char, sz, &mut to, &mut from,
                  0 as libc::c_int as libc::c_uint);
        if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (28 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            inet_ntop(AF_INET,
                      &mut (*relay).local as *mut all_addr as
                          *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                      ADDRSTRLEN as socklen_t);
            my_syslog(MS_DHCP | LOG_INFO,
                      b"DHCP relay %s -> %s\x00" as *const u8 as
                          *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                      inet_ntoa((*relay).server.addr4));
        }
        /* Save this for replies */
        (*relay).iface_index = iface_index;
        relay = (*relay).current
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "1111:1"]
unsafe extern "C" fn relay_reply4(mut mess: *mut dhcp_packet,
                                  mut arrival_interface: *mut libc::c_char)
 -> *mut dhcp_relay {
    let mut relay = 0 as *mut dhcp_relay;
    if (*mess).giaddr.s_addr == 0 as libc::c_int as libc::c_uint ||
           (*mess).op as libc::c_int != BOOTREPLY {
        return NULL_0 as *mut dhcp_relay
    }
    relay = (*dnsmasq_daemon).relay4;
    while !relay.is_null() {
        if (*mess).giaddr.s_addr == (*relay).local.addr4.s_addr {
            if (*relay).interface.is_null() ||
                   wildcard_match((*relay).interface, arrival_interface) != 0
               {
                return if (*relay).iface_index != 0 as libc::c_int {
                           relay
                       } else { NULL_0 as *mut dhcp_relay }
            }
        }
        relay = (*relay).next
    }
    return NULL_0 as *mut dhcp_relay;
}
