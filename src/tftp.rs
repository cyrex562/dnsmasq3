#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
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
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    #[c2rust::src_loc = "87:1"]
    pub type off_t = __off64_t;
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::{__ino64_t, __dev_t, __uid_t, __off64_t, __pid_t,
                         __ssize_t};
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
        } /* may be zero to use ephemeral port */
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
    #[c2rust::src_loc = "53:9"]
    pub const PF_INET6: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "104:9"]
    pub const AF_INET6: libc::c_int = PF_INET6;
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
    #[c2rust::src_loc = "79:9"]
    pub union __SOCKADDR_ARG {
        pub __sockaddr__: *mut sockaddr,
        pub __sockaddr_at__: *mut sockaddr_at,
        pub __sockaddr_ax25__: *mut sockaddr_ax25,
        pub __sockaddr_dl__: *mut sockaddr_dl,
        pub __sockaddr_eon__: *mut sockaddr_eon,
        pub __sockaddr_in__: *mut sockaddr_in,
        pub __sockaddr_in6__: *mut sockaddr_in6,
        pub __sockaddr_inarp__: *mut sockaddr_inarp,
        pub __sockaddr_ipx__: *mut sockaddr_ipx,
        pub __sockaddr_iso__: *mut sockaddr_iso,
        pub __sockaddr_ns__: *mut sockaddr_ns,
        pub __sockaddr_un__: *mut sockaddr_un,
        pub __sockaddr_x25__: *mut sockaddr_x25,
    }
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
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                    __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "537:8"]
    pub struct in6_pktinfo {
        pub ipi6_addr: in6_addr,
        pub ipi6_ifindex: libc::c_uint,
    }
    #[c2rust::src_loc = "43:9"]
    pub const IPPROTO_IP_0: libc::c_int = IPPROTO_IP as libc::c_int;
    #[c2rust::src_loc = "65:9"]
    pub const IPPROTO_IPV6_0: libc::c_int = IPPROTO_IPV6 as libc::c_int;
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
    #[c2rust::src_loc = "107:9"]
    pub const IP_PMTUDISC_DONT: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "84:9"]
    pub const IP_MTU_DISCOVER: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "81:9"]
    pub const IP_PKTINFO: libc::c_int = 8 as libc::c_int;
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
    #[c2rust::src_loc = "286:9"]
    pub const MS_TFTP: libc::c_int = LOG_USER;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6,
                      in_addr_t};
    use super::time_t_h::time_t;
    use super::socket_h::sockaddr;
    use super::sys_types_h::{off_t, dev_t, ino_t, pid_t};
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::FILE_h::FILE;
    use super::syslog_h::LOG_USER;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1172:4"]
        pub static mut dnsmasq_daemon: *mut dnsmasq_daemon;
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
        #[c2rust::src_loc = "1304:1"]
        pub fn prettyprint_addr(addr: *mut mysockaddr, buf: *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1311:1"]
        pub fn read_write(fd: libc::c_int, packet: *mut libc::c_uchar,
                          size: libc::c_int, rw: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1313:1"]
        pub fn wildcard_match(wildcard: *const libc::c_char,
                              match_0: *const libc::c_char) -> libc::c_int;
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
        #[c2rust::src_loc = "1371:1"]
        pub fn enumerate_interfaces(reset: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn iface_check(family: libc::c_int, addr: *mut all_addr,
                           name: *mut libc::c_char, auth: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1379:1"]
        pub fn loopback_exception(fd: libc::c_int, family: libc::c_int,
                                  addr: *mut all_addr,
                                  name: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1380:1"]
        pub fn label_exception(index: libc::c_int, family: libc::c_int,
                               addr: *mut all_addr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1381:1"]
        pub fn fix_fd(fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1440:1"]
        pub fn lease_find_by_addr(addr: in_addr) -> *mut dhcp_lease;
        #[no_mangle]
        #[c2rust::src_loc = "1522:1"]
        pub fn queue_tftp(file_len: off_t, filename: *mut libc::c_char,
                          peer: *mut mysockaddr);
        #[no_mangle]
        #[c2rust::src_loc = "1672:1"]
        pub fn find_mac(addr: *mut mysockaddr, mac: *mut libc::c_uchar,
                        lazy: libc::c_int, now: time_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1652:1"]
        pub fn poll_check(fd: libc::c_int, event: libc::c_short)
         -> libc::c_int;
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
    #[c2rust::src_loc = "201:9"]
    pub const __S_IREAD: libc::c_int = 0o400 as libc::c_int;
    #[c2rust::src_loc = "179:9"]
    pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t, __ino64_t, __blkcnt64_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/net/if.h:17"]
pub mod if_h {
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
        pub ifr_ifrn: C2RustUnnamed_2,
        pub ifr_ifru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:5"]
    pub union C2RustUnnamed_1 {
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
    pub union C2RustUnnamed_2 {
        pub ifrn_name: [libc::c_char; 16],
    }
    #[c2rust::src_loc = "31:9"]
    pub const IF_NAMESIZE: libc::c_int = 16 as libc::c_int;
    use super::socket_h::sockaddr;
    use super::types_h::__caddr_t;
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
    pub type C2RustUnnamed_3 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_3 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_3 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_3 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_3 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_3 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_3 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_3 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_3 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_3 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_3 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_3 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_3 = 256;
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
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "486:1"]
        pub fn getc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "122:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "133:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "137:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "330:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "397:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:17"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:17"]
pub mod unistd_h {
    use super::types_h::{__off64_t, __uid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "337:1"]
        pub fn lseek(__fd: libc::c_int, __offset: __off64_t,
                     __whence: libc::c_int) -> __off64_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn geteuid() -> __uid_t;
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
    #[c2rust::src_loc = "137:10"]
    pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "105:10"]
    pub const S_IRUSR: libc::c_int = __S_IREAD;
    #[c2rust::src_loc = "111:10"]
    pub const S_IRGRP: libc::c_int = S_IRUSR >> 3 as libc::c_int;
    #[c2rust::src_loc = "117:10"]
    pub const S_IROTH: libc::c_int = S_IRGRP >> 3 as libc::c_int;
    use super::stat_h::__S_IREAD;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "171:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:17"]
pub mod syslog_h {
    #[c2rust::src_loc = "54:9"]
    pub const LOG_ERR: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const LOG_USER: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const LOG_INFO: libc::c_int = 6 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno-base.h:17"]
pub mod errno_base_h {
    #[c2rust::src_loc = "17:9"]
    pub const EACCES: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "16:9"]
    pub const ENOMEM: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "6:9"]
    pub const ENOENT: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/fcntl-linux.h:17"]
pub mod fcntl_linux_h {
    #[c2rust::src_loc = "43:9"]
    pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:17"]
pub mod dns_protocol_h {
    #[c2rust::src_loc = "26:9"]
    pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
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
#[c2rust::header_src = "/usr/include/net/if_arp.h:17"]
pub mod if_arp_h {
    #[c2rust::src_loc = "73:9"]
    pub const ARPHRD_ETHER: libc::c_int = 1 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno.h:17"]
pub mod asm_generic_errno_h {
    #[c2rust::src_loc = "81:9"]
    pub const EADDRINUSE: libc::c_int = 98 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/linux/sockios.h:17"]
pub mod sockios_h {
    #[c2rust::src_loc = "74:9"]
    pub const SIOCGIFMTU: libc::c_int = 0x8921 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/poll.h:17"]
pub mod poll_h {
    #[c2rust::src_loc = "25:9"]
    pub const POLLIN: libc::c_int = 0x1 as libc::c_int;
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __intmax_t, __uintmax_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __ino64_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __pid_t, __time_t, __blksize_t,
                        __blkcnt_t, __blkcnt64_t, __ssize_t,
                        __syscall_slong_t, __caddr_t, __socklen_t};
pub use self::sys_types_h::{ino_t, dev_t, uid_t, off_t, pid_t, ssize_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, sockaddr, msghdr, cmsghdr, __cmsg_nxthdr,
                         PF_INET, AF_INET, PF_INET6, AF_INET6};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM,
                              SOCK_DGRAM_0};
pub use self::sockaddr_h::sa_family_t;
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_ns, sockaddr_iso,
                             sockaddr_ipx, sockaddr_inarp, sockaddr_eon,
                             sockaddr_dl, sockaddr_ax25, sockaddr_at, socket,
                             bind, getsockname, recv, recvmsg, setsockopt};
pub use self::un_h::sockaddr_un;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, in6_pktinfo, IPPROTO_IP_0,
                     IPPROTO_IPV6_0};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::bits_in_h::{in_pktinfo, IP_PMTUDISC_DONT, IP_MTU_DISCOVER,
                          IP_PKTINFO};
pub use self::dnsmasq_h::{u32_0, all_addr, C2RustUnnamed_4, C2RustUnnamed_5,
                          blockdata, C2RustUnnamed_6, C2RustUnnamed_7,
                          C2RustUnnamed_8, C2RustUnnamed_9, crec,
                          C2RustUnnamed_10, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_lease, slaac_address, dhcp_netid,
                          dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_11, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, MS_TFTP, safe_strncpy,
                          whine_malloc, sa_len, sockaddr_isequal,
                          prettyprint_addr, read_write, wildcard_match,
                          my_syslog, send_from, indextoname,
                          enumerate_interfaces, iface_check,
                          loopback_exception, label_exception, fix_fd,
                          lease_find_by_addr, queue_tftp, find_mac,
                          poll_check};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER, __S_IREAD,
                       __S_IFMT};
pub use self::if_h::{ifmap, ifreq, C2RustUnnamed_1, C2RustUnnamed_2,
                     IF_NAMESIZE};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll, free};
pub use self::ctype_h::{C2RustUnnamed_3, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper, tolower,
                        toupper, __ctype_b_loc, __ctype_tolower_loc,
                        __ctype_toupper_loc};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
use self::stdio_h::{stdin, stdout, sprintf, vfprintf, snprintf, getc, putc,
                    __getdelim, __uflow, __overflow};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::ioctl_h::ioctl;
use self::string_h::{memmove, memset, strcpy, strncat, strcmp, strstr, strlen,
                     strerror};
use self::strings_h::strcasecmp;
use self::unistd_h::{lseek, close, geteuid};
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
pub use self::fcntl_h::{SEEK_SET, S_IRUSR, S_IRGRP, S_IROTH, open};
use self::time_h::difftime;
pub use self::errno_h::{errno, __errno_location};
pub use self::syslog_h::{LOG_ERR, LOG_USER, LOG_INFO};
pub use self::errno_base_h::{EACCES, ENOMEM, ENOENT};
pub use self::fcntl_linux_h::O_RDONLY;
pub use self::dns_protocol_h::MAXDNAME;
pub use self::if_ether_h::ETH_ALEN;
pub use self::ethernet_h::ETHER_ADDR_LEN;
pub use self::if_arp_h::ARPHRD_ETHER;
pub use self::asm_generic_errno_h::EADDRINUSE;
pub use self::sockios_h::SIOCGIFMTU;
pub use self::poll_h::POLLIN;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "731:10"]
pub struct errmess {
    pub op: libc::c_ushort,
    pub err: libc::c_ushort,
    pub message: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "790:14"]
pub struct datamess {
    pub op: libc::c_ushort,
    pub block: libc::c_ushort,
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "767:14"]
pub struct oackmess {
    pub op: libc::c_ushort,
    pub data: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "651:10"]
pub struct ack {
    pub op: libc::c_ushort,
    pub block: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "180:17"]
pub union C2RustUnnamed_12 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "132:8"]
pub union C2RustUnnamed_13 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "67:3"]
pub union C2RustUnnamed_14 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
    pub control: [libc::c_char; 32],
}
#[c2rust::src_loc = "30:9"]
pub const OP_RRQ: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "33:9"]
pub const OP_ACK: libc::c_int = 4 as libc::c_int;
#[c2rust::src_loc = "34:9"]
pub const OP_ERR: libc::c_int = 5 as libc::c_int;
#[c2rust::src_loc = "37:9"]
pub const ERR_NOTDEF: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "38:9"]
pub const ERR_FNF: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "39:9"]
pub const ERR_PERM: libc::c_int = 2 as libc::c_int;
#[c2rust::src_loc = "41:9"]
pub const ERR_ILL: libc::c_int = 4 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn tftp_request(mut listen: *mut listener,
                                      mut now: time_t) {
    let mut len: ssize_t = 0;
    let mut packet = (*dnsmasq_daemon).packet;
    let mut filename = 0 as *mut libc::c_char;
    let mut mode = 0 as *mut libc::c_char;
    let mut p = 0 as *mut libc::c_char;
    let mut end = 0 as *mut libc::c_char;
    let mut opt = 0 as *mut libc::c_char;
    let mut addr = mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut peer = mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut msg =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut iov = iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,};
    let mut ifr =
        ifreq{ifr_ifrn: C2RustUnnamed_2{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_1{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut is_err = 1 as libc::c_int;
    let mut if_index = 0 as libc::c_int;
    let mut mtu = 0 as libc::c_int;
    let mut tmp = 0 as *mut iname;
    let mut transfer = NULL_0 as *mut tftp_transfer;
    let mut up = 0 as *mut *mut tftp_transfer;
    let mut port = (*dnsmasq_daemon).start_tftp_port;
    let mut mtuflag = IP_PMTUDISC_DONT;
    let mut namebuff: [libc::c_char; 16] = [0; 16];
    let mut name = NULL_0 as *mut libc::c_char;
    let mut prefix = (*dnsmasq_daemon).tftp_prefix;
    let mut pref = 0 as *mut tftp_prefix;
    let mut addra = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut family = (*listen).addr.sa.sa_family as libc::c_int;
    /* Can always get recvd interface for IPv6 */
    let mut check_dest =
        ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
             == 0 || family == AF_INET6) as libc::c_int;
    let mut control_u =
        C2RustUnnamed_14{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong;
    msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = &mut peer as *mut mysockaddr as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    iov.iov_base = packet as *mut libc::c_void;
    iov.iov_len = (*dnsmasq_daemon).packet_buff_sz as size_t;
    /* we overwrote the buffer... */
    (*dnsmasq_daemon).srv_save = NULL_0 as *mut server;
    len = recvmsg((*listen).tftpfd, &mut msg, 0 as libc::c_int);
    if len < 2 as libc::c_int as libc::c_long { return }
    /* Can always get recvd interface for IPv6 */
    if check_dest == 0 {
        if !(*listen).iface.is_null() {
            addr = (*(*listen).iface).addr;
            name = (*(*listen).iface).name;
            mtu = (*(*listen).iface).mtu;
            if (*dnsmasq_daemon).tftp_mtu != 0 as libc::c_int &&
                   (*dnsmasq_daemon).tftp_mtu < mtu {
                mtu = (*dnsmasq_daemon).tftp_mtu
            }
        } else {
            /* we're listening on an address that doesn't appear on an interface,
	     ask the kernel what the socket is bound to */
            let mut tcp_len =
                ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as
                    socklen_t;
            if getsockname((*listen).tftpfd,
                           __SOCKADDR_ARG{__sockaddr__:
                                              &mut addr as *mut mysockaddr as
                                                  *mut sockaddr,},
                           &mut tcp_len) == -(1 as libc::c_int) {
                return
            }
        }
    } else {
        let mut cmptr = 0 as *mut cmsghdr;
        if msg.msg_controllen <
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            return
        }
        addr.sa.sa_family = family as sa_family_t;
        if family == AF_INET {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IP_0 &&
                       (*cmptr).cmsg_type == IP_PKTINFO {
                    let mut p_0 =
                        C2RustUnnamed_13{c: 0 as *mut libc::c_uchar,};
                    p_0.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    addr.in_0.sin_addr = (*p_0.p).ipi_spec_dst;
                    if_index = (*p_0.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if family == AF_INET6 {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IPV6_0 &&
                       (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
                    let mut p_1 =
                        C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
                    p_1.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    addr.in6.sin6_addr = (*p_1.p).ipi6_addr;
                    if_index = (*p_1.p).ipi6_ifindex as libc::c_int
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if indextoname((*listen).tftpfd, if_index, namebuff.as_mut_ptr()) == 0
           {
            return
        }
        name = namebuff.as_mut_ptr();
        addra.addr4 = addr.in_0.sin_addr;
        if family == AF_INET6 { addra.addr6 = addr.in6.sin6_addr }
        if !(*dnsmasq_daemon).tftp_interfaces.is_null() {
            /* dedicated tftp interface list */
            tmp = (*dnsmasq_daemon).tftp_interfaces;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name, name) != 0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() { return }
        } else {
            /* Do the same as DHCP */
            if iface_check(family, &mut addra, name,
                           NULL_0 as *mut libc::c_int) == 0 {
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
                    enumerate_interfaces(0 as libc::c_int);
                }
                if loopback_exception((*listen).tftpfd, family, &mut addra,
                                      name) == 0 &&
                       label_exception(if_index, family, &mut addra) == 0 {
                    return
                }
            }
            /* allowed interfaces are the same as for DHCP */
            tmp = (*dnsmasq_daemon).dhcp_except;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name, name) != 0 {
                    return
                }
                tmp = (*tmp).next
            }
        }
        safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                     IF_NAMESIZE as size_t);
        if ioctl((*listen).tftpfd, SIOCGIFMTU as libc::c_ulong,
                 &mut ifr as *mut ifreq) != -(1 as libc::c_int) {
            mtu = ifr.ifr_ifru.ifru_mtu;
            if (*dnsmasq_daemon).tftp_mtu != 0 as libc::c_int &&
                   (*dnsmasq_daemon).tftp_mtu < mtu {
                mtu = (*dnsmasq_daemon).tftp_mtu
            }
        }
    }
    /* Failed to get interface mtu - can use configured value. */
    if mtu == 0 as libc::c_int { mtu = (*dnsmasq_daemon).tftp_mtu }
    /* data transfer via server listening socket */
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut tftp_cnt: libc::c_int = 0;
        tftp_cnt = 0 as libc::c_int;
        transfer = (*dnsmasq_daemon).tftp_trans;
        up = &mut (*dnsmasq_daemon).tftp_trans;
        while !transfer.is_null() {
            tftp_cnt += 1;
            if sockaddr_isequal(&mut peer, &mut (*transfer).peer) != 0 {
                if __bswap_16(*(packet as *mut libc::c_ushort)) as libc::c_int
                       == OP_RRQ {
                    /* Handle repeated RRQ or abandoned transfer from same host and port 
		     by unlinking and reusing the struct transfer. */
                    *up = (*transfer).next;
                    break ;
                } else { handle_tftp(now, transfer, len); return }
            } else { up = &mut (*transfer).next; transfer = (*transfer).next }
        }
        /* Enforce simultaneous transfer limit. In non-single-port mode
	 this is doene by not listening on the server socket when
	 too many transfers are in progress. */
        if transfer.is_null() && tftp_cnt >= (*dnsmasq_daemon).tftp_max {
            return
        }
    }
    if !name.is_null() {
        /* check for per-interface prefix */
        pref = (*dnsmasq_daemon).if_prefix;
        while !pref.is_null() {
            if strcmp((*pref).interface, name) == 0 as libc::c_int {
                prefix = (*pref).prefix
            }
            pref = (*pref).next
        }
    }
    if family == AF_INET {
        addr.in_0.sin_port = __bswap_16(port as __uint16_t)
    } else {
        addr.in6.sin6_port = __bswap_16(port as __uint16_t);
        addr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
        addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t
    }
    /* May reuse struct transfer from abandoned transfer in single port mode. */
    if transfer.is_null() &&
           {
               transfer =
                   whine_malloc(::std::mem::size_of::<tftp_transfer>() as
                                    libc::c_ulong) as *mut tftp_transfer;
               transfer.is_null()
           } {
        return
    }
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        (*transfer).sockfd = (*listen).tftpfd
    } else {
        (*transfer).sockfd = socket(family, SOCK_DGRAM_0, 0 as libc::c_int);
        if (*transfer).sockfd == -(1 as libc::c_int) {
            free(transfer as *mut libc::c_void);
            return
        }
    }
    (*transfer).peer = peer;
    (*transfer).source = addra;
    (*transfer).if_index = if_index;
    (*transfer).timeout = now + 2 as libc::c_int as libc::c_long;
    (*transfer).backoff = 1 as libc::c_int;
    (*transfer).block = 1 as libc::c_int as libc::c_uint;
    (*transfer).blocksize = 512 as libc::c_int as libc::c_uint;
    (*transfer).offset = 0 as libc::c_int as off_t;
    (*transfer).file = NULL_0 as *mut tftp_file;
    (*transfer).opt_transize = 0 as libc::c_int as libc::c_char;
    (*transfer).opt_blocksize = (*transfer).opt_transize;
    (*transfer).carrylf = 0 as libc::c_int as libc::c_char;
    (*transfer).netascii = (*transfer).carrylf;
    prettyprint_addr(&mut peer, (*dnsmasq_daemon).addrbuff);
    /* if we have a nailed-down range, iterate until we find a free one. */
    while (*dnsmasq_daemon).options[(60 as libc::c_int as
                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong))
                                        as usize] &
              (1 as libc::c_uint) <<
                  (60 as libc::c_int as
                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                        as
                                                        libc::c_ulong).wrapping_mul(8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
              == 0 {
        if !(bind((*transfer).sockfd,
                  __CONST_SOCKADDR_ARG{__sockaddr__: &mut addr.sa,},
                  sa_len(&mut addr) as socklen_t) == -(1 as libc::c_int) ||
                 setsockopt((*transfer).sockfd, IPPROTO_IP_0, IP_MTU_DISCOVER,
                            &mut mtuflag as *mut libc::c_int as
                                *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as socklen_t) ==
                     -(1 as libc::c_int) || fix_fd((*transfer).sockfd) == 0) {
            break ;
        }
        if errno == EADDRINUSE &&
               (*dnsmasq_daemon).start_tftp_port != 0 as libc::c_int {
            port += 1;
            if port <= (*dnsmasq_daemon).end_tftp_port {
                if family == AF_INET {
                    addr.in_0.sin_port = __bswap_16(port as __uint16_t)
                } else { addr.in6.sin6_port = __bswap_16(port as __uint16_t) }
                continue ;
            } else {
                my_syslog(MS_TFTP | LOG_ERR,
                          b"unable to get free port for TFTP\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        free_transfer(transfer);
        return
    }
    p = packet.offset(2 as libc::c_int as isize);
    end = packet.offset(len as isize);
    if __bswap_16(*(packet as *mut libc::c_ushort)) as libc::c_int != OP_RRQ
           || { filename = next(&mut p, end); filename.is_null() } ||
           { mode = next(&mut p, end); mode.is_null() } ||
           strcasecmp(mode, b"octet\x00" as *const u8 as *const libc::c_char)
               != 0 as libc::c_int &&
               strcasecmp(mode,
                          b"netascii\x00" as *const u8 as *const libc::c_char)
                   != 0 as libc::c_int {
        len =
            tftp_err(ERR_ILL, packet,
                     b"unsupported request from %s\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char,
                     (*dnsmasq_daemon).addrbuff);
        is_err = 1 as libc::c_int
    } else {
        if strcasecmp(mode,
                      b"netascii\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*transfer).netascii = 1 as libc::c_int as libc::c_char
        }
        loop  {
            opt = next(&mut p, end);
            if opt.is_null() { break ; }
            if strcasecmp(opt,
                          b"blksize\x00" as *const u8 as *const libc::c_char)
                   == 0 as libc::c_int {
                opt = next(&mut p, end);
                if !opt.is_null() &&
                       (*dnsmasq_daemon).options[(27 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (27 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                    /* 32 bytes for IP, UDP and TFTP headers, 52 bytes for IPv6 */
                    let mut overhead =
                        if family == AF_INET {
                            32 as libc::c_int
                        } else { 52 as libc::c_int };
                    (*transfer).blocksize = atoi(opt) as libc::c_uint;
                    if (*transfer).blocksize <
                           1 as libc::c_int as libc::c_uint {
                        (*transfer).blocksize =
                            1 as libc::c_int as libc::c_uint
                    }
                    if (*transfer).blocksize >
                           ((*dnsmasq_daemon).packet_buff_sz as
                                libc::c_uint).wrapping_sub(4 as libc::c_int as
                                                               libc::c_uint) {
                        (*transfer).blocksize =
                            ((*dnsmasq_daemon).packet_buff_sz as
                                 libc::c_uint).wrapping_sub(4 as libc::c_int
                                                                as
                                                                libc::c_uint)
                    }
                    if mtu != 0 as libc::c_int &&
                           (*transfer).blocksize >
                               (mtu as
                                    libc::c_uint).wrapping_sub(overhead as
                                                                   libc::c_uint)
                       {
                        (*transfer).blocksize =
                            (mtu as
                                 libc::c_uint).wrapping_sub(overhead as
                                                                libc::c_uint)
                    }
                    (*transfer).opt_blocksize =
                        1 as libc::c_int as libc::c_char;
                    (*transfer).block = 0 as libc::c_int as libc::c_uint
                }
            } else if strcasecmp(opt,
                                 b"tsize\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                          && !next(&mut p, end).is_null() &&
                          (*transfer).netascii == 0 {
                (*transfer).opt_transize = 1 as libc::c_int as libc::c_char;
                (*transfer).block = 0 as libc::c_int as libc::c_uint
            }
        }
        /* cope with backslashes from windows boxen. */
        p = filename;
        while *p != 0 {
            if *p as libc::c_int == '\\' as i32 {
                *p = '/' as i32 as libc::c_char
            } else if (*dnsmasq_daemon).options[(38 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (38 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          != 0 {
                *p =
                    ({
                         let mut __res: libc::c_int = 0;
                         if ::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong >
                                1 as libc::c_int as libc::c_ulong {
                             if 0 != 0 {
                                 let mut __c = *p as libc::c_int;
                                 __res =
                                     if __c < -(128 as libc::c_int) ||
                                            __c > 255 as libc::c_int {
                                         __c
                                     } else {
                                         *(*__ctype_tolower_loc()).offset(__c
                                                                              as
                                                                              isize)
                                     }
                             } else { __res = tolower(*p as libc::c_int) }
                         } else {
                             __res =
                                 *(*__ctype_tolower_loc()).offset(*p as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                         }
                         __res
                     }) as libc::c_char
            }
            p = p.offset(1)
        }
        strcpy((*dnsmasq_daemon).namebuff,
               b"/\x00" as *const u8 as *const libc::c_char);
        if !prefix.is_null() {
            if *prefix.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '/' as i32 {
                *(*dnsmasq_daemon).namebuff.offset(0 as libc::c_int as isize)
                    = 0 as libc::c_int as libc::c_char
            }
            strncat((*dnsmasq_daemon).namebuff, prefix,
                    ((MAXDNAME - 1 as libc::c_int) as
                         libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
            if *prefix.offset(strlen(prefix).wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                                  as isize) as libc::c_int != '/' as i32 {
                strncat((*dnsmasq_daemon).namebuff,
                        b"/\x00" as *const u8 as *const libc::c_char,
                        ((MAXDNAME - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
            }
            if (*dnsmasq_daemon).options[(29 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (29 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                let mut oldlen = strlen((*dnsmasq_daemon).namebuff);
                let mut statbuf =
                    stat{st_dev: 0,
                         st_ino: 0,
                         st_nlink: 0,
                         st_mode: 0,
                         st_uid: 0,
                         st_gid: 0,
                         __pad0: 0,
                         st_rdev: 0,
                         st_size: 0,
                         st_blksize: 0,
                         st_blocks: 0,
                         st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                         st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                         st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                         __glibc_reserved: [0; 3],};
                strncat((*dnsmasq_daemon).namebuff,
                        (*dnsmasq_daemon).addrbuff,
                        ((MAXDNAME - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
                strncat((*dnsmasq_daemon).namebuff,
                        b"/\x00" as *const u8 as *const libc::c_char,
                        ((MAXDNAME - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
                /* remove unique-directory if it doesn't exist */
                if stat((*dnsmasq_daemon).namebuff, &mut statbuf) ==
                       -(1 as libc::c_int) ||
                       !(statbuf.st_mode & __S_IFMT as libc::c_uint ==
                             0o40000 as libc::c_int as libc::c_uint) {
                    *(*dnsmasq_daemon).namebuff.offset(oldlen as isize) =
                        0 as libc::c_int as libc::c_char
                }
            }
            if (*dnsmasq_daemon).options[(56 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (56 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                let mut macaddr = NULL_0 as *mut libc::c_uchar;
                let mut macbuf: [libc::c_uchar; 16] = [0; 16];
                if !(*dnsmasq_daemon).dhcp.is_null() &&
                       peer.sa.sa_family as libc::c_int == AF_INET {
                    /* Check if the client IP is in our lease database */
                    let mut lease = lease_find_by_addr(peer.in_0.sin_addr);
                    if !lease.is_null() &&
                           (*lease).hwaddr_type == ARPHRD_ETHER &&
                           (*lease).hwaddr_len == ETHER_ADDR_LEN {
                        macaddr = (*lease).hwaddr.as_mut_ptr()
                    }
                }
                /* If no luck, try to find in ARP table. This only works if client is in same (V)LAN */
                if macaddr.is_null() &&
                       find_mac(&mut peer, macbuf.as_mut_ptr(),
                                1 as libc::c_int, now) > 0 as libc::c_int {
                    macaddr = macbuf.as_mut_ptr()
                }
                if !macaddr.is_null() {
                    let mut oldlen_0 = strlen((*dnsmasq_daemon).namebuff);
                    let mut statbuf_0 =
                        stat{st_dev: 0,
                             st_ino: 0,
                             st_nlink: 0,
                             st_mode: 0,
                             st_uid: 0,
                             st_gid: 0,
                             __pad0: 0,
                             st_rdev: 0,
                             st_size: 0,
                             st_blksize: 0,
                             st_blocks: 0,
                             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                             __glibc_reserved: [0; 3],};
                    snprintf((*dnsmasq_daemon).namebuff.offset(oldlen_0 as
                                                                   isize),
                             ((MAXDNAME - 1 as libc::c_int) as
                                  libc::c_ulong).wrapping_sub(oldlen_0),
                             b"%.2x-%.2x-%.2x-%.2x-%.2x-%.2x/\x00" as
                                 *const u8 as *const libc::c_char,
                             *macaddr.offset(0 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(1 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(2 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(3 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(4 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(5 as libc::c_int as isize) as
                                 libc::c_int);
                    /* remove unique-directory if it doesn't exist */
                    if stat((*dnsmasq_daemon).namebuff, &mut statbuf_0) ==
                           -(1 as libc::c_int) ||
                           !(statbuf_0.st_mode & __S_IFMT as libc::c_uint ==
                                 0o40000 as libc::c_int as libc::c_uint) {
                        *(*dnsmasq_daemon).namebuff.offset(oldlen_0 as isize)
                            = 0 as libc::c_int as libc::c_char
                    }
                }
            }
            /* Absolute pathnames OK if they match prefix */
            if *filename.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '/' as i32 {
                if strstr(filename, (*dnsmasq_daemon).namebuff) == filename {
                    *(*dnsmasq_daemon).namebuff.offset(0 as libc::c_int as
                                                           isize) =
                        0 as libc::c_int as libc::c_char
                } else { filename = filename.offset(1) }
            }
        } else if *filename.offset(0 as libc::c_int as isize) as libc::c_int
                      == '/' as i32 {
            *(*dnsmasq_daemon).namebuff.offset(0 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char
        }
        strncat((*dnsmasq_daemon).namebuff, filename,
                ((MAXDNAME - 1 as libc::c_int) as
                     libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
        /* check permissions and open file */
        (*transfer).file = check_tftp_fileperm(&mut len, prefix);
        if !(*transfer).file.is_null() {
            len = get_block(packet, transfer);
            if len == -(1 as libc::c_int) as libc::c_long {
                len = tftp_err_oops(packet, (*dnsmasq_daemon).namebuff)
            } else { is_err = 0 as libc::c_int }
        }
    }
    send_from((*transfer).sockfd,
              ((*dnsmasq_daemon).options[(60 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (60 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0) as libc::c_int, packet, len as size_t, &mut peer,
              &mut addra, if_index as libc::c_uint);
    if is_err != 0 {
        free_transfer(transfer);
    } else {
        (*transfer).next = (*dnsmasq_daemon).tftp_trans;
        (*dnsmasq_daemon).tftp_trans = transfer
    };
}
#[c2rust::src_loc = "494:1"]
unsafe extern "C" fn check_tftp_fileperm(mut len: *mut ssize_t,
                                         mut prefix: *mut libc::c_char)
 -> *mut tftp_file {
    let mut current_block: u64;
    let mut packet = (*dnsmasq_daemon).packet;
    let mut namebuff = (*dnsmasq_daemon).namebuff;
    let mut file = 0 as *mut tftp_file;
    let mut t = 0 as *mut tftp_transfer;
    let mut uid = geteuid();
    let mut statbuf =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut fd = -(1 as libc::c_int);
    /* trick to ban moving out of the subtree */
    if !(!prefix.is_null() &&
             !strstr(namebuff,
                     b"/../\x00" as *const u8 as
                         *const libc::c_char).is_null()) {
        fd = open(namebuff, O_RDONLY);
        if fd == -(1 as libc::c_int) {
            if errno == ENOENT {
                *len =
                    tftp_err(ERR_FNF, packet,
                             b"file %s not found\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             namebuff);
                return NULL_0 as *mut tftp_file
            } else if errno == EACCES {
                current_block = 15533630919196144218;
            } else { current_block = 9018216499526184084; }
        } else if fstat(fd, &mut statbuf) == -(1 as libc::c_int) {
            current_block = 9018216499526184084;
        } else {
            /* stat the file descriptor to avoid stat->open races */
            /* running as root, must be world-readable */
            if uid == 0 as libc::c_int as libc::c_uint {
                if statbuf.st_mode & S_IROTH as libc::c_uint == 0 {
                    current_block = 15533630919196144218;
                } else { current_block = 1054647088692577877; }
            } else if (*dnsmasq_daemon).options[(26 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (26 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          != 0 && uid != statbuf.st_uid {
                current_block = 15533630919196144218;
            } else { current_block = 1054647088692577877; }
            match current_block {
                15533630919196144218 => { }
                _ => {
                    /* in secure mode, must be owned by user running dnsmasq */
                    /* If we're doing many transfers from the same file, only 
     open it once this saves lots of file descriptors 
     when mass-booting a big cluster, for instance. 
     Be conservative and only share when inode and name match
     this keeps error messages sane. */
                    t = (*dnsmasq_daemon).tftp_trans;
                    while !t.is_null() {
                        if (*(*t).file).dev == statbuf.st_dev &&
                               (*(*t).file).inode == statbuf.st_ino &&
                               strcmp((*(*t).file).filename.as_mut_ptr(),
                                      namebuff) == 0 as libc::c_int {
                            close(fd);
                            (*(*t).file).refcount += 1;
                            return (*t).file
                        }
                        t = (*t).next
                    }
                    file =
                        whine_malloc((::std::mem::size_of::<tftp_file>() as
                                          libc::c_ulong).wrapping_add(strlen(namebuff)).wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                            as *mut tftp_file;
                    if file.is_null() {
                        errno = ENOMEM
                    } else {
                        (*file).fd = fd;
                        (*file).size = statbuf.st_size;
                        (*file).dev = statbuf.st_dev;
                        (*file).inode = statbuf.st_ino;
                        (*file).refcount = 1 as libc::c_int;
                        strcpy((*file).filename.as_mut_ptr(), namebuff);
                        return file
                    }
                    current_block = 9018216499526184084;
                }
            }
        }
        match current_block {
            15533630919196144218 => { }
            _ => {
                *len = tftp_err_oops(packet, namebuff);
                if fd != -(1 as libc::c_int) { close(fd); }
                return NULL_0 as *mut tftp_file
            }
        }
    }
    errno = EACCES;
    *len =
        tftp_err(ERR_PERM, packet,
                 b"cannot access %s: %s\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char, namebuff);
    if fd != -(1 as libc::c_int) { close(fd); }
    return NULL_0 as *mut tftp_file;
}
#[no_mangle]
#[c2rust::src_loc = "577:1"]
pub unsafe extern "C" fn check_tftp_listeners(mut now: time_t) {
    let mut transfer = 0 as *mut tftp_transfer;
    let mut tmp = 0 as *mut tftp_transfer;
    let mut up = 0 as *mut *mut tftp_transfer;
    /* In single port mode, all packets come via port 69 and tftp_request() */
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        transfer = (*dnsmasq_daemon).tftp_trans;
        while !transfer.is_null() {
            if poll_check((*transfer).sockfd, POLLIN as libc::c_short) != 0 {
                /* we overwrote the buffer... */
                (*dnsmasq_daemon).srv_save = NULL_0 as *mut server;
                handle_tftp(now, transfer,
                            recv((*transfer).sockfd,
                                 (*dnsmasq_daemon).packet as
                                     *mut libc::c_void,
                                 (*dnsmasq_daemon).packet_buff_sz as size_t,
                                 0 as libc::c_int));
            }
            transfer = (*transfer).next
        }
    }
    let mut current_block_32: u64;
    transfer = (*dnsmasq_daemon).tftp_trans;
    up = &mut (*dnsmasq_daemon).tftp_trans;
    while !transfer.is_null() {
        tmp = (*transfer).next;
        if difftime(now, (*transfer).timeout) >= 0.0f64 {
            let mut endcon = 0 as libc::c_int;
            let mut len: ssize_t = 0;
            /* timeout, retransmit */
            (*transfer).timeout +=
                (1 as libc::c_int +
                     ((1 as libc::c_int) <<
                          (*transfer).backoff / 2 as libc::c_int)) as
                    libc::c_long;
            /* we overwrote the buffer... */
            (*dnsmasq_daemon).srv_save = NULL_0 as *mut server;
            len = get_block((*dnsmasq_daemon).packet, transfer);
            if len == -(1 as libc::c_int) as libc::c_long {
                len =
                    tftp_err_oops((*dnsmasq_daemon).packet,
                                  (*(*transfer).file).filename.as_mut_ptr());
                endcon = 1 as libc::c_int
            } else {
                (*transfer).backoff += 1;
                if (*transfer).backoff > 7 as libc::c_int {
                    /* don't complain about timeout when we're awaiting the last
		 ACK, some clients never send it */
                    if len as libc::c_uint ==
                           (*transfer).blocksize.wrapping_add(4 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                       {
                        endcon = 1 as libc::c_int
                    }
                    len = 0 as libc::c_int as ssize_t
                }
            }
            if len != 0 as libc::c_int as libc::c_long {
                send_from((*transfer).sockfd,
                          ((*dnsmasq_daemon).options[(60 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (60 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0) as libc::c_int, (*dnsmasq_daemon).packet,
                          len as size_t, &mut (*transfer).peer,
                          &mut (*transfer).source,
                          (*transfer).if_index as libc::c_uint);
            }
            if endcon != 0 || len == 0 as libc::c_int as libc::c_long {
                strcpy((*dnsmasq_daemon).namebuff,
                       (*(*transfer).file).filename.as_mut_ptr());
                sanitise((*dnsmasq_daemon).namebuff);
                prettyprint_addr(&mut (*transfer).peer,
                                 (*dnsmasq_daemon).addrbuff);
                my_syslog(MS_TFTP | LOG_INFO,
                          if endcon != 0 {
                              b"failed sending %s to %s\x00" as *const u8 as
                                  *const libc::c_char
                          } else {
                              b"sent %s to %s\x00" as *const u8 as
                                  *const libc::c_char
                          }, (*dnsmasq_daemon).namebuff,
                          (*dnsmasq_daemon).addrbuff);
                /* unlink */
                *up = tmp;
                if endcon != 0 {
                    free_transfer(transfer);
                } else {
                    /* put on queue to be sent to script and deleted */
                    (*transfer).next = (*dnsmasq_daemon).tftp_done_trans;
                    (*dnsmasq_daemon).tftp_done_trans = transfer
                }
                current_block_32 = 14523784380283086299;
            } else { current_block_32 = 11385396242402735691; }
        } else { current_block_32 = 11385396242402735691; }
        match current_block_32 {
            11385396242402735691 => { up = &mut (*transfer).next }
            _ => { }
        }
        transfer = tmp
    };
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
/* packet in daemon->packet as this is called. */
#[c2rust::src_loc = "649:1"]
unsafe extern "C" fn handle_tftp(mut now: time_t,
                                 mut transfer: *mut tftp_transfer,
                                 mut len: ssize_t) {
    let mut mess = (*dnsmasq_daemon).packet as *mut ack;
    if len >= ::std::mem::size_of::<ack>() as libc::c_ulong as ssize_t {
        if __bswap_16((*mess).op) as libc::c_int == OP_ACK &&
               __bswap_16((*mess).block) as libc::c_int ==
                   (*transfer).block as libc::c_ushort as libc::c_int {
            /* Got ack, ensure we take the (re)transmit path */
            (*transfer).timeout = now;
            (*transfer).backoff = 0 as libc::c_int;
            let fresh6 = (*transfer).block;
            (*transfer).block = (*transfer).block.wrapping_add(1);
            if fresh6 != 0 as libc::c_int as libc::c_uint {
                (*transfer).offset +=
                    (*transfer).blocksize.wrapping_sub((*transfer).expansion)
                        as libc::c_long
            }
        } else if __bswap_16((*mess).op) as libc::c_int == OP_ERR {
            let mut p =
                (*dnsmasq_daemon).packet.offset(::std::mem::size_of::<ack>()
                                                    as libc::c_ulong as
                                                    isize);
            let mut end = (*dnsmasq_daemon).packet.offset(len as isize);
            let mut err = next(&mut p, end);
            prettyprint_addr(&mut (*transfer).peer,
                             (*dnsmasq_daemon).addrbuff);
            /* Sanitise error message */
            if err.is_null() {
                err =
                    b"\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            } else { sanitise(err); }
            my_syslog(MS_TFTP | LOG_ERR,
                      b"error %d %s received from %s\x00" as *const u8 as
                          *const libc::c_char,
                      __bswap_16((*mess).block) as libc::c_int, err,
                      (*dnsmasq_daemon).addrbuff);
            /* Got err, ensure we take abort */
            (*transfer).timeout = now;
            (*transfer).backoff = 100 as libc::c_int
        }
    };
}
#[c2rust::src_loc = "690:1"]
unsafe extern "C" fn free_transfer(mut transfer: *mut tftp_transfer) {
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        close((*transfer).sockfd);
    }
    if !(*transfer).file.is_null() &&
           {
               (*(*transfer).file).refcount -= 1;
               ((*(*transfer).file).refcount) == 0 as libc::c_int
           } {
        close((*(*transfer).file).fd);
        free((*transfer).file as *mut libc::c_void);
    }
    free(transfer as *mut libc::c_void);
}
#[c2rust::src_loc = "704:1"]
unsafe extern "C" fn next(mut p: *mut *mut libc::c_char,
                          mut end: *mut libc::c_char) -> *mut libc::c_char {
    let mut ret = *p;
    let mut len: size_t = 0;
    if *end.offset(-(1 as libc::c_int as isize)) as libc::c_int !=
           0 as libc::c_int || *p == end ||
           { len = strlen(ret); (len) == 0 as libc::c_int as libc::c_ulong } {
        return NULL_0 as *mut libc::c_char
    }
    *p =
        (*p).offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                        isize);
    return ret;
}
#[c2rust::src_loc = "718:1"]
unsafe extern "C" fn sanitise(mut buf: *mut libc::c_char) {
    let mut q = 0 as *mut libc::c_uchar;
    let mut r = 0 as *mut libc::c_uchar;
    r = buf as *mut libc::c_uchar;
    q = r;
    while *r != 0 {
        if *(*__ctype_b_loc()).offset(*r as libc::c_int as isize) as
               libc::c_int &
               _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            let fresh7 = q;
            q = q.offset(1);
            *fresh7 = *r
        }
        r = r.offset(1)
    }
    *q = 0 as libc::c_int as libc::c_uchar;
}
#[c2rust::src_loc = "728:9"]
pub const MAXMESSAGE: libc::c_int = 500 as libc::c_int;
/* limit to make packet < 512 bytes and definitely smaller than buffer */
#[c2rust::src_loc = "729:1"]
unsafe extern "C" fn tftp_err(mut err: libc::c_int,
                              mut packet: *mut libc::c_char,
                              mut message: *mut libc::c_char,
                              mut file: *mut libc::c_char) -> ssize_t {
    let mut mess = packet as *mut errmess; /* include terminating zero */
    let mut len: ssize_t = 0;
    let mut ret = 4 as libc::c_int as ssize_t;
    let mut errstr = strerror(errno);
    memset(packet as *mut libc::c_void, 0 as libc::c_int,
           (*dnsmasq_daemon).packet_buff_sz as libc::c_ulong);
    sanitise(file);
    (*mess).op = __bswap_16(5 as libc::c_int as __uint16_t);
    (*mess).err = __bswap_16(err as __uint16_t);
    len =
        snprintf((*mess).message.as_mut_ptr(), MAXMESSAGE as libc::c_ulong,
                 message, file, errstr) as ssize_t;
    ret +=
        if len < MAXMESSAGE as libc::c_long {
            (len) + 1 as libc::c_int as libc::c_long
        } else { MAXMESSAGE as libc::c_long };
    my_syslog(MS_TFTP | LOG_ERR,
              b"%s\x00" as *const u8 as *const libc::c_char,
              (*mess).message.as_mut_ptr());
    return ret;
}
#[c2rust::src_loc = "751:1"]
unsafe extern "C" fn tftp_err_oops(mut packet: *mut libc::c_char,
                                   mut file: *mut libc::c_char) -> ssize_t {
    /* May have >1 refs to file, so potentially mangle a copy of the name */
    strcpy((*dnsmasq_daemon).namebuff, file);
    return tftp_err(ERR_NOTDEF, packet,
                    b"cannot read %s: %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    (*dnsmasq_daemon).namebuff);
}
/* return -1 for error, zero for done. */
#[c2rust::src_loc = "759:1"]
unsafe extern "C" fn get_block(mut packet: *mut libc::c_char,
                               mut transfer: *mut tftp_transfer) -> ssize_t {
    memset(packet as *mut libc::c_void, 0 as libc::c_int,
           (*dnsmasq_daemon).packet_buff_sz as libc::c_ulong);
    if (*transfer).block == 0 as libc::c_int as libc::c_uint {
        /* send OACK */
        let mut p = 0 as *mut libc::c_char;
        let mut mess = packet as *mut oackmess;
        p = (*mess).data.as_mut_ptr();
        (*mess).op = __bswap_16(6 as libc::c_int as __uint16_t);
        if (*transfer).opt_blocksize != 0 {
            p =
                p.offset((sprintf(p,
                                  b"blksize\x00" as *const u8 as
                                      *const libc::c_char) + 1 as libc::c_int)
                             as isize);
            p =
                p.offset((sprintf(p,
                                  b"%u\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*transfer).blocksize) + 1 as libc::c_int)
                             as isize)
        }
        if (*transfer).opt_transize != 0 {
            p =
                p.offset((sprintf(p,
                                  b"tsize\x00" as *const u8 as
                                      *const libc::c_char) + 1 as libc::c_int)
                             as isize);
            p =
                p.offset((sprintf(p,
                                  b"%u\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*(*transfer).file).size as libc::c_uint) +
                              1 as libc::c_int) as isize)
        }
        return p.wrapping_offset_from(packet) as libc::c_long
    } else {
        /* send data packet */
        let mut mess_0 = packet as *mut datamess; /* finished */
        let mut size =
            ((*(*transfer).file).size - (*transfer).offset) as size_t;
        if (*transfer).offset > (*(*transfer).file).size {
            return 0 as libc::c_int as ssize_t
        }
        if size > (*transfer).blocksize as libc::c_ulong {
            size = (*transfer).blocksize as size_t
        }
        (*mess_0).op = __bswap_16(3 as libc::c_int as __uint16_t);
        (*mess_0).block = __bswap_16((*transfer).block as libc::c_ushort);
        if lseek((*(*transfer).file).fd, (*transfer).offset, SEEK_SET) ==
               -(1 as libc::c_int) as off_t ||
               read_write((*(*transfer).file).fd, (*mess_0).data.as_mut_ptr(),
                          size as libc::c_int, 1 as libc::c_int) == 0 {
            return -(1 as libc::c_int) as ssize_t
        }
        (*transfer).expansion = 0 as libc::c_int as libc::c_uint;
        /* Map '\n' to CR-LF in netascii mode */
        if (*transfer).netascii != 0 {
            let mut i: size_t =
                0; /* don't expand LF again if it moves to the next block */
            let mut newcarrylf: libc::c_int = 0; /* room in this block */
            i = 0 as libc::c_int as size_t;
            newcarrylf = 0 as libc::c_int;
            while i < size {
                if *(*mess_0).data.as_mut_ptr().offset(i as isize) as
                       libc::c_int == '\n' as i32 &&
                       (i != 0 as libc::c_int as libc::c_ulong ||
                            (*transfer).carrylf == 0) {
                    (*transfer).expansion =
                        (*transfer).expansion.wrapping_add(1);
                    if size != (*transfer).blocksize as libc::c_ulong {
                        size = size.wrapping_add(1)
                    } else if i ==
                                  size.wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) {
                        newcarrylf = 1 as libc::c_int
                    }
                    /* make space and insert CR */
                    memmove(&mut *(*mess_0).data.as_mut_ptr().offset(i.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                                         as
                                                                         isize)
                                as *mut libc::c_uchar as *mut libc::c_void,
                            &mut *(*mess_0).data.as_mut_ptr().offset(i as
                                                                         isize)
                                as *mut libc::c_uchar as *const libc::c_void,
                            size.wrapping_sub(i.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)));
                    *(*mess_0).data.as_mut_ptr().offset(i as isize) =
                        '\r' as i32 as libc::c_uchar;
                    i = i.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
            (*transfer).carrylf = newcarrylf as libc::c_char
        }
        return size.wrapping_add(4 as libc::c_int as libc::c_ulong) as ssize_t
    };
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
pub unsafe extern "C" fn do_tftp_script_run() -> libc::c_int {
    let mut transfer = 0 as *mut tftp_transfer;
    transfer = (*dnsmasq_daemon).tftp_done_trans;
    if !transfer.is_null() {
        (*dnsmasq_daemon).tftp_done_trans = (*transfer).next;
        queue_tftp((*(*transfer).file).size,
                   (*(*transfer).file).filename.as_mut_ptr(),
                   &mut (*transfer).peer);
        free_transfer(transfer);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
