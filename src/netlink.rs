#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
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
    #[c2rust::src_loc = "200:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "248:5"]
    pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
    #[c2rust::src_loc = "245:5"]
    pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
    #[c2rust::src_loc = "243:5"]
    pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
    #[c2rust::src_loc = "241:5"]
    pub const MSG_BATCH: C2RustUnnamed = 262144;
    #[c2rust::src_loc = "239:5"]
    pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
    #[c2rust::src_loc = "237:5"]
    pub const MSG_MORE: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "235:5"]
    pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "233:5"]
    pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "231:5"]
    pub const MSG_RST: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "229:5"]
    pub const MSG_CONFIRM: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "227:5"]
    pub const MSG_SYN: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "225:5"]
    pub const MSG_FIN: C2RustUnnamed = 512;
    #[c2rust::src_loc = "223:5"]
    pub const MSG_WAITALL: C2RustUnnamed = 256;
    #[c2rust::src_loc = "221:5"]
    pub const MSG_EOR: C2RustUnnamed = 128;
    #[c2rust::src_loc = "219:5"]
    pub const MSG_DONTWAIT: C2RustUnnamed = 64;
    #[c2rust::src_loc = "217:5"]
    pub const MSG_TRUNC: C2RustUnnamed = 32;
    #[c2rust::src_loc = "215:5"]
    pub const MSG_PROXY: C2RustUnnamed = 16;
    #[c2rust::src_loc = "213:5"]
    pub const MSG_CTRUNC: C2RustUnnamed = 8;
    #[c2rust::src_loc = "210:5"]
    pub const MSG_TRYHARD: C2RustUnnamed = 4;
    #[c2rust::src_loc = "206:5"]
    pub const MSG_DONTROUTE: C2RustUnnamed = 4;
    #[c2rust::src_loc = "204:5"]
    pub const MSG_PEEK: C2RustUnnamed = 2;
    #[c2rust::src_loc = "202:5"]
    pub const MSG_OOB: C2RustUnnamed = 1;
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
        } /* autobind */
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
    #[c2rust::src_loc = "156:9"]
    pub const SOL_NETLINK: libc::c_int = 270 as libc::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const PF_NETLINK: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "110:9"]
    pub const AF_NETLINK: libc::c_int = PF_NETLINK;
    #[c2rust::src_loc = "218:9"]
    pub const MSG_TRUNC_0: libc::c_int = MSG_TRUNC as libc::c_int;
    #[c2rust::src_loc = "205:9"]
    pub const MSG_PEEK_0: libc::c_int = MSG_PEEK as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const PF_LOCAL: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "93:9"]
    pub const AF_LOCAL: libc::c_int = PF_LOCAL;
    #[c2rust::src_loc = "41:9"]
    pub const PF_UNSPEC: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "92:9"]
    pub const AF_UNSPEC: libc::c_int = PF_UNSPEC;
    #[c2rust::src_loc = "53:9"]
    pub const PF_INET6: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "104:9"]
    pub const AF_INET6: libc::c_int = PF_INET6;
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
    #[c2rust::src_loc = "33:9"]
    pub const SOCK_RAW_0: libc::c_int = SOCK_RAW as libc::c_int;
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
        #[c2rust::src_loc = "152:1"]
        pub fn sendto(__fd: libc::c_int, __buf: *const libc::c_void,
                      __n: size_t, __flags: libc::c_int,
                      __addr: __CONST_SOCKADDR_ARG, __addr_len: socklen_t)
         -> ssize_t;
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
        pub cname: C2RustUnnamed_6,
        pub key: C2RustUnnamed_5,
        pub ds: C2RustUnnamed_4,
        pub srv: C2RustUnnamed_3,
        pub log: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "322:3"]
    pub struct C2RustUnnamed_2 {
        pub keytag: libc::c_ushort,
        pub algo: libc::c_ushort,
        pub digest: libc::c_ushort,
        pub rcode: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:3"]
    pub struct C2RustUnnamed_3 {
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
    pub struct C2RustUnnamed_4 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
        pub digest: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:3"]
    pub struct C2RustUnnamed_5 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub flags: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:3"]
    pub struct C2RustUnnamed_6 {
        pub target: C2RustUnnamed_7,
        pub uid: libc::c_uint,
        pub is_name_ptr: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:5"]
    pub union C2RustUnnamed_7 {
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
        pub name: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "455:3"]
    pub union C2RustUnnamed_8 {
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
        pub u: C2RustUnnamed_9,
        pub val: *mut libc::c_uchar,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_opt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "815:3"]
    pub union C2RustUnnamed_9 {
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
    #[c2rust::src_loc = "211:9"]
    pub const EC_MISC: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "199:9"]
    pub const EVENT_NEWADDR: libc::c_int = 22 as libc::c_int;
    #[c2rust::src_loc = "200:9"]
    pub const EVENT_NEWROUTE: libc::c_int = 23 as libc::c_int;
    #[c2rust::src_loc = "516:9"]
    pub const IFACE_PERMANENT: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "515:9"]
    pub const IFACE_DEPRECATED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "514:9"]
    pub const IFACE_TENTATIVE: libc::c_int = 1 as libc::c_int;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6};
    use super::time_t_h::time_t;
    use super::socket_h::sockaddr;
    use super::sys_types_h::{off_t, dev_t, ino_t, pid_t, ssize_t};
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1172:4"]
        pub static mut dnsmasq_daemon: *mut dnsmasq_daemon;
        #[no_mangle]
        #[c2rust::src_loc = "1288:1"]
        pub fn safe_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1302:1"]
        pub fn retry_send(rc: ssize_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1309:1"]
        pub fn expand_buf(iov_0: *mut iovec, size: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1320:1"]
        pub fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
                   exit_code: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char,
                         _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1468:1"]
        pub fn queue_event(event: libc::c_int);
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
#[c2rust::header_src = "/usr/include/asm-generic/int-ll64.h:17"]
pub mod int_ll64_h {
    #[c2rust::src_loc = "21:1"]
    pub type __u8 = libc::c_uchar;
    #[c2rust::src_loc = "24:1"]
    pub type __u16 = libc::c_ushort;
    #[c2rust::src_loc = "26:1"]
    pub type __s32 = libc::c_int;
    #[c2rust::src_loc = "27:1"]
    pub type __u32 = libc::c_uint;
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
    #[c2rust::src_loc = "53:10"]
    pub const IFF_POINTOPOINT_0: libc::c_int = IFF_POINTOPOINT as libc::c_int;
    #[c2rust::src_loc = "51:10"]
    pub const IFF_LOOPBACK_0: libc::c_int = IFF_LOOPBACK as libc::c_int;
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
#[c2rust::header_src = "/usr/include/linux/netlink.h:22"]
pub mod netlink_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:8"]
    pub struct sockaddr_nl {
        pub nl_family: __kernel_sa_family_t,
        pub nl_pad: libc::c_ushort,
        pub nl_pid: __u32,
        pub nl_groups: __u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct nlmsghdr {
        pub nlmsg_len: __u32,
        pub nlmsg_type: __u16,
        pub nlmsg_flags: __u16,
        pub nlmsg_seq: __u32,
        pub nlmsg_pid: __u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct nlmsgerr {
        pub error: libc::c_int,
        pub msg: nlmsghdr,
    }
    #[c2rust::src_loc = "9:9"]
    pub const NETLINK_ROUTE: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const NLM_F_REQUEST: libc::c_int = 0x1 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const NLM_F_ACK: libc::c_int = 0x4 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const NLM_F_ROOT: libc::c_int = 0x100 as libc::c_int;
    #[c2rust::src_loc = "63:9"]
    pub const NLM_F_MATCH: libc::c_int = 0x200 as libc::c_int;
    #[c2rust::src_loc = "89:9"]
    pub const NLMSG_ALIGNTO: libc::c_uint = 4 as libc::c_uint;
    #[c2rust::src_loc = "103:9"]
    pub const NLMSG_ERROR: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "104:9"]
    pub const NLMSG_DONE: libc::c_int = 0x3 as libc::c_int;
    #[c2rust::src_loc = "149:9"]
    pub const NETLINK_NO_ENOBUFS: libc::c_int = 5 as libc::c_int;
    use super::linux_socket_h::__kernel_sa_family_t;
    use super::int_ll64_h::{__u32, __u16};
}
#[c2rust::header_src = "/usr/include/linux/socket.h:22"]
pub mod linux_socket_h {
    #[c2rust::src_loc = "10:1"]
    pub type __kernel_sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/linux/rtnetlink.h:23"]
pub mod rtnetlink_h {
    #[c2rust::src_loc = "39:2"]
    pub const RTM_DELADDR: C2RustUnnamed_14 = 21;
    #[c2rust::src_loc = "37:2"]
    pub const RTM_NEWADDR: C2RustUnnamed_14 = 20;
    #[c2rust::src_loc = "314:2"]
    pub const RT_TABLE_LOCAL: rt_class_t = 255;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "207:8"]
    pub struct rtmsg {
        pub rtm_family: libc::c_uchar,
        pub rtm_dst_len: libc::c_uchar,
        pub rtm_src_len: libc::c_uchar,
        pub rtm_tos: libc::c_uchar,
        pub rtm_table: libc::c_uchar,
        pub rtm_protocol: libc::c_uchar,
        pub rtm_scope: libc::c_uchar,
        pub rtm_type: libc::c_uchar,
        pub rtm_flags: libc::c_uint,
    }
    #[c2rust::src_loc = "313:2"]
    pub const RT_TABLE_MAIN: rt_class_t = 254;
    #[c2rust::src_loc = "292:2"]
    pub const RT_SCOPE_LINK: rt_scope_t = 253;
    #[c2rust::src_loc = "225:2"]
    pub const RTN_UNICAST: C2RustUnnamed_15 = 1;
    #[c2rust::src_loc = "44:2"]
    pub const RTM_NEWROUTE: C2RustUnnamed_14 = 24;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "516:8"]
    pub struct ifinfomsg {
        pub ifi_family: libc::c_uchar,
        pub __ifi_pad: libc::c_uchar,
        pub ifi_type: libc::c_ushort,
        pub ifi_index: libc::c_int,
        pub ifi_flags: libc::c_uint,
        pub ifi_change: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "181:8"]
    pub struct rtattr {
        pub rta_len: libc::c_ushort,
        pub rta_type: libc::c_ushort,
    }
    #[c2rust::src_loc = "28:2"]
    pub const RTM_NEWLINK: C2RustUnnamed_14 = 16;
    #[c2rust::src_loc = "51:2"]
    pub const RTM_NEWNEIGH: C2RustUnnamed_14 = 28;
    #[c2rust::src_loc = "41:2"]
    pub const RTM_GETADDR: C2RustUnnamed_14 = 22;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "503:8"]
    pub struct rtgenmsg {
        pub rtgen_family: libc::c_uchar,
    }
    #[c2rust::src_loc = "32:2"]
    pub const RTM_GETLINK: C2RustUnnamed_14 = 18;
    #[c2rust::src_loc = "55:2"]
    pub const RTM_GETNEIGH: C2RustUnnamed_14 = 30;
    #[c2rust::src_loc = "24:1"]
    pub type C2RustUnnamed_14 = libc::c_uint;
    #[c2rust::src_loc = "167:2"]
    pub const __RTM_MAX: C2RustUnnamed_14 = 107;
    #[c2rust::src_loc = "164:2"]
    pub const RTM_GETNEXTHOP: C2RustUnnamed_14 = 106;
    #[c2rust::src_loc = "162:2"]
    pub const RTM_DELNEXTHOP: C2RustUnnamed_14 = 105;
    #[c2rust::src_loc = "160:2"]
    pub const RTM_NEWNEXTHOP: C2RustUnnamed_14 = 104;
    #[c2rust::src_loc = "157:2"]
    pub const RTM_GETCHAIN: C2RustUnnamed_14 = 102;
    #[c2rust::src_loc = "155:2"]
    pub const RTM_DELCHAIN: C2RustUnnamed_14 = 101;
    #[c2rust::src_loc = "153:2"]
    pub const RTM_NEWCHAIN: C2RustUnnamed_14 = 100;
    #[c2rust::src_loc = "150:2"]
    pub const RTM_NEWCACHEREPORT: C2RustUnnamed_14 = 96;
    #[c2rust::src_loc = "147:2"]
    pub const RTM_GETSTATS: C2RustUnnamed_14 = 94;
    #[c2rust::src_loc = "145:2"]
    pub const RTM_NEWSTATS: C2RustUnnamed_14 = 92;
    #[c2rust::src_loc = "142:2"]
    pub const RTM_GETNSID: C2RustUnnamed_14 = 90;
    #[c2rust::src_loc = "140:2"]
    pub const RTM_DELNSID: C2RustUnnamed_14 = 89;
    #[c2rust::src_loc = "138:2"]
    pub const RTM_NEWNSID: C2RustUnnamed_14 = 88;
    #[c2rust::src_loc = "135:2"]
    pub const RTM_GETMDB: C2RustUnnamed_14 = 86;
    #[c2rust::src_loc = "133:2"]
    pub const RTM_DELMDB: C2RustUnnamed_14 = 85;
    #[c2rust::src_loc = "131:2"]
    pub const RTM_NEWMDB: C2RustUnnamed_14 = 84;
    #[c2rust::src_loc = "128:2"]
    pub const RTM_GETNETCONF: C2RustUnnamed_14 = 82;
    #[c2rust::src_loc = "126:2"]
    pub const RTM_DELNETCONF: C2RustUnnamed_14 = 81;
    #[c2rust::src_loc = "124:2"]
    pub const RTM_NEWNETCONF: C2RustUnnamed_14 = 80;
    #[c2rust::src_loc = "121:2"]
    pub const RTM_SETDCB: C2RustUnnamed_14 = 79;
    #[c2rust::src_loc = "119:2"]
    pub const RTM_GETDCB: C2RustUnnamed_14 = 78;
    #[c2rust::src_loc = "116:2"]
    pub const RTM_GETADDRLABEL: C2RustUnnamed_14 = 74;
    #[c2rust::src_loc = "114:2"]
    pub const RTM_DELADDRLABEL: C2RustUnnamed_14 = 73;
    #[c2rust::src_loc = "112:2"]
    pub const RTM_NEWADDRLABEL: C2RustUnnamed_14 = 72;
    #[c2rust::src_loc = "109:2"]
    pub const RTM_NEWNDUSEROPT: C2RustUnnamed_14 = 68;
    #[c2rust::src_loc = "106:2"]
    pub const RTM_SETNEIGHTBL: C2RustUnnamed_14 = 67;
    #[c2rust::src_loc = "104:2"]
    pub const RTM_GETNEIGHTBL: C2RustUnnamed_14 = 66;
    #[c2rust::src_loc = "102:2"]
    pub const RTM_NEWNEIGHTBL: C2RustUnnamed_14 = 64;
    #[c2rust::src_loc = "99:2"]
    pub const RTM_GETANYCAST: C2RustUnnamed_14 = 62;
    #[c2rust::src_loc = "96:2"]
    pub const RTM_GETMULTICAST: C2RustUnnamed_14 = 58;
    #[c2rust::src_loc = "93:2"]
    pub const RTM_NEWPREFIX: C2RustUnnamed_14 = 52;
    #[c2rust::src_loc = "90:2"]
    pub const RTM_GETACTION: C2RustUnnamed_14 = 50;
    #[c2rust::src_loc = "88:2"]
    pub const RTM_DELACTION: C2RustUnnamed_14 = 49;
    #[c2rust::src_loc = "86:2"]
    pub const RTM_NEWACTION: C2RustUnnamed_14 = 48;
    #[c2rust::src_loc = "83:2"]
    pub const RTM_GETTFILTER: C2RustUnnamed_14 = 46;
    #[c2rust::src_loc = "81:2"]
    pub const RTM_DELTFILTER: C2RustUnnamed_14 = 45;
    #[c2rust::src_loc = "79:2"]
    pub const RTM_NEWTFILTER: C2RustUnnamed_14 = 44;
    #[c2rust::src_loc = "76:2"]
    pub const RTM_GETTCLASS: C2RustUnnamed_14 = 42;
    #[c2rust::src_loc = "74:2"]
    pub const RTM_DELTCLASS: C2RustUnnamed_14 = 41;
    #[c2rust::src_loc = "72:2"]
    pub const RTM_NEWTCLASS: C2RustUnnamed_14 = 40;
    #[c2rust::src_loc = "69:2"]
    pub const RTM_GETQDISC: C2RustUnnamed_14 = 38;
    #[c2rust::src_loc = "67:2"]
    pub const RTM_DELQDISC: C2RustUnnamed_14 = 37;
    #[c2rust::src_loc = "65:2"]
    pub const RTM_NEWQDISC: C2RustUnnamed_14 = 36;
    #[c2rust::src_loc = "62:2"]
    pub const RTM_GETRULE: C2RustUnnamed_14 = 34;
    #[c2rust::src_loc = "60:2"]
    pub const RTM_DELRULE: C2RustUnnamed_14 = 33;
    #[c2rust::src_loc = "58:2"]
    pub const RTM_NEWRULE: C2RustUnnamed_14 = 32;
    #[c2rust::src_loc = "53:2"]
    pub const RTM_DELNEIGH: C2RustUnnamed_14 = 29;
    #[c2rust::src_loc = "48:2"]
    pub const RTM_GETROUTE: C2RustUnnamed_14 = 26;
    #[c2rust::src_loc = "46:2"]
    pub const RTM_DELROUTE: C2RustUnnamed_14 = 25;
    #[c2rust::src_loc = "34:2"]
    pub const RTM_SETLINK: C2RustUnnamed_14 = 19;
    #[c2rust::src_loc = "30:2"]
    pub const RTM_DELLINK: C2RustUnnamed_14 = 17;
    #[c2rust::src_loc = "25:2"]
    pub const RTM_BASE: C2RustUnnamed_14 = 16;
    #[c2rust::src_loc = "223:1"]
    pub type C2RustUnnamed_15 = libc::c_uint;
    #[c2rust::src_loc = "238:2"]
    pub const __RTN_MAX: C2RustUnnamed_15 = 12;
    #[c2rust::src_loc = "237:2"]
    pub const RTN_XRESOLVE: C2RustUnnamed_15 = 11;
    #[c2rust::src_loc = "236:2"]
    pub const RTN_NAT: C2RustUnnamed_15 = 10;
    #[c2rust::src_loc = "235:2"]
    pub const RTN_THROW: C2RustUnnamed_15 = 9;
    #[c2rust::src_loc = "234:2"]
    pub const RTN_PROHIBIT: C2RustUnnamed_15 = 8;
    #[c2rust::src_loc = "233:2"]
    pub const RTN_UNREACHABLE: C2RustUnnamed_15 = 7;
    #[c2rust::src_loc = "232:2"]
    pub const RTN_BLACKHOLE: C2RustUnnamed_15 = 6;
    #[c2rust::src_loc = "231:2"]
    pub const RTN_MULTICAST: C2RustUnnamed_15 = 5;
    #[c2rust::src_loc = "229:2"]
    pub const RTN_ANYCAST: C2RustUnnamed_15 = 4;
    #[c2rust::src_loc = "227:2"]
    pub const RTN_BROADCAST: C2RustUnnamed_15 = 3;
    #[c2rust::src_loc = "226:2"]
    pub const RTN_LOCAL: C2RustUnnamed_15 = 2;
    #[c2rust::src_loc = "224:2"]
    pub const RTN_UNSPEC: C2RustUnnamed_15 = 0;
    #[c2rust::src_loc = "288:1"]
    pub type rt_scope_t = libc::c_uint;
    #[c2rust::src_loc = "294:2"]
    pub const RT_SCOPE_NOWHERE: rt_scope_t = 255;
    #[c2rust::src_loc = "293:2"]
    pub const RT_SCOPE_HOST: rt_scope_t = 254;
    #[c2rust::src_loc = "291:2"]
    pub const RT_SCOPE_SITE: rt_scope_t = 200;
    #[c2rust::src_loc = "289:2"]
    pub const RT_SCOPE_UNIVERSE: rt_scope_t = 0;
    #[c2rust::src_loc = "308:1"]
    pub type rt_class_t = libc::c_uint;
    #[c2rust::src_loc = "315:2"]
    pub const RT_TABLE_MAX: rt_class_t = 4294967295;
    #[c2rust::src_loc = "312:2"]
    pub const RT_TABLE_DEFAULT: rt_class_t = 253;
    #[c2rust::src_loc = "311:2"]
    pub const RT_TABLE_COMPAT: rt_class_t = 252;
    #[c2rust::src_loc = "309:2"]
    pub const RT_TABLE_UNSPEC: rt_class_t = 0;
    #[c2rust::src_loc = "52:9"]
    pub const RTM_NEWNEIGH_0: libc::c_int = RTM_NEWNEIGH as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const RTM_GETLINK_0: libc::c_int = RTM_GETLINK as libc::c_int;
    #[c2rust::src_loc = "29:9"]
    pub const RTM_NEWLINK_0: libc::c_int = RTM_NEWLINK as libc::c_int;
    #[c2rust::src_loc = "188:9"]
    pub const RTA_ALIGNTO: libc::c_uint = 4 as libc::c_uint;
    #[c2rust::src_loc = "45:9"]
    pub const RTM_NEWROUTE_0: libc::c_int = RTM_NEWROUTE as libc::c_int;
    #[c2rust::src_loc = "38:9"]
    pub const RTM_NEWADDR_0: libc::c_int = RTM_NEWADDR as libc::c_int;
    #[c2rust::src_loc = "40:9"]
    pub const RTM_DELADDR_0: libc::c_int = RTM_DELADDR as libc::c_int;
    #[c2rust::src_loc = "636:9"]
    pub const RTMGRP_IPV4_ROUTE: libc::c_int = 0x40 as libc::c_int;
    #[c2rust::src_loc = "634:9"]
    pub const RTMGRP_IPV4_IFADDR: libc::c_int = 0x10 as libc::c_int;
    #[c2rust::src_loc = "641:9"]
    pub const RTMGRP_IPV6_ROUTE: libc::c_int = 0x400 as libc::c_int;
    #[c2rust::src_loc = "639:9"]
    pub const RTMGRP_IPV6_IFADDR: libc::c_int = 0x100 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const RTM_GETNEIGH_0: libc::c_int = RTM_GETNEIGH as libc::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const RTM_GETADDR_0: libc::c_int = RTM_GETADDR as libc::c_int;
}
#[c2rust::header_src = "/usr/include/linux/if_link.h:23"]
pub mod if_link_h {
    #[c2rust::src_loc = "108:2"]
    pub const IFLA_ADDRESS: C2RustUnnamed_11 = 1;
    #[c2rust::src_loc = "106:1"]
    pub type C2RustUnnamed_11 = libc::c_uint;
    #[c2rust::src_loc = "170:2"]
    pub const __IFLA_MAX: C2RustUnnamed_11 = 52;
    #[c2rust::src_loc = "169:2"]
    pub const IFLA_MAX_MTU: C2RustUnnamed_11 = 51;
    #[c2rust::src_loc = "168:2"]
    pub const IFLA_MIN_MTU: C2RustUnnamed_11 = 50;
    #[c2rust::src_loc = "167:2"]
    pub const IFLA_NEW_IFINDEX: C2RustUnnamed_11 = 49;
    #[c2rust::src_loc = "166:2"]
    pub const IFLA_CARRIER_DOWN_COUNT: C2RustUnnamed_11 = 48;
    #[c2rust::src_loc = "165:2"]
    pub const IFLA_CARRIER_UP_COUNT: C2RustUnnamed_11 = 47;
    #[c2rust::src_loc = "164:2"]
    pub const IFLA_TARGET_NETNSID: C2RustUnnamed_11 = 46;
    #[c2rust::src_loc = "163:2"]
    pub const IFLA_IF_NETNSID: C2RustUnnamed_11 = 46;
    #[c2rust::src_loc = "162:2"]
    pub const IFLA_NEW_NETNSID: C2RustUnnamed_11 = 45;
    #[c2rust::src_loc = "161:2"]
    pub const IFLA_EVENT: C2RustUnnamed_11 = 44;
    #[c2rust::src_loc = "160:2"]
    pub const IFLA_XDP: C2RustUnnamed_11 = 43;
    #[c2rust::src_loc = "159:2"]
    pub const IFLA_PAD: C2RustUnnamed_11 = 42;
    #[c2rust::src_loc = "158:2"]
    pub const IFLA_GSO_MAX_SIZE: C2RustUnnamed_11 = 41;
    #[c2rust::src_loc = "157:2"]
    pub const IFLA_GSO_MAX_SEGS: C2RustUnnamed_11 = 40;
    #[c2rust::src_loc = "156:2"]
    pub const IFLA_PROTO_DOWN: C2RustUnnamed_11 = 39;
    #[c2rust::src_loc = "155:2"]
    pub const IFLA_PHYS_PORT_NAME: C2RustUnnamed_11 = 38;
    #[c2rust::src_loc = "154:2"]
    pub const IFLA_LINK_NETNSID: C2RustUnnamed_11 = 37;
    #[c2rust::src_loc = "153:2"]
    pub const IFLA_PHYS_SWITCH_ID: C2RustUnnamed_11 = 36;
    #[c2rust::src_loc = "152:2"]
    pub const IFLA_CARRIER_CHANGES: C2RustUnnamed_11 = 35;
    #[c2rust::src_loc = "151:2"]
    pub const IFLA_PHYS_PORT_ID: C2RustUnnamed_11 = 34;
    #[c2rust::src_loc = "150:2"]
    pub const IFLA_CARRIER: C2RustUnnamed_11 = 33;
    #[c2rust::src_loc = "149:2"]
    pub const IFLA_NUM_RX_QUEUES: C2RustUnnamed_11 = 32;
    #[c2rust::src_loc = "148:2"]
    pub const IFLA_NUM_TX_QUEUES: C2RustUnnamed_11 = 31;
    #[c2rust::src_loc = "146:2"]
    pub const IFLA_PROMISCUITY: C2RustUnnamed_11 = 30;
    #[c2rust::src_loc = "145:2"]
    pub const IFLA_EXT_MASK: C2RustUnnamed_11 = 29;
    #[c2rust::src_loc = "144:2"]
    pub const IFLA_NET_NS_FD: C2RustUnnamed_11 = 28;
    #[c2rust::src_loc = "143:2"]
    pub const IFLA_GROUP: C2RustUnnamed_11 = 27;
    #[c2rust::src_loc = "142:2"]
    pub const IFLA_AF_SPEC: C2RustUnnamed_11 = 26;
    #[c2rust::src_loc = "141:2"]
    pub const IFLA_PORT_SELF: C2RustUnnamed_11 = 25;
    #[c2rust::src_loc = "140:2"]
    pub const IFLA_VF_PORTS: C2RustUnnamed_11 = 24;
    #[c2rust::src_loc = "139:2"]
    pub const IFLA_STATS64: C2RustUnnamed_11 = 23;
    #[c2rust::src_loc = "138:2"]
    pub const IFLA_VFINFO_LIST: C2RustUnnamed_11 = 22;
    #[c2rust::src_loc = "137:2"]
    pub const IFLA_NUM_VF: C2RustUnnamed_11 = 21;
    #[c2rust::src_loc = "136:2"]
    pub const IFLA_IFALIAS: C2RustUnnamed_11 = 20;
    #[c2rust::src_loc = "135:2"]
    pub const IFLA_NET_NS_PID: C2RustUnnamed_11 = 19;
    #[c2rust::src_loc = "133:2"]
    pub const IFLA_LINKINFO: C2RustUnnamed_11 = 18;
    #[c2rust::src_loc = "132:2"]
    pub const IFLA_LINKMODE: C2RustUnnamed_11 = 17;
    #[c2rust::src_loc = "131:2"]
    pub const IFLA_OPERSTATE: C2RustUnnamed_11 = 16;
    #[c2rust::src_loc = "129:2"]
    pub const IFLA_WEIGHT: C2RustUnnamed_11 = 15;
    #[c2rust::src_loc = "127:2"]
    pub const IFLA_MAP: C2RustUnnamed_11 = 14;
    #[c2rust::src_loc = "125:2"]
    pub const IFLA_TXQLEN: C2RustUnnamed_11 = 13;
    #[c2rust::src_loc = "123:2"]
    pub const IFLA_PROTINFO: C2RustUnnamed_11 = 12;
    #[c2rust::src_loc = "121:2"]
    pub const IFLA_WIRELESS: C2RustUnnamed_11 = 11;
    #[c2rust::src_loc = "119:2"]
    pub const IFLA_MASTER: C2RustUnnamed_11 = 10;
    #[c2rust::src_loc = "117:2"]
    pub const IFLA_PRIORITY: C2RustUnnamed_11 = 9;
    #[c2rust::src_loc = "115:2"]
    pub const IFLA_COST: C2RustUnnamed_11 = 8;
    #[c2rust::src_loc = "114:2"]
    pub const IFLA_STATS: C2RustUnnamed_11 = 7;
    #[c2rust::src_loc = "113:2"]
    pub const IFLA_QDISC: C2RustUnnamed_11 = 6;
    #[c2rust::src_loc = "112:2"]
    pub const IFLA_LINK: C2RustUnnamed_11 = 5;
    #[c2rust::src_loc = "111:2"]
    pub const IFLA_MTU: C2RustUnnamed_11 = 4;
    #[c2rust::src_loc = "110:2"]
    pub const IFLA_IFNAME: C2RustUnnamed_11 = 3;
    #[c2rust::src_loc = "109:2"]
    pub const IFLA_BROADCAST: C2RustUnnamed_11 = 2;
    #[c2rust::src_loc = "107:2"]
    pub const IFLA_UNSPEC: C2RustUnnamed_11 = 0;
}
#[c2rust::header_src = "/usr/include/linux/neighbour.h:23"]
pub mod neighbour_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct ndmsg {
        pub ndm_family: __u8,
        pub ndm_pad1: __u8,
        pub ndm_pad2: __u16,
        pub ndm_ifindex: __s32,
        pub ndm_state: __u16,
        pub ndm_flags: __u8,
        pub ndm_type: __u8,
    }
    #[c2rust::src_loc = "21:2"]
    pub const NDA_LLADDR: C2RustUnnamed_13 = 2;
    #[c2rust::src_loc = "20:2"]
    pub const NDA_DST: C2RustUnnamed_13 = 1;
    #[c2rust::src_loc = "18:1"]
    pub type C2RustUnnamed_13 = libc::c_uint;
    #[c2rust::src_loc = "32:2"]
    pub const __NDA_MAX: C2RustUnnamed_13 = 13;
    #[c2rust::src_loc = "31:2"]
    pub const NDA_PROTOCOL: C2RustUnnamed_13 = 12;
    #[c2rust::src_loc = "30:2"]
    pub const NDA_SRC_VNI: C2RustUnnamed_13 = 11;
    #[c2rust::src_loc = "29:2"]
    pub const NDA_LINK_NETNSID: C2RustUnnamed_13 = 10;
    #[c2rust::src_loc = "28:2"]
    pub const NDA_MASTER: C2RustUnnamed_13 = 9;
    #[c2rust::src_loc = "27:2"]
    pub const NDA_IFINDEX: C2RustUnnamed_13 = 8;
    #[c2rust::src_loc = "26:2"]
    pub const NDA_VNI: C2RustUnnamed_13 = 7;
    #[c2rust::src_loc = "25:2"]
    pub const NDA_PORT: C2RustUnnamed_13 = 6;
    #[c2rust::src_loc = "24:2"]
    pub const NDA_VLAN: C2RustUnnamed_13 = 5;
    #[c2rust::src_loc = "23:2"]
    pub const NDA_PROBES: C2RustUnnamed_13 = 4;
    #[c2rust::src_loc = "22:2"]
    pub const NDA_CACHEINFO: C2RustUnnamed_13 = 3;
    #[c2rust::src_loc = "19:2"]
    pub const NDA_UNSPEC: C2RustUnnamed_13 = 0;
    #[c2rust::src_loc = "62:9"]
    pub const NUD_NOARP: libc::c_int = 0x40 as libc::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const NUD_INCOMPLETE: libc::c_int = 0x1 as libc::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const NUD_FAILED: libc::c_int = 0x20 as libc::c_int;
    use super::int_ll64_h::{__u8, __u16, __s32};
}
#[c2rust::header_src = "/usr/include/linux/if_addr.h:23"]
pub mod if_addr_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct ifaddrmsg {
        pub ifa_family: __u8,
        pub ifa_prefixlen: __u8,
        pub ifa_flags: __u8,
        pub ifa_scope: __u8,
        pub ifa_index: __u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:8"]
    pub struct ifa_cacheinfo {
        pub ifa_prefered: __u32,
        pub ifa_valid: __u32,
        pub cstamp: __u32,
        pub tstamp: __u32,
    }
    #[c2rust::src_loc = "33:2"]
    pub const IFA_CACHEINFO: C2RustUnnamed_12 = 6;
    #[c2rust::src_loc = "28:2"]
    pub const IFA_ADDRESS: C2RustUnnamed_12 = 1;
    #[c2rust::src_loc = "30:2"]
    pub const IFA_LABEL: C2RustUnnamed_12 = 3;
    #[c2rust::src_loc = "31:2"]
    pub const IFA_BROADCAST: C2RustUnnamed_12 = 4;
    #[c2rust::src_loc = "29:2"]
    pub const IFA_LOCAL: C2RustUnnamed_12 = 2;
    #[c2rust::src_loc = "26:1"]
    pub type C2RustUnnamed_12 = libc::c_uint;
    #[c2rust::src_loc = "38:2"]
    pub const __IFA_MAX: C2RustUnnamed_12 = 11;
    #[c2rust::src_loc = "37:2"]
    pub const IFA_TARGET_NETNSID: C2RustUnnamed_12 = 10;
    #[c2rust::src_loc = "36:2"]
    pub const IFA_RT_PRIORITY: C2RustUnnamed_12 = 9;
    #[c2rust::src_loc = "35:2"]
    pub const IFA_FLAGS: C2RustUnnamed_12 = 8;
    #[c2rust::src_loc = "34:2"]
    pub const IFA_MULTICAST: C2RustUnnamed_12 = 7;
    #[c2rust::src_loc = "32:2"]
    pub const IFA_ANYCAST: C2RustUnnamed_12 = 5;
    #[c2rust::src_loc = "27:2"]
    pub const IFA_UNSPEC: C2RustUnnamed_12 = 0;
    #[c2rust::src_loc = "44:9"]
    pub const IFA_F_SECONDARY: libc::c_int = 0x1 as libc::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const IFA_F_DEPRECATED: libc::c_int = 0x20 as libc::c_int;
    #[c2rust::src_loc = "52:9"]
    pub const IFA_F_TENTATIVE: libc::c_int = 0x40 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const IFA_F_TEMPORARY: libc::c_int = IFA_F_SECONDARY;
    use super::int_ll64_h::{__u8, __u32};
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
#[c2rust::header_src = "/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "397:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:17"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
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
#[c2rust::header_src = "/usr/include/asm-generic/errno-base.h:17"]
pub mod errno_base_h {
    #[c2rust::src_loc = "5:9"]
    pub const EPERM: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "16:9"]
    pub const ENOMEM: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "8:9"]
    pub const EINTR: libc::c_int = 4 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/fcntl-linux.h:17"]
pub mod fcntl_linux_h {
    #[c2rust::src_loc = "171:9"]
    pub const F_SETFL: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "62:10"]
    pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
    #[c2rust::src_loc = "170:9"]
    pub const F_GETFL: libc::c_int = 3 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:17"]
pub mod syslog_h {
    #[c2rust::src_loc = "54:9"]
    pub const LOG_ERR: libc::c_int = 3 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno.h:17"]
pub mod asm_generic_errno_h {
    #[c2rust::src_loc = "88:9"]
    pub const ENOBUFS: libc::c_int = 105 as libc::c_int;
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __intmax_t, __uintmax_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __ino64_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __pid_t, __time_t, __blksize_t,
                        __blkcnt_t, __blkcnt64_t, __ssize_t,
                        __syscall_slong_t, __socklen_t};
pub use self::sys_types_h::{ino_t, dev_t, off_t, pid_t, ssize_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, sockaddr, C2RustUnnamed, MSG_CMSG_CLOEXEC,
                         MSG_FASTOPEN, MSG_ZEROCOPY, MSG_BATCH,
                         MSG_WAITFORONE, MSG_MORE, MSG_NOSIGNAL, MSG_ERRQUEUE,
                         MSG_RST, MSG_CONFIRM, MSG_SYN, MSG_FIN, MSG_WAITALL,
                         MSG_EOR, MSG_DONTWAIT, MSG_TRUNC, MSG_PROXY,
                         MSG_CTRUNC, MSG_TRYHARD, MSG_DONTROUTE, MSG_PEEK,
                         MSG_OOB, msghdr, cmsghdr, __cmsg_nxthdr, SOL_NETLINK,
                         PF_NETLINK, AF_NETLINK, MSG_TRUNC_0, MSG_PEEK_0,
                         PF_LOCAL, AF_LOCAL, PF_UNSPEC, AF_UNSPEC, PF_INET6,
                         AF_INET6, PF_INET, AF_INET};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM,
                              SOCK_RAW_0};
pub use self::sockaddr_h::sa_family_t;
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_ns, sockaddr_iso,
                             sockaddr_ipx, sockaddr_inarp, sockaddr_eon,
                             sockaddr_dl, sockaddr_ax25, sockaddr_at, socket,
                             bind, getsockname, sendto, recvmsg, setsockopt};
pub use self::un_h::sockaddr_un;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_0, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::dnsmasq_h::{u32_0, all_addr, C2RustUnnamed_2, C2RustUnnamed_3,
                          blockdata, C2RustUnnamed_4, C2RustUnnamed_5,
                          C2RustUnnamed_6, C2RustUnnamed_7, crec,
                          C2RustUnnamed_8, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_netid, dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_9, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, EC_MISC, EVENT_NEWADDR,
                          EVENT_NEWROUTE, IFACE_PERMANENT, IFACE_DEPRECATED,
                          IFACE_TENTATIVE, safe_malloc, retry_send,
                          expand_buf, die, my_syslog, queue_event};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER};
pub use self::int_ll64_h::{__u8, __u16, __s32, __u32};
pub use self::if_h::{C2RustUnnamed_1, IFF_DYNAMIC, IFF_AUTOMEDIA, IFF_PORTSEL,
                     IFF_MULTICAST, IFF_SLAVE, IFF_MASTER, IFF_ALLMULTI,
                     IFF_PROMISC, IFF_NOARP, IFF_RUNNING, IFF_NOTRAILERS,
                     IFF_POINTOPOINT, IFF_LOOPBACK, IFF_DEBUG, IFF_BROADCAST,
                     IFF_UP, IFF_POINTOPOINT_0, IFF_LOOPBACK_0};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
pub use self::netlink_h::{sockaddr_nl, nlmsghdr, nlmsgerr, NETLINK_ROUTE,
                          NLM_F_REQUEST, NLM_F_ACK, NLM_F_ROOT, NLM_F_MATCH,
                          NLMSG_ALIGNTO, NLMSG_ERROR, NLMSG_DONE,
                          NETLINK_NO_ENOBUFS};
pub use self::linux_socket_h::__kernel_sa_family_t;
pub use self::rtnetlink_h::{RTM_DELADDR, RTM_NEWADDR, RT_TABLE_LOCAL, rtmsg,
                            RT_TABLE_MAIN, RT_SCOPE_LINK, RTN_UNICAST,
                            RTM_NEWROUTE, ifinfomsg, rtattr, RTM_NEWLINK,
                            RTM_NEWNEIGH, RTM_GETADDR, rtgenmsg, RTM_GETLINK,
                            RTM_GETNEIGH, C2RustUnnamed_14, __RTM_MAX,
                            RTM_GETNEXTHOP, RTM_DELNEXTHOP, RTM_NEWNEXTHOP,
                            RTM_GETCHAIN, RTM_DELCHAIN, RTM_NEWCHAIN,
                            RTM_NEWCACHEREPORT, RTM_GETSTATS, RTM_NEWSTATS,
                            RTM_GETNSID, RTM_DELNSID, RTM_NEWNSID, RTM_GETMDB,
                            RTM_DELMDB, RTM_NEWMDB, RTM_GETNETCONF,
                            RTM_DELNETCONF, RTM_NEWNETCONF, RTM_SETDCB,
                            RTM_GETDCB, RTM_GETADDRLABEL, RTM_DELADDRLABEL,
                            RTM_NEWADDRLABEL, RTM_NEWNDUSEROPT,
                            RTM_SETNEIGHTBL, RTM_GETNEIGHTBL, RTM_NEWNEIGHTBL,
                            RTM_GETANYCAST, RTM_GETMULTICAST, RTM_NEWPREFIX,
                            RTM_GETACTION, RTM_DELACTION, RTM_NEWACTION,
                            RTM_GETTFILTER, RTM_DELTFILTER, RTM_NEWTFILTER,
                            RTM_GETTCLASS, RTM_DELTCLASS, RTM_NEWTCLASS,
                            RTM_GETQDISC, RTM_DELQDISC, RTM_NEWQDISC,
                            RTM_GETRULE, RTM_DELRULE, RTM_NEWRULE,
                            RTM_DELNEIGH, RTM_GETROUTE, RTM_DELROUTE,
                            RTM_SETLINK, RTM_DELLINK, RTM_BASE,
                            C2RustUnnamed_15, __RTN_MAX, RTN_XRESOLVE,
                            RTN_NAT, RTN_THROW, RTN_PROHIBIT, RTN_UNREACHABLE,
                            RTN_BLACKHOLE, RTN_MULTICAST, RTN_ANYCAST,
                            RTN_BROADCAST, RTN_LOCAL, RTN_UNSPEC, rt_scope_t,
                            RT_SCOPE_NOWHERE, RT_SCOPE_HOST, RT_SCOPE_SITE,
                            RT_SCOPE_UNIVERSE, rt_class_t, RT_TABLE_MAX,
                            RT_TABLE_DEFAULT, RT_TABLE_COMPAT,
                            RT_TABLE_UNSPEC, RTM_NEWNEIGH_0, RTM_GETLINK_0,
                            RTM_NEWLINK_0, RTA_ALIGNTO, RTM_NEWROUTE_0,
                            RTM_NEWADDR_0, RTM_DELADDR_0, RTMGRP_IPV4_ROUTE,
                            RTMGRP_IPV4_IFADDR, RTMGRP_IPV6_ROUTE,
                            RTMGRP_IPV6_IFADDR, RTM_GETNEIGH_0,
                            RTM_GETADDR_0};
pub use self::if_link_h::{IFLA_ADDRESS, C2RustUnnamed_11, __IFLA_MAX,
                          IFLA_MAX_MTU, IFLA_MIN_MTU, IFLA_NEW_IFINDEX,
                          IFLA_CARRIER_DOWN_COUNT, IFLA_CARRIER_UP_COUNT,
                          IFLA_TARGET_NETNSID, IFLA_IF_NETNSID,
                          IFLA_NEW_NETNSID, IFLA_EVENT, IFLA_XDP, IFLA_PAD,
                          IFLA_GSO_MAX_SIZE, IFLA_GSO_MAX_SEGS,
                          IFLA_PROTO_DOWN, IFLA_PHYS_PORT_NAME,
                          IFLA_LINK_NETNSID, IFLA_PHYS_SWITCH_ID,
                          IFLA_CARRIER_CHANGES, IFLA_PHYS_PORT_ID,
                          IFLA_CARRIER, IFLA_NUM_RX_QUEUES,
                          IFLA_NUM_TX_QUEUES, IFLA_PROMISCUITY, IFLA_EXT_MASK,
                          IFLA_NET_NS_FD, IFLA_GROUP, IFLA_AF_SPEC,
                          IFLA_PORT_SELF, IFLA_VF_PORTS, IFLA_STATS64,
                          IFLA_VFINFO_LIST, IFLA_NUM_VF, IFLA_IFALIAS,
                          IFLA_NET_NS_PID, IFLA_LINKINFO, IFLA_LINKMODE,
                          IFLA_OPERSTATE, IFLA_WEIGHT, IFLA_MAP, IFLA_TXQLEN,
                          IFLA_PROTINFO, IFLA_WIRELESS, IFLA_MASTER,
                          IFLA_PRIORITY, IFLA_COST, IFLA_STATS, IFLA_QDISC,
                          IFLA_LINK, IFLA_MTU, IFLA_IFNAME, IFLA_BROADCAST,
                          IFLA_UNSPEC};
pub use self::neighbour_h::{ndmsg, NDA_LLADDR, NDA_DST, C2RustUnnamed_13,
                            __NDA_MAX, NDA_PROTOCOL, NDA_SRC_VNI,
                            NDA_LINK_NETNSID, NDA_MASTER, NDA_IFINDEX,
                            NDA_VNI, NDA_PORT, NDA_VLAN, NDA_PROBES,
                            NDA_CACHEINFO, NDA_UNSPEC, NUD_NOARP,
                            NUD_INCOMPLETE, NUD_FAILED};
pub use self::if_addr_h::{ifaddrmsg, ifa_cacheinfo, IFA_CACHEINFO,
                          IFA_ADDRESS, IFA_LABEL, IFA_BROADCAST, IFA_LOCAL,
                          C2RustUnnamed_12, __IFA_MAX, IFA_TARGET_NETNSID,
                          IFA_RT_PRIORITY, IFA_FLAGS, IFA_MULTICAST,
                          IFA_ANYCAST, IFA_UNSPEC, IFA_F_SECONDARY,
                          IFA_F_DEPRECATED, IFA_F_TENTATIVE, IFA_F_TEMPORARY};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity,
                                 __uint64_identity};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::string_h::{memset, strerror};
use self::unistd_h::sleep;
use self::stdio_h::{stdin, stdout, vfprintf, getc, __uflow, putc, __overflow,
                    __getdelim};
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
pub use self::errno_base_h::{EPERM, ENOMEM, EINTR};
pub use self::fcntl_linux_h::{F_SETFL, O_NONBLOCK, F_GETFL};
pub use self::syslog_h::LOG_ERR;
pub use self::asm_generic_errno_h::ENOBUFS;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "163:3"]
pub struct C2RustUnnamed_10 {
    pub nlh: nlmsghdr,
    pub g: rtgenmsg,
}
#[c2rust::src_loc = "47:21"]
static mut iov: iovec =
    iovec{iov_base: 0 as *const libc::c_void as *mut libc::c_void,
          iov_len: 0,};
#[c2rust::src_loc = "48:12"]
static mut netlink_pid: u32_0 = 0;
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub unsafe extern "C" fn netlink_init() -> *mut libc::c_char {
    let mut addr =
        sockaddr_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut slen =
        ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t;
    let mut opt = 1 as libc::c_int;
    addr.nl_family = AF_NETLINK as __kernel_sa_family_t;
    addr.nl_pad = 0 as libc::c_int as libc::c_ushort;
    addr.nl_pid = 0 as libc::c_int as __u32;
    addr.nl_groups = RTMGRP_IPV4_ROUTE as __u32;
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
        addr.nl_groups |= RTMGRP_IPV4_IFADDR as libc::c_uint
    }
    addr.nl_groups |= RTMGRP_IPV6_ROUTE as libc::c_uint;
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
        addr.nl_groups |= RTMGRP_IPV6_IFADDR as libc::c_uint
    }
    if (*dnsmasq_daemon).doing_ra != 0 || (*dnsmasq_daemon).doing_dhcp6 != 0 {
        addr.nl_groups |= RTMGRP_IPV6_IFADDR as libc::c_uint
    }
    /* May not be able to have permission to set multicast groups don't die in that case */
    (*dnsmasq_daemon).netlinkfd =
        socket(AF_NETLINK, SOCK_RAW_0, NETLINK_ROUTE);
    if (*dnsmasq_daemon).netlinkfd != -(1 as libc::c_int) {
        if bind((*dnsmasq_daemon).netlinkfd,
                __CONST_SOCKADDR_ARG{__sockaddr__:
                                         &mut addr as *mut sockaddr_nl as
                                             *mut sockaddr,},
                ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as
                    socklen_t) == -(1 as libc::c_int) {
            addr.nl_groups = 0 as libc::c_int as __u32;
            if errno != EPERM ||
                   bind((*dnsmasq_daemon).netlinkfd,
                        __CONST_SOCKADDR_ARG{__sockaddr__:
                                                 &mut addr as *mut sockaddr_nl
                                                     as *mut sockaddr,},
                        ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong
                            as socklen_t) == -(1 as libc::c_int) {
                (*dnsmasq_daemon).netlinkfd = -(1 as libc::c_int)
            }
        }
    }
    if (*dnsmasq_daemon).netlinkfd == -(1 as libc::c_int) ||
           getsockname((*dnsmasq_daemon).netlinkfd,
                       __SOCKADDR_ARG{__sockaddr__:
                                          &mut addr as *mut sockaddr_nl as
                                              *mut sockaddr,}, &mut slen) ==
               -(1 as libc::c_int) {
        die(b"cannot create netlink socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            NULL_0 as *mut libc::c_char, EC_MISC);
    }
    /* save pid assigned by bind() and retrieved by getsockname() */
    netlink_pid = addr.nl_pid;
    iov.iov_len = 100 as libc::c_int as size_t;
    iov.iov_base = safe_malloc(iov.iov_len);
    if (*dnsmasq_daemon).kernel_version >=
           ((2 as libc::c_int) << 16 as libc::c_int) +
               ((6 as libc::c_int) << 8 as libc::c_int) + 30 as libc::c_int &&
           setsockopt((*dnsmasq_daemon).netlinkfd, SOL_NETLINK,
                      NETLINK_NO_ENOBUFS,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) {
        return b"warning: failed to set NETLINK_NO_ENOBUFS on netlink socket\x00"
                   as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    return NULL_0 as *mut libc::c_char;
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn netlink_recv() -> ssize_t {
    let mut msg =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut nladdr =
        sockaddr_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut rc: ssize_t = 0;
    loop  {
        msg.msg_control = NULL_0 as *mut libc::c_void;
        msg.msg_controllen = 0 as libc::c_int as size_t;
        msg.msg_name = &mut nladdr as *mut sockaddr_nl as *mut libc::c_void;
        msg.msg_namelen =
            ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as
                socklen_t;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1 as libc::c_int as size_t;
        msg.msg_flags = 0 as libc::c_int;
        loop  {
            rc =
                recvmsg((*dnsmasq_daemon).netlinkfd, &mut msg,
                        MSG_PEEK_0 | MSG_TRUNC_0);
            if !(rc == -(1 as libc::c_int) as libc::c_long && errno == EINTR)
               {
                break ;
            }
        }
        /* make buffer big enough */
        if rc != -(1 as libc::c_int) as libc::c_long &&
               msg.msg_flags & MSG_TRUNC_0 != 0 {
            /* Very new Linux kernels return the actual size needed, older ones always return truncated size */
            if rc as size_t == iov.iov_len {
                if expand_buf(&mut iov,
                              (rc + 100 as libc::c_int as libc::c_long) as
                                  size_t) != 0 {
                    continue ;
                }
            } else { expand_buf(&mut iov, rc as size_t); }
        }
        /* read it for real */
        msg.msg_flags = 0 as libc::c_int;
        loop  {
            rc =
                recvmsg((*dnsmasq_daemon).netlinkfd, &mut msg,
                        0 as libc::c_int);
            if !(rc == -(1 as libc::c_int) as libc::c_long && errno == EINTR)
               {
                break ;
            }
        }
        /* Make sure this is from the kernel */
        if rc == -(1 as libc::c_int) as libc::c_long ||
               nladdr.nl_pid == 0 as libc::c_int as libc::c_uint {
            break ;
        }
    }
    /* discard stuff which is truncated at this point (expand_buf() may fail) */
    if msg.msg_flags & MSG_TRUNC_0 != 0 {
        rc = -(1 as libc::c_int) as ssize_t;
        errno = ENOMEM
    }
    return rc;
}
/* family = AF_UNSPEC finds ARP table entries.
   family = AF_LOCAL finds MAC addresses. */
#[no_mangle]
#[c2rust::src_loc = "155:1"]
pub unsafe extern "C" fn iface_enumerate(mut family: libc::c_int,
                                         mut parm: *mut libc::c_void,
                                         mut callback:
                                             Option<unsafe extern "C" fn()
                                                        -> libc::c_int>)
 -> libc::c_int {
    let mut addr =
        sockaddr_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut h = 0 as *mut nlmsghdr;
    let mut len: ssize_t = 0;
    static mut seq: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut callback_ok = 1 as libc::c_int;
    let mut req =
        C2RustUnnamed_10{nlh:
                             nlmsghdr{nlmsg_len: 0,
                                      nlmsg_type: 0,
                                      nlmsg_flags: 0,
                                      nlmsg_seq: 0,
                                      nlmsg_pid: 0,},
                         g: rtgenmsg{rtgen_family: 0,},};
    memset(&mut req as *mut C2RustUnnamed_10 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong);
    memset(&mut addr as *mut sockaddr_nl as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong);
    addr.nl_family = AF_NETLINK as __kernel_sa_family_t;
    loop  {
        if family == AF_UNSPEC {
            req.nlh.nlmsg_type = RTM_GETNEIGH_0 as __u16
        } else if family == AF_LOCAL {
            req.nlh.nlmsg_type = RTM_GETLINK_0 as __u16
        } else { req.nlh.nlmsg_type = RTM_GETADDR_0 as __u16 }
        req.nlh.nlmsg_len =
            ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as
                __u32;
        req.nlh.nlmsg_flags =
            (NLM_F_ROOT | NLM_F_MATCH | NLM_F_REQUEST | NLM_F_ACK) as __u16;
        req.nlh.nlmsg_pid = 0 as libc::c_int as __u32;
        seq = seq.wrapping_add(1);
        req.nlh.nlmsg_seq = seq;
        req.g.rtgen_family = family as libc::c_uchar;
        /* Don't block in recvfrom if send fails */
        while retry_send(sendto((*dnsmasq_daemon).netlinkfd,
                                &mut req as *mut C2RustUnnamed_10 as
                                    *mut libc::c_void,
                                ::std::mem::size_of::<C2RustUnnamed_10>() as
                                    libc::c_ulong, 0 as libc::c_int,
                                __CONST_SOCKADDR_ARG{__sockaddr__:
                                                         &mut addr as
                                                             *mut sockaddr_nl
                                                             as
                                                             *mut sockaddr,},
                                ::std::mem::size_of::<sockaddr_nl>() as
                                    libc::c_ulong as socklen_t)) != 0 {
        }
        if errno != 0 as libc::c_int { return 0 as libc::c_int }
        loop  {
            len = netlink_recv();
            if len == -(1 as libc::c_int) as libc::c_long { break ; }
            h = iov.iov_base as *mut nlmsghdr;
            while len as size_t >=
                      ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as
                          libc::c_int as libc::c_ulong &&
                      (*h).nlmsg_len as libc::c_ulong >=
                          ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
                      && (*h).nlmsg_len as libc::c_ulong <= len as size_t {
                if (*h).nlmsg_pid != netlink_pid ||
                       (*h).nlmsg_type as libc::c_int == NLMSG_ERROR {
                    /* May be multicast arriving async */
                    nl_async(h);
                } else if !((*h).nlmsg_seq != seq) {
                    if (*h).nlmsg_type as libc::c_int == NLMSG_DONE {
                        return callback_ok
                    } else {
                        if (*h).nlmsg_type as libc::c_int == RTM_NEWADDR_0 &&
                               family != AF_UNSPEC && family != AF_LOCAL {
                            let mut ifa =
                                (h as
                                     *mut libc::c_char).offset((0 as
                                                                    libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)
                                                                         &
                                                                         !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                             as
                                                                             libc::c_ulong)
                                                                        as
                                                                        libc::c_int)
                                                                   as isize)
                                    as *mut libc::c_void as *mut ifaddrmsg;
                            let mut rta =
                                (ifa as
                                     *mut libc::c_char).offset(((::std::mem::size_of::<ifaddrmsg>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                    &
                                                                    !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as isize)
                                    as *mut rtattr;
                            let mut len1 =
                                ((*h).nlmsg_len as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<ifaddrmsg>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((::std::mem::size_of::<nlmsghdr>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                       &
                                                                                                       !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                                                                                                           as
                                                                                                           libc::c_ulong)
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_uint;
                            if (*ifa).ifa_family as libc::c_int == family {
                                if (*ifa).ifa_family as libc::c_int == AF_INET
                                   {
                                    let mut netmask = in_addr{s_addr: 0,};
                                    let mut addr_0 = in_addr{s_addr: 0,};
                                    let mut broadcast = in_addr{s_addr: 0,};
                                    let mut label =
                                        NULL_0 as *mut libc::c_char;
                                    netmask.s_addr =
                                        __bswap_32((!(0 as libc::c_int as
                                                          in_addr_t)) <<
                                                       32 as libc::c_int -
                                                           (*ifa).ifa_prefixlen
                                                               as
                                                               libc::c_int);
                                    addr_0.s_addr =
                                        0 as libc::c_int as in_addr_t;
                                    broadcast.s_addr =
                                        0 as libc::c_int as in_addr_t;
                                    while len1 >=
                                              ::std::mem::size_of::<rtattr>()
                                                  as libc::c_ulong as
                                                  libc::c_int as libc::c_uint
                                              &&
                                              (*rta).rta_len as libc::c_ulong
                                                  >=
                                                  ::std::mem::size_of::<rtattr>()
                                                      as libc::c_ulong &&
                                              (*rta).rta_len as libc::c_uint
                                                  <= len1 {
                                        if (*rta).rta_type as libc::c_int ==
                                               IFA_LOCAL as libc::c_int {
                                            addr_0 =
                                                *(rta.offset(1 as libc::c_int
                                                                 as isize) as
                                                      *mut in_addr)
                                        } else if (*rta).rta_type as
                                                      libc::c_int ==
                                                      IFA_BROADCAST as
                                                          libc::c_int {
                                            broadcast =
                                                *(rta.offset(1 as libc::c_int
                                                                 as isize) as
                                                      *mut in_addr)
                                        } else if (*rta).rta_type as
                                                      libc::c_int ==
                                                      IFA_LABEL as libc::c_int
                                         {
                                            label =
                                                (rta as
                                                     *mut libc::c_char).offset(((::std::mem::size_of::<rtattr>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(RTA_ALIGNTO
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong)
                                                                                    &
                                                                                    !RTA_ALIGNTO.wrapping_sub(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)
                                                                                        as
                                                                                        libc::c_ulong).wrapping_add(0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong)
                                                                                   as
                                                                                   isize)
                                                    as *mut libc::c_void as
                                                    *mut libc::c_char
                                        }
                                        len1 =
                                            len1.wrapping_sub(((*rta).rta_len
                                                                   as
                                                                   libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)
                                                                  &
                                                                  !RTA_ALIGNTO.wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint));
                                        rta =
                                            (rta as
                                                 *mut libc::c_char).offset((((*rta).rta_len
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint)
                                                                                &
                                                                                !RTA_ALIGNTO.wrapping_sub(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
                                                                               as
                                                                               isize)
                                                as *mut rtattr
                                    }
                                    if addr_0.s_addr != 0 && callback_ok != 0
                                       {
                                        if ::std::mem::transmute::<_,
                                                                   fn(_: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _)
                                                                       ->
                                                                           libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(addr_0,
                                                                                                                                                                                (*ifa).ifa_index,
                                                                                                                                                                                label,
                                                                                                                                                                                netmask,
                                                                                                                                                                                broadcast,
                                                                                                                                                                                parm)
                                               == 0 {
                                            callback_ok = 0 as libc::c_int
                                        }
                                    }
                                } else if (*ifa).ifa_family as libc::c_int ==
                                              AF_INET6 {
                                    let mut addrp = NULL_0 as *mut in6_addr;
                                    let mut valid = 0 as libc::c_int as u32_0;
                                    let mut preferred =
                                        0 as libc::c_int as u32_0;
                                    let mut flags = 0 as libc::c_int;
                                    while len1 >=
                                              ::std::mem::size_of::<rtattr>()
                                                  as libc::c_ulong as
                                                  libc::c_int as libc::c_uint
                                              &&
                                              (*rta).rta_len as libc::c_ulong
                                                  >=
                                                  ::std::mem::size_of::<rtattr>()
                                                      as libc::c_ulong &&
                                              (*rta).rta_len as libc::c_uint
                                                  <= len1 {
                                        if (*rta).rta_type as libc::c_int ==
                                               IFA_ADDRESS as libc::c_int {
                                            addrp =
                                                rta.offset(1 as libc::c_int as
                                                               isize) as
                                                    *mut in6_addr
                                        } else if (*rta).rta_type as
                                                      libc::c_int ==
                                                      IFA_CACHEINFO as
                                                          libc::c_int {
                                            let mut ifc =
                                                rta.offset(1 as libc::c_int as
                                                               isize) as
                                                    *mut ifa_cacheinfo;
                                            preferred = (*ifc).ifa_prefered;
                                            valid = (*ifc).ifa_valid
                                        }
                                        len1 =
                                            len1.wrapping_sub(((*rta).rta_len
                                                                   as
                                                                   libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)
                                                                  &
                                                                  !RTA_ALIGNTO.wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint));
                                        rta =
                                            (rta as
                                                 *mut libc::c_char).offset((((*rta).rta_len
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint)
                                                                                &
                                                                                !RTA_ALIGNTO.wrapping_sub(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
                                                                               as
                                                                               isize)
                                                as *mut rtattr
                                    }
                                    if (*ifa).ifa_flags as libc::c_int &
                                           IFA_F_TENTATIVE != 0 {
                                        flags |= IFACE_TENTATIVE
                                    }
                                    if (*ifa).ifa_flags as libc::c_int &
                                           IFA_F_DEPRECATED != 0 {
                                        flags |= IFACE_DEPRECATED
                                    }
                                    if (*ifa).ifa_flags as libc::c_int &
                                           IFA_F_TEMPORARY == 0 {
                                        flags |= IFACE_PERMANENT
                                    }
                                    if !addrp.is_null() && callback_ok != 0 {
                                        if ::std::mem::transmute::<_,
                                                                   fn(_: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _)
                                                                       ->
                                                                           libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(addrp,
                                                                                                                                                                                (*ifa).ifa_prefixlen
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                (*ifa).ifa_scope
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                (*ifa).ifa_index
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                flags,
                                                                                                                                                                                preferred
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                valid
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                parm)
                                               == 0 {
                                            callback_ok = 0 as libc::c_int
                                        }
                                    }
                                }
                            }
                        } else if (*h).nlmsg_type as libc::c_int ==
                                      RTM_NEWNEIGH_0 && family == AF_UNSPEC {
                            let mut neigh =
                                (h as
                                     *mut libc::c_char).offset((0 as
                                                                    libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)
                                                                         &
                                                                         !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                             as
                                                                             libc::c_ulong)
                                                                        as
                                                                        libc::c_int)
                                                                   as isize)
                                    as *mut libc::c_void as *mut ndmsg;
                            let mut rta_0 =
                                (neigh as
                                     *mut libc::c_char).offset(((::std::mem::size_of::<ndmsg>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                    &
                                                                    !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as isize)
                                    as *mut rtattr;
                            let mut len1_0 =
                                ((*h).nlmsg_len as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<ndmsg>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((::std::mem::size_of::<nlmsghdr>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                       &
                                                                                                       !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                                                                                                           as
                                                                                                           libc::c_ulong)
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_uint;
                            let mut maclen = 0 as libc::c_int as size_t;
                            let mut inaddr = NULL_0 as *mut libc::c_char;
                            let mut mac = NULL_0 as *mut libc::c_char;
                            while len1_0 >=
                                      ::std::mem::size_of::<rtattr>() as
                                          libc::c_ulong as libc::c_int as
                                          libc::c_uint &&
                                      (*rta_0).rta_len as libc::c_ulong >=
                                          ::std::mem::size_of::<rtattr>() as
                                              libc::c_ulong &&
                                      (*rta_0).rta_len as libc::c_uint <=
                                          len1_0 {
                                if (*rta_0).rta_type as libc::c_int ==
                                       NDA_DST as libc::c_int {
                                    inaddr =
                                        rta_0.offset(1 as libc::c_int as
                                                         isize) as
                                            *mut libc::c_char
                                } else if (*rta_0).rta_type as libc::c_int ==
                                              NDA_LLADDR as libc::c_int {
                                    maclen =
                                        ((*rta_0).rta_len as
                                             libc::c_ulong).wrapping_sub(::std::mem::size_of::<rtattr>()
                                                                             as
                                                                             libc::c_ulong);
                                    mac =
                                        rta_0.offset(1 as libc::c_int as
                                                         isize) as
                                            *mut libc::c_char
                                }
                                len1_0 =
                                    len1_0.wrapping_sub(((*rta_0).rta_len as
                                                             libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint)
                                                            &
                                                            !RTA_ALIGNTO.wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint));
                                rta_0 =
                                    (rta_0 as
                                         *mut libc::c_char).offset((((*rta_0).rta_len
                                                                         as
                                                                         libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint)
                                                                        &
                                                                        !RTA_ALIGNTO.wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
                                                                       as
                                                                       isize)
                                        as *mut rtattr
                            }
                            if (*neigh).ndm_state as libc::c_int &
                                   (NUD_NOARP | NUD_INCOMPLETE | NUD_FAILED)
                                   == 0 && !inaddr.is_null() && !mac.is_null()
                                   && callback_ok != 0 {
                                if ::std::mem::transmute::<_,
                                                           fn(_: _, _: _,
                                                              _: _, _: _,
                                                              _: _)
                                                               ->
                                                                   libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))((*neigh).ndm_family
                                                                                                                                                                            as
                                                                                                                                                                            libc::c_int,
                                                                                                                                                                        inaddr,
                                                                                                                                                                        mac,
                                                                                                                                                                        maclen,
                                                                                                                                                                        parm)
                                       == 0 {
                                    callback_ok = 0 as libc::c_int
                                }
                            }
                        } else if (*h).nlmsg_type as libc::c_int ==
                                      RTM_NEWLINK_0 && family == AF_LOCAL {
                            let mut link =
                                (h as
                                     *mut libc::c_char).offset((0 as
                                                                    libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)
                                                                         &
                                                                         !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                             as
                                                                             libc::c_ulong)
                                                                        as
                                                                        libc::c_int)
                                                                   as isize)
                                    as *mut libc::c_void as *mut ifinfomsg;
                            let mut rta_1 =
                                (link as
                                     *mut libc::c_char).offset(((::std::mem::size_of::<ifinfomsg>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                    &
                                                                    !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as isize)
                                    as *mut rtattr;
                            let mut len1_1 =
                                ((*h).nlmsg_len as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<ifinfomsg>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((::std::mem::size_of::<nlmsghdr>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                       &
                                                                                                       !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                                                                                                           as
                                                                                                           libc::c_ulong)
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_uint;
                            let mut mac_0 = NULL_0 as *mut libc::c_char;
                            let mut maclen_0 = 0 as libc::c_int as size_t;
                            while len1_1 >=
                                      ::std::mem::size_of::<rtattr>() as
                                          libc::c_ulong as libc::c_int as
                                          libc::c_uint &&
                                      (*rta_1).rta_len as libc::c_ulong >=
                                          ::std::mem::size_of::<rtattr>() as
                                              libc::c_ulong &&
                                      (*rta_1).rta_len as libc::c_uint <=
                                          len1_1 {
                                if (*rta_1).rta_type as libc::c_int ==
                                       IFLA_ADDRESS as libc::c_int {
                                    maclen_0 =
                                        ((*rta_1).rta_len as
                                             libc::c_ulong).wrapping_sub(::std::mem::size_of::<rtattr>()
                                                                             as
                                                                             libc::c_ulong);
                                    mac_0 =
                                        rta_1.offset(1 as libc::c_int as
                                                         isize) as
                                            *mut libc::c_char
                                }
                                len1_1 =
                                    len1_1.wrapping_sub(((*rta_1).rta_len as
                                                             libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint)
                                                            &
                                                            !RTA_ALIGNTO.wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint));
                                rta_1 =
                                    (rta_1 as
                                         *mut libc::c_char).offset((((*rta_1).rta_len
                                                                         as
                                                                         libc::c_uint).wrapping_add(RTA_ALIGNTO).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint)
                                                                        &
                                                                        !RTA_ALIGNTO.wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
                                                                       as
                                                                       isize)
                                        as *mut rtattr
                            }
                            if !mac_0.is_null() && callback_ok != 0 &&
                                   (*link).ifi_flags &
                                       (IFF_LOOPBACK_0 | IFF_POINTOPOINT_0) as
                                           libc::c_uint == 0 &&
                                   ::std::mem::transmute::<_,
                                                           fn(_: _, _: _,
                                                              _: _, _: _,
                                                              _: _)
                                                               ->
                                                                   libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))((*link).ifi_index,
                                                                                                                                                                        (*link).ifi_type
                                                                                                                                                                            as
                                                                                                                                                                            libc::c_uint,
                                                                                                                                                                        mac_0,
                                                                                                                                                                        maclen_0,
                                                                                                                                                                        parm)
                                       == 0 {
                                callback_ok = 0 as libc::c_int
                            }
                        }
                    }
                }
                /* May be part of incomplete response to previous request after
	       ENOBUFS. Drop it. */
                len -=
                    ((*h).nlmsg_len.wrapping_add(NLMSG_ALIGNTO).wrapping_sub(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                         &
                         !NLMSG_ALIGNTO.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint)) as
                        libc::c_long;
                h =
                    (h as
                         *mut libc::c_char).offset(((*h).nlmsg_len.wrapping_add(NLMSG_ALIGNTO).wrapping_sub(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                        &
                                                        !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
                                                       as isize) as
                        *mut nlmsghdr
            }
        }
        if !(errno == ENOBUFS) { break ; }
        sleep(1 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn netlink_multicast() {
    let mut len: ssize_t = 0;
    let mut h = 0 as *mut nlmsghdr;
    let mut flags: libc::c_int = 0;
    /* don't risk blocking reading netlink messages here. */
    flags = fcntl((*dnsmasq_daemon).netlinkfd, F_GETFL);
    if flags == -(1 as libc::c_int) ||
           fcntl((*dnsmasq_daemon).netlinkfd, F_SETFL, flags | O_NONBLOCK) ==
               -(1 as libc::c_int) {
        return
    }
    len = netlink_recv();
    if len != -(1 as libc::c_int) as libc::c_long {
        h = iov.iov_base as *mut nlmsghdr;
        while len as size_t >=
                  ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as
                      libc::c_int as libc::c_ulong &&
                  (*h).nlmsg_len as libc::c_ulong >=
                      ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong &&
                  (*h).nlmsg_len as libc::c_ulong <= len as size_t {
            nl_async(h);
            len -=
                ((*h).nlmsg_len.wrapping_add(NLMSG_ALIGNTO).wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                     &
                     !NLMSG_ALIGNTO.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint)) as
                    libc::c_long;
            h =
                (h as
                     *mut libc::c_char).offset(((*h).nlmsg_len.wrapping_add(NLMSG_ALIGNTO).wrapping_sub(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint)
                                                    &
                                                    !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
                                                   as isize) as *mut nlmsghdr
        }
    }
    /* restore non-blocking status */
    fcntl((*dnsmasq_daemon).netlinkfd, F_SETFL, flags);
}
#[c2rust::src_loc = "363:1"]
unsafe extern "C" fn nl_async(mut h: *mut nlmsghdr) {
    if (*h).nlmsg_type as libc::c_int == NLMSG_ERROR {
        let mut err =
            (h as
                 *mut libc::c_char).offset((0 as libc::c_int +
                                                ((::std::mem::size_of::<nlmsghdr>()
                                                      as
                                                      libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                      as
                                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong)
                                                     &
                                                     !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                                         as libc::c_ulong) as
                                                    libc::c_int) as isize) as
                *mut libc::c_void as *mut nlmsgerr;
        if (*err).error != 0 as libc::c_int {
            my_syslog(LOG_ERR,
                      b"netlink returns error: %s\x00" as *const u8 as
                          *const libc::c_char, strerror(-(*err).error));
        }
    } else if (*h).nlmsg_pid == 0 as libc::c_int as libc::c_uint &&
                  (*h).nlmsg_type as libc::c_int == RTM_NEWROUTE_0 {
        /* We arrange to receive netlink multicast messages whenever the network route is added.
	 If this happens and we still have a DNS packet in the buffer, we re-send it.
	 This helps on DoD links, where frequently the packet which triggers dialling is
	 a DNS query, which then gets lost. By re-sending, we can avoid the lookup
	 failing. */
        let mut rtm =
            (h as
                 *mut libc::c_char).offset((0 as libc::c_int +
                                                ((::std::mem::size_of::<nlmsghdr>()
                                                      as
                                                      libc::c_ulong).wrapping_add(NLMSG_ALIGNTO
                                                                                      as
                                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong)
                                                     &
                                                     !NLMSG_ALIGNTO.wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                                         as libc::c_ulong) as
                                                    libc::c_int) as isize) as
                *mut libc::c_void as *mut rtmsg;
        if (*rtm).rtm_type as libc::c_int == RTN_UNICAST as libc::c_int &&
               (*rtm).rtm_scope as libc::c_int == RT_SCOPE_LINK as libc::c_int
               &&
               ((*rtm).rtm_table as libc::c_int ==
                    RT_TABLE_MAIN as libc::c_int ||
                    (*rtm).rtm_table as libc::c_int ==
                        RT_TABLE_LOCAL as libc::c_int) {
            queue_event(EVENT_NEWROUTE);
        }
    } else if (*h).nlmsg_type as libc::c_int == RTM_NEWADDR_0 ||
                  (*h).nlmsg_type as libc::c_int == RTM_DELADDR_0 {
        queue_event(EVENT_NEWADDR);
    };
}
