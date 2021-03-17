#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
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
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/metrics.h:17"]
pub mod metrics_h {
    #[c2rust::src_loc = "18:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "40:3"]
    pub const __METRIC_MAX: C2RustUnnamed_0 = 20;
    #[c2rust::src_loc = "38:3"]
    pub const METRIC_LEASES_PRUNED_6: C2RustUnnamed_0 = 19;
    #[c2rust::src_loc = "37:3"]
    pub const METRIC_LEASES_ALLOCATED_6: C2RustUnnamed_0 = 18;
    #[c2rust::src_loc = "36:3"]
    pub const METRIC_LEASES_PRUNED_4: C2RustUnnamed_0 = 17;
    #[c2rust::src_loc = "35:3"]
    pub const METRIC_LEASES_ALLOCATED_4: C2RustUnnamed_0 = 16;
    #[c2rust::src_loc = "34:3"]
    pub const METRIC_NOANSWER: C2RustUnnamed_0 = 15;
    #[c2rust::src_loc = "33:3"]
    pub const METRIC_DHCPREQUEST: C2RustUnnamed_0 = 14;
    #[c2rust::src_loc = "32:3"]
    pub const METRIC_DHCPRELEASE: C2RustUnnamed_0 = 13;
    #[c2rust::src_loc = "31:3"]
    pub const METRIC_DHCPOFFER: C2RustUnnamed_0 = 12;
    #[c2rust::src_loc = "30:3"]
    pub const METRIC_DHCPNAK: C2RustUnnamed_0 = 11;
    #[c2rust::src_loc = "29:3"]
    pub const METRIC_DHCPINFORM: C2RustUnnamed_0 = 10;
    #[c2rust::src_loc = "28:3"]
    pub const METRIC_DHCPDISCOVER: C2RustUnnamed_0 = 9;
    #[c2rust::src_loc = "27:3"]
    pub const METRIC_DHCPDECLINE: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "26:3"]
    pub const METRIC_DHCPACK: C2RustUnnamed_0 = 7;
    #[c2rust::src_loc = "25:3"]
    pub const METRIC_PXE: C2RustUnnamed_0 = 6;
    #[c2rust::src_loc = "24:3"]
    pub const METRIC_BOOTP: C2RustUnnamed_0 = 5;
    #[c2rust::src_loc = "23:3"]
    pub const METRIC_DNS_LOCAL_ANSWERED: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "22:3"]
    pub const METRIC_DNS_AUTH_ANSWERED: C2RustUnnamed_0 = 3;
    #[c2rust::src_loc = "21:3"]
    pub const METRIC_DNS_QUERIES_FORWARDED: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "20:3"]
    pub const METRIC_DNS_CACHE_LIVE_FREED: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "19:3"]
    pub const METRIC_DNS_CACHE_INSERTED: C2RustUnnamed_0 = 0;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dnsmasq.h:17"]
pub mod dnsmasq_h {
    #[c2rust::src_loc = "70:1"]
    pub type u32_0 = libc::c_uint;
    #[c2rust::src_loc = "71:1"]
    pub type u64_0 = libc::c_ulonglong;
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
    #[c2rust::src_loc = "287:9"]
    pub const MS_DHCP: libc::c_int = LOG_DAEMON;
    #[c2rust::src_loc = "716:9"]
    pub const LEASE_TA: libc::c_int = 64 as libc::c_int;
    #[c2rust::src_loc = "171:9"]
    pub const ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "715:9"]
    pub const LEASE_NA: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "212:9"]
    pub const EC_INIT_OFFSET: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "209:9"]
    pub const EC_FILE: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "711:9"]
    pub const LEASE_CHANGED: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "710:9"]
    pub const LEASE_NEW: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "713:9"]
    pub const LEASE_AUTH_NAME: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "712:9"]
    pub const LEASE_AUX_CHANGED: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "717:9"]
    pub const LEASE_HAVE_HWADDR: libc::c_int = 128 as libc::c_int;
    #[c2rust::src_loc = "211:9"]
    pub const EC_MISC: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "714:9"]
    pub const LEASE_USED: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "944:9"]
    pub const CONTEXT_PROXY: libc::c_uint =
        (1 as libc::c_uint) << 3 as libc::c_int;
    #[c2rust::src_loc = "941:9"]
    pub const CONTEXT_STATIC: libc::c_uint =
        (1 as libc::c_uint) << 0 as libc::c_int;
    #[c2rust::src_loc = "718:9"]
    pub const LEASE_EXP_CHANGED: libc::c_int = 256 as libc::c_int;
    #[c2rust::src_loc = "804:9"]
    pub const CONFIG_ADDR: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "803:9"]
    pub const CONFIG_NAME: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "704:9"]
    pub const ACTION_OLD: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "705:9"]
    pub const ACTION_ADD: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "703:9"]
    pub const ACTION_OLD_HOSTNAME: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "702:9"]
    pub const ACTION_DEL: libc::c_int = 1 as libc::c_int;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6,
                      in_addr_t};
    use super::time_t_h::time_t;
    use super::socket_h::sockaddr;
    use super::sys_types_h::{off_t, dev_t, ino_t, pid_t};
    use super::stddef_h::size_t;
    use super::struct_iovec_h::iovec;
    use super::FILE_h::FILE;
    use super::syslog_h::LOG_DAEMON;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1172:4"]
        pub static mut dnsmasq_daemon: *mut dnsmasq_daemon;
        #[no_mangle]
        #[c2rust::src_loc = "1192:1"]
        pub fn cache_add_dhcp_entry(host_name: *mut libc::c_char,
                                    prot: libc::c_int,
                                    host_address: *mut all_addr, ttd: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1194:1"]
        pub fn cache_unhash_dhcp();
        #[no_mangle]
        #[c2rust::src_loc = "1215:1"]
        pub fn get_domain(addr: in_addr) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1216:1"]
        pub fn get_domain6(addr: *mut in6_addr) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1288:1"]
        pub fn safe_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1291:1"]
        pub fn whine_malloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1294:1"]
        pub fn hostname_isequal(a: *const libc::c_char,
                                b: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1297:1"]
        pub fn netmask_length(mask: in_addr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1298:1"]
        pub fn is_same_net(a: in_addr, b: in_addr, mask: in_addr)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1299:1"]
        pub fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                            prefixlen: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1300:1"]
        pub fn addr6part(addr: *mut in6_addr) -> u64_0;
        #[no_mangle]
        #[c2rust::src_loc = "1305:1"]
        pub fn parse_hex(in_0: *mut libc::c_char, out: *mut libc::c_uchar,
                         maxlen: libc::c_int,
                         wildcard_mask: *mut libc::c_uint,
                         mac_type: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1320:1"]
        pub fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
                   exit_code: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn my_syslog(priority: libc::c_int, format: *const libc::c_char,
                         _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "1409:1"]
        pub fn host_from_dns(addr: in_addr) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1469:1"]
        pub fn send_alarm(event: time_t, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1626:1"]
        pub fn periodic_ra(now: time_t) -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "1633:1"]
        pub fn periodic_slaac(now: time_t, leases_0: *mut dhcp_lease)
         -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "1632:1"]
        pub fn slaac_add_addrs(lease: *mut dhcp_lease, now: time_t,
                               force: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1634:1"]
        pub fn slaac_ping_reply(sender: *mut in6_addr,
                                packet: *mut libc::c_uchar,
                                interface: *mut libc::c_char,
                                leases_0: *mut dhcp_lease);
        #[no_mangle]
        #[c2rust::src_loc = "1558:1"]
        pub fn make_duid(now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1590:1"]
        pub fn find_config(configs: *mut dhcp_config,
                           context: *mut dhcp_context,
                           clid: *mut libc::c_uchar, clid_len: libc::c_int,
                           hwaddr: *mut libc::c_uchar, hw_len: libc::c_int,
                           hw_type: libc::c_int, hostname: *mut libc::c_char,
                           filter: *mut dhcp_netid) -> *mut dhcp_config;
        #[no_mangle]
        #[c2rust::src_loc = "1519:1"]
        pub fn queue_script(action: libc::c_int, lease: *mut dhcp_lease,
                            hostname: *mut libc::c_char, now: time_t);
        #[no_mangle]
        #[c2rust::src_loc = "1489:1"]
        pub fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                               callback:
                                   Option<unsafe extern "C" fn()
                                              -> libc::c_int>) -> libc::c_int;
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
        #[no_mangle]
        #[c2rust::src_loc = "180:26"]
        pub fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
                       _: libc::c_int) -> libc::c_ulong;
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
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE)
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
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE)
     -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(__fp)
               } else {
                   let fresh1 = (*__fp)._IO_read_ptr;
                   (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
                   *(fresh1 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "72:1"]
    pub unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
        return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(stdin)
               } else {
                   let fresh2 = (*stdin)._IO_read_ptr;
                   (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
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
    use super::socket_h::socklen_t;
    extern "C" {
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
        #[c2rust::src_loc = "130:14"]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "137:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
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
    use super::types_h::__off64_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "954:1"]
        pub fn fsync(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1017:1"]
        pub fn ftruncate(__fd: libc::c_int, __length: __off64_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:17"]
pub mod stdio_h {
    #[c2rust::src_loc = "104:9"]
    pub const EOF: libc::c_int = -(1 as libc::c_int);
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
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
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
        #[c2rust::src_loc = "391:12"]
        pub fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
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
        #[no_mangle]
        #[c2rust::src_loc = "694:1"]
        pub fn rewind(__stream: *mut FILE);
        #[no_mangle]
        #[c2rust::src_loc = "761:1"]
        pub fn ferror(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn popen(__command: *const libc::c_char,
                     __modes: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "806:1"]
        pub fn pclose(__stream: *mut FILE) -> libc::c_int;
    }
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
    #[c2rust::src_loc = "96:9"]
    pub const LOG_DAEMON: libc::c_int =
        (3 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/config.h:17"]
pub mod config_h {
    #[c2rust::src_loc = "33:9"]
    pub const LEASE_RETRY: libc::c_int = 60 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/net/if_arp.h:17"]
pub mod if_arp_h {
    #[c2rust::src_loc = "73:9"]
    pub const ARPHRD_ETHER: libc::c_int = 1 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno-base.h:17"]
pub mod errno_base_h {
    #[c2rust::src_loc = "17:9"]
    pub const EACCES: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "6:9"]
    pub const ENOENT: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dhcp-protocol.h:17"]
pub mod dhcp_protocol_h {
    #[c2rust::src_loc = "92:9"]
    pub const DHCP_CHADDR_MAX: libc::c_int = 16 as libc::c_int;
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
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
pub use self::metrics_h::{C2RustUnnamed_0, __METRIC_MAX,
                          METRIC_LEASES_PRUNED_6, METRIC_LEASES_ALLOCATED_6,
                          METRIC_LEASES_PRUNED_4, METRIC_LEASES_ALLOCATED_4,
                          METRIC_NOANSWER, METRIC_DHCPREQUEST,
                          METRIC_DHCPRELEASE, METRIC_DHCPOFFER,
                          METRIC_DHCPNAK, METRIC_DHCPINFORM,
                          METRIC_DHCPDISCOVER, METRIC_DHCPDECLINE,
                          METRIC_DHCPACK, METRIC_PXE, METRIC_BOOTP,
                          METRIC_DNS_LOCAL_ANSWERED, METRIC_DNS_AUTH_ANSWERED,
                          METRIC_DNS_QUERIES_FORWARDED,
                          METRIC_DNS_CACHE_LIVE_FREED,
                          METRIC_DNS_CACHE_INSERTED};
pub use self::dnsmasq_h::{u32_0, u64_0, all_addr, C2RustUnnamed_1,
                          C2RustUnnamed_2, blockdata, C2RustUnnamed_3,
                          C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6,
                          crec, C2RustUnnamed_7, bigname, bogus_addr, doctor,
                          mx_srv_record, naptr, txt_record, ptr_record, cname,
                          addrlist, auth_zone, auth_name_list, host_record,
                          name_list, interface_name, mysockaddr, serverfd,
                          randfd, server, ipsets, irec, listener, iname,
                          mysubnet, resolvc, hostsfile, frec, frec_src,
                          dhcp_lease, slaac_address, dhcp_netid,
                          dhcp_netid_list, tag_if, delay_config,
                          hwaddr_config, dhcp_config, dhcp_opt,
                          C2RustUnnamed_8, dhcp_boot, dhcp_match_name,
                          pxe_service, dhcp_vendor, dhcp_pxe_vendor, dhcp_mac,
                          dhcp_bridge, cond_domain, ra_interface,
                          dhcp_context, shared_network, ping_result,
                          tftp_file, tftp_transfer, addr_list, tftp_prefix,
                          dhcp_relay, dnsmasq_daemon, MS_DHCP, LEASE_TA,
                          ADDRSTRLEN, LEASE_NA, EC_INIT_OFFSET, EC_FILE,
                          LEASE_CHANGED, LEASE_NEW, LEASE_AUTH_NAME,
                          LEASE_AUX_CHANGED, LEASE_HAVE_HWADDR, EC_MISC,
                          LEASE_USED, CONTEXT_PROXY, CONTEXT_STATIC,
                          LEASE_EXP_CHANGED, CONFIG_ADDR, CONFIG_NAME,
                          ACTION_OLD, ACTION_ADD, ACTION_OLD_HOSTNAME,
                          ACTION_DEL, cache_add_dhcp_entry, cache_unhash_dhcp,
                          get_domain, get_domain6, safe_malloc, whine_malloc,
                          hostname_isequal, netmask_length, is_same_net,
                          is_same_net6, addr6part, parse_hex, die, my_syslog,
                          host_from_dns, send_alarm, periodic_ra,
                          periodic_slaac, slaac_add_addrs, slaac_ping_reply,
                          make_duid, find_config, queue_script,
                          iface_enumerate};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER};
pub use self::stdarg_h::va_list;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll, strtoul, free};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
pub use self::bits_stdio_h::{vprintf, getchar, fgetc_unlocked, getc_unlocked,
                             getchar_unlocked, putchar, fputc_unlocked,
                             putc_unlocked, putchar_unlocked, getline,
                             feof_unlocked, ferror_unlocked};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity,
                                 __uint64_identity};
use self::inet_h::{inet_pton, inet_ntop};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::string_h::{memcpy, memset, memcmp, strcpy, strcat, strcmp, strlen,
                     strerror};
use self::unistd_h::{fsync, ftruncate};
pub use self::stdio_h::{EOF, stdin, stdout, fflush, fopen, sprintf, vfprintf,
                        fscanf, getc, __uflow, putc, __overflow, __getdelim,
                        rewind, ferror, fileno, popen, pclose};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
pub use self::ctype_h::{tolower, toupper, __ctype_tolower_loc,
                        __ctype_toupper_loc};
use self::time_h::difftime;
pub use self::errno_h::{errno, __errno_location};
pub use self::syslog_h::{LOG_ERR, LOG_DAEMON, LOG_WARNING};
pub use self::config_h::LEASE_RETRY;
pub use self::if_arp_h::ARPHRD_ETHER;
pub use self::errno_base_h::{EACCES, ENOENT};
pub use self::dhcp_protocol_h::DHCP_CHADDR_MAX;
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
#[c2rust::src_loc = "21:27"]
static mut leases: *mut dhcp_lease = NULL_0 as *mut dhcp_lease;
#[c2rust::src_loc = "21:43"]
static mut old_leases: *mut dhcp_lease = NULL_0 as *mut dhcp_lease;
#[c2rust::src_loc = "22:12"]
static mut dns_dirty: libc::c_int = 0;
#[c2rust::src_loc = "22:23"]
static mut file_dirty: libc::c_int = 0;
#[c2rust::src_loc = "22:35"]
static mut leases_left: libc::c_int = 0;
#[c2rust::src_loc = "24:1"]
unsafe extern "C" fn read_leases(mut now: time_t, mut leasestream: *mut FILE)
 -> libc::c_int {
    let mut ei: libc::c_ulong = 0;
    let mut addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut lease = 0 as *mut dhcp_lease;
    let mut clid_len: libc::c_int = 0;
    let mut hw_len: libc::c_int = 0;
    let mut hw_type: libc::c_int = 0;
    let mut items: libc::c_int = 0;
    let mut domain = NULL_0 as *mut libc::c_char;
    *(*dnsmasq_daemon).dhcp_buff2 = '\u{0}' as i32 as libc::c_char;
    *(*dnsmasq_daemon).dhcp_buff3 = *(*dnsmasq_daemon).dhcp_buff2;
    loop 
         /* client-id max length is 255 which is 255*2 digits + 254 colons
     borrow DNS packet buffer which is always larger than 1000 bytes

     Check various buffers are big enough for the code below */
         {
        items =
            fscanf(leasestream,
                   b"%255s %255s\x00" as *const u8 as *const libc::c_char,
                   (*dnsmasq_daemon).dhcp_buff3,
                   (*dnsmasq_daemon).dhcp_buff2);
        if !(items == 2 as libc::c_int) { break ; }
        *(*dnsmasq_daemon).packet = '\u{0}' as i32 as libc::c_char;
        *(*dnsmasq_daemon).dhcp_buff = *(*dnsmasq_daemon).packet;
        *(*dnsmasq_daemon).namebuff = *(*dnsmasq_daemon).dhcp_buff;
        clid_len = 0 as libc::c_int;
        hw_type = clid_len;
        hw_len = hw_type;
        if strcmp((*dnsmasq_daemon).dhcp_buff3,
                  b"duid\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*dnsmasq_daemon).duid_len =
                parse_hex((*dnsmasq_daemon).dhcp_buff2,
                          (*dnsmasq_daemon).dhcp_buff2 as *mut libc::c_uchar,
                          130 as libc::c_int, NULL_0 as *mut libc::c_uint,
                          NULL_0 as *mut libc::c_int);
            if (*dnsmasq_daemon).duid_len < 0 as libc::c_int {
                return 0 as libc::c_int
            }
            (*dnsmasq_daemon).duid =
                safe_malloc((*dnsmasq_daemon).duid_len as size_t) as
                    *mut libc::c_uchar;
            memcpy((*dnsmasq_daemon).duid as *mut libc::c_void,
                   (*dnsmasq_daemon).dhcp_buff2 as *const libc::c_void,
                   (*dnsmasq_daemon).duid_len as libc::c_ulong);
        } else if fscanf(leasestream,
                         b" %64s %255s %764s\x00" as *const u8 as
                             *const libc::c_char, (*dnsmasq_daemon).namebuff,
                         (*dnsmasq_daemon).dhcp_buff,
                         (*dnsmasq_daemon).packet) != 3 as libc::c_int {
            my_syslog(MS_DHCP | LOG_WARNING,
                      b"ignoring invalid line in lease database: %s %s %s %s ...\x00"
                          as *const u8 as *const libc::c_char,
                      (*dnsmasq_daemon).dhcp_buff3,
                      (*dnsmasq_daemon).dhcp_buff2,
                      (*dnsmasq_daemon).namebuff,
                      (*dnsmasq_daemon).dhcp_buff);
        } else {
            if inet_pton(AF_INET, (*dnsmasq_daemon).namebuff,
                         &mut addr.addr4 as *mut in_addr as *mut libc::c_void)
                   != 0 {
                lease = lease4_allocate(addr.addr4);
                if !lease.is_null() { domain = get_domain((*lease).addr) }
                hw_len =
                    parse_hex((*dnsmasq_daemon).dhcp_buff2,
                              (*dnsmasq_daemon).dhcp_buff2 as
                                  *mut libc::c_uchar, DHCP_CHADDR_MAX,
                              NULL_0 as *mut libc::c_uint, &mut hw_type);
                /* For backwards compatibility, no explicit MAC address type means ether. */
                if hw_type == 0 as libc::c_int && hw_len != 0 as libc::c_int {
                    hw_type = ARPHRD_ETHER
                }
            } else if inet_pton(AF_INET6, (*dnsmasq_daemon).namebuff,
                                &mut addr.addr6 as *mut in6_addr as
                                    *mut libc::c_void) != 0 {
                let mut s = (*dnsmasq_daemon).dhcp_buff2;
                let mut lease_type = LEASE_NA;
                if *s.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'T' as i32 {
                    lease_type = LEASE_TA;
                    s = s.offset(1)
                }
                lease = lease6_allocate(&mut addr.addr6, lease_type);
                if !lease.is_null() {
                    lease_set_iaid(lease,
                                   strtoul(s,
                                           NULL_0 as *mut *mut libc::c_char,
                                           10 as libc::c_int) as
                                       libc::c_uint);
                    domain = get_domain6(&mut (*lease).addr6)
                }
            } else {
                my_syslog(MS_DHCP | LOG_WARNING,
                          b"ignoring invalid line in lease database, bad address: %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).namebuff);
                continue ;
            }
            if lease.is_null() {
                die(b"too many stored leases\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    NULL_0 as *mut libc::c_char, EC_MISC);
            }
            if strcmp((*dnsmasq_daemon).packet,
                      b"*\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                clid_len =
                    parse_hex((*dnsmasq_daemon).packet,
                              (*dnsmasq_daemon).packet as *mut libc::c_uchar,
                              255 as libc::c_int, NULL_0 as *mut libc::c_uint,
                              NULL_0 as *mut libc::c_int)
            }
            lease_set_hwaddr(lease,
                             (*dnsmasq_daemon).dhcp_buff2 as
                                 *mut libc::c_uchar,
                             (*dnsmasq_daemon).packet as *mut libc::c_uchar,
                             hw_len, hw_type, clid_len, now,
                             0 as libc::c_int);
            if strcmp((*dnsmasq_daemon).dhcp_buff,
                      b"*\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                lease_set_hostname(lease, (*dnsmasq_daemon).dhcp_buff,
                                   0 as libc::c_int, domain,
                                   NULL_0 as *mut libc::c_char);
            }
            ei = atol((*dnsmasq_daemon).dhcp_buff3) as libc::c_ulong;
            /* strictly time_t is opaque, but this hack should work on all sane systems,
	   even when sizeof(time_t) == 8 */
            (*lease).expires = ei as time_t;
            /* set these correctly: the "old" events are generated later from
	   the startup synthesised SIGHUP. */
            (*lease).flags &= !(LEASE_NEW | LEASE_CHANGED);
            *(*dnsmasq_daemon).dhcp_buff2 = '\u{0}' as i32 as libc::c_char;
            *(*dnsmasq_daemon).dhcp_buff3 = *(*dnsmasq_daemon).dhcp_buff2
        }
    }
    return (items == 0 as libc::c_int || items == EOF) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn lease_init(mut now: time_t) {
    let mut leasestream = 0 as *mut FILE;
    leases_left = (*dnsmasq_daemon).dhcp_max;
    if (*dnsmasq_daemon).options[(22 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (22 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        /* run "<lease_change_script> init" once to get the
	 initial state of the database. If leasefile-ro is
	 set without a script, we just do without any
	 lease database. */
        if !(*dnsmasq_daemon).lease_change_command.is_null() {
            strcpy((*dnsmasq_daemon).dhcp_buff,
                   (*dnsmasq_daemon).lease_change_command);
            strcat((*dnsmasq_daemon).dhcp_buff,
                   b" init\x00" as *const u8 as *const libc::c_char);
            leasestream =
                popen((*dnsmasq_daemon).dhcp_buff,
                      b"r\x00" as *const u8 as *const libc::c_char)
        } else {
            dns_dirty = 0 as libc::c_int;
            file_dirty = dns_dirty;
            return
        }
    } else {
        /* NOTE: need a+ mode to create file if it doesn't exist */
        (*dnsmasq_daemon).lease_stream =
            fopen((*dnsmasq_daemon).lease_file,
                  b"a+\x00" as *const u8 as *const libc::c_char);
        leasestream = (*dnsmasq_daemon).lease_stream;
        if leasestream.is_null() {
            die(b"cannot open or create lease file %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_file, EC_FILE);
        }
        /* a+ mode leaves pointer at end. */
        rewind(leasestream);
    }
    if !leasestream.is_null() {
        if read_leases(now, leasestream) == 0 {
            my_syslog(MS_DHCP | LOG_ERR,
                      b"failed to parse lease database cleanly\x00" as
                          *const u8 as *const libc::c_char);
        }
        if ferror(leasestream) != 0 {
            die(b"failed to read lease file %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_file, EC_FILE);
        }
    }
    if (*dnsmasq_daemon).lease_stream.is_null() {
        let mut rc = 0 as libc::c_int;
        /* shell returns 127 for "command not found", 126 for bad permissions. */
        if leasestream.is_null() ||
               { rc = pclose(leasestream); (rc) == -(1 as libc::c_int) } ||
               (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   127 as libc::c_int ||
               (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   126 as libc::c_int {
            if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   127 as libc::c_int {
                errno = ENOENT
            } else if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                          126 as libc::c_int {
                errno = EACCES
            }
            die(b"cannot run lease-init script %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_change_command, EC_FILE);
        }
        if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int !=
               0 as libc::c_int {
            sprintf((*dnsmasq_daemon).dhcp_buff,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int);
            die(b"lease-init script returned exit code %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).dhcp_buff,
                ((rc & 0xff00 as libc::c_int) >> 8 as libc::c_int) +
                    EC_INIT_OFFSET);
        }
    }
    /* Some leases may have expired */
    file_dirty = 0 as libc::c_int;
    lease_prune(NULL_0 as *mut dhcp_lease, now);
    dns_dirty = 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "221:1"]
pub unsafe extern "C" fn lease_update_from_configs() {
    /* changes to the config may change current leases. */
    let mut lease = 0 as *mut dhcp_lease;
    let mut config = 0 as *mut dhcp_config;
    let mut name = 0 as *mut libc::c_char;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (LEASE_TA | LEASE_NA) != 0) {
            config =
                find_config((*dnsmasq_daemon).dhcp_conf,
                            NULL_0 as *mut dhcp_context, (*lease).clid,
                            (*lease).clid_len, (*lease).hwaddr.as_mut_ptr(),
                            (*lease).hwaddr_len, (*lease).hwaddr_type,
                            NULL_0 as *mut libc::c_char,
                            NULL_0 as *mut dhcp_netid);
            if !config.is_null() &&
                   (*config).flags & CONFIG_NAME as libc::c_uint != 0 &&
                   ((*config).flags & CONFIG_ADDR as libc::c_uint == 0 ||
                        (*config).addr.s_addr == (*lease).addr.s_addr) {
                lease_set_hostname(lease, (*config).hostname,
                                   1 as libc::c_int,
                                   get_domain((*lease).addr),
                                   NULL_0 as *mut libc::c_char);
            } else {
                name = host_from_dns((*lease).addr);
                if !name.is_null() {
                    lease_set_hostname(lease, name, 1 as libc::c_int,
                                       get_domain((*lease).addr),
                                       NULL_0 as *mut libc::c_char);
                }
            }
        }
        lease = (*lease).next
    };
    /* updates auth flag only */
}
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn ourprintf(mut errp: *mut libc::c_int,
                               mut format: *mut libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if *errp == 0 &&
           vfprintf((*dnsmasq_daemon).lease_stream, format, ap.as_va_list()) <
               0 as libc::c_int {
        *errp = errno
    };
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn lease_update_file(mut now: time_t) {
    let mut lease = 0 as *mut dhcp_lease;
    let mut next_event: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut err = 0 as libc::c_int;
    if file_dirty != 0 as libc::c_int &&
           !(*dnsmasq_daemon).lease_stream.is_null() {
        errno = 0 as libc::c_int;
        rewind((*dnsmasq_daemon).lease_stream);
        if errno != 0 as libc::c_int ||
               ftruncate(fileno((*dnsmasq_daemon).lease_stream),
                         0 as libc::c_int as __off64_t) != 0 as libc::c_int {
            err = errno
        }
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (LEASE_TA | LEASE_NA) != 0) {
                ourprintf(&mut err as *mut libc::c_int,
                          b"%lu \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*lease).expires as libc::c_ulong);
                if (*lease).hwaddr_type != ARPHRD_ETHER ||
                       (*lease).hwaddr_len == 0 as libc::c_int {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%.2x-\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char, (*lease).hwaddr_type);
                }
                i = 0 as libc::c_int;
                while i < (*lease).hwaddr_len {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%.2x\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              (*lease).hwaddr[i as usize] as libc::c_int);
                    if i != (*lease).hwaddr_len - 1 as libc::c_int {
                        ourprintf(&mut err as *mut libc::c_int,
                                  b":\x00" as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                    }
                    i += 1
                }
                inet_ntop(AF_INET,
                          &mut (*lease).addr as *mut in_addr as
                              *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                          ADDRSTRLEN as socklen_t);
                ourprintf(&mut err as *mut libc::c_int,
                          b" %s \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, (*dnsmasq_daemon).addrbuff);
                ourprintf(&mut err as *mut libc::c_int,
                          b"%s \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          if !(*lease).hostname.is_null() {
                              (*lease).hostname as *const libc::c_char
                          } else {
                              b"*\x00" as *const u8 as *const libc::c_char
                          });
                if !(*lease).clid.is_null() &&
                       (*lease).clid_len != 0 as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < (*lease).clid_len - 1 as libc::c_int {
                        ourprintf(&mut err as *mut libc::c_int,
                                  b"%.2x:\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  *(*lease).clid.offset(i as isize) as
                                      libc::c_int);
                        i += 1
                    }
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%.2x\n\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              *(*lease).clid.offset(i as isize) as
                                  libc::c_int);
                } else {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"*\n\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char);
                }
            }
            lease = (*lease).next
        }
        if !(*dnsmasq_daemon).duid.is_null() {
            ourprintf(&mut err as *mut libc::c_int,
                      b"duid \x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
            i = 0 as libc::c_int;
            while i < (*dnsmasq_daemon).duid_len - 1 as libc::c_int {
                ourprintf(&mut err as *mut libc::c_int,
                          b"%.2x:\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          *(*dnsmasq_daemon).duid.offset(i as isize) as
                              libc::c_int);
                i += 1
            }
            ourprintf(&mut err as *mut libc::c_int,
                      b"%.2x\n\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      *(*dnsmasq_daemon).duid.offset(i as isize) as
                          libc::c_int);
            lease = leases;
            while !lease.is_null() {
                if !((*lease).flags & (LEASE_TA | LEASE_NA) == 0) {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%lu \x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              (*lease).expires as libc::c_ulong);
                    inet_ntop(AF_INET6,
                              &mut (*lease).addr6 as *mut in6_addr as
                                  *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              ADDRSTRLEN as socklen_t);
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%s%u %s \x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              if (*lease).flags & LEASE_TA != 0 {
                                  b"T\x00" as *const u8 as *const libc::c_char
                              } else {
                                  b"\x00" as *const u8 as *const libc::c_char
                              }, (*lease).iaid, (*dnsmasq_daemon).addrbuff);
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%s \x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              if !(*lease).hostname.is_null() {
                                  (*lease).hostname as *const libc::c_char
                              } else {
                                  b"*\x00" as *const u8 as *const libc::c_char
                              });
                    if !(*lease).clid.is_null() &&
                           (*lease).clid_len != 0 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < (*lease).clid_len - 1 as libc::c_int {
                            ourprintf(&mut err as *mut libc::c_int,
                                      b"%.2x:\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      *(*lease).clid.offset(i as isize) as
                                          libc::c_int);
                            i += 1
                        }
                        ourprintf(&mut err as *mut libc::c_int,
                                  b"%.2x\n\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  *(*lease).clid.offset(i as isize) as
                                      libc::c_int);
                    } else {
                        ourprintf(&mut err as *mut libc::c_int,
                                  b"*\n\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                    }
                }
                lease = (*lease).next
            }
        }
        if fflush((*dnsmasq_daemon).lease_stream) != 0 as libc::c_int ||
               fsync(fileno((*dnsmasq_daemon).lease_stream)) <
                   0 as libc::c_int {
            err = errno
        }
        if err == 0 { file_dirty = 0 as libc::c_int }
    }
    /* Set alarm for when the first lease expires. */
    next_event = 0 as libc::c_int as time_t;
    /* do timed RAs and determine when the next is, also pings to potential SLAAC addresses */
    if (*dnsmasq_daemon).doing_ra != 0 {
        let mut event: time_t = 0;
        event = periodic_slaac(now, leases);
        if event != 0 as libc::c_int as libc::c_long {
            if next_event == 0 as libc::c_int as libc::c_long ||
                   difftime(next_event, event) > 0.0f64 {
                next_event = event
            }
        }
        event = periodic_ra(now);
        if event != 0 as libc::c_int as libc::c_long {
            if next_event == 0 as libc::c_int as libc::c_long ||
                   difftime(next_event, event) > 0.0f64 {
                next_event = event
            }
        }
    }
    lease = leases;
    while !lease.is_null() {
        if (*lease).expires != 0 as libc::c_int as libc::c_long &&
               (next_event == 0 as libc::c_int as libc::c_long ||
                    difftime(next_event, (*lease).expires) > 0.0f64) {
            next_event = (*lease).expires
        }
        lease = (*lease).next
    }
    if err != 0 {
        if next_event == 0 as libc::c_int as libc::c_long ||
               difftime(next_event, LEASE_RETRY as libc::c_long + now) >
                   0.0f64 {
            next_event = LEASE_RETRY as libc::c_long + now
        }
        my_syslog(MS_DHCP | LOG_ERR,
                  b"failed to write %s: %s (retry in %us)\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).lease_file,
                  strerror(err), difftime(next_event, now) as libc::c_uint);
    }
    send_alarm(next_event, now);
}
#[c2rust::src_loc = "390:1"]
unsafe extern "C" fn find_interface_v4(mut local: in_addr,
                                       mut if_index: libc::c_int,
                                       mut label: *mut libc::c_char,
                                       mut netmask: in_addr,
                                       mut broadcast: in_addr,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut lease = 0 as *mut dhcp_lease;
    let mut prefix = netmask_length(netmask);
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (LEASE_TA | LEASE_NA) == 0 &&
               is_same_net(local, (*lease).addr, netmask) != 0 &&
               prefix > (*lease).new_prefixlen {
            (*lease).new_interface = if_index;
            (*lease).new_prefixlen = prefix
        }
        lease = (*lease).next
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "413:1"]
unsafe extern "C" fn find_interface_v6(mut local: *mut in6_addr,
                                       mut prefix: libc::c_int,
                                       mut scope: libc::c_int,
                                       mut if_index: libc::c_int,
                                       mut flags: libc::c_int,
                                       mut preferred: libc::c_int,
                                       mut valid: libc::c_int,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (LEASE_TA | LEASE_NA) != 0 {
            if is_same_net6(local, &mut (*lease).addr6, prefix) != 0 &&
                   prefix > (*lease).new_prefixlen {
                /* save prefix length for comparison, as we might get shorter matching
         * prefix in upcoming netlink GETADDR responses
         * */
                (*lease).new_interface = if_index;
                (*lease).new_prefixlen = prefix
            }
        }
        lease = (*lease).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "438:1"]
pub unsafe extern "C" fn lease_ping_reply(mut sender: *mut in6_addr,
                                          mut packet: *mut libc::c_uchar,
                                          mut interface: *mut libc::c_char) {
    /* We may be doing RA but not DHCPv4, in which case the lease
     database may not exist and we have nothing to do anyway */
    if !(*dnsmasq_daemon).dhcp.is_null() {
        slaac_ping_reply(sender, packet, interface, leases);
    };
}
#[no_mangle]
#[c2rust::src_loc = "446:1"]
pub unsafe extern "C" fn lease_update_slaac(mut now: time_t) {
    /* Called when we construct a new RA-names context, to add putative
     new SLAAC addresses to existing leases. */
    let mut lease = 0 as *mut dhcp_lease;
    if !(*dnsmasq_daemon).dhcp.is_null() {
        lease = leases;
        while !lease.is_null() {
            slaac_add_addrs(lease, now, 0 as libc::c_int);
            lease = (*lease).next
        }
    };
}
/* Find interfaces associated with leases at start-up. This gets updated as
   we do DHCP transactions, but information about directly-connected subnets
   is useful from scrips and necessary for determining SLAAC addresses from
   start-time. */
#[no_mangle]
#[c2rust::src_loc = "465:1"]
pub unsafe extern "C" fn lease_find_interfaces(mut now: time_t) {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).new_interface = 0 as libc::c_int;
        (*lease).new_prefixlen = (*lease).new_interface;
        lease = (*lease).next
    }
    iface_enumerate(AF_INET, &mut now as *mut time_t as *mut libc::c_void,
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
                                                           libc::c_int>>(Some(find_interface_v4
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
                                                                                          libc::c_int)));
    iface_enumerate(AF_INET6, &mut now as *mut time_t as *mut libc::c_void,
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
                                                           libc::c_int>>(Some(find_interface_v6
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
    lease = leases;
    while !lease.is_null() {
        if (*lease).new_interface != 0 as libc::c_int {
            lease_set_interface(lease, (*lease).new_interface, now);
        }
        lease = (*lease).next
    };
}
#[no_mangle]
#[c2rust::src_loc = "483:1"]
pub unsafe extern "C" fn lease_make_duid(mut now: time_t) {
    /* If we're not doing DHCPv6, and there are not v6 leases, don't add the DUID to the database */
    if (*dnsmasq_daemon).duid.is_null() && (*dnsmasq_daemon).doing_dhcp6 != 0
       {
        file_dirty = 1 as libc::c_int;
        make_duid(now);
    };
}
#[no_mangle]
#[c2rust::src_loc = "497:1"]
pub unsafe extern "C" fn lease_update_dns(mut force: libc::c_int) {
    let mut lease = 0 as *mut dhcp_lease;
    if (*dnsmasq_daemon).port != 0 as libc::c_int &&
           (dns_dirty != 0 || force != 0) {
        /* force transfer to authoritative secondaries */
        (*dnsmasq_daemon).soa_sn =
            (*dnsmasq_daemon).soa_sn.wrapping_add(1); /* unlink */
        cache_unhash_dhcp();
        lease = leases;
        while !lease.is_null() {
            let mut prot = AF_INET;
            if (*lease).flags & (LEASE_TA | LEASE_NA) != 0 {
                prot = AF_INET6
            } else if !(*lease).hostname.is_null() || !(*lease).fqdn.is_null()
             {
                let mut slaac = 0 as *mut slaac_address;
                slaac = (*lease).slaac_address;
                while !slaac.is_null() {
                    if (*slaac).backoff == 0 as libc::c_int {
                        if !(*lease).fqdn.is_null() {
                            cache_add_dhcp_entry((*lease).fqdn, AF_INET6,
                                                 &mut (*slaac).addr as
                                                     *mut in6_addr as
                                                     *mut all_addr,
                                                 (*lease).expires);
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
                               == 0 && !(*lease).hostname.is_null() {
                            cache_add_dhcp_entry((*lease).hostname, AF_INET6,
                                                 &mut (*slaac).addr as
                                                     *mut in6_addr as
                                                     *mut all_addr,
                                                 (*lease).expires);
                        }
                    }
                    slaac = (*slaac).next
                }
            }
            if !(*lease).fqdn.is_null() {
                cache_add_dhcp_entry((*lease).fqdn, prot,
                                     if prot == AF_INET {
                                         &mut (*lease).addr as *mut in_addr as
                                             *mut all_addr
                                     } else {
                                         &mut (*lease).addr6 as *mut in6_addr
                                             as *mut all_addr
                                     }, (*lease).expires);
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
                   == 0 && !(*lease).hostname.is_null() {
                cache_add_dhcp_entry((*lease).hostname, prot,
                                     if prot == AF_INET {
                                         &mut (*lease).addr as *mut in_addr as
                                             *mut all_addr
                                     } else {
                                         &mut (*lease).addr6 as *mut in6_addr
                                             as *mut all_addr
                                     }, (*lease).expires);
            }
            lease = (*lease).next
        }
        dns_dirty = 0 as libc::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "554:1"]
pub unsafe extern "C" fn lease_prune(mut target: *mut dhcp_lease,
                                     mut now: time_t) {
    let mut lease = 0 as *mut dhcp_lease;
    let mut tmp = 0 as *mut dhcp_lease;
    let mut up = 0 as *mut *mut dhcp_lease;
    lease = leases;
    up = &mut leases;
    while !lease.is_null() {
        tmp = (*lease).next;
        if (*lease).expires != 0 as libc::c_int as libc::c_long &&
               difftime(now, (*lease).expires) >=
                   0 as libc::c_int as libc::c_double || lease == target {
            file_dirty = 1 as libc::c_int;
            if !(*lease).hostname.is_null() { dns_dirty = 1 as libc::c_int }
            (*dnsmasq_daemon).metrics[if (*lease).addr.s_addr != 0 {
                                          METRIC_LEASES_PRUNED_4 as
                                              libc::c_int
                                      } else {
                                          METRIC_LEASES_PRUNED_6 as
                                              libc::c_int
                                      } as usize] =
                (*dnsmasq_daemon).metrics[if (*lease).addr.s_addr != 0 {
                                              METRIC_LEASES_PRUNED_4 as
                                                  libc::c_int
                                          } else {
                                              METRIC_LEASES_PRUNED_6 as
                                                  libc::c_int
                                          } as usize].wrapping_add(1);
            *up = (*lease).next;
            /* Put on old_leases list 'till we
	     can run the script */
            (*lease).next = old_leases;
            old_leases = lease;
            leases_left += 1
        } else { up = &mut (*lease).next }
        lease = tmp
    };
}
#[no_mangle]
#[c2rust::src_loc = "584:1"]
pub unsafe extern "C" fn lease_find_by_client(mut hwaddr: *mut libc::c_uchar,
                                              mut hw_len: libc::c_int,
                                              mut hw_type: libc::c_int,
                                              mut clid: *mut libc::c_uchar,
                                              mut clid_len: libc::c_int)
 -> *mut dhcp_lease {
    let mut lease = 0 as *mut dhcp_lease;
    if !clid.is_null() {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (LEASE_TA | LEASE_NA) != 0) {
                if !(*lease).clid.is_null() && clid_len == (*lease).clid_len
                       &&
                       memcmp(clid as *const libc::c_void,
                              (*lease).clid as *const libc::c_void,
                              clid_len as libc::c_ulong) == 0 as libc::c_int {
                    return lease
                }
            }
            lease = (*lease).next
        }
    }
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (LEASE_TA | LEASE_NA) != 0) {
            if ((*lease).clid.is_null() || clid.is_null()) &&
                   hw_len != 0 as libc::c_int && (*lease).hwaddr_len == hw_len
                   && (*lease).hwaddr_type == hw_type &&
                   memcmp(hwaddr as *const libc::c_void,
                          (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                          hw_len as libc::c_ulong) == 0 as libc::c_int {
                return lease
            }
        }
        lease = (*lease).next
    }
    return NULL_0 as *mut dhcp_lease;
}
#[no_mangle]
#[c2rust::src_loc = "618:1"]
pub unsafe extern "C" fn lease_find_by_addr(mut addr: in_addr)
 -> *mut dhcp_lease {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (LEASE_TA | LEASE_NA) != 0) {
            if (*lease).addr.s_addr == addr.s_addr { return lease }
        }
        lease = (*lease).next
    }
    return NULL_0 as *mut dhcp_lease;
}
/* find address for {CLID, IAID, address} */
#[no_mangle]
#[c2rust::src_loc = "637:1"]
pub unsafe extern "C" fn lease6_find(mut clid: *mut libc::c_uchar,
                                     mut clid_len: libc::c_int,
                                     mut lease_type: libc::c_int,
                                     mut iaid: libc::c_uint,
                                     mut addr: *mut in6_addr)
 -> *mut dhcp_lease {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & lease_type == 0 || (*lease).iaid != iaid) {
            if !(({
                      let mut __a =
                          &mut (*lease).addr6 as *mut in6_addr as
                              *const in6_addr;
                      let mut __b = addr as *const in6_addr;
                      ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                           ==
                           (*__b).__in6_u.__u6_addr32[0 as libc::c_int as
                                                          usize] &&
                           (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                          usize] ==
                               (*__b).__in6_u.__u6_addr32[1 as libc::c_int as
                                                              usize] &&
                           (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                          usize] ==
                               (*__b).__in6_u.__u6_addr32[2 as libc::c_int as
                                                              usize] &&
                           (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                          usize] ==
                               (*__b).__in6_u.__u6_addr32[3 as libc::c_int as
                                                              usize]) as
                          libc::c_int
                  }) == 0) {
                if !(clid_len != (*lease).clid_len ||
                         memcmp(clid as *const libc::c_void,
                                (*lease).clid as *const libc::c_void,
                                clid_len as libc::c_ulong) !=
                             0 as libc::c_int) {
                    return lease
                }
            }
        }
        lease = (*lease).next
    }
    return NULL_0 as *mut dhcp_lease;
}
/* reset "USED flags */
#[no_mangle]
#[c2rust::src_loc = "662:1"]
pub unsafe extern "C" fn lease6_reset() {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).flags &= !LEASE_USED;
        lease = (*lease).next
    };
}
/* enumerate all leases belonging to {CLID, IAID} */
#[no_mangle]
#[c2rust::src_loc = "671:1"]
pub unsafe extern "C" fn lease6_find_by_client(mut first: *mut dhcp_lease,
                                               mut lease_type: libc::c_int,
                                               mut clid: *mut libc::c_uchar,
                                               mut clid_len: libc::c_int,
                                               mut iaid: libc::c_uint)
 -> *mut dhcp_lease {
    let mut lease = 0 as *mut dhcp_lease;
    if first.is_null() { first = leases } else { first = (*first).next }
    lease = first;
    while !lease.is_null() {
        if !((*lease).flags & LEASE_USED != 0) {
            if !((*lease).flags & lease_type == 0 || (*lease).iaid != iaid) {
                if !(clid_len != (*lease).clid_len ||
                         memcmp(clid as *const libc::c_void,
                                (*lease).clid as *const libc::c_void,
                                clid_len as libc::c_ulong) !=
                             0 as libc::c_int) {
                    return lease
                }
            }
        }
        lease = (*lease).next
    }
    return NULL_0 as *mut dhcp_lease;
}
#[no_mangle]
#[c2rust::src_loc = "700:1"]
pub unsafe extern "C" fn lease6_find_by_addr(mut net: *mut in6_addr,
                                             mut prefix: libc::c_int,
                                             mut addr: u64_0)
 -> *mut dhcp_lease {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (LEASE_TA | LEASE_NA) == 0) {
            if is_same_net6(&mut (*lease).addr6, net, prefix) != 0 &&
                   (prefix == 128 as libc::c_int ||
                        addr6part(&mut (*lease).addr6) == addr) {
                return lease
            }
        }
        lease = (*lease).next
    }
    return NULL_0 as *mut dhcp_lease;
}
/* Find largest assigned address in context */
#[no_mangle]
#[c2rust::src_loc = "718:1"]
pub unsafe extern "C" fn lease_find_max_addr6(mut context: *mut dhcp_context)
 -> u64_0 {
    let mut lease = 0 as *mut dhcp_lease;
    let mut addr = addr6part(&mut (*context).start6);
    if (*context).flags as libc::c_uint & (CONTEXT_STATIC | CONTEXT_PROXY) ==
           0 {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (LEASE_TA | LEASE_NA) == 0) {
                if is_same_net6(&mut (*lease).addr6, &mut (*context).start6,
                                64 as libc::c_int) != 0 &&
                       addr6part(&mut (*lease).addr6) >
                           addr6part(&mut (*context).start6) &&
                       addr6part(&mut (*lease).addr6) <=
                           addr6part(&mut (*context).end6) &&
                       addr6part(&mut (*lease).addr6) > addr {
                    addr = addr6part(&mut (*lease).addr6)
                }
            }
            lease = (*lease).next
        }
    }
    return addr;
}
/* Find largest assigned address in context */
#[no_mangle]
#[c2rust::src_loc = "742:1"]
pub unsafe extern "C" fn lease_find_max_addr(mut context: *mut dhcp_context)
 -> in_addr {
    let mut lease = 0 as *mut dhcp_lease; /* illegal value */
    let mut addr = (*context).start;
    if (*context).flags as libc::c_uint & (CONTEXT_STATIC | CONTEXT_PROXY) ==
           0 {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (LEASE_TA | LEASE_NA) != 0) {
                if __bswap_32((*lease).addr.s_addr) >
                       __bswap_32((*context).start.s_addr) &&
                       __bswap_32((*lease).addr.s_addr) <=
                           __bswap_32((*context).end.s_addr) &&
                       __bswap_32((*lease).addr.s_addr) >
                           __bswap_32(addr.s_addr) {
                    addr = (*lease).addr
                }
            }
            lease = (*lease).next
        }
    }
    return addr;
}
#[c2rust::src_loc = "763:1"]
unsafe extern "C" fn lease_allocate() -> *mut dhcp_lease {
    let mut lease = 0 as *mut dhcp_lease;
    if leases_left == 0 ||
           {
               lease =
                   whine_malloc(::std::mem::size_of::<dhcp_lease>() as
                                    libc::c_ulong) as *mut dhcp_lease;
               lease.is_null()
           } {
        return NULL_0 as *mut dhcp_lease
    }
    memset(lease as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dhcp_lease>() as libc::c_ulong);
    (*lease).flags = LEASE_NEW;
    (*lease).expires = 1 as libc::c_int as time_t;
    (*lease).hwaddr_len = 256 as libc::c_int;
    (*lease).next = leases;
    leases = lease;
    file_dirty = 1 as libc::c_int;
    leases_left -= 1;
    return lease;
}
#[no_mangle]
#[c2rust::src_loc = "785:1"]
pub unsafe extern "C" fn lease4_allocate(mut addr: in_addr)
 -> *mut dhcp_lease {
    let mut lease = lease_allocate();
    if !lease.is_null() {
        (*lease).addr = addr;
        (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_4 as libc::c_int as
                                      usize] =
            (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_4 as libc::c_int
                                          as usize].wrapping_add(1)
    }
    return lease;
}
#[no_mangle]
#[c2rust::src_loc = "798:1"]
pub unsafe extern "C" fn lease6_allocate(mut addrp: *mut in6_addr,
                                         mut lease_type: libc::c_int)
 -> *mut dhcp_lease {
    let mut lease = lease_allocate();
    if !lease.is_null() {
        (*lease).addr6 = *addrp;
        (*lease).flags |= lease_type;
        (*lease).iaid = 0 as libc::c_int as libc::c_uint;
        (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_6 as libc::c_int as
                                      usize] =
            (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_6 as libc::c_int
                                          as usize].wrapping_add(1)
    }
    return lease;
}
#[no_mangle]
#[c2rust::src_loc = "815:1"]
pub unsafe extern "C" fn lease_set_expires(mut lease: *mut dhcp_lease,
                                           mut len: libc::c_uint,
                                           mut now: time_t) {
    let mut exp: time_t = 0;
    if len == 0xffffffff as libc::c_uint {
        exp = 0 as libc::c_int as time_t;
        len = 0 as libc::c_int as libc::c_uint
    } else {
        exp = now + len as time_t;
        /* Check for 2038 overflow. Make the lease
	 infinite in that case, as the least disruptive
	 thing we can do. */
        if difftime(exp, now) <= 0.0f64 { exp = 0 as libc::c_int as time_t }
    }
    if exp != (*lease).expires {
        dns_dirty = 1 as libc::c_int;
        (*lease).expires = exp;
        (*lease).flags |= LEASE_AUX_CHANGED | LEASE_EXP_CHANGED;
        file_dirty = 1 as libc::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "855:1"]
pub unsafe extern "C" fn lease_set_iaid(mut lease: *mut dhcp_lease,
                                        mut iaid: libc::c_uint) {
    if (*lease).iaid != iaid {
        (*lease).iaid = iaid;
        (*lease).flags |= LEASE_CHANGED
    };
}
#[no_mangle]
#[c2rust::src_loc = "865:1"]
pub unsafe extern "C" fn lease_set_hwaddr(mut lease: *mut dhcp_lease,
                                          mut hwaddr: *const libc::c_uchar,
                                          mut clid: *const libc::c_uchar,
                                          mut hw_len: libc::c_int,
                                          mut hw_type: libc::c_int,
                                          mut clid_len: libc::c_int,
                                          mut now: time_t,
                                          mut force: libc::c_int) {
    let mut change = force;
    (*lease).flags |= LEASE_HAVE_HWADDR;
    if hw_len != (*lease).hwaddr_len || hw_type != (*lease).hwaddr_type ||
           hw_len != 0 as libc::c_int &&
               memcmp((*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                      hwaddr as *const libc::c_void, hw_len as libc::c_ulong)
                   != 0 as libc::c_int {
        if hw_len != 0 as libc::c_int {
            memcpy((*lease).hwaddr.as_mut_ptr() as *mut libc::c_void,
                   hwaddr as *const libc::c_void, hw_len as libc::c_ulong);
        }
        (*lease).hwaddr_len = hw_len;
        (*lease).hwaddr_type = hw_type;
        (*lease).flags |= LEASE_CHANGED;
        file_dirty = 1 as libc::c_int
        /* run script on change */
    }
    /* only update clid when one is available, stops packets
     without a clid removing the record. Lease init uses
     clid_len == 0 for no clid. */
    if clid_len != 0 as libc::c_int && !clid.is_null() {
        if (*lease).clid.is_null() { (*lease).clid_len = 0 as libc::c_int }
        if (*lease).clid_len != clid_len {
            (*lease).flags |= LEASE_AUX_CHANGED;
            file_dirty = 1 as libc::c_int;
            free((*lease).clid as *mut libc::c_void);
            (*lease).clid =
                whine_malloc(clid_len as size_t) as *mut libc::c_uchar;
            if (*lease).clid.is_null() { return }
            change = 1 as libc::c_int
        } else if memcmp((*lease).clid as *const libc::c_void,
                         clid as *const libc::c_void,
                         clid_len as libc::c_ulong) != 0 as libc::c_int {
            (*lease).flags |= LEASE_AUX_CHANGED;
            file_dirty = 1 as libc::c_int;
            change = 1 as libc::c_int
        }
        (*lease).clid_len = clid_len;
        memcpy((*lease).clid as *mut libc::c_void,
               clid as *const libc::c_void, clid_len as libc::c_ulong);
    }
    if change != 0 { slaac_add_addrs(lease, now, force); };
}
#[c2rust::src_loc = "927:1"]
unsafe extern "C" fn kill_name(mut lease: *mut dhcp_lease) {
    /* run script to say we lost our old name */
    /* this shouldn't happen unless updates are very quick and the
     script very slow, we just avoid a memory leak if it does. */
    free((*lease).old_hostname as *mut libc::c_void);
    /* If we know the fqdn, pass that. The helper will derive the
     unqualified name from it, free the unqualified name here. */
    if !(*lease).fqdn.is_null() {
        (*lease).old_hostname = (*lease).fqdn;
        free((*lease).hostname as *mut libc::c_void);
    } else { (*lease).old_hostname = (*lease).hostname }
    (*lease).fqdn = NULL_0 as *mut libc::c_char;
    (*lease).hostname = (*lease).fqdn;
}
#[no_mangle]
#[c2rust::src_loc = "949:1"]
pub unsafe extern "C" fn lease_set_hostname(mut lease: *mut dhcp_lease,
                                            mut name: *const libc::c_char,
                                            mut auth: libc::c_int,
                                            mut domain: *mut libc::c_char,
                                            mut config_domain:
                                                *mut libc::c_char) {
    let mut lease_tmp = 0 as *mut dhcp_lease;
    let mut new_name = NULL_0 as *mut libc::c_char;
    let mut new_fqdn = NULL_0 as *mut libc::c_char;
    if !config_domain.is_null() &&
           (domain.is_null() || hostname_isequal(domain, config_domain) == 0)
       {
        my_syslog(MS_DHCP | LOG_WARNING,
                  b"Ignoring domain %s for DHCP host name %s\x00" as *const u8
                      as *const libc::c_char, config_domain, name);
    }
    if !(*lease).hostname.is_null() && !name.is_null() &&
           hostname_isequal((*lease).hostname, name) != 0 {
        if auth != 0 { (*lease).flags |= LEASE_AUTH_NAME }
        return
    }
    if name.is_null() && (*lease).hostname.is_null() { return }
    /* If a machine turns up on a new net without dropping the old lease,
     or two machines claim the same name, then we end up with two interfaces with
     the same name. Check for that here and remove the name from the old lease.
     Note that IPv6 leases are different. All the leases to the same DUID are 
     allowed the same name.

     Don't allow a name from the client to override a name from dnsmasq config. */
    if !name.is_null() {
        new_name =
            whine_malloc(strlen(name).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong)) as
                *mut libc::c_char;
        if !new_name.is_null() {
            strcpy(new_name, name);
            if !domain.is_null() &&
                   {
                       new_fqdn =
                           whine_malloc(strlen(new_name).wrapping_add(strlen(domain)).wrapping_add(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                               as *mut libc::c_char;
                       !new_fqdn.is_null()
                   } {
                strcpy(new_fqdn, name);
                strcat(new_fqdn,
                       b".\x00" as *const u8 as *const libc::c_char);
                strcat(new_fqdn, domain);
            }
        }
        let mut current_block_23: u64;
        /* Depending on mode, we check either unqualified name or FQDN. */
        lease_tmp = leases;
        while !lease_tmp.is_null() {
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
                   != 0 {
                if new_fqdn.is_null() || (*lease_tmp).fqdn.is_null() ||
                       hostname_isequal((*lease_tmp).fqdn, new_fqdn) == 0 {
                    current_block_23 = 17833034027772472439;
                } else { current_block_23 = 11307063007268554308; }
            } else if new_name.is_null() || (*lease_tmp).hostname.is_null() ||
                          hostname_isequal((*lease_tmp).hostname, new_name) ==
                              0 {
                current_block_23 = 17833034027772472439;
            } else { current_block_23 = 11307063007268554308; }
            match current_block_23 {
                11307063007268554308 => {
                    if (*lease).flags & (LEASE_TA | LEASE_NA) != 0 {
                        if (*lease_tmp).flags & (LEASE_TA | LEASE_NA) == 0 {
                            current_block_23 = 17833034027772472439;
                        } else if (*lease).clid_len == (*lease_tmp).clid_len
                                      && !(*lease).clid.is_null() &&
                                      !(*lease_tmp).clid.is_null() &&
                                      memcmp((*lease).clid as
                                                 *const libc::c_void,
                                             (*lease_tmp).clid as
                                                 *const libc::c_void,
                                             (*lease).clid_len as
                                                 libc::c_ulong) ==
                                          0 as libc::c_int {
                            current_block_23 = 17833034027772472439;
                        } else { current_block_23 = 1608152415753874203; }
                    } else if (*lease_tmp).flags & (LEASE_TA | LEASE_NA) != 0
                     {
                        current_block_23 = 17833034027772472439;
                    } else { current_block_23 = 1608152415753874203; }
                    match current_block_23 {
                        17833034027772472439 => { }
                        _ => {
                            if (*lease_tmp).flags & LEASE_AUTH_NAME != 0 &&
                                   auth == 0 {
                                free(new_name as *mut libc::c_void);
                                free(new_fqdn as *mut libc::c_void);
                                return
                            }
                            kill_name(lease_tmp);
                            break ;
                        }
                    }
                }
                _ => { }
            }
            lease_tmp = (*lease_tmp).next
        }
    }
    if !(*lease).hostname.is_null() { kill_name(lease); }
    (*lease).hostname = new_name;
    (*lease).fqdn = new_fqdn;
    if auth != 0 { (*lease).flags |= LEASE_AUTH_NAME }
    file_dirty = 1 as libc::c_int;
    dns_dirty = 1 as libc::c_int;
    (*lease).flags |= LEASE_CHANGED;
    /* another lease for the same DUID is OK for IPv6 */
    /* run script on change */
}
#[no_mangle]
#[c2rust::src_loc = "1042:1"]
pub unsafe extern "C" fn lease_set_interface(mut lease: *mut dhcp_lease,
                                             mut interface: libc::c_int,
                                             mut now: time_t) {
    if (*lease).last_interface == interface { return }
    (*lease).last_interface = interface;
    (*lease).flags |= LEASE_CHANGED;
    slaac_add_addrs(lease, now, 0 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1057:1"]
pub unsafe extern "C" fn rerun_scripts() {
    let mut lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).flags |= LEASE_CHANGED;
        lease = (*lease).next
    };
}
/* deleted leases get transferred to the old_leases list.
   remove them here, after calling the lease change
   script. Also run the lease change script on new/modified leases.

   Return zero if nothing to do. */
#[no_mangle]
#[c2rust::src_loc = "1070:1"]
pub unsafe extern "C" fn do_script_run(mut now: time_t) -> libc::c_int {
    let mut lease = 0 as *mut dhcp_lease;
    if !old_leases.is_null() {
        lease = old_leases;
        /* If the lease still has an old_hostname, do the "old" action on that first */
        if !(*lease).old_hostname.is_null() {
            queue_script(ACTION_OLD_HOSTNAME, lease, (*lease).old_hostname,
                         now);
            free((*lease).old_hostname as *mut libc::c_void);
            (*lease).old_hostname = NULL_0 as *mut libc::c_char;
            return 1 as libc::c_int
        } else {
            let mut slaac = 0 as *mut slaac_address;
            let mut tmp = 0 as *mut slaac_address;
            slaac = (*lease).slaac_address;
            while !slaac.is_null() {
                tmp = (*slaac).next;
                free(slaac as *mut libc::c_void);
                slaac = tmp
            }
            kill_name(lease);
            queue_script(ACTION_DEL, lease, (*lease).old_hostname, now);
            old_leases = (*lease).next;
            free((*lease).old_hostname as *mut libc::c_void);
            free((*lease).clid as *mut libc::c_void);
            free((*lease).extradata as *mut libc::c_void);
            free(lease as *mut libc::c_void);
            return 1 as libc::c_int
        }
    }
    /* make sure we announce the loss of a hostname before its new location. */
    lease = leases;
    while !lease.is_null() {
        if !(*lease).old_hostname.is_null() {
            queue_script(ACTION_OLD_HOSTNAME, lease, (*lease).old_hostname,
                         now);
            free((*lease).old_hostname as *mut libc::c_void);
            (*lease).old_hostname = NULL_0 as *mut libc::c_char;
            return 1 as libc::c_int
        }
        lease = (*lease).next
    }
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (LEASE_NEW | LEASE_CHANGED) != 0 ||
               (*lease).flags & LEASE_AUX_CHANGED != 0 &&
                   (*dnsmasq_daemon).options[(22 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (22 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 ||
               (*lease).flags & LEASE_EXP_CHANGED != 0 &&
                   (*dnsmasq_daemon).options[(61 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (61 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 {
            queue_script(if (*lease).flags & LEASE_NEW != 0 {
                             ACTION_ADD
                         } else { ACTION_OLD }, lease,
                         if !(*lease).fqdn.is_null() {
                             (*lease).fqdn
                         } else { (*lease).hostname }, now);
            (*lease).flags &=
                !(LEASE_NEW | LEASE_CHANGED | LEASE_AUX_CHANGED |
                      LEASE_EXP_CHANGED);
            /* this is used for the "add" call, then junked, since they're not in the database */
            free((*lease).extradata as *mut libc::c_void);
            (*lease).extradata = NULL_0 as *mut libc::c_uchar;
            return 1 as libc::c_int
        }
        lease = (*lease).next
    }
    return 0 as libc::c_int;
    /* nothing to do */
}
/* delim == -1 -> delim = 0, but embedded 0s, creating extra records, are OK. */
#[no_mangle]
#[c2rust::src_loc = "1164:1"]
pub unsafe extern "C" fn lease_add_extradata(mut lease: *mut dhcp_lease,
                                             mut data: *mut libc::c_uchar,
                                             mut len: libc::c_uint,
                                             mut delim: libc::c_int) {
    let mut i: libc::c_uint = 0;
    if delim == -(1 as libc::c_int) {
        delim = 0 as libc::c_int
    } else {
        /* check for embedded NULLs */
        i = 0 as libc::c_int as libc::c_uint;
        while i < len {
            if *data.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                len = i;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    }
    if (*lease).extradata_size.wrapping_sub((*lease).extradata_len) <
           len.wrapping_add(1 as libc::c_int as libc::c_uint) {
        let mut newsz =
            (*lease).extradata_len.wrapping_add(len).wrapping_add(100 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                as size_t;
        let mut new = whine_malloc(newsz) as *mut libc::c_uchar;
        if new.is_null() { return }
        if !(*lease).extradata.is_null() {
            memcpy(new as *mut libc::c_void,
                   (*lease).extradata as *const libc::c_void,
                   (*lease).extradata_len as libc::c_ulong);
            free((*lease).extradata as *mut libc::c_void);
        }
        (*lease).extradata = new;
        (*lease).extradata_size = newsz as libc::c_uint
    }
    if len != 0 as libc::c_int as libc::c_uint {
        memcpy((*lease).extradata.offset((*lease).extradata_len as isize) as
                   *mut libc::c_void, data as *const libc::c_void,
               len as libc::c_ulong);
    }
    *(*lease).extradata.offset((*lease).extradata_len.wrapping_add(len) as
                                   isize) = delim as libc::c_uchar;
    (*lease).extradata_len =
        (*lease).extradata_len.wrapping_add(len.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint));
}
