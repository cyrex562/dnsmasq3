#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    #[c2rust::src_loc = "38:9"]
    pub const __LONG_MAX__: libc::c_long =
        9223372036854775807 as libc::c_long;
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
    use super::types_h::__socklen_t;
    use super::sockaddr_h::sa_family_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/sockaddr.h:17"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
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
        pub cname: C2RustUnnamed_5,
        pub key: C2RustUnnamed_4,
        pub ds: C2RustUnnamed_3,
        pub srv: C2RustUnnamed_2,
        pub log: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "322:3"]
    pub struct C2RustUnnamed_1 {
        pub keytag: libc::c_ushort,
        pub algo: libc::c_ushort,
        pub digest: libc::c_ushort,
        pub rcode: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:3"]
    pub struct C2RustUnnamed_2 {
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
    pub struct C2RustUnnamed_3 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
        pub digest: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:3"]
    pub struct C2RustUnnamed_4 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub flags: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:3"]
    pub struct C2RustUnnamed_5 {
        pub target: C2RustUnnamed_6,
        pub uid: libc::c_uint,
        pub is_name_ptr: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:5"]
    pub union C2RustUnnamed_6 {
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
        pub name: C2RustUnnamed_7,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "455:3"]
    pub union C2RustUnnamed_7 {
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
        pub u: C2RustUnnamed_8,
        pub val: *mut libc::c_uchar,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_opt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "815:3"]
    pub union C2RustUnnamed_8 {
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
    #[c2rust::src_loc = "484:9"]
    pub const F_QUERY: libc::c_uint =
        (1 as libc::c_uint) << 19 as libc::c_int;
    #[c2rust::src_loc = "480:9"]
    pub const F_DNSSECOK: libc::c_uint =
        (1 as libc::c_uint) << 15 as libc::c_int;
    #[c2rust::src_loc = "473:9"]
    pub const F_IPV6: libc::c_uint = (1 as libc::c_uint) << 8 as libc::c_int;
    #[c2rust::src_loc = "472:9"]
    pub const F_IPV4: libc::c_uint = (1 as libc::c_uint) << 7 as libc::c_int;
    #[c2rust::src_loc = "494:9"]
    pub const F_RCODE: libc::c_uint =
        (1 as libc::c_uint) << 29 as libc::c_int;
    #[c2rust::src_loc = "478:9"]
    pub const F_CONFIG: libc::c_uint =
        (1 as libc::c_uint) << 13 as libc::c_int;
    #[c2rust::src_loc = "493:9"]
    pub const F_SERVFAIL: libc::c_uint =
        (1 as libc::c_uint) << 28 as libc::c_int;
    #[c2rust::src_loc = "475:9"]
    pub const F_NXDOMAIN: libc::c_uint =
        (1 as libc::c_uint) << 10 as libc::c_int;
    #[c2rust::src_loc = "485:9"]
    pub const F_NOERR: libc::c_uint =
        (1 as libc::c_uint) << 20 as libc::c_int;
    #[c2rust::src_loc = "470:9"]
    pub const F_NEG: libc::c_uint = (1 as libc::c_uint) << 5 as libc::c_int;
    #[c2rust::src_loc = "468:9"]
    pub const F_FORWARD: libc::c_uint =
        (1 as libc::c_uint) << 3 as libc::c_int;
    #[c2rust::src_loc = "491:9"]
    pub const F_IPSET: libc::c_uint =
        (1 as libc::c_uint) << 26 as libc::c_int;
    #[c2rust::src_loc = "495:9"]
    pub const F_SRV: libc::c_uint = (1 as libc::c_uint) << 30 as libc::c_int;
    #[c2rust::src_loc = "476:9"]
    pub const F_CNAME: libc::c_uint =
        (1 as libc::c_uint) << 11 as libc::c_int;
    #[c2rust::src_loc = "467:9"]
    pub const F_REVERSE: libc::c_uint =
        (1 as libc::c_uint) << 2 as libc::c_int;
    #[c2rust::src_loc = "465:9"]
    pub const F_IMMORTAL: libc::c_uint =
        (1 as libc::c_uint) << 0 as libc::c_int;
    #[c2rust::src_loc = "469:9"]
    pub const F_DHCP: libc::c_uint = (1 as libc::c_uint) << 4 as libc::c_int;
    #[c2rust::src_loc = "482:9"]
    pub const F_RRNAME: libc::c_uint =
        (1 as libc::c_uint) << 17 as libc::c_int;
    #[c2rust::src_loc = "490:9"]
    pub const F_NO_RR: libc::c_uint =
        (1 as libc::c_uint) << 25 as libc::c_int;
    #[c2rust::src_loc = "471:9"]
    pub const F_HOSTS: libc::c_uint = (1 as libc::c_uint) << 6 as libc::c_int;
    #[c2rust::src_loc = "389:9"]
    pub const ADDRLIST_REVONLY: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "388:9"]
    pub const ADDRLIST_IPV6: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "522:9"]
    pub const SERV_HAS_DOMAIN: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "520:9"]
    pub const SERV_NO_ADDR: libc::c_int = 2 as libc::c_int;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6,
                      in_addr_t};
    use super::time_t_h::time_t;
    use super::socket_h::sockaddr;
    use super::sys_types_h::{off_t, dev_t, ino_t, pid_t};
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::FILE_h::FILE;
    use super::dns_protocol_h::dns_header;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1172:4"]
        pub static mut dnsmasq_daemon: *mut dnsmasq_daemon;
        #[no_mangle]
        #[c2rust::src_loc = "1176:1"]
        pub fn next_uid(crecp: *mut crec);
        #[no_mangle]
        #[c2rust::src_loc = "1177:1"]
        pub fn log_query(flags: libc::c_uint, name: *mut libc::c_char,
                         addr: *mut all_addr, arg: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "1178:1"]
        pub fn record_source(index: libc::c_uint) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1179:1"]
        pub fn querystr(desc: *mut libc::c_char, type_0: libc::c_ushort)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1180:1"]
        pub fn cache_find_non_terminal(name: *mut libc::c_char, now: time_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1181:1"]
        pub fn cache_find_by_addr(crecp: *mut crec, addr: *mut all_addr,
                                  now: time_t, prot: libc::c_uint)
         -> *mut crec;
        #[no_mangle]
        #[c2rust::src_loc = "1184:1"]
        pub fn cache_find_by_name(crecp: *mut crec, name: *mut libc::c_char,
                                  now: time_t, prot: libc::c_uint)
         -> *mut crec;
        #[no_mangle]
        #[c2rust::src_loc = "1186:1"]
        pub fn cache_end_insert();
        #[no_mangle]
        #[c2rust::src_loc = "1187:1"]
        pub fn cache_start_insert();
        #[no_mangle]
        #[c2rust::src_loc = "1189:1"]
        pub fn cache_insert(name: *mut libc::c_char, addr: *mut all_addr,
                            class: libc::c_ushort, now: time_t,
                            ttl: libc::c_ulong, flags: libc::c_uint)
         -> *mut crec;
        #[no_mangle]
        #[c2rust::src_loc = "1197:1"]
        pub fn cache_make_stat(t: *mut txt_record) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1199:1"]
        pub fn cache_get_name(crecp: *mut crec) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1200:1"]
        pub fn cache_get_cname_target(crecp: *mut crec) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1208:1"]
        pub fn blockdata_alloc(data: *mut libc::c_char, len: size_t)
         -> *mut blockdata;
        #[no_mangle]
        #[c2rust::src_loc = "1209:1"]
        pub fn blockdata_retrieve(block: *mut blockdata, len: size_t,
                                  data: *mut libc::c_void)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1217:1"]
        pub fn is_name_synthetic(flags: libc::c_int, name: *mut libc::c_char,
                                 addr: *mut all_addr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1218:1"]
        pub fn is_rev_synth(flag: libc::c_int, addr: *mut all_addr,
                            name: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1287:1"]
        pub fn do_rfc1035_name(p: *mut libc::c_uchar, sval: *mut libc::c_char,
                               limit: *mut libc::c_char)
         -> *mut libc::c_uchar;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char,
                         _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1298:1"]
        pub fn is_same_net(a: in_addr, b: in_addr, mask: in_addr)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1512:1"]
        pub fn add_to_ipset(setname: *const libc::c_char,
                            ipaddr: *const all_addr, flags: libc::c_int,
                            remove: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1294:1"]
        pub fn hostname_isequal(a: *const libc::c_char,
                                b: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1664:1"]
        pub fn add_pseudoheader(header: *mut dns_header, plen: size_t,
                                limit: *mut libc::c_uchar,
                                udp_sz: libc::c_ushort, optno: libc::c_int,
                                opt: *mut libc::c_uchar, optlen: size_t,
                                set_do: libc::c_int, replace: libc::c_int)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "1371:1"]
        pub fn enumerate_interfaces(reset: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1295:1"]
        pub fn hostname_issubdomain(a: *mut libc::c_char,
                                    b: *mut libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:17"]
pub mod dns_protocol_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct dns_header {
        pub id: u16_0,
        pub hb3: u8_0,
        pub hb4: u8_0,
        pub qdcount: u16_0,
        pub ancount: u16_0,
        pub nscount: u16_0,
        pub arcount: u16_0,
    }
    #[c2rust::src_loc = "26:9"]
    pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const T_DNSKEY: libc::c_int = 48 as libc::c_int;
    #[c2rust::src_loc = "69:9"]
    pub const T_DS: libc::c_int = 43 as libc::c_int;
    #[c2rust::src_loc = "78:9"]
    pub const T_ANY: libc::c_int = 255 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const T_AAAA: libc::c_int = 28 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const T_A: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const C_IN: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const QUERY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "93:9"]
    pub const HB3_OPCODE: libc::c_int = 0x78 as libc::c_int;
    #[c2rust::src_loc = "101:9"]
    pub const HB4_RCODE: libc::c_int = 0xf as libc::c_int;
    #[c2rust::src_loc = "35:9"]
    pub const REFUSED: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "23:9"]
    pub const INADDRSZ: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const IN6ADDRSZ: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const HB3_AA: libc::c_int = 0x4 as libc::c_int;
    #[c2rust::src_loc = "32:9"]
    pub const SERVFAIL: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "98:9"]
    pub const HB4_RA: libc::c_int = 0x80 as libc::c_int;
    #[c2rust::src_loc = "99:9"]
    pub const HB4_AD: libc::c_int = 0x20 as libc::c_int;
    #[c2rust::src_loc = "92:9"]
    pub const HB3_QR: libc::c_int = 0x80 as libc::c_int;
    #[c2rust::src_loc = "95:9"]
    pub const HB3_TC: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "100:9"]
    pub const HB4_CD: libc::c_int = 0x10 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const NXDOMAIN: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const T_TXT: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const T_SOA: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const T_CNAME: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "64:9"]
    pub const T_SRV: libc::c_int = 33 as libc::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const T_PTR: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "77:9"]
    pub const T_MAILB: libc::c_int = 253 as libc::c_int;
    #[c2rust::src_loc = "65:9"]
    pub const T_NAPTR: libc::c_int = 35 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const HB3_RD: libc::c_int = 0x1 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const T_MX: libc::c_int = 15 as libc::c_int;
    #[c2rust::src_loc = "34:9"]
    pub const NOTIMP: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "40:9"]
    pub const C_CHAOS: libc::c_int = 3 as libc::c_int;
    use super::dnsmasq_h::{u16_0, u8_0};
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
#[c2rust::header_src =
  "/usr/lib/llvm-10/lib/clang/10.0.0/include/stdarg.h:17"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
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
#[c2rust::header_src = "/usr/include/ctype.h:17"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_0 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_0 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_0 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_0 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_0 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_0 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_0 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_0 = 256;
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
#[c2rust::header_src = "/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
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
        #[c2rust::src_loc = "137:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "226:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[c2rust::header_src =
  "/usr/lib/llvm-10/lib/clang/10.0.0/include/limits.h:17"]
pub mod limits_h {
    use super::internal::__LONG_MAX__;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:17"]
pub mod syslog_h {
    #[c2rust::src_loc = "57:9"]
    pub const LOG_INFO: libc::c_int = 6 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/config.h:17"]
pub mod config_h {
    #[c2rust::src_loc = "42:9"]
    pub const CNAME_CHAIN: libc::c_int = 10 as libc::c_int;
}
pub use self::internal::{__builtin_va_list, __va_list_tag, __LONG_MAX__};
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __intmax_t, __uintmax_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __ino64_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __pid_t, __time_t, __blksize_t,
                        __blkcnt_t, __blkcnt64_t, __ssize_t,
                        __syscall_slong_t, __socklen_t};
pub use self::sys_types_h::{ino_t, dev_t, off_t, pid_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, sockaddr, msghdr, cmsghdr, __cmsg_nxthdr};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::dnsmasq_h::{u8_0, u16_0, u32_0, all_addr, C2RustUnnamed_1,
                          C2RustUnnamed_2, blockdata, C2RustUnnamed_3,
                          C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6,
                          crec, C2RustUnnamed_7, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_netid, dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_8, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, F_QUERY, F_DNSSECOK,
                          F_IPV6, F_IPV4, F_RCODE, F_CONFIG, F_SERVFAIL,
                          F_NXDOMAIN, F_NOERR, F_NEG, F_FORWARD, F_IPSET,
                          F_SRV, F_CNAME, F_REVERSE, F_IMMORTAL, F_DHCP,
                          F_RRNAME, F_NO_RR, F_HOSTS, ADDRLIST_REVONLY,
                          ADDRLIST_IPV6, SERV_HAS_DOMAIN, SERV_NO_ADDR,
                          next_uid, log_query, record_source, querystr,
                          cache_find_non_terminal, cache_find_by_addr,
                          cache_find_by_name, cache_end_insert,
                          cache_start_insert, cache_insert, cache_make_stat,
                          cache_get_name, cache_get_cname_target,
                          blockdata_alloc, blockdata_retrieve,
                          is_name_synthetic, is_rev_synth, do_rfc1035_name,
                          my_syslog, is_same_net, add_to_ipset,
                          hostname_isequal, add_pseudoheader,
                          enumerate_interfaces, hostname_issubdomain};
pub use self::dns_protocol_h::{dns_header, MAXDNAME, T_DNSKEY, T_DS, T_ANY,
                               T_AAAA, T_A, C_IN, QUERY, HB3_OPCODE,
                               HB4_RCODE, REFUSED, INADDRSZ, IN6ADDRSZ,
                               HB3_AA, SERVFAIL, HB4_RA, HB4_AD, HB3_QR,
                               HB3_TC, HB4_CD, NXDOMAIN, T_TXT, T_SOA,
                               T_CNAME, T_SRV, T_PTR, T_MAILB, T_NAPTR,
                               HB3_RD, T_MX, NOTIMP, C_CHAOS};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER};
pub use self::stdarg_h::va_list;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll};
pub use self::ctype_h::{C2RustUnnamed_0, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper, tolower,
                        toupper, __ctype_b_loc, __ctype_tolower_loc,
                        __ctype_toupper_loc};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
use self::stdio_h::{stdin, stdout, vfprintf, getc, putc, __getdelim, __uflow,
                    __overflow};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::string_h::{memcpy, memmove, memset, memcmp, strcpy, strcmp, strchr,
                     strlen};
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
pub use self::syslog_h::LOG_INFO;
pub use self::config_h::CNAME_CHAIN;
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
#[c2rust::src_loc = "19:1"]
pub unsafe extern "C" fn extract_name(mut header: *mut dns_header,
                                      mut plen: size_t,
                                      mut pp: *mut *mut libc::c_uchar,
                                      mut name: *mut libc::c_char,
                                      mut isExtract: libc::c_int,
                                      mut extrabytes: libc::c_int)
 -> libc::c_int {
    let mut cp = name as *mut libc::c_uchar;
    let mut p = *pp;
    let mut p1 = NULL_0 as *mut libc::c_uchar;
    let mut j: libc::c_uint = 0;
    let mut l: libc::c_uint = 0;
    let mut namelen = 0 as libc::c_int as libc::c_uint;
    let mut hops = 0 as libc::c_int as libc::c_uint;
    let mut retvalue = 1 as libc::c_int;
    if isExtract != 0 { *cp = 0 as libc::c_int as libc::c_uchar }
    loop  {
        let mut label_type: libc::c_uint = 0;
        if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + 1 as libc::c_int as libc::c_long) as size_t
                 <= plen) {
            return 0 as libc::c_int
        }
        let fresh6 = p;
        p = p.offset(1);
        l = *fresh6 as libc::c_uint;
        if l == 0 as libc::c_int as libc::c_uint {
            /* label types 0x40 and 0x80 not supported */
            /* end marker */
            /* check that there are the correct no. of bytes after the name */
            if !(((if !p1.is_null() {
                       p1
                   } else {
                       p
                   }).wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + extrabytes as libc::c_long) as size_t <=
                     plen) {
                return 0 as libc::c_int
            }
            if isExtract != 0 {
                if cp != name as *mut libc::c_uchar { cp = cp.offset(-1) }
                *cp = 0 as libc::c_int as libc::c_uchar
                /* terminate: lose final period */
            } else if *cp as libc::c_int != 0 as libc::c_int {
                retvalue = 2 as libc::c_int
            }
            if !p1.is_null() {
                /* we jumped via compression */
                *pp = p1
            } else { *pp = p }
            return retvalue
        }
        label_type = l & 0xc0 as libc::c_int as libc::c_uint;
        if label_type == 0xc0 as libc::c_int as libc::c_uint {
            /* pointer */
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 1 as libc::c_int as libc::c_long) as
                     size_t <= plen) {
                return 0 as libc::c_int
            }
            /* get offset */
            l = (l & 0x3f as libc::c_int as libc::c_uint) << 8 as libc::c_int;
            let fresh7 = p;
            p = p.offset(1);
            l |= *fresh7 as libc::c_uint;
            if p1.is_null() {
                /* first jump, save location to go back to */
                p1 = p
            } /* break malicious infinite loops */
            hops = hops.wrapping_add(1);
            if hops > 255 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            p = (header as *mut libc::c_uchar).offset(l as isize)
        } else if label_type == 0 as libc::c_int as libc::c_uint {
            /* label_type = 0 -> label. */
            namelen =
                namelen.wrapping_add(l.wrapping_add(1 as libc::c_int as
                                                        libc::c_uint)); /* include period */
            if namelen >= MAXDNAME as libc::c_uint { return 0 as libc::c_int }
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + l as libc::c_long) as size_t <= plen) {
                return 0 as libc::c_int
            }
            j = 0 as libc::c_int as libc::c_uint;
            while j < l {
                if isExtract != 0 {
                    let mut c = *p;
                    if c as libc::c_int != 0 as libc::c_int &&
                           c as libc::c_int != '.' as i32 {
                        let fresh8 = cp;
                        cp = cp.offset(1);
                        *fresh8 = c
                    } else { return 0 as libc::c_int }
                } else {
                    let mut c1 = *cp;
                    let mut c2 = *p;
                    if c1 as libc::c_int == 0 as libc::c_int {
                        retvalue = 2 as libc::c_int
                    } else {
                        cp = cp.offset(1);
                        if c1 as libc::c_int >= 'A' as i32 &&
                               c1 as libc::c_int <= 'Z' as i32 {
                            c1 =
                                (c1 as libc::c_int +
                                     ('a' as i32 - 'A' as i32)) as
                                    libc::c_uchar
                        }
                        if c2 as libc::c_int >= 'A' as i32 &&
                               c2 as libc::c_int <= 'Z' as i32 {
                            c2 =
                                (c2 as libc::c_int +
                                     ('a' as i32 - 'A' as i32)) as
                                    libc::c_uchar
                        }
                        if c1 as libc::c_int != c2 as libc::c_int {
                            retvalue = 2 as libc::c_int
                        }
                    }
                }
                j = j.wrapping_add(1);
                p = p.offset(1)
            }
            if isExtract != 0 {
                let fresh9 = cp;
                cp = cp.offset(1);
                *fresh9 = '.' as i32 as libc::c_uchar
            } else if *cp as libc::c_int != 0 as libc::c_int &&
                          {
                              let fresh10 = cp;
                              cp = cp.offset(1);
                              (*fresh10 as libc::c_int) != '.' as i32
                          } {
                retvalue = 2 as libc::c_int
            }
        } else { return 0 as libc::c_int }
    };
}
/* Max size of input string (for IPv6) is 75 chars.) */
#[c2rust::src_loc = "145:9"]
pub const MAXARPANAME: libc::c_int = 75 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn in_arpa_name_2_addr(mut namein: *mut libc::c_char,
                                             mut addrp: *mut all_addr)
 -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut name: [libc::c_char; 76] = [0; 76];
    let mut cp1 = 0 as *mut libc::c_char;
    let mut addr = addrp as *mut libc::c_uchar;
    let mut lastchunk = NULL_0 as *mut libc::c_char;
    let mut penchunk = NULL_0 as *mut libc::c_char;
    if strlen(namein) > MAXARPANAME as libc::c_ulong {
        return 0 as libc::c_int
    }
    memset(addrp as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<all_addr>() as libc::c_ulong);
    /* turn name into a series of asciiz strings */
  /* j counts no. of labels */
    j = 1 as libc::c_int;
    cp1 = name.as_mut_ptr();
    while *namein != 0 {
        if *namein as libc::c_int == '.' as i32 {
            penchunk = lastchunk;
            lastchunk = cp1.offset(1 as libc::c_int as isize);
            *cp1 = 0 as libc::c_int as libc::c_char;
            j += 1
        } else { *cp1 = *namein }
        cp1 = cp1.offset(1);
        namein = namein.offset(1)
    }
    *cp1 = 0 as libc::c_int as libc::c_char;
    if j < 3 as libc::c_int { return 0 as libc::c_int }
    if hostname_isequal(lastchunk,
                        b"arpa\x00" as *const u8 as *const libc::c_char) != 0
           &&
           hostname_isequal(penchunk,
                            b"in-addr\x00" as *const u8 as
                                *const libc::c_char) != 0 {
        /* IP v4 */
      /* address arrives as a name of the form
	 www.xxx.yyy.zzz.in-addr.arpa
	 some of the low order address octets might be missing
	 and should be set to zero. */
        cp1 = name.as_mut_ptr();
        while cp1 != penchunk {
            /* check for digits only (weeds out things like
	     50.0/24.67.28.64.in-addr.arpa which are used 
	     as CNAME targets according to RFC 2317 */
            let mut cp = 0 as *mut libc::c_char;
            cp = cp1;
            while *cp != 0 {
                if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                    return 0 as libc::c_int
                }
                cp = cp.offset(1)
            }
            *addr.offset(3 as libc::c_int as isize) =
                *addr.offset(2 as libc::c_int as isize);
            *addr.offset(2 as libc::c_int as isize) =
                *addr.offset(1 as libc::c_int as isize);
            *addr.offset(1 as libc::c_int as isize) =
                *addr.offset(0 as libc::c_int as isize);
            *addr.offset(0 as libc::c_int as isize) =
                atoi(cp1) as libc::c_uchar;
            cp1 =
                cp1.offset(strlen(cp1).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) as
                               isize)
        }
        return F_IPV4 as libc::c_int
    } else {
        if hostname_isequal(penchunk,
                            b"ip6\x00" as *const u8 as *const libc::c_char) !=
               0 &&
               (hostname_isequal(lastchunk,
                                 b"int\x00" as *const u8 as
                                     *const libc::c_char) != 0 ||
                    hostname_isequal(lastchunk,
                                     b"arpa\x00" as *const u8 as
                                         *const libc::c_char) != 0) {
            /* IP v6:
         Address arrives as 0.1.2.3.4.5.6.7.8.9.a.b.c.d.e.f.ip6.[int|arpa]
    	 or \[xfedcba9876543210fedcba9876543210/128].ip6.[int|arpa]
      
	 Note that most of these the various representations are obsolete and 
	 left-over from the many DNS-for-IPv6 wars. We support all the formats
	 that we can since there is no reason not to.
      */
            if *name.as_mut_ptr() as libc::c_int == '\\' as i32 &&
                   *name.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       libc::c_int == '[' as i32 &&
                   (*name.as_mut_ptr().offset(2 as libc::c_int as isize) as
                        libc::c_int == 'x' as i32 ||
                        *name.as_mut_ptr().offset(2 as libc::c_int as isize)
                            as libc::c_int == 'X' as i32) {
                j = 0 as libc::c_int;
                cp1 = name.as_mut_ptr().offset(3 as libc::c_int as isize);
                while *cp1 as libc::c_int != 0 &&
                          *(*__ctype_b_loc()).offset(*cp1 as libc::c_uchar as
                                                         libc::c_int as isize)
                              as libc::c_int &
                              _ISxdigit as libc::c_int as libc::c_ushort as
                                  libc::c_int != 0 && j < 32 as libc::c_int {
                    let mut xdig: [libc::c_char; 2] = [0; 2];
                    xdig[0 as libc::c_int as usize] = *cp1;
                    xdig[1 as libc::c_int as usize] =
                        0 as libc::c_int as libc::c_char;
                    if j % 2 as libc::c_int != 0 {
                        let ref mut fresh11 =
                            *addr.offset((j / 2 as libc::c_int) as isize);
                        *fresh11 =
                            (*fresh11 as libc::c_long |
                                 strtol(xdig.as_mut_ptr(),
                                        NULL_0 as *mut *mut libc::c_char,
                                        16 as libc::c_int)) as libc::c_uchar
                    } else {
                        *addr.offset((j / 2 as libc::c_int) as isize) =
                            (strtol(xdig.as_mut_ptr(),
                                    NULL_0 as *mut *mut libc::c_char,
                                    16 as libc::c_int) << 4 as libc::c_int) as
                                libc::c_uchar
                    }
                    cp1 = cp1.offset(1);
                    j += 1
                }
                if *cp1 as libc::c_int == '/' as i32 && j == 32 as libc::c_int
                   {
                    return F_IPV6 as libc::c_int
                }
            } else {
                cp1 = name.as_mut_ptr();
                while cp1 != penchunk {
                    if *cp1.offset(1 as libc::c_int as isize) as libc::c_int
                           != 0 ||
                           *(*__ctype_b_loc()).offset(*cp1 as libc::c_uchar as
                                                          libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISxdigit as libc::c_int as libc::c_ushort as
                                   libc::c_int == 0 {
                        return 0 as libc::c_int
                    }
                    j =
                        (::std::mem::size_of::<in6_addr>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                            libc::c_int;
                    while j > 0 as libc::c_int {
                        *addr.offset(j as isize) =
                            (*addr.offset(j as isize) as libc::c_int >>
                                 4 as libc::c_int |
                                 (*addr.offset((j - 1 as libc::c_int) as
                                                   isize) as libc::c_int) <<
                                     4 as libc::c_int) as libc::c_uchar;
                        j -= 1
                    }
                    *addr.offset(0 as libc::c_int as isize) =
                        ((*addr.offset(0 as libc::c_int as isize) as
                              libc::c_int >> 4 as libc::c_int) as libc::c_long
                             |
                             strtol(cp1, NULL_0 as *mut *mut libc::c_char,
                                    16 as libc::c_int) << 4 as libc::c_int) as
                            libc::c_uchar;
                    cp1 =
                        cp1.offset(strlen(cp1).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                                       as isize)
                }
                return F_IPV6 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "249:1"]
pub unsafe extern "C" fn skip_name(mut ansp: *mut libc::c_uchar,
                                   mut header: *mut dns_header,
                                   mut plen: size_t,
                                   mut extrabytes: libc::c_int)
 -> *mut libc::c_uchar {
    loop  {
        let mut label_type: libc::c_uint = 0;
        if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + 1 as libc::c_int as libc::c_long) as size_t
                 <= plen) {
            return NULL_0 as *mut libc::c_uchar
        }
        label_type =
            (*ansp as libc::c_int & 0xc0 as libc::c_int) as libc::c_uint;
        if label_type == 0xc0 as libc::c_int as libc::c_uint {
            /* pointer for compression. */
            ansp = ansp.offset(2 as libc::c_int as isize); /* reserved */
            break ;
        } else if label_type == 0x80 as libc::c_int as libc::c_uint {
            return NULL_0 as *mut libc::c_uchar
        } else if label_type == 0x40 as libc::c_int as libc::c_uint {
            /* Extended label type */
            let mut count: libc::c_uint =
                0; /* we only understand bitstrings */
            if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 2 as libc::c_int as libc::c_long) as
                     size_t <= plen) {
                return NULL_0 as *mut libc::c_uchar
            } /* Bits in bitstring */
            let fresh12 = ansp;
            ansp = ansp.offset(1);
            if *fresh12 as libc::c_int & 0x3f as libc::c_int !=
                   1 as libc::c_int {
                return NULL_0 as *mut libc::c_uchar
            }
            let fresh13 = ansp;
            ansp = ansp.offset(1);
            count = *fresh13 as libc::c_uint;
            if count == 0 as libc::c_int as libc::c_uint {
                /* count == 0 means 256 bits */
                ansp = ansp.offset(32 as libc::c_int as isize)
            } else {
                ansp =
                    ansp.offset((count.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) >>
                                     3 as
                                         libc::c_int).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                    as isize)
            }
        } else {
            /* label type == 0 Bottom six bits is length */
            let fresh14 = ansp;
            ansp = ansp.offset(1);
            let mut len =
                (*fresh14 as libc::c_int & 0x3f as libc::c_int) as
                    libc::c_uint;
            if if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar)
                         as libc::c_long + len as libc::c_long) as size_t <=
                        plen) {
                   0 as libc::c_int
               } else { ansp = ansp.offset(len as isize); 1 as libc::c_int }
                   == 0 {
                return NULL_0 as *mut libc::c_uchar
            }
            if len == 0 as libc::c_int as libc::c_uint { break ; }
            /* zero length label marks the end. */
        }
    }
    if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
              libc::c_long + extrabytes as libc::c_long) as size_t <= plen) {
        return NULL_0 as *mut libc::c_uchar
    }
    return ansp;
}
#[no_mangle]
#[c2rust::src_loc = "304:1"]
pub unsafe extern "C" fn skip_questions(mut header: *mut dns_header,
                                        mut plen: size_t)
 -> *mut libc::c_uchar {
    let mut q: libc::c_int = 0;
    let mut ansp =
        header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    q = __bswap_16((*header).qdcount) as libc::c_int;
    while q != 0 as libc::c_int {
        ansp = skip_name(ansp, header, plen, 4 as libc::c_int);
        if ansp.is_null() { return NULL_0 as *mut libc::c_uchar }
        ansp = ansp.offset(4 as libc::c_int as isize);
        q -= 1
        /* class and type */
    } /* type, class, TTL */
    return ansp;
}
#[no_mangle]
#[c2rust::src_loc = "319:1"]
pub unsafe extern "C" fn skip_section(mut ansp: *mut libc::c_uchar,
                                      mut count: libc::c_int,
                                      mut header: *mut dns_header,
                                      mut plen: size_t)
 -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        ansp = skip_name(ansp, header, plen, 10 as libc::c_int);
        if ansp.is_null() { return NULL_0 as *mut libc::c_uchar }
        ansp = ansp.offset(8 as libc::c_int as isize);
        let mut t_cp = ansp;
        rdlen =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        ansp = ansp.offset(2 as libc::c_int as isize);
        if if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= plen)
              {
               0 as libc::c_int
           } else { ansp = ansp.offset(rdlen as isize); 1 as libc::c_int } ==
               0 {
            return NULL_0 as *mut libc::c_uchar
        }
        i += 1
    }
    return ansp;
}
#[no_mangle]
#[c2rust::src_loc = "336:1"]
pub unsafe extern "C" fn resize_packet(mut header: *mut dns_header,
                                       mut plen: size_t,
                                       mut pheader: *mut libc::c_uchar,
                                       mut hlen: size_t) -> size_t {
    let mut ansp = skip_questions(header, plen);
    /* if packet is malformed, just return as-is. */
    if ansp.is_null() { return plen }
    ansp =
        skip_section(ansp,
                     __bswap_16((*header).ancount) as libc::c_int +
                         __bswap_16((*header).nscount) as libc::c_int +
                         __bswap_16((*header).arcount) as libc::c_int, header,
                     plen);
    if ansp.is_null() { return plen }
    /* restore pseudoheader */
    if !pheader.is_null() &&
           __bswap_16((*header).arcount) as libc::c_int == 0 as libc::c_int {
        /* must use memmove, may overlap */
        memmove(ansp as *mut libc::c_void, pheader as *const libc::c_void,
                hlen);
        (*header).arcount = __bswap_16(1 as libc::c_int as __uint16_t);
        ansp = ansp.offset(hlen as isize)
    }
    return ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
/* is addr in the non-globally-routed IP space? */
#[no_mangle]
#[c2rust::src_loc = "361:1"]
pub unsafe extern "C" fn private_net(mut addr: in_addr,
                                     mut ban_localhost: libc::c_int)
 -> libc::c_int {
    let mut ip_addr = __bswap_32(addr.s_addr);
    return (ip_addr & 0xff000000 as libc::c_uint ==
                0x7f000000 as libc::c_int as libc::c_uint &&
                ban_localhost != 0 ||
                ip_addr & 0xff000000 as libc::c_uint ==
                    0 as libc::c_int as libc::c_uint ||
                ip_addr & 0xff000000 as libc::c_uint ==
                    0xa000000 as libc::c_int as libc::c_uint ||
                ip_addr & 0xfff00000 as libc::c_uint ==
                    0xac100000 as libc::c_uint ||
                ip_addr & 0xffff0000 as libc::c_uint ==
                    0xc0a80000 as libc::c_uint ||
                ip_addr & 0xffff0000 as libc::c_uint ==
                    0xa9fe0000 as libc::c_uint ||
                ip_addr & 0xffffff00 as libc::c_uint ==
                    0xc0000200 as libc::c_uint ||
                ip_addr & 0xffffff00 as libc::c_uint ==
                    0xc6336400 as libc::c_uint ||
                ip_addr & 0xffffff00 as libc::c_uint ==
                    0xcb007100 as libc::c_uint ||
                ip_addr & 0xffffffff as libc::c_uint ==
                    0xffffffff as libc::c_uint) as libc::c_int;
    /* 255.255.255.255/32 (broadcast)*/
}
#[c2rust::src_loc = "378:1"]
unsafe extern "C" fn private_net6(mut a: *mut in6_addr) -> libc::c_int {
    return (({
                 let mut __a = a as *const in6_addr;
                 ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                      0 as libc::c_int as libc::c_uint &&
                      (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                      (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                      (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint) as libc::c_int
             }) != 0 ||
                ({
                     let mut __a = a as *const in6_addr;
                     ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                         usize] ==
                              __bswap_32(1 as libc::c_int as __uint32_t)) as
                         libc::c_int
                 }) != 0 ||
                ({
                     let mut __a = a as *const in6_addr;
                     ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                          __bswap_32(0xffc00000 as libc::c_uint) ==
                          __bswap_32(0xfe800000 as libc::c_uint)) as
                         libc::c_int
                 }) != 0 ||
                *(a as *mut libc::c_uchar).offset(0 as libc::c_int as isize)
                    as libc::c_int == 0xfd as libc::c_int ||
                *(a as *mut u32_0).offset(0 as libc::c_int as isize) ==
                    __bswap_32(0x20010db8 as libc::c_int as __uint32_t)) as
               libc::c_int;
    /* RFC 6303 4.6 */
}
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn do_doctor(mut p: *mut libc::c_uchar,
                               mut count: libc::c_int,
                               mut header: *mut dns_header, mut qlen: size_t,
                               mut name: *mut libc::c_char,
                               mut doctored: *mut libc::c_int)
 -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0; /* bad packet */
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    i = count;
    while i != 0 as libc::c_int {
        if !name.is_null() &&
               (*dnsmasq_daemon).options[(2 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (2 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
            if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                            10 as libc::c_int) == 0 {
                return 0 as *mut libc::c_uchar
            }
        } else {
            p = skip_name(p, header, qlen, 10 as libc::c_int);
            if p.is_null() { return 0 as *mut libc::c_uchar }
        }
        let mut t_cp = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0 = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        /* bad packet */
        p = p.offset(4 as libc::c_int as isize); /* ttl */
        let mut t_cp_1 = p;
        rdlen =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == C_IN && qtype == T_A {
            let mut doctor = 0 as *mut doctor;
            let mut addr = in_addr{s_addr: 0,};
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 4 as libc::c_int as libc::c_long) as
                     size_t <= qlen) {
                return 0 as *mut libc::c_uchar
            }
            /* alignment */
            memcpy(&mut addr as *mut in_addr as *mut libc::c_void,
                   p as *const libc::c_void, INADDRSZ as libc::c_ulong);
            let mut current_block_28: u64;
            doctor = (*dnsmasq_daemon).doctors;
            while !doctor.is_null() {
                if (*doctor).end.s_addr == 0 as libc::c_int as libc::c_uint {
                    if is_same_net((*doctor).in_0, addr, (*doctor).mask) == 0
                       {
                        current_block_28 = 6669252993407410313;
                    } else { current_block_28 = 11636175345244025579; }
                } else if __bswap_32((*doctor).in_0.s_addr) >
                              __bswap_32(addr.s_addr) ||
                              __bswap_32((*doctor).end.s_addr) <
                                  __bswap_32(addr.s_addr) {
                    current_block_28 = 6669252993407410313;
                } else { current_block_28 = 11636175345244025579; }
                match current_block_28 {
                    6669252993407410313 => { doctor = (*doctor).next }
                    _ => {
                        addr.s_addr &= !(*doctor).mask.s_addr;
                        addr.s_addr |=
                            (*doctor).out.s_addr & (*doctor).mask.s_addr;
                        /* Since we munged the data, the server it came from is no longer authoritative */
                        (*header).hb3 =
                            ((*header).hb3 as libc::c_int & !HB3_AA) as
                                u8_0; /* bad packet */
                        *doctored = 1 as libc::c_int;
                        memcpy(p as *mut libc::c_void,
                               &mut addr as *mut in_addr as
                                   *const libc::c_void,
                               INADDRSZ as libc::c_ulong);
                        break ;
                    }
                }
            }
        } else if qtype == T_TXT && !name.is_null() &&
                      (*dnsmasq_daemon).options[(2 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (2 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          != 0 {
            let mut p1 = p;
            if !((p1.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
               {
                return 0 as *mut libc::c_uchar
            }
            while (p1.wrapping_offset_from(p) as libc::c_long) <
                      rdlen as libc::c_long {
                let mut i_0: libc::c_uint = 0;
                let mut len = *p1 as libc::c_uint;
                let mut p2 = p1;
                if p1.offset(len as isize).wrapping_offset_from(p) as
                       libc::c_long >= rdlen as libc::c_long {
                    return 0 as *mut libc::c_uchar
                }
                /* make counted string zero-term  and sanitise */
                i_0 = 0 as libc::c_int as libc::c_uint;
                while i_0 < len {
                    if *(*__ctype_b_loc()).offset(*p2.offset(1 as libc::c_int
                                                                 as isize) as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISprint as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        break ;
                    }
                    *p2 = *p2.offset(1 as libc::c_int as isize);
                    p2 = p2.offset(1);
                    i_0 = i_0.wrapping_add(1)
                }
                *p2 = 0 as libc::c_int as libc::c_uchar;
                my_syslog(LOG_INFO,
                          b"reply %s is %s\x00" as *const u8 as
                              *const libc::c_char, name, p1);
                /* restore */
                memmove(p1.offset(1 as libc::c_int as isize) as
                            *mut libc::c_void, p1 as *const libc::c_void,
                        i_0 as libc::c_ulong);
                *p1 = len as libc::c_uchar;
                p1 =
                    p1.offset(len.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as isize)
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as *mut libc::c_uchar
        }
        i -= 1
    }
    return p;
}
#[c2rust::src_loc = "474:1"]
unsafe extern "C" fn find_soa(mut header: *mut dns_header, mut qlen: size_t,
                              mut name: *mut libc::c_char,
                              mut doctored: *mut libc::c_int) -> libc::c_int {
    let mut p = 0 as *mut libc::c_uchar;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut ttl: libc::c_ulong = 0;
    let mut minttl =
        (__LONG_MAX__ as
             libc::c_ulong).wrapping_mul(2 as
                                             libc::c_ulong).wrapping_add(1 as
                                                                             libc::c_ulong);
    let mut i: libc::c_int = 0;
    let mut found_soa = 0 as libc::c_int;
    /* first move to NS section and find TTL from any SOA section */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() ||
           {
               p =
                   do_doctor(p, __bswap_16((*header).ancount) as libc::c_int,
                             header, qlen, name, doctored); /* bad packet */
               p.is_null()
           } {
        return 0 as libc::c_int
    }
    i = __bswap_16((*header).nscount) as libc::c_int;
    while i != 0 as libc::c_int {
        p = skip_name(p, header, qlen, 10 as libc::c_int);
        if p.is_null() { return 0 as libc::c_int }
        let mut t_cp = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0 = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_1 = p;
        ttl =
            ((*t_cp_1.offset(0 as libc::c_int as isize) as u32_0) <<
                 24 as libc::c_int |
                 (*t_cp_1.offset(1 as libc::c_int as isize) as u32_0) <<
                     16 as libc::c_int |
                 (*t_cp_1.offset(2 as libc::c_int as isize) as u32_0) <<
                     8 as libc::c_int |
                 *t_cp_1.offset(3 as libc::c_int as isize) as u32_0) as
                libc::c_ulong;
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_2 = p;
        rdlen =
            (*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == C_IN && qtype == T_SOA {
            found_soa = 1 as libc::c_int;
            if ttl < minttl { minttl = ttl }
            /* bad packet */
            /* MNAME */
            p = skip_name(p, header, qlen, 0 as libc::c_int);
            if p.is_null() { return 0 as libc::c_int }
            /* RNAME */
            p =
                skip_name(p, header, qlen,
                          20 as
                              libc::c_int); /* SERIAL REFRESH RETRY EXPIRE */
            if p.is_null() { return 0 as libc::c_int }
            p = p.offset(16 as libc::c_int as isize);
            let mut t_cp_3 = p;
            ttl =
                ((*t_cp_3.offset(0 as libc::c_int as isize) as u32_0) <<
                     24 as libc::c_int |
                     (*t_cp_3.offset(1 as libc::c_int as isize) as u32_0) <<
                         16 as libc::c_int |
                     (*t_cp_3.offset(2 as libc::c_int as isize) as u32_0) <<
                         8 as libc::c_int |
                     *t_cp_3.offset(3 as libc::c_int as isize) as u32_0) as
                    libc::c_ulong;
            p = p.offset(4 as libc::c_int as isize);
            /* minTTL */
            if ttl < minttl { minttl = ttl }
        } else if if !((p.wrapping_offset_from(header as *mut libc::c_uchar)
                            as libc::c_long + rdlen as libc::c_long) as size_t
                           <= qlen) {
                      0 as libc::c_int
                  } else { p = p.offset(rdlen as isize); 1 as libc::c_int } ==
                      0 {
            return 0 as libc::c_int
        }
        i -= 1
    }
    /* rewrite addresses in additional section too */
    if do_doctor(p, __bswap_16((*header).arcount) as libc::c_int, header,
                 qlen, NULL_0 as *mut libc::c_char, doctored).is_null() {
        return 0 as libc::c_int
    }
    if found_soa == 0 { minttl = (*dnsmasq_daemon).neg_ttl }
    return minttl as libc::c_int;
}
/* Note that the following code can create CNAME chains that don't point to a real record,
   either because of lack of memory, or lack of SOA records.  These are treated by the cache code as 
   expired and cleaned out that way. 
   Return 1 if we reject an address because it look like part of dns-rebinding attack. */
#[no_mangle]
#[c2rust::src_loc = "532:1"]
pub unsafe extern "C" fn extract_addresses(mut header: *mut dns_header,
                                           mut qlen: size_t,
                                           mut name: *mut libc::c_char,
                                           mut now: time_t,
                                           mut ipsets: *mut *mut libc::c_char,
                                           mut is_sign: libc::c_int,
                                           mut check_rebind: libc::c_int,
                                           mut no_cache_dnssec: libc::c_int,
                                           mut secure: libc::c_int,
                                           mut doctored: *mut libc::c_int)
 -> libc::c_int {
    let mut p = 0 as *mut libc::c_uchar;
    let mut p1 = 0 as *mut libc::c_uchar;
    let mut endrr = 0 as *mut libc::c_uchar;
    let mut namep = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut aqtype: libc::c_int = 0;
    let mut aqclass: libc::c_int = 0;
    let mut ardlen: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut searched_soa = 0 as libc::c_int;
    let mut ttl = 0 as libc::c_int as libc::c_ulong;
    let mut addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut ipsets_cur = 0 as *mut *mut libc::c_char;
    cache_start_insert();
    /* find_soa is needed for dns_doctor and logging side-effects, so don't call it lazily if there are any. */
    if !(*dnsmasq_daemon).doctors.is_null() ||
           (*dnsmasq_daemon).options[(2 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (2 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 ||
           (*dnsmasq_daemon).options[(45 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (45 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        searched_soa = 1 as libc::c_int;
        ttl = find_soa(header, qlen, name, doctored) as libc::c_ulong;
        if *doctored != 0 { if secure != 0 { return 0 as libc::c_int } }
    }
    /* go through the questions. */
    p =
        header.offset(1 as libc::c_int as isize) as
            *mut libc::c_uchar; /* bad packet */
    let mut current_block_206: u64;
    i = __bswap_16((*header).qdcount) as libc::c_int;
    while i != 0 as libc::c_int {
        let mut found = 0 as libc::c_int;
        let mut cname_count = CNAME_CHAIN;
        let mut cpp = NULL_0 as *mut crec;
        let mut flags =
            if (*header).hb4 as libc::c_int & HB4_RCODE == NXDOMAIN {
                F_NXDOMAIN
            } else { 0 as libc::c_int as libc::c_uint } as libc::c_int;
        let mut cttl =
            (__LONG_MAX__ as
                 libc::c_ulong).wrapping_mul(2 as
                                                 libc::c_ulong).wrapping_add(1
                                                                                 as
                                                                                 libc::c_ulong);
        let mut attl: libc::c_ulong = 0;
        namep = p;
        if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                        4 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        let mut t_cp = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0 = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if !(qclass != C_IN) {
            /* PTRs: we chase CNAMEs here, since we have no way to 
	 represent them in the cache. */
            if qtype == T_PTR {
                let mut name_encoding = in_arpa_name_2_addr(name, &mut addr);
                if !(name_encoding == 0) {
                    if flags as libc::c_uint & F_NXDOMAIN == 0 {
                        'c_14031:
                            loop  {
                                p1 = skip_questions(header, qlen);
                                if p1.is_null() { return 0 as libc::c_int }
                                j = 0 as libc::c_int;
                                loop  {
                                    if !(j <
                                             __bswap_16((*header).ancount) as
                                                 libc::c_int) {
                                        break 'c_14031 ;
                                    }
                                    let mut secflag = 0 as libc::c_int;
                                    let mut tmp = namep;
                                    /* bad packet */
                                    /* the loop body overwrites the original name, so get it back here. */
                                    if extract_name(header, qlen, &mut tmp,
                                                    name, 1 as libc::c_int,
                                                    0 as libc::c_int) == 0 ||
                                           {
                                               res =
                                                   extract_name(header, qlen,
                                                                &mut p1, name,
                                                                0 as
                                                                    libc::c_int,
                                                                10 as
                                                                    libc::c_int); /* bad packet */
                                               (res) == 0
                                           } {
                                        return 0 as libc::c_int
                                    }
                                    let mut t_cp_1 = p1;
                                    aqtype =
                                        (*t_cp_1.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_1.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_2 = p1;
                                    aqclass =
                                        (*t_cp_2.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_2.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_3 = p1;
                                    attl =
                                        ((*t_cp_3.offset(0 as libc::c_int as
                                                             isize) as u32_0)
                                             << 24 as libc::c_int |
                                             (*t_cp_3.offset(1 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 16 as libc::c_int
                                             |
                                             (*t_cp_3.offset(2 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 8 as libc::c_int |
                                             *t_cp_3.offset(3 as libc::c_int
                                                                as isize) as
                                                 u32_0) as libc::c_ulong;
                                    p1 = p1.offset(4 as libc::c_int as isize);
                                    if (*dnsmasq_daemon).max_ttl !=
                                           0 as libc::c_int as libc::c_ulong
                                           && attl > (*dnsmasq_daemon).max_ttl
                                           && is_sign == 0 {
                                        p1 =
                                            p1.offset(-(4 as libc::c_int as
                                                            isize));
                                        let mut t_l =
                                            (*dnsmasq_daemon).max_ttl as
                                                u32_0;
                                        let mut t_cp_4 = p1;
                                        let fresh15 = t_cp_4;
                                        t_cp_4 = t_cp_4.offset(1);
                                        *fresh15 =
                                            (t_l >> 24 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh16 = t_cp_4;
                                        t_cp_4 = t_cp_4.offset(1);
                                        *fresh16 =
                                            (t_l >> 16 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh17 = t_cp_4;
                                        t_cp_4 = t_cp_4.offset(1);
                                        *fresh17 =
                                            (t_l >> 8 as libc::c_int) as
                                                libc::c_uchar;
                                        *t_cp_4 = t_l as libc::c_uchar;
                                        p1 =
                                            p1.offset(4 as libc::c_int as
                                                          isize)
                                    }
                                    let mut t_cp_5 = p1;
                                    ardlen =
                                        (*t_cp_5.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_5.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    endrr = p1.offset(ardlen as isize);
                                    /* TTL of record is minimum of CNAMES and PTR */
                                    if attl < cttl {
                                        cttl = attl
                                    } /* looped CNAMES, we can't cache. */
                                    if aqclass == C_IN &&
                                           res != 2 as libc::c_int &&
                                           (aqtype == T_CNAME ||
                                                aqtype == T_PTR) {
                                        if extract_name(header, qlen, &mut p1,
                                                        name,
                                                        1 as libc::c_int,
                                                        0 as libc::c_int) == 0
                                           {
                                            return 0 as libc::c_int
                                        }
                                        if aqtype == T_CNAME {
                                            let fresh18 = cname_count;
                                            cname_count = cname_count - 1;
                                            if fresh18 == 0 {
                                                return 0 as libc::c_int
                                            }
                                            break ;
                                        } else {
                                            cache_insert(name, &mut addr,
                                                         C_IN as
                                                             libc::c_ushort,
                                                         now, cttl,
                                                         (name_encoding |
                                                              secflag) as
                                                             libc::c_uint |
                                                             F_REVERSE);
                                            found = 1 as libc::c_int
                                        }
                                    }
                                    p1 = endrr;
                                    if !((p1.wrapping_offset_from(header as
                                                                      *mut libc::c_uchar)
                                              as libc::c_long +
                                              0 as libc::c_int as
                                                  libc::c_long) as size_t <=
                                             qlen) {
                                        return 0 as libc::c_int
                                    }
                                    j += 1
                                }
                            }
                    }
                    if found == 0 &&
                           (*dnsmasq_daemon).options[(11 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (11 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 {
                        if searched_soa == 0 {
                            searched_soa = 1 as libc::c_int;
                            ttl =
                                find_soa(header, qlen,
                                         NULL_0 as *mut libc::c_char,
                                         doctored) as libc::c_ulong
                        }
                        if ttl != 0 {
                            cache_insert(NULL_0 as *mut libc::c_char,
                                         &mut addr, C_IN as libc::c_ushort,
                                         now, ttl,
                                         name_encoding as libc::c_uint |
                                             F_REVERSE | F_NEG |
                                             flags as libc::c_uint |
                                             (if secure != 0 {
                                                  F_DNSSECOK
                                              } else {
                                                  0 as libc::c_int as
                                                      libc::c_uint
                                              }));
                        }
                    }
                }
            } else {
                /* everything other than PTR */
                let mut newc = 0 as *mut crec;
                let mut addrlen = 0 as libc::c_int;
                if qtype == T_A {
                    addrlen = INADDRSZ;
                    flags = (flags as libc::c_uint | F_IPV4) as libc::c_int;
                    current_block_206 = 18356193971123529525;
                } else if qtype == T_AAAA {
                    addrlen = IN6ADDRSZ;
                    flags = (flags as libc::c_uint | F_IPV6) as libc::c_int;
                    current_block_206 = 18356193971123529525;
                } else if qtype == T_SRV {
                    flags = (flags as libc::c_uint | F_SRV) as libc::c_int;
                    current_block_206 = 18356193971123529525;
                } else { current_block_206 = 1856101646708284338; }
                match current_block_206 {
                    1856101646708284338 => { }
                    _ => {
                        'c_10467:
                            loop  {
                                p1 = skip_questions(header, qlen);
                                if p1.is_null() { return 0 as libc::c_int }
                                j = 0 as libc::c_int;
                                loop  {
                                    if !(j <
                                             __bswap_16((*header).ancount) as
                                                 libc::c_int) {
                                        break 'c_10467 ;
                                    }
                                    let mut secflag_0 = 0 as libc::c_int;
                                    /* bad packet */
                                    res =
                                        extract_name(header, qlen, &mut p1,
                                                     name, 0 as libc::c_int,
                                                     10 as
                                                         libc::c_int); /* bad packet */
                                    if res == 0 {
                                        return 0 as libc::c_int
                                    } /* looped CNAMES */
                                    let mut t_cp_6 = p1; /* bad packet */
                                    aqtype =
                                        (*t_cp_6.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_6.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as
                                                libc::c_int; /* include terminating zero */
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_7 = p1;
                                    aqclass =
                                        (*t_cp_7.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_7.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_8 = p1;
                                    attl =
                                        ((*t_cp_8.offset(0 as libc::c_int as
                                                             isize) as u32_0)
                                             << 24 as libc::c_int |
                                             (*t_cp_8.offset(1 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 16 as libc::c_int
                                             |
                                             (*t_cp_8.offset(2 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 8 as libc::c_int |
                                             *t_cp_8.offset(3 as libc::c_int
                                                                as isize) as
                                                 u32_0) as libc::c_ulong;
                                    p1 = p1.offset(4 as libc::c_int as isize);
                                    if (*dnsmasq_daemon).max_ttl !=
                                           0 as libc::c_int as libc::c_ulong
                                           && attl > (*dnsmasq_daemon).max_ttl
                                           && is_sign == 0 {
                                        p1 =
                                            p1.offset(-(4 as libc::c_int as
                                                            isize));
                                        let mut t_l_0 =
                                            (*dnsmasq_daemon).max_ttl as
                                                u32_0;
                                        let mut t_cp_9 = p1;
                                        let fresh19 = t_cp_9;
                                        t_cp_9 = t_cp_9.offset(1);
                                        *fresh19 =
                                            (t_l_0 >> 24 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh20 = t_cp_9;
                                        t_cp_9 = t_cp_9.offset(1);
                                        *fresh20 =
                                            (t_l_0 >> 16 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh21 = t_cp_9;
                                        t_cp_9 = t_cp_9.offset(1);
                                        *fresh21 =
                                            (t_l_0 >> 8 as libc::c_int) as
                                                libc::c_uchar;
                                        *t_cp_9 = t_l_0 as libc::c_uchar;
                                        p1 =
                                            p1.offset(4 as libc::c_int as
                                                          isize)
                                    }
                                    let mut t_cp_10 = p1;
                                    ardlen =
                                        (*t_cp_10.offset(0 as libc::c_int as
                                                             isize) as u16_0
                                             as libc::c_int) <<
                                            8 as libc::c_int |
                                            *t_cp_10.offset(1 as libc::c_int
                                                                as isize) as
                                                u16_0 as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    endrr = p1.offset(ardlen as isize);
                                    if aqclass == C_IN &&
                                           res != 2 as libc::c_int &&
                                           (aqtype == T_CNAME ||
                                                aqtype == qtype) {
                                        if aqtype == T_CNAME {
                                            let fresh22 = cname_count;
                                            cname_count = cname_count - 1;
                                            if fresh22 == 0 {
                                                return 0 as libc::c_int
                                            }
                                            newc =
                                                cache_insert(name,
                                                             NULL_0 as
                                                                 *mut all_addr,
                                                             C_IN as
                                                                 libc::c_ushort,
                                                             now, attl,
                                                             F_CNAME |
                                                                 F_FORWARD |
                                                                 secflag_0 as
                                                                     libc::c_uint);
                                            if !newc.is_null() {
                                                (*newc).addr.cname.target.cache
                                                    = NULL_0 as *mut crec;
                                                (*newc).addr.cname.is_name_ptr
                                                    = 0 as libc::c_int;
                                                if !cpp.is_null() {
                                                    next_uid(newc);
                                                    (*cpp).addr.cname.target.cache
                                                        = newc;
                                                    (*cpp).addr.cname.uid =
                                                        (*newc).uid
                                                }
                                            }
                                            cpp = newc;
                                            if attl < cttl { cttl = attl }
                                            namep = p1;
                                            if extract_name(header, qlen,
                                                            &mut p1, name,
                                                            1 as libc::c_int,
                                                            0 as libc::c_int)
                                                   == 0 {
                                                return 0 as libc::c_int
                                            }
                                            break ;
                                        } else if flags as libc::c_uint &
                                                      F_NXDOMAIN == 0 {
                                            found = 1 as libc::c_int;
                                            if flags as libc::c_uint & F_SRV
                                                   != 0 {
                                                let mut tmp_0 = namep;
                                                if !((p1.wrapping_offset_from(header
                                                                                  as
                                                                                  *mut libc::c_uchar)
                                                          as libc::c_long +
                                                          6 as libc::c_int as
                                                              libc::c_long) as
                                                         size_t <= qlen) {
                                                    return 0 as libc::c_int
                                                }
                                                let mut t_cp_11 = p1;
                                                addr.srv.priority =
                                                    ((*t_cp_11.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                          as u16_0 as
                                                          libc::c_int) <<
                                                         8 as libc::c_int |
                                                         *t_cp_11.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                             as u16_0 as
                                                             libc::c_int) as
                                                        libc::c_ushort;
                                                p1 =
                                                    p1.offset(2 as libc::c_int
                                                                  as isize);
                                                let mut t_cp_12 = p1;
                                                addr.srv.weight =
                                                    ((*t_cp_12.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                          as u16_0 as
                                                          libc::c_int) <<
                                                         8 as libc::c_int |
                                                         *t_cp_12.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                             as u16_0 as
                                                             libc::c_int) as
                                                        libc::c_ushort;
                                                p1 =
                                                    p1.offset(2 as libc::c_int
                                                                  as isize);
                                                let mut t_cp_13 = p1;
                                                addr.srv.srvport =
                                                    ((*t_cp_13.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                          as u16_0 as
                                                          libc::c_int) <<
                                                         8 as libc::c_int |
                                                         *t_cp_13.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                             as u16_0 as
                                                             libc::c_int) as
                                                        libc::c_ushort;
                                                p1 =
                                                    p1.offset(2 as libc::c_int
                                                                  as isize);
                                                if extract_name(header, qlen,
                                                                &mut p1, name,
                                                                1 as
                                                                    libc::c_int,
                                                                0 as
                                                                    libc::c_int)
                                                       == 0 {
                                                    return 0 as libc::c_int
                                                }
                                                addr.srv.targetlen =
                                                    strlen(name).wrapping_add(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                                                        as libc::c_ushort;
                                                addr.srv.target =
                                                    blockdata_alloc(name,
                                                                    addr.srv.targetlen
                                                                        as
                                                                        size_t);
                                                if addr.srv.target.is_null() {
                                                    return 0 as libc::c_int
                                                }
                                                /* we overwrote the original name, so get it back here. */
                                                if extract_name(header, qlen,
                                                                &mut tmp_0,
                                                                name,
                                                                1 as
                                                                    libc::c_int,
                                                                0 as
                                                                    libc::c_int)
                                                       == 0 {
                                                    return 0 as libc::c_int
                                                }
                                            } else {
                                                /* copy address into aligned storage */
                                                if !((p1.wrapping_offset_from(header
                                                                                  as
                                                                                  *mut libc::c_uchar)
                                                          as libc::c_long +
                                                          addrlen as
                                                              libc::c_long) as
                                                         size_t <= qlen) {
                                                    return 0 as libc::c_int
                                                } /* bad packet */
                                                memcpy(&mut addr as
                                                           *mut all_addr as
                                                           *mut libc::c_void,
                                                       p1 as
                                                           *const libc::c_void,
                                                       addrlen as
                                                           libc::c_ulong);
                                                /* check for returned address in private space */
                                                if check_rebind != 0 {
                                                    if flags as libc::c_uint &
                                                           F_IPV4 != 0 &&
                                                           private_net(addr.addr4,
                                                                       ((*dnsmasq_daemon).options[(25
                                                                                                       as
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
                                                                            (1
                                                                                 as
                                                                                 libc::c_uint)
                                                                                <<
                                                                                (25
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong))
                                                                            ==
                                                                            0)
                                                                           as
                                                                           libc::c_int)
                                                               != 0 {
                                                        return 1 as
                                                                   libc::c_int
                                                    }
                                                    /* Block IPv4-mapped IPv6 addresses in private IPv4 address space */
                                                    if flags as libc::c_uint &
                                                           F_IPV6 != 0 {
                                                        if ({
                                                                let mut __a =
                                                                    &mut addr.addr6
                                                                        as
                                                                        *mut in6_addr
                                                                        as
                                                                        *const in6_addr;
                                                                ((*__a).__in6_u.__u6_addr32[0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                     ==
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint
                                                                     &&
                                                                     (*__a).__in6_u.__u6_addr32[1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         ==
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint
                                                                     &&
                                                                     (*__a).__in6_u.__u6_addr32[2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         ==
                                                                         __bswap_32(0xffff
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        __uint32_t))
                                                                    as
                                                                    libc::c_int
                                                            }) != 0 {
                                                            let mut v4 =
                                                                in_addr{s_addr:
                                                                            0,};
                                                            v4.s_addr =
                                                                *(&mut addr.addr6
                                                                      as
                                                                      *mut in6_addr
                                                                      as
                                                                      *const uint32_t).offset(3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize);
                                                            if private_net(v4,
                                                                           ((*dnsmasq_daemon).options[(25
                                                                                                           as
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
                                                                                (1
                                                                                     as
                                                                                     libc::c_uint)
                                                                                    <<
                                                                                    (25
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong))
                                                                                ==
                                                                                0)
                                                                               as
                                                                               libc::c_int)
                                                                   != 0 {
                                                                return 1 as
                                                                           libc::c_int
                                                            }
                                                        }
                                                        /* Check for link-local (LL) and site-local (ULA) IPv6 addresses */
                                                        if ({
                                                                let mut __a =
                                                                    &mut addr.addr6
                                                                        as
                                                                        *mut in6_addr
                                                                        as
                                                                        *const in6_addr;
                                                                ((*__a).__in6_u.__u6_addr32[0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                     &
                                                                     __bswap_32(0xffc00000
                                                                                    as
                                                                                    libc::c_uint)
                                                                     ==
                                                                     __bswap_32(0xfe800000
                                                                                    as
                                                                                    libc::c_uint))
                                                                    as
                                                                    libc::c_int
                                                            }) != 0 ||
                                                               ({
                                                                    let mut __a =
                                                                        &mut addr.addr6
                                                                            as
                                                                            *mut in6_addr
                                                                            as
                                                                            *const in6_addr;
                                                                    ((*__a).__in6_u.__u6_addr32[0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         &
                                                                         __bswap_32(0xffc00000
                                                                                        as
                                                                                        libc::c_uint)
                                                                         ==
                                                                         __bswap_32(0xfec00000
                                                                                        as
                                                                                        libc::c_uint))
                                                                        as
                                                                        libc::c_int
                                                                }) != 0 {
                                                            return 1 as
                                                                       libc::c_int
                                                        }
                                                        /* Check for the IPv6 loopback address (::1) when
				     option rebind-localhost-ok is NOT set */
                                                        if (*dnsmasq_daemon).options[(25
                                                                                          as
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
                                                               (1 as
                                                                    libc::c_uint)
                                                                   <<
                                                                   (25 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_ulong))
                                                               == 0 &&
                                                               ({
                                                                    let mut __a =
                                                                        &mut addr.addr6
                                                                            as
                                                                            *mut in6_addr
                                                                            as
                                                                            *const in6_addr;
                                                                    ((*__a).__in6_u.__u6_addr32[0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         ==
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint
                                                                         &&
                                                                         (*__a).__in6_u.__u6_addr32[1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                                                                             ==
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint
                                                                         &&
                                                                         (*__a).__in6_u.__u6_addr32[2
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                                                                             ==
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint
                                                                         &&
                                                                         (*__a).__in6_u.__u6_addr32[3
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                                                                             ==
                                                                             __bswap_32(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            __uint32_t))
                                                                        as
                                                                        libc::c_int
                                                                }) != 0 {
                                                            return 1 as
                                                                       libc::c_int
                                                        }
                                                    }
                                                }
                                                if !ipsets.is_null() &&
                                                       flags as libc::c_uint &
                                                           (F_IPV4 | F_IPV6)
                                                           != 0 {
                                                    ipsets_cur = ipsets;
                                                    while !(*ipsets_cur).is_null()
                                                          {
                                                        log_query(flags as
                                                                      libc::c_uint
                                                                      &
                                                                      (F_IPV4
                                                                           |
                                                                           F_IPV6)
                                                                      |
                                                                      F_IPSET,
                                                                  name,
                                                                  &mut addr,
                                                                  *ipsets_cur);
                                                        let fresh23 =
                                                            ipsets_cur;
                                                        ipsets_cur =
                                                            ipsets_cur.offset(1);
                                                        add_to_ipset(*fresh23,
                                                                     &mut addr,
                                                                     flags,
                                                                     0 as
                                                                         libc::c_int);
                                                    }
                                                }
                                            }
                                            newc =
                                                cache_insert(name, &mut addr,
                                                             C_IN as
                                                                 libc::c_ushort,
                                                             now, attl,
                                                             flags as
                                                                 libc::c_uint
                                                                 | F_FORWARD |
                                                                 secflag_0 as
                                                                     libc::c_uint);
                                            if !newc.is_null() &&
                                                   !cpp.is_null() {
                                                next_uid(newc);
                                                (*cpp).addr.cname.target.cache
                                                    = newc;
                                                (*cpp).addr.cname.uid =
                                                    (*newc).uid
                                            }
                                            cpp = NULL_0 as *mut crec
                                        }
                                    }
                                    p1 = endrr;
                                    if !((p1.wrapping_offset_from(header as
                                                                      *mut libc::c_uchar)
                                              as libc::c_long +
                                              0 as libc::c_int as
                                                  libc::c_long) as size_t <=
                                             qlen) {
                                        return 0 as libc::c_int
                                    }
                                    j += 1
                                }
                            }
                        if found == 0 &&
                               (*dnsmasq_daemon).options[(11 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (11 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   == 0 {
                            if searched_soa == 0 {
                                searched_soa = 1 as libc::c_int;
                                ttl =
                                    find_soa(header, qlen,
                                             NULL_0 as *mut libc::c_char,
                                             doctored) as libc::c_ulong
                            }
                            /* If there's no SOA to get the TTL from, but there is a CNAME 
		 pointing at this, inherit its TTL */
                            if ttl != 0 || !cpp.is_null() {
                                newc =
                                    cache_insert(name,
                                                 NULL_0 as *mut all_addr,
                                                 C_IN as libc::c_ushort, now,
                                                 if ttl != 0 {
                                                     ttl
                                                 } else { cttl },
                                                 F_FORWARD | F_NEG |
                                                     flags as libc::c_uint |
                                                     (if secure != 0 {
                                                          F_DNSSECOK
                                                      } else {
                                                          0 as libc::c_int as
                                                              libc::c_uint
                                                      }));
                                if !newc.is_null() && !cpp.is_null() {
                                    next_uid(newc);
                                    (*cpp).addr.cname.target.cache = newc;
                                    (*cpp).addr.cname.uid = (*newc).uid
                                }
                            }
                        }
                    }
                }
            }
        }
        i -= 1
    }
    /* Don't put stuff from a truncated packet into the cache.
     Don't cache replies from non-recursive nameservers, since we may get a 
     reply containing a CNAME but not its target, even though the target 
     does exist. */
    if (*header).hb3 as libc::c_int & HB3_TC == 0 &&
           (*header).hb4 as libc::c_int & HB4_CD == 0 &&
           (*header).hb4 as libc::c_int & HB4_RA != 0 && no_cache_dnssec == 0
       {
        cache_end_insert();
    }
    return 0 as libc::c_int;
}
/* If the packet holds exactly one query
   return F_IPV4 or F_IPV6  and leave the name from the query in name */
#[no_mangle]
#[c2rust::src_loc = "889:1"]
pub unsafe extern "C" fn extract_request(mut header: *mut dns_header,
                                         mut qlen: size_t,
                                         mut name: *mut libc::c_char,
                                         mut typep: *mut libc::c_ushort)
 -> libc::c_uint {
    let mut p =
        header.offset(1 as libc::c_int as isize) as
            *mut libc::c_uchar; /* must be exactly one query. */
    let mut qtype: libc::c_int = 0; /* bad packet */
    let mut qclass: libc::c_int = 0;
    if !typep.is_null() { *typep = 0 as libc::c_int as libc::c_ushort }
    if __bswap_16((*header).qdcount) as libc::c_int != 1 as libc::c_int ||
           ((*header).hb3 as libc::c_int & HB3_OPCODE) >> 3 as libc::c_int !=
               QUERY {
        return 0 as libc::c_int as libc::c_uint
    }
    if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                    4 as libc::c_int) == 0 {
        return 0 as libc::c_int as libc::c_uint
    }
    let mut t_cp = p;
    qtype =
        (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
            8 as libc::c_int |
            *t_cp.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let mut t_cp_0 = p;
    qclass =
        (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
            8 as libc::c_int |
            *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    if !typep.is_null() { *typep = qtype as libc::c_ushort }
    if qclass == C_IN {
        if qtype == T_A { return F_IPV4 }
        if qtype == T_AAAA { return F_IPV6 }
        if qtype == T_ANY { return F_IPV4 | F_IPV6 }
    }
    /* F_DNSSECOK as agument to search_servers() inhibits forwarding
     to servers for domains without a trust anchor. This make the
     behaviour for DS and DNSKEY queries we forward the same
     as for DS and DNSKEY queries we originate. */
    if qtype == T_DS || qtype == T_DNSKEY { return F_DNSSECOK }
    return F_QUERY;
}
#[no_mangle]
#[c2rust::src_loc = "929:1"]
pub unsafe extern "C" fn setup_reply(mut header: *mut dns_header,
                                     mut qlen: size_t,
                                     mut addrp: *mut all_addr,
                                     mut flags: libc::c_uint,
                                     mut ttl: libc::c_ulong) -> size_t {
    let mut p = 0 as *mut libc::c_uchar;
    p = skip_questions(header, qlen);
    if p.is_null() { return 0 as libc::c_int as size_t }
    /* clear authoritative and truncated flags, set QR flag */
    (*header).hb3 =
        ((*header).hb3 as libc::c_int & !(HB3_AA | HB3_TC) | HB3_QR) as u8_0;
    /* clear AD flag, set RA flag */
    (*header).hb4 =
        ((*header).hb4 as libc::c_int & !HB4_AD | HB4_RA) as
            u8_0; /* no answers unless changed below */
    (*header).nscount =
        __bswap_16(0 as libc::c_int as __uint16_t); /* empty domain */
    (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
    (*header).ancount = __bswap_16(0 as libc::c_int as __uint16_t);
    if flags == F_NOERR {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 0 as libc::c_int) as
                u8_0
    } else if flags == F_NXDOMAIN {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 3 as libc::c_int) as
                u8_0
    } else if flags == F_SERVFAIL {
        let mut a = all_addr{addr4: in_addr{s_addr: 0,},};
        a.log.rcode = SERVFAIL as libc::c_ushort;
        log_query(F_CONFIG | F_RCODE,
                  b"error\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut a, NULL_0 as *mut libc::c_char);
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 2 as libc::c_int) as
                u8_0
    } else if flags & (F_IPV4 | F_IPV6) != 0 {
        if flags & F_IPV4 != 0 {
            /* we know the address */
            (*header).hb4 =
                ((*header).hb4 as libc::c_int & !HB4_RCODE | 0 as libc::c_int)
                    as u8_0;
            (*header).ancount = __bswap_16(1 as libc::c_int as __uint16_t);
            (*header).hb3 = ((*header).hb3 as libc::c_int | HB3_AA) as u8_0;
            add_resource_record(header, NULL_0 as *mut libc::c_char,
                                NULL_0 as *mut libc::c_int,
                                ::std::mem::size_of::<dns_header>() as
                                    libc::c_ulong as libc::c_int,
                                &mut p as *mut *mut libc::c_uchar, ttl,
                                NULL_0 as *mut libc::c_int,
                                T_A as libc::c_ushort, C_IN as libc::c_ushort,
                                b"4\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char, addrp);
        }
        if flags & F_IPV6 != 0 {
            (*header).hb4 =
                ((*header).hb4 as libc::c_int & !HB4_RCODE | 0 as libc::c_int)
                    as u8_0;
            (*header).ancount =
                __bswap_16((__bswap_16((*header).ancount) as libc::c_int +
                                1 as libc::c_int) as __uint16_t);
            (*header).hb3 = ((*header).hb3 as libc::c_int | HB3_AA) as u8_0;
            add_resource_record(header, NULL_0 as *mut libc::c_char,
                                NULL_0 as *mut libc::c_int,
                                ::std::mem::size_of::<dns_header>() as
                                    libc::c_ulong as libc::c_int,
                                &mut p as *mut *mut libc::c_uchar, ttl,
                                NULL_0 as *mut libc::c_int,
                                T_AAAA as libc::c_ushort,
                                C_IN as libc::c_ushort,
                                b"6\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char, addrp);
        }
    } else {
        /* nowhere to forward to */
        let mut a_0 = all_addr{addr4: in_addr{s_addr: 0,},};
        a_0.log.rcode = REFUSED as libc::c_ushort;
        log_query(F_CONFIG | F_RCODE,
                  b"error\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut a_0,
                  NULL_0 as *mut libc::c_char);
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 5 as libc::c_int) as
                u8_0
    }
    return p.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
/* check if name matches local names ie from /etc/hosts or DHCP or local mx names. */
#[no_mangle]
#[c2rust::src_loc = "986:1"]
pub unsafe extern "C" fn check_for_local_domain(mut name: *mut libc::c_char,
                                                mut now: time_t)
 -> libc::c_int {
    let mut mx = 0 as *mut mx_srv_record;
    let mut txt = 0 as *mut txt_record;
    let mut intr = 0 as *mut interface_name;
    let mut ptr = 0 as *mut ptr_record;
    let mut naptr = 0 as *mut naptr;
    naptr = (*dnsmasq_daemon).naptr;
    while !naptr.is_null() {
        if hostname_issubdomain(name, (*naptr).name) != 0 {
            return 1 as libc::c_int
        }
        naptr = (*naptr).next
    }
    mx = (*dnsmasq_daemon).mxnames;
    while !mx.is_null() {
        if hostname_issubdomain(name, (*mx).name) != 0 {
            return 1 as libc::c_int
        }
        mx = (*mx).next
    }
    txt = (*dnsmasq_daemon).txt;
    while !txt.is_null() {
        if hostname_issubdomain(name, (*txt).name) != 0 {
            return 1 as libc::c_int
        }
        txt = (*txt).next
    }
    intr = (*dnsmasq_daemon).int_names;
    while !intr.is_null() {
        if hostname_issubdomain(name, (*intr).name) != 0 {
            return 1 as libc::c_int
        }
        intr = (*intr).next
    }
    ptr = (*dnsmasq_daemon).ptr;
    while !ptr.is_null() {
        if hostname_issubdomain(name, (*ptr).name) != 0 {
            return 1 as libc::c_int
        }
        ptr = (*ptr).next
    }
    if cache_find_non_terminal(name, now) != 0 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* Is the packet a reply with the answer address equal to addr?
   If so mung is into an NXDOMAIN reply and also put that information
   in the cache. */
#[no_mangle]
#[c2rust::src_loc = "1023:1"]
pub unsafe extern "C" fn check_for_bogus_wildcard(mut header: *mut dns_header,
                                                  mut qlen: size_t,
                                                  mut name: *mut libc::c_char,
                                                  mut baddr: *mut bogus_addr,
                                                  mut now: time_t)
 -> libc::c_int {
    let mut p = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut ttl: libc::c_ulong = 0;
    let mut baddrp = 0 as *mut bogus_addr;
    /* skip over questions */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() { return 0 as libc::c_int } /* bad packet */
    i = __bswap_16((*header).ancount) as libc::c_int;
    while i != 0 as libc::c_int {
        if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                        10 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        let mut t_cp = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0 = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_1 = p;
        ttl =
            ((*t_cp_1.offset(0 as libc::c_int as isize) as u32_0) <<
                 24 as libc::c_int |
                 (*t_cp_1.offset(1 as libc::c_int as isize) as u32_0) <<
                     16 as libc::c_int |
                 (*t_cp_1.offset(2 as libc::c_int as isize) as u32_0) <<
                     8 as libc::c_int |
                 *t_cp_1.offset(3 as libc::c_int as isize) as u32_0) as
                libc::c_ulong;
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_2 = p;
        rdlen =
            (*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == C_IN && qtype == T_A {
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 4 as libc::c_int as libc::c_long) as
                     size_t <= qlen) {
                return 0 as libc::c_int
            }
            baddrp = baddr;
            while !baddrp.is_null() {
                if memcmp(&mut (*baddrp).addr as *mut in_addr as
                              *const libc::c_void, p as *const libc::c_void,
                          INADDRSZ as libc::c_ulong) == 0 as libc::c_int {
                    /* Found a bogus address. Insert that info here, since there no SOA record
		   to get the ttl from in the normal processing */
                    cache_start_insert();
                    cache_insert(name, NULL_0 as *mut all_addr,
                                 C_IN as libc::c_ushort, now, ttl,
                                 F_IPV4 | F_FORWARD | F_NEG | F_NXDOMAIN);
                    cache_end_insert();
                    return 1 as libc::c_int
                }
                baddrp = (*baddrp).next
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as libc::c_int
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1070:1"]
pub unsafe extern "C" fn check_for_ignored_address(mut header:
                                                       *mut dns_header,
                                                   mut qlen: size_t,
                                                   mut baddr: *mut bogus_addr)
 -> libc::c_int {
    let mut p = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut baddrp = 0 as *mut bogus_addr;
    /* skip over questions */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() { return 0 as libc::c_int } /* bad packet */
    i = __bswap_16((*header).ancount) as libc::c_int; /* TTL */
    while i != 0 as libc::c_int {
        p =
            skip_name(p, header, qlen,
                      10 as
                          libc::c_int); /* make ap point to 1st unamed argument */
        if p.is_null() { return 0 as libc::c_int }
        let mut t_cp = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0 = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_1 = p;
        rdlen =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == C_IN && qtype == T_A {
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 4 as libc::c_int as libc::c_long) as
                     size_t <= qlen) {
                return 0 as libc::c_int
            }
            baddrp = baddr;
            while !baddrp.is_null() {
                if memcmp(&mut (*baddrp).addr as *mut in_addr as
                              *const libc::c_void, p as *const libc::c_void,
                          INADDRSZ as libc::c_ulong) == 0 as libc::c_int {
                    return 1 as libc::c_int
                }
                baddrp = (*baddrp).next
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as libc::c_int
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1108:1"]
pub unsafe extern "C" fn add_resource_record(mut header: *mut dns_header,
                                             mut limit: *mut libc::c_char,
                                             mut truncp: *mut libc::c_int,
                                             mut nameoffset: libc::c_int,
                                             mut pp: *mut *mut libc::c_uchar,
                                             mut ttl: libc::c_ulong,
                                             mut offset: *mut libc::c_int,
                                             mut type_0: libc::c_ushort,
                                             mut class: libc::c_ushort,
                                             mut format: *mut libc::c_char,
                                             mut args: ...) -> libc::c_int {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    let mut sav = 0 as *mut libc::c_uchar;
    let mut p = *pp;
    let mut j: libc::c_int = 0;
    let mut usval: libc::c_ushort = 0;
    let mut lval: libc::c_long = 0;
    let mut sval = 0 as *mut libc::c_char;
    ap = args.clone();
    if !(!truncp.is_null() && *truncp != 0) {
        if nameoffset > 0 as libc::c_int {
            if !limit.is_null() &&
                   p.offset(2 as libc::c_int as isize) >
                       limit as *mut libc::c_uchar {
                current_block = 16696653877814833746;
            } else {
                let mut t_s = (nameoffset | 0xc000 as libc::c_int) as u16_0;
                let mut t_cp = p;
                let fresh24 = t_cp;
                t_cp = t_cp.offset(1);
                *fresh24 =
                    (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
                *t_cp = t_s as libc::c_uchar;
                p = p.offset(2 as libc::c_int as isize);
                current_block = 4488286894823169796;
            }
        } else {
            let mut name = ap.arg::<*mut libc::c_char>();
            if !name.is_null() &&
                   { p = do_rfc1035_name(p, name, limit); p.is_null() } {
                current_block = 16696653877814833746;
            } else if nameoffset < 0 as libc::c_int {
                if !limit.is_null() &&
                       p.offset(2 as libc::c_int as isize) >
                           limit as *mut libc::c_uchar {
                    current_block = 16696653877814833746;
                } else {
                    let mut t_s_0 =
                        (-nameoffset | 0xc000 as libc::c_int) as u16_0;
                    let mut t_cp_0 = p;
                    let fresh25 = t_cp_0;
                    t_cp_0 = t_cp_0.offset(1);
                    *fresh25 =
                        (t_s_0 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_0 = t_s_0 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    current_block = 4488286894823169796;
                }
            } else if !limit.is_null() &&
                          p.offset(1 as libc::c_int as isize) >
                              limit as *mut libc::c_uchar {
                current_block = 16696653877814833746;
            } else {
                let fresh26 = p;
                p = p.offset(1);
                *fresh26 = 0 as libc::c_int as libc::c_uchar;
                current_block = 4488286894823169796;
            }
        }
        match current_block {
            16696653877814833746 => { }
            _ =>
            /* type (2) + class (2) + ttl (4) + rdlen (2) */
            {
                if !(!limit.is_null() &&
                         p.offset(10 as libc::c_int as isize) >
                             limit as *mut libc::c_uchar) {
                    let mut t_s_1 = type_0;
                    let mut t_cp_1 = p;
                    let fresh27 = t_cp_1;
                    t_cp_1 = t_cp_1.offset(1);
                    *fresh27 =
                        (t_s_1 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_1 = t_s_1 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    let mut t_s_2 = class;
                    let mut t_cp_2 = p;
                    let fresh28 = t_cp_2;
                    t_cp_2 = t_cp_2.offset(1);
                    *fresh28 =
                        (t_s_2 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_2 = t_s_2 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    let mut t_l = ttl as u32_0;
                    let mut t_cp_3 = p;
                    let fresh29 = t_cp_3;
                    t_cp_3 = t_cp_3.offset(1);
                    *fresh29 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
                    let fresh30 = t_cp_3;
                    t_cp_3 = t_cp_3.offset(1);
                    *fresh30 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
                    let fresh31 = t_cp_3;
                    t_cp_3 = t_cp_3.offset(1);
                    *fresh31 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
                    *t_cp_3 = t_l as libc::c_uchar;
                    p = p.offset(4 as libc::c_int as isize);
                    /* TTL */
                    sav = p; /* Save pointer to RDLength field */
                    let mut t_s_3 = 0 as libc::c_int as u16_0;
                    let mut t_cp_4 = p;
                    let fresh32 = t_cp_4;
                    t_cp_4 = t_cp_4.offset(1);
                    *fresh32 =
                        (t_s_3 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_4 = t_s_3 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    loop 
                         /* Placeholder RDLength */
                         {
                        if !(*format != 0) {
                            current_block = 6665878751423064961;
                            break ;
                        }
                        match *format as libc::c_int {
                            54 => {
                                if !limit.is_null() &&
                                       p.offset(16 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                sval = ap.arg::<*mut libc::c_char>();
                                memcpy(p as *mut libc::c_void,
                                       sval as *const libc::c_void,
                                       IN6ADDRSZ as libc::c_ulong);
                                p = p.offset(IN6ADDRSZ as isize)
                            }
                            52 => {
                                if !limit.is_null() &&
                                       p.offset(4 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                sval = ap.arg::<*mut libc::c_char>();
                                memcpy(p as *mut libc::c_void,
                                       sval as *const libc::c_void,
                                       INADDRSZ as libc::c_ulong);
                                p = p.offset(INADDRSZ as isize)
                            }
                            98 => {
                                if !limit.is_null() &&
                                       p.offset(1 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                usval =
                                    ap.arg::<libc::c_int>() as libc::c_ushort;
                                let fresh33 = p;
                                p = p.offset(1);
                                *fresh33 = usval as libc::c_uchar
                            }
                            115 => {
                                if !limit.is_null() &&
                                       p.offset(2 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                usval =
                                    ap.arg::<libc::c_int>() as libc::c_ushort;
                                let mut t_s_4 = usval;
                                let mut t_cp_5 = p;
                                let fresh34 = t_cp_5;
                                t_cp_5 = t_cp_5.offset(1);
                                *fresh34 =
                                    (t_s_4 as libc::c_int >> 8 as libc::c_int)
                                        as libc::c_uchar;
                                *t_cp_5 = t_s_4 as libc::c_uchar;
                                p = p.offset(2 as libc::c_int as isize)
                            }
                            108 => {
                                if !limit.is_null() &&
                                       p.offset(4 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                lval = ap.arg::<libc::c_long>();
                                let mut t_l_0 = lval as u32_0;
                                let mut t_cp_6 = p;
                                let fresh35 = t_cp_6;
                                t_cp_6 = t_cp_6.offset(1);
                                *fresh35 =
                                    (t_l_0 >> 24 as libc::c_int) as
                                        libc::c_uchar;
                                let fresh36 = t_cp_6;
                                t_cp_6 = t_cp_6.offset(1);
                                *fresh36 =
                                    (t_l_0 >> 16 as libc::c_int) as
                                        libc::c_uchar;
                                let fresh37 = t_cp_6;
                                t_cp_6 = t_cp_6.offset(1);
                                *fresh37 =
                                    (t_l_0 >> 8 as libc::c_int) as
                                        libc::c_uchar;
                                *t_cp_6 = t_l_0 as libc::c_uchar;
                                p = p.offset(4 as libc::c_int as isize)
                            }
                            100 => {
                                /* get domain-name answer arg and store it in RDATA field */
                                if !offset.is_null() {
                                    *offset =
                                        p.wrapping_offset_from(header as
                                                                   *mut libc::c_uchar)
                                            as libc::c_long as libc::c_int
                                }
                                p =
                                    do_rfc1035_name(p,
                                                    ap.arg::<*mut libc::c_char>(),
                                                    limit);
                                if p.is_null() {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                if !limit.is_null() &&
                                       p.offset(1 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                let fresh38 = p;
                                p = p.offset(1);
                                *fresh38 = 0 as libc::c_int as libc::c_uchar
                            }
                            116 => {
                                usval =
                                    ap.arg::<libc::c_int>() as libc::c_ushort;
                                if !limit.is_null() &&
                                       p.offset(usval as libc::c_int as isize)
                                           > limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                sval = ap.arg::<*mut libc::c_char>();
                                if usval as libc::c_int != 0 as libc::c_int {
                                    memcpy(p as *mut libc::c_void,
                                           sval as *const libc::c_void,
                                           usval as libc::c_ulong);
                                }
                                p = p.offset(usval as libc::c_int as isize)
                            }
                            122 => {
                                sval = ap.arg::<*mut libc::c_char>();
                                usval =
                                    if !sval.is_null() {
                                        strlen(sval)
                                    } else {
                                        0 as libc::c_int as libc::c_ulong
                                    } as libc::c_ushort;
                                if usval as libc::c_int > 255 as libc::c_int {
                                    usval =
                                        255 as libc::c_int as libc::c_ushort
                                }
                                if !limit.is_null() &&
                                       p.offset((usval as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                let fresh39 = p;
                                p = p.offset(1);
                                *fresh39 = usval as libc::c_uchar;
                                memcpy(p as *mut libc::c_void,
                                       sval as *const libc::c_void,
                                       usval as libc::c_ulong);
                                p = p.offset(usval as libc::c_int as isize)
                            }
                            _ => { }
                        }
                        format = format.offset(1)
                    }
                    match current_block {
                        16696653877814833746 => { }
                        _ =>
                        /* clean up variable argument pointer */
                        /* Now, store real RDLength. sav already checked against limit. */
                        {
                            j =
                                (p.wrapping_offset_from(sav) as libc::c_long -
                                     2 as libc::c_int as libc::c_long) as
                                    libc::c_int;
                            let mut t_s_5 = j as u16_0;
                            let mut t_cp_7 = sav;
                            let fresh40 = t_cp_7;
                            t_cp_7 = t_cp_7.offset(1);
                            *fresh40 =
                                (t_s_5 as libc::c_int >> 8 as libc::c_int) as
                                    libc::c_uchar;
                            *t_cp_7 = t_s_5 as libc::c_uchar;
                            sav = sav.offset(2 as libc::c_int as isize);
                            *pp = p;
                            return 1 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    if !truncp.is_null() { *truncp = 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1243:1"]
unsafe extern "C" fn crec_ttl(mut crecp: *mut crec, mut now: time_t)
 -> libc::c_ulong {
    /* Return 0 ttl for DHCP entries, which might change
     before the lease expires, unless configured otherwise. */
    if (*crecp).flags & F_DHCP != 0 {
        let mut conf_ttl =
            if (*dnsmasq_daemon).use_dhcp_ttl != 0 {
                (*dnsmasq_daemon).dhcp_ttl
            } else { (*dnsmasq_daemon).local_ttl } as libc::c_int;
        /* Apply ceiling of actual lease length to configured TTL. */
        if (*crecp).flags & F_IMMORTAL == 0 &&
               (*crecp).ttd - now < conf_ttl as libc::c_long {
            return ((*crecp).ttd - now) as libc::c_ulong
        }
        return conf_ttl as libc::c_ulong
    }
    /* Immortal entries other than DHCP are local, and hold TTL in TTD field. */
    if (*crecp).flags & F_IMMORTAL != 0 {
        return (*crecp).ttd as libc::c_ulong
    }
    /* Return the Max TTL value if it is lower than the actual TTL */
    if (*dnsmasq_daemon).max_ttl == 0 as libc::c_int as libc::c_ulong ||
           (((*crecp).ttd - now) as libc::c_uint as libc::c_ulong) <
               (*dnsmasq_daemon).max_ttl {
        return ((*crecp).ttd - now) as libc::c_ulong
    } else { return (*dnsmasq_daemon).max_ttl };
}
#[c2rust::src_loc = "1270:1"]
unsafe extern "C" fn cache_validated(mut crecp: *const crec) -> libc::c_int {
    return ((*dnsmasq_daemon).options[(45 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (45 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                != 0 && (*crecp).flags & F_DNSSECOK == 0) as libc::c_int;
}
/* return zero if we can't answer from cache, or packet size if we can */
#[no_mangle]
#[c2rust::src_loc = "1276:1"]
pub unsafe extern "C" fn answer_request(mut header: *mut dns_header,
                                        mut limit: *mut libc::c_char,
                                        mut qlen: size_t,
                                        mut local_addr: in_addr,
                                        mut local_netmask: in_addr,
                                        mut now: time_t,
                                        mut ad_reqd: libc::c_int,
                                        mut do_bit: libc::c_int,
                                        mut have_pseudoheader: libc::c_int)
 -> size_t {
    let mut name = (*dnsmasq_daemon).namebuff;
    let mut p = 0 as *mut libc::c_uchar;
    let mut ansp = 0 as *mut libc::c_uchar;
    let mut qtype: libc::c_uint = 0;
    let mut qclass: libc::c_uint = 0;
    let mut addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut nameoffset: libc::c_int = 0;
    let mut flag: libc::c_ushort = 0;
    let mut q: libc::c_int = 0;
    let mut ans: libc::c_int = 0;
    let mut anscount = 0 as libc::c_int;
    let mut addncount = 0 as libc::c_int;
    let mut dryrun = 0 as libc::c_int;
    let mut crecp = 0 as *mut crec;
    let mut nxdomain = 0 as libc::c_int;
    let mut notimp = 0 as libc::c_int;
    let mut auth = 1 as libc::c_int;
    let mut trunc = 0 as libc::c_int;
    let mut sec_data = 1 as libc::c_int;
    let mut rec = 0 as *mut mx_srv_record;
    let mut len: size_t = 0;
    let mut rd_bit = (*header).hb3 as libc::c_int & HB3_RD;
    /* never answer queries with RD unset, to avoid cache snooping. */
    if __bswap_16((*header).ancount) as libc::c_int != 0 as libc::c_int ||
           __bswap_16((*header).nscount) as libc::c_int != 0 as libc::c_int ||
           __bswap_16((*header).qdcount) as libc::c_int == 0 as libc::c_int ||
           ((*header).hb3 as libc::c_int & HB3_OPCODE) >> 3 as libc::c_int !=
               QUERY {
        return 0 as libc::c_int as size_t
    }
    /* Don't return AD set if checking disabled. */
    if (*header).hb4 as libc::c_int & HB4_CD != 0 {
        sec_data = 0 as libc::c_int
    }
    /* If there is an  additional data section then it will be overwritten by
     partial replies, so we have to do a dry run to see if we can answer
     the query. */
    if __bswap_16((*header).arcount) as libc::c_int != 0 as libc::c_int {
        dryrun = 1 as libc::c_int
    } /* bad packet */
    rec = (*dnsmasq_daemon).mxnames;
    while !rec.is_null() {
        (*rec).offset = 0 as libc::c_int as libc::c_uint;
        rec = (*rec).next
    }
    loop 
         /* determine end of question section (we put answers there) */
         {
        ansp = skip_questions(header, qlen);
        if ansp.is_null() { return 0 as libc::c_int as size_t }
        /* now process each question, answers go in RRs after the question */
        p =
            header.offset(1 as libc::c_int as isize) as
                *mut libc::c_uchar; /* catch loops */
        q = __bswap_16((*header).qdcount) as libc::c_int;
        while q != 0 as libc::c_int {
            let mut count = 255 as libc::c_int;
            /* save pointer to name for copying into answers */
            nameoffset =
                p.wrapping_offset_from(header as *mut libc::c_uchar) as
                    libc::c_long as libc::c_int;
            /* now extract name as .-concatenated string into name */
            if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                            4 as libc::c_int) == 0 {
                return 0 as libc::c_int as size_t
            } /* bad packet */
            let mut t_cp = p; /* have we answered this question */
            qtype =
                ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_uint;
            p = p.offset(2 as libc::c_int as isize);
            let mut t_cp_0 = p;
            qclass =
                ((*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_uint;
            p = p.offset(2 as libc::c_int as isize);
            ans = 0 as libc::c_int;
            loop  {
                count -= 1;
                if !(count != 0 as libc::c_int &&
                         {
                             crecp =
                                 cache_find_by_name(NULL_0 as *mut crec, name,
                                                    now, F_CNAME);
                             !crecp.is_null()
                         }) {
                    break ;
                }
                let mut cname_target = cache_get_cname_target(crecp);
                /* If the client asked for DNSSEC  don't use cached data. */
                if (*crecp).flags & (F_HOSTS | F_DHCP | F_CONFIG) != 0 ||
                       rd_bit != 0 &&
                           (do_bit == 0 || cache_validated(crecp) != 0) {
                    if (*crecp).flags & F_CONFIG != 0 ||
                           qtype == T_CNAME as libc::c_uint {
                        ans = 1 as libc::c_int
                    } /* give up if any cached CNAME in chain can't be used for DNSSEC reasons. */
                    if (*crecp).flags & F_DNSSECOK == 0 {
                        sec_data = 0 as libc::c_int
                    }
                    if dryrun == 0 {
                        log_query((*crecp).flags, name,
                                  NULL_0 as *mut all_addr,
                                  record_source((*crecp).uid));
                        if add_resource_record(header, limit,
                                               &mut trunc as *mut libc::c_int,
                                               nameoffset,
                                               &mut ansp as
                                                   *mut *mut libc::c_uchar,
                                               crec_ttl(crecp, now),
                                               &mut nameoffset as
                                                   *mut libc::c_int,
                                               T_CNAME as libc::c_ushort,
                                               C_IN as libc::c_ushort,
                                               b"d\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               cname_target) != 0 {
                            anscount += 1
                        }
                    }
                } else { return 0 as libc::c_int as size_t }
                strcpy(name, cname_target);
            }
            if qtype == T_TXT as libc::c_uint ||
                   qtype == T_ANY as libc::c_uint {
                let mut t = 0 as *mut txt_record;
                t = (*dnsmasq_daemon).txt;
                while !t.is_null() {
                    if (*t).class as libc::c_uint == qclass &&
                           hostname_isequal(name, (*t).name) != 0 {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            let mut ttl = (*dnsmasq_daemon).local_ttl;
                            let mut ok = 1 as libc::c_int;
                            /* Dynamically generate stat record */
                            if (*t).stat != 0 as libc::c_int {
                                ttl = 0 as libc::c_int as libc::c_ulong;
                                if cache_make_stat(t) == 0 {
                                    ok = 0 as libc::c_int
                                }
                            }
                            if ok != 0 {
                                log_query(F_CONFIG | F_RRNAME, name,
                                          NULL_0 as *mut all_addr,
                                          b"<TXT>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       ttl,
                                                       NULL_0 as
                                                           *mut libc::c_int,
                                                       T_TXT as
                                                           libc::c_ushort,
                                                       (*t).class,
                                                       b"t\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*t).len as
                                                           libc::c_int,
                                                       (*t).txt) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                    }
                    t = (*t).next
                }
            }
            if qclass == C_CHAOS as libc::c_uint {
                /* don't forward *.bind and *.server chaos queries - always reply with NOTIMP */
                if hostname_issubdomain(b"bind\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char, name) != 0 ||
                       hostname_issubdomain(b"server\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char, name) != 0
                   {
                    if ans == 0 {
                        notimp = 1 as libc::c_int;
                        auth = 0 as libc::c_int;
                        if dryrun == 0 {
                            addr.log.rcode = NOTIMP as libc::c_ushort;
                            log_query(F_CONFIG | F_RCODE, name, &mut addr,
                                      NULL_0 as *mut libc::c_char);
                        }
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int
                    }
                }
            }
            if qclass == C_IN as libc::c_uint {
                let mut t_0 = 0 as *mut txt_record;
                t_0 = (*dnsmasq_daemon).rr;
                while !t_0.is_null() {
                    if ((*t_0).class as libc::c_uint == qtype ||
                            qtype == T_ANY as libc::c_uint) &&
                           hostname_isequal(name, (*t_0).name) != 0 {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query(F_CONFIG | F_RRNAME, name,
                                      NULL_0 as *mut all_addr,
                                      querystr(NULL_0 as *mut libc::c_char,
                                               (*t_0).class));
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   nameoffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).local_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   (*t_0).class,
                                                   C_IN as libc::c_ushort,
                                                   b"t\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (*t_0).len as libc::c_int,
                                                   (*t_0).txt) != 0 {
                                anscount += 1
                            }
                        }
                    }
                    t_0 = (*t_0).next
                }
                if qtype == T_PTR as libc::c_uint ||
                       qtype == T_ANY as libc::c_uint {
                    /* see if it's w.z.y.z.in-addr.arpa format */
                    let mut is_arpa = in_arpa_name_2_addr(name, &mut addr);
                    let mut ptr = 0 as *mut ptr_record;
                    let mut intr = NULL_0 as *mut interface_name;
                    ptr = (*dnsmasq_daemon).ptr;
                    while !ptr.is_null() {
                        if hostname_isequal(name, (*ptr).name) != 0 {
                            break ;
                        }
                        ptr = (*ptr).next
                    }
                    if is_arpa as libc::c_uint == F_IPV4 {
                        intr = (*dnsmasq_daemon).int_names;
                        while !intr.is_null() {
                            let mut addrlist = 0 as *mut addrlist;
                            addrlist = (*intr).addr;
                            while !addrlist.is_null() {
                                if (*addrlist).flags & ADDRLIST_IPV6 == 0 &&
                                       addr.addr4.s_addr ==
                                           (*addrlist).addr.addr4.s_addr {
                                    break ;
                                }
                                addrlist = (*addrlist).next
                            }
                            if !addrlist.is_null() { break ; }
                            while !(*intr).next.is_null() &&
                                      strcmp((*intr).intr,
                                             (*(*intr).next).intr) ==
                                          0 as libc::c_int {
                                intr = (*intr).next
                            }
                            intr = (*intr).next
                        }
                    } else if is_arpa as libc::c_uint == F_IPV6 {
                        intr = (*dnsmasq_daemon).int_names;
                        while !intr.is_null() {
                            let mut addrlist_0 = 0 as *mut addrlist;
                            addrlist_0 = (*intr).addr;
                            while !addrlist_0.is_null() {
                                if (*addrlist_0).flags & ADDRLIST_IPV6 != 0 &&
                                       ({
                                            let mut __a =
                                                &mut addr.addr6 as
                                                    *mut in6_addr as
                                                    *const in6_addr;
                                            let mut __b =
                                                &mut (*addrlist_0).addr.addr6
                                                    as *mut in6_addr as
                                                    *const in6_addr;
                                            ((*__a).__in6_u.__u6_addr32[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 (*__b).__in6_u.__u6_addr32[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                 &&
                                                 (*__a).__in6_u.__u6_addr32[1
                                                                                as
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
                                                 (*__a).__in6_u.__u6_addr32[2
                                                                                as
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
                                                 (*__a).__in6_u.__u6_addr32[3
                                                                                as
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
                                    break ;
                                }
                                addrlist_0 = (*addrlist_0).next
                            }
                            if !addrlist_0.is_null() { break ; }
                            while !(*intr).next.is_null() &&
                                      strcmp((*intr).intr,
                                             (*(*intr).next).intr) ==
                                          0 as libc::c_int {
                                intr = (*intr).next
                            }
                            intr = (*intr).next
                        }
                    }
                    if !intr.is_null() {
                        sec_data = 0 as libc::c_int;
                        ans = 1 as libc::c_int;
                        if dryrun == 0 {
                            log_query(is_arpa as libc::c_uint | F_REVERSE |
                                          F_CONFIG, (*intr).name, &mut addr,
                                      NULL_0 as *mut libc::c_char);
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   nameoffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).local_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   T_PTR as libc::c_ushort,
                                                   C_IN as libc::c_ushort,
                                                   b"d\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (*intr).name) != 0 {
                                anscount += 1
                            }
                        }
                    } else if !ptr.is_null() {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query(F_CONFIG | F_RRNAME, name,
                                      NULL_0 as *mut all_addr,
                                      b"<PTR>\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                            ptr = (*dnsmasq_daemon).ptr;
                            while !ptr.is_null() {
                                if hostname_isequal(name, (*ptr).name) != 0 &&
                                       add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).local_ttl,
                                                           NULL_0 as
                                                               *mut libc::c_int,
                                                           T_PTR as
                                                               libc::c_ushort,
                                                           C_IN as
                                                               libc::c_ushort,
                                                           b"d\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*ptr).ptr) != 0 {
                                    anscount += 1
                                }
                                ptr = (*ptr).next
                            }
                        }
                    } else {
                        crecp =
                            cache_find_by_addr(NULL_0 as *mut crec, &mut addr,
                                               now, is_arpa as libc::c_uint);
                        if !crecp.is_null() {
                            /* Don't use cache when DNSSEC data required, unless we know that
		     the zone is unsigned, which implies that we're doing
		     validation. */
                            if (*crecp).flags & (F_HOSTS | F_DHCP | F_CONFIG)
                                   != 0 ||
                                   rd_bit != 0 &&
                                       (do_bit == 0 ||
                                            cache_validated(crecp) != 0) {
                                loop 
                                     /* don't answer wildcard queries with data not from /etc/hosts or dhcp leases */
                                     {
                                    if !(qtype == T_ANY as libc::c_uint &&
                                             (*crecp).flags &
                                                 (F_HOSTS | F_DHCP) == 0) {
                                        if (*crecp).flags & F_DNSSECOK == 0 {
                                            sec_data = 0 as libc::c_int
                                        }
                                        ans = 1 as libc::c_int;
                                        if (*crecp).flags & F_NEG != 0 {
                                            auth = 0 as libc::c_int;
                                            if (*crecp).flags & F_NXDOMAIN !=
                                                   0 {
                                                nxdomain = 1 as libc::c_int
                                            }
                                            if dryrun == 0 {
                                                log_query((*crecp).flags &
                                                              !F_FORWARD,
                                                          name, &mut addr,
                                                          NULL_0 as
                                                              *mut libc::c_char);
                                            }
                                        } else {
                                            if (*crecp).flags &
                                                   (F_HOSTS | F_DHCP) == 0 {
                                                auth = 0 as libc::c_int
                                            }
                                            if dryrun == 0 {
                                                log_query((*crecp).flags &
                                                              !F_FORWARD,
                                                          cache_get_name(crecp),
                                                          &mut addr,
                                                          record_source((*crecp).uid));
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       crec_ttl(crecp,
                                                                                now),
                                                                       NULL_0
                                                                           as
                                                                           *mut libc::c_int,
                                                                       T_PTR
                                                                           as
                                                                           libc::c_ushort,
                                                                       C_IN as
                                                                           libc::c_ushort,
                                                                       b"d\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char
                                                                           as
                                                                           *mut libc::c_char,
                                                                       cache_get_name(crecp))
                                                       != 0 {
                                                    anscount += 1
                                                }
                                            }
                                        }
                                    }
                                    crecp =
                                        cache_find_by_addr(crecp, &mut addr,
                                                           now,
                                                           is_arpa as
                                                               libc::c_uint);
                                    if crecp.is_null() { break ; }
                                }
                            }
                        } else if is_rev_synth(is_arpa, &mut addr, name) != 0
                         {
                            ans = 1 as libc::c_int;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                log_query(F_CONFIG | F_REVERSE |
                                              is_arpa as libc::c_uint, name,
                                          &mut addr,
                                          NULL_0 as *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       NULL_0 as
                                                           *mut libc::c_int,
                                                       T_PTR as
                                                           libc::c_ushort,
                                                       C_IN as libc::c_ushort,
                                                       b"d\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       name) != 0 {
                                    anscount += 1
                                }
                            }
                        } else if (*dnsmasq_daemon).options[(0 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_mul(8
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_ulong))
                                                                as usize] &
                                      (1 as libc::c_uint) <<
                                          (0 as libc::c_int as
                                               libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                as
                                                                                libc::c_ulong).wrapping_mul(8
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong))
                                      != 0 &&
                                      (is_arpa as libc::c_uint == F_IPV6 &&
                                           private_net6(&mut addr.addr6) != 0
                                           ||
                                           is_arpa as libc::c_uint == F_IPV4
                                               &&
                                               private_net(addr.addr4,
                                                           1 as libc::c_int)
                                                   != 0) {
                            let mut serv = 0 as *mut server;
                            let mut namelen = strlen(name) as libc::c_uint;
                            let mut nameend = name.offset(namelen as isize);
                            /* see if have rev-server set */
                            serv = (*dnsmasq_daemon).servers;
                            while !serv.is_null() {
                                let mut domainlen: libc::c_uint = 0;
                                let mut matchstart = 0 as *mut libc::c_char;
                                if !((*serv).flags &
                                         (SERV_HAS_DOMAIN | SERV_NO_ADDR) !=
                                         SERV_HAS_DOMAIN) {
                                    domainlen =
                                        strlen((*serv).domain) as
                                            libc::c_uint;
                                    if !(domainlen ==
                                             0 as libc::c_int as libc::c_uint
                                             || domainlen > namelen) {
                                        matchstart =
                                            nameend.offset(-(domainlen as
                                                                 isize));
                                        if hostname_isequal(matchstart,
                                                            (*serv).domain) !=
                                               0 &&
                                               (namelen == domainlen ||
                                                    *matchstart.offset(-(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize))
                                                        as libc::c_int ==
                                                        '.' as i32) {
                                            break ;
                                        }
                                    }
                                }
                                serv = (*serv).next
                            }
                            /* if no configured server, not in cache, enabled and private IPV4 address, return NXDOMAIN */
                            if serv.is_null() {
                                ans = 1 as libc::c_int;
                                sec_data = 0 as libc::c_int;
                                nxdomain = 1 as libc::c_int;
                                if dryrun == 0 {
                                    log_query(F_CONFIG | F_REVERSE |
                                                  is_arpa as libc::c_uint |
                                                  F_NEG | F_NXDOMAIN, name,
                                              &mut addr,
                                              NULL_0 as *mut libc::c_char);
                                }
                            }
                        }
                    }
                }
                flag = F_IPV4 as libc::c_ushort;
                while flag != 0 {
                    let mut type_0 =
                        if flag as libc::c_uint == F_IPV6 {
                            T_AAAA
                        } else { T_A } as libc::c_ushort;
                    let mut intr_0 = 0 as *mut interface_name;
                    if !(qtype != type_0 as libc::c_uint &&
                             qtype != T_ANY as libc::c_uint) {
                        /* interface name stuff */
                        intr_0 = (*dnsmasq_daemon).int_names;
                        while !intr_0.is_null() {
                            if hostname_isequal(name, (*intr_0).name) != 0 {
                                break ;
                            }
                            intr_0 = (*intr_0).next
                        }
                        if !intr_0.is_null() {
                            let mut addrlist_1 = 0 as *mut addrlist;
                            let mut gotit = 0 as libc::c_int;
                            let mut localise = 0 as libc::c_int;
                            enumerate_interfaces(0 as libc::c_int);
                            /* See if a putative address is on the network from which we received
		     the query, is so we'll filter other answers. */
                            if local_addr.s_addr !=
                                   0 as libc::c_int as libc::c_uint &&
                                   (*dnsmasq_daemon).options[(18 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   as
                                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_ulong))
                                                                 as usize] &
                                       (1 as libc::c_uint) <<
                                           (18 as libc::c_int as
                                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong))
                                       != 0 && type_0 as libc::c_int == T_A {
                                intr_0 = (*dnsmasq_daemon).int_names;
                                while !intr_0.is_null() {
                                    if hostname_isequal(name, (*intr_0).name)
                                           != 0 {
                                        addrlist_1 = (*intr_0).addr;
                                        while !addrlist_1.is_null() {
                                            if (*addrlist_1).flags &
                                                   ADDRLIST_IPV6 == 0 &&
                                                   is_same_net((*addrlist_1).addr.addr4,
                                                               local_addr,
                                                               local_netmask)
                                                       != 0 {
                                                localise = 1 as libc::c_int;
                                                break ;
                                            } else {
                                                addrlist_1 =
                                                    (*addrlist_1).next
                                            }
                                        }
                                    }
                                    intr_0 = (*intr_0).next
                                }
                            }
                            intr_0 = (*dnsmasq_daemon).int_names;
                            while !intr_0.is_null() {
                                if hostname_isequal(name, (*intr_0).name) != 0
                                   {
                                    addrlist_1 = (*intr_0).addr;
                                    while !addrlist_1.is_null() {
                                        if (if (*addrlist_1).flags &
                                                   ADDRLIST_IPV6 != 0 {
                                                T_AAAA
                                            } else { T_A }) ==
                                               type_0 as libc::c_int {
                                            if !(localise != 0 &&
                                                     is_same_net((*addrlist_1).addr.addr4,
                                                                 local_addr,
                                                                 local_netmask)
                                                         == 0) {
                                                if !((*addrlist_1).flags &
                                                         ADDRLIST_REVONLY !=
                                                         0) {
                                                    ans = 1 as libc::c_int;
                                                    sec_data =
                                                        0 as libc::c_int;
                                                    if dryrun == 0 {
                                                        gotit =
                                                            1 as libc::c_int;
                                                        log_query(F_FORWARD |
                                                                      F_CONFIG
                                                                      |
                                                                      flag as
                                                                          libc::c_uint,
                                                                  name,
                                                                  &mut (*addrlist_1).addr,
                                                                  NULL_0 as
                                                                      *mut libc::c_char);
                                                        if add_resource_record(header,
                                                                               limit,
                                                                               &mut trunc
                                                                                   as
                                                                                   *mut libc::c_int,
                                                                               nameoffset,
                                                                               &mut ansp
                                                                                   as
                                                                                   *mut *mut libc::c_uchar,
                                                                               (*dnsmasq_daemon).local_ttl,
                                                                               NULL_0
                                                                                   as
                                                                                   *mut libc::c_int,
                                                                               type_0,
                                                                               C_IN
                                                                                   as
                                                                                   libc::c_ushort,
                                                                               if type_0
                                                                                      as
                                                                                      libc::c_int
                                                                                      ==
                                                                                      T_A
                                                                                  {
                                                                                   b"4\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char
                                                                               } else {
                                                                                   b"6\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char
                                                                               }
                                                                                   as
                                                                                   *mut libc::c_char,
                                                                               &mut (*addrlist_1).addr
                                                                                   as
                                                                                   *mut all_addr)
                                                               != 0 {
                                                            anscount += 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        addrlist_1 = (*addrlist_1).next
                                    }
                                }
                                intr_0 = (*intr_0).next
                            }
                            if dryrun == 0 && gotit == 0 {
                                log_query(F_FORWARD | F_CONFIG |
                                              flag as libc::c_uint | F_NEG,
                                          name, NULL_0 as *mut all_addr,
                                          NULL_0 as *mut libc::c_char);
                            }
                        } else {
                            crecp =
                                cache_find_by_name(NULL_0 as *mut crec, name,
                                                   now,
                                                   flag as libc::c_uint |
                                                       (if dryrun != 0 {
                                                            F_NO_RR
                                                        } else {
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_uint
                                                        }));
                            if !crecp.is_null() {
                                let mut localise_0 = 0 as libc::c_int;
                                /* See if a putative address is on the network from which we received
		     the query, is so we'll filter other answers. */
                                if local_addr.s_addr !=
                                       0 as libc::c_int as libc::c_uint &&
                                       (*dnsmasq_daemon).options[(18 as
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
                                               (18 as libc::c_int as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                           != 0 &&
                                       flag as libc::c_uint == F_IPV4 {
                                    let mut save = crecp;
                                    loop  {
                                        if (*crecp).flags & F_HOSTS != 0 &&
                                               is_same_net((*crecp).addr.addr4,
                                                           local_addr,
                                                           local_netmask) != 0
                                           {
                                            localise_0 = 1 as libc::c_int;
                                            break ;
                                        } else {
                                            crecp =
                                                cache_find_by_name(crecp,
                                                                   name, now,
                                                                   flag as
                                                                       libc::c_uint);
                                            if crecp.is_null() { break ; }
                                        }
                                    }
                                    crecp = save
                                }
                                /* If the client asked for DNSSEC  don't use cached data. */
                                if (*crecp).flags &
                                       (F_HOSTS | F_DHCP | F_CONFIG) != 0 ||
                                       rd_bit != 0 &&
                                           (do_bit == 0 ||
                                                cache_validated(crecp) != 0) {
                                    /* don't answer wildcard queries with data not from /etc/hosts
			   or DHCP leases */
                                    while !(qtype == T_ANY as libc::c_uint &&
                                                (*crecp).flags &
                                                    (F_HOSTS | F_DHCP |
                                                         F_CONFIG) == 0) {
                                        if (*crecp).flags & F_DNSSECOK == 0 {
                                            sec_data = 0 as libc::c_int
                                        }
                                        if (*crecp).flags & F_NEG != 0 {
                                            ans = 1 as libc::c_int;
                                            auth = 0 as libc::c_int;
                                            if (*crecp).flags & F_NXDOMAIN !=
                                                   0 {
                                                nxdomain = 1 as libc::c_int
                                            }
                                            if dryrun == 0 {
                                                log_query((*crecp).flags,
                                                          name,
                                                          NULL_0 as
                                                              *mut all_addr,
                                                          NULL_0 as
                                                              *mut libc::c_char);
                                            }
                                        } else if !(localise_0 != 0 &&
                                                        (*crecp).flags &
                                                            F_HOSTS != 0 &&
                                                        is_same_net((*crecp).addr.addr4,
                                                                    local_addr,
                                                                    local_netmask)
                                                            == 0) {
                                            if (*crecp).flags &
                                                   (F_HOSTS | F_DHCP) == 0 {
                                                auth = 0 as libc::c_int
                                            }
                                            ans = 1 as libc::c_int;
                                            if dryrun == 0 {
                                                log_query((*crecp).flags &
                                                              !F_REVERSE,
                                                          name,
                                                          &mut (*crecp).addr,
                                                          record_source((*crecp).uid));
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       crec_ttl(crecp,
                                                                                now),
                                                                       NULL_0
                                                                           as
                                                                           *mut libc::c_int,
                                                                       type_0,
                                                                       C_IN as
                                                                           libc::c_ushort,
                                                                       if type_0
                                                                              as
                                                                              libc::c_int
                                                                              ==
                                                                              T_A
                                                                          {
                                                                           b"4\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       } else {
                                                                           b"6\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       } as
                                                                           *mut libc::c_char,
                                                                       &mut (*crecp).addr
                                                                           as
                                                                           *mut all_addr)
                                                       != 0 {
                                                    anscount += 1
                                                }
                                            }
                                        }
                                        crecp =
                                            cache_find_by_name(crecp, name,
                                                               now,
                                                               flag as
                                                                   libc::c_uint);
                                        if crecp.is_null() { break ; }
                                    }
                                }
                            } else if is_name_synthetic(flag as libc::c_int,
                                                        name, &mut addr) != 0
                             {
                                ans = 1 as libc::c_int;
                                sec_data = 0 as libc::c_int;
                                if dryrun == 0 {
                                    log_query(F_FORWARD | F_CONFIG |
                                                  flag as libc::c_uint, name,
                                              &mut addr,
                                              NULL_0 as *mut libc::c_char);
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).local_ttl,
                                                           NULL_0 as
                                                               *mut libc::c_int,
                                                           type_0,
                                                           C_IN as
                                                               libc::c_ushort,
                                                           if type_0 as
                                                                  libc::c_int
                                                                  == T_A {
                                                               b"4\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           } else {
                                                               b"6\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           } as
                                                               *mut libc::c_char,
                                                           &mut addr as
                                                               *mut all_addr)
                                           != 0 {
                                        anscount += 1
                                    }
                                }
                            }
                        }
                    }
                    flag =
                        if flag as libc::c_uint == F_IPV4 {
                            F_IPV6
                        } else { 0 as libc::c_int as libc::c_uint } as
                            libc::c_ushort
                }
                if qtype == T_MX as libc::c_uint ||
                       qtype == T_ANY as libc::c_uint {
                    let mut found = 0 as libc::c_int;
                    rec = (*dnsmasq_daemon).mxnames;
                    while !rec.is_null() {
                        if (*rec).issrv == 0 &&
                               hostname_isequal(name, (*rec).name) != 0 {
                            found = 1 as libc::c_int;
                            ans = found;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                let mut offset: libc::c_int = 0;
                                log_query(F_CONFIG | F_RRNAME, name,
                                          NULL_0 as *mut all_addr,
                                          b"<MX>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       &mut offset as
                                                           *mut libc::c_int,
                                                       T_MX as libc::c_ushort,
                                                       C_IN as libc::c_ushort,
                                                       b"sd\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*rec).weight,
                                                       (*rec).target) != 0 {
                                    anscount += 1;
                                    if !(*rec).target.is_null() {
                                        (*rec).offset = offset as libc::c_uint
                                    }
                                }
                            }
                        }
                        rec = (*rec).next
                    }
                    if found == 0 &&
                           ((*dnsmasq_daemon).options[(3 as libc::c_int as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as usize] &
                                (1 as libc::c_uint) <<
                                    (3 as libc::c_int as
                                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong))
                                != 0 ||
                                (*dnsmasq_daemon).options[(10 as libc::c_int
                                                               as
                                                               libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                as
                                                                                                libc::c_ulong).wrapping_mul(8
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_ulong))
                                                              as usize] &
                                    (1 as libc::c_uint) <<
                                        (10 as libc::c_int as
                                             libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(8
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_ulong))
                                    != 0) &&
                           !cache_find_by_name(NULL_0 as *mut crec, name, now,
                                               F_HOSTS | F_DHCP |
                                                   F_NO_RR).is_null() {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query(F_CONFIG | F_RRNAME, name,
                                      NULL_0 as *mut all_addr,
                                      b"<MX>\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   nameoffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).local_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   T_MX as libc::c_ushort,
                                                   C_IN as libc::c_ushort,
                                                   b"sd\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   1 as libc::c_int,
                                                   if (*dnsmasq_daemon).options[(3
                                                                                     as
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
                                                          (1 as libc::c_uint)
                                                              <<
                                                              (3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_ulong))
                                                          != 0 {
                                                       name
                                                   } else {
                                                       (*dnsmasq_daemon).mxtarget
                                                   }) != 0 {
                                anscount += 1
                            }
                        }
                    }
                }
                if qtype == T_SRV as libc::c_uint ||
                       qtype == T_ANY as libc::c_uint {
                    let mut found_0 = 0 as libc::c_int;
                    let mut move_0 = NULL_0 as *mut mx_srv_record;
                    let mut up: *mut *mut mx_srv_record =
                        &mut (*dnsmasq_daemon).mxnames;
                    rec = (*dnsmasq_daemon).mxnames;
                    while !rec.is_null() {
                        if (*rec).issrv != 0 &&
                               hostname_isequal(name, (*rec).name) != 0 {
                            ans = 1 as libc::c_int;
                            found_0 = ans;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                let mut offset_0: libc::c_int = 0;
                                log_query(F_CONFIG | F_RRNAME, name,
                                          NULL_0 as *mut all_addr,
                                          b"<SRV>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       &mut offset_0 as
                                                           *mut libc::c_int,
                                                       T_SRV as
                                                           libc::c_ushort,
                                                       C_IN as libc::c_ushort,
                                                       b"sssd\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*rec).priority,
                                                       (*rec).weight,
                                                       (*rec).srvport,
                                                       (*rec).target) != 0 {
                                    anscount += 1;
                                    if !(*rec).target.is_null() {
                                        (*rec).offset =
                                            offset_0 as libc::c_uint
                                    }
                                }
                            }
                            /* If we are returning local answers depending on network,
			       filter here. */
                            /* unlink first SRV record found */
                            if move_0.is_null() {
                                move_0 = rec;
                                *up = (*rec).next
                            } else { up = &mut (*rec).next }
                        } else { up = &mut (*rec).next }
                        rec = (*rec).next
                    }
                    /* put first SRV record back at the end. */
                    if !move_0.is_null() {
                        *up = move_0;
                        (*move_0).next = NULL_0 as *mut mx_srv_record
                    }
                    if found_0 == 0 {
                        crecp =
                            cache_find_by_name(NULL_0 as *mut crec, name, now,
                                               F_SRV |
                                                   (if dryrun != 0 {
                                                        F_NO_RR
                                                    } else {
                                                        0 as libc::c_int as
                                                            libc::c_uint
                                                    }));
                        if !crecp.is_null() && rd_bit != 0 &&
                               (do_bit == 0 ||
                                    (*dnsmasq_daemon).options[(45 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_ulong))
                                                                  as usize] &
                                        (1 as libc::c_uint) <<
                                            (45 as libc::c_int as
                                                 libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(8
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong))
                                        != 0 &&
                                        (*crecp).flags & F_DNSSECOK == 0) {
                            if (*crecp).flags & F_DNSSECOK == 0 {
                                sec_data = 0 as libc::c_int
                            }
                            auth = 0 as libc::c_int;
                            ans = 1 as libc::c_int;
                            found_0 = ans;
                            loop  {
                                if (*crecp).flags & F_NEG != 0 {
                                    if (*crecp).flags & F_NXDOMAIN != 0 {
                                        nxdomain = 1 as libc::c_int
                                    }
                                    if dryrun == 0 {
                                        log_query((*crecp).flags, name,
                                                  NULL_0 as *mut all_addr,
                                                  NULL_0 as
                                                      *mut libc::c_char);
                                    }
                                } else if dryrun == 0 {
                                    let mut target =
                                        blockdata_retrieve((*crecp).addr.srv.target,
                                                           (*crecp).addr.srv.targetlen
                                                               as size_t,
                                                           NULL_0 as
                                                               *mut libc::c_void)
                                            as *mut libc::c_char;
                                    log_query((*crecp).flags, name,
                                              NULL_0 as *mut all_addr,
                                              0 as *mut libc::c_char);
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           crec_ttl(crecp,
                                                                    now),
                                                           NULL_0 as
                                                               *mut libc::c_int,
                                                           T_SRV as
                                                               libc::c_ushort,
                                                           C_IN as
                                                               libc::c_ushort,
                                                           b"sssd\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*crecp).addr.srv.priority
                                                               as libc::c_int,
                                                           (*crecp).addr.srv.weight
                                                               as libc::c_int,
                                                           (*crecp).addr.srv.srvport
                                                               as libc::c_int,
                                                           target) != 0 {
                                        anscount += 1
                                    }
                                }
                                crecp =
                                    cache_find_by_name(crecp, name, now,
                                                       F_SRV);
                                if crecp.is_null() { break ; }
                            }
                        }
                    }
                    if found_0 == 0 &&
                           (*dnsmasq_daemon).options[(1 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (1 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               != 0 &&
                           (qtype == T_SRV as libc::c_uint ||
                                qtype == T_ANY as libc::c_uint &&
                                    !strchr(name, '_' as i32).is_null()) {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query(F_CONFIG | F_NEG, name,
                                      NULL_0 as *mut all_addr,
                                      NULL_0 as *mut libc::c_char);
                        }
                    }
                }
                if qtype == T_NAPTR as libc::c_uint ||
                       qtype == T_ANY as libc::c_uint {
                    let mut na = 0 as *mut naptr;
                    na = (*dnsmasq_daemon).naptr;
                    while !na.is_null() {
                        if hostname_isequal(name, (*na).name) != 0 {
                            ans = 1 as libc::c_int;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                log_query(F_CONFIG | F_RRNAME, name,
                                          NULL_0 as *mut all_addr,
                                          b"<NAPTR>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       NULL_0 as
                                                           *mut libc::c_int,
                                                       T_NAPTR as
                                                           libc::c_ushort,
                                                       C_IN as libc::c_ushort,
                                                       b"sszzzd\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*na).order,
                                                       (*na).pref,
                                                       (*na).flags,
                                                       (*na).services,
                                                       (*na).regexp,
                                                       (*na).replace) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                        na = (*na).next
                    }
                }
                if qtype == T_MAILB as libc::c_uint {
                    ans = 1 as libc::c_int;
                    nxdomain = 1 as libc::c_int;
                    sec_data = 0 as libc::c_int
                }
                if qtype == T_SOA as libc::c_uint &&
                       (*dnsmasq_daemon).options[(1 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (1 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 {
                    ans = 1 as libc::c_int;
                    sec_data = 0 as libc::c_int;
                    if dryrun == 0 {
                        log_query(F_CONFIG | F_NEG, name, &mut addr,
                                  NULL_0 as *mut libc::c_char);
                    }
                }
            }
            if ans == 0 { return 0 as libc::c_int as size_t }
            q -= 1
        }
        if !(dryrun != 0) { break ; }
        dryrun = 0 as libc::c_int
    }
    /* create an additional data section, for stuff in SRV and MX record replies. */
    rec = (*dnsmasq_daemon).mxnames;
    while !rec.is_null() {
        if (*rec).offset != 0 as libc::c_int as libc::c_uint {
            /* squash dupes */
            let mut tmp = 0 as *mut mx_srv_record;
            tmp = (*rec).next;
            while !tmp.is_null() {
                if (*tmp).offset != 0 as libc::c_int as libc::c_uint &&
                       hostname_isequal((*rec).target, (*tmp).target) != 0 {
                    (*tmp).offset = 0 as libc::c_int as libc::c_uint
                }
                tmp = (*tmp).next
            }
            crecp = NULL_0 as *mut crec;
            loop  {
                crecp =
                    cache_find_by_name(crecp, (*rec).target, now,
                                       F_IPV4 | F_IPV6);
                if crecp.is_null() { break ; }
                let mut type_1 =
                    if (*crecp).flags & F_IPV4 != 0 { T_A } else { T_AAAA };
                if (*crecp).flags & F_NEG != 0 { continue ; }
                if add_resource_record(header, limit,
                                       NULL_0 as *mut libc::c_int,
                                       (*rec).offset as libc::c_int,
                                       &mut ansp as *mut *mut libc::c_uchar,
                                       crec_ttl(crecp, now),
                                       NULL_0 as *mut libc::c_int,
                                       type_1 as libc::c_ushort,
                                       C_IN as libc::c_ushort,
                                       if (*crecp).flags & F_IPV4 != 0 {
                                           b"4\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"6\x00" as *const u8 as
                                               *const libc::c_char
                                       } as *mut libc::c_char,
                                       &mut (*crecp).addr as *mut all_addr) !=
                       0 {
                    addncount += 1
                }
            }
        }
        rec = (*rec).next
    }
    /* done all questions, set up header and return length of result */
  /* clear authoritative and truncated flags, set QR flag */
    (*header).hb3 =
        ((*header).hb3 as libc::c_int & !(HB3_AA | HB3_TC) | HB3_QR) as u8_0;
    /* set RA flag */
    (*header).hb4 = ((*header).hb4 as libc::c_int | HB4_RA) as u8_0;
    /* authoritative - only hosts and DHCP derived names. */
    if auth != 0 {
        (*header).hb3 = ((*header).hb3 as libc::c_int | HB3_AA) as u8_0
    }
    /* truncation */
    if trunc != 0 {
        (*header).hb3 = ((*header).hb3 as libc::c_int | HB3_TC) as u8_0
    } /* no error */
    if nxdomain != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 3 as libc::c_int) as
                u8_0
    } else if notimp != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 4 as libc::c_int) as
                u8_0
    } else {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 0 as libc::c_int) as
                u8_0
    }
    (*header).ancount = __bswap_16(anscount as __uint16_t);
    (*header).nscount = __bswap_16(0 as libc::c_int as __uint16_t);
    (*header).arcount = __bswap_16(addncount as __uint16_t);
    len =
        ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
            libc::c_long as size_t;
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        len =
            add_pseudoheader(header, len, limit as *mut libc::c_uchar,
                             (*dnsmasq_daemon).edns_pktsz, 0 as libc::c_int,
                             NULL_0 as *mut libc::c_uchar,
                             0 as libc::c_int as size_t, do_bit,
                             0 as libc::c_int)
    }
    if ad_reqd != 0 && sec_data != 0 {
        (*header).hb4 = ((*header).hb4 as libc::c_int | HB4_AD) as u8_0
    } else {
        (*header).hb4 = ((*header).hb4 as libc::c_int & !HB4_AD) as u8_0
    }
    return len;
}
