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
        pub cname: C2RustUnnamed_4,
        pub key: C2RustUnnamed_3,
        pub ds: C2RustUnnamed_2,
        pub srv: C2RustUnnamed_1,
        pub log: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "322:3"]
    pub struct C2RustUnnamed_0 {
        pub keytag: libc::c_ushort,
        pub algo: libc::c_ushort,
        pub digest: libc::c_ushort,
        pub rcode: libc::c_ushort,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:3"]
    pub struct C2RustUnnamed_1 {
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
    pub struct C2RustUnnamed_2 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
        pub digest: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:3"]
    pub struct C2RustUnnamed_3 {
        pub keydata: *mut blockdata,
        pub keylen: libc::c_ushort,
        pub flags: libc::c_ushort,
        pub keytag: libc::c_ushort,
        pub algo: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:3"]
    pub struct C2RustUnnamed_4 {
        pub target: C2RustUnnamed_5,
        pub uid: libc::c_uint,
        pub is_name_ptr: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "299:5"]
    pub union C2RustUnnamed_5 {
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
        pub name: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "455:3"]
    pub union C2RustUnnamed_6 {
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
        pub u: C2RustUnnamed_7,
        pub val: *mut libc::c_uchar,
        pub netid: *mut dhcp_netid,
        pub next: *mut dhcp_opt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "815:3"]
    pub union C2RustUnnamed_7 {
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
    #[c2rust::src_loc = "472:9"]
    pub const F_IPV4: libc::c_uint = (1 as libc::c_uint) << 7 as libc::c_int;
    #[c2rust::src_loc = "473:9"]
    pub const F_IPV6: libc::c_uint = (1 as libc::c_uint) << 8 as libc::c_int;
    #[c2rust::src_loc = "388:9"]
    pub const ADDRLIST_IPV6: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "469:9"]
    pub const F_DHCP: libc::c_uint = (1 as libc::c_uint) << 4 as libc::c_int;
    #[c2rust::src_loc = "471:9"]
    pub const F_HOSTS: libc::c_uint = (1 as libc::c_uint) << 6 as libc::c_int;
    #[c2rust::src_loc = "468:9"]
    pub const F_FORWARD: libc::c_uint =
        (1 as libc::c_uint) << 3 as libc::c_int;
    #[c2rust::src_loc = "475:9"]
    pub const F_NXDOMAIN: libc::c_uint =
        (1 as libc::c_uint) << 10 as libc::c_int;
    #[c2rust::src_loc = "470:9"]
    pub const F_NEG: libc::c_uint = (1 as libc::c_uint) << 5 as libc::c_int;
    #[c2rust::src_loc = "486:9"]
    pub const F_AUTH: libc::c_uint = (1 as libc::c_uint) << 21 as libc::c_int;
    #[c2rust::src_loc = "476:9"]
    pub const F_CNAME: libc::c_uint =
        (1 as libc::c_uint) << 11 as libc::c_int;
    #[c2rust::src_loc = "478:9"]
    pub const F_CONFIG: libc::c_uint =
        (1 as libc::c_uint) << 13 as libc::c_int;
    #[c2rust::src_loc = "482:9"]
    pub const F_RRNAME: libc::c_uint =
        (1 as libc::c_uint) << 17 as libc::c_int;
    #[c2rust::src_loc = "171:9"]
    pub const ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "389:9"]
    pub const ADDRLIST_REVONLY: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "467:9"]
    pub const F_REVERSE: libc::c_uint =
        (1 as libc::c_uint) << 2 as libc::c_int;
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
        #[c2rust::src_loc = "1199:1"]
        pub fn cache_get_name(crecp: *mut crec) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1201:1"]
        pub fn cache_enumerate(init: libc::c_int) -> *mut crec;
        #[no_mangle]
        #[c2rust::src_loc = "1221:1"]
        pub fn extract_name(header: *mut dns_header, plen: size_t,
                            pp: *mut *mut libc::c_uchar,
                            name: *mut libc::c_char, isExtract: libc::c_int,
                            extrabytes: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1224:1"]
        pub fn skip_questions(header: *mut dns_header, plen: size_t)
         -> *mut libc::c_uchar;
        #[no_mangle]
        #[c2rust::src_loc = "1243:1"]
        pub fn add_resource_record(header: *mut dns_header,
                                   limit: *mut libc::c_char,
                                   truncp: *mut libc::c_int,
                                   nameoffset: libc::c_int,
                                   pp: *mut *mut libc::c_uchar,
                                   ttl: libc::c_ulong,
                                   offset: *mut libc::c_int,
                                   type_0: libc::c_ushort,
                                   class: libc::c_ushort,
                                   format: *mut libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1246:1"]
        pub fn in_arpa_name_2_addr(namein: *mut libc::c_char,
                                   addrp: *mut all_addr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1664:1"]
        pub fn add_pseudoheader(header: *mut dns_header, plen: size_t,
                                limit: *mut libc::c_uchar,
                                udp_sz: libc::c_ushort, optno: libc::c_int,
                                opt: *mut libc::c_uchar, optlen: size_t,
                                set_do: libc::c_int, replace: libc::c_int)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "1299:1"]
        pub fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                            prefixlen: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1298:1"]
        pub fn is_same_net(a: in_addr, b: in_addr, mask: in_addr)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1294:1"]
        pub fn hostname_isequal(a: *const libc::c_char,
                                b: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char,
                         _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1293:1"]
        pub fn sockaddr_isequal(s1: *mut mysockaddr, s2: *mut mysockaddr)
         -> libc::c_int;
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
    #[c2rust::src_loc = "101:9"]
    pub const HB4_RCODE: libc::c_int = 0xf as libc::c_int;
    #[c2rust::src_loc = "95:9"]
    pub const HB3_TC: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const HB3_AA: libc::c_int = 0x4 as libc::c_int;
    #[c2rust::src_loc = "99:9"]
    pub const HB4_AD: libc::c_int = 0x20 as libc::c_int;
    #[c2rust::src_loc = "98:9"]
    pub const HB4_RA: libc::c_int = 0x80 as libc::c_int;
    #[c2rust::src_loc = "92:9"]
    pub const HB3_QR: libc::c_int = 0x80 as libc::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const C_IN: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const T_SOA: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const T_A: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const T_AAAA: libc::c_int = 28 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const T_CNAME: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "65:9"]
    pub const T_NAPTR: libc::c_int = 35 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const T_TXT: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const T_MX: libc::c_int = 15 as libc::c_int;
    #[c2rust::src_loc = "64:9"]
    pub const T_SRV: libc::c_int = 33 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const T_NS: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "76:9"]
    pub const T_AXFR: libc::c_int = 252 as libc::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const T_PTR: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const QUERY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "93:9"]
    pub const HB3_OPCODE: libc::c_int = 0x78 as libc::c_int;
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:17"]
pub mod byteswap_h {
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
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::{__uint32_t, __uint64_t, __uint16_t};
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
    use super::socket_h::socklen_t;
    extern "C" {
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
#[c2rust::header_src = "/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
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
        #[c2rust::src_loc = "226:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:17"]
pub mod syslog_h {
    #[c2rust::src_loc = "55:9"]
    pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
}
pub use self::internal::__va_list_tag;
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
pub use self::socket_h::{socklen_t, sockaddr, msghdr, cmsghdr, __cmsg_nxthdr,
                         PF_INET6, AF_INET6, PF_INET, AF_INET};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, INET6_ADDRSTRLEN};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::dnsmasq_h::{u8_0, u16_0, u32_0, all_addr, C2RustUnnamed_0,
                          C2RustUnnamed_1, blockdata, C2RustUnnamed_2,
                          C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5,
                          crec, C2RustUnnamed_6, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_netid, dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_7, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, F_IPV4, F_IPV6,
                          ADDRLIST_IPV6, F_DHCP, F_HOSTS, F_FORWARD,
                          F_NXDOMAIN, F_NEG, F_AUTH, F_CNAME, F_CONFIG,
                          F_RRNAME, ADDRSTRLEN, ADDRLIST_REVONLY, F_REVERSE,
                          log_query, record_source, querystr,
                          cache_find_non_terminal, cache_find_by_addr,
                          cache_find_by_name, cache_get_name, cache_enumerate,
                          extract_name, skip_questions, add_resource_record,
                          in_arpa_name_2_addr, add_pseudoheader, is_same_net6,
                          is_same_net, hostname_isequal, my_syslog,
                          sockaddr_isequal, hostname_issubdomain};
pub use self::dns_protocol_h::{dns_header, HB4_RCODE, HB3_TC, HB3_AA, HB4_AD,
                               HB4_RA, HB3_QR, C_IN, T_SOA, T_A, T_AAAA,
                               T_CNAME, T_NAPTR, T_TXT, T_MX, T_SRV, T_NS,
                               T_AXFR, T_PTR, QUERY, HB3_OPCODE};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER};
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
use self::stdio_h::{stdin, stdout, sprintf, vfprintf, getc, putc, __getdelim,
                    __uflow, __overflow};
pub use self::byteswap_h::{__bswap_32, __bswap_64, __bswap_16};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity,
                                 __uint64_identity};
use self::inet_h::inet_ntop;
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::string_h::{strcpy, strcat, strcmp, strchr, strlen};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
pub use self::ctype_h::{tolower, toupper, __ctype_tolower_loc,
                        __ctype_toupper_loc};
pub use self::syslog_h::LOG_WARNING;
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
#[c2rust::src_loc = "21:1"]
unsafe extern "C" fn find_addrlist(mut list: *mut addrlist,
                                   mut flag: libc::c_int,
                                   mut addr_u: *mut all_addr)
 -> *mut addrlist {
    loop  {
        if (*list).flags & ADDRLIST_IPV6 == 0 {
            let mut netmask = in_addr{s_addr: 0,};
            let mut addr = (*addr_u).addr4;
            if !(flag as libc::c_uint & F_IPV4 == 0) {
                netmask.s_addr =
                    __bswap_32((!(0 as libc::c_int as in_addr_t)) <<
                                   32 as libc::c_int - (*list).prefixlen);
                if is_same_net(addr, (*list).addr.addr4, netmask) != 0 {
                    return list
                }
            }
        } else if is_same_net6(&mut (*addr_u).addr6, &mut (*list).addr.addr6,
                               (*list).prefixlen) != 0 {
            return list
        }
        list = (*list).next;
        if list.is_null() { break ; }
    }
    return NULL_0 as *mut addrlist;
}
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn find_subnet(mut zone: *mut auth_zone,
                                 mut flag: libc::c_int,
                                 mut addr_u: *mut all_addr) -> *mut addrlist {
    if (*zone).subnet.is_null() { return NULL_0 as *mut addrlist }
    return find_addrlist((*zone).subnet, flag, addr_u);
}
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn find_exclude(mut zone: *mut auth_zone,
                                  mut flag: libc::c_int,
                                  mut addr_u: *mut all_addr)
 -> *mut addrlist {
    if (*zone).exclude.is_null() { return NULL_0 as *mut addrlist }
    return find_addrlist((*zone).exclude, flag, addr_u);
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn filter_zone(mut zone: *mut auth_zone,
                                 mut flag: libc::c_int,
                                 mut addr_u: *mut all_addr) -> libc::c_int {
    if !find_exclude(zone, flag, addr_u).is_null() { return 0 as libc::c_int }
    /* No subnets specified, no filter */
    if (*zone).subnet.is_null() { return 1 as libc::c_int }
    return (find_subnet(zone, flag, addr_u) != NULL_0 as *mut addrlist) as
               libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "72:1"]
pub unsafe extern "C" fn in_zone(mut zone: *mut auth_zone,
                                 mut name: *mut libc::c_char,
                                 mut cut: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut namelen = strlen(name);
    let mut domainlen = strlen((*zone).domain);
    if !cut.is_null() { *cut = NULL_0 as *mut libc::c_char }
    if namelen >= domainlen &&
           hostname_isequal((*zone).domain,
                            &mut *name.offset(namelen.wrapping_sub(domainlen)
                                                  as isize)) != 0 {
        if namelen == domainlen { return 1 as libc::c_int }
        if *name.offset(namelen.wrapping_sub(domainlen).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                            as isize) as libc::c_int == '.' as i32 {
            if !cut.is_null() {
                *cut =
                    &mut *name.offset(namelen.wrapping_sub(domainlen).wrapping_sub(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                          as isize) as *mut libc::c_char
            }
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn answer_auth(mut header: *mut dns_header,
                                     mut limit: *mut libc::c_char,
                                     mut qlen: size_t, mut now: time_t,
                                     mut peer_addr: *mut mysockaddr,
                                     mut local_query: libc::c_int,
                                     mut do_bit: libc::c_int,
                                     mut have_pseudoheader: libc::c_int)
 -> size_t {
    let mut name = (*dnsmasq_daemon).namebuff;
    let mut p = 0 as *mut libc::c_uchar;
    let mut ansp = 0 as *mut libc::c_uchar;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut nameoffset: libc::c_int = 0;
    let mut axfroffset = 0 as libc::c_int;
    let mut q: libc::c_int = 0;
    let mut anscount = 0 as libc::c_int;
    let mut authcount = 0 as libc::c_int;
    let mut crecp = 0 as *mut crec;
    let mut auth = (local_query == 0) as libc::c_int;
    let mut trunc = 0 as libc::c_int;
    let mut nxdomain = 1 as libc::c_int;
    let mut soa = 0 as libc::c_int;
    let mut ns = 0 as libc::c_int;
    let mut axfr = 0 as libc::c_int;
    let mut zone = NULL_0 as *mut auth_zone;
    let mut subnet = NULL_0 as *mut addrlist;
    let mut cut = 0 as *mut libc::c_char;
    let mut rec = 0 as *mut mx_srv_record;
    let mut move_0 = 0 as *mut mx_srv_record;
    let mut up = 0 as *mut *mut mx_srv_record;
    let mut txt = 0 as *mut txt_record;
    let mut intr = 0 as *mut interface_name;
    let mut na = 0 as *mut naptr;
    let mut addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut a = 0 as *mut cname;
    let mut candidate = 0 as *mut cname;
    let mut wclen: libc::c_uint = 0;
    if __bswap_16((*header).qdcount) as libc::c_int == 0 as libc::c_int ||
           ((*header).hb3 as libc::c_int & HB3_OPCODE) >> 3 as libc::c_int !=
               QUERY {
        return 0 as libc::c_int as size_t
    }
    /* determine end of question section (we put answers there) */
    ansp = skip_questions(header, qlen); /* bad packet */
    if ansp.is_null() { return 0 as libc::c_int as size_t }
    /* now process each question, answers go in RRs after the question */
    p = header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut current_block_247: u64;
    q = __bswap_16((*header).qdcount) as libc::c_int;
    while q != 0 as libc::c_int {
        let mut flag = 0 as libc::c_int as libc::c_uint;
        let mut found = 0 as libc::c_int;
        let mut cname_wildcard = 0 as libc::c_int;
        /* save pointer to name for copying into answers */
        nameoffset =
            p.wrapping_offset_from(header as *mut libc::c_uchar) as
                libc::c_long as libc::c_int;
        /* now extract name as .-concatenated string into name */
        if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                        4 as libc::c_int) == 0 {
            return 0 as libc::c_int as size_t
        } /* bad packet */
        let mut t_cp = p; /* must be bare name */
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
        if qclass != C_IN {
            auth = 0 as libc::c_int
        } else {
            if (qtype == T_PTR || qtype == T_SOA || qtype == T_NS) &&
                   {
                       flag =
                           in_arpa_name_2_addr(name, &mut addr) as
                               libc::c_uint;
                       (flag) != 0
                   } && local_query == 0 {
                zone = (*dnsmasq_daemon).auth_zones;
                while !zone.is_null() {
                    subnet =
                        find_subnet(zone, flag as libc::c_int, &mut addr);
                    if !subnet.is_null() { break ; }
                    zone = (*zone).next
                }
                if zone.is_null() {
                    auth = 0 as libc::c_int;
                    current_block_247 = 17860125682698302841;
                } else {
                    if qtype == T_SOA {
                        soa = 1 as libc::c_int;
                        found = 1 as libc::c_int
                    } else if qtype == T_NS {
                        ns = 1 as libc::c_int;
                        found = 1 as libc::c_int
                    }
                    current_block_247 = 4567019141635105728;
                }
            } else { current_block_247 = 4567019141635105728; }
            match current_block_247 {
                17860125682698302841 => { }
                _ => {
                    if qtype == T_PTR && flag != 0 {
                        intr = NULL_0 as *mut interface_name;
                        if flag == F_IPV4 {
                            intr = (*dnsmasq_daemon).int_names;
                            while !intr.is_null() {
                                let mut addrlist = 0 as *mut addrlist;
                                addrlist = (*intr).addr;
                                while !addrlist.is_null() {
                                    if (*addrlist).flags & ADDRLIST_IPV6 == 0
                                           &&
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
                        } else if flag == F_IPV6 {
                            intr = (*dnsmasq_daemon).int_names;
                            while !intr.is_null() {
                                let mut addrlist_0 = 0 as *mut addrlist;
                                addrlist_0 = (*intr).addr;
                                while !addrlist_0.is_null() {
                                    if (*addrlist_0).flags & ADDRLIST_IPV6 !=
                                           0 &&
                                           ({
                                                let mut __a =
                                                    &mut addr.addr6 as
                                                        *mut in6_addr as
                                                        *const in6_addr;
                                                let mut __b =
                                                    &mut (*addrlist_0).addr.addr6
                                                        as *mut in6_addr as
                                                        *const in6_addr;
                                                ((*__a).__in6_u.__u6_addr32[0
                                                                                as
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
                            if local_query != 0 ||
                                   in_zone(zone, (*intr).name,
                                           NULL_0 as *mut *mut libc::c_char)
                                       != 0 {
                                found = 1 as libc::c_int;
                                log_query(flag | F_REVERSE | F_CONFIG,
                                          (*intr).name, &mut addr,
                                          NULL_0 as *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).auth_ttl,
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
                                                       (*intr).name) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                        crecp =
                            cache_find_by_addr(NULL_0 as *mut crec, &mut addr,
                                               now, flag);
                        if !crecp.is_null() {
                            loop  {
                                strcpy(name, cache_get_name(crecp));
                                if (*crecp).flags & F_DHCP != 0 &&
                                       (*dnsmasq_daemon).options[(20 as
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
                                               (20 as libc::c_int as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                           == 0 {
                                    let mut p_0 = strchr(name, '.' as i32);
                                    if !p_0.is_null() {
                                        *p_0 =
                                            0 as libc::c_int as libc::c_char
                                    }
                                    /* add  external domain */
                                    if !zone.is_null() {
                                        strcat(name,
                                               b".\x00" as *const u8 as
                                                   *const libc::c_char);
                                        strcat(name, (*zone).domain);
                                    }
                                    log_query(flag | F_DHCP | F_REVERSE, name,
                                              &mut addr,
                                              record_source((*crecp).uid));
                                    found = 1 as libc::c_int;
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).auth_ttl,
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
                                                           name) != 0 {
                                        anscount += 1
                                    }
                                } else if (*crecp).flags & (F_DHCP | F_HOSTS)
                                              != 0 &&
                                              (local_query != 0 ||
                                                   in_zone(zone, name,
                                                           NULL_0 as
                                                               *mut *mut libc::c_char)
                                                       != 0) {
                                    log_query((*crecp).flags & !F_FORWARD,
                                              name, &mut addr,
                                              record_source((*crecp).uid));
                                    found = 1 as libc::c_int;
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).auth_ttl,
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
                                                           name) != 0 {
                                        anscount += 1
                                    }
                                }
                                crecp =
                                    cache_find_by_addr(crecp, &mut addr, now,
                                                       flag);
                                if crecp.is_null() { break ; }
                            }
                        }
                        if found != 0 {
                            nxdomain = 0 as libc::c_int
                        } else {
                            log_query(flag | F_NEG | F_NXDOMAIN | F_REVERSE |
                                          (if auth != 0 {
                                               F_AUTH
                                           } else {
                                               0 as libc::c_int as
                                                   libc::c_uint
                                           }), NULL_0 as *mut libc::c_char,
                                      &mut addr, NULL_0 as *mut libc::c_char);
                        }
                    } else {
                        loop  {
                            if found != 0 {
                                /* NS and SOA .arpa requests have set found above. */
                                cut = NULL_0 as *mut libc::c_char
                            } else {
                                zone = (*dnsmasq_daemon).auth_zones;
                                while !zone.is_null() {
                                    if in_zone(zone, name, &mut cut) != 0 {
                                        break ;
                                    }
                                    zone = (*zone).next
                                }
                                if zone.is_null() {
                                    auth = 0 as libc::c_int;
                                    break ;
                                }
                            }
                            rec = (*dnsmasq_daemon).mxnames;
                            while !rec.is_null() {
                                if (*rec).issrv == 0 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*rec).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int && qtype == T_MX
                                       {
                                        found = 1 as libc::c_int;
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
                                                               (*dnsmasq_daemon).auth_ttl,
                                                               NULL_0 as
                                                                   *mut libc::c_int,
                                                               T_MX as
                                                                   libc::c_ushort,
                                                               C_IN as
                                                                   libc::c_ushort,
                                                               b"sd\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*rec).weight,
                                                               (*rec).target)
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                rec = (*rec).next
                            }
                            move_0 = NULL_0 as *mut mx_srv_record;
                            up = &mut (*dnsmasq_daemon).mxnames;
                            rec = (*dnsmasq_daemon).mxnames;
                            while !rec.is_null() {
                                if (*rec).issrv != 0 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*rec).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == T_SRV {
                                        found = 1 as libc::c_int;
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
                                                               (*dnsmasq_daemon).auth_ttl,
                                                               NULL_0 as
                                                                   *mut libc::c_int,
                                                               T_SRV as
                                                                   libc::c_ushort,
                                                               C_IN as
                                                                   libc::c_ushort,
                                                               b"sssd\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*rec).priority,
                                                               (*rec).weight,
                                                               (*rec).srvport,
                                                               (*rec).target)
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
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
                                *up = move_0; /* inhibits auth section */
                                (*move_0).next = NULL_0 as *mut mx_srv_record
                            }
                            txt = (*dnsmasq_daemon).rr;
                            while !txt.is_null() {
                                rc = hostname_issubdomain(name, (*txt).name);
                                if rc != 0 {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           (*txt).class as libc::c_int ==
                                               qtype {
                                        found = 1 as libc::c_int;
                                        log_query(F_CONFIG | F_RRNAME, name,
                                                  NULL_0 as *mut all_addr,
                                                  querystr(NULL_0 as
                                                               *mut libc::c_char,
                                                           (*txt).class));
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
                                                               NULL_0 as
                                                                   *mut libc::c_int,
                                                               (*txt).class,
                                                               C_IN as
                                                                   libc::c_ushort,
                                                               b"t\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*txt).len as
                                                                   libc::c_int,
                                                               (*txt).txt) !=
                                               0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                txt = (*txt).next
                            }
                            txt = (*dnsmasq_daemon).txt;
                            while !txt.is_null() {
                                if (*txt).class as libc::c_int == C_IN &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*txt).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == T_TXT {
                                        found = 1 as libc::c_int;
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
                                                               (*dnsmasq_daemon).auth_ttl,
                                                               NULL_0 as
                                                                   *mut libc::c_int,
                                                               T_TXT as
                                                                   libc::c_ushort,
                                                               C_IN as
                                                                   libc::c_ushort,
                                                               b"t\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*txt).len as
                                                                   libc::c_int,
                                                               (*txt).txt) !=
                                               0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                txt = (*txt).next
                            }
                            na = (*dnsmasq_daemon).naptr;
                            while !na.is_null() {
                                rc = hostname_issubdomain(name, (*na).name);
                                if rc != 0 {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == T_NAPTR {
                                        found = 1 as libc::c_int;
                                        log_query(F_CONFIG | F_RRNAME, name,
                                                  NULL_0 as *mut all_addr,
                                                  b"<NAPTR>\x00" as *const u8
                                                      as *const libc::c_char
                                                      as *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
                                                               NULL_0 as
                                                                   *mut libc::c_int,
                                                               T_NAPTR as
                                                                   libc::c_ushort,
                                                               C_IN as
                                                                   libc::c_ushort,
                                                               b"sszzzd\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*na).order,
                                                               (*na).pref,
                                                               (*na).flags,
                                                               (*na).services,
                                                               (*na).regexp,
                                                               (*na).replace)
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                na = (*na).next
                            }
                            if qtype == T_A { flag = F_IPV4 }
                            if qtype == T_AAAA { flag = F_IPV6 }
                            intr = (*dnsmasq_daemon).int_names;
                            while !intr.is_null() {
                                rc = hostname_issubdomain(name, (*intr).name);
                                if rc != 0 {
                                    let mut addrlist_1 = 0 as *mut addrlist;
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int && flag != 0 {
                                        addrlist_1 = (*intr).addr;
                                        while !addrlist_1.is_null() {
                                            if (if (*addrlist_1).flags &
                                                       ADDRLIST_IPV6 != 0 {
                                                    T_AAAA
                                                } else { T_A }) == qtype &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag as
                                                                        libc::c_int,
                                                                    &mut (*addrlist_1).addr)
                                                            != 0) {
                                                if !((*addrlist_1).flags &
                                                         ADDRLIST_REVONLY !=
                                                         0) {
                                                    found = 1 as libc::c_int;
                                                    log_query(F_FORWARD |
                                                                  F_CONFIG |
                                                                  flag, name,
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
                                                                           (*dnsmasq_daemon).auth_ttl,
                                                                           NULL_0
                                                                               as
                                                                               *mut libc::c_int,
                                                                           qtype
                                                                               as
                                                                               libc::c_ushort,
                                                                           C_IN
                                                                               as
                                                                               libc::c_ushort,
                                                                           if qtype
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
                                            addrlist_1 = (*addrlist_1).next
                                        }
                                    }
                                }
                                intr = (*intr).next
                            }
                            if cut.is_null() {
                                nxdomain = 0 as libc::c_int;
                                if qtype == T_SOA {
                                    soa = 1 as libc::c_int;
                                    auth = soa;
                                    found = 1 as libc::c_int;
                                    log_query(F_RRNAME | F_AUTH,
                                              (*zone).domain,
                                              NULL_0 as *mut all_addr,
                                              b"<SOA>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                } else if qtype == T_AXFR {
                                    let mut peers = 0 as *mut iname;
                                    if (*peer_addr).sa.sa_family as
                                           libc::c_int == AF_INET {
                                        (*peer_addr).in_0.sin_port =
                                            0 as libc::c_int as in_port_t
                                    } else {
                                        (*peer_addr).in6.sin6_port =
                                            0 as libc::c_int as in_port_t;
                                        (*peer_addr).in6.sin6_scope_id =
                                            0 as libc::c_int as uint32_t
                                    }
                                    peers = (*dnsmasq_daemon).auth_peers;
                                    while !peers.is_null() {
                                        if sockaddr_isequal(peer_addr,
                                                            &mut (*peers).addr)
                                               != 0 {
                                            break ;
                                        }
                                        peers = (*peers).next
                                    }
                                    /* Refuse all AXFR unless --auth-sec-servers or auth-peers is set */
                                    if (*dnsmasq_daemon).secondary_forward_server.is_null()
                                           &&
                                           (*dnsmasq_daemon).auth_peers.is_null()
                                           ||
                                           !(*dnsmasq_daemon).auth_peers.is_null()
                                               && peers.is_null() {
                                        if (*peer_addr).sa.sa_family as
                                               libc::c_int == AF_INET {
                                            inet_ntop(AF_INET,
                                                      &mut (*peer_addr).in_0.sin_addr
                                                          as *mut in_addr as
                                                          *const libc::c_void,
                                                      (*dnsmasq_daemon).addrbuff,
                                                      ADDRSTRLEN as
                                                          socklen_t); /* inhibits auth section */
                                        } else {
                                            inet_ntop(AF_INET6,
                                                      &mut (*peer_addr).in6.sin6_addr
                                                          as *mut in6_addr as
                                                          *const libc::c_void,
                                                      (*dnsmasq_daemon).addrbuff,
                                                      ADDRSTRLEN as
                                                          socklen_t); /* ensure we include NS records! */
                                        } /* inhibits auth section */
                                        my_syslog(LOG_WARNING,
                                                  b"ignoring zone transfer request from %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  (*dnsmasq_daemon).addrbuff); /* remove domain part */
                                        return 0 as libc::c_int as size_t
                                    }
                                    auth = 1 as libc::c_int;
                                    soa = 1 as libc::c_int;
                                    ns = 1 as libc::c_int;
                                    axfr = 1 as libc::c_int;
                                    found = 1 as libc::c_int;
                                    axfroffset = nameoffset;
                                    log_query(F_RRNAME | F_AUTH,
                                              (*zone).domain,
                                              NULL_0 as *mut all_addr,
                                              b"<AXFR>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                } else if qtype == T_NS {
                                    auth = 1 as libc::c_int;
                                    ns = 1 as libc::c_int;
                                    found = 1 as libc::c_int;
                                    log_query(F_RRNAME | F_AUTH,
                                              (*zone).domain,
                                              NULL_0 as *mut all_addr,
                                              b"<NS>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                }
                            }
                            if (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (20 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   == 0 && !cut.is_null() {
                                *cut = 0 as libc::c_int as libc::c_char;
                                if strchr(name, '.' as i32).is_null() &&
                                       {
                                           crecp =
                                               cache_find_by_name(NULL_0 as
                                                                      *mut crec,
                                                                  name, now,
                                                                  F_IPV4 |
                                                                      F_IPV6);
                                           !crecp.is_null()
                                       } {
                                    if (*crecp).flags & F_DHCP != 0 {
                                        loop  {
                                            nxdomain = 0 as libc::c_int;
                                            if (*crecp).flags & flag != 0 &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag as
                                                                        libc::c_int,
                                                                    &mut (*crecp).addr)
                                                            != 0) {
                                                /* restore domain part */
                                                *cut =
                                                    '.' as i32 as
                                                        libc::c_char; /* restore domain part */
                                                log_query((*crecp).flags,
                                                          name,
                                                          &mut (*crecp).addr,
                                                          record_source((*crecp).uid)); /* remove domain part */
                                                *cut =
                                                    0 as libc::c_int as
                                                        libc::c_char;
                                                found = 1 as libc::c_int;
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       (*dnsmasq_daemon).auth_ttl,
                                                                       NULL_0
                                                                           as
                                                                           *mut libc::c_int,
                                                                       qtype
                                                                           as
                                                                           libc::c_ushort,
                                                                       C_IN as
                                                                           libc::c_ushort,
                                                                       if qtype
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
                                            crecp =
                                                cache_find_by_name(crecp,
                                                                   name, now,
                                                                   F_IPV4 |
                                                                       F_IPV6);
                                            if crecp.is_null() { break ; }
                                        }
                                    }
                                }
                                *cut = '.' as i32 as libc::c_char
                            }
                            crecp =
                                cache_find_by_name(NULL_0 as *mut crec, name,
                                                   now, F_IPV4 | F_IPV6);
                            if !crecp.is_null() {
                                if (*crecp).flags & F_HOSTS != 0 ||
                                       (*crecp).flags & F_DHCP != 0 &&
                                           (*dnsmasq_daemon).options[(20 as
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
                                                   (20 as libc::c_int as
                                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_ulong))
                                               != 0 {
                                    loop  {
                                        nxdomain = 0 as libc::c_int;
                                        if (*crecp).flags & flag != 0 &&
                                               (local_query != 0 ||
                                                    filter_zone(zone,
                                                                flag as
                                                                    libc::c_int,
                                                                &mut (*crecp).addr)
                                                        != 0) {
                                            log_query((*crecp).flags, name,
                                                      &mut (*crecp).addr,
                                                      record_source((*crecp).uid));
                                            found = 1 as libc::c_int;
                                            if add_resource_record(header,
                                                                   limit,
                                                                   &mut trunc
                                                                       as
                                                                       *mut libc::c_int,
                                                                   nameoffset,
                                                                   &mut ansp
                                                                       as
                                                                       *mut *mut libc::c_uchar,
                                                                   (*dnsmasq_daemon).auth_ttl,
                                                                   NULL_0 as
                                                                       *mut libc::c_int,
                                                                   qtype as
                                                                       libc::c_ushort,
                                                                   C_IN as
                                                                       libc::c_ushort,
                                                                   if qtype ==
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
                                        crecp =
                                            cache_find_by_name(crecp, name,
                                                               now,
                                                               F_IPV4 |
                                                                   F_IPV6);
                                        if crecp.is_null() { break ; }
                                    }
                                }
                            }
                            /* Only supply CNAME if no record for any type is known. */
                            if !(nxdomain != 0) { break ; }
                            /* Check for possible wildcard match against *.domain 
	     return length of match, to get longest.
	     Note that if return length of wildcard section, so
	     we match b.simon to _both_ *.simon and b.simon
	     but return a longer (better) match to b.simon.
	  */
                            wclen = 0 as libc::c_int as libc::c_uint;
                            candidate = NULL_0 as *mut cname;
                            a = (*dnsmasq_daemon).cnames;
                            while !a.is_null() {
                                if *(*a).alias.offset(0 as libc::c_int as
                                                          isize) as
                                       libc::c_int == '*' as i32 {
                                    let mut test = name;
                                    loop  {
                                        test =
                                            strchr(test.offset(1 as
                                                                   libc::c_int
                                                                   as isize),
                                                   '.' as i32);
                                        if test.is_null() { break ; }
                                        if !(hostname_isequal(test,
                                                              &mut *(*a).alias.offset(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize))
                                                 != 0) {
                                            continue ;
                                        }
                                        if strlen(test) >
                                               wclen as libc::c_ulong &&
                                               cname_wildcard == 0 {
                                            wclen =
                                                strlen(test) as libc::c_uint;
                                            candidate = a;
                                            cname_wildcard = 1 as libc::c_int
                                        }
                                        break ;
                                    }
                                } else if hostname_isequal((*a).alias, name)
                                              != 0 &&
                                              strlen((*a).alias) >
                                                  wclen as libc::c_ulong {
                                    /* Simple case, no wildcard */
                                    wclen =
                                        strlen((*a).alias) as libc::c_uint;
                                    candidate = a
                                }
                                a = (*a).next
                            }
                            if !candidate.is_null() {
                                log_query(F_CONFIG | F_CNAME, name,
                                          NULL_0 as *mut all_addr,
                                          NULL_0 as *mut libc::c_char);
                                strcpy(name, (*candidate).target);
                                if strchr(name, '.' as i32).is_null() {
                                    strcat(name,
                                           b".\x00" as *const u8 as
                                               *const libc::c_char);
                                    strcat(name, (*zone).domain);
                                }
                                found = 1 as libc::c_int;
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).auth_ttl,
                                                       &mut nameoffset as
                                                           *mut libc::c_int,
                                                       T_CNAME as
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
                            } else {
                                if cache_find_non_terminal(name, now) != 0 {
                                    nxdomain = 0 as libc::c_int
                                }
                                log_query(flag | F_NEG |
                                              (if nxdomain != 0 {
                                                   F_NXDOMAIN
                                               } else {
                                                   0 as libc::c_int as
                                                       libc::c_uint
                                               }) | F_FORWARD | F_AUTH, name,
                                          NULL_0 as *mut all_addr,
                                          NULL_0 as *mut libc::c_char);
                                break ;
                            }
                        }
                    }
                }
            }
        }
        q -= 1
    }
    /* Add auth section */
    if auth != 0 && !zone.is_null() {
        let mut authname = 0 as *mut libc::c_char;
        let mut newoffset: libc::c_int = 0;
        let mut offset = 0 as libc::c_int;
        if subnet.is_null() {
            authname = (*zone).domain
        } else {
            /* handle NS and SOA for PTR records */
            authname = name;
            if (*subnet).flags & ADDRLIST_IPV6 == 0 {
                let mut a_0 =
                    __bswap_32((*subnet).addr.addr4.s_addr) >>
                        8 as libc::c_int;
                let mut p_1 = name;
                if (*subnet).prefixlen >= 24 as libc::c_int {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" as *const u8 as
                                               *const libc::c_char,
                                           a_0 &
                                               0xff as libc::c_int as
                                                   libc::c_uint) as isize)
                }
                a_0 = a_0 >> 8 as libc::c_int;
                if (*subnet).prefixlen >= 16 as libc::c_int {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" as *const u8 as
                                               *const libc::c_char,
                                           a_0 &
                                               0xff as libc::c_int as
                                                   libc::c_uint) as isize)
                }
                a_0 = a_0 >> 8 as libc::c_int;
                p_1 =
                    p_1.offset(sprintf(p_1,
                                       b"%u.in-addr.arpa\x00" as *const u8 as
                                           *const libc::c_char,
                                       a_0 &
                                           0xff as libc::c_int as
                                               libc::c_uint) as isize)
            } else {
                let mut p_2 = name;
                let mut i: libc::c_int = 0;
                i = (*subnet).prefixlen - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    let mut dig =
                        *(&mut (*subnet).addr.addr6 as *mut in6_addr as
                              *mut libc::c_uchar).offset((i >>
                                                              3 as
                                                                  libc::c_int)
                                                             as isize) as
                            libc::c_int;
                    p_2 =
                        p_2.offset(sprintf(p_2,
                                           b"%.1x.\x00" as *const u8 as
                                               *const libc::c_char,
                                           if i >> 2 as libc::c_int &
                                                  1 as libc::c_int != 0 {
                                               (dig) & 15 as libc::c_int
                                           } else {
                                               (dig) >> 4 as libc::c_int
                                           }) as isize);
                    i -= 4 as libc::c_int
                }
                p_2 =
                    p_2.offset(sprintf(p_2,
                                       b"ip6.arpa\x00" as *const u8 as
                                           *const libc::c_char) as isize)
            }
        }
        /* handle NS and SOA in auth section or for explicit queries */
        newoffset =
            ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                libc::c_long as libc::c_int;
        if (anscount == 0 as libc::c_int && ns == 0 || soa != 0) &&
               add_resource_record(header, limit,
                                   &mut trunc as *mut libc::c_int,
                                   0 as libc::c_int,
                                   &mut ansp as *mut *mut libc::c_uchar,
                                   (*dnsmasq_daemon).auth_ttl,
                                   NULL_0 as *mut libc::c_int,
                                   T_SOA as libc::c_ushort,
                                   C_IN as libc::c_ushort,
                                   b"ddlllll\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, authname,
                                   (*dnsmasq_daemon).authserver,
                                   (*dnsmasq_daemon).hostmaster,
                                   (*dnsmasq_daemon).soa_sn,
                                   (*dnsmasq_daemon).soa_refresh,
                                   (*dnsmasq_daemon).soa_retry,
                                   (*dnsmasq_daemon).soa_expiry,
                                   (*dnsmasq_daemon).auth_ttl) != 0 {
            offset = newoffset;
            if soa != 0 { anscount += 1 } else { authcount += 1 }
        }
        if anscount != 0 as libc::c_int || ns != 0 {
            let mut secondary = 0 as *mut name_list;
            /* Only include the machine running dnsmasq if it's acting as an auth server */
            if !(*dnsmasq_daemon).authinterface.is_null() {
                newoffset =
                    ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                        libc::c_long as libc::c_int;
                if add_resource_record(header, limit,
                                       &mut trunc as *mut libc::c_int,
                                       -offset,
                                       &mut ansp as *mut *mut libc::c_uchar,
                                       (*dnsmasq_daemon).auth_ttl,
                                       NULL_0 as *mut libc::c_int,
                                       T_NS as libc::c_ushort,
                                       C_IN as libc::c_ushort,
                                       b"d\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char,
                                       if offset == 0 as libc::c_int {
                                           authname
                                       } else { NULL_0 as *mut libc::c_char },
                                       (*dnsmasq_daemon).authserver) != 0 {
                    if offset == 0 as libc::c_int { offset = newoffset }
                    if ns != 0 { anscount += 1 } else { authcount += 1 }
                }
            }
            if subnet.is_null() {
                secondary = (*dnsmasq_daemon).secondary_forward_server;
                while !secondary.is_null() {
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           offset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           NULL_0 as *mut libc::c_int,
                                           T_NS as libc::c_ushort,
                                           C_IN as libc::c_ushort,
                                           b"d\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           (*secondary).name) != 0 {
                        if ns != 0 { anscount += 1 } else { authcount += 1 }
                    }
                    secondary = (*secondary).next
                }
            }
        }
        if axfr != 0 {
            rec = (*dnsmasq_daemon).mxnames;
            while !rec.is_null() {
                if in_zone(zone, (*rec).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if (*rec).issrv != 0 {
                        if add_resource_record(header, limit,
                                               &mut trunc as *mut libc::c_int,
                                               -axfroffset,
                                               &mut ansp as
                                                   *mut *mut libc::c_uchar,
                                               (*dnsmasq_daemon).auth_ttl,
                                               NULL_0 as *mut libc::c_int,
                                               T_SRV as libc::c_ushort,
                                               C_IN as libc::c_ushort,
                                               b"sssd\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               if !cut.is_null() {
                                                   (*rec).name
                                               } else {
                                                   NULL_0 as *mut libc::c_char
                                               }, (*rec).priority,
                                               (*rec).weight, (*rec).srvport,
                                               (*rec).target) != 0 {
                            anscount += 1
                        }
                    } else if add_resource_record(header, limit,
                                                  &mut trunc as
                                                      *mut libc::c_int,
                                                  -axfroffset,
                                                  &mut ansp as
                                                      *mut *mut libc::c_uchar,
                                                  (*dnsmasq_daemon).auth_ttl,
                                                  NULL_0 as *mut libc::c_int,
                                                  T_MX as libc::c_ushort,
                                                  C_IN as libc::c_ushort,
                                                  b"sd\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  if !cut.is_null() {
                                                      (*rec).name
                                                  } else {
                                                      NULL_0 as
                                                          *mut libc::c_char
                                                  }, (*rec).weight,
                                                  (*rec).target) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                rec = (*rec).next
            }
            txt = (*dnsmasq_daemon).rr;
            while !txt.is_null() {
                if in_zone(zone, (*txt).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           NULL_0 as *mut libc::c_int,
                                           (*txt).class,
                                           C_IN as libc::c_ushort,
                                           b"t\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*txt).name
                                           } else {
                                               NULL_0 as *mut libc::c_char
                                           }, (*txt).len as libc::c_int,
                                           (*txt).txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                txt = (*txt).next
            }
            txt = (*dnsmasq_daemon).txt;
            while !txt.is_null() {
                if (*txt).class as libc::c_int == C_IN &&
                       in_zone(zone, (*txt).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           NULL_0 as *mut libc::c_int,
                                           T_TXT as libc::c_ushort,
                                           C_IN as libc::c_ushort,
                                           b"t\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*txt).name
                                           } else {
                                               NULL_0 as *mut libc::c_char
                                           }, (*txt).len as libc::c_int,
                                           (*txt).txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                txt = (*txt).next
            }
            na = (*dnsmasq_daemon).naptr;
            while !na.is_null() {
                if in_zone(zone, (*na).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           NULL_0 as *mut libc::c_int,
                                           T_NAPTR as libc::c_ushort,
                                           C_IN as libc::c_ushort,
                                           b"sszzzd\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*na).name
                                           } else {
                                               NULL_0 as *mut libc::c_char
                                           }, (*na).order, (*na).pref,
                                           (*na).flags, (*na).services,
                                           (*na).regexp, (*na).replace) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                na = (*na).next
            }
            intr = (*dnsmasq_daemon).int_names;
            while !intr.is_null() {
                if in_zone(zone, (*intr).name, &mut cut) != 0 {
                    let mut addrlist_2 = 0 as *mut addrlist;
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    addrlist_2 = (*intr).addr;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & ADDRLIST_IPV6 == 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone, F_IPV4 as libc::c_int,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   T_A as libc::c_ushort,
                                                   C_IN as libc::c_ushort,
                                                   b"4\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (if !cut.is_null() {
                                                        (*intr).name
                                                    } else {
                                                        NULL_0 as
                                                            *mut libc::c_char
                                                    }),
                                                   &mut (*addrlist_2).addr as
                                                       *mut all_addr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    addrlist_2 = (*intr).addr;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & ADDRLIST_IPV6 != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone, F_IPV6 as libc::c_int,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   T_AAAA as libc::c_ushort,
                                                   C_IN as libc::c_ushort,
                                                   b"6\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (if !cut.is_null() {
                                                        (*intr).name
                                                    } else {
                                                        NULL_0 as
                                                            *mut libc::c_char
                                                    }),
                                                   &mut (*addrlist_2).addr as
                                                       *mut all_addr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                intr = (*intr).next
            }
            a = (*dnsmasq_daemon).cnames;
            while !a.is_null() {
                if in_zone(zone, (*a).alias, &mut cut) != 0 {
                    strcpy(name, (*a).target);
                    if strchr(name, '.' as i32).is_null() {
                        strcat(name,
                               b".\x00" as *const u8 as *const libc::c_char);
                        strcat(name, (*zone).domain);
                    }
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           NULL_0 as *mut libc::c_int,
                                           T_CNAME as libc::c_ushort,
                                           C_IN as libc::c_ushort,
                                           b"d\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*a).alias
                                           } else {
                                               NULL_0 as *mut libc::c_char
                                           }, name) != 0 {
                        anscount += 1
                    }
                }
                a = (*a).next
            }
            cache_enumerate(1 as libc::c_int);
            loop  {
                crecp = cache_enumerate(0 as libc::c_int);
                if crecp.is_null() { break ; }
                if (*crecp).flags & (F_IPV4 | F_IPV6) != 0 &&
                       (*crecp).flags & (F_NEG | F_NXDOMAIN) == 0 &&
                       (*crecp).flags & F_FORWARD != 0 {
                    if (*crecp).flags & F_DHCP != 0 &&
                           (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (20 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 {
                        let mut cache_name = cache_get_name(crecp);
                        if strchr(cache_name, '.' as i32).is_null() &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((*crecp).flags &
                                                     (F_IPV6 | F_IPV4)) as
                                                    libc::c_int,
                                                &mut (*crecp).addr) != 0) &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   (if (*crecp).flags & F_IPV6
                                                           != 0 {
                                                        T_AAAA
                                                    } else { T_A }) as
                                                       libc::c_ushort,
                                                   C_IN as libc::c_ushort,
                                                   (if (*crecp).flags & F_IPV4
                                                           != 0 {
                                                        b"4\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    } else {
                                                        b"6\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    }) as *mut libc::c_char,
                                                   cache_name,
                                                   &mut (*crecp).addr as
                                                       *mut all_addr) != 0 {
                            anscount += 1
                        }
                    }
                    if (*crecp).flags & F_HOSTS != 0 ||
                           (*crecp).flags & F_DHCP != 0 &&
                               (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (20 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   != 0 {
                        strcpy(name, cache_get_name(crecp));
                        if in_zone(zone, name, &mut cut) != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((*crecp).flags &
                                                     (F_IPV6 | F_IPV4)) as
                                                    libc::c_int,
                                                &mut (*crecp).addr) != 0) {
                            if !cut.is_null() {
                                *cut = 0 as libc::c_int as libc::c_char
                            }
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   NULL_0 as *mut libc::c_int,
                                                   if (*crecp).flags & F_IPV6
                                                          != 0 {
                                                       T_AAAA
                                                   } else { T_A } as
                                                       libc::c_ushort,
                                                   C_IN as libc::c_ushort,
                                                   if (*crecp).flags & F_IPV4
                                                          != 0 {
                                                       b"4\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   } else {
                                                       b"6\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   } as *mut libc::c_char,
                                                   if !cut.is_null() {
                                                       name
                                                   } else {
                                                       NULL_0 as
                                                           *mut libc::c_char
                                                   },
                                                   &mut (*crecp).addr as
                                                       *mut all_addr) != 0 {
                                anscount += 1
                            }
                        }
                    }
                }
            }
            /* repeat SOA as last record */
            if add_resource_record(header, limit,
                                   &mut trunc as *mut libc::c_int, axfroffset,
                                   &mut ansp as *mut *mut libc::c_uchar,
                                   (*dnsmasq_daemon).auth_ttl,
                                   NULL_0 as *mut libc::c_int,
                                   T_SOA as libc::c_ushort,
                                   C_IN as libc::c_ushort,
                                   b"ddlllll\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                                   (*dnsmasq_daemon).authserver,
                                   (*dnsmasq_daemon).hostmaster,
                                   (*dnsmasq_daemon).soa_sn,
                                   (*dnsmasq_daemon).soa_refresh,
                                   (*dnsmasq_daemon).soa_retry,
                                   (*dnsmasq_daemon).soa_expiry,
                                   (*dnsmasq_daemon).auth_ttl) != 0 {
                anscount += 1
            }
        }
    }
    /* done all questions, set up header and return length of result */
  /* clear authoritative and truncated flags, set QR flag */
    (*header).hb3 =
        ((*header).hb3 as libc::c_int & !(HB3_AA | HB3_TC) | HB3_QR) as u8_0;
    if local_query != 0 {
        /* set RA flag */
        (*header).hb4 = ((*header).hb4 as libc::c_int | HB4_RA) as u8_0
    } else {
        /* clear RA flag */
        (*header).hb4 = ((*header).hb4 as libc::c_int & !HB4_RA) as u8_0
    }
    /* data is never DNSSEC signed. */
    (*header).hb4 = ((*header).hb4 as libc::c_int & !HB4_AD) as u8_0;
    /* authoritative */
    if auth != 0 {
        (*header).hb3 = ((*header).hb3 as libc::c_int | HB3_AA) as u8_0
    }
    /* truncation */
    if trunc != 0 {
        (*header).hb3 = ((*header).hb3 as libc::c_int | HB3_TC) as u8_0
    } /* no error */
    if (auth != 0 || local_query != 0) && nxdomain != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 3 as libc::c_int) as
                u8_0
    } else {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !HB4_RCODE | 0 as libc::c_int) as
                u8_0
    }
    (*header).ancount = __bswap_16(anscount as __uint16_t);
    (*header).nscount = __bswap_16(authcount as __uint16_t);
    (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        return add_pseudoheader(header,
                                ansp.wrapping_offset_from(header as
                                                              *mut libc::c_uchar)
                                    as libc::c_long as size_t,
                                limit as *mut libc::c_uchar,
                                (*dnsmasq_daemon).edns_pktsz,
                                0 as libc::c_int,
                                NULL_0 as *mut libc::c_uchar,
                                0 as libc::c_int as size_t, do_bit,
                                0 as libc::c_int)
    }
    return ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
