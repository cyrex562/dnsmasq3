#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,~
    extern_types,
    main,
    register_tool
)]
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
    #[c2rust::src_loc = "1:9"]
    pub const VERSION: [libc::c_char; 8] =
        unsafe { *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"2.84rc2\x00") };
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:20"]
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
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
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
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/types.h:20"]
pub mod sys_types_h {
    #[c2rust::src_loc = "49:1"]
    pub type ino_t = __ino64_t;
    #[c2rust::src_loc = "59:1"]
    pub type dev_t = __dev_t;
    #[c2rust::src_loc = "64:1"]
    pub type gid_t = __gid_t;
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    #[c2rust::src_loc = "87:1"]
    pub type off_t = __off64_t;
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::{__dev_t, __gid_t, __ino64_t, __off64_t, __pid_t, __ssize_t, __uid_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:20"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/llvm-10/lib/clang/10.0.0/include/stddef.h:20"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/__sigset_t.h:20"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/sigset_t.h:20"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_timespec.h:20"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:20"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/socket.h:20"]
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
    pub unsafe extern "C" fn __cmsg_nxthdr(
        mut __mhdr: *mut msghdr,
        mut __cmsg: *mut cmsghdr,
    ) -> *mut cmsghdr {
        if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            return 0 as *mut cmsghdr;
        }
        __cmsg = (__cmsg as *mut libc::c_uchar).offset(
            ((*__cmsg)
                .cmsg_len
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut cmsghdr;
        if __cmsg.offset(1 as libc::c_int as isize) as *mut libc::c_uchar
            > ((*__mhdr).msg_control as *mut libc::c_uchar)
                .offset((*__mhdr).msg_controllen as isize)
            || (__cmsg as *mut libc::c_uchar).offset(
                ((*__cmsg)
                    .cmsg_len
                    .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) > ((*__mhdr).msg_control as *mut libc::c_uchar)
                .offset((*__mhdr).msg_controllen as isize)
        {
            return 0 as *mut cmsghdr;
        }
        return __cmsg;
    }
    #[c2rust::src_loc = "104:9"]
    pub const AF_INET6: libc::c_int = PF_INET6;
    #[c2rust::src_loc = "53:9"]
    pub const PF_INET6: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const AF_INET: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const PF_INET: libc::c_int = 2 as libc::c_int;
    use super::sockaddr_h::sa_family_t;
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::types_h::__socklen_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/socket_type.h:20"]
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
    #[c2rust::src_loc = "33:9"]
    pub const SOCK_RAW_0: libc::c_int = SOCK_RAW as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/sockaddr.h:20"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/socket.h:20"]
pub mod sys_socket_h {
    #[c2rust::src_loc = "41:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "47:3"]
    pub const SHUT_RDWR: C2RustUnnamed = 2;
    #[c2rust::src_loc = "45:3"]
    pub const SHUT_WR: C2RustUnnamed = 1;
    #[c2rust::src_loc = "43:3"]
    pub const SHUT_RD: C2RustUnnamed = 0;
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
    #[c2rust::src_loc = "48:9"]
    pub const SHUT_RDWR_0: libc::c_int = SHUT_RDWR as libc::c_int;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::socket_h::{sockaddr, socklen_t};
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
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
        pub fn socket(
            __domain: libc::c_int,
            __type: libc::c_int,
            __protocol: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(
            __fd: libc::c_int,
            __addr: __SOCKADDR_ARG,
            __len: *mut socklen_t,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn sendto(
            __fd: libc::c_int,
            __buf: *const libc::c_void,
            __n: size_t,
            __flags: libc::c_int,
            __addr: __CONST_SOCKADDR_ARG,
            __addr_len: socklen_t,
        ) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "163:1"]
        pub fn recvfrom(
            __fd: libc::c_int,
            __buf: *mut libc::c_void,
            __n: size_t,
            __flags: libc::c_int,
            __addr: __SOCKADDR_ARG,
            __addr_len: *mut socklen_t,
        ) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(
            __fd: libc::c_int,
            __level: libc::c_int,
            __optname: libc::c_int,
            __optval: *const libc::c_void,
            __optlen: socklen_t,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn accept(
            __fd: libc::c_int,
            __addr: __SOCKADDR_ARG,
            __addr_len: *mut socklen_t,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "250:1"]
        pub fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/un.h:20"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:20"]
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
        pub __in6_u: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed_0 {
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
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
    #[c2rust::src_loc = "234:9"]
    pub const INET6_ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const IPPROTO_ICMP_0: libc::c_int = IPPROTO_ICMP as libc::c_int;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:20"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dnsmasq.h:20"]
pub mod dnsmasq_h {
    #[c2rust::src_loc = "69:1"]
    pub type u16_0 = libc::c_ushort;
    #[c2rust::src_loc = "70:1"]
    pub type u32_0 = libc::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "174:8"]
    pub struct event_desc {
        pub event: libc::c_int,
        pub data: libc::c_int,
        pub msg_sz: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "295:7"]
    pub union all_addr {
        pub addr4: in_addr,
        pub addr6: in6_addr,
        pub cname: C2RustUnnamed_22,
        pub key: C2RustUnnamed_21,
        pub ds: C2RustUnnamed_20,
        pub srv: C2RustUnnamed_19,
        pub log: C2RustUnnamed_18,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "322:3"]
    pub struct C2RustUnnamed_18 {
        pub keytag: libc::c_ushort,
        pub algo: libc::c_ushort,
        pub digest: libc::c_ushort,
        pub rcode: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:3"]
    pub struct C2RustUnnamed_19 {
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
    pub struct C2RustUnnamed_20 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
        pub digest: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:3"]
    pub struct C2RustUnnamed_21 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub flags: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:3"]
    pub struct C2RustUnnamed_22 {
        pub target: C2RustUnnamed_23,
        pub uid: libc::c_uint,
        pub is_name_ptr: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:5"]
    pub union C2RustUnnamed_23 {
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
        pub name: C2RustUnnamed_24,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "455:3"]
    pub union C2RustUnnamed_24 {
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
        pub u: C2RustUnnamed_25,
        pub val: *mut libc::c_uchar,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_opt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "815:3"]
    pub union C2RustUnnamed_25 {
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
    #[c2rust::src_loc = "171:9"]
    pub const ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "208:9"]
    pub const EC_BADNET: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "151:9"]
    pub const LINUX_CAPABILITY_VERSION_1: libc::c_int = 0x19980330 as libc::c_int;
    #[c2rust::src_loc = "152:9"]
    pub const LINUX_CAPABILITY_VERSION_2: libc::c_int = 0x20071026 as libc::c_int;
    #[c2rust::src_loc = "153:9"]
    pub const LINUX_CAPABILITY_VERSION_3: libc::c_int = 0x20080522 as libc::c_int;
    #[c2rust::src_loc = "287:9"]
    pub const MS_DHCP: libc::c_int = LOG_DAEMON;
    #[c2rust::src_loc = "286:9"]
    pub const MS_TFTP: libc::c_int = LOG_USER;
    #[c2rust::src_loc = "198:9"]
    pub const EVENT_INIT: libc::c_int = 21;
    #[c2rust::src_loc = "185:9"]
    pub const EVENT_KILLED: libc::c_int = 8;
    #[c2rust::src_loc = "184:9"]
    pub const EVENT_EXITED: libc::c_int = 7;
    #[c2rust::src_loc = "186:9"]
    pub const EVENT_EXEC_ERR: libc::c_int = 9;
    #[c2rust::src_loc = "202:9"]
    pub const EVENT_SCRIPT_LOG: libc::c_int = 25;
    #[c2rust::src_loc = "288:9"]
    pub const MS_SCRIPT: libc::c_int = LOG_MAIL;
    #[c2rust::src_loc = "199:9"]
    pub const EVENT_NEWADDR: libc::c_int = 22;
    #[c2rust::src_loc = "200:9"]
    pub const EVENT_NEWROUTE: libc::c_int = 23;
    #[c2rust::src_loc = "206:9"]
    pub const EC_GOOD: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "178:9"]
    pub const EVENT_RELOAD: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "182:9"]
    pub const EVENT_CHILD: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "181:9"]
    pub const EVENT_TERM: libc::c_int = 4;
    #[c2rust::src_loc = "179:9"]
    pub const EVENT_DUMP: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "183:9"]
    pub const EVENT_REOPEN: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "203:9"]
    pub const EVENT_TIME: libc::c_int = 26;
    #[c2rust::src_loc = "180:9"]
    pub const EVENT_ALARM: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "193:9"]
    pub const EVENT_DIE: libc::c_int = 16;
    #[c2rust::src_loc = "195:9"]
    pub const EVENT_FORK_ERR: libc::c_int = 18 as libc::c_int;
    #[c2rust::src_loc = "187:9"]
    pub const EVENT_PIPE_ERR: libc::c_int = 10;
    #[c2rust::src_loc = "189:9"]
    pub const EVENT_CAP_ERR: libc::c_int = 12;
    #[c2rust::src_loc = "188:9"]
    pub const EVENT_USER_ERR: libc::c_int = 11;
    #[c2rust::src_loc = "192:9"]
    pub const EVENT_GROUP_ERR: libc::c_int = 15;
    #[c2rust::src_loc = "190:9"]
    pub const EVENT_PIDFILE: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "194:9"]
    pub const EVENT_LOG_ERR: libc::c_int = 17;
    #[c2rust::src_loc = "196:9"]
    pub const EVENT_LUA_ERR: libc::c_int = 19;
    #[c2rust::src_loc = "211:9"]
    pub const EC_MISC: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "197:9"]
    pub const EVENT_TFTP_ERR: libc::c_int = 20 as libc::c_int;
    #[c2rust::src_loc = "209:9"]
    pub const EC_FILE: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "201:9"]
    pub const EVENT_TIME_ERR: libc::c_int = 24;
    #[c2rust::src_loc = "207:9"]
    pub const EC_BADCONF: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "949:9"]
    pub const CONTEXT_DHCP: libc::c_uint = (1 as libc::c_uint) << 8 as libc::c_int;
    #[c2rust::src_loc = "954:9"]
    pub const CONTEXT_RA: libc::c_uint = (1 as libc::c_uint) << 13 as libc::c_int;
    use super::capability_h::{
        __user_cap_data_struct, __user_cap_header_struct, cap_user_data_t, cap_user_header_t,
    };
    use super::in_h::{in6_addr, in_addr, in_addr_t, sockaddr_in, sockaddr_in6};
    use super::pwd_h::passwd;
    use super::socket_h::sockaddr;
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::sys_types_h::{dev_t, gid_t, ino_t, off_t, pid_t, ssize_t, uid_t};
    use super::syslog_h::{LOG_DAEMON, LOG_MAIL, LOG_USER};
    use super::time_t_h::time_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn capset(header: cap_user_header_t, data: cap_user_data_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn capget(header: cap_user_header_t, data: cap_user_data_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1296:1"]
        pub fn dnsmasq_time() -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "1302:1"]
        pub fn retry_send(rc: ssize_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1290:1"]
        pub fn safe_pipe(fd: *mut libc::c_int, read_noblock: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1175:1"]
        pub fn cache_init();
        #[no_mangle]
        #[c2rust::src_loc = "1188:1"]
        pub fn cache_recv_insert(now: time_t, fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1191:1"]
        pub fn cache_reload();
        #[no_mangle]
        #[c2rust::src_loc = "1195:1"]
        pub fn dump_cache(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1206:1"]
        pub fn blockdata_init();
        #[no_mangle]
        #[c2rust::src_loc = "1293:1"]
        pub fn sockaddr_isequal(s1: *mut mysockaddr, s2: *mut mysockaddr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1288:1"]
        pub fn safe_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1268:1"]
        pub fn hash_questions_init();
        #[no_mangle]
        #[c2rust::src_loc = "1282:1"]
        pub fn rand16() -> libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "1281:1"]
        pub fn rand_init();
        #[no_mangle]
        #[c2rust::src_loc = "1511:1"]
        pub fn ipset_init();
        #[no_mangle]
        #[c2rust::src_loc = "1446:1"]
        pub fn lease_find_interfaces(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1311:1"]
        pub fn read_write(
            fd: libc::c_int,
            packet: *mut libc::c_uchar,
            size: libc::c_int,
            rw: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1444:1"]
        pub fn do_script_run(now: time_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1312:1"]
        pub fn close_fds(
            max_fd: libc::c_long,
            spare1: libc::c_int,
            spare2: libc::c_int,
            spare3: libc::c_int,
        );
        #[no_mangle]
        #[c2rust::src_loc = "1316:1"]
        pub fn kernel_version() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1443:1"]
        pub fn lease_update_from_configs();
        #[no_mangle]
        #[c2rust::src_loc = "1320:1"]
        pub fn die(
            message: *mut libc::c_char,
            arg1: *mut libc::c_char,
            exit_code: libc::c_int,
        ) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "1321:1"]
        pub fn log_start(ent_pw: *mut passwd, errfd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1322:1"]
        pub fn log_reopen(log_file: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1442:1"]
        pub fn lease_prune(target: *mut dhcp_lease, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1430:1"]
        pub fn lease_make_duid(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1326:1"]
        pub fn set_log_writer();
        #[no_mangle]
        #[c2rust::src_loc = "1327:1"]
        pub fn check_log_writer(force: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1328:1"]
        pub fn flush_log();
        #[no_mangle]
        #[c2rust::src_loc = "1331:1"]
        pub fn read_opts(
            argc: libc::c_int,
            argv: *mut *mut libc::c_char,
            compile_opts_0: *mut libc::c_char,
        );
        #[no_mangle]
        #[c2rust::src_loc = "1475:1"]
        pub fn netlink_init() -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1476:1"]
        pub fn netlink_multicast();
        #[no_mangle]
        #[c2rust::src_loc = "1334:1"]
        pub fn reread_dhcp();
        #[no_mangle]
        #[c2rust::src_loc = "1335:1"]
        pub fn read_servers_file();
        #[no_mangle]
        #[c2rust::src_loc = "1344:1"]
        pub fn reply_query(fd: libc::c_int, family: libc::c_int, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1345:1"]
        pub fn receive_query(listen: *mut listener, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1346:1"]
        pub fn tcp_request(
            confd: libc::c_int,
            now: time_t,
            local_addr: *mut mysockaddr,
            netmask: in_addr,
            auth_dns: libc::c_int,
        ) -> *mut libc::c_uchar;
        #[no_mangle]
        #[c2rust::src_loc = "1349:1"]
        pub fn get_new_frec(now: time_t, wait: *mut libc::c_int, force: *mut frec) -> *mut frec;
        #[no_mangle]
        #[c2rust::src_loc = "1353:1"]
        pub fn resend_query();
        #[no_mangle]
        #[c2rust::src_loc = "1358:1"]
        pub fn indextoname(
            fd: libc::c_int,
            index: libc::c_int,
            name: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1361:1"]
        pub fn pre_allocate_sfds();
        #[no_mangle]
        #[c2rust::src_loc = "1362:1"]
        pub fn reload_servers(fname: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1370:1"]
        pub fn check_servers();
        #[no_mangle]
        #[c2rust::src_loc = "1371:1"]
        pub fn enumerate_interfaces(reset: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1372:1"]
        pub fn create_wildcard_listeners();
        #[no_mangle]
        #[c2rust::src_loc = "1373:1"]
        pub fn create_bound_listeners(dienow: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1374:1"]
        pub fn warn_bound_listeners();
        #[no_mangle]
        #[c2rust::src_loc = "1375:1"]
        pub fn warn_wild_labels();
        #[no_mangle]
        #[c2rust::src_loc = "1376:1"]
        pub fn warn_int_names();
        #[no_mangle]
        #[c2rust::src_loc = "1377:1"]
        pub fn is_dad_listeners() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1379:1"]
        pub fn loopback_exception(
            fd: libc::c_int,
            family: libc::c_int,
            addr: *mut all_addr,
            name: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1381:1"]
        pub fn fix_fd(fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1382:1"]
        pub fn tcp_interface(fd: libc::c_int, af: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1385:1"]
        pub fn join_multicast(dienow: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1388:1"]
        pub fn newaddress(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1394:1"]
        pub fn dhcp_init();
        #[no_mangle]
        #[c2rust::src_loc = "1395:1"]
        pub fn dhcp_packet(now: time_t, pxe_fd: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1407:1"]
        pub fn dhcp_read_ethers();
        #[no_mangle]
        #[c2rust::src_loc = "1414:1"]
        pub fn lease_update_file(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1415:1"]
        pub fn lease_update_dns(force: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1416:1"]
        pub fn lease_init(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1445:1"]
        pub fn rerun_scripts();
        #[no_mangle]
        #[c2rust::src_loc = "1626:1"]
        pub fn periodic_ra(now: time_t) -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "1625:1"]
        pub fn icmp6_packet(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1532:1"]
        pub fn check_tftp_listeners(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1531:1"]
        pub fn tftp_request(listen: *mut listener, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1586:1"]
        pub fn dhcp_update_configs(configs: *mut dhcp_config);
        #[no_mangle]
        #[c2rust::src_loc = "1517:1"]
        pub fn create_helper(
            event_fd: libc::c_int,
            err_fd: libc::c_int,
            uid: uid_t,
            gid: gid_t,
            max_fd: libc::c_long,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1518:1"]
        pub fn helper_write();
        #[no_mangle]
        #[c2rust::src_loc = "1526:1"]
        pub fn helper_buf_empty() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1533:1"]
        pub fn do_tftp_script_run() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1544:1"]
        pub fn dhcp6_init();
        #[no_mangle]
        #[c2rust::src_loc = "1545:1"]
        pub fn dhcp6_packet(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1624:1"]
        pub fn ra_init(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1559:1"]
        pub fn dhcp_construct_contexts(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1605:1"]
        pub fn log_relay(family: libc::c_int, relay: *mut dhcp_relay);
        #[no_mangle]
        #[c2rust::src_loc = "1604:1"]
        pub fn log_context(family: libc::c_int, context: *mut dhcp_context);
        #[no_mangle]
        #[c2rust::src_loc = "1599:1"]
        pub fn bindtodevice(device: *mut libc::c_char, fd: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1598:1"]
        pub fn whichdevice() -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1577:1"]
        pub fn dhcp_common_init();
        #[no_mangle]
        #[c2rust::src_loc = "1652:1"]
        pub fn poll_check(fd: libc::c_int, event: libc::c_short) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1654:1"]
        pub fn do_poll(timeout: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1653:1"]
        pub fn poll_listen(fd: libc::c_int, event: libc::c_short);
        #[no_mangle]
        #[c2rust::src_loc = "1651:1"]
        pub fn poll_reset();
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn inotify_dnsmasq_init();
        #[no_mangle]
        #[c2rust::src_loc = "1646:1"]
        pub fn inotify_check(now: time_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1672:1"]
        pub fn find_mac(
            addr: *mut mysockaddr,
            mac: *mut libc::c_uchar,
            lazy: libc::c_int,
            now: time_t,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1673:1"]
        pub fn do_arp_script_run() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1677:1"]
        pub fn dump_init();
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stat.h:20"]
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
    #[c2rust::src_loc = "202:9"]
    pub const __S_IWRITE: libc::c_int = 0o200 as libc::c_int;
    #[c2rust::src_loc = "201:9"]
    pub const __S_IREAD: libc::c_int = 0o400 as libc::c_int;
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt64_t, __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino64_t, __ino_t, __mode_t,
        __nlink_t, __off_t, __syscall_slong_t, __uid_t,
    };
}
#[c2rust::header_src = "/usr/include/asm-generic/int-ll64.h:20"]
pub mod int_ll64_h {
    #[c2rust::src_loc = "27:1"]
    pub type __u32 = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/__sigval_t.h:20"]
pub mod __sigval_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/siginfo_t.h:20"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed_2 {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_11,
        pub _timer: C2RustUnnamed_10,
        pub _rt: C2RustUnnamed_9,
        pub _sigchld: C2RustUnnamed_8,
        pub _sigfault: C2RustUnnamed_5,
        pub _sigpoll: C2RustUnnamed_4,
        pub _sigsys: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_3 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_4 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_5 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_6 {
        pub _addr_bnd: C2RustUnnamed_7,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_7 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_8 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_9 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_10 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_11 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::__sigval_t_h::__sigval_t;
    use super::types_h::{__clock_t, __pid_t, __uid_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/signal.h:20"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::sigaction_h::sigaction;
    use super::sigset_t_h::sigset_t;
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(
            __sig: libc::c_int,
            __act: *const sigaction,
            __oact: *mut sigaction,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/sigaction.h:20"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_12,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_12 {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option<
            unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> (),
        >,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::siginfo_t_h::siginfo_t;
    use super::signal_h::__sighandler_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/confname.h:20"]
pub mod confname_h {
    #[c2rust::src_loc = "71:1"]
    pub type C2RustUnnamed_13 = libc::c_uint;
    #[c2rust::src_loc = "528:5"]
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_13 = 248;
    #[c2rust::src_loc = "526:5"]
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_13 = 247;
    #[c2rust::src_loc = "523:5"]
    pub const _SC_XOPEN_STREAMS: C2RustUnnamed_13 = 246;
    #[c2rust::src_loc = "520:5"]
    pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_13 = 245;
    #[c2rust::src_loc = "518:5"]
    pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_13 = 244;
    #[c2rust::src_loc = "516:5"]
    pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_13 = 243;
    #[c2rust::src_loc = "514:5"]
    pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_13 = 242;
    #[c2rust::src_loc = "511:5"]
    pub const _SC_SS_REPL_MAX: C2RustUnnamed_13 = 241;
    #[c2rust::src_loc = "508:5"]
    pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_13 = 240;
    #[c2rust::src_loc = "506:5"]
    pub const _SC_V7_LP64_OFF64: C2RustUnnamed_13 = 239;
    #[c2rust::src_loc = "504:5"]
    pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_13 = 238;
    #[c2rust::src_loc = "502:5"]
    pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_13 = 237;
    #[c2rust::src_loc = "499:5"]
    pub const _SC_RAW_SOCKETS: C2RustUnnamed_13 = 236;
    #[c2rust::src_loc = "497:5"]
    pub const _SC_IPV6: C2RustUnnamed_13 = 235;
    #[c2rust::src_loc = "493:5"]
    pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_13 = 199;
    #[c2rust::src_loc = "491:5"]
    pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_13 = 198;
    #[c2rust::src_loc = "489:5"]
    pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_13 = 197;
    #[c2rust::src_loc = "487:5"]
    pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_13 = 196;
    #[c2rust::src_loc = "485:5"]
    pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_13 = 195;
    #[c2rust::src_loc = "483:5"]
    pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_13 = 194;
    #[c2rust::src_loc = "481:5"]
    pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_13 = 193;
    #[c2rust::src_loc = "479:5"]
    pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_13 = 192;
    #[c2rust::src_loc = "477:5"]
    pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_13 = 191;
    #[c2rust::src_loc = "475:5"]
    pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_13 = 190;
    #[c2rust::src_loc = "473:5"]
    pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_13 = 189;
    #[c2rust::src_loc = "471:5"]
    pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_13 = 188;
    #[c2rust::src_loc = "469:5"]
    pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_13 = 187;
    #[c2rust::src_loc = "467:5"]
    pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_13 = 186;
    #[c2rust::src_loc = "465:5"]
    pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_13 = 185;
    #[c2rust::src_loc = "462:5"]
    pub const _SC_TRACE_LOG: C2RustUnnamed_13 = 184;
    #[c2rust::src_loc = "460:5"]
    pub const _SC_TRACE_INHERIT: C2RustUnnamed_13 = 183;
    #[c2rust::src_loc = "458:5"]
    pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_13 = 182;
    #[c2rust::src_loc = "456:5"]
    pub const _SC_TRACE: C2RustUnnamed_13 = 181;
    #[c2rust::src_loc = "454:5"]
    pub const _SC_HOST_NAME_MAX: C2RustUnnamed_13 = 180;
    #[c2rust::src_loc = "451:5"]
    pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_13 = 179;
    #[c2rust::src_loc = "449:5"]
    pub const _SC_V6_LP64_OFF64: C2RustUnnamed_13 = 178;
    #[c2rust::src_loc = "447:5"]
    pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_13 = 177;
    #[c2rust::src_loc = "445:5"]
    pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_13 = 176;
    #[c2rust::src_loc = "442:5"]
    pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_13 = 175;
    #[c2rust::src_loc = "440:5"]
    pub const _SC_STREAMS: C2RustUnnamed_13 = 174;
    #[c2rust::src_loc = "438:5"]
    pub const _SC_SYMLOOP_MAX: C2RustUnnamed_13 = 173;
    #[c2rust::src_loc = "436:5"]
    pub const _SC_2_PBS_TRACK: C2RustUnnamed_13 = 172;
    #[c2rust::src_loc = "434:5"]
    pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_13 = 171;
    #[c2rust::src_loc = "432:5"]
    pub const _SC_2_PBS_LOCATE: C2RustUnnamed_13 = 170;
    #[c2rust::src_loc = "430:5"]
    pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_13 = 169;
    #[c2rust::src_loc = "428:5"]
    pub const _SC_2_PBS: C2RustUnnamed_13 = 168;
    #[c2rust::src_loc = "426:5"]
    pub const _SC_USER_GROUPS_R: C2RustUnnamed_13 = 167;
    #[c2rust::src_loc = "424:5"]
    pub const _SC_USER_GROUPS: C2RustUnnamed_13 = 166;
    #[c2rust::src_loc = "422:5"]
    pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_13 = 165;
    #[c2rust::src_loc = "420:5"]
    pub const _SC_TIMEOUTS: C2RustUnnamed_13 = 164;
    #[c2rust::src_loc = "418:5"]
    pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_13 = 163;
    #[c2rust::src_loc = "416:5"]
    pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_13 = 162;
    #[c2rust::src_loc = "414:5"]
    pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_13 = 161;
    #[c2rust::src_loc = "412:5"]
    pub const _SC_SPORADIC_SERVER: C2RustUnnamed_13 = 160;
    #[c2rust::src_loc = "410:5"]
    pub const _SC_SPAWN: C2RustUnnamed_13 = 159;
    #[c2rust::src_loc = "408:5"]
    pub const _SC_SIGNALS: C2RustUnnamed_13 = 158;
    #[c2rust::src_loc = "406:5"]
    pub const _SC_SHELL: C2RustUnnamed_13 = 157;
    #[c2rust::src_loc = "404:5"]
    pub const _SC_REGEX_VERSION: C2RustUnnamed_13 = 156;
    #[c2rust::src_loc = "402:5"]
    pub const _SC_REGEXP: C2RustUnnamed_13 = 155;
    #[c2rust::src_loc = "400:5"]
    pub const _SC_SPIN_LOCKS: C2RustUnnamed_13 = 154;
    #[c2rust::src_loc = "398:5"]
    pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_13 = 153;
    #[c2rust::src_loc = "396:5"]
    pub const _SC_NETWORKING: C2RustUnnamed_13 = 152;
    #[c2rust::src_loc = "394:5"]
    pub const _SC_SINGLE_PROCESS: C2RustUnnamed_13 = 151;
    #[c2rust::src_loc = "392:5"]
    pub const _SC_MULTI_PROCESS: C2RustUnnamed_13 = 150;
    #[c2rust::src_loc = "390:5"]
    pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_13 = 149;
    #[c2rust::src_loc = "388:5"]
    pub const _SC_FILE_SYSTEM: C2RustUnnamed_13 = 148;
    #[c2rust::src_loc = "386:5"]
    pub const _SC_FILE_LOCKING: C2RustUnnamed_13 = 147;
    #[c2rust::src_loc = "384:5"]
    pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_13 = 146;
    #[c2rust::src_loc = "382:5"]
    pub const _SC_PIPE: C2RustUnnamed_13 = 145;
    #[c2rust::src_loc = "380:5"]
    pub const _SC_FIFO: C2RustUnnamed_13 = 144;
    #[c2rust::src_loc = "378:5"]
    pub const _SC_FD_MGMT: C2RustUnnamed_13 = 143;
    #[c2rust::src_loc = "376:5"]
    pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_13 = 142;
    #[c2rust::src_loc = "374:5"]
    pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_13 = 141;
    #[c2rust::src_loc = "372:5"]
    pub const _SC_DEVICE_IO: C2RustUnnamed_13 = 140;
    #[c2rust::src_loc = "370:5"]
    pub const _SC_THREAD_CPUTIME: C2RustUnnamed_13 = 139;
    #[c2rust::src_loc = "368:5"]
    pub const _SC_CPUTIME: C2RustUnnamed_13 = 138;
    #[c2rust::src_loc = "366:5"]
    pub const _SC_CLOCK_SELECTION: C2RustUnnamed_13 = 137;
    #[c2rust::src_loc = "364:5"]
    pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_13 = 136;
    #[c2rust::src_loc = "362:5"]
    pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_13 = 135;
    #[c2rust::src_loc = "360:5"]
    pub const _SC_BASE: C2RustUnnamed_13 = 134;
    #[c2rust::src_loc = "358:5"]
    pub const _SC_BARRIERS: C2RustUnnamed_13 = 133;
    #[c2rust::src_loc = "356:5"]
    pub const _SC_ADVISORY_INFO: C2RustUnnamed_13 = 132;
    #[c2rust::src_loc = "353:5"]
    pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_13 = 131;
    #[c2rust::src_loc = "351:5"]
    pub const _SC_XOPEN_REALTIME: C2RustUnnamed_13 = 130;
    #[c2rust::src_loc = "349:5"]
    pub const _SC_XOPEN_LEGACY: C2RustUnnamed_13 = 129;
    #[c2rust::src_loc = "346:5"]
    pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_13 = 128;
    #[c2rust::src_loc = "344:5"]
    pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_13 = 127;
    #[c2rust::src_loc = "342:5"]
    pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_13 = 126;
    #[c2rust::src_loc = "340:5"]
    pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_13 = 125;
    #[c2rust::src_loc = "337:5"]
    pub const _SC_NL_TEXTMAX: C2RustUnnamed_13 = 124;
    #[c2rust::src_loc = "335:5"]
    pub const _SC_NL_SETMAX: C2RustUnnamed_13 = 123;
    #[c2rust::src_loc = "333:5"]
    pub const _SC_NL_NMAX: C2RustUnnamed_13 = 122;
    #[c2rust::src_loc = "331:5"]
    pub const _SC_NL_MSGMAX: C2RustUnnamed_13 = 121;
    #[c2rust::src_loc = "329:5"]
    pub const _SC_NL_LANGMAX: C2RustUnnamed_13 = 120;
    #[c2rust::src_loc = "327:5"]
    pub const _SC_NL_ARGMAX: C2RustUnnamed_13 = 119;
    #[c2rust::src_loc = "324:5"]
    pub const _SC_USHRT_MAX: C2RustUnnamed_13 = 118;
    #[c2rust::src_loc = "322:5"]
    pub const _SC_ULONG_MAX: C2RustUnnamed_13 = 117;
    #[c2rust::src_loc = "320:5"]
    pub const _SC_UINT_MAX: C2RustUnnamed_13 = 116;
    #[c2rust::src_loc = "318:5"]
    pub const _SC_UCHAR_MAX: C2RustUnnamed_13 = 115;
    #[c2rust::src_loc = "316:5"]
    pub const _SC_SHRT_MIN: C2RustUnnamed_13 = 114;
    #[c2rust::src_loc = "314:5"]
    pub const _SC_SHRT_MAX: C2RustUnnamed_13 = 113;
    #[c2rust::src_loc = "312:5"]
    pub const _SC_SCHAR_MIN: C2RustUnnamed_13 = 112;
    #[c2rust::src_loc = "310:5"]
    pub const _SC_SCHAR_MAX: C2RustUnnamed_13 = 111;
    #[c2rust::src_loc = "308:5"]
    pub const _SC_SSIZE_MAX: C2RustUnnamed_13 = 110;
    #[c2rust::src_loc = "306:5"]
    pub const _SC_NZERO: C2RustUnnamed_13 = 109;
    #[c2rust::src_loc = "304:5"]
    pub const _SC_MB_LEN_MAX: C2RustUnnamed_13 = 108;
    #[c2rust::src_loc = "302:5"]
    pub const _SC_WORD_BIT: C2RustUnnamed_13 = 107;
    #[c2rust::src_loc = "300:5"]
    pub const _SC_LONG_BIT: C2RustUnnamed_13 = 106;
    #[c2rust::src_loc = "298:5"]
    pub const _SC_INT_MIN: C2RustUnnamed_13 = 105;
    #[c2rust::src_loc = "296:5"]
    pub const _SC_INT_MAX: C2RustUnnamed_13 = 104;
    #[c2rust::src_loc = "294:5"]
    pub const _SC_CHAR_MIN: C2RustUnnamed_13 = 103;
    #[c2rust::src_loc = "292:5"]
    pub const _SC_CHAR_MAX: C2RustUnnamed_13 = 102;
    #[c2rust::src_loc = "290:5"]
    pub const _SC_CHAR_BIT: C2RustUnnamed_13 = 101;
    #[c2rust::src_loc = "287:5"]
    pub const _SC_XOPEN_XPG4: C2RustUnnamed_13 = 100;
    #[c2rust::src_loc = "285:5"]
    pub const _SC_XOPEN_XPG3: C2RustUnnamed_13 = 99;
    #[c2rust::src_loc = "283:5"]
    pub const _SC_XOPEN_XPG2: C2RustUnnamed_13 = 98;
    #[c2rust::src_loc = "280:5"]
    pub const _SC_2_UPE: C2RustUnnamed_13 = 97;
    #[c2rust::src_loc = "278:5"]
    pub const _SC_2_C_VERSION: C2RustUnnamed_13 = 96;
    #[c2rust::src_loc = "276:5"]
    pub const _SC_2_CHAR_TERM: C2RustUnnamed_13 = 95;
    #[c2rust::src_loc = "273:5"]
    pub const _SC_XOPEN_SHM: C2RustUnnamed_13 = 94;
    #[c2rust::src_loc = "271:5"]
    pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_13 = 93;
    #[c2rust::src_loc = "269:5"]
    pub const _SC_XOPEN_CRYPT: C2RustUnnamed_13 = 92;
    #[c2rust::src_loc = "267:5"]
    pub const _SC_XOPEN_UNIX: C2RustUnnamed_13 = 91;
    #[c2rust::src_loc = "265:5"]
    pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_13 = 90;
    #[c2rust::src_loc = "263:5"]
    pub const _SC_XOPEN_VERSION: C2RustUnnamed_13 = 89;
    #[c2rust::src_loc = "260:5"]
    pub const _SC_PASS_MAX: C2RustUnnamed_13 = 88;
    #[c2rust::src_loc = "258:5"]
    pub const _SC_ATEXIT_MAX: C2RustUnnamed_13 = 87;
    #[c2rust::src_loc = "256:5"]
    pub const _SC_AVPHYS_PAGES: C2RustUnnamed_13 = 86;
    #[c2rust::src_loc = "254:5"]
    pub const _SC_PHYS_PAGES: C2RustUnnamed_13 = 85;
    #[c2rust::src_loc = "252:5"]
    pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_13 = 84;
    #[c2rust::src_loc = "250:5"]
    pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_13 = 83;
    #[c2rust::src_loc = "247:5"]
    pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_13 = 82;
    #[c2rust::src_loc = "245:5"]
    pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_13 = 81;
    #[c2rust::src_loc = "243:5"]
    pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_13 = 80;
    #[c2rust::src_loc = "241:5"]
    pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_13 = 79;
    #[c2rust::src_loc = "239:5"]
    pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_13 = 78;
    #[c2rust::src_loc = "237:5"]
    pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_13 = 77;
    #[c2rust::src_loc = "235:5"]
    pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_13 = 76;
    #[c2rust::src_loc = "233:5"]
    pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_13 = 75;
    #[c2rust::src_loc = "231:5"]
    pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_13 = 74;
    #[c2rust::src_loc = "229:5"]
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_13 = 73;
    #[c2rust::src_loc = "227:5"]
    pub const _SC_TTY_NAME_MAX: C2RustUnnamed_13 = 72;
    #[c2rust::src_loc = "225:5"]
    pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_13 = 71;
    #[c2rust::src_loc = "223:5"]
    pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_13 = 70;
    #[c2rust::src_loc = "221:5"]
    pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_13 = 69;
    #[c2rust::src_loc = "219:5"]
    pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_13 = 68;
    #[c2rust::src_loc = "217:5"]
    pub const _SC_THREADS: C2RustUnnamed_13 = 67;
    #[c2rust::src_loc = "213:5"]
    pub const _SC_T_IOV_MAX: C2RustUnnamed_13 = 66;
    #[c2rust::src_loc = "211:5"]
    pub const _SC_PII_OSI_M: C2RustUnnamed_13 = 65;
    #[c2rust::src_loc = "209:5"]
    pub const _SC_PII_OSI_CLTS: C2RustUnnamed_13 = 64;
    #[c2rust::src_loc = "207:5"]
    pub const _SC_PII_OSI_COTS: C2RustUnnamed_13 = 63;
    #[c2rust::src_loc = "205:5"]
    pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_13 = 62;
    #[c2rust::src_loc = "203:5"]
    pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_13 = 61;
    #[c2rust::src_loc = "201:5"]
    pub const _SC_IOV_MAX: C2RustUnnamed_13 = 60;
    #[c2rust::src_loc = "199:5"]
    pub const _SC_UIO_MAXIOV: C2RustUnnamed_13 = 60;
    #[c2rust::src_loc = "197:5"]
    pub const _SC_SELECT: C2RustUnnamed_13 = 59;
    #[c2rust::src_loc = "195:5"]
    pub const _SC_POLL: C2RustUnnamed_13 = 58;
    #[c2rust::src_loc = "193:5"]
    pub const _SC_PII_OSI: C2RustUnnamed_13 = 57;
    #[c2rust::src_loc = "191:5"]
    pub const _SC_PII_INTERNET: C2RustUnnamed_13 = 56;
    #[c2rust::src_loc = "189:5"]
    pub const _SC_PII_SOCKET: C2RustUnnamed_13 = 55;
    #[c2rust::src_loc = "187:5"]
    pub const _SC_PII_XTI: C2RustUnnamed_13 = 54;
    #[c2rust::src_loc = "185:5"]
    pub const _SC_PII: C2RustUnnamed_13 = 53;
    #[c2rust::src_loc = "182:5"]
    pub const _SC_2_LOCALEDEF: C2RustUnnamed_13 = 52;
    #[c2rust::src_loc = "180:5"]
    pub const _SC_2_SW_DEV: C2RustUnnamed_13 = 51;
    #[c2rust::src_loc = "178:5"]
    pub const _SC_2_FORT_RUN: C2RustUnnamed_13 = 50;
    #[c2rust::src_loc = "176:5"]
    pub const _SC_2_FORT_DEV: C2RustUnnamed_13 = 49;
    #[c2rust::src_loc = "174:5"]
    pub const _SC_2_C_DEV: C2RustUnnamed_13 = 48;
    #[c2rust::src_loc = "172:5"]
    pub const _SC_2_C_BIND: C2RustUnnamed_13 = 47;
    #[c2rust::src_loc = "170:5"]
    pub const _SC_2_VERSION: C2RustUnnamed_13 = 46;
    #[c2rust::src_loc = "167:5"]
    pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_13 = 45;
    #[c2rust::src_loc = "165:5"]
    pub const _SC_RE_DUP_MAX: C2RustUnnamed_13 = 44;
    #[c2rust::src_loc = "163:5"]
    pub const _SC_LINE_MAX: C2RustUnnamed_13 = 43;
    #[c2rust::src_loc = "161:5"]
    pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_13 = 42;
    #[c2rust::src_loc = "159:5"]
    pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_13 = 41;
    #[c2rust::src_loc = "157:5"]
    pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_13 = 40;
    #[c2rust::src_loc = "155:5"]
    pub const _SC_BC_STRING_MAX: C2RustUnnamed_13 = 39;
    #[c2rust::src_loc = "153:5"]
    pub const _SC_BC_SCALE_MAX: C2RustUnnamed_13 = 38;
    #[c2rust::src_loc = "151:5"]
    pub const _SC_BC_DIM_MAX: C2RustUnnamed_13 = 37;
    #[c2rust::src_loc = "149:5"]
    pub const _SC_BC_BASE_MAX: C2RustUnnamed_13 = 36;
    #[c2rust::src_loc = "144:5"]
    pub const _SC_TIMER_MAX: C2RustUnnamed_13 = 35;
    #[c2rust::src_loc = "142:5"]
    pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_13 = 34;
    #[c2rust::src_loc = "140:5"]
    pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_13 = 33;
    #[c2rust::src_loc = "138:5"]
    pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_13 = 32;
    #[c2rust::src_loc = "136:5"]
    pub const _SC_RTSIG_MAX: C2RustUnnamed_13 = 31;
    #[c2rust::src_loc = "133:5"]
    pub const _SC_PAGESIZE: C2RustUnnamed_13 = 30;
    #[c2rust::src_loc = "131:5"]
    pub const _SC_VERSION: C2RustUnnamed_13 = 29;
    #[c2rust::src_loc = "129:5"]
    pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_13 = 28;
    #[c2rust::src_loc = "127:5"]
    pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_13 = 27;
    #[c2rust::src_loc = "125:5"]
    pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_13 = 26;
    #[c2rust::src_loc = "123:5"]
    pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_13 = 25;
    #[c2rust::src_loc = "121:5"]
    pub const _SC_AIO_MAX: C2RustUnnamed_13 = 24;
    #[c2rust::src_loc = "119:5"]
    pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_13 = 23;
    #[c2rust::src_loc = "117:5"]
    pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_13 = 22;
    #[c2rust::src_loc = "115:5"]
    pub const _SC_SEMAPHORES: C2RustUnnamed_13 = 21;
    #[c2rust::src_loc = "113:5"]
    pub const _SC_MESSAGE_PASSING: C2RustUnnamed_13 = 20;
    #[c2rust::src_loc = "111:5"]
    pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_13 = 19;
    #[c2rust::src_loc = "109:5"]
    pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_13 = 18;
    #[c2rust::src_loc = "107:5"]
    pub const _SC_MEMLOCK: C2RustUnnamed_13 = 17;
    #[c2rust::src_loc = "105:5"]
    pub const _SC_MAPPED_FILES: C2RustUnnamed_13 = 16;
    #[c2rust::src_loc = "103:5"]
    pub const _SC_FSYNC: C2RustUnnamed_13 = 15;
    #[c2rust::src_loc = "101:5"]
    pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_13 = 14;
    #[c2rust::src_loc = "99:5"]
    pub const _SC_PRIORITIZED_IO: C2RustUnnamed_13 = 13;
    #[c2rust::src_loc = "97:5"]
    pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_13 = 12;
    #[c2rust::src_loc = "95:5"]
    pub const _SC_TIMERS: C2RustUnnamed_13 = 11;
    #[c2rust::src_loc = "93:5"]
    pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_13 = 10;
    #[c2rust::src_loc = "91:5"]
    pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_13 = 9;
    #[c2rust::src_loc = "89:5"]
    pub const _SC_SAVED_IDS: C2RustUnnamed_13 = 8;
    #[c2rust::src_loc = "87:5"]
    pub const _SC_JOB_CONTROL: C2RustUnnamed_13 = 7;
    #[c2rust::src_loc = "85:5"]
    pub const _SC_TZNAME_MAX: C2RustUnnamed_13 = 6;
    #[c2rust::src_loc = "83:5"]
    pub const _SC_STREAM_MAX: C2RustUnnamed_13 = 5;
    #[c2rust::src_loc = "81:5"]
    pub const _SC_OPEN_MAX: C2RustUnnamed_13 = 4;
    #[c2rust::src_loc = "79:5"]
    pub const _SC_NGROUPS_MAX: C2RustUnnamed_13 = 3;
    #[c2rust::src_loc = "77:5"]
    pub const _SC_CLK_TCK: C2RustUnnamed_13 = 2;
    #[c2rust::src_loc = "75:5"]
    pub const _SC_CHILD_MAX: C2RustUnnamed_13 = 1;
    #[c2rust::src_loc = "73:5"]
    pub const _SC_ARG_MAX: C2RustUnnamed_13 = 0;
    #[c2rust::src_loc = "82:9"]
    pub const _SC_OPEN_MAX_0: libc::c_int = _SC_OPEN_MAX as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:20"]
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
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:20"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:20"]
pub mod stdlib_h {
    #[c2rust::src_loc = "808:1"]
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return strtol(
            __nptr,
            NULL as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "365:1"]
    pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
        return strtol(
            __nptr,
            NULL as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
    }
    #[inline]
    #[c2rust::src_loc = "372:15"]
    pub unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
        return strtoll(
            __nptr,
            NULL as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
    }
    use super::stddef_h::NULL;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "117:15"]
        pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "200:22"]
        pub fn strtoll(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_longlong;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/pwd.h:20"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    use super::types_h::{__gid_t, __uid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    }
}
#[c2rust::header_src = "/usr/include/grp.h:20"]
pub mod grp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:8"]
    pub struct group {
        pub gr_name: *mut libc::c_char,
        pub gr_passwd: *mut libc::c_char,
        pub gr_gid: __gid_t,
        pub gr_mem: *mut *mut libc::c_char,
    }
    use super::stddef_h::size_t;
    use super::types_h::__gid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn getgrgid(__gid: __gid_t) -> *mut group;
        #[no_mangle]
        #[c2rust::src_loc = "107:1"]
        pub fn getgrnam(__name: *const libc::c_char) -> *mut group;
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdint.h:20"]
pub mod stdint_h {
    #[c2rust::src_loc = "101:1"]
    pub type intmax_t = __intmax_t;
    #[c2rust::src_loc = "102:1"]
    pub type uintmax_t = __uintmax_t;
    use super::types_h::{__intmax_t, __uintmax_t};
}
#[c2rust::header_src = "/usr/include/netinet/ip.h:20"]
pub mod ip_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "107:8"]
    pub struct ip {
        #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
        #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
        pub ip_hl_ip_v: [u8; 1],
        pub ip_tos: uint8_t,
        pub ip_len: libc::c_ushort,
        pub ip_id: libc::c_ushort,
        pub ip_off: libc::c_ushort,
        pub ip_ttl: uint8_t,
        pub ip_p: uint8_t,
        pub ip_sum: libc::c_ushort,
        pub ip_src: in_addr,
        pub ip_dst: in_addr,
    }
    use super::in_h::in_addr;
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/usr/include/inttypes.h:20"]
pub mod inttypes_h {
    #[c2rust::src_loc = "34:1"]
    pub type __gwchar_t = libc::c_int;
    #[inline]
    #[c2rust::src_loc = "323:1"]
    pub unsafe extern "C" fn strtoimax(
        mut nptr: *const libc::c_char,
        mut endptr: *mut *mut libc::c_char,
        mut base: libc::c_int,
    ) -> intmax_t {
        return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "335:1"]
    pub unsafe extern "C" fn strtoumax(
        mut nptr: *const libc::c_char,
        mut endptr: *mut *mut libc::c_char,
        mut base: libc::c_int,
    ) -> uintmax_t {
        return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "347:1"]
    pub unsafe extern "C" fn wcstoimax(
        mut nptr: *const __gwchar_t,
        mut endptr: *mut *mut __gwchar_t,
        mut base: libc::c_int,
    ) -> intmax_t {
        return __wcstol_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe extern "C" fn wcstoumax(
        mut nptr: *const __gwchar_t,
        mut endptr: *mut *mut __gwchar_t,
        mut base: libc::c_int,
    ) -> uintmax_t {
        return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    use super::stdint_h::{intmax_t, uintmax_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "318:1"]
        pub fn __strtol_internal(
            __nptr: *const libc::c_char,
            __endptr: *mut *mut libc::c_char,
            __base: libc::c_int,
            __group: libc::c_int,
        ) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "330:1"]
        pub fn __strtoul_internal(
            __nptr: *const libc::c_char,
            __endptr: *mut *mut libc::c_char,
            __base: libc::c_int,
            __group: libc::c_int,
        ) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "342:1"]
        pub fn __wcstol_internal(
            __nptr: *const __gwchar_t,
            __endptr: *mut *mut __gwchar_t,
            __base: libc::c_int,
            __group: libc::c_int,
        ) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "354:1"]
        pub fn __wcstoul_internal(
            __nptr: *const __gwchar_t,
            __endptr: *mut *mut __gwchar_t,
            __base: libc::c_int,
            __group: libc::c_int,
        ) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/netinet/ip_icmp.h:20"]
pub mod ip_icmp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "131:8"]
    pub struct icmp_ra_addr {
        pub ira_addr: uint32_t,
        pub ira_preference: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:8"]
    pub struct icmp {
        pub icmp_type: uint8_t,
        pub icmp_code: uint8_t,
        pub icmp_cksum: uint16_t,
        pub icmp_hun: C2RustUnnamed_17,
        pub icmp_dun: C2RustUnnamed_14,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:3"]
    pub union C2RustUnnamed_14 {
        pub id_ts: C2RustUnnamed_16,
        pub id_ip: C2RustUnnamed_15,
        pub id_radv: icmp_ra_addr,
        pub id_mask: uint32_t,
        pub id_data: [uint8_t; 1],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "185:5"]
    pub struct C2RustUnnamed_15 {
        pub idi_ip: ip,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "179:5"]
    pub struct C2RustUnnamed_16 {
        pub its_otime: uint32_t,
        pub its_rtime: uint32_t,
        pub its_ttime: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "142:3"]
    pub union C2RustUnnamed_17 {
        pub ih_pptr: libc::c_uchar,
        pub ih_gwaddr: in_addr,
        pub ih_idseq: ih_idseq,
        pub ih_void: uint32_t,
        pub ih_pmtu: ih_pmtu,
        pub ih_rtradv: ih_rtradv,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "160:12"]
    pub struct ih_rtradv {
        pub irt_num_addrs: uint8_t,
        pub irt_wpa: uint8_t,
        pub irt_lifetime: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:12"]
    pub struct ih_pmtu {
        pub ipm_void: uint16_t,
        pub ipm_nextmtu: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:12"]
    pub struct ih_idseq {
        pub icd_id: uint16_t,
        pub icd_seq: uint16_t,
    }
    #[c2rust::src_loc = "51:9"]
    pub const ICMP_ECHO: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "47:9"]
    pub const ICMP_ECHOREPLY: libc::c_int = 0 as libc::c_int;
    use super::in_h::in_addr;
    use super::ip_h::ip;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
}
#[c2rust::header_src = "/usr/include/dirent.h:20"]
pub mod dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/linux/capability.h:20"]
pub mod capability_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct __user_cap_header_struct {
        pub version: __u32,
        pub pid: libc::c_int,
    }
    #[c2rust::src_loc = "39:1"]
    pub type cap_user_header_t = *mut __user_cap_header_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:16"]
    pub struct __user_cap_data_struct {
        pub effective: __u32,
        pub permitted: __u32,
        pub inheritable: __u32,
    }
    #[c2rust::src_loc = "44:1"]
    pub type cap_user_data_t = *mut __user_cap_data_struct;
    #[c2rust::src_loc = "201:9"]
    pub const CAP_NET_ADMIN: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "207:9"]
    pub const CAP_NET_RAW: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "181:9"]
    pub const CAP_NET_BIND_SERVICE: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "155:9"]
    pub const CAP_SETUID: libc::c_int = 7 as libc::c_int;
    use super::int_ll64_h::__u32;
}
#[c2rust::header_src = "/usr/include/stdio.h:20"]
pub mod stdio_h {
    use super::internal::__va_list_tag;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;
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
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
            -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "486:1"]
        pub fn getc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "858:1"]
        pub fn __uflow(_: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "859:1"]
        pub fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "603:1"]
        pub fn __getdelim(
            __lineptr: *mut *mut libc::c_char,
            __n: *mut size_t,
            __delimiter: libc::c_int,
            __stream: *mut FILE,
        ) -> __ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:20"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
            | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
            as __uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
            | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
            | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
            | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "69:15"]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong)
            >> 56 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
                >> 40 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
                >> 24 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong) >> 8 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong) << 24 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56 as libc::c_int)
            as __uint64_t;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:20"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
        return __x;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/config.h:20"]
pub mod config_h {
    #[c2rust::src_loc = "364:14"]
    pub static mut compile_opts: *mut libc::c_char =
        b"IPv6 GNU-getopt no-DBus no-UBus no-i18n no-IDN DHCP DHCPv6 no-Lua TFTP no-conntrack ipset auth no-cryptohash no-DNSSEC loop-detect inotify dumpfile\x00"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    #[c2rust::src_loc = "207:15"]
    pub const LEASEFILE: [libc::c_char; 29] = unsafe {
        *::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"/var/lib/misc/dnsmasq.leases\x00",
        )
    };
    #[c2rust::src_loc = "49:9"]
    pub const CHGRP: [libc::c_char; 4] =
        unsafe { *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dip\x00") };
    #[c2rust::src_loc = "32:9"]
    pub const RANDOM_SOCKS: libc::c_int = 64 as libc::c_int;
    #[c2rust::src_loc = "18:9"]
    pub const MAX_PROCS: libc::c_int = 20 as libc::c_int;
    #[c2rust::src_loc = "20:9"]
    pub const TCP_MAX_QUERIES: libc::c_int = 100 as libc::c_int;
    #[c2rust::src_loc = "19:9"]
    pub const CHILD_LIFETIME: libc::c_int = 150 as libc::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const PING_WAIT: libc::c_int = 3 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/stat.h:20"]
pub mod sys_stat_h {
    #[inline]
    #[c2rust::src_loc = "452:1"]
    pub unsafe extern "C" fn stat(
        mut __path: *const libc::c_char,
        mut __statbuf: *mut stat,
    ) -> libc::c_int {
        return __xstat(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "466:1"]
    pub unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
        return __fxstat(_STAT_VER, __fd, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "501:1"]
    pub unsafe extern "C" fn stat64(
        mut __path: *const libc::c_char,
        mut __statbuf: *mut stat64,
    ) -> libc::c_int {
        return __xstat64(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "515:1"]
    pub unsafe extern "C" fn fstat64(
        mut __fd: libc::c_int,
        mut __statbuf: *mut stat64,
    ) -> libc::c_int {
        return __fxstat64(_STAT_VER, __fd, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "473:1"]
    pub unsafe extern "C" fn fstatat(
        mut __fd: libc::c_int,
        mut __filename: *const libc::c_char,
        mut __statbuf: *mut stat,
        mut __flag: libc::c_int,
    ) -> libc::c_int {
        return __fxstatat(_STAT_VER, __fd, __filename, __statbuf, __flag);
    }
    #[inline]
    #[c2rust::src_loc = "522:1"]
    pub unsafe extern "C" fn fstatat64(
        mut __fd: libc::c_int,
        mut __filename: *const libc::c_char,
        mut __statbuf: *mut stat64,
        mut __flag: libc::c_int,
    ) -> libc::c_int {
        return __fxstatat64(_STAT_VER, __fd, __filename, __statbuf, __flag);
    }
    #[inline]
    #[c2rust::src_loc = "459:1"]
    pub unsafe extern "C" fn lstat(
        mut __path: *const libc::c_char,
        mut __statbuf: *mut stat,
    ) -> libc::c_int {
        return __lxstat(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "508:1"]
    pub unsafe extern "C" fn lstat64(
        mut __path: *const libc::c_char,
        mut __statbuf: *mut stat64,
    ) -> libc::c_int {
        return __lxstat64(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "482:1"]
    pub unsafe extern "C" fn mknod(
        mut __path: *const libc::c_char,
        mut __mode: __mode_t,
        mut __dev: __dev_t,
    ) -> libc::c_int {
        return __xmknod(_MKNOD_VER, __path, __mode, &mut __dev);
    }
    #[c2rust::src_loc = "390:10"]
    pub const _MKNOD_VER: libc::c_int = 0 as libc::c_int;
    #[inline]
    #[c2rust::src_loc = "490:1"]
    pub unsafe extern "C" fn mknodat(
        mut __fd: libc::c_int,
        mut __path: *const libc::c_char,
        mut __mode: __mode_t,
        mut __dev: __dev_t,
    ) -> libc::c_int {
        return __xmknodat(_MKNOD_VER, __fd, __path, __mode, &mut __dev);
    }
    use super::stat_h::{stat, stat64, _STAT_VER, _STAT_VER_LINUX};
    use super::types_h::{__dev_t, __mode_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "409:1"]
        pub fn __xstat(
            __ver: libc::c_int,
            __filename: *const libc::c_char,
            __stat_buf: *mut stat,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "406:1"]
        pub fn __fxstat(
            __ver: libc::c_int,
            __fildes: libc::c_int,
            __stat_buf: *mut stat,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "430:1"]
        pub fn __xstat64(
            __ver: libc::c_int,
            __filename: *const libc::c_char,
            __stat_buf: *mut stat64,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "428:1"]
        pub fn __fxstat64(
            __ver: libc::c_int,
            __fildes: libc::c_int,
            __stat_buf: *mut stat64,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn __fxstatat(
            __ver: libc::c_int,
            __fildes: libc::c_int,
            __filename: *const libc::c_char,
            __stat_buf: *mut stat,
            __flag: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "434:1"]
        pub fn __fxstatat64(
            __ver: libc::c_int,
            __fildes: libc::c_int,
            __filename: *const libc::c_char,
            __stat_buf: *mut stat64,
            __flag: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "412:1"]
        pub fn __lxstat(
            __ver: libc::c_int,
            __filename: *const libc::c_char,
            __stat_buf: *mut stat,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn __lxstat64(
            __ver: libc::c_int,
            __filename: *const libc::c_char,
            __stat_buf: *mut stat64,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn umask(__mask: __mode_t) -> __mode_t;
        #[no_mangle]
        #[c2rust::src_loc = "438:1"]
        pub fn __xmknod(
            __ver: libc::c_int,
            __path: *const libc::c_char,
            __mode: __mode_t,
            __dev: *mut __dev_t,
        ) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "441:1"]
        pub fn __xmknodat(
            __ver: libc::c_int,
            __fd: libc::c_int,
            __path: *const libc::c_char,
            __mode: __mode_t,
            __dev: *mut __dev_t,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/wait.h:20"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn waitpid(
            __pid: __pid_t,
            __stat_loc: *mut libc::c_int,
            __options: libc::c_int,
        ) -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:20"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "397:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:20"]
pub mod unistd_h {
    #[c2rust::src_loc = "211:9"]
    pub const STDOUT_FILENO: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "212:9"]
    pub const STDERR_FILENO: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "210:9"]
    pub const STDIN_FILENO: libc::c_int = 0 as libc::c_int;
    use super::types_h::{__gid_t, __pid_t, __uid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "417:1"]
        pub fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "478:1"]
        pub fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "497:1"]
        pub fn chdir(__path: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "603:13"]
        pub fn _exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "619:1"]
        pub fn sysconf(__name: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "667:1"]
        pub fn setsid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "700:1"]
        pub fn setuid(__uid: __uid_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "717:1"]
        pub fn setgid(__gid: __gid_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdio.h:20"]
pub mod bits_stdio_h {
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn vprintf(
        mut __fmt: *const libc::c_char,
        mut __arg: ::std::ffi::VaList,
    ) -> libc::c_int {
        return vfprintf(stdout, __fmt, __arg.as_va_list());
    }
    #[inline]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn getchar() -> libc::c_int {
        return getc(stdin);
    }
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as libc::c_long != 0
        {
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
        return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as libc::c_int as libc::c_long
            != 0
        {
            __uflow(stdin)
        } else {
            let fresh1 = (*stdin)._IO_read_ptr;
            (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
            *(fresh1 as *mut libc::c_uchar) as libc::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as libc::c_long != 0
        {
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
    pub unsafe extern "C" fn fputc_unlocked(
        mut __c: libc::c_int,
        mut __stream: *mut FILE,
    ) -> libc::c_int {
        return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
            as libc::c_long
            != 0
        {
            __overflow(__stream, __c as libc::c_uchar as libc::c_int)
        } else {
            let fresh3 = (*__stream)._IO_write_ptr;
            (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
            *fresh3 = __c as libc::c_char;
            *fresh3 as libc::c_uchar as libc::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn putc_unlocked(
        mut __c: libc::c_int,
        mut __stream: *mut FILE,
    ) -> libc::c_int {
        return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
            as libc::c_long
            != 0
        {
            __overflow(__stream, __c as libc::c_uchar as libc::c_int)
        } else {
            let fresh4 = (*__stream)._IO_write_ptr;
            (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
            *fresh4 = __c as libc::c_char;
            *fresh4 as libc::c_uchar as libc::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
        return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
            as libc::c_long
            != 0
        {
            __overflow(stdout, __c as libc::c_uchar as libc::c_int)
        } else {
            let fresh5 = (*stdout)._IO_write_ptr;
            (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
            *fresh5 = __c as libc::c_char;
            *fresh5 as libc::c_uchar as libc::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "117:1"]
    pub unsafe extern "C" fn getline(
        mut __lineptr: *mut *mut libc::c_char,
        mut __n: *mut size_t,
        mut __stream: *mut FILE,
    ) -> __ssize_t {
        return __getdelim(__lineptr, __n, '\n' as i32, __stream);
    }
    #[inline]
    #[c2rust::src_loc = "127:1"]
    pub unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
        return ((*__stream)._flags & _IO_EOF_SEEN != 0 as libc::c_int) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "134:1"]
    pub unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
        return ((*__stream)._flags & _IO_ERR_SEEN != 0 as libc::c_int) as libc::c_int;
    }
    use super::internal::__va_list_tag;
    use super::stddef_h::size_t;
    use super::stdio_h::{__getdelim, __overflow, __uflow, getc, putc, stdin, stdout, vfprintf};
    use super::struct_FILE_h::{_IO_EOF_SEEN, _IO_ERR_SEEN};
    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdlib-float.h:20"]
pub mod stdlib_float_h {
    #[inline]
    #[c2rust::src_loc = "24:1"]
    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return strtod(__nptr, NULL as *mut libc::c_void as *mut *mut libc::c_char);
    }
    use super::stddef_h::NULL;
    use super::stdlib_h::strtod;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/signum-generic.h:20"]
pub mod signum_generic_h {
    #[c2rust::src_loc = "30:9"]
    pub const SIG_IGN: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const SIGHUP: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "63:9"]
    pub const SIGALRM: libc::c_int = 14 as libc::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const SIGTERM: libc::c_int = 15 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const SIGINT: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdlib-bsearch.h:20"]
pub mod stdlib_bsearch_h {
    #[inline]
    #[c2rust::src_loc = "19:1"]
    pub unsafe extern "C" fn bsearch(
        mut __key: *const libc::c_void,
        mut __base: *const libc::c_void,
        mut __nmemb: size_t,
        mut __size: size_t,
        mut __compar: __compar_fn_t,
    ) -> *mut libc::c_void {
        let mut __l: size_t = 0;
        let mut __u: size_t = 0;
        let mut __idx: size_t = 0;
        let mut __p = 0 as *const libc::c_void;
        let mut __comparison: libc::c_int = 0;
        __l = 0 as libc::c_int as size_t;
        __u = __nmemb;
        while __l < __u {
            __idx = __l
                .wrapping_add(__u)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
            __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
                as *mut libc::c_void;
            __comparison = Some(__compar.expect("non-null function pointer"))
                .expect("non-null function pointer")(__key, __p);
            if __comparison < 0 as libc::c_int {
                __u = __idx
            } else if __comparison > 0 as libc::c_int {
                __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                return __p as *mut libc::c_void;
            }
        }
        return NULL as *mut libc::c_void;
    }
    use super::stddef_h::{size_t, NULL};
    use super::stdlib_h::__compar_fn_t;
}
#[c2rust::header_src = "/usr/include/fcntl.h:20"]
pub mod fcntl_h {
    #[c2rust::src_loc = "106:10"]
    pub const S_IWUSR: libc::c_int = __S_IWRITE;
    #[c2rust::src_loc = "117:10"]
    pub const S_IROTH: libc::c_int = S_IRGRP >> 3 as libc::c_int;
    #[c2rust::src_loc = "111:10"]
    pub const S_IRGRP: libc::c_int = S_IRUSR >> 3 as libc::c_int;
    #[c2rust::src_loc = "105:10"]
    pub const S_IRUSR: libc::c_int = __S_IREAD;
    use super::stat_h::{__S_IREAD, __S_IWRITE};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "171:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:20"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
    #[inline]
    #[c2rust::src_loc = "212:1"]
    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
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
#[c2rust::header_src = "/usr/include/time.h:20"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:20"]
pub mod errno_h {
    #[c2rust::src_loc = "38:10"]
    pub const errno: libc::c_int = *__errno_location();
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/uio.h:20"]
pub mod uio_h {
    use super::struct_iovec_h::iovec;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/prctl.h:20"]
pub mod prctl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "27:1"]
        pub fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:20"]
pub mod dns_protocol_h {
    #[c2rust::src_loc = "25:9"]
    pub const PACKETSZ: libc::c_int = 512 as libc::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
    #[c2rust::src_loc = "27:9"]
    pub const RRFIXEDSZ: libc::c_int = 10 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/fcntl-linux.h:20"]
pub mod fcntl_linux_h {
    #[c2rust::src_loc = "44:9"]
    pub const O_WRONLY: libc::c_int = 0o1 as libc::c_int;
    #[c2rust::src_loc = "47:10"]
    pub const O_CREAT: libc::c_int = 0o100 as libc::c_int;
    #[c2rust::src_loc = "56:10"]
    pub const O_TRUNC: libc::c_int = 0o1000 as libc::c_int;
    #[c2rust::src_loc = "50:10"]
    pub const O_EXCL: libc::c_int = 0o200 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const O_RDWR: libc::c_int = 0o2 as libc::c_int;
    #[c2rust::src_loc = "170:9"]
    pub const F_GETFL: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "171:9"]
    pub const F_SETFL: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "62:10"]
    pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/linux/prctl.h:20"]
pub mod linux_prctl_h {
    #[c2rust::src_loc = "25:9"]
    pub const PR_SET_KEEPCAPS: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "14:9"]
    pub const PR_SET_DUMPABLE: libc::c_int = 4 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:20"]
pub mod syslog_h {
    #[c2rust::src_loc = "96:9"]
    pub const LOG_DAEMON: libc::c_int = (3 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const LOG_USER: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const LOG_ERR: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "95:9"]
    pub const LOG_MAIL: libc::c_int = (2 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "58:9"]
    pub const LOG_DEBUG: libc::c_int = 7 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const LOG_INFO: libc::c_int = 6 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/poll.h:20"]
pub mod poll_h {
    #[c2rust::src_loc = "27:9"]
    pub const POLLOUT: libc::c_int = 0x4 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const POLLHUP: libc::c_int = 0x10 as libc::c_int;
    #[c2rust::src_loc = "25:9"]
    pub const POLLIN: libc::c_int = 0x1 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/waitflags.h:20"]
pub mod waitflags_h {
    #[c2rust::src_loc = "25:9"]
    pub const WNOHANG: libc::c_int = 1 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/signum.h:20"]
pub mod signum_h {
    #[c2rust::src_loc = "41:9"]
    pub const SIGCHLD: libc::c_int = 17 as libc::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const SIGUSR1: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const SIGUSR2: libc::c_int = 12 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno-base.h:20"]
pub mod errno_base_h {
    #[c2rust::src_loc = "8:9"]
    pub const EINTR: libc::c_int = 4 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/socket.h:20"]
pub mod asm_generic_socket_h {
    #[c2rust::src_loc = "9:9"]
    pub const SOL_SOCKET: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "15:9"]
    pub const SO_DONTROUTE: libc::c_int = 5 as libc::c_int;
}
pub use self::__sigset_t_h::__sigset_t;
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::asm_generic_socket_h::{SOL_SOCKET, SO_DONTROUTE};
pub use self::bits_stdio_h::{
    feof_unlocked, ferror_unlocked, fgetc_unlocked, fputc_unlocked, getc_unlocked, getchar,
    getchar_unlocked, getline, putc_unlocked, putchar, putchar_unlocked, vprintf,
};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::capability_h::{
    __user_cap_data_struct, __user_cap_header_struct, cap_user_data_t, cap_user_header_t,
    CAP_NET_ADMIN, CAP_NET_BIND_SERVICE, CAP_NET_RAW, CAP_SETUID,
};
pub use self::config_h::{
    compile_opts, CHGRP, CHILD_LIFETIME, LEASEFILE, MAX_PROCS, PING_WAIT, RANDOM_SOCKS,
    TCP_MAX_QUERIES,
};
pub use self::confname_h::{
    C2RustUnnamed_13, _SC_2_CHAR_TERM, _SC_2_C_BIND, _SC_2_C_DEV, _SC_2_C_VERSION, _SC_2_FORT_DEV,
    _SC_2_FORT_RUN, _SC_2_LOCALEDEF, _SC_2_PBS, _SC_2_PBS_ACCOUNTING, _SC_2_PBS_CHECKPOINT,
    _SC_2_PBS_LOCATE, _SC_2_PBS_MESSAGE, _SC_2_PBS_TRACK, _SC_2_SW_DEV, _SC_2_UPE, _SC_2_VERSION,
    _SC_ADVISORY_INFO, _SC_AIO_LISTIO_MAX, _SC_AIO_MAX, _SC_AIO_PRIO_DELTA_MAX, _SC_ARG_MAX,
    _SC_ASYNCHRONOUS_IO, _SC_ATEXIT_MAX, _SC_AVPHYS_PAGES, _SC_BARRIERS, _SC_BASE, _SC_BC_BASE_MAX,
    _SC_BC_DIM_MAX, _SC_BC_SCALE_MAX, _SC_BC_STRING_MAX, _SC_CHARCLASS_NAME_MAX, _SC_CHAR_BIT,
    _SC_CHAR_MAX, _SC_CHAR_MIN, _SC_CHILD_MAX, _SC_CLK_TCK, _SC_CLOCK_SELECTION,
    _SC_COLL_WEIGHTS_MAX, _SC_CPUTIME, _SC_C_LANG_SUPPORT, _SC_C_LANG_SUPPORT_R,
    _SC_DELAYTIMER_MAX, _SC_DEVICE_IO, _SC_DEVICE_SPECIFIC, _SC_DEVICE_SPECIFIC_R,
    _SC_EQUIV_CLASS_MAX, _SC_EXPR_NEST_MAX, _SC_FD_MGMT, _SC_FIFO, _SC_FILE_ATTRIBUTES,
    _SC_FILE_LOCKING, _SC_FILE_SYSTEM, _SC_FSYNC, _SC_GETGR_R_SIZE_MAX, _SC_GETPW_R_SIZE_MAX,
    _SC_HOST_NAME_MAX, _SC_INT_MAX, _SC_INT_MIN, _SC_IOV_MAX, _SC_IPV6, _SC_JOB_CONTROL,
    _SC_LEVEL1_DCACHE_ASSOC, _SC_LEVEL1_DCACHE_LINESIZE, _SC_LEVEL1_DCACHE_SIZE,
    _SC_LEVEL1_ICACHE_ASSOC, _SC_LEVEL1_ICACHE_LINESIZE, _SC_LEVEL1_ICACHE_SIZE,
    _SC_LEVEL2_CACHE_ASSOC, _SC_LEVEL2_CACHE_LINESIZE, _SC_LEVEL2_CACHE_SIZE,
    _SC_LEVEL3_CACHE_ASSOC, _SC_LEVEL3_CACHE_LINESIZE, _SC_LEVEL3_CACHE_SIZE,
    _SC_LEVEL4_CACHE_ASSOC, _SC_LEVEL4_CACHE_LINESIZE, _SC_LEVEL4_CACHE_SIZE, _SC_LINE_MAX,
    _SC_LOGIN_NAME_MAX, _SC_LONG_BIT, _SC_MAPPED_FILES, _SC_MB_LEN_MAX, _SC_MEMLOCK,
    _SC_MEMLOCK_RANGE, _SC_MEMORY_PROTECTION, _SC_MESSAGE_PASSING, _SC_MONOTONIC_CLOCK,
    _SC_MQ_OPEN_MAX, _SC_MQ_PRIO_MAX, _SC_MULTI_PROCESS, _SC_NETWORKING, _SC_NGROUPS_MAX,
    _SC_NL_ARGMAX, _SC_NL_LANGMAX, _SC_NL_MSGMAX, _SC_NL_NMAX, _SC_NL_SETMAX, _SC_NL_TEXTMAX,
    _SC_NPROCESSORS_CONF, _SC_NPROCESSORS_ONLN, _SC_NZERO, _SC_OPEN_MAX, _SC_OPEN_MAX_0,
    _SC_PAGESIZE, _SC_PASS_MAX, _SC_PHYS_PAGES, _SC_PII, _SC_PII_INTERNET, _SC_PII_INTERNET_DGRAM,
    _SC_PII_INTERNET_STREAM, _SC_PII_OSI, _SC_PII_OSI_CLTS, _SC_PII_OSI_COTS, _SC_PII_OSI_M,
    _SC_PII_SOCKET, _SC_PII_XTI, _SC_PIPE, _SC_POLL, _SC_PRIORITIZED_IO, _SC_PRIORITY_SCHEDULING,
    _SC_RAW_SOCKETS, _SC_READER_WRITER_LOCKS, _SC_REALTIME_SIGNALS, _SC_REGEXP, _SC_REGEX_VERSION,
    _SC_RE_DUP_MAX, _SC_RTSIG_MAX, _SC_SAVED_IDS, _SC_SCHAR_MAX, _SC_SCHAR_MIN, _SC_SELECT,
    _SC_SEMAPHORES, _SC_SEM_NSEMS_MAX, _SC_SEM_VALUE_MAX, _SC_SHARED_MEMORY_OBJECTS, _SC_SHELL,
    _SC_SHRT_MAX, _SC_SHRT_MIN, _SC_SIGNALS, _SC_SIGQUEUE_MAX, _SC_SINGLE_PROCESS, _SC_SPAWN,
    _SC_SPIN_LOCKS, _SC_SPORADIC_SERVER, _SC_SSIZE_MAX, _SC_SS_REPL_MAX, _SC_STREAMS,
    _SC_STREAM_MAX, _SC_SYMLOOP_MAX, _SC_SYNCHRONIZED_IO, _SC_SYSTEM_DATABASE,
    _SC_SYSTEM_DATABASE_R, _SC_THREADS, _SC_THREAD_ATTR_STACKADDR, _SC_THREAD_ATTR_STACKSIZE,
    _SC_THREAD_CPUTIME, _SC_THREAD_DESTRUCTOR_ITERATIONS, _SC_THREAD_KEYS_MAX,
    _SC_THREAD_PRIORITY_SCHEDULING, _SC_THREAD_PRIO_INHERIT, _SC_THREAD_PRIO_PROTECT,
    _SC_THREAD_PROCESS_SHARED, _SC_THREAD_ROBUST_PRIO_INHERIT, _SC_THREAD_ROBUST_PRIO_PROTECT,
    _SC_THREAD_SAFE_FUNCTIONS, _SC_THREAD_SPORADIC_SERVER, _SC_THREAD_STACK_MIN,
    _SC_THREAD_THREADS_MAX, _SC_TIMEOUTS, _SC_TIMERS, _SC_TIMER_MAX, _SC_TRACE,
    _SC_TRACE_EVENT_FILTER, _SC_TRACE_EVENT_NAME_MAX, _SC_TRACE_INHERIT, _SC_TRACE_LOG,
    _SC_TRACE_NAME_MAX, _SC_TRACE_SYS_MAX, _SC_TRACE_USER_EVENT_MAX, _SC_TTY_NAME_MAX,
    _SC_TYPED_MEMORY_OBJECTS, _SC_TZNAME_MAX, _SC_T_IOV_MAX, _SC_UCHAR_MAX, _SC_UINT_MAX,
    _SC_UIO_MAXIOV, _SC_ULONG_MAX, _SC_USER_GROUPS, _SC_USER_GROUPS_R, _SC_USHRT_MAX,
    _SC_V6_ILP32_OFF32, _SC_V6_ILP32_OFFBIG, _SC_V6_LP64_OFF64, _SC_V6_LPBIG_OFFBIG,
    _SC_V7_ILP32_OFF32, _SC_V7_ILP32_OFFBIG, _SC_V7_LP64_OFF64, _SC_V7_LPBIG_OFFBIG, _SC_VERSION,
    _SC_WORD_BIT, _SC_XBS5_ILP32_OFF32, _SC_XBS5_ILP32_OFFBIG, _SC_XBS5_LP64_OFF64,
    _SC_XBS5_LPBIG_OFFBIG, _SC_XOPEN_CRYPT, _SC_XOPEN_ENH_I18N, _SC_XOPEN_LEGACY,
    _SC_XOPEN_REALTIME, _SC_XOPEN_REALTIME_THREADS, _SC_XOPEN_SHM, _SC_XOPEN_STREAMS,
    _SC_XOPEN_UNIX, _SC_XOPEN_VERSION, _SC_XOPEN_XCU_VERSION, _SC_XOPEN_XPG2, _SC_XOPEN_XPG3,
    _SC_XOPEN_XPG4,
};
pub use self::ctype_h::{__ctype_tolower_loc, __ctype_toupper_loc, tolower, toupper};
pub use self::dirent_h::{__dirstream, closedir, opendir, DIR};
pub use self::dns_protocol_h::{MAXDNAME, PACKETSZ, RRFIXEDSZ};
pub use self::dnsmasq_h::{
    addr_list, addrlist, all_addr, auth_name_list, auth_zone, bigname, bindtodevice, blockdata,
    blockdata_init, bogus_addr, cache_init, cache_recv_insert, cache_reload, capget, capset,
    check_log_writer, check_servers, check_tftp_listeners, close_fds, cname, cond_domain,
    create_bound_listeners, create_helper, create_wildcard_listeners, crec, delay_config,
    dhcp6_init, dhcp6_packet, dhcp_boot, dhcp_bridge, dhcp_common_init, dhcp_config,
    dhcp_construct_contexts, dhcp_context, dhcp_init, dhcp_lease, dhcp_mac, dhcp_match_name,
    dhcp_netid, dhcp_netid_list, dhcp_opt, dhcp_packet, dhcp_pxe_vendor, dhcp_read_ethers,
    dhcp_relay, dhcp_update_configs, dhcp_vendor, die, dnsmasq_daemon, dnsmasq_time,
    do_arp_script_run, do_poll, do_script_run, do_tftp_script_run, doctor, dump_cache, dump_init,
    enumerate_interfaces, event_desc, find_mac, fix_fd, flush_log, frec, frec_src, get_new_frec,
    hash_questions_init, helper_buf_empty, helper_write, host_record, hostsfile, hwaddr_config,
    icmp6_packet, iname, indextoname, inotify_check, inotify_dnsmasq_init, interface_name,
    ipset_init, ipsets, irec, is_dad_listeners, join_multicast, kernel_version,
    lease_find_interfaces, lease_init, lease_make_duid, lease_prune, lease_update_dns,
    lease_update_file, lease_update_from_configs, listener, log_context, log_relay, log_reopen,
    log_start, loopback_exception, mx_srv_record, my_syslog, mysockaddr, mysubnet, name_list,
    naptr, netlink_init, netlink_multicast, newaddress, periodic_ra, ping_result, poll_check,
    poll_listen, poll_reset, pre_allocate_sfds, ptr_record, pxe_service, ra_init, ra_interface,
    rand16, rand_init, randfd, read_opts, read_servers_file, read_write, receive_query,
    reload_servers, reply_query, reread_dhcp, rerun_scripts, resend_query, resolvc, retry_send,
    safe_malloc, safe_pipe, server, serverfd, set_log_writer, shared_network, slaac_address,
    sockaddr_isequal, tag_if, tcp_interface, tcp_request, tftp_file, tftp_prefix, tftp_request,
    tftp_transfer, txt_record, u16_0, u32_0, warn_bound_listeners, warn_int_names,
    warn_wild_labels, whichdevice, C2RustUnnamed_18, C2RustUnnamed_19, C2RustUnnamed_20,
    C2RustUnnamed_21, C2RustUnnamed_22, C2RustUnnamed_23, C2RustUnnamed_24, C2RustUnnamed_25,
    ADDRSTRLEN, CONTEXT_DHCP, CONTEXT_RA, EC_BADCONF, EC_BADNET, EC_FILE, EC_GOOD, EC_MISC,
    EVENT_ALARM, EVENT_CAP_ERR, EVENT_CHILD, EVENT_DIE, EVENT_DUMP, EVENT_EXEC_ERR, EVENT_EXITED,
    EVENT_FORK_ERR, EVENT_GROUP_ERR, EVENT_INIT, EVENT_KILLED, EVENT_LOG_ERR, EVENT_LUA_ERR,
    EVENT_NEWADDR, EVENT_NEWROUTE, EVENT_PIDFILE, EVENT_PIPE_ERR, EVENT_RELOAD, EVENT_REOPEN,
    EVENT_SCRIPT_LOG, EVENT_TERM, EVENT_TFTP_ERR, EVENT_TIME, EVENT_TIME_ERR, EVENT_USER_ERR,
    LINUX_CAPABILITY_VERSION_1, LINUX_CAPABILITY_VERSION_2, LINUX_CAPABILITY_VERSION_3, MS_DHCP,
    MS_SCRIPT, MS_TFTP,
};
pub use self::errno_base_h::EINTR;
pub use self::errno_h::{__errno_location, errno};
pub use self::fcntl_h::{fcntl, open, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR};
pub use self::fcntl_linux_h::{
    F_GETFL, F_SETFL, O_CREAT, O_EXCL, O_NONBLOCK, O_RDWR, O_TRUNC, O_WRONLY,
};
pub use self::grp_h::{getgrgid, getgrnam, group, setgroups};
pub use self::in_h::{
    in6_addr, in_addr, in_addr_t, in_port_t, sockaddr_in, sockaddr_in6, C2RustUnnamed_0,
    C2RustUnnamed_1, INET6_ADDRSTRLEN, IPPROTO_AH, IPPROTO_BEETPH, IPPROTO_COMP, IPPROTO_DCCP,
    IPPROTO_EGP, IPPROTO_ENCAP, IPPROTO_ESP, IPPROTO_GRE, IPPROTO_ICMP, IPPROTO_ICMP_0,
    IPPROTO_IDP, IPPROTO_IGMP, IPPROTO_IP, IPPROTO_IPIP, IPPROTO_IPV6, IPPROTO_MAX, IPPROTO_MPLS,
    IPPROTO_MTP, IPPROTO_PIM, IPPROTO_PUP, IPPROTO_RAW, IPPROTO_RSVP, IPPROTO_SCTP, IPPROTO_TCP,
    IPPROTO_TP, IPPROTO_UDP, IPPROTO_UDPLITE,
};
pub use self::int_ll64_h::__u32;
pub use self::internal::{__va_list_tag, VERSION};
pub use self::inttypes_h::{
    __gwchar_t, __strtol_internal, __strtoul_internal, __wcstol_internal, __wcstoul_internal,
    strtoimax, strtoumax, wcstoimax, wcstoumax,
};
pub use self::ip_h::ip;
pub use self::ip_icmp_h::{
    icmp, icmp_ra_addr, ih_idseq, ih_pmtu, ih_rtradv, C2RustUnnamed_14, C2RustUnnamed_15,
    C2RustUnnamed_16, C2RustUnnamed_17, ICMP_ECHO, ICMP_ECHOREPLY,
};
pub use self::linux_prctl_h::{PR_SET_DUMPABLE, PR_SET_KEEPCAPS};
pub use self::poll_h::{POLLHUP, POLLIN, POLLOUT};
use self::prctl_h::prctl;
pub use self::pwd_h::{getpwnam, passwd};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_12};
pub use self::siginfo_t_h::{
    siginfo_t, C2RustUnnamed_10, C2RustUnnamed_11, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
    C2RustUnnamed_9,
};
pub use self::signal_h::{__sighandler_t, kill, sigaction, sigemptyset};
pub use self::signum_generic_h::{SIGALRM, SIGHUP, SIGINT, SIGPIPE, SIGTERM, SIG_IGN};
pub use self::signum_h::{SIGCHLD, SIGUSR1, SIGUSR2};
pub use self::sigset_t_h::sigset_t;
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{
    __cmsg_nxthdr, cmsghdr, msghdr, sockaddr, socklen_t, AF_INET, AF_INET6, PF_INET, PF_INET6,
};
pub use self::socket_type_h::{
    __socket_type, SOCK_CLOEXEC, SOCK_DCCP, SOCK_DGRAM, SOCK_NONBLOCK, SOCK_PACKET, SOCK_RAW,
    SOCK_RAW_0, SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
};
pub use self::stat_h::{stat, stat64, _STAT_VER, _STAT_VER_LINUX, __S_IREAD, __S_IWRITE};
pub use self::stddef_h::{size_t, NULL};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::stdio_h::{
    __getdelim, __overflow, __uflow, fclose, getc, putc, sprintf, stdin, stdout, vfprintf,
};
pub use self::stdlib_bsearch_h::bsearch;
pub use self::stdlib_float_h::atof;
pub use self::stdlib_h::{
    __compar_fn_t, atoi, atol, atoll, exit, free, malloc, strtod, strtol, strtoll,
};
use self::string_h::{memset, strerror, strlen};
pub use self::struct_FILE_h::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_EOF_SEEN, _IO_ERR_SEEN, _IO_FILE,
};
pub use self::struct_iovec_h::iovec;
pub use self::struct_timespec_h::timespec;
pub use self::sys_socket_h::{
    accept, getsockname, recvfrom, sendto, setsockopt, shutdown, sockaddr_at, sockaddr_ax25,
    sockaddr_dl, sockaddr_eon, sockaddr_inarp, sockaddr_ipx, sockaddr_iso, sockaddr_ns,
    sockaddr_x25, socket, C2RustUnnamed, SHUT_RD, SHUT_RDWR, SHUT_RDWR_0, SHUT_WR,
    __CONST_SOCKADDR_ARG, __SOCKADDR_ARG,
};
pub use self::sys_stat_h::{
    __fxstat, __fxstat64, __fxstatat, __fxstatat64, __lxstat, __lxstat64, __xmknod, __xmknodat,
    __xstat, __xstat64, fstat, fstat64, fstatat, fstatat64, lstat, lstat64, mknod, mknodat, stat,
    stat64, umask, _MKNOD_VER,
};
pub use self::sys_types_h::{dev_t, gid_t, ino_t, off_t, pid_t, ssize_t, uid_t};
pub use self::syslog_h::{
    LOG_DAEMON, LOG_DEBUG, LOG_ERR, LOG_INFO, LOG_MAIL, LOG_USER, LOG_WARNING,
};
use self::time_h::difftime;
pub use self::time_t_h::time_t;
pub use self::types_h::{
    __blkcnt64_t, __blkcnt_t, __blksize_t, __clock_t, __dev_t, __gid_t, __ino64_t, __ino_t,
    __int32_t, __intmax_t, __mode_t, __nlink_t, __off64_t, __off_t, __pid_t, __socklen_t,
    __ssize_t, __syscall_slong_t, __time_t, __uid_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
    __uintmax_t,
};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity, __uint64_identity};
use self::uio_h::writev;
pub use self::un_h::sockaddr_un;
pub use self::unistd_h::{
    _exit, alarm, chdir, close, dup2, fchown, fork, getpid, getuid, pipe, setgid, setsid, setuid,
    sysconf, unlink, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO,
};
use self::wait_h::waitpid;
pub use self::waitflags_h::WNOHANG;
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "2102:11"]
pub struct C2RustUnnamed_26 {
    pub ip: ip,
    pub icmp: icmp,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1995:3"]
pub struct C2RustUnnamed_27 {
    pub ip: ip,
    pub icmp: icmp,
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
/* Declare static char *compiler_opts  in config.h */
#[no_mangle]
#[c2rust::src_loc = "22:16"]
pub static mut dnsmasq_daemon: *mut dnsmasq_daemon =
    0 as *const dnsmasq_daemon as *mut dnsmasq_daemon;
#[c2rust::src_loc = "24:23"]
static mut pid: pid_t = 0 as libc::c_int;
#[c2rust::src_loc = "25:21"]
static mut pipewrite: libc::c_int = 0;
#[c2rust::src_loc = "35:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut bind_fallback = 0 as libc::c_int;
    let mut now: time_t = 0;
    let mut sigact = sigaction {
        __sigaction_handler: C2RustUnnamed_12 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut if_tmp = 0 as *mut iname;
    let mut piperead: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut err_pipe: [libc::c_int; 2] = [0; 2];
    let mut ent_pw = 0 as *mut passwd;
    let mut script_uid = 0 as libc::c_int as uid_t;
    let mut script_gid = 0 as libc::c_int as gid_t;
    let mut gp = 0 as *mut group;
    let mut i: libc::c_long = 0;
    let mut max_fd = sysconf(_SC_OPEN_MAX_0);
    let mut baduser = 0 as *mut libc::c_char;
    let mut log_err: libc::c_int = 0;
    let mut chown_warn = 0 as libc::c_int;
    let mut hdr = 0 as cap_user_header_t;
    let mut data = 0 as cap_user_data_t;
    let mut need_cap_net_admin = 0 as libc::c_int;
    let mut need_cap_net_raw = 0 as libc::c_int;
    let mut need_cap_net_bind_service = 0 as libc::c_int;
    let mut bound_device = 0 as *mut libc::c_char;
    let mut did_bind = 0 as libc::c_int;
    let mut serv = 0 as *mut server;
    let mut netlink_warn = 0 as *mut libc::c_char;
    let mut context = 0 as *mut dhcp_context;
    let mut relay = 0 as *mut dhcp_relay;
    let mut tftp_prefix_missing = 0 as libc::c_int;
    sigact.__sigaction_handler.sa_handler =
        Some(sig_handler as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigact.sa_flags = 0 as libc::c_int;
    sigemptyset(&mut sigact.sa_mask);
    sigaction(SIGUSR1, &mut sigact, 0 as *mut sigaction);
    sigaction(SIGUSR2, &mut sigact, 0 as *mut sigaction);
    sigaction(SIGHUP, &mut sigact, 0 as *mut sigaction);
    sigaction(SIGTERM, &mut sigact, 0 as *mut sigaction);
    sigaction(SIGALRM, &mut sigact, 0 as *mut sigaction);
    sigaction(SIGCHLD, &mut sigact, 0 as *mut sigaction);
    sigaction(SIGINT, &mut sigact, 0 as *mut sigaction);
    /* ignore SIGPIPE */
    sigact.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(SIG_IGN as libc::intptr_t); /* known umask, create leases and pid files as 0644 */
    sigaction(SIGPIPE, &mut sigact, 0 as *mut sigaction); /* Must precede read_opts() */
    umask(0o22 as libc::c_int as __mode_t);
    rand_init();
    read_opts(argc, argv, compile_opts);
    (*dnsmasq_daemon).kernel_version = kernel_version();
    if ((*dnsmasq_daemon).edns_pktsz as libc::c_int) < PACKETSZ {
        (*dnsmasq_daemon).edns_pktsz = PACKETSZ as libc::c_ushort
    }
    /* Min buffer size: we check after adding each record, so there must be
    memory for the largest packet, and the largest record so the
    min for DNS is PACKETSZ+MAXDNAME+RRFIXEDSZ which is < 1000.
    This might be increased is EDNS packet size if greater than the minimum. */
    (*dnsmasq_daemon).packet_buff_sz =
        (*dnsmasq_daemon).edns_pktsz as libc::c_int + MAXDNAME + RRFIXEDSZ;
    (*dnsmasq_daemon).packet =
        safe_malloc((*dnsmasq_daemon).packet_buff_sz as size_t) as *mut libc::c_char;
    (*dnsmasq_daemon).addrbuff = safe_malloc(ADDRSTRLEN as size_t) as *mut libc::c_char;
    if (*dnsmasq_daemon).options[(51 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (51 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        (*dnsmasq_daemon).addrbuff2 = safe_malloc(ADDRSTRLEN as size_t) as *mut libc::c_char
    }
    if (*dnsmasq_daemon).lease_file.is_null() {
        if !(*dnsmasq_daemon).dhcp.is_null() || !(*dnsmasq_daemon).dhcp6.is_null() {
            (*dnsmasq_daemon).lease_file = LEASEFILE.as_mut_ptr()
        }
    }
    /* Ensure that at least stdin, stdout and stderr (fd 0, 1, 2) exist,
    otherwise file descriptors we create can end up being 0, 1, or 2
    and then get accidentally closed later when we make 0, 1, and 2
    open to /dev/null. Normally we'll be started with 0, 1 and 2 open,
    but it's not guaranteed. By opening /dev/null three times, we
    ensure that we're not using those fds for real stuff. */
    i = 0 as libc::c_int as libc::c_long;
    while i < 3 as libc::c_int as libc::c_long {
        open(b"/dev/null\x00" as *const u8 as *const libc::c_char, O_RDWR);
        i += 1
    }
    /* Close any file descriptors we inherited apart from std{in|out|err} */
    close_fds(
        max_fd,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    );
    if (*dnsmasq_daemon).options[(45 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (45 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        die(
            b"DNSSEC not available: set HAVE_DNSSEC in src/config.h\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    if (*dnsmasq_daemon).options[(35 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (35 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        die(
            b"conntrack support not available: set HAVE_CONNTRACK in src/config.h\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    if (*dnsmasq_daemon).options[(58 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (58 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        die(
            b"Ubus not available: set HAVE_UBUS in src/config.h\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    if (*dnsmasq_daemon).max_port < (*dnsmasq_daemon).min_port {
        die(
            b"max_port cannot be smaller than min_port\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    now = dnsmasq_time();
    if !(*dnsmasq_daemon).auth_zones.is_null() {
        if (*dnsmasq_daemon).authserver.is_null() {
            die(
                b"--auth-server required when an auth zone is defined.\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char,
                EC_BADCONF,
            );
        }
        /* Create a serial at startup if not configured. */
        if (*dnsmasq_daemon).soa_sn == 0 as libc::c_int as libc::c_ulong {
            (*dnsmasq_daemon).soa_sn = now as libc::c_ulong
        }
    }
    if !(*dnsmasq_daemon).dhcp6.is_null() {
        (*dnsmasq_daemon).doing_ra = ((*dnsmasq_daemon).options[(37 as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_uint)
                << (37 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )) as libc::c_int;
        context = (*dnsmasq_daemon).dhcp6;
        while !context.is_null() {
            if (*context).flags as libc::c_uint & CONTEXT_DHCP != 0 {
                (*dnsmasq_daemon).doing_dhcp6 = 1 as libc::c_int
            }
            if (*context).flags as libc::c_uint & CONTEXT_RA != 0 {
                (*dnsmasq_daemon).doing_ra = 1 as libc::c_int
            }
            context = (*context).next
        }
    }
    /* Note that order matters here, we must call lease_init before
    creating any file descriptors which shouldn't be leaked
    to the lease-script init process. We need to call common_init
    before lease_init to allocate buffers it uses.
    The script subsystem relies on DHCP buffers, hence the last two
    conditions below. */
    if !(*dnsmasq_daemon).dhcp.is_null()
        || (*dnsmasq_daemon).doing_dhcp6 != 0
        || !(*dnsmasq_daemon).relay4.is_null()
        || !(*dnsmasq_daemon).relay6.is_null()
        || (*dnsmasq_daemon).options[(40 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (40 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
        || (*dnsmasq_daemon).options[(53 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (53 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
    {
        dhcp_common_init();
        if !(*dnsmasq_daemon).dhcp.is_null() || (*dnsmasq_daemon).doing_dhcp6 != 0 {
            lease_init(now);
        }
    }
    if !(*dnsmasq_daemon).dhcp.is_null() || !(*dnsmasq_daemon).relay4.is_null() {
        dhcp_init();
        if (*dnsmasq_daemon).options[(21 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (21 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            == 0
        {
            need_cap_net_raw = 1 as libc::c_int
        }
        need_cap_net_admin = 1 as libc::c_int
    }
    if (*dnsmasq_daemon).doing_ra != 0
        || (*dnsmasq_daemon).doing_dhcp6 != 0
        || !(*dnsmasq_daemon).relay6.is_null()
    {
        ra_init(now);
        need_cap_net_raw = 1 as libc::c_int;
        need_cap_net_admin = 1 as libc::c_int
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 || !(*dnsmasq_daemon).relay6.is_null() {
        dhcp6_init();
    }
    if !(*dnsmasq_daemon).ipsets.is_null() {
        ipset_init();
        need_cap_net_admin = 1 as libc::c_int
    }
    netlink_warn = netlink_init();
    if (*dnsmasq_daemon).options[(13 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (13 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
        && (*dnsmasq_daemon).options[(39 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (39 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
    {
        die(
            b"cannot set --bind-interfaces and --bind-dynamic\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    if enumerate_interfaces(1 as libc::c_int) == 0 || enumerate_interfaces(0 as libc::c_int) == 0 {
        die(
            b"failed to find list of interfaces: %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_MISC,
        );
    }
    if (*dnsmasq_daemon).options[(13 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (13 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
        || (*dnsmasq_daemon).options[(39 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (39 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
    {
        create_bound_listeners(1 as libc::c_int);
        if (*dnsmasq_daemon).options[(39 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (39 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            == 0
        {
            if_tmp = (*dnsmasq_daemon).if_names;
            while !if_tmp.is_null() {
                if !(*if_tmp).name.is_null() && (*if_tmp).used == 0 {
                    die(
                        b"unknown interface %s\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*if_tmp).name,
                        EC_BADNET,
                    );
                }
                if_tmp = (*if_tmp).next
            }
        }
        /* after enumerate_interfaces()  */
        bound_device = whichdevice();
        if !(*dnsmasq_daemon).dhcp.is_null() {
            if (*dnsmasq_daemon).relay4.is_null() && !bound_device.is_null() {
                bindtodevice(bound_device, (*dnsmasq_daemon).dhcpfd);
                did_bind = 1 as libc::c_int
            }
            if (*dnsmasq_daemon).enable_pxe != 0 && !bound_device.is_null() {
                bindtodevice(bound_device, (*dnsmasq_daemon).pxefd);
                did_bind = 1 as libc::c_int
            }
        }
        if (*dnsmasq_daemon).doing_dhcp6 != 0
            && (*dnsmasq_daemon).relay6.is_null()
            && !bound_device.is_null()
        {
            bindtodevice(bound_device, (*dnsmasq_daemon).dhcp6fd);
            did_bind = 1 as libc::c_int
        }
    } else {
        create_wildcard_listeners();
    }
    /* after enumerate_interfaces() */
    if (*dnsmasq_daemon).doing_dhcp6 != 0
        || !(*dnsmasq_daemon).relay6.is_null()
        || (*dnsmasq_daemon).doing_ra != 0
    {
        join_multicast(1 as libc::c_int);
    }
    /* After netlink_init() and before create_helper() */
    lease_make_duid(now);
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        cache_init();
        blockdata_init();
        hash_questions_init();
    }
    if ((*dnsmasq_daemon).port != 0 as libc::c_int
        || !(*dnsmasq_daemon).dhcp.is_null()
        || (*dnsmasq_daemon).doing_dhcp6 != 0)
        && ((*dnsmasq_daemon).options[(8 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (8 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            == 0
            || !(*dnsmasq_daemon).dynamic_dirs.is_null())
    {
        inotify_dnsmasq_init();
    } else {
        (*dnsmasq_daemon).inotifyfd = -(1 as libc::c_int)
    }
    if !(*dnsmasq_daemon).dump_file.is_null() {
        dump_init();
    } else {
        (*dnsmasq_daemon).dumpfd = -(1 as libc::c_int)
    }
    if (*dnsmasq_daemon).options[(19 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (19 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        die(
            b"DBus not available: set HAVE_DBUS in src/config.h\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    if (*dnsmasq_daemon).options[(58 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (58 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        die(
            b"UBus not available: set HAVE_UBUS in src/config.h\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
            EC_BADCONF,
        );
    }
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        pre_allocate_sfds();
    }
    /* Note getpwnam returns static storage */
    if (!(*dnsmasq_daemon).dhcp.is_null() || !(*dnsmasq_daemon).dhcp6.is_null())
        && !(*dnsmasq_daemon).scriptuser.is_null()
        && (!(*dnsmasq_daemon).lease_change_command.is_null()
            || !(*dnsmasq_daemon).luascript.is_null())
    {
        let mut scr_pw = 0 as *mut passwd;
        scr_pw = getpwnam((*dnsmasq_daemon).scriptuser);
        if !scr_pw.is_null() {
            script_uid = (*scr_pw).pw_uid;
            script_gid = (*scr_pw).pw_gid
        } else {
            baduser = (*dnsmasq_daemon).scriptuser
        }
    }
    if !(*dnsmasq_daemon).username.is_null() && {
        ent_pw = getpwnam((*dnsmasq_daemon).username);
        ent_pw.is_null()
    } {
        baduser = (*dnsmasq_daemon).username
    } else if !(*dnsmasq_daemon).groupname.is_null() && {
        gp = getgrnam((*dnsmasq_daemon).groupname);
        gp.is_null()
    } {
        baduser = (*dnsmasq_daemon).groupname
    }
    if !baduser.is_null() {
        die(
            b"unknown user or group: %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            baduser,
            EC_BADCONF,
        );
    }
    /* implement group defaults, "dip" if available, or group associated with uid */
    if (*dnsmasq_daemon).group_set == 0 && gp.is_null() {
        gp = getgrnam(CHGRP.as_ptr());
        if gp.is_null() && !ent_pw.is_null() {
            gp = getgrgid((*ent_pw).pw_gid)
        }
        /* for error message */
        if !gp.is_null() {
            (*dnsmasq_daemon).groupname = (*gp).gr_name
        }
    }
    /* We keep CAP_NETADMIN (for ARP-injection) and
    CAP_NET_RAW (for icmp) if we're doing dhcp,
    if we have yet to bind ports because of DAD,
    or we're doing it dynamically, we need CAP_NET_BIND_SERVICE. */
    if (is_dad_listeners() != 0
        || (*dnsmasq_daemon).options[(39 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (39 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0)
        && ((*dnsmasq_daemon).options[(40 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (40 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
            || (*dnsmasq_daemon).port != 0 as libc::c_int
                && (*dnsmasq_daemon).port <= 1024 as libc::c_int)
    {
        need_cap_net_bind_service = 1 as libc::c_int
    }
    /* usptream servers which bind to an interface call SO_BINDTODEVICE
    for each TCP connection, so need CAP_NET_RAW */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).interface[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
            need_cap_net_raw = 1 as libc::c_int
        }
        serv = (*serv).next
    }
    /* If we're doing Dbus or UBus, the above can be set dynamically,
    (as can ports) so always (potentially) needed. */
    /* determine capability API version here, while we can still
    call safe_malloc */
    let mut capsize = 1 as libc::c_int; /* for header version 1 */
    let mut fail = 0 as *mut libc::c_char;
    hdr = safe_malloc(::std::mem::size_of::<__user_cap_header_struct>() as libc::c_ulong)
        as cap_user_header_t;
    /* find version supported by kernel */
    memset(
        hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<__user_cap_header_struct>() as libc::c_ulong,
    );
    capget(hdr, 0 as cap_user_data_t);
    if (*hdr).version != LINUX_CAPABILITY_VERSION_1 as libc::c_uint {
        /* if unknown version, use largest supported version (3) */
        if (*hdr).version != LINUX_CAPABILITY_VERSION_2 as libc::c_uint {
            (*hdr).version = LINUX_CAPABILITY_VERSION_3 as __u32
        } /* Get current values, for verification */
        capsize = 2 as libc::c_int
    }
    data = safe_malloc(
        (::std::mem::size_of::<__user_cap_data_struct>() as libc::c_ulong)
            .wrapping_mul(capsize as libc::c_ulong),
    ) as cap_user_data_t;
    capget(hdr, data);
    if need_cap_net_admin != 0
        && (*data).permitted & ((1 as libc::c_int) << CAP_NET_ADMIN) as libc::c_uint == 0
    {
        fail = b"NET_ADMIN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if need_cap_net_raw != 0
        && (*data).permitted & ((1 as libc::c_int) << CAP_NET_RAW) as libc::c_uint == 0
    {
        fail = b"NET_RAW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if need_cap_net_bind_service != 0
        && (*data).permitted & ((1 as libc::c_int) << CAP_NET_BIND_SERVICE) as libc::c_uint == 0
    {
        fail = b"NET_BIND_SERVICE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if !fail.is_null() {
        die(
            b"process is missing required capability %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            fail,
            EC_MISC,
        );
    }
    /* Now set bitmaps to set caps after daemonising */
    memset(
        data as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<__user_cap_data_struct>() as libc::c_ulong)
            .wrapping_mul(capsize as libc::c_ulong),
    );
    if need_cap_net_admin != 0 {
        (*data).effective |= ((1 as libc::c_int) << CAP_NET_ADMIN) as libc::c_uint
    }
    if need_cap_net_raw != 0 {
        (*data).effective |= ((1 as libc::c_int) << CAP_NET_RAW) as libc::c_uint
    }
    if need_cap_net_bind_service != 0 {
        (*data).effective |= ((1 as libc::c_int) << CAP_NET_BIND_SERVICE) as libc::c_uint
    }
    (*data).permitted = (*data).effective;
    /* Use a pipe to carry signals and other events back to the event loop
    in a race-free manner and another to carry errors to daemon-invoking process */
    safe_pipe(pipefd.as_mut_ptr(), 1 as libc::c_int);
    piperead = pipefd[0 as libc::c_int as usize];
    ::std::ptr::write_volatile(
        &mut pipewrite as *mut libc::c_int,
        pipefd[1 as libc::c_int as usize],
    );
    /* prime the pipe to load stuff first time. */
    send_event(
        pipewrite,
        EVENT_INIT,
        0 as libc::c_int,
        0 as *mut libc::c_char,
    );
    err_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
    if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        /* The following code "daemonizes" the process.
        See Stevens section 12.4 */
        if chdir(b"/\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
            die(
                b"cannot chdir to filesystem root: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
                EC_MISC,
            );
        }
        if (*dnsmasq_daemon).options[(16 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (16 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            == 0
        {
            let mut pid_0: pid_t = 0;
            /* pipe to carry errors back to original process.
            When startup is complete we close this and the process terminates. */
            safe_pipe(err_pipe.as_mut_ptr(), 0 as libc::c_int);
            pid_0 = fork();
            if pid_0 == -(1 as libc::c_int) {
                /* fd == -1 since we've not forked, never returns. */
                send_event(
                    -(1 as libc::c_int),
                    EVENT_FORK_ERR,
                    errno,
                    0 as *mut libc::c_char,
                );
            }
            if pid_0 != 0 as libc::c_int {
                let mut ev = event_desc {
                    event: 0,
                    data: 0,
                    msg_sz: 0,
                };
                let mut msg = 0 as *mut libc::c_char;
                /* close our copy of write-end */
                close(err_pipe[1 as libc::c_int as usize]);
                /* check for errors after the fork */
                if read_event(err_pipe[0 as libc::c_int as usize], &mut ev, &mut msg) != 0 {
                    fatal_event(&mut ev, msg);
                }
                _exit(EC_GOOD);
            }
            close(err_pipe[0 as libc::c_int as usize]);
            /* NO calls to die() from here on. */
            setsid();
            pid_0 = fork();
            if pid_0 == -(1 as libc::c_int) {
                send_event(
                    err_pipe[1 as libc::c_int as usize],
                    EVENT_FORK_ERR,
                    errno,
                    0 as *mut libc::c_char,
                );
            }
            if pid_0 != 0 as libc::c_int {
                _exit(0 as libc::c_int);
            }
        }
        /* write pidfile _after_ forking ! */
        if !(*dnsmasq_daemon).runfile.is_null() {
            let mut fd: libc::c_int = 0;
            let mut err = 0 as libc::c_int;
            sprintf(
                (*dnsmasq_daemon).namebuff,
                b"%d\n\x00" as *const u8 as *const libc::c_char,
                getpid(),
            );
            /* Explanation: Some installations of dnsmasq (eg Debian/Ubuntu) locate the pid-file
               in a directory which is writable by the non-privileged user that dnsmasq runs as. This
               allows the daemon to delete the file as part of its shutdown. This is a security hole to the
               extent that an attacker running as the unprivileged  user could replace the pidfile with a
               symlink, and have the target of that symlink overwritten as root next time dnsmasq starts.

               The following code first deletes any existing file, and then opens it with the O_EXCL flag,
               ensuring that the open() fails should there be any existing file (because the unlink() failed,
               or an attacker exploited the race between unlink() and open()). This ensures that no symlink
               attack can succeed.

               Any compromise of the non-privileged user still theoretically allows the pid-file to be
               replaced whilst dnsmasq is running. The worst that could allow is that the usual
               "shutdown dnsmasq" shell command could be tricked into stopping any other process.

               Note that if dnsmasq is started as non-root (eg for testing) it silently ignores
               failure to write the pid-file.
            */
            unlink((*dnsmasq_daemon).runfile);
            fd = open(
                (*dnsmasq_daemon).runfile,
                O_WRONLY | O_CREAT | O_TRUNC | O_EXCL,
                S_IWUSR | S_IRUSR | S_IRGRP | S_IROTH,
            );
            if fd == -(1 as libc::c_int) {
                /* only complain if started as root */
                if getuid() == 0 as libc::c_int as libc::c_uint {
                    err = 1 as libc::c_int
                }
            } else {
                /* We're still running as root here. Change the ownership of the PID file
                to the user we will be running as. Note that this is not to allow
                us to delete the file, since that depends on the permissions
                of the directory containing the file. That directory will
                need to by owned by the dnsmasq user, and the ownership of the
                file has to match, to keep systemd >273 happy. */
                if getuid() == 0 as libc::c_int as libc::c_uint
                    && !ent_pw.is_null()
                    && (*ent_pw).pw_uid != 0 as libc::c_int as libc::c_uint
                    && fchown(fd, (*ent_pw).pw_uid, (*ent_pw).pw_gid) == -(1 as libc::c_int)
                {
                    chown_warn = errno
                }
                if read_write(
                    fd,
                    (*dnsmasq_daemon).namebuff as *mut libc::c_uchar,
                    strlen((*dnsmasq_daemon).namebuff) as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    err = 1 as libc::c_int
                } else if close(fd) == -(1 as libc::c_int) {
                    err = 1 as libc::c_int
                }
            }
            if err != 0 {
                send_event(
                    err_pipe[1 as libc::c_int as usize],
                    EVENT_PIDFILE,
                    errno,
                    (*dnsmasq_daemon).runfile,
                );
                _exit(0 as libc::c_int);
            }
        }
    }
    log_err = log_start(ent_pw, err_pipe[1 as libc::c_int as usize]);
    if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        /* open  stdout etc to /dev/null */
        let mut nullfd = open(b"/dev/null\x00" as *const u8 as *const libc::c_char, O_RDWR);
        if nullfd != -(1 as libc::c_int) {
            dup2(nullfd, STDOUT_FILENO);
            dup2(nullfd, STDERR_FILENO);
            dup2(nullfd, STDIN_FILENO);
            close(nullfd);
        }
    }
    /* if we are to run scripts, we need to fork a helper before dropping root. */
    (*dnsmasq_daemon).helperfd = -(1 as libc::c_int);
    if (!(*dnsmasq_daemon).dhcp.is_null()
        || !(*dnsmasq_daemon).dhcp6.is_null()
        || (*dnsmasq_daemon).options[(40 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (40 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
        || (*dnsmasq_daemon).options[(53 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (53 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0)
        && (!(*dnsmasq_daemon).lease_change_command.is_null()
            || !(*dnsmasq_daemon).luascript.is_null())
    {
        (*dnsmasq_daemon).helperfd = create_helper(
            pipewrite,
            err_pipe[1 as libc::c_int as usize],
            script_uid,
            script_gid,
            max_fd,
        )
    }
    if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
        && getuid() == 0 as libc::c_int as libc::c_uint
    {
        let mut bad_capabilities = 0 as libc::c_int;
        let mut dummy: gid_t = 0;
        /* remove all supplementary groups */
        if !gp.is_null()
            && (setgroups(0 as libc::c_int as size_t, &mut dummy) == -(1 as libc::c_int)
                || setgid((*gp).gr_gid) == -(1 as libc::c_int))
        {
            send_event(
                err_pipe[1 as libc::c_int as usize],
                EVENT_GROUP_ERR,
                errno,
                (*dnsmasq_daemon).groupname,
            );
            _exit(0 as libc::c_int);
        }
        if !ent_pw.is_null() && (*ent_pw).pw_uid != 0 as libc::c_int as libc::c_uint {
            /* Need to be able to drop root. */
            (*data).effective |= ((1 as libc::c_int) << CAP_SETUID) as libc::c_uint;
            (*data).permitted |= ((1 as libc::c_int) << CAP_SETUID) as libc::c_uint;
            /* Tell kernel to not clear capabilities when dropping root */
            if capset(hdr, data) == -(1 as libc::c_int)
                || prctl(
                    PR_SET_KEEPCAPS,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == -(1 as libc::c_int)
            {
                bad_capabilities = errno
            }
            if bad_capabilities != 0 as libc::c_int {
                send_event(
                    err_pipe[1 as libc::c_int as usize],
                    EVENT_CAP_ERR,
                    bad_capabilities,
                    0 as *mut libc::c_char,
                );
                _exit(0 as libc::c_int);
            }
            /* finally drop root */
            if setuid((*ent_pw).pw_uid) == -(1 as libc::c_int) {
                send_event(
                    err_pipe[1 as libc::c_int as usize],
                    EVENT_USER_ERR,
                    errno,
                    (*dnsmasq_daemon).username,
                );
                _exit(0 as libc::c_int);
            }
            (*data).effective &= !((1 as libc::c_int) << CAP_SETUID) as libc::c_uint;
            (*data).permitted &= !((1 as libc::c_int) << CAP_SETUID) as libc::c_uint;
            /* lose the setuid capability */
            if capset(hdr, data) == -(1 as libc::c_int) {
                send_event(
                    err_pipe[1 as libc::c_int as usize],
                    EVENT_CAP_ERR,
                    errno,
                    0 as *mut libc::c_char,
                );
                _exit(0 as libc::c_int);
            }
        }
    }
    free(hdr as *mut libc::c_void);
    free(data as *mut libc::c_void);
    if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        prctl(
            PR_SET_DUMPABLE,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    if (*dnsmasq_daemon).options[(40 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (40 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        let mut dir = 0 as *mut DIR;
        let mut p = 0 as *mut tftp_prefix;
        if !(*dnsmasq_daemon).tftp_prefix.is_null() {
            dir = opendir((*dnsmasq_daemon).tftp_prefix);
            if dir.is_null() {
                tftp_prefix_missing = 1 as libc::c_int;
                if (*dnsmasq_daemon).options[(52 as libc::c_int as libc::c_ulong).wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                    & (1 as libc::c_uint)
                        << (52 as libc::c_int as libc::c_ulong).wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        )
                    == 0
                {
                    send_event(
                        err_pipe[1 as libc::c_int as usize],
                        EVENT_TFTP_ERR,
                        errno,
                        (*dnsmasq_daemon).tftp_prefix,
                    );
                    _exit(0 as libc::c_int);
                }
            } else {
                closedir(dir);
            }
        }
        p = (*dnsmasq_daemon).if_prefix;
        while !p.is_null() {
            (*p).missing = 0 as libc::c_int;
            dir = opendir((*p).prefix);
            if dir.is_null() {
                (*p).missing = 1 as libc::c_int;
                if (*dnsmasq_daemon).options[(52 as libc::c_int as libc::c_ulong).wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                    & (1 as libc::c_uint)
                        << (52 as libc::c_int as libc::c_ulong).wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        )
                    == 0
                {
                    send_event(
                        err_pipe[1 as libc::c_int as usize],
                        EVENT_TFTP_ERR,
                        errno,
                        (*p).prefix,
                    );
                    _exit(0 as libc::c_int);
                }
            } else {
                closedir(dir);
            }
            p = (*p).next
        }
    }
    if (*dnsmasq_daemon).port == 0 as libc::c_int {
        my_syslog(
            LOG_INFO,
            b"started, version %s DNS disabled\x00" as *const u8 as *const libc::c_char,
            VERSION.as_ptr(),
        );
    } else {
        if (*dnsmasq_daemon).cachesize != 0 as libc::c_int {
            my_syslog(
                LOG_INFO,
                b"started, version %s cachesize %d\x00" as *const u8 as *const libc::c_char,
                VERSION.as_ptr(),
                (*dnsmasq_daemon).cachesize,
            );
            if (*dnsmasq_daemon).cachesize > 10000 as libc::c_int {
                my_syslog(LOG_WARNING,
                          b"cache size greater than 10000 may cause performance issues, and is unlikely to be useful.\x00"
                              as *const u8 as *const libc::c_char);
            }
        } else {
            my_syslog(
                LOG_INFO,
                b"started, version %s cache disabled\x00" as *const u8 as *const libc::c_char,
                VERSION.as_ptr(),
            );
        }
        if (*dnsmasq_daemon).options[(49 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (49 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
        {
            my_syslog(
                LOG_INFO,
                b"DNS service limited to local subnets\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    my_syslog(
        LOG_INFO,
        b"compile time options: %s\x00" as *const u8 as *const libc::c_char,
        compile_opts,
    );
    if chown_warn != 0 as libc::c_int {
        my_syslog(
            LOG_WARNING,
            b"chown of PID file %s failed: %s\x00" as *const u8 as *const libc::c_char,
            (*dnsmasq_daemon).runfile,
            strerror(chown_warn),
        );
    }
    if log_err != 0 as libc::c_int {
        my_syslog(
            LOG_WARNING,
            b"warning: failed to change owner of %s: %s\x00" as *const u8 as *const libc::c_char,
            (*dnsmasq_daemon).log_file,
            strerror(log_err),
        );
    }
    if bind_fallback != 0 {
        my_syslog(
            LOG_WARNING,
            b"setting --bind-interfaces option because of OS limitations\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if (*dnsmasq_daemon).options[(13 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (13 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        warn_bound_listeners();
    } else if (*dnsmasq_daemon).options[(39 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (39 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        warn_wild_labels();
    }
    warn_int_names();
    if (*dnsmasq_daemon).options[(13 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (13 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        if_tmp = (*dnsmasq_daemon).if_names;
        while !if_tmp.is_null() {
            if !(*if_tmp).name.is_null() && (*if_tmp).used == 0 {
                my_syslog(
                    LOG_WARNING,
                    b"warning: interface %s does not currently exist\x00" as *const u8
                        as *const libc::c_char,
                    (*if_tmp).name,
                );
            }
            if_tmp = (*if_tmp).next
        }
    }
    if (*dnsmasq_daemon).port != 0 as libc::c_int
        && (*dnsmasq_daemon).options[(8 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (8 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
    {
        if !(*dnsmasq_daemon).resolv_files.is_null()
            && (*(*dnsmasq_daemon).resolv_files).is_default == 0
        {
            my_syslog(
                LOG_WARNING,
                b"warning: ignoring resolv-file flag because no-resolv is set\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        (*dnsmasq_daemon).resolv_files = 0 as *mut resolvc;
        if (*dnsmasq_daemon).servers.is_null() {
            my_syslog(
                LOG_WARNING,
                b"warning: no upstream servers configured\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*dnsmasq_daemon).max_logs != 0 as libc::c_int {
        my_syslog(
            LOG_INFO,
            b"asynchronous logging enabled, queue limit is %d messages\x00" as *const u8
                as *const libc::c_char,
            (*dnsmasq_daemon).max_logs,
        );
    }
    context = (*dnsmasq_daemon).dhcp;
    while !context.is_null() {
        log_context(AF_INET, context);
        context = (*context).next
    }
    relay = (*dnsmasq_daemon).relay4;
    while !relay.is_null() {
        log_relay(AF_INET, relay);
        relay = (*relay).next
    }
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        log_context(AF_INET6, context);
        context = (*context).next
    }
    relay = (*dnsmasq_daemon).relay6;
    while !relay.is_null() {
        log_relay(AF_INET6, relay);
        relay = (*relay).next
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 || (*dnsmasq_daemon).doing_ra != 0 {
        dhcp_construct_contexts(now);
    }
    if (*dnsmasq_daemon).options[(37 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (37 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        my_syslog(
            MS_DHCP | LOG_INFO,
            b"IPv6 router advertisement enabled\x00" as *const u8 as *const libc::c_char,
        );
    }
    if did_bind != 0 {
        my_syslog(
            MS_DHCP | LOG_INFO,
            b"DHCP, sockets bound exclusively to interface %s\x00" as *const u8
                as *const libc::c_char,
            bound_device,
        );
    }
    if !netlink_warn.is_null() {
        my_syslog(LOG_WARNING, netlink_warn);
    }
    /* after dhcp_construct_contexts */
    if !(*dnsmasq_daemon).dhcp.is_null() || (*dnsmasq_daemon).doing_dhcp6 != 0 {
        lease_find_interfaces(now);
    }
    if (*dnsmasq_daemon).options[(40 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (40 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        != 0
    {
        let mut p_0 = 0 as *mut tftp_prefix;
        my_syslog(
            MS_TFTP | LOG_INFO,
            b"TFTP %s%s %s %s\x00" as *const u8 as *const libc::c_char,
            if !(*dnsmasq_daemon).tftp_prefix.is_null() {
                b"root is \x00" as *const u8 as *const libc::c_char
            } else {
                b"enabled\x00" as *const u8 as *const libc::c_char
            },
            if !(*dnsmasq_daemon).tftp_prefix.is_null() {
                (*dnsmasq_daemon).tftp_prefix as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if (*dnsmasq_daemon).options[(26 as libc::c_int as libc::c_ulong).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
                & (1 as libc::c_uint)
                    << (26 as libc::c_int as libc::c_ulong).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                != 0
            {
                b"secure mode\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if (*dnsmasq_daemon).options[(60 as libc::c_int as libc::c_ulong).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
                & (1 as libc::c_uint)
                    << (60 as libc::c_int as libc::c_ulong).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                != 0
            {
                b"single port mode\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
        if tftp_prefix_missing != 0 {
            my_syslog(
                MS_TFTP | LOG_WARNING,
                b"warning: %s inaccessible\x00" as *const u8 as *const libc::c_char,
                (*dnsmasq_daemon).tftp_prefix,
            );
        }
        p_0 = (*dnsmasq_daemon).if_prefix;
        while !p_0.is_null() {
            if (*p_0).missing != 0 {
                my_syslog(
                    MS_TFTP | LOG_WARNING,
                    b"warning: TFTP directory %s inaccessible\x00" as *const u8
                        as *const libc::c_char,
                    (*p_0).prefix,
                );
            }
            p_0 = (*p_0).next
        }
        /* This is a guess, it assumes that for small limits,
        disjoint files might be served, but for large limits,
        a single file will be sent to may clients (the file only needs
        one fd). */
        max_fd -= 30 as libc::c_int as libc::c_long; /* use other than TFTP */
        if max_fd < 0 as libc::c_int as libc::c_long {
            max_fd = 5 as libc::c_int as libc::c_long
        } else if max_fd < 100 as libc::c_int as libc::c_long
            && (*dnsmasq_daemon).options[(60 as libc::c_int as libc::c_ulong).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
                & (1 as libc::c_uint)
                    << (60 as libc::c_int as libc::c_ulong).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                == 0
        {
            max_fd = max_fd / 2 as libc::c_int as libc::c_long
        } else {
            max_fd = max_fd - 20 as libc::c_int as libc::c_long
        }
        /* if we have to use a limited range of ports,
        that will limit the number of transfers */
        if (*dnsmasq_daemon).start_tftp_port != 0 as libc::c_int
            && (((*dnsmasq_daemon).end_tftp_port - (*dnsmasq_daemon).start_tftp_port
                + 1 as libc::c_int) as libc::c_long)
                < max_fd
        {
            max_fd = ((*dnsmasq_daemon).end_tftp_port - (*dnsmasq_daemon).start_tftp_port
                + 1 as libc::c_int) as libc::c_long
        }
        if (*dnsmasq_daemon).tftp_max as libc::c_long > max_fd {
            (*dnsmasq_daemon).tftp_max = max_fd as libc::c_int;
            my_syslog(
                MS_TFTP | LOG_WARNING,
                b"restricting maximum simultaneous TFTP transfers to %d\x00" as *const u8
                    as *const libc::c_char,
                (*dnsmasq_daemon).tftp_max,
            );
        }
    }
    /* finished start-up - release original process */
    if err_pipe[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(err_pipe[1 as libc::c_int as usize]);
    }
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        check_servers();
    }
    ::std::ptr::write_volatile(&mut pid as *mut pid_t, getpid());
    (*dnsmasq_daemon).pipe_to_parent = -(1 as libc::c_int);
    i = 0 as libc::c_int as libc::c_long;
    while i < MAX_PROCS as libc::c_long {
        (*dnsmasq_daemon).tcp_pipes[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    /* Using inotify, have to select a resolv file at startup */
    poll_resolv(1 as libc::c_int, 0 as libc::c_int, now);
    loop {
        let mut t: libc::c_int = 0;
        let mut timeout = -(1 as libc::c_int);
        poll_reset();
        /* if we are out of resources, find how long we have to wait
        for some to come free, we'll loop around then and restart
        listening for queries */
        t = set_dns_listeners(now);
        if t != 0 as libc::c_int {
            timeout = t * 1000 as libc::c_int
        }
        /* Whilst polling for the dbus, or doing a tftp transfer, wake every quarter second */
        if !(*dnsmasq_daemon).tftp_trans.is_null()
            || (*dnsmasq_daemon).options[(19 as libc::c_int as libc::c_ulong).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
                & (1 as libc::c_uint)
                    << (19 as libc::c_int as libc::c_ulong).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                != 0
                && (*dnsmasq_daemon).dbus.is_null()
        {
            timeout = 250 as libc::c_int
        } else if is_dad_listeners() != 0 {
            timeout = 1000 as libc::c_int
        }
        if !(*dnsmasq_daemon).dhcp.is_null() || !(*dnsmasq_daemon).relay4.is_null() {
            poll_listen((*dnsmasq_daemon).dhcpfd, POLLIN as libc::c_short);
            if (*dnsmasq_daemon).pxefd != -(1 as libc::c_int) {
                poll_listen((*dnsmasq_daemon).pxefd, POLLIN as libc::c_short);
            }
        }
        if (*dnsmasq_daemon).doing_dhcp6 != 0 || !(*dnsmasq_daemon).relay6.is_null() {
            poll_listen((*dnsmasq_daemon).dhcp6fd, POLLIN as libc::c_short);
        }
        if (*dnsmasq_daemon).doing_ra != 0 {
            poll_listen((*dnsmasq_daemon).icmp6fd, POLLIN as libc::c_short);
        }
        if (*dnsmasq_daemon).inotifyfd != -(1 as libc::c_int) {
            poll_listen((*dnsmasq_daemon).inotifyfd, POLLIN as libc::c_short);
        }
        poll_listen((*dnsmasq_daemon).netlinkfd, POLLIN as libc::c_short);
        poll_listen(piperead, POLLIN as libc::c_short);
        while helper_buf_empty() != 0 && do_script_run(now) != 0 {}
        /* Wake every second whilst waiting for DAD to complete */
        /* Refresh cache */
        if (*dnsmasq_daemon).options[(53 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (53 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
        {
            find_mac(
                0 as *mut mysockaddr,
                0 as *mut libc::c_uchar,
                0 as libc::c_int,
                now,
            );
        }
        while helper_buf_empty() != 0 && do_arp_script_run() != 0 {}
        while helper_buf_empty() != 0 && do_tftp_script_run() != 0 {}
        if helper_buf_empty() == 0 {
            poll_listen((*dnsmasq_daemon).helperfd, POLLOUT as libc::c_short);
        }
        /* must do this just before do_poll(), when we know no
        more calls to my_syslog() can occur */
        set_log_writer();
        if do_poll(timeout) < 0 as libc::c_int {
            continue;
        }
        now = dnsmasq_time();
        check_log_writer(0 as libc::c_int);
        /* prime. */
        enumerate_interfaces(1 as libc::c_int);
        /* Check the interfaces to see if any have exited DAD state
        and if so, bind the address. */
        if is_dad_listeners() != 0 {
            enumerate_interfaces(0 as libc::c_int);
            /* NB, is_dad_listeners() == 1 --> we're binding interfaces */
            create_bound_listeners(0 as libc::c_int);
            warn_bound_listeners();
        }
        if poll_check((*dnsmasq_daemon).netlinkfd, POLLIN as libc::c_short) != 0 {
            netlink_multicast();
        }
        if (*dnsmasq_daemon).inotifyfd != -(1 as libc::c_int)
            && poll_check((*dnsmasq_daemon).inotifyfd, POLLIN as libc::c_short) != 0
            && inotify_check(now) != 0
        {
            if (*dnsmasq_daemon).port != 0 as libc::c_int
                && (*dnsmasq_daemon).options[(5 as libc::c_int as libc::c_ulong).wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                    & (1 as libc::c_uint)
                        << (5 as libc::c_int as libc::c_ulong).wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        )
                    == 0
            {
                poll_resolv(1 as libc::c_int, 1 as libc::c_int, now);
            }
        }
        if poll_check(piperead, POLLIN as libc::c_short) != 0 {
            async_event(piperead, now);
        }
        check_dns_listeners(now);
        check_tftp_listeners(now);
        if !(*dnsmasq_daemon).dhcp.is_null() || !(*dnsmasq_daemon).relay4.is_null() {
            if poll_check((*dnsmasq_daemon).dhcpfd, POLLIN as libc::c_short) != 0 {
                dhcp_packet(now, 0 as libc::c_int);
            }
            if (*dnsmasq_daemon).pxefd != -(1 as libc::c_int)
                && poll_check((*dnsmasq_daemon).pxefd, POLLIN as libc::c_short) != 0
            {
                dhcp_packet(now, 1 as libc::c_int);
            }
        }
        if ((*dnsmasq_daemon).doing_dhcp6 != 0 || !(*dnsmasq_daemon).relay6.is_null())
            && poll_check((*dnsmasq_daemon).dhcp6fd, POLLIN as libc::c_short) != 0
        {
            dhcp6_packet(now);
        }
        if (*dnsmasq_daemon).doing_ra != 0
            && poll_check((*dnsmasq_daemon).icmp6fd, POLLIN as libc::c_short) != 0
        {
            icmp6_packet(now);
        }
        if (*dnsmasq_daemon).helperfd != -(1 as libc::c_int)
            && poll_check((*dnsmasq_daemon).helperfd, POLLOUT as libc::c_short) != 0
        {
            helper_write();
        }
    }
}
#[c2rust::src_loc = "1231:1"]
unsafe extern "C" fn sig_handler(mut sig: libc::c_int) {
    if pid == 0 as libc::c_int {
        /* ignore anything other than TERM during startup
        and in helper proc. (helper ignore TERM too) */
        if sig == SIGTERM || sig == SIGINT {
            exit(EC_MISC);
        }
    } else if pid != getpid() {
        /* alarm is used to kill TCP children after a fixed time. */
        if sig == SIGALRM {
            _exit(0 as libc::c_int);
        }
    } else {
        /* master process */
        let mut event: libc::c_int = 0;
        let mut errsave = errno;
        if sig == SIGHUP {
            event = EVENT_RELOAD
        } else if sig == SIGCHLD {
            event = EVENT_CHILD
        } else if sig == SIGALRM {
            event = EVENT_ALARM
        } else if sig == SIGTERM {
            event = EVENT_TERM
        } else if sig == SIGUSR1 {
            event = EVENT_DUMP
        } else if sig == SIGUSR2 {
            event = EVENT_REOPEN
        } else if sig == SIGINT {
            /* Handle SIGINT normally in debug mode, so
            ctrl-c continues to operate. */
            if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
                & (1 as libc::c_uint)
                    << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                != 0
            {
                exit(EC_MISC);
            } else {
                event = EVENT_TIME
            }
        } else {
            return;
        }
        send_event(pipewrite, event, 0 as libc::c_int, 0 as *mut libc::c_char);
        errno = errsave
    };
}
/* now == 0 -> queue immediate callback */
#[no_mangle]
#[c2rust::src_loc = "1281:1"]
pub unsafe extern "C" fn send_alarm(mut event: time_t, mut now: time_t) {
    if now == 0 as libc::c_int as libc::c_long || event != 0 as libc::c_int as libc::c_long {
        /* alarm(0) or alarm(-ve) doesn't do what we want.... */
        if now == 0 as libc::c_int as libc::c_long || difftime(event, now) <= 0.0f64 {
            send_event(
                pipewrite,
                EVENT_ALARM,
                0 as libc::c_int,
                0 as *mut libc::c_char,
            );
        } else {
            alarm(difftime(event, now) as libc::c_uint);
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1293:1"]
pub unsafe extern "C" fn queue_event(mut event: libc::c_int) {
    send_event(pipewrite, event, 0 as libc::c_int, 0 as *mut libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "1298:1"]
pub unsafe extern "C" fn send_event(
    mut fd: libc::c_int,
    mut event: libc::c_int,
    mut data: libc::c_int,
    mut msg: *mut libc::c_char,
) {
    let mut ev = event_desc {
        event: 0,
        data: 0,
        msg_sz: 0,
    };
    let mut iov: [iovec; 2] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 2];
    ev.event = event;
    ev.data = data;
    ev.msg_sz = if !msg.is_null() {
        strlen(msg)
    } else {
        0 as libc::c_int as libc::c_ulong
    } as libc::c_int;
    iov[0 as libc::c_int as usize].iov_base = &mut ev as *mut event_desc as *mut libc::c_void;
    iov[0 as libc::c_int as usize].iov_len = ::std::mem::size_of::<event_desc>() as libc::c_ulong;
    iov[1 as libc::c_int as usize].iov_base = msg as *mut libc::c_void;
    iov[1 as libc::c_int as usize].iov_len = ev.msg_sz as size_t;
    /* error pipe, debug mode. */
    if fd == -(1 as libc::c_int) {
        fatal_event(&mut ev, msg);
    } else {
        /* pipe is non-blocking and struct event_desc is smaller than
        PIPE_BUF, so this either fails or writes everything */
        while writev(
            fd,
            iov.as_mut_ptr(),
            (if !msg.is_null() {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }),
        ) == -(1 as libc::c_int) as libc::c_long
            && errno == EINTR
        {}
    };
}
/* NOTE: the memory used to return msg is leaked: use msgs in events only
to describe fatal errors. */
#[c2rust::src_loc = "1323:1"]
unsafe extern "C" fn read_event(
    mut fd: libc::c_int,
    mut evp: *mut event_desc,
    mut msg: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf = 0 as *mut libc::c_char;
    if read_write(
        fd,
        evp as *mut libc::c_uchar,
        ::std::mem::size_of::<event_desc>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    *msg = 0 as *mut libc::c_char;
    if (*evp).msg_sz != 0 as libc::c_int
        && {
            buf = malloc(((*evp).msg_sz + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
            !buf.is_null()
        }
        && read_write(
            fd,
            buf as *mut libc::c_uchar,
            (*evp).msg_sz,
            1 as libc::c_int,
        ) != 0
    {
        *buf.offset((*evp).msg_sz as isize) = 0 as libc::c_int as libc::c_char;
        *msg = buf
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "1343:1"]
unsafe extern "C" fn fatal_event(mut ev: *mut event_desc, mut msg: *mut libc::c_char) {
    errno = (*ev).data;
    match (*ev).event {
        EVENT_DIE => {
            exit(0 as libc::c_int);
        }
        EVENT_FORK_ERR => {
            die(
                b"cannot fork into background: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
                EC_MISC,
            );
        }
        EVENT_PIPE_ERR => {
            /* fall through */
            die(
                b"failed to create helper: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
                EC_MISC,
            );
        }
        EVENT_CAP_ERR => {
            /* fall through */
            die(
                b"setting capabilities failed: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
                EC_MISC,
            );
        }
        EVENT_USER_ERR => {
            /* fall through */
            die(
                b"failed to change user-id to %s: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_MISC,
            );
        }
        EVENT_GROUP_ERR => {
            /* fall through */
            die(
                b"failed to change group-id to %s: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_MISC,
            );
        }
        EVENT_PIDFILE => {
            /* fall through */
            die(
                b"failed to open pidfile %s: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_FILE,
            );
        }
        EVENT_LOG_ERR => {
            /* fall through */
            die(
                b"cannot open log %s: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_FILE,
            );
        }
        EVENT_LUA_ERR => {
            /* fall through */
            die(
                b"failed to load Lua script: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_MISC,
            );
        }
        EVENT_TFTP_ERR => {
            /* fall through */
            die(
                b"TFTP directory %s inaccessible: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_FILE,
            );
        }
        EVENT_TIME_ERR => {
            /* fall through */
            die(
                b"cannot create timestamp file %s: %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
                EC_BADCONF,
            );
        }
        _ => {}
    };
}
#[c2rust::src_loc = "1393:1"]
unsafe extern "C" fn async_event(mut pipe_0: libc::c_int, mut now: time_t) {
    let mut p: pid_t = 0;
    let mut ev = event_desc {
        event: 0,
        data: 0,
        msg_sz: 0,
    };
    let mut i: libc::c_int = 0;
    let mut check = 0 as libc::c_int;
    let mut msg = 0 as *mut libc::c_char;
    /* NOTE: the memory used to return msg is leaked: use msgs in events only
    to describe fatal errors. */
    if read_event(pipe_0, &mut ev, &mut msg) != 0 {
        let mut current_block_60: u64; /* Bump zone serial, as it may have changed. */
        match ev.event {
            EVENT_RELOAD => {
                (*dnsmasq_daemon).soa_sn = (*dnsmasq_daemon).soa_sn.wrapping_add(1);
                current_block_60 = 11195962526119371365;
            }
            EVENT_INIT => {
                current_block_60 = 11195962526119371365;
            }
            EVENT_DUMP => {
                if (*dnsmasq_daemon).port != 0 as libc::c_int {
                    dump_cache(now);
                }
                current_block_60 = 6367734732029634840;
            }
            EVENT_ALARM => {
                if !(*dnsmasq_daemon).dhcp.is_null() || (*dnsmasq_daemon).doing_dhcp6 != 0 {
                    lease_prune(0 as *mut dhcp_lease, now);
                    lease_update_file(now);
                } else if (*dnsmasq_daemon).doing_ra != 0 {
                    /* Not doing DHCP, so no lease system, manage alarms for ra only */
                    send_alarm(periodic_ra(now), now);
                }
                current_block_60 = 6367734732029634840;
            }
            EVENT_CHILD => {
                loop
                /* See Stevens 5.10 */
                {
                    p = waitpid(-(1 as libc::c_int), 0 as *mut libc::c_int, WNOHANG);
                    if !(p != 0 as libc::c_int) {
                        break;
                    }
                    if p == -(1 as libc::c_int) {
                        if errno != EINTR {
                            break;
                        }
                    } else {
                        i = 0 as libc::c_int;
                        while i < MAX_PROCS {
                            if (*dnsmasq_daemon).tcp_pids[i as usize] == p {
                                (*dnsmasq_daemon).tcp_pids[i as usize] = 0 as libc::c_int
                            }
                            i += 1
                        }
                    }
                }
                current_block_60 = 6367734732029634840;
            }
            EVENT_KILLED => {
                my_syslog(
                    LOG_WARNING,
                    b"script process killed by signal %d\x00" as *const u8 as *const libc::c_char,
                    ev.data,
                );
                current_block_60 = 6367734732029634840;
            }
            EVENT_EXITED => {
                my_syslog(
                    LOG_WARNING,
                    b"script process exited with status %d\x00" as *const u8 as *const libc::c_char,
                    ev.data,
                );
                current_block_60 = 6367734732029634840;
            }
            EVENT_EXEC_ERR => {
                my_syslog(
                    LOG_ERR,
                    b"failed to execute %s: %s\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).lease_change_command,
                    strerror(ev.data),
                );
                current_block_60 = 6367734732029634840;
            }
            EVENT_SCRIPT_LOG => {
                my_syslog(
                    MS_SCRIPT | LOG_DEBUG,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    if !msg.is_null() {
                        msg as *const libc::c_char
                    } else {
                        b"\x00" as *const u8 as *const libc::c_char
                    },
                );
                free(msg as *mut libc::c_void);
                msg = 0 as *mut libc::c_char;
                current_block_60 = 6367734732029634840;
            }
            EVENT_USER_ERR | EVENT_DIE | EVENT_LUA_ERR => {
                /* necessary for fatal errors in helper */
                fatal_event(&mut ev, msg);
                current_block_60 = 6367734732029634840;
            }
            EVENT_REOPEN => {
                /* Note: this may leave TCP-handling processes with the old file still open.
                Since any such process will die in CHILD_LIFETIME or probably much sooner,
                we leave them logging to the old file. */
                if !(*dnsmasq_daemon).log_file.is_null() {
                    log_reopen((*dnsmasq_daemon).log_file);
                }
                current_block_60 = 6367734732029634840;
            }
            EVENT_NEWADDR => {
                newaddress(now);
                current_block_60 = 6367734732029634840;
            }
            EVENT_NEWROUTE => {
                resend_query();
                /* Force re-reading resolv file right now, for luck. */
                poll_resolv(0 as libc::c_int, 1 as libc::c_int, now);
                current_block_60 = 6367734732029634840;
            }
            EVENT_TERM => {
                /* Knock all our children on the head. */
                i = 0 as libc::c_int;
                while i < MAX_PROCS {
                    if (*dnsmasq_daemon).tcp_pids[i as usize] != 0 as libc::c_int {
                        kill((*dnsmasq_daemon).tcp_pids[i as usize], SIGALRM);
                    }
                    i += 1
                }
                /* handle pending lease transitions */
                if (*dnsmasq_daemon).helperfd != -(1 as libc::c_int) {
                    /* block in writes until all done */
                    i = fcntl((*dnsmasq_daemon).helperfd, F_GETFL);
                    if i != -(1 as libc::c_int) {
                        fcntl((*dnsmasq_daemon).helperfd, F_SETFL, i & !O_NONBLOCK);
                    }
                    loop {
                        helper_write();
                        if !(helper_buf_empty() == 0 || do_script_run(now) != 0) {
                            break;
                        }
                    }
                    close((*dnsmasq_daemon).helperfd);
                }
                if !(*dnsmasq_daemon).lease_stream.is_null() {
                    fclose((*dnsmasq_daemon).lease_stream);
                }
                if !(*dnsmasq_daemon).runfile.is_null() {
                    unlink((*dnsmasq_daemon).runfile);
                }
                if (*dnsmasq_daemon).dumpfd != -(1 as libc::c_int) {
                    close((*dnsmasq_daemon).dumpfd);
                }
                my_syslog(
                    LOG_INFO,
                    b"exiting on receipt of SIGTERM\x00" as *const u8 as *const libc::c_char,
                );
                flush_log();
                exit(EC_GOOD);
            }
            EVENT_TIME | _ => {
                current_block_60 = 6367734732029634840;
            }
        }
        match current_block_60 {
            11195962526119371365 =>
            /* fall through */
            {
                clear_cache_and_reload(now);
                if (*dnsmasq_daemon).port != 0 as libc::c_int {
                    if !(*dnsmasq_daemon).resolv_files.is_null()
                        && (*dnsmasq_daemon).options[(5 as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_uint)
                                << (5 as libc::c_int as libc::c_ulong).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                )
                            != 0
                    {
                        reload_servers((*(*dnsmasq_daemon).resolv_files).name);
                        check = 1 as libc::c_int
                    }
                    if !(*dnsmasq_daemon).servers_file.is_null() {
                        read_servers_file();
                        check = 1 as libc::c_int
                    }
                    if check != 0 {
                        check_servers();
                    }
                }
                rerun_scripts();
            }
            _ => {}
        }
    };
}
#[c2rust::src_loc = "1574:1"]
unsafe extern "C" fn poll_resolv(
    mut force: libc::c_int,
    mut do_reload: libc::c_int,
    mut now: time_t,
) {
    let mut res = 0 as *mut resolvc;
    let mut latest = 0 as *mut resolvc;
    let mut statbuf = stat {
        st_dev: 0,
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut last_change = 0 as libc::c_int as time_t;
    /* There may be more than one possible file.
    Go through and find the one which changed _last_.
    Warn of any which can't be read. */
    if (*dnsmasq_daemon).port == 0 as libc::c_int
        || (*dnsmasq_daemon).options[(5 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (5 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
    {
        return;
    }
    latest = 0 as *mut resolvc;
    res = (*dnsmasq_daemon).resolv_files;
    while !res.is_null() {
        if stat((*res).name, &mut statbuf) == -(1 as libc::c_int) {
            if force != 0 {
                (*res).mtime = 0 as libc::c_int as time_t
            } else {
                if (*res).logged == 0 {
                    my_syslog(
                        LOG_WARNING,
                        b"failed to access %s: %s\x00" as *const u8 as *const libc::c_char,
                        (*res).name,
                        strerror(errno),
                    );
                }
                (*res).logged = 1 as libc::c_int;
                if (*res).mtime != 0 as libc::c_int as libc::c_long {
                    /* existing file evaporated, force selection of the latest
                    file even if its mtime hasn't changed since we last looked */
                    poll_resolv(1 as libc::c_int, do_reload, now);
                    return;
                }
            }
        } else {
            (*res).logged = 0 as libc::c_int;
            if force != 0 || statbuf.st_mtim.tv_sec != (*res).mtime {
                (*res).mtime = statbuf.st_mtim.tv_sec;
                if difftime(statbuf.st_mtim.tv_sec, last_change) > 0.0f64 {
                    last_change = statbuf.st_mtim.tv_sec;
                    latest = res
                }
            }
        }
        res = (*res).next
    }
    if !latest.is_null() {
        static mut warned: libc::c_int = 0 as libc::c_int;
        if reload_servers((*latest).name) != 0 {
            my_syslog(
                LOG_INFO,
                b"reading %s\x00" as *const u8 as *const libc::c_char,
                (*latest).name,
            );
            warned = 0 as libc::c_int;
            check_servers();
            if (*dnsmasq_daemon).options[(24 as libc::c_int as libc::c_ulong).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
                & (1 as libc::c_uint)
                    << (24 as libc::c_int as libc::c_ulong).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                != 0
                && do_reload != 0
            {
                clear_cache_and_reload(now);
            }
        } else {
            (*latest).mtime = 0 as libc::c_int as time_t;
            if warned == 0 {
                my_syslog(
                    LOG_WARNING,
                    b"no servers found in %s, will retry\x00" as *const u8 as *const libc::c_char,
                    (*latest).name,
                );
                warned = 1 as libc::c_int
            }
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1644:1"]
pub unsafe extern "C" fn clear_cache_and_reload(mut now: time_t) {
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        cache_reload();
    }
    if !(*dnsmasq_daemon).dhcp.is_null() || (*dnsmasq_daemon).doing_dhcp6 != 0 {
        if (*dnsmasq_daemon).options[(14 as libc::c_int as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
            & (1 as libc::c_uint)
                << (14 as libc::c_int as libc::c_ulong).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
            != 0
        {
            dhcp_read_ethers();
        }
        reread_dhcp();
        dhcp_update_configs((*dnsmasq_daemon).dhcp_conf);
        lease_update_from_configs();
        lease_update_file(now);
        lease_update_dns(1 as libc::c_int);
    } else if (*dnsmasq_daemon).doing_ra != 0 {
        /* Not doing DHCP, so no lease system, manage
        alarms for ra only */
        send_alarm(periodic_ra(now), now);
    };
}
#[c2rust::src_loc = "1671:1"]
unsafe extern "C" fn set_dns_listeners(mut now: time_t) -> libc::c_int {
    let mut serverfdp = 0 as *mut serverfd;
    let mut listener = 0 as *mut listener;
    let mut wait = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut tftp = 0 as libc::c_int;
    let mut transfer = 0 as *mut tftp_transfer;
    if (*dnsmasq_daemon).options[(60 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (60 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        transfer = (*dnsmasq_daemon).tftp_trans;
        while !transfer.is_null() {
            tftp += 1;
            poll_listen((*transfer).sockfd, POLLIN as libc::c_short);
            transfer = (*transfer).next
        }
    }
    /* will we be able to get memory? */
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        get_new_frec(now, &mut wait, 0 as *mut frec);
    }
    serverfdp = (*dnsmasq_daemon).sfds;
    while !serverfdp.is_null() {
        poll_listen((*serverfdp).fd, POLLIN as libc::c_short);
        serverfdp = (*serverfdp).next
    }
    if (*dnsmasq_daemon).port != 0 as libc::c_int && (*dnsmasq_daemon).osport == 0 {
        i = 0 as libc::c_int;
        while i < RANDOM_SOCKS {
            if (*dnsmasq_daemon).randomsocks[i as usize].refcount as libc::c_int != 0 as libc::c_int
            {
                poll_listen(
                    (*dnsmasq_daemon).randomsocks[i as usize].fd,
                    POLLIN as libc::c_short,
                );
            }
            i += 1
        }
    }
    listener = (*dnsmasq_daemon).listeners;
    while !listener.is_null() {
        /* only listen for queries if we have resources */
        if (*listener).fd != -(1 as libc::c_int) && wait == 0 as libc::c_int {
            poll_listen((*listener).fd, POLLIN as libc::c_short);
        }
        /* death of a child goes through the select loop, so
        we don't need to explicitly arrange to wake up here */
        if (*listener).tcpfd != -(1 as libc::c_int) {
            i = 0 as libc::c_int;
            while i < MAX_PROCS {
                if (*dnsmasq_daemon).tcp_pids[i as usize] == 0 as libc::c_int
                    && (*dnsmasq_daemon).tcp_pipes[i as usize] == -(1 as libc::c_int)
                {
                    poll_listen((*listener).tcpfd, POLLIN as libc::c_short);
                    break;
                } else {
                    i += 1
                }
            }
        }
        /* tftp == 0 in single-port mode. */
        if tftp <= (*dnsmasq_daemon).tftp_max && (*listener).tftpfd != -(1 as libc::c_int) {
            poll_listen((*listener).tftpfd, POLLIN as libc::c_short);
        }
        listener = (*listener).next
    }
    if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        i = 0 as libc::c_int;
        while i < MAX_PROCS {
            if (*dnsmasq_daemon).tcp_pipes[i as usize] != -(1 as libc::c_int) {
                poll_listen(
                    (*dnsmasq_daemon).tcp_pipes[i as usize],
                    POLLIN as libc::c_short,
                );
            }
            i += 1
        }
    }
    return wait;
}
#[c2rust::src_loc = "1732:1"]
unsafe extern "C" fn check_dns_listeners(mut now: time_t) {
    let mut serverfdp = 0 as *mut serverfd;
    let mut listener = 0 as *mut listener;
    let mut i: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    serverfdp = (*dnsmasq_daemon).sfds;
    while !serverfdp.is_null() {
        if poll_check((*serverfdp).fd, POLLIN as libc::c_short) != 0 {
            reply_query(
                (*serverfdp).fd,
                (*serverfdp).source_addr.sa.sa_family as libc::c_int,
                now,
            );
        }
        serverfdp = (*serverfdp).next
    }
    if (*dnsmasq_daemon).port != 0 as libc::c_int && (*dnsmasq_daemon).osport == 0 {
        i = 0 as libc::c_int;
        while i < RANDOM_SOCKS {
            if (*dnsmasq_daemon).randomsocks[i as usize].refcount as libc::c_int != 0 as libc::c_int
                && poll_check(
                    (*dnsmasq_daemon).randomsocks[i as usize].fd,
                    POLLIN as libc::c_short,
                ) != 0
            {
                reply_query(
                    (*dnsmasq_daemon).randomsocks[i as usize].fd,
                    (*dnsmasq_daemon).randomsocks[i as usize].family as libc::c_int,
                    now,
                );
            }
            i += 1
        }
    }
    /* Races. The child process can die before we read all of the data from the
    pipe, or vice versa. Therefore send tcp_pids to zero when we wait() the
    process, and tcp_pipes to -1 and close the FD when we read the last
    of the data - indicated by cache_recv_insert returning zero.
    The order of these events is indeterminate, and both are needed
    to free the process slot. Once the child process has gone, poll()
    returns POLLHUP, not POLLIN, so have to check for both here. */
    if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    ) as usize]
        & (1 as libc::c_uint)
            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
        == 0
    {
        i = 0 as libc::c_int;
        while i < MAX_PROCS {
            if (*dnsmasq_daemon).tcp_pipes[i as usize] != -(1 as libc::c_int)
                && poll_check(
                    (*dnsmasq_daemon).tcp_pipes[i as usize],
                    (POLLIN | POLLHUP) as libc::c_short,
                ) != 0
                && cache_recv_insert(now, (*dnsmasq_daemon).tcp_pipes[i as usize]) == 0
            {
                close((*dnsmasq_daemon).tcp_pipes[i as usize]);
                (*dnsmasq_daemon).tcp_pipes[i as usize] = -(1 as libc::c_int)
            }
            i += 1
        }
    }
    listener = (*dnsmasq_daemon).listeners;
    while !listener.is_null() {
        if (*listener).fd != -(1 as libc::c_int)
            && poll_check((*listener).fd, POLLIN as libc::c_short) != 0
        {
            receive_query(listener, now);
        }
        if (*listener).tftpfd != -(1 as libc::c_int)
            && poll_check((*listener).tftpfd, POLLIN as libc::c_short) != 0
        {
            tftp_request(listener, now);
        }
        if (*listener).tcpfd != -(1 as libc::c_int)
            && poll_check((*listener).tcpfd, POLLIN as libc::c_short) != 0
        {
            let mut confd: libc::c_int = 0;
            let mut client_ok = 1 as libc::c_int;
            let mut iface = 0 as *mut irec;
            let mut p: pid_t = 0;
            let mut tcp_addr = mysockaddr {
                sa: sockaddr {
                    sa_family: 0,
                    sa_data: [0; 14],
                },
            };
            let mut tcp_len = ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as socklen_t;
            loop {
                confd = accept(
                    (*listener).tcpfd,
                    __SOCKADDR_ARG {
                        __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
                    },
                    0 as *mut socklen_t,
                );
                if !(confd == -(1 as libc::c_int) && errno == EINTR) {
                    break;
                }
            }
            if !(confd == -(1 as libc::c_int)) {
                if getsockname(
                    confd,
                    __SOCKADDR_ARG {
                        __sockaddr__: &mut tcp_addr as *mut mysockaddr as *mut sockaddr,
                    },
                    &mut tcp_len,
                ) == -(1 as libc::c_int)
                {
                    close(confd);
                } else {
                    /* Make sure that the interface list is up-to-date.

                       We do this here as we may need the results below, and
                       the DNS code needs them for --interface-name stuff.

                       Multiple calls to enumerate_interfaces() per select loop are
                       inhibited, so calls to it in the child process (which doesn't select())
                       have no effect. This avoids two processes reading from the same
                       netlink fd and screwing the pooch entirely.
                    */
                    enumerate_interfaces(0 as libc::c_int); /* May be NULL */
                    if (*dnsmasq_daemon).options[(13 as libc::c_int as libc::c_ulong).wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                        & (1 as libc::c_uint)
                            << (13 as libc::c_int as libc::c_ulong).wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            )
                        != 0
                    {
                        iface = (*listener).iface
                    } else {
                        let mut if_index: libc::c_int = 0;
                        let mut intr_name: [libc::c_char; 16] = [0; 16];
                        /* if we can find the arrival interface, check it's one that's allowed */
                        if_index = tcp_interface(confd, tcp_addr.sa.sa_family as libc::c_int); /* May be NULL */
                        if if_index != 0 as libc::c_int
                            && indextoname((*listener).tcpfd, if_index, intr_name.as_mut_ptr()) != 0
                        {
                            let mut addr = all_addr {
                                addr4: in_addr { s_addr: 0 },
                            };
                            if tcp_addr.sa.sa_family as libc::c_int == AF_INET6 {
                                addr.addr6 = tcp_addr.in6.sin6_addr
                            } else {
                                addr.addr4 = tcp_addr.in_0.sin_addr
                            }
                            iface = (*dnsmasq_daemon).interfaces;
                            while !iface.is_null() {
                                if (*iface).index == if_index
                                    && (*iface).addr.sa.sa_family as libc::c_int
                                        == tcp_addr.sa.sa_family as libc::c_int
                                {
                                    break;
                                }
                                iface = (*iface).next
                            }
                            if iface.is_null()
                                && loopback_exception(
                                    (*listener).tcpfd,
                                    tcp_addr.sa.sa_family as libc::c_int,
                                    &mut addr,
                                    intr_name.as_mut_ptr(),
                                ) == 0
                            {
                                client_ok = 0 as libc::c_int
                            }
                        }
                        if (*dnsmasq_daemon).options[(39 as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_uint)
                                << (39 as libc::c_int as libc::c_ulong).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                )
                            != 0
                        {
                            iface = (*listener).iface
                        } else {
                            /* Check for allowed interfaces when binding the wildcard address:
                            we do this by looking for an interface with the same address as
                            the local address of the TCP connection, then looking to see if that's
                            an allowed interface. As a side effect, we get the netmask of the
                            interface too, for localisation. */
                            iface = (*dnsmasq_daemon).interfaces; /* parent needs read pipe end. */
                            while !iface.is_null() {
                                if sockaddr_isequal(&mut (*iface).addr, &mut tcp_addr) != 0 {
                                    break;
                                }
                                iface = (*iface).next
                            }
                            if iface.is_null() {
                                client_ok = 0 as libc::c_int
                            }
                        }
                    }
                    if client_ok == 0 {
                        shutdown(confd, SHUT_RDWR_0);
                        close(confd);
                    } else if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_uint)
                            << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            )
                        == 0
                        && pipe(pipefd.as_mut_ptr()) == 0 as libc::c_int
                        && {
                            p = fork();
                            (p) != 0 as libc::c_int
                        }
                    {
                        close(pipefd[1 as libc::c_int as usize]);
                        if p == -(1 as libc::c_int) {
                            close(pipefd[0 as libc::c_int as usize]);
                        } else {
                            let mut i_0: libc::c_int = 0;
                            /* The child process inherits the netlink socket,
                            which it never uses, but when the parent (us)
                            uses it in the future, the answer may go to the
                            child, resulting in the parent blocking
                            forever awaiting the result. To avoid this
                            the child closes the netlink socket, but there's
                            a nasty race, since the parent may use netlink
                            before the child has done the close.

                            To avoid this, the parent blocks here until a
                            single byte comes back up the pipe, which
                            is sent by the child after it has closed the
                            netlink socket. */
                            let mut a: libc::c_uchar = 0;
                            read_write(
                                pipefd[0 as libc::c_int as usize],
                                &mut a,
                                1 as libc::c_int,
                                1 as libc::c_int,
                            );
                            i_0 = 0 as libc::c_int;
                            while i_0 < MAX_PROCS {
                                if (*dnsmasq_daemon).tcp_pids[i_0 as usize] == 0 as libc::c_int
                                    && (*dnsmasq_daemon).tcp_pipes[i_0 as usize]
                                        == -(1 as libc::c_int)
                                {
                                    (*dnsmasq_daemon).tcp_pids[i_0 as usize] = p;
                                    (*dnsmasq_daemon).tcp_pipes[i_0 as usize] =
                                        pipefd[0 as libc::c_int as usize];
                                    break;
                                } else {
                                    i_0 += 1
                                }
                            }
                        }
                        close(confd);
                        /* The child can use up to TCP_MAX_QUERIES ids, so skip that many. */
                        (*dnsmasq_daemon).log_id += TCP_MAX_QUERIES
                    } else {
                        let mut buff = 0 as *mut libc::c_uchar;
                        let mut s = 0 as *mut server;
                        let mut flags: libc::c_int = 0;
                        let mut netmask = in_addr { s_addr: 0 };
                        let mut auth_dns: libc::c_int = 0;
                        if !iface.is_null() {
                            netmask = (*iface).netmask;
                            auth_dns = (*iface).dns_auth
                        } else {
                            netmask.s_addr = 0 as libc::c_int as in_addr_t;
                            auth_dns = 0 as libc::c_int
                        }
                        /* Arrange for SIGALRM after CHILD_LIFETIME seconds to
                        terminate the process. */
                        if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_uint)
                                << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                )
                            == 0
                        {
                            /* See comment above re: netlink socket. */
                            let mut a_0 = 0 as libc::c_int as libc::c_uchar; /* close read end in child. */
                            close((*dnsmasq_daemon).netlinkfd);
                            read_write(
                                pipefd[1 as libc::c_int as usize],
                                &mut a_0,
                                1 as libc::c_int,
                                0 as libc::c_int,
                            );
                            alarm(CHILD_LIFETIME as libc::c_uint);
                            close(pipefd[0 as libc::c_int as usize]);
                            (*dnsmasq_daemon).pipe_to_parent = pipefd[1 as libc::c_int as usize]
                        }
                        /* start with no upstream connections. */
                        s = (*dnsmasq_daemon).servers;
                        while !s.is_null() {
                            (*s).tcpfd = -(1 as libc::c_int);
                            s = (*s).next
                        }
                        /* The connected socket inherits non-blocking
                        attribute from the listening socket.
                        Reset that here. */
                        flags = fcntl(confd, F_GETFL, 0 as libc::c_int);
                        if flags != -(1 as libc::c_int) {
                            fcntl(confd, F_SETFL, flags & !O_NONBLOCK);
                        }
                        buff = tcp_request(confd, now, &mut tcp_addr, netmask, auth_dns);
                        shutdown(confd, SHUT_RDWR_0);
                        close(confd);
                        if !buff.is_null() {
                            free(buff as *mut libc::c_void);
                        }
                        s = (*dnsmasq_daemon).servers;
                        while !s.is_null() {
                            if (*s).tcpfd != -(1 as libc::c_int) {
                                shutdown((*s).tcpfd, SHUT_RDWR_0);
                                close((*s).tcpfd);
                            }
                            s = (*s).next
                        }
                        if (*dnsmasq_daemon).options[(6 as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_uint)
                                << (6 as libc::c_int as libc::c_ulong).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                )
                            == 0
                        {
                            close((*dnsmasq_daemon).pipe_to_parent);
                            flush_log();
                            _exit(0 as libc::c_int);
                        }
                    }
                }
            }
        }
        listener = (*listener).next
    }
}
#[no_mangle]
#[c2rust::src_loc = "1971:1"]
pub unsafe extern "C" fn make_icmp_sock() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut zeroopt = 0 as libc::c_int;
    fd = socket(AF_INET, SOCK_RAW_0, IPPROTO_ICMP_0);
    if fd != -(1 as libc::c_int) {
        if fix_fd(fd) == 0
            || setsockopt(
                fd,
                SOL_SOCKET,
                SO_DONTROUTE,
                &mut zeroopt as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            ) == -(1 as libc::c_int)
        {
            close(fd);
            fd = -(1 as libc::c_int)
        }
    }
    return fd;
}
#[no_mangle]
#[c2rust::src_loc = "1989:1"]
pub unsafe extern "C" fn icmp_ping(mut addr: in_addr) -> libc::c_int {
    /* Try and get an ICMP echo from a machine. */
    let mut fd: libc::c_int = 0;
    let mut saddr = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut packet = C2RustUnnamed_27 {
        ip: ip {
            ip_hl_ip_v: [0; 1],
            ip_tos: 0,
            ip_len: 0,
            ip_id: 0,
            ip_off: 0,
            ip_ttl: 0,
            ip_p: 0,
            ip_sum: 0,
            ip_src: in_addr { s_addr: 0 },
            ip_dst: in_addr { s_addr: 0 },
        },
        icmp: icmp {
            icmp_type: 0,
            icmp_code: 0,
            icmp_cksum: 0,
            icmp_hun: C2RustUnnamed_17 { ih_pptr: 0 },
            icmp_dun: C2RustUnnamed_14 {
                id_ts: C2RustUnnamed_16 {
                    its_otime: 0,
                    its_rtime: 0,
                    its_ttime: 0,
                },
            },
        },
    };
    let mut id = rand16();
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut gotreply = 0 as libc::c_int;
    fd = make_icmp_sock();
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    saddr.sin_family = AF_INET as sa_family_t;
    saddr.sin_port = 0 as libc::c_int as in_port_t;
    saddr.sin_addr = addr;
    memset(
        &mut packet.icmp as *mut icmp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<icmp>() as libc::c_ulong,
    );
    packet.icmp.icmp_type = ICMP_ECHO as uint8_t;
    packet.icmp.icmp_hun.ih_idseq.icd_id = id;
    j = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<icmp>() as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        j = j.wrapping_add(
            *(&mut packet.icmp as *mut icmp as *mut u16_0).offset(i as isize) as libc::c_uint,
        );
        i = i.wrapping_add(1)
    }
    while j >> 16 as libc::c_int != 0 {
        j = (j & 0xffff as libc::c_int as libc::c_uint).wrapping_add(j >> 16 as libc::c_int)
    }
    packet.icmp.icmp_cksum = if j == 0xffff as libc::c_int as libc::c_uint {
        j
    } else {
        !j
    } as uint16_t;
    while retry_send(sendto(
        fd,
        &mut packet.icmp as *mut icmp as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<icmp>() as libc::c_ulong,
        0 as libc::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut saddr as *mut sockaddr_in as *mut sockaddr,
        },
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    )) != 0
    {}
    gotreply = delay_dhcp(dnsmasq_time(), PING_WAIT, fd, addr.s_addr, id);
    close(fd);
    return gotreply;
}
#[no_mangle]
#[c2rust::src_loc = "2043:1"]
pub unsafe extern "C" fn delay_dhcp(
    mut start: time_t,
    mut sec: libc::c_int,
    mut fd: libc::c_int,
    mut addr: uint32_t,
    mut id: libc::c_ushort,
) -> libc::c_int {
    /* Delay processing DHCP packets for "sec" seconds counting from "start".
    If "fd" is not -1 it will stop waiting if an ICMP echo reply is received
    from "addr" with ICMP ID "id" and return 1 */
    /* Note that whilst waiting, we check for
    (and service) events on the DNS and TFTP  sockets, (so doing that
    better not use any resources our caller has in use...)
    but we remain deaf to signals or further DHCP packets. */
    /* There can be a problem using dnsmasq_time() to end the loop, since
    it's not monotonic, and can go backwards if the system clock is
    tweaked, leading to the code getting stuck in this loop and
    ignoring DHCP requests. To fix this, we check to see if select returned
    as a result of a timeout rather than a socket becoming available. We
    only allow this to happen as many times as it takes to get to the wait time
    in quarter-second chunks. This provides a fallback way to end loop. */
    let mut rc: libc::c_int = 0;
    let mut timeout_count: libc::c_int = 0;
    let mut now: time_t = 0;
    now = dnsmasq_time();
    timeout_count = 0 as libc::c_int;
    while difftime(now, start) <= sec as libc::c_float as libc::c_double
        && timeout_count < sec * 4 as libc::c_int
    {
        poll_reset();
        if fd != -(1 as libc::c_int) {
            poll_listen(fd, POLLIN as libc::c_short);
        }
        set_dns_listeners(now);
        set_log_writer();
        if (*dnsmasq_daemon).doing_ra != 0 {
            poll_listen((*dnsmasq_daemon).icmp6fd, POLLIN as libc::c_short);
        }
        rc = do_poll(250 as libc::c_int);
        if rc < 0 as libc::c_int {
            continue;
        }
        if rc == 0 as libc::c_int {
            timeout_count += 1
        }
        now = dnsmasq_time();
        check_log_writer(0 as libc::c_int);
        check_dns_listeners(now);
        if (*dnsmasq_daemon).doing_ra != 0
            && poll_check((*dnsmasq_daemon).icmp6fd, POLLIN as libc::c_short) != 0
        {
            icmp6_packet(now);
        }
        check_tftp_listeners(now);
        if fd != -(1 as libc::c_int) {
            let mut packet = C2RustUnnamed_26 {
                ip: ip {
                    ip_hl_ip_v: [0; 1],
                    ip_tos: 0,
                    ip_len: 0,
                    ip_id: 0,
                    ip_off: 0,
                    ip_ttl: 0,
                    ip_p: 0,
                    ip_sum: 0,
                    ip_src: in_addr { s_addr: 0 },
                    ip_dst: in_addr { s_addr: 0 },
                },
                icmp: icmp {
                    icmp_type: 0,
                    icmp_code: 0,
                    icmp_cksum: 0,
                    icmp_hun: C2RustUnnamed_17 { ih_pptr: 0 },
                    icmp_dun: C2RustUnnamed_14 {
                        id_ts: C2RustUnnamed_16 {
                            its_otime: 0,
                            its_rtime: 0,
                            its_ttime: 0,
                        },
                    },
                },
            };
            let mut faddr = sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
            if poll_check(fd, POLLIN as libc::c_short) != 0
                && recvfrom(
                    fd,
                    &mut packet as *mut C2RustUnnamed_26 as *mut libc::c_void,
                    ::std::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong,
                    0 as libc::c_int,
                    __SOCKADDR_ARG {
                        __sockaddr__: &mut faddr as *mut sockaddr_in as *mut sockaddr,
                    },
                    &mut len,
                ) as libc::c_ulong
                    == ::std::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong
                && addr == faddr.sin_addr.s_addr
                && packet.icmp.icmp_type as libc::c_int == ICMP_ECHOREPLY
                && packet.icmp.icmp_hun.ih_idseq.icd_seq as libc::c_int == 0 as libc::c_int
                && packet.icmp.icmp_hun.ih_idseq.icd_id as libc::c_int == id as libc::c_int
            {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
/* HAVE_DHCP */
