#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
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
    #[c2rust::src_loc = "1:9"]
    pub const VERSION: [libc::c_char; 8] =
        unsafe {
            *::std::mem::transmute::<&[u8; 8],
                                     &[libc::c_char; 8]>(b"2.84rc2\x00")
        };
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/types.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h:19"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src =
  "/usr/lib/llvm-10/lib/clang/10.0.0/include/stddef.h:19"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "89:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/__sigset_t.h:19"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_timespec.h:19"]
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
  "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/socket.h:19"]
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
    pub const AF_INET: libc::c_int = 2 as libc::c_int;
    use super::types_h::__socklen_t;
    use super::sockaddr_h::sa_family_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/sockaddr.h:19"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:19"]
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
    #[c2rust::src_loc = "190:9"]
    pub const INADDR_ANY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "233:9"]
    pub const INET_ADDRSTRLEN: libc::c_int = 16 as libc::c_int;
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:19"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint32_t, __uint16_t, __uint8_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/getopt_ext.h:19"]
pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct option {
        pub name: *const libc::c_char,
        pub has_arg: libc::c_int,
        pub flag: *mut libc::c_int,
        pub val: libc::c_int,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn getopt_long(___argc: libc::c_int,
                           ___argv: *const *mut libc::c_char,
                           __shortopts: *const libc::c_char,
                           __longopts: *const option,
                           __longind: *mut libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dnsmasq.h:19"]
pub mod dnsmasq_h {
    #[c2rust::src_loc = "69:1"]
    pub type u16_0 = libc::c_ushort;
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
    #[c2rust::src_loc = "263:9"]
    pub const OPT_LOCAL_SERVICE: libc::c_int = 49 as libc::c_int;
    #[c2rust::src_loc = "207:9"]
    pub const EC_BADCONF: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "211:9"]
    pub const EC_MISC: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "210:9"]
    pub const EC_NOMEM: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "209:9"]
    pub const EC_FILE: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "863:9"]
    pub const DHCP_PXE_DEF_VENDOR: [libc::c_char; 10] =
        unsafe {
            *::std::mem::transmute::<&[u8; 10],
                                     &[libc::c_char; 10]>(b"PXEClient\x00")
        };
    #[c2rust::src_loc = "523:9"]
    pub const SERV_HAS_SOURCE: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "416:9"]
    pub const HR_6: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "417:9"]
    pub const HR_4: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "869:9"]
    pub const MATCH_SUBSCRIBER: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "868:9"]
    pub const MATCH_REMOTE: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "867:9"]
    pub const MATCH_CIRCUIT: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "865:9"]
    pub const MATCH_VENDOR: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "866:9"]
    pub const MATCH_USER: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "839:9"]
    pub const DHOPT_VENDOR_PXE: libc::c_int = 16384 as libc::c_int;
    #[c2rust::src_loc = "833:9"]
    pub const DHOPT_VENDOR: libc::c_int = 256 as libc::c_int;
    #[c2rust::src_loc = "830:9"]
    pub const DHOPT_BANK: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "832:9"]
    pub const DHOPT_MATCH: libc::c_int = 128 as libc::c_int;
    #[c2rust::src_loc = "829:9"]
    pub const DHOPT_FORCE: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "827:9"]
    pub const DHOPT_ENCAPSULATE: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "836:9"]
    pub const DHOPT_RFC3925: libc::c_int = 2048 as libc::c_int;
    #[c2rust::src_loc = "826:9"]
    pub const DHOPT_STRING: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "694:9"]
    pub const OT_RFC1035_NAME: libc::c_int = 0x4000 as libc::c_int;
    #[c2rust::src_loc = "697:9"]
    pub const OT_CSTRING: libc::c_int = 0x800 as libc::c_int;
    #[c2rust::src_loc = "838:9"]
    pub const DHOPT_ADDR6: libc::c_int = 8192 as libc::c_int;
    #[c2rust::src_loc = "825:9"]
    pub const DHOPT_ADDR: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "834:9"]
    pub const DHOPT_HEX: libc::c_int = 512 as libc::c_int;
    #[c2rust::src_loc = "699:9"]
    pub const OT_TIME: libc::c_int = 0x200 as libc::c_int;
    #[c2rust::src_loc = "696:9"]
    pub const OT_NAME: libc::c_int = 0x1000 as libc::c_int;
    #[c2rust::src_loc = "693:9"]
    pub const OT_ADDR_LIST: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "695:9"]
    pub const OT_INTERNAL: libc::c_int = 0x2000 as libc::c_int;
    #[c2rust::src_loc = "802:9"]
    pub const CONFIG_TIME: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "803:9"]
    pub const CONFIG_NAME: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "810:9"]
    pub const CONFIG_ADDR6: libc::c_int = 4096 as libc::c_int;
    #[c2rust::src_loc = "801:9"]
    pub const CONFIG_CLID: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "800:9"]
    pub const CONFIG_DISABLE: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "804:9"]
    pub const CONFIG_ADDR: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "391:9"]
    pub const ADDRLIST_WILDCARD: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "390:9"]
    pub const ADDRLIST_PREFIX: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "805:9"]
    pub const CONFIG_NOCLID: libc::c_int = 128 as libc::c_int;
    #[c2rust::src_loc = "809:9"]
    pub const CONFIG_BANK: libc::c_int = 2048 as libc::c_int;
    #[c2rust::src_loc = "960:9"]
    pub const CONTEXT_SETLEASE: libc::c_uint =
        (1 as libc::c_uint) << 19 as libc::c_int;
    #[c2rust::src_loc = "950:9"]
    pub const CONTEXT_DEPRECATE: libc::c_uint =
        (1 as libc::c_uint) << 9 as libc::c_int;
    #[c2rust::src_loc = "951:9"]
    pub const CONTEXT_TEMPLATE: libc::c_uint =
        (1 as libc::c_uint) << 10 as libc::c_int;
    #[c2rust::src_loc = "954:9"]
    pub const CONTEXT_RA: libc::c_uint =
        (1 as libc::c_uint) << 13 as libc::c_int;
    #[c2rust::src_loc = "949:9"]
    pub const CONTEXT_DHCP: libc::c_uint =
        (1 as libc::c_uint) << 8 as libc::c_int;
    #[c2rust::src_loc = "959:9"]
    pub const CONTEXT_RA_OFF_LINK: libc::c_uint =
        (1 as libc::c_uint) << 18 as libc::c_int;
    #[c2rust::src_loc = "948:9"]
    pub const CONTEXT_RA_STATELESS: libc::c_uint =
        (1 as libc::c_uint) << 7 as libc::c_int;
    #[c2rust::src_loc = "945:9"]
    pub const CONTEXT_RA_ROUTER: libc::c_uint =
        (1 as libc::c_uint) << 4 as libc::c_int;
    #[c2rust::src_loc = "947:9"]
    pub const CONTEXT_RA_NAME: libc::c_uint =
        (1 as libc::c_uint) << 6 as libc::c_int;
    #[c2rust::src_loc = "941:9"]
    pub const CONTEXT_STATIC: libc::c_uint =
        (1 as libc::c_uint) << 0 as libc::c_int;
    #[c2rust::src_loc = "958:9"]
    pub const CONTEXT_V6: libc::c_uint =
        (1 as libc::c_uint) << 17 as libc::c_int;
    #[c2rust::src_loc = "943:9"]
    pub const CONTEXT_BRDCAST: libc::c_uint =
        (1 as libc::c_uint) << 2 as libc::c_int;
    #[c2rust::src_loc = "942:9"]
    pub const CONTEXT_NETMASK: libc::c_uint =
        (1 as libc::c_uint) << 1 as libc::c_int;
    #[c2rust::src_loc = "944:9"]
    pub const CONTEXT_PROXY: libc::c_uint =
        (1 as libc::c_uint) << 3 as libc::c_int;
    #[c2rust::src_loc = "270:9"]
    pub const OPT_TFTP_APREF_MAC: libc::c_int = 56 as libc::c_int;
    #[c2rust::src_loc = "243:9"]
    pub const OPT_TFTP_APREF_IP: libc::c_int = 29 as libc::c_int;
    #[c2rust::src_loc = "265:9"]
    pub const OPT_EXTRALOG: libc::c_int = 51 as libc::c_int;
    #[c2rust::src_loc = "216:9"]
    pub const OPT_LOG: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "532:9"]
    pub const SERV_FROM_FILE: libc::c_int = 4096 as libc::c_int;
    #[c2rust::src_loc = "520:9"]
    pub const SERV_NO_ADDR: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "522:9"]
    pub const SERV_HAS_DOMAIN: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "524:9"]
    pub const SERV_FOR_NODOTS: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "530:9"]
    pub const SERV_USE_RESOLV: libc::c_int = 1024 as libc::c_int;
    #[c2rust::src_loc = "531:9"]
    pub const SERV_NO_REBIND: libc::c_int = 2048 as libc::c_int;
    #[c2rust::src_loc = "528:9"]
    pub const SERV_TYPE: libc::c_int = SERV_HAS_DOMAIN | SERV_FOR_NODOTS;
    #[c2rust::src_loc = "521:9"]
    pub const SERV_LITERAL_ADDRESS: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "254:9"]
    pub const OPT_TFTP: libc::c_int = 40 as libc::c_int;
    #[c2rust::src_loc = "269:9"]
    pub const OPT_MAC_HEX: libc::c_int = 55 as libc::c_int;
    #[c2rust::src_loc = "268:9"]
    pub const OPT_MAC_B64: libc::c_int = 54 as libc::c_int;
    #[c2rust::src_loc = "246:9"]
    pub const OPT_ADD_MAC: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "229:9"]
    pub const OPT_RESOLV_DOMAIN: libc::c_int = 15 as libc::c_int;
    #[c2rust::src_loc = "402:9"]
    pub const AUTH4: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "401:9"]
    pub const AUTH6: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "388:9"]
    pub const ADDRLIST_IPV6: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "387:9"]
    pub const ADDRLIST_LITERAL: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "616:9"]
    pub const AH_HOSTS: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "618:9"]
    pub const AH_DHCP_OPT: libc::c_int = 32 as libc::c_int;
    #[c2rust::src_loc = "617:9"]
    pub const AH_DHCP_HST: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "501:9"]
    pub const SRC_AH: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const OPT_UBUS: libc::c_int = 58 as libc::c_int;
    #[c2rust::src_loc = "233:9"]
    pub const OPT_DBUS: libc::c_int = 19 as libc::c_int;
    #[c2rust::src_loc = "255:9"]
    pub const OPT_CLIENT_SUBNET: libc::c_int = 41 as libc::c_int;
    #[c2rust::src_loc = "275:9"]
    pub const OPT_LEASE_RENEW: libc::c_int = 61 as libc::c_int;
    #[c2rust::src_loc = "276:9"]
    pub const OPT_LAST: libc::c_int = 62 as libc::c_int;
    #[c2rust::src_loc = "271:9"]
    pub const OPT_RAPID_COMMIT: libc::c_int = 57 as libc::c_int;
    #[c2rust::src_loc = "264:9"]
    pub const OPT_LOOP_DETECT: libc::c_int = 50 as libc::c_int;
    #[c2rust::src_loc = "258:9"]
    pub const OPT_QUIET_RA: libc::c_int = 44 as libc::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const OPT_QUIET_DHCP6: libc::c_int = 43 as libc::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const OPT_QUIET_DHCP: libc::c_int = 42 as libc::c_int;
    #[c2rust::src_loc = "260:9"]
    pub const OPT_DNSSEC_TIME: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "261:9"]
    pub const OPT_DNSSEC_DEBUG: libc::c_int = 47 as libc::c_int;
    #[c2rust::src_loc = "259:9"]
    pub const OPT_DNSSEC_VALID: libc::c_int = 45 as libc::c_int;
    #[c2rust::src_loc = "253:9"]
    pub const OPT_CLEVERBIND: libc::c_int = 39 as libc::c_int;
    #[c2rust::src_loc = "251:9"]
    pub const OPT_RA: libc::c_int = 37 as libc::c_int;
    #[c2rust::src_loc = "250:9"]
    pub const OPT_FQDN_UPDATE: libc::c_int = 36 as libc::c_int;
    #[c2rust::src_loc = "249:9"]
    pub const OPT_CONNTRACK: libc::c_int = 35 as libc::c_int;
    #[c2rust::src_loc = "273:9"]
    pub const OPT_IGNORE_CLID: libc::c_int = 59 as libc::c_int;
    #[c2rust::src_loc = "248:9"]
    pub const OPT_CONSEC_ADDR: libc::c_int = 34 as libc::c_int;
    #[c2rust::src_loc = "247:9"]
    pub const OPT_DNSSEC_PROXY: libc::c_int = 33 as libc::c_int;
    #[c2rust::src_loc = "234:9"]
    pub const OPT_DHCP_FQDN: libc::c_int = 20 as libc::c_int;
    #[c2rust::src_loc = "237:9"]
    pub const OPT_ALL_SERVERS: libc::c_int = 23 as libc::c_int;
    #[c2rust::src_loc = "239:9"]
    pub const OPT_LOCAL_REBIND: libc::c_int = 25 as libc::c_int;
    #[c2rust::src_loc = "245:9"]
    pub const OPT_NO_REBIND: libc::c_int = 31 as libc::c_int;
    #[c2rust::src_loc = "242:9"]
    pub const OPT_LOG_OPTS: libc::c_int = 28 as libc::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const OPT_SINGLE_PORT: libc::c_int = 60 as libc::c_int;
    #[c2rust::src_loc = "252:9"]
    pub const OPT_TFTP_LC: libc::c_int = 38 as libc::c_int;
    #[c2rust::src_loc = "241:9"]
    pub const OPT_TFTP_NOBLOCK: libc::c_int = 27 as libc::c_int;
    #[c2rust::src_loc = "266:9"]
    pub const OPT_TFTP_NO_FAIL: libc::c_int = 52 as libc::c_int;
    #[c2rust::src_loc = "240:9"]
    pub const OPT_TFTP_SECURE: libc::c_int = 26 as libc::c_int;
    #[c2rust::src_loc = "244:9"]
    pub const OPT_NO_OVERRIDE: libc::c_int = 30 as libc::c_int;
    #[c2rust::src_loc = "238:9"]
    pub const OPT_RELOAD: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "236:9"]
    pub const OPT_LEASE_RO: libc::c_int = 22 as libc::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const OPT_SCRIPT_ARP: libc::c_int = 53 as libc::c_int;
    #[c2rust::src_loc = "235:9"]
    pub const OPT_NO_PING: libc::c_int = 21 as libc::c_int;
    #[c2rust::src_loc = "228:9"]
    pub const OPT_ETHERS: libc::c_int = 14 as libc::c_int;
    #[c2rust::src_loc = "227:9"]
    pub const OPT_NOWILD: libc::c_int = 13 as libc::c_int;
    #[c2rust::src_loc = "232:9"]
    pub const OPT_LOCALISE: libc::c_int = 18 as libc::c_int;
    #[c2rust::src_loc = "222:9"]
    pub const OPT_NO_RESOLV: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "221:9"]
    pub const OPT_ORDER: libc::c_int = 7 as libc::c_int;
    #[c2rust::src_loc = "225:9"]
    pub const OPT_NO_NEG: libc::c_int = 11 as libc::c_int;
    #[c2rust::src_loc = "219:9"]
    pub const OPT_NO_POLL: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "224:9"]
    pub const OPT_LOCALMX: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "231:9"]
    pub const OPT_AUTHORITATIVE: libc::c_int = 17 as libc::c_int;
    #[c2rust::src_loc = "230:9"]
    pub const OPT_NO_FORK: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "218:9"]
    pub const OPT_NO_HOSTS: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "215:9"]
    pub const OPT_FILTER: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "223:9"]
    pub const OPT_EXPAND: libc::c_int = 9 as libc::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const OPT_SELFMX: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "226:9"]
    pub const OPT_NODOTS_LOCAL: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "220:9"]
    pub const OPT_DEBUG: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "214:9"]
    pub const OPT_BOGUSPRIV: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "17:9"]
    pub const COPYRIGHT: [libc::c_char; 37] =
        unsafe {
            *::std::mem::transmute::<&[u8; 37],
                                     &[libc::c_char; 37]>(b"Copyright (c) 2000-2021 Simon Kelley\x00")
        };
    #[c2rust::src_loc = "613:9"]
    pub const AH_DIR: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "614:9"]
    pub const AH_INACTIVE: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "359:9"]
    pub const TXT_STAT_SERVERS: libc::c_int = 7 as libc::c_int;
    #[c2rust::src_loc = "287:9"]
    pub const MS_DHCP: libc::c_int = LOG_DAEMON;
    #[c2rust::src_loc = "358:9"]
    pub const TXT_STAT_AUTH: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "357:9"]
    pub const TXT_STAT_HITS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "356:9"]
    pub const TXT_STAT_MISSES: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "355:9"]
    pub const TXT_STAT_EVICTIONS: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "354:9"]
    pub const TXT_STAT_INSERTS: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "353:9"]
    pub const TXT_STAT_CACHESIZE: libc::c_int = 1 as libc::c_int;
    use super::in_h::{in_addr, in6_addr, sockaddr_in, sockaddr_in6};
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
        #[c2rust::src_loc = "1283:1"]
        pub fn rand32() -> u32_0;
        #[no_mangle]
        #[c2rust::src_loc = "1285:1"]
        pub fn legal_hostname(name: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1286:1"]
        pub fn canonicalise(in_0: *mut libc::c_char, nomem: *mut libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1287:1"]
        pub fn do_rfc1035_name(p: *mut libc::c_uchar, sval: *mut libc::c_char,
                               limit: *mut libc::c_char)
         -> *mut libc::c_uchar;
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
        #[c2rust::src_loc = "1294:1"]
        pub fn hostname_isequal(a: *const libc::c_char,
                                b: *const libc::c_char) -> libc::c_int;
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
        #[c2rust::src_loc = "1301:1"]
        pub fn setaddr6part(addr: *mut in6_addr, host: u64_0);
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
        #[c2rust::src_loc = "1589:1"]
        pub fn lookup_dhcp_len(prot: libc::c_int, val: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1588:1"]
        pub fn lookup_dhcp_opt(prot: libc::c_int, name: *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1583:1"]
        pub fn strip_hostname(hostname: *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "1363:1"]
        pub fn mark_servers(flag: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "1364:1"]
        pub fn cleanup_servers();
        #[no_mangle]
        #[c2rust::src_loc = "1602:1"]
        pub fn display_opts6();
        #[no_mangle]
        #[c2rust::src_loc = "1587:1"]
        pub fn display_opts();
        #[no_mangle]
        #[c2rust::src_loc = "1647:1"]
        pub fn set_dynamic_inotify(flag: libc::c_int, total_size: libc::c_int,
                                   rhash: *mut *mut crec,
                                   revhashsz: libc::c_int);
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stat.h:19"]
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
    #[c2rust::src_loc = "179:9"]
    pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t, __ino64_t, __blkcnt64_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:19"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
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
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:19"]
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
        #[c2rust::src_loc = "83:1"]
        pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
    }
}
#[c2rust::header_src = "/usr/include/stdint.h:19"]
pub mod stdint_h {
    #[c2rust::src_loc = "101:1"]
    pub type intmax_t = __intmax_t;
    #[c2rust::src_loc = "102:1"]
    pub type uintmax_t = __uintmax_t;
    use super::types_h::{__intmax_t, __uintmax_t};
}
#[c2rust::header_src = "/usr/include/inttypes.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/syslog.h:19"]
pub mod syslog_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:16"]
    pub struct _code {
        pub c_name: *mut libc::c_char,
        pub c_val: libc::c_int,
    }
    #[c2rust::src_loc = "69:1"]
    pub type CODE = _code;
    #[no_mangle]
    #[c2rust::src_loc = "74:6"]
    pub static mut prioritynames: [CODE; 13] =
        [{
             let mut init =
                 _code{c_name:
                           b"alert\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_ALERT,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"crit\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_CRIT,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"debug\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_DEBUG,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"emerg\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_EMERG,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"err\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_ERR,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"error\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_ERR,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"info\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_INFO,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"none\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: INTERNAL_NOPRI,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"notice\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_NOTICE,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"panic\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_EMERG,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"warn\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_WARNING,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"warning\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_WARNING,};
             init
         },
         {
             let mut init =
                 _code{c_name: NULL_0 as *mut libc::c_char,
                       c_val: -(1 as libc::c_int),};
             init
         }];
    #[c2rust::src_loc = "55:9"]
    pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const LOG_EMERG: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const LOG_NOTICE: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "66:9"]
    pub const INTERNAL_NOPRI: libc::c_int = 0x10 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const LOG_INFO: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const LOG_ERR: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "58:9"]
    pub const LOG_DEBUG: libc::c_int = 7 as libc::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const LOG_CRIT: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "52:9"]
    pub const LOG_ALERT: libc::c_int = 1 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "122:6"]
    pub static mut facilitynames: [CODE; 23] =
        [{
             let mut init =
                 _code{c_name:
                           b"auth\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_AUTH,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"authpriv\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_AUTHPRIV,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"cron\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_CRON,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"daemon\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_DAEMON,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"ftp\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_FTP,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"kern\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_KERN,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"lpr\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_LPR,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"mail\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_MAIL,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"mark\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: INTERNAL_MARK,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"news\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_NEWS,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"security\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_AUTH,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"syslog\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_SYSLOG,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"user\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_USER,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"uucp\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                       c_val: LOG_UUCP,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local0\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL0,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local1\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL1,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local2\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL2,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local3\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL3,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local4\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL4,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local5\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL5,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local6\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL6,};
             init
         },
         {
             let mut init =
                 _code{c_name:
                           b"local7\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                       c_val: LOG_LOCAL7,};
             init
         },
         {
             let mut init =
                 _code{c_name: NULL_0 as *mut libc::c_char,
                       c_val: -(1 as libc::c_int),};
             init
         }];
    #[c2rust::src_loc = "114:9"]
    pub const LOG_LOCAL7: libc::c_int =
        (23 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "113:9"]
    pub const LOG_LOCAL6: libc::c_int =
        (22 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "112:9"]
    pub const LOG_LOCAL5: libc::c_int =
        (21 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "111:9"]
    pub const LOG_LOCAL4: libc::c_int =
        (20 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "110:9"]
    pub const LOG_LOCAL3: libc::c_int =
        (19 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "109:9"]
    pub const LOG_LOCAL2: libc::c_int =
        (18 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const LOG_LOCAL1: libc::c_int =
        (17 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "107:9"]
    pub const LOG_LOCAL0: libc::c_int =
        (16 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "101:9"]
    pub const LOG_UUCP: libc::c_int = (8 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "94:9"]
    pub const LOG_USER: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "98:9"]
    pub const LOG_SYSLOG: libc::c_int =
        (5 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "97:9"]
    pub const LOG_AUTH: libc::c_int = (4 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "100:9"]
    pub const LOG_NEWS: libc::c_int = (7 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "68:9"]
    pub const INTERNAL_MARK: libc::c_int =
        (24 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int;
    #[c2rust::src_loc = "95:9"]
    pub const LOG_MAIL: libc::c_int = (2 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "99:9"]
    pub const LOG_LPR: libc::c_int = (6 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "93:9"]
    pub const LOG_KERN: libc::c_int = (0 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "104:9"]
    pub const LOG_FTP: libc::c_int = (11 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "96:9"]
    pub const LOG_DAEMON: libc::c_int =
        (3 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "102:9"]
    pub const LOG_CRON: libc::c_int = (9 as libc::c_int) << 3 as libc::c_int;
    #[c2rust::src_loc = "103:9"]
    pub const LOG_AUTHPRIV: libc::c_int =
        (10 as libc::c_int) << 3 as libc::c_int;
    use super::stddef_h::NULL_0;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/dirent.h:19"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct dirent {
        pub d_ino: __ino64_t,
        pub d_off: __off64_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    use super::types_h::{__ino64_t, __off64_t};
}
#[c2rust::header_src = "/usr/include/dirent.h:19"]
pub mod include_dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "165:1"]
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/usr/include/setjmp.h:20"]
pub mod include_setjmp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: __jmp_buf,
        pub __mask_was_saved: libc::c_int,
        pub __saved_mask: __sigset_t,
    }
    #[c2rust::src_loc = "45:1"]
    pub type jmp_buf = [__jmp_buf_tag; 1];
    use super::setjmp_h::__jmp_buf;
    use super::__sigset_t_h::__sigset_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "58:12"]
        pub fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "67:13"]
        pub fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/setjmp.h:20"]
pub mod setjmp_h {
    #[c2rust::src_loc = "31:1"]
    pub type __jmp_buf = [libc::c_long; 8];
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/getopt_core.h:19"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "55:12"]
        pub static mut opterr: libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:19"]
pub mod inet_h {
    use super::in_h::{in_addr, in_addr_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                         __buf: *mut libc::c_void) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/stat.h:19"]
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
#[c2rust::header_src = "/usr/include/string.h:19"]
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
        #[c2rust::src_loc = "253:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "330:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "336:14"]
        pub fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn strcasestr(__haystack: *const libc::c_char,
                          __needle: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "397:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:19"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/net/if.h:19"]
pub mod if_h {
    #[c2rust::src_loc = "31:9"]
    pub const IF_NAMESIZE: libc::c_int = 16 as libc::c_int;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "193:1"]
        pub fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:19"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "877:1"]
        pub fn gethostname(__name: *mut libc::c_char, __len: size_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:19"]
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
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "257:26"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdio.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdlib-float.h:19"]
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
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:19"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:19"]
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
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/stdlib-bsearch.h:19"]
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
#[c2rust::header_src = "/usr/include/errno.h:19"]
pub mod errno_h {
    #[c2rust::src_loc = "38:10"]
    pub const errno: libc::c_int = *__errno_location();
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:19"]
pub mod dns_protocol_h {
    #[c2rust::src_loc = "26:9"]
    pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const C_IN: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "79:9"]
    pub const T_CAA: libc::c_int = 257 as libc::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const IN6ADDRSZ: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "23:9"]
    pub const INADDRSZ: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "17:9"]
    pub const NAMESERVER_PORT: libc::c_int = 53 as libc::c_int;
    #[c2rust::src_loc = "28:9"]
    pub const MAXLABEL: libc::c_int = 63 as libc::c_int;
    #[c2rust::src_loc = "40:9"]
    pub const C_CHAOS: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "19:9"]
    pub const MIN_PORT: libc::c_int = 1024 as libc::c_int;
    #[c2rust::src_loc = "20:9"]
    pub const MAX_PORT: libc::c_uint = 65535 as libc::c_uint;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/config.h:19"]
pub mod config_h {
    #[c2rust::src_loc = "215:15"]
    pub const CONFFILE: [libc::c_char; 18] =
        unsafe {
            *::std::mem::transmute::<&[u8; 18],
                                     &[libc::c_char; 18]>(b"/etc/dnsmasq.conf\x00")
        };
    #[c2rust::src_loc = "47:9"]
    pub const DEFLEASE6: libc::c_int =
        3600 as libc::c_int * 24 as libc::c_int;
    #[c2rust::src_loc = "46:9"]
    pub const DEFLEASE: libc::c_int = 3600 as libc::c_int;
    #[c2rust::src_loc = "35:9"]
    pub const TTL_FLOOR_LIMIT: libc::c_int = 3600 as libc::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const LOG_MAX: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const DNSMASQ_UBUS_NAME: [libc::c_char; 8] =
        unsafe {
            *::std::mem::transmute::<&[u8; 8],
                                     &[libc::c_char; 8]>(b"dnsmasq\x00")
        };
    #[c2rust::src_loc = "53:9"]
    pub const DNSMASQ_SERVICE: [libc::c_char; 26] =
        unsafe {
            *::std::mem::transmute::<&[u8; 26],
                                     &[libc::c_char; 26]>(b"uk.org.thekelleys.dnsmasq\x00")
        };
    #[c2rust::src_loc = "223:15"]
    pub const RESOLVFILE: [libc::c_char; 17] =
        unsafe {
            *::std::mem::transmute::<&[u8; 17],
                                     &[libc::c_char; 17]>(b"/etc/resolv.conf\x00")
        };
    #[c2rust::src_loc = "45:9"]
    pub const ETHERSFILE: [libc::c_char; 12] =
        unsafe {
            *::std::mem::transmute::<&[u8; 12],
                                     &[libc::c_char; 12]>(b"/etc/ethers\x00")
        };
    #[c2rust::src_loc = "231:15"]
    pub const RUNFILE: [libc::c_char; 21] =
        unsafe {
            *::std::mem::transmute::<&[u8; 21],
                                     &[libc::c_char; 21]>(b"/var/run/dnsmasq.pid\x00")
        };
    #[c2rust::src_loc = "48:9"]
    pub const CHUSER: [libc::c_char; 7] =
        unsafe {
            *::std::mem::transmute::<&[u8; 7],
                                     &[libc::c_char; 7]>(b"nobody\x00")
        };
    #[c2rust::src_loc = "207:15"]
    pub const LEASEFILE: [libc::c_char; 29] =
        unsafe {
            *::std::mem::transmute::<&[u8; 29],
                                     &[libc::c_char; 29]>(b"/var/lib/misc/dnsmasq.leases\x00")
        };
    #[c2rust::src_loc = "44:9"]
    pub const HOSTSFILE: [libc::c_char; 11] =
        unsafe {
            *::std::mem::transmute::<&[u8; 11],
                                     &[libc::c_char; 11]>(b"/etc/hosts\x00")
        };
    #[c2rust::src_loc = "49:9"]
    pub const CHGRP: [libc::c_char; 4] =
        unsafe {
            *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dip\x00")
        };
    #[c2rust::src_loc = "50:9"]
    pub const TFTP_MAX_CONNECTIONS: libc::c_int = 50 as libc::c_int;
    #[c2rust::src_loc = "17:9"]
    pub const FTABSIZ: libc::c_int = 150 as libc::c_int;
    #[c2rust::src_loc = "36:9"]
    pub const MAXLEASES: libc::c_int = 1000 as libc::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const EDNS_PKTSZ: libc::c_int = 4096 as libc::c_int;
    #[c2rust::src_loc = "34:9"]
    pub const CACHESIZ: libc::c_int = 150 as libc::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const SOA_EXPIRY: libc::c_int = 1209600 as libc::c_int;
    #[c2rust::src_loc = "58:9"]
    pub const SOA_RETRY: libc::c_int = 180 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const SOA_REFRESH: libc::c_int = 1200 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const AUTH_TTL: libc::c_int = 600 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dhcp-protocol.h:19"]
pub mod dhcp_protocol_h {
    #[c2rust::src_loc = "19:9"]
    pub const DHCP_CLIENT_ALTPORT: libc::c_int = 1068 as libc::c_int;
    #[c2rust::src_loc = "18:9"]
    pub const DHCP_SERVER_ALTPORT: libc::c_int = 1067 as libc::c_int;
    #[c2rust::src_loc = "92:9"]
    pub const DHCP_CHADDR_MAX: libc::c_int = 16 as libc::c_int;
    #[c2rust::src_loc = "63:9"]
    pub const OPTION_DOMAIN_SEARCH: libc::c_int = 119 as libc::c_int;
    #[c2rust::src_loc = "64:9"]
    pub const OPTION_SIP_SERVER: libc::c_int = 120 as libc::c_int;
    #[c2rust::src_loc = "16:9"]
    pub const DHCP_SERVER_PORT: libc::c_int = 67 as libc::c_int;
    #[c2rust::src_loc = "17:9"]
    pub const DHCP_CLIENT_PORT: libc::c_int = 68 as libc::c_int;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dhcp6-protocol.h:19"]
pub mod dhcp6_protocol_h {
    #[c2rust::src_loc = "62:9"]
    pub const OPTION6_NTP_SERVER: libc::c_int = 56 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/asm-generic/errno-base.h:19"]
pub mod errno_base_h {
    #[c2rust::src_loc = "6:9"]
    pub const ENOENT: libc::c_int = 2 as libc::c_int;
}
pub use self::internal::{__va_list_tag, VERSION};
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __intmax_t, __uintmax_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __ino64_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __pid_t, __time_t, __blksize_t,
                        __blkcnt_t, __blkcnt64_t, __ssize_t,
                        __syscall_slong_t, __socklen_t};
pub use self::sys_types_h::{ino_t, dev_t, off_t, pid_t, ssize_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::__sigset_t_h::__sigset_t;
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, sockaddr, msghdr, cmsghdr, __cmsg_nxthdr,
                         PF_INET6, AF_INET6, PF_INET, AF_INET};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, INADDR_ANY,
                     INET_ADDRSTRLEN, INET6_ADDRSTRLEN, in6addr_any};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::getopt_ext_h::{option, getopt_long};
pub use self::dnsmasq_h::{u16_0, u32_0, u64_0, all_addr, C2RustUnnamed_1,
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
                          dhcp_relay, dnsmasq_daemon, OPT_LOCAL_SERVICE,
                          EC_BADCONF, EC_MISC, EC_NOMEM, EC_FILE,
                          DHCP_PXE_DEF_VENDOR, SERV_HAS_SOURCE, HR_6, HR_4,
                          MATCH_SUBSCRIBER, MATCH_REMOTE, MATCH_CIRCUIT,
                          MATCH_VENDOR, MATCH_USER, DHOPT_VENDOR_PXE,
                          DHOPT_VENDOR, DHOPT_BANK, DHOPT_MATCH, DHOPT_FORCE,
                          DHOPT_ENCAPSULATE, DHOPT_RFC3925, DHOPT_STRING,
                          OT_RFC1035_NAME, OT_CSTRING, DHOPT_ADDR6,
                          DHOPT_ADDR, DHOPT_HEX, OT_TIME, OT_NAME,
                          OT_ADDR_LIST, OT_INTERNAL, CONFIG_TIME, CONFIG_NAME,
                          CONFIG_ADDR6, CONFIG_CLID, CONFIG_DISABLE,
                          CONFIG_ADDR, ADDRLIST_WILDCARD, ADDRLIST_PREFIX,
                          CONFIG_NOCLID, CONFIG_BANK, CONTEXT_SETLEASE,
                          CONTEXT_DEPRECATE, CONTEXT_TEMPLATE, CONTEXT_RA,
                          CONTEXT_DHCP, CONTEXT_RA_OFF_LINK,
                          CONTEXT_RA_STATELESS, CONTEXT_RA_ROUTER,
                          CONTEXT_RA_NAME, CONTEXT_STATIC, CONTEXT_V6,
                          CONTEXT_BRDCAST, CONTEXT_NETMASK, CONTEXT_PROXY,
                          OPT_TFTP_APREF_MAC, OPT_TFTP_APREF_IP, OPT_EXTRALOG,
                          OPT_LOG, SERV_FROM_FILE, SERV_NO_ADDR,
                          SERV_HAS_DOMAIN, SERV_FOR_NODOTS, SERV_USE_RESOLV,
                          SERV_NO_REBIND, SERV_TYPE, SERV_LITERAL_ADDRESS,
                          OPT_TFTP, OPT_MAC_HEX, OPT_MAC_B64, OPT_ADD_MAC,
                          OPT_RESOLV_DOMAIN, AUTH4, AUTH6, ADDRLIST_IPV6,
                          ADDRLIST_LITERAL, AH_HOSTS, AH_DHCP_OPT,
                          AH_DHCP_HST, SRC_AH, OPT_UBUS, OPT_DBUS,
                          OPT_CLIENT_SUBNET, OPT_LEASE_RENEW, OPT_LAST,
                          OPT_RAPID_COMMIT, OPT_LOOP_DETECT, OPT_QUIET_RA,
                          OPT_QUIET_DHCP6, OPT_QUIET_DHCP, OPT_DNSSEC_TIME,
                          OPT_DNSSEC_DEBUG, OPT_DNSSEC_VALID, OPT_CLEVERBIND,
                          OPT_RA, OPT_FQDN_UPDATE, OPT_CONNTRACK,
                          OPT_IGNORE_CLID, OPT_CONSEC_ADDR, OPT_DNSSEC_PROXY,
                          OPT_DHCP_FQDN, OPT_ALL_SERVERS, OPT_LOCAL_REBIND,
                          OPT_NO_REBIND, OPT_LOG_OPTS, OPT_SINGLE_PORT,
                          OPT_TFTP_LC, OPT_TFTP_NOBLOCK, OPT_TFTP_NO_FAIL,
                          OPT_TFTP_SECURE, OPT_NO_OVERRIDE, OPT_RELOAD,
                          OPT_LEASE_RO, OPT_SCRIPT_ARP, OPT_NO_PING,
                          OPT_ETHERS, OPT_NOWILD, OPT_LOCALISE, OPT_NO_RESOLV,
                          OPT_ORDER, OPT_NO_NEG, OPT_NO_POLL, OPT_LOCALMX,
                          OPT_AUTHORITATIVE, OPT_NO_FORK, OPT_NO_HOSTS,
                          OPT_FILTER, OPT_EXPAND, OPT_SELFMX,
                          OPT_NODOTS_LOCAL, OPT_DEBUG, OPT_BOGUSPRIV,
                          COPYRIGHT, AH_DIR, AH_INACTIVE, TXT_STAT_SERVERS,
                          MS_DHCP, TXT_STAT_AUTH, TXT_STAT_HITS,
                          TXT_STAT_MISSES, TXT_STAT_EVICTIONS,
                          TXT_STAT_INSERTS, TXT_STAT_CACHESIZE, rand32,
                          legal_hostname, canonicalise, do_rfc1035_name,
                          safe_malloc, safe_strncpy, whine_malloc,
                          hostname_isequal, is_same_net, is_same_net6,
                          addr6part, setaddr6part, parse_hex, die, my_syslog,
                          lookup_dhcp_len, lookup_dhcp_opt, strip_hostname,
                          mark_servers, cleanup_servers, display_opts6,
                          display_opts, set_dynamic_inotify};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER, __S_IFMT};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll, free, exit};
pub use self::ctype_h::{C2RustUnnamed_0, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper, tolower,
                        toupper, __ctype_toupper_loc, __ctype_b_loc,
                        __ctype_tolower_loc};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
pub use self::syslog_h::{_code, CODE, prioritynames, LOG_WARNING, LOG_EMERG,
                         LOG_NOTICE, INTERNAL_NOPRI, LOG_INFO, LOG_ERR,
                         LOG_DEBUG, LOG_CRIT, LOG_ALERT, facilitynames,
                         LOG_LOCAL7, LOG_LOCAL6, LOG_LOCAL5, LOG_LOCAL4,
                         LOG_LOCAL3, LOG_LOCAL2, LOG_LOCAL1, LOG_LOCAL0,
                         LOG_UUCP, LOG_USER, LOG_SYSLOG, LOG_AUTH, LOG_NEWS,
                         INTERNAL_MARK, LOG_MAIL, LOG_LPR, LOG_KERN, LOG_FTP,
                         LOG_DAEMON, LOG_CRON, LOG_AUTHPRIV};
pub use self::dirent_h::dirent;
pub use self::include_dirent_h::{DIR, __dirstream, opendir, closedir,
                                 readdir};
pub use self::include_setjmp_h::{__jmp_buf_tag, jmp_buf, _setjmp, longjmp};
pub use self::setjmp_h::__jmp_buf;
use self::getopt_core_h::{optarg, optind, opterr};
use self::inet_h::{inet_ntoa, inet_pton};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::string_h::{memcpy, memmove, memset, strcpy, strcat, strcmp, strchr,
                     strrchr, strstr, strtok, strcasestr, strlen, strerror};
use self::strings_h::strcasecmp;
pub use self::if_h::{IF_NAMESIZE, if_nametoindex};
use self::unistd_h::gethostname;
use self::stdio_h::{stdin, stdout, stderr, fclose, fopen, fprintf, printf,
                    sprintf, vfprintf, getc, __uflow, putc, __overflow, fgets,
                    __getdelim};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::uintn_identity_h::{__uint64_identity, __uint32_identity,
                                 __uint16_identity};
pub use self::byteswap_h::{__bswap_64, __bswap_32, __bswap_16};
pub use self::stdlib_bsearch_h::bsearch;
pub use self::errno_h::{errno, __errno_location};
pub use self::dns_protocol_h::{MAXDNAME, C_IN, T_CAA, IN6ADDRSZ, INADDRSZ,
                               NAMESERVER_PORT, MAXLABEL, C_CHAOS, MIN_PORT,
                               MAX_PORT};
pub use self::config_h::{CONFFILE, DEFLEASE6, DEFLEASE, TTL_FLOOR_LIMIT,
                         LOG_MAX, DNSMASQ_UBUS_NAME, DNSMASQ_SERVICE,
                         RESOLVFILE, ETHERSFILE, RUNFILE, CHUSER, LEASEFILE,
                         HOSTSFILE, CHGRP, TFTP_MAX_CONNECTIONS, FTABSIZ,
                         MAXLEASES, EDNS_PKTSZ, CACHESIZ, SOA_EXPIRY,
                         SOA_RETRY, SOA_REFRESH, AUTH_TTL};
pub use self::dhcp_protocol_h::{DHCP_CLIENT_ALTPORT, DHCP_SERVER_ALTPORT,
                                DHCP_CHADDR_MAX, OPTION_DOMAIN_SEARCH,
                                OPTION_SIP_SERVER, DHCP_SERVER_PORT,
                                DHCP_CLIENT_PORT};
pub use self::dhcp6_protocol_h::OPTION6_NTP_SERVER;
pub use self::errno_base_h::ENOENT;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1703:9"]
pub struct list {
    pub name: *mut libc::c_char,
    pub next: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "353:8"]
pub struct C2RustUnnamed_9 {
    pub opt: libc::c_int,
    pub rept: libc::c_uint,
    pub flagdesc: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub arg: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "4713:17"]
pub struct fileread {
    pub dev: dev_t,
    pub ino: ino_t,
    pub next: *mut fileread,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "716:3"]
pub struct C2RustUnnamed_10 {
    pub handle: libc::c_char,
    pub val: libc::c_int,
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
/* define this to get facilitynames */
#[c2rust::src_loc = "22:21"]
static mut mem_recover: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "23:16"]
static mut mem_jmp: jmp_buf =
    [__jmp_buf_tag{__jmpbuf: [0; 8],
                   __mask_was_saved: 0,
                   __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
/* Solaris headers don't have facility names. */
#[c2rust::src_loc = "64:9"]
pub const OPTSTRING: [libc::c_char; 102] =
    unsafe {
        *::std::mem::transmute::<&[u8; 102],
                                 &[libc::c_char; 102]>(b"951yZDNLERKzowefnbvhdkqr:m:p:c:l:s:i:t:u:g:a:x:S:C:A:T:H:Q:I:B:F:G:O:M:X:V:U:j:P:J:W:Y:2:4:6:7:8:0:3:\x00")
    };
/* options which don't have a one-char version */
#[c2rust::src_loc = "67:9"]
pub const LOPT_RELOAD: libc::c_int = 256 as libc::c_int;
#[c2rust::src_loc = "68:9"]
pub const LOPT_NO_NAMES: libc::c_int = 257 as libc::c_int;
#[c2rust::src_loc = "69:9"]
pub const LOPT_TFTP: libc::c_int = 258 as libc::c_int;
#[c2rust::src_loc = "70:9"]
pub const LOPT_SECURE: libc::c_int = 259 as libc::c_int;
#[c2rust::src_loc = "71:9"]
pub const LOPT_PREFIX: libc::c_int = 260;
#[c2rust::src_loc = "72:9"]
pub const LOPT_PTR: libc::c_int = 261;
#[c2rust::src_loc = "73:9"]
pub const LOPT_BRIDGE: libc::c_int = 262 as libc::c_int;
#[c2rust::src_loc = "74:9"]
pub const LOPT_TFTP_MAX: libc::c_int = 263 as libc::c_int;
#[c2rust::src_loc = "75:9"]
pub const LOPT_FORCE: libc::c_int = 264;
#[c2rust::src_loc = "76:9"]
pub const LOPT_NOBLOCK: libc::c_int = 265 as libc::c_int;
#[c2rust::src_loc = "77:9"]
pub const LOPT_LOG_OPTS: libc::c_int = 266 as libc::c_int;
#[c2rust::src_loc = "78:9"]
pub const LOPT_MAX_LOGS: libc::c_int = 267 as libc::c_int;
#[c2rust::src_loc = "79:9"]
pub const LOPT_CIRCUIT: libc::c_int = 268;
#[c2rust::src_loc = "80:9"]
pub const LOPT_REMOTE: libc::c_int = 269;
#[c2rust::src_loc = "81:9"]
pub const LOPT_SUBSCR: libc::c_int = 270 as libc::c_int;
#[c2rust::src_loc = "82:9"]
pub const LOPT_INTNAME: libc::c_int = 271 as libc::c_int;
#[c2rust::src_loc = "83:9"]
pub const LOPT_BANK: libc::c_int = 272 as libc::c_int;
#[c2rust::src_loc = "84:9"]
pub const LOPT_DHCP_HOST: libc::c_int = 273 as libc::c_int;
#[c2rust::src_loc = "85:9"]
pub const LOPT_APREF: libc::c_int = 274 as libc::c_int;
#[c2rust::src_loc = "86:9"]
pub const LOPT_OVERRIDE: libc::c_int = 275 as libc::c_int;
#[c2rust::src_loc = "87:9"]
pub const LOPT_TFTPPORTS: libc::c_int = 276;
#[c2rust::src_loc = "88:9"]
pub const LOPT_REBIND: libc::c_int = 277 as libc::c_int;
#[c2rust::src_loc = "89:9"]
pub const LOPT_NOLAST: libc::c_int = 278 as libc::c_int;
#[c2rust::src_loc = "90:9"]
pub const LOPT_OPTS: libc::c_int = 279 as libc::c_int;
#[c2rust::src_loc = "91:9"]
pub const LOPT_DHCP_OPTS: libc::c_int = 280;
#[c2rust::src_loc = "92:9"]
pub const LOPT_MATCH: libc::c_int = 281;
#[c2rust::src_loc = "93:9"]
pub const LOPT_BROADCAST: libc::c_int = 282 as libc::c_int;
#[c2rust::src_loc = "94:9"]
pub const LOPT_NEGTTL: libc::c_int = 283 as libc::c_int;
#[c2rust::src_loc = "95:9"]
pub const LOPT_ALTPORT: libc::c_int = 284;
#[c2rust::src_loc = "96:9"]
pub const LOPT_SCRIPTUSR: libc::c_int = 285;
#[c2rust::src_loc = "97:9"]
pub const LOPT_LOCAL: libc::c_int = 286;
#[c2rust::src_loc = "98:9"]
pub const LOPT_NAPTR: libc::c_int = 287;
#[c2rust::src_loc = "99:9"]
pub const LOPT_MINPORT: libc::c_int = 288;
#[c2rust::src_loc = "100:9"]
pub const LOPT_DHCP_FQDN: libc::c_int = 289 as libc::c_int;
#[c2rust::src_loc = "101:9"]
pub const LOPT_CNAME: libc::c_int = 290 as libc::c_int;
#[c2rust::src_loc = "102:9"]
pub const LOPT_PXE_PROMT: libc::c_int = 291;
#[c2rust::src_loc = "103:9"]
pub const LOPT_PXE_SERV: libc::c_int = 292;
#[c2rust::src_loc = "104:9"]
pub const LOPT_TEST: libc::c_int = 293 as libc::c_int;
#[c2rust::src_loc = "105:9"]
pub const LOPT_TAG_IF: libc::c_int = 294 as libc::c_int;
#[c2rust::src_loc = "106:9"]
pub const LOPT_PROXY: libc::c_int = 295 as libc::c_int;
#[c2rust::src_loc = "107:9"]
pub const LOPT_GEN_NAMES: libc::c_int = 296 as libc::c_int;
#[c2rust::src_loc = "108:9"]
pub const LOPT_MAXTTL: libc::c_int = 297 as libc::c_int;
#[c2rust::src_loc = "109:9"]
pub const LOPT_NO_REBIND: libc::c_int = 298;
#[c2rust::src_loc = "110:9"]
pub const LOPT_LOC_REBND: libc::c_int = 299 as libc::c_int;
#[c2rust::src_loc = "111:9"]
pub const LOPT_ADD_MAC: libc::c_int = 300 as libc::c_int;
#[c2rust::src_loc = "112:9"]
pub const LOPT_DNSSEC: libc::c_int = 301 as libc::c_int;
#[c2rust::src_loc = "113:9"]
pub const LOPT_INCR_ADDR: libc::c_int = 302 as libc::c_int;
#[c2rust::src_loc = "114:9"]
pub const LOPT_CONNTRACK: libc::c_int = 303 as libc::c_int;
#[c2rust::src_loc = "115:9"]
pub const LOPT_FQDN: libc::c_int = 304 as libc::c_int;
#[c2rust::src_loc = "116:9"]
pub const LOPT_LUASCRIPT: libc::c_int = 305 as libc::c_int;
#[c2rust::src_loc = "117:9"]
pub const LOPT_RA: libc::c_int = 306 as libc::c_int;
#[c2rust::src_loc = "118:9"]
pub const LOPT_DUID: libc::c_int = 307 as libc::c_int;
#[c2rust::src_loc = "119:9"]
pub const LOPT_HOST_REC: libc::c_int = 308 as libc::c_int;
#[c2rust::src_loc = "120:9"]
pub const LOPT_TFTP_LC: libc::c_int = 309 as libc::c_int;
#[c2rust::src_loc = "121:9"]
pub const LOPT_RR: libc::c_int = 310 as libc::c_int;
#[c2rust::src_loc = "122:9"]
pub const LOPT_CLVERBIND: libc::c_int = 311 as libc::c_int;
#[c2rust::src_loc = "123:9"]
pub const LOPT_MAXCTTL: libc::c_int = 312 as libc::c_int;
#[c2rust::src_loc = "124:9"]
pub const LOPT_AUTHZONE: libc::c_int = 313 as libc::c_int;
#[c2rust::src_loc = "125:9"]
pub const LOPT_AUTHSERV: libc::c_int = 314 as libc::c_int;
#[c2rust::src_loc = "126:9"]
pub const LOPT_AUTHTTL: libc::c_int = 315 as libc::c_int;
#[c2rust::src_loc = "127:9"]
pub const LOPT_AUTHSOA: libc::c_int = 316 as libc::c_int;
#[c2rust::src_loc = "128:9"]
pub const LOPT_AUTHSFS: libc::c_int = 317;
#[c2rust::src_loc = "129:9"]
pub const LOPT_AUTHPEER: libc::c_int = 318 as libc::c_int;
#[c2rust::src_loc = "130:9"]
pub const LOPT_IPSET: libc::c_int = 319;
#[c2rust::src_loc = "131:9"]
pub const LOPT_SYNTH: libc::c_int = 320 as libc::c_int;
#[c2rust::src_loc = "132:9"]
pub const LOPT_RELAY: libc::c_int = 323 as libc::c_int;
#[c2rust::src_loc = "133:9"]
pub const LOPT_RA_PARAM: libc::c_int = 324;
#[c2rust::src_loc = "134:9"]
pub const LOPT_ADD_SBNET: libc::c_int = 325 as libc::c_int;
#[c2rust::src_loc = "135:9"]
pub const LOPT_QUIET_DHCP: libc::c_int = 326 as libc::c_int;
#[c2rust::src_loc = "136:9"]
pub const LOPT_QUIET_DHCP6: libc::c_int = 327 as libc::c_int;
#[c2rust::src_loc = "137:9"]
pub const LOPT_QUIET_RA: libc::c_int = 328 as libc::c_int;
#[c2rust::src_loc = "138:9"]
pub const LOPT_SEC_VALID: libc::c_int = 329 as libc::c_int;
#[c2rust::src_loc = "139:9"]
pub const LOPT_TRUST_ANCHOR: libc::c_int = 330 as libc::c_int;
#[c2rust::src_loc = "140:9"]
pub const LOPT_DNSSEC_DEBUG: libc::c_int = 331 as libc::c_int;
#[c2rust::src_loc = "141:9"]
pub const LOPT_REV_SERV: libc::c_int = 332 as libc::c_int;
#[c2rust::src_loc = "142:9"]
pub const LOPT_SERVERS_FILE: libc::c_int = 333;
#[c2rust::src_loc = "143:9"]
pub const LOPT_DNSSEC_CHECK: libc::c_int = 334 as libc::c_int;
#[c2rust::src_loc = "144:9"]
pub const LOPT_LOCAL_SERVICE: libc::c_int = 335 as libc::c_int;
#[c2rust::src_loc = "145:9"]
pub const LOPT_DNSSEC_TIME: libc::c_int = 336 as libc::c_int;
#[c2rust::src_loc = "146:9"]
pub const LOPT_LOOP_DETECT: libc::c_int = 337 as libc::c_int;
#[c2rust::src_loc = "147:9"]
pub const LOPT_IGNORE_ADDR: libc::c_int = 338;
#[c2rust::src_loc = "148:9"]
pub const LOPT_MINCTTL: libc::c_int = 339;
#[c2rust::src_loc = "149:9"]
pub const LOPT_DHCP_INOTIFY: libc::c_int = 340;
#[c2rust::src_loc = "150:9"]
pub const LOPT_DHOPT_INOTIFY: libc::c_int = 341 as libc::c_int;
#[c2rust::src_loc = "151:9"]
pub const LOPT_HOST_INOTIFY: libc::c_int = 342;
#[c2rust::src_loc = "152:9"]
pub const LOPT_DNSSEC_STAMP: libc::c_int = 343 as libc::c_int;
#[c2rust::src_loc = "153:9"]
pub const LOPT_TFTP_NO_FAIL: libc::c_int = 344 as libc::c_int;
#[c2rust::src_loc = "154:9"]
pub const LOPT_MAXPORT: libc::c_int = 345;
#[c2rust::src_loc = "155:9"]
pub const LOPT_CPE_ID: libc::c_int = 346 as libc::c_int;
#[c2rust::src_loc = "156:9"]
pub const LOPT_SCRIPT_ARP: libc::c_int = 347 as libc::c_int;
#[c2rust::src_loc = "157:9"]
pub const LOPT_DHCPTTL: libc::c_int = 348 as libc::c_int;
#[c2rust::src_loc = "158:9"]
pub const LOPT_TFTP_MTU: libc::c_int = 349 as libc::c_int;
#[c2rust::src_loc = "159:9"]
pub const LOPT_REPLY_DELAY: libc::c_int = 350 as libc::c_int;
#[c2rust::src_loc = "160:9"]
pub const LOPT_RAPID_COMMIT: libc::c_int = 351 as libc::c_int;
#[c2rust::src_loc = "161:9"]
pub const LOPT_DUMPFILE: libc::c_int = 352 as libc::c_int;
#[c2rust::src_loc = "162:9"]
pub const LOPT_DUMPMASK: libc::c_int = 353 as libc::c_int;
#[c2rust::src_loc = "163:9"]
pub const LOPT_UBUS: libc::c_int = 354 as libc::c_int;
#[c2rust::src_loc = "164:9"]
pub const LOPT_NAME_MATCH: libc::c_int = 355 as libc::c_int;
#[c2rust::src_loc = "165:9"]
pub const LOPT_CAA: libc::c_int = 356 as libc::c_int;
#[c2rust::src_loc = "166:9"]
pub const LOPT_SHARED_NET: libc::c_int = 357 as libc::c_int;
#[c2rust::src_loc = "167:9"]
pub const LOPT_IGNORE_CLID: libc::c_int = 358 as libc::c_int;
#[c2rust::src_loc = "168:9"]
pub const LOPT_SINGLE_PORT: libc::c_int = 359 as libc::c_int;
#[c2rust::src_loc = "169:9"]
pub const LOPT_SCRIPT_TIME: libc::c_int = 360 as libc::c_int;
#[c2rust::src_loc = "170:9"]
pub const LOPT_PXE_VENDOR: libc::c_int = 361;
#[c2rust::src_loc = "173:28"]
static mut opts: [option; 167] =
    [{
         let mut init =
             option{name: b"version\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'v' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"no-hosts\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'h' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"no-poll\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'n' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"help\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'w' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"no-daemon\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'd' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"log-queries\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'q' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"user\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'u' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"group\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'g' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"resolv-file\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'r' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"servers-file\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SERVERS_FILE,};
         init
     },
     {
         let mut init =
             option{name: b"mx-host\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'm' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"mx-target\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 't' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"cache-size\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'c' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"port\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'p' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-leasefile\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'l' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-lease\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'l' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-host\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'G' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-range\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'F' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-option\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'O' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-boot\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'M' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"domain\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 's' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"domain-suffix\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 's' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"interface\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'i' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"listen-address\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'a' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"local-service\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_LOCAL_SERVICE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bogus-priv\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'b' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bogus-nxdomain\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'B' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"ignore-address\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_IGNORE_ADDR,};
         init
     },
     {
         let mut init =
             option{name: b"selfmx\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'e' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"filterwin2k\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'f' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"pid-file\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'x' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"strict-order\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'o' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"server\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'S' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"rev-server\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_REV_SERV,};
         init
     },
     {
         let mut init =
             option{name: b"local\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_LOCAL,};
         init
     },
     {
         let mut init =
             option{name: b"address\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'A' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"conf-file\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'C' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"no-resolv\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'R' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"expand-hosts\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'E' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"localmx\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'L' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"local-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'T' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"no-negcache\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'N' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"addn-hosts\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'H' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"hostsdir\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_HOST_INOTIFY,};
         init
     },
     {
         let mut init =
             option{name:
                        b"query-port\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'Q' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"except-interface\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'I' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"no-dhcp-interface\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '2' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"domain-needed\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'D' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-lease-max\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'X' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bind-interfaces\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'z' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"read-ethers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'Z' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"alias\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'V' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-vendorclass\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'U' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-userclass\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'j' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-ignore\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'J' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"edns-packet-max\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'P' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"keep-in-foreground\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'k' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-authoritative\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'K' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"srv-host\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'W' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"localise-queries\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'y' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"txt-record\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'Y' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"caa-record\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_CAA,};
         init
     },
     {
         let mut init =
             option{name: b"dns-rr\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_RR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"enable-dbus\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '1' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"enable-ubus\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_UBUS,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bootp-dynamic\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '3' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"dhcp-mac\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '4' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"no-ping\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '5' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-script\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '6' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"conf-dir\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '7' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"log-facility\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '8' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"leasefile-ro\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '9' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"script-on-renewal\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SCRIPT_TIME,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dns-forward-max\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '0' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"clear-on-reload\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_RELOAD,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-ignore-names\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NO_NAMES,};
         init
     },
     {
         let mut init =
             option{name:
                        b"enable-tftp\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TFTP,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-secure\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SECURE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-no-fail\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TFTP_NO_FAIL,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-unique-root\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_APREF,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-root\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_PREFIX,};
         init
     },
     {
         let mut init =
             option{name: b"tftp-max\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TFTP_MAX,};
         init
     },
     {
         let mut init =
             option{name: b"tftp-mtu\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TFTP_MTU,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-lowercase\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TFTP_LC,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-single-port\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SINGLE_PORT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"ptr-record\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_PTR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"naptr-record\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NAPTR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bridge-interface\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_BRIDGE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"shared-network\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SHARED_NET,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-option-force\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_FORCE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-no-blocksize\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NOBLOCK,};
         init
     },
     {
         let mut init =
             option{name: b"log-dhcp\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_LOG_OPTS,};
         init
     },
     {
         let mut init =
             option{name:
                        b"log-async\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MAX_LOGS,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-circuitid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_CIRCUIT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-remoteid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_REMOTE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-subscrid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SUBSCR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-pxe-vendor\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_PXE_VENDOR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"interface-name\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_INTNAME,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-hostsfile\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DHCP_HOST,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-optsfile\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DHCP_OPTS,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-hostsdir\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DHCP_INOTIFY,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-optsdir\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DHOPT_INOTIFY,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-no-override\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_OVERRIDE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-port-range\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TFTPPORTS,};
         init
     },
     {
         let mut init =
             option{name:
                        b"stop-dns-rebind\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_REBIND,};
         init
     },
     {
         let mut init =
             option{name:
                        b"rebind-domain-ok\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NO_REBIND,};
         init
     },
     {
         let mut init =
             option{name:
                        b"all-servers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NOLAST,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-match\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MATCH,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-name-match\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NAME_MATCH,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-broadcast\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_BROADCAST,};
         init
     },
     {
         let mut init =
             option{name: b"neg-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_NEGTTL,};
         init
     },
     {
         let mut init =
             option{name: b"max-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MAXTTL,};
         init
     },
     {
         let mut init =
             option{name:
                        b"min-cache-ttl\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MINCTTL,};
         init
     },
     {
         let mut init =
             option{name:
                        b"max-cache-ttl\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MAXCTTL,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-alternate-port\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_ALTPORT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-scriptuser\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SCRIPTUSR,};
         init
     },
     {
         let mut init =
             option{name: b"min-port\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MINPORT,};
         init
     },
     {
         let mut init =
             option{name: b"max-port\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_MAXPORT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-fqdn\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DHCP_FQDN,};
         init
     },
     {
         let mut init =
             option{name: b"cname\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_CNAME,};
         init
     },
     {
         let mut init =
             option{name:
                        b"pxe-prompt\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_PXE_PROMT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"pxe-service\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_PXE_SERV,};
         init
     },
     {
         let mut init =
             option{name: b"test\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TEST,};
         init
     },
     {
         let mut init =
             option{name: b"tag-if\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TAG_IF,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-proxy\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_PROXY,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-generate-names\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_GEN_NAMES,};
         init
     },
     {
         let mut init =
             option{name:
                        b"rebind-localhost-ok\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_LOC_REBND,};
         init
     },
     {
         let mut init =
             option{name: b"add-mac\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_ADD_MAC,};
         init
     },
     {
         let mut init =
             option{name:
                        b"add-subnet\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_ADD_SBNET,};
         init
     },
     {
         let mut init =
             option{name:
                        b"add-cpe-id\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_CPE_ID,};
         init
     },
     {
         let mut init =
             option{name:
                        b"proxy-dnssec\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DNSSEC,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-sequential-ip\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_INCR_ADDR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"conntrack\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_CONNTRACK,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-client-update\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_FQDN,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-luascript\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_LUASCRIPT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"enable-ra\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_RA,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-duid\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DUID,};
         init
     },
     {
         let mut init =
             option{name:
                        b"host-record\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_HOST_REC,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bind-dynamic\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_CLVERBIND,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-zone\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_AUTHZONE,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-server\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_AUTHSERV,};
         init
     },
     {
         let mut init =
             option{name: b"auth-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_AUTHTTL,};
         init
     },
     {
         let mut init =
             option{name: b"auth-soa\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_AUTHSOA,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-sec-servers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_AUTHSFS,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-peer\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_AUTHPEER,};
         init
     },
     {
         let mut init =
             option{name: b"ipset\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_IPSET,};
         init
     },
     {
         let mut init =
             option{name:
                        b"synth-domain\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SYNTH,};
         init
     },
     {
         let mut init =
             option{name: b"dnssec\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SEC_VALID,};
         init
     },
     {
         let mut init =
             option{name:
                        b"trust-anchor\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_TRUST_ANCHOR,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-debug\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DNSSEC_DEBUG,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-check-unsigned\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DNSSEC_CHECK,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-no-timecheck\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DNSSEC_TIME,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-timestamp\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DNSSEC_STAMP,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-relay\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_RELAY,};
         init
     },
     {
         let mut init =
             option{name: b"ra-param\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_RA_PARAM,};
         init
     },
     {
         let mut init =
             option{name:
                        b"quiet-dhcp\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_QUIET_DHCP,};
         init
     },
     {
         let mut init =
             option{name:
                        b"quiet-dhcp6\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_QUIET_DHCP6,};
         init
     },
     {
         let mut init =
             option{name: b"quiet-ra\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_QUIET_RA,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dns-loop-detect\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_LOOP_DETECT,};
         init
     },
     {
         let mut init =
             option{name:
                        b"script-arp\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_SCRIPT_ARP,};
         init
     },
     {
         let mut init =
             option{name: b"dhcp-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DHCPTTL,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-reply-delay\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_REPLY_DELAY,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-rapid-commit\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_RAPID_COMMIT,};
         init
     },
     {
         let mut init =
             option{name: b"dumpfile\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DUMPFILE,};
         init
     },
     {
         let mut init =
             option{name: b"dumpmask\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_DUMPMASK,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-ignore-clid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: LOPT_IGNORE_CLID,};
         init
     },
     {
         let mut init =
             option{name: NULL_0 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "348:9"]
pub const ARG_DUP: libc::c_int = 62 as libc::c_int;
#[c2rust::src_loc = "359:3"]
static mut usage: [C2RustUnnamed_9; 165] =
    [{
         let mut init =
             C2RustUnnamed_9{opt: 'a' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify local address(es) to listen on.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'A' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Return ipaddr for all hosts in specified domains.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'b' as i32,
                             rept: OPT_BOGUSPRIV as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Fake reverse lookups for RFC1918 private address ranges.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'B' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Treat ipaddr as NXDOMAIN (defeats Verisign wildcard).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'c' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify the size of the cache in entries (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"$\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'C' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify configuration file (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: CONFFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'd' as i32,
                             rept: OPT_DEBUG as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT fork into the background: run in debug mode.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'D' as i32,
                             rept: OPT_NODOTS_LOCAL as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT forward queries with no domain part.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'e' as i32,
                             rept: OPT_SELFMX as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Return self-pointing MX records for local hosts.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'E' as i32,
                             rept: OPT_EXPAND as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Expand simple names in /etc/hosts with domain-suffix.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'f' as i32,
                             rept: OPT_FILTER as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Don\'t forward spurious DNS requests from Windows hosts.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'F' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>,...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable DHCP in the range given with lease duration.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'g' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<groupname>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Change to this group after startup (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: CHGRP.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'G' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<hostspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set address or hostname for a specified machine.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DHCP_HOST,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP host specs from file.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DHCP_OPTS,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP option specs from file.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DHCP_INOTIFY,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP host specs from a directory.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DHOPT_INOTIFY,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP options from a directory.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TAG_IF,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"tag-expression\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Evaluate conditional tag expression.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'h' as i32,
                             rept: OPT_NO_HOSTS as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT load %s file.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg: HOSTSFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'H' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify a hosts file to be read in addition to %s.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: HOSTSFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_HOST_INOTIFY,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read hosts files from a directory.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'i' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify interface(s) to listen on.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'I' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify interface(s) NOT to listen on.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'j' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<class>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map DHCP user class to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_CIRCUIT,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<circuit>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map RFC3046 circuit-id to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_REMOTE,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<remote>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map RFC3046 remote-id to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SUBSCR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<remote>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map RFC3993 subscriber-id to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_PXE_VENDOR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<vendor>[,...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify vendor class to match for PXE requests.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'J' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"tag:<tag>...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Don\'t do DHCP for hosts with tag set.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_BROADCAST,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Force broadcast replies for hosts with tag set.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'k' as i32,
                             rept: OPT_NO_FORK as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT fork into the background, do NOT run in debug mode.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'K' as i32,
                             rept: OPT_AUTHORITATIVE as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Assume we are the only DHCP server on the local network.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'l' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify where to store DHCP leases (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: LEASEFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'L' as i32,
                             rept: OPT_LOCALMX as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Return MX records for local hosts.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'm' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<host_name>,<target>,<pref>\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify an MX record.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'M' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<bootp opts>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify BOOTP options to DHCP server.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'n' as i32,
                             rept: OPT_NO_POLL as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT poll %s file, reload only on SIGHUP.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: RESOLVFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'N' as i32,
                             rept: OPT_NO_NEG as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT cache failed search results.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'o' as i32,
                             rept: OPT_ORDER as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Use nameservers strictly in the order given in %s.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: RESOLVFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'O' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<optspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify options to be sent to DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_FORCE,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<optspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"DHCP option sent even if the client does not request it.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'p' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify port to listen for DNS requests on (defaults to 53).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'P' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Maximum supported UDP packet size for EDNS.0 (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"*\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'q' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Log DNS queries.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'Q' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Force the originating port for upstream DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'R' as i32,
                             rept: OPT_NO_RESOLV as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT read resolv.conf.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'r' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify path to resolv.conf (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: RESOLVFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SERVERS_FILE,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify path to file with server= options\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'S' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify address(es) of upstream servers with optional domains.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_REV_SERV,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<addr>/<prefix>,<ipaddr>\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify address of upstream servers for reverse address queries\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_LOCAL,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Never forward queries to specified domains.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 's' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<domain>[,<range>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify the domain to be assigned in DHCP leases.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 't' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<host_name>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify default target in an MX record.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'T' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live in seconds for replies from /etc/hosts.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NEGTTL,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live in seconds for negative caching.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MAXTTL,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live in seconds for maximum TTL to send to clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MAXCTTL,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live ceiling for cache.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MINCTTL,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live floor for cache.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'u' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<username>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Change to this user after startup. (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: CHUSER.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'U' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<class>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map DHCP vendor class to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'v' as i32,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Display dnsmasq version and copyright information.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'V' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>,<ipaddr>,<netmask>\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Translate IPv4 addresses from upstream servers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'W' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<target>,...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify a SRV record.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'w' as i32,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Display this message. Use --help dhcp or --help dhcp6 for known DHCP options.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'x' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify path of PID file (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: RUNFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'X' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify maximum number of DHCP leases (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"&\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'y' as i32,
                             rept: OPT_LOCALISE as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Answer DNS queries based on the interface a query was sent to.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'Y' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<txt>[,<txt]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify TXT DNS record.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_PTR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<target>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify PTR DNS record.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_INTNAME,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Give DNS name to IPv4 address of interface.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'z' as i32,
                             rept: OPT_NOWILD as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Bind only to interfaces in use.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'Z' as i32,
                             rept: OPT_ETHERS as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Read DHCP static host information from %s.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: ETHERSFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '1' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<busname>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable the DBus interface for setting upstream servers, etc.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_UBUS,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<busname>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable the UBus interface.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '2' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Do not provide DHCP on this interface, only provide DNS.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '3' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>]...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable dynamic address allocation for bootp.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '4' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<mac address>\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Map MAC address (with wildcards) to option set.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_BRIDGE,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<iface>,<alias>..\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Treat DHCP requests on aliases as arriving from interface.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SHARED_NET,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<iface>|<addr>,<addr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify extra networks sharing a broadcast domain for DHCP\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '5' as i32,
                             rept: OPT_NO_PING as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Disable ICMP echo address checking in the DHCP server.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '6' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Shell script to run on DHCP lease creation and destruction.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_LUASCRIPT,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"path\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Lua script to run on DHCP lease creation and destruction.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SCRIPTUSR,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<username>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Run lease-change scripts as this user.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SCRIPT_ARP,
                             rept: OPT_SCRIPT_ARP as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Call dhcp-script with changes to local ARP table.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '7' as i32,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read configuration from all the files in this directory.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '8' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<facility>|<file>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Log to this syslog facility or file. (defaults to DAEMON)\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '9' as i32,
                             rept: OPT_LEASE_RO as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do not use leasefile.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '0' as i32,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Maximum number of concurrent DNS queries. (defaults to %s)\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"!\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_RELOAD,
                             rept: OPT_RELOAD as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Clear DNS cache when reloading %s.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: RESOLVFILE.as_ptr() as *mut _,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NO_NAMES,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>]...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Ignore hostnames provided by DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_OVERRIDE,
                             rept: OPT_NO_OVERRIDE as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do NOT reuse filename and server fields for extra DHCP options.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TFTP,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=<intr>[,<intr>]]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable integrated read-only TFTP server.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_PREFIX,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<dir>[,<iface>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Export files by TFTP only from the specified subtree.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_APREF,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=ip|mac]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add client IP or hardware address to tftp-root.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SECURE,
                             rept: OPT_TFTP_SECURE as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Allow access only to files owned by the user running dnsmasq.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TFTP_NO_FAIL,
                             rept: OPT_TFTP_NO_FAIL as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do not terminate the service if TFTP directories are inaccessible.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TFTP_MAX,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Maximum number of concurrent TFTP transfers (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"#\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TFTP_MTU,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Maximum MTU to use for TFTP transfers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NOBLOCK,
                             rept: OPT_TFTP_NOBLOCK as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Disable the TFTP blocksize extension.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TFTP_LC,
                             rept: OPT_TFTP_LC as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Convert TFTP filenames to lowercase\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TFTPPORTS,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<start>,<end>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Ephemeral port range for use by TFTP transfers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SINGLE_PORT,
                             rept: OPT_SINGLE_PORT as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Use only one port for TFTP server.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_LOG_OPTS,
                             rept: OPT_LOG_OPTS as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Extra logging for DHCP.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MAX_LOGS,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<integer>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable async. logging; optionally set queue length.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_REBIND,
                             rept: OPT_NO_REBIND as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Stop DNS rebinding. Filter private IP ranges when resolving.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_LOC_REBND,
                             rept: OPT_LOCAL_REBIND as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Allow rebinding of 127.0.0.0/8, for RBL servers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NO_REBIND,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Inhibit DNS-rebind protection on this domain.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NOLAST,
                             rept: OPT_ALL_SERVERS as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Always perform DNS queries to all servers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MATCH,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<optspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set tag if client includes matching option in request.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NAME_MATCH,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<string>[*]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set tag if client provides given name.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_ALTPORT,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<ports>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Use alternative ports for DHCP.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_NAPTR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<naptr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify NAPTR DNS record.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MINPORT,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<port>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify lowest port available for DNS query transmission.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_MAXPORT,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<port>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify highest port available for DNS query transmission.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DHCP_FQDN,
                             rept: OPT_DHCP_FQDN as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Use only fully qualified domain names for DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_GEN_NAMES,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Generate hostnames based on MAC address for nameless clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_PROXY,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=<ipaddr>]...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Use these DHCP relays as full proxies.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_RELAY,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<local-addr>,<server>[,<iface>]\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Relay DHCP requests to a remote server\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_CNAME,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<alias>,<target>[,<ttl>]\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify alias name for LOCAL DNS name.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_PXE_PROMT,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<prompt>,[<timeout>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Prompt to send to PXE clients.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_PXE_SERV,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<service>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Boot service for PXE menu.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TEST,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Check configuration syntax.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_ADD_MAC,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"[=base64|text]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add requestor\'s MAC address to forwarded DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_ADD_SBNET,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<v4 pref>[,<v6 pref>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add specified IP subnet to forwarded DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_CPE_ID,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<text>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add client identification to forwarded DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DNSSEC,
                             rept: OPT_DNSSEC_PROXY as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Proxy DNSSEC validation results from upstream nameservers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_INCR_ADDR,
                             rept: OPT_CONSEC_ADDR as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Attempt to allocate sequential IP addresses to DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_IGNORE_CLID,
                             rept: OPT_IGNORE_CLID as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Ignore client identifier option sent by DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_CONNTRACK,
                             rept: OPT_CONNTRACK as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Copy connection-track mark from queries to upstream connections.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_FQDN,
                             rept: OPT_FQDN_UPDATE as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Allow DHCP clients to do their own DDNS updates.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_RA,
                             rept: OPT_RA as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Send router-advertisements for interfaces doing DHCPv6\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DUID,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<enterprise>,<duid>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify DUID_EN-type DHCPv6 server DUID\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_HOST_REC,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<address>[,<ttl>]\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify host (A/AAAA and PTR) records\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_CAA,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<flags>,<tag>,<value>\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify certification authority authorization record\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_RR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<name>,<RR-number>,[<data>]\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify arbitrary DNS resource record\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_CLVERBIND,
                             rept: OPT_CLEVERBIND as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Bind to interfaces in use - check for new interfaces\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_AUTHSERV,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<NS>,<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Export local names to global DNS\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_AUTHZONE,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<domain>,[<subnet>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Domain to export to global DNS\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_AUTHTTL,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set TTL for authoritative replies\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_AUTHSOA,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<serial>[,...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set authoritative zone information\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_AUTHSFS,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<NS>[,<NS>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Secondary authoritative nameservers for forward domains\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_AUTHPEER,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>[,<ipaddr>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Peers which are allowed to do zone transfer\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_IPSET,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"/<domain>[/<domain>...]/<ipset>...\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify ipsets to which matching domains should be added\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SYNTH,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<domain>,<range>,[<prefix>]\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify a domain and address range for synthesised names\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SEC_VALID,
                             rept: OPT_DNSSEC_VALID as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Activate DNSSEC validation\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_TRUST_ANCHOR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<domain>,[<class>],...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify trust anchor key digest.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DNSSEC_DEBUG,
                             rept: OPT_DNSSEC_DEBUG as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Disable upstream checking for DNSSEC debugging.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DNSSEC_CHECK,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Ensure answers without DNSSEC are in unsigned zones.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DNSSEC_TIME,
                             rept: OPT_DNSSEC_TIME as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Don\'t check DNSSEC signature timestamps until first cache-reload\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DNSSEC_STAMP,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Timestamp file to verify system clock for DNSSEC\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_RA_PARAM,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<iface>,[mtu:<value>|<interface>|off,][<prio>,]<intval>[,<lifetime>]\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Set MTU, priority, resend-interval and router-lifetime\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_QUIET_DHCP,
                             rept: OPT_QUIET_DHCP as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do not log routine DHCP.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_QUIET_DHCP6,
                             rept: OPT_QUIET_DHCP6 as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do not log routine DHCPv6.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_QUIET_RA,
                             rept: OPT_QUIET_RA as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Do not log RA.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_LOCAL_SERVICE,
                             rept: OPT_LOCAL_SERVICE as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Accept queries only from directly-connected networks.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_LOOP_DETECT,
                             rept: OPT_LOOP_DETECT as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Detect and remove DNS forwarding loops.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_IGNORE_ADDR,
                             rept: ARG_DUP as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Ignore DNS responses containing ipaddr.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DHCPTTL,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<ttl>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set TTL in DNS responses with DHCP-derived addresses.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_REPLY_DELAY,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Delay DHCP replies for at least number of seconds.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_RAPID_COMMIT,
                             rept: OPT_RAPID_COMMIT as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Enables DHCPv4 Rapid Commit option.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DUMPFILE,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Path to debug packet dump file\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_DUMPMASK,
                             rept:
                                 (OPT_LAST + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<hex>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Mask which packets to dump\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: LOPT_SCRIPT_TIME,
                             rept: OPT_LEASE_RENEW as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc:
                                 b"Call dhcp-script when lease expiry changes.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 0 as libc::c_int,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc: NULL_0 as *mut libc::c_char,
                             desc: NULL_0 as *mut libc::c_char,
                             arg: NULL_0 as *mut libc::c_char,};
         init
     }];
/* We hide metacharacters in quoted strings by mapping them into the ASCII control
   character space. Note that the \0, \t \b \r \033 and \n characters are carefully placed in the
   following sequence so that they map to themselves: it is therefore possible to call
   unhide_metas repeatedly on string without breaking things.
   The transformation gets undone by opt_canonicalise, atoi_check and opt_string_alloc, and a 
   couple of other places. 
   Note that space is included here so that
   --dhcp-option=3, string
   has five characters, whilst
   --dhcp-option=3," string"
   has six.
*/
#[c2rust::src_loc = "540:19"]
static mut meta: [libc::c_char; 33] =
    unsafe {
        *::std::mem::transmute::<&[u8; 33],
                                 &[libc::c_char; 33]>(b"\x00123456 \x08\t\n78\r90abcdefABCDE\x1bF:,.\x00")
    };
#[c2rust::src_loc = "542:1"]
unsafe extern "C" fn hide_meta(mut c: libc::c_char) -> libc::c_char {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[libc::c_char; 33]>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) {
        if c as libc::c_int == meta[i as usize] as libc::c_int {
            return i as libc::c_char
        }
        i = i.wrapping_add(1)
    }
    return c;
}
#[c2rust::src_loc = "553:1"]
unsafe extern "C" fn unhide_meta(mut cr: libc::c_char) -> libc::c_char {
    let mut c = cr as libc::c_uint;
    if (c as libc::c_ulong) <
           (::std::mem::size_of::<[libc::c_char; 33]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        cr = meta[c as usize]
    }
    return cr;
}
#[c2rust::src_loc = "563:1"]
unsafe extern "C" fn unhide_metas(mut cp: *mut libc::c_char) {
    if !cp.is_null() {
        while *cp != 0 { *cp = unhide_meta(*cp); cp = cp.offset(1) }
    };
}
#[c2rust::src_loc = "570:1"]
unsafe extern "C" fn opt_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret = 0 as *mut libc::c_void;
    if mem_recover != 0 {
        ret = whine_malloc(size);
        if ret.is_null() { longjmp(mem_jmp.as_mut_ptr(), 1 as libc::c_int); }
    } else { ret = safe_malloc(size) }
    return ret;
}
#[c2rust::src_loc = "586:1"]
unsafe extern "C" fn opt_string_alloc(mut cp: *const libc::c_char)
 -> *mut libc::c_char {
    let mut ret = NULL_0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if !cp.is_null() &&
           { len = strlen(cp); (len) != 0 as libc::c_int as libc::c_ulong } {
        ret =
            opt_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
                *mut libc::c_char;
        memcpy(ret as *mut libc::c_void, cp as *const libc::c_void,
               len.wrapping_add(1 as libc::c_int as libc::c_ulong));
        /* restore hidden metachars */
        unhide_metas(ret);
    }
    return ret;
}
/* find next comma, split string with zero and eliminate spaces.
   return start of string following comma */
#[c2rust::src_loc = "607:1"]
unsafe extern "C" fn split_chr(mut s: *mut libc::c_char, mut c: libc::c_char)
 -> *mut libc::c_char {
    let mut comma =
        0 as *mut libc::c_char; /* strlen("xxx.yyy.zzz.ttt.in-addr.arpa")+1 */
    let mut p = 0 as *mut libc::c_char;
    if s.is_null() || { comma = strchr(s, c as libc::c_int); comma.is_null() }
       {
        return NULL_0 as *mut libc::c_char
    }
    p = comma;
    *comma = ' ' as i32 as libc::c_char;
    while *comma as libc::c_int == ' ' as i32 { comma = comma.offset(1) }
    while p >= s && *p as libc::c_int == ' ' as i32 {
        *p = 0 as libc::c_int as libc::c_char;
        p = p.offset(-1)
    }
    return comma;
}
#[c2rust::src_loc = "625:1"]
unsafe extern "C" fn split(mut s: *mut libc::c_char) -> *mut libc::c_char {
    return split_chr(s, ',' as i32 as libc::c_char);
}
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn canonicalise_opt(mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut ret = 0 as *mut libc::c_char;
    let mut nomem: libc::c_int = 0;
    if s.is_null() { return 0 as *mut libc::c_char }
    unhide_metas(s);
    ret = canonicalise(s, &mut nomem);
    if ret.is_null() && nomem != 0 {
        if mem_recover != 0 {
            longjmp(mem_jmp.as_mut_ptr(), 1 as libc::c_int);
        } else {
            die(b"could not get memory\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                NULL_0 as *mut libc::c_char, EC_NOMEM);
        }
    }
    return ret;
}
#[c2rust::src_loc = "650:1"]
unsafe extern "C" fn atoi_check(mut a: *mut libc::c_char,
                                mut res: *mut libc::c_int) -> libc::c_int {
    let mut p = 0 as *mut libc::c_char;
    if a.is_null() { return 0 as libc::c_int }
    unhide_metas(a);
    p = a;
    while *p != 0 {
        if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32
           {
            return 0 as libc::c_int
        }
        p = p.offset(1)
    }
    *res = atoi(a);
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "667:1"]
unsafe extern "C" fn atoi_check16(mut a: *mut libc::c_char,
                                  mut res: *mut libc::c_int) -> libc::c_int {
    if atoi_check(a, res) == 0 || *res < 0 as libc::c_int ||
           *res > 0xffff as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "690:1"]
unsafe extern "C" fn add_txt(mut name: *mut libc::c_char,
                             mut txt: *mut libc::c_char,
                             mut stat_0: libc::c_int) {
    let mut r =
        opt_malloc(::std::mem::size_of::<txt_record>() as libc::c_ulong) as
            *mut txt_record;
    if !txt.is_null() {
        let mut len = strlen(txt);
        (*r).txt =
            opt_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
                *mut libc::c_uchar;
        (*r).len =
            len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                libc::c_ushort;
        *(*r).txt = len as libc::c_uchar;
        memcpy((*r).txt.offset(1 as libc::c_int as isize) as
                   *mut libc::c_void, txt as *const libc::c_void, len);
    }
    (*r).stat = stat_0;
    (*r).name = opt_string_alloc(name);
    (*r).next = (*dnsmasq_daemon).txt;
    (*dnsmasq_daemon).txt = r;
    (*r).class = C_CHAOS as libc::c_ushort;
}
#[c2rust::src_loc = "711:1"]
unsafe extern "C" fn do_usage() {
    let mut buff: [libc::c_char; 100] = [0; 100];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tab: [C2RustUnnamed_10; 6] =
        [{
             let mut init =
                 C2RustUnnamed_10{handle: '$' as i32 as libc::c_char,
                                  val: CACHESIZ,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '*' as i32 as libc::c_char,
                                  val: EDNS_PKTSZ,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '&' as i32 as libc::c_char,
                                  val: MAXLEASES,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '!' as i32 as libc::c_char,
                                  val: FTABSIZ,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '#' as i32 as libc::c_char,
                                  val: TFTP_MAX_CONNECTIONS,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '\u{0}' as i32 as libc::c_char,
                                  val: 0 as libc::c_int,};
             init
         }];
    printf(b"Usage: dnsmasq [options]\n\n\x00" as *const u8 as
               *const libc::c_char);
    printf(b"Valid options are:\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while usage[i as usize].opt != 0 as libc::c_int {
        let mut desc = usage[i as usize].flagdesc;
        let mut eq =
            b"=\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if desc.is_null() || *desc as libc::c_int == '[' as i32 {
            eq =
                b"\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        if desc.is_null() {
            desc =
                b"\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        j = 0 as libc::c_int;
        while !opts[j as usize].name.is_null() {
            if opts[j as usize].val == usage[i as usize].opt { break ; }
            j += 1
        }
        if usage[i as usize].opt < 256 as libc::c_int {
            sprintf(buff.as_mut_ptr(),
                    b"-%c, \x00" as *const u8 as *const libc::c_char,
                    usage[i as usize].opt);
        } else {
            sprintf(buff.as_mut_ptr(),
                    b"    \x00" as *const u8 as *const libc::c_char);
        }
        sprintf(buff.as_mut_ptr().offset(4 as libc::c_int as isize),
                b"--%s%s%s\x00" as *const u8 as *const libc::c_char,
                opts[j as usize].name, eq, desc);
        printf(b"%-55.55s\x00" as *const u8 as *const libc::c_char,
               buff.as_mut_ptr());
        if !usage[i as usize].arg.is_null() {
            strcpy(buff.as_mut_ptr(), usage[i as usize].arg);
            j = 0 as libc::c_int;
            while tab[j as usize].handle != 0 {
                if tab[j as usize].handle as libc::c_int ==
                       *usage[i as usize].arg as libc::c_int {
                    sprintf(buff.as_mut_ptr(),
                            b"%d\x00" as *const u8 as *const libc::c_char,
                            tab[j as usize].val);
                }
                j += 1
            }
        }
        printf(usage[i as usize].desc, buff.as_mut_ptr());
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    };
}
#[c2rust::src_loc = "772:1"]
unsafe extern "C" fn parse_mysockaddr(mut arg: *mut libc::c_char,
                                      mut addr: *mut mysockaddr)
 -> *mut libc::c_char {
    if inet_pton(AF_INET, arg,
                 &mut (*addr).in_0.sin_addr as *mut in_addr as
                     *mut libc::c_void) > 0 as libc::c_int {
        (*addr).sa.sa_family = AF_INET as sa_family_t
    } else if inet_pton(AF_INET6, arg,
                        &mut (*addr).in6.sin6_addr as *mut in6_addr as
                            *mut libc::c_void) > 0 as libc::c_int {
        (*addr).sa.sa_family = AF_INET6 as sa_family_t
    } else {
        return b"bad address\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return NULL_0 as *mut libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "784:1"]
pub unsafe extern "C" fn parse_server(mut arg: *mut libc::c_char,
                                      mut addr: *mut mysockaddr,
                                      mut source_addr: *mut mysockaddr,
                                      mut interface: *mut libc::c_char,
                                      mut flags: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut source_port = 0 as libc::c_int;
    let mut serv_port = NAMESERVER_PORT;
    let mut portno = 0 as *mut libc::c_char;
    let mut source = 0 as *mut libc::c_char;
    let mut interface_opt = NULL_0 as *mut libc::c_char;
    let mut scope_index = 0 as libc::c_int;
    let mut scope_id = 0 as *mut libc::c_char;
    if arg.is_null() || strlen(arg) == 0 as libc::c_int as libc::c_ulong {
        *flags |= SERV_NO_ADDR;
        *interface = 0 as libc::c_int as libc::c_char;
        return NULL_0 as *mut libc::c_char
    }
    source = split_chr(arg, '@' as i32 as libc::c_char);
    if !source.is_null() &&
           {
               portno = split_chr(source, '#' as i32 as libc::c_char);
               !portno.is_null()
           } && atoi_check16(portno, &mut source_port) == 0 {
        return b"bad port\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    portno = split_chr(arg, '#' as i32 as libc::c_char);
    if !portno.is_null() && atoi_check16(portno, &mut serv_port) == 0 {
        return b"bad port\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    scope_id = split_chr(arg, '%' as i32 as libc::c_char);
    if !source.is_null() {
        interface_opt = split_chr(source, '@' as i32 as libc::c_char);
        if !interface_opt.is_null() {
            safe_strncpy(interface, interface_opt, IF_NAMESIZE as size_t);
        }
    }
    if inet_pton(AF_INET, arg,
                 &mut (*addr).in_0.sin_addr as *mut in_addr as
                     *mut libc::c_void) > 0 as libc::c_int {
        (*addr).in_0.sin_port = __bswap_16(serv_port as __uint16_t);
        (*source_addr).sa.sa_family = AF_INET as sa_family_t;
        (*addr).sa.sa_family = (*source_addr).sa.sa_family;
        (*source_addr).in_0.sin_addr.s_addr = INADDR_ANY as in_addr_t;
        (*source_addr).in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        if !source.is_null() {
            if !flags.is_null() { *flags |= SERV_HAS_SOURCE }
            (*source_addr).in_0.sin_port =
                __bswap_16(source_port as __uint16_t);
            if !(inet_pton(AF_INET, source,
                           &mut (*source_addr).in_0.sin_addr as *mut in_addr
                               as *mut libc::c_void) > 0 as libc::c_int) {
                if !interface_opt.is_null() {
                    return b"interface can only be specified once\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char
                }
                (*source_addr).in_0.sin_addr.s_addr = INADDR_ANY as in_addr_t;
                safe_strncpy(interface, source, IF_NAMESIZE as size_t);
            }
        }
    } else if inet_pton(AF_INET6, arg,
                        &mut (*addr).in6.sin6_addr as *mut in6_addr as
                            *mut libc::c_void) > 0 as libc::c_int {
        if !scope_id.is_null() &&
               {
                   scope_index = if_nametoindex(scope_id) as libc::c_int;
                   (scope_index) == 0 as libc::c_int
               } {
            return b"bad interface name\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        (*addr).in6.sin6_port = __bswap_16(serv_port as __uint16_t);
        (*addr).in6.sin6_scope_id = scope_index as uint32_t;
        (*source_addr).in6.sin6_addr = in6addr_any;
        (*source_addr).in6.sin6_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        (*source_addr).in6.sin6_scope_id = 0 as libc::c_int as uint32_t;
        (*source_addr).sa.sa_family = AF_INET6 as sa_family_t;
        (*addr).sa.sa_family = (*source_addr).sa.sa_family;
        (*source_addr).in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
        (*addr).in6.sin6_flowinfo = (*source_addr).in6.sin6_flowinfo;
        if !source.is_null() {
            if !flags.is_null() { *flags |= SERV_HAS_SOURCE }
            (*source_addr).in6.sin6_port =
                __bswap_16(source_port as __uint16_t);
            if inet_pton(AF_INET6, source,
                         &mut (*source_addr).in6.sin6_addr as *mut in6_addr as
                             *mut libc::c_void) == 0 as libc::c_int {
                if !interface_opt.is_null() {
                    return b"interface can only be specified once\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char
                }
                (*source_addr).in6.sin6_addr = in6addr_any;
                safe_strncpy(interface, source, IF_NAMESIZE as size_t);
            }
        }
    } else {
        return b"bad address\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return NULL_0 as *mut libc::c_char;
}
#[c2rust::src_loc = "892:1"]
unsafe extern "C" fn add_rev4(mut addr: in_addr, mut msize: libc::c_int)
 -> *mut server {
    let mut serv =
        opt_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
            *mut server;
    let mut a = __bswap_32(addr.s_addr);
    let mut p = 0 as *mut libc::c_char;
    memset(serv as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<server>() as libc::c_ulong);
    (*serv).domain =
        opt_malloc(29 as libc::c_int as size_t) as *mut libc::c_char;
    p = (*serv).domain;
    let mut current_block_8: u64;
    match msize {
        32 => {
            p =
                p.offset(sprintf(p,
                                 b"%u.\x00" as *const u8 as
                                     *const libc::c_char,
                                 a & 0xff as libc::c_int as libc::c_uint) as
                             isize);
            current_block_8 = 643824972125085941;
        }
        24 => { current_block_8 = 643824972125085941; }
        16 => { current_block_8 = 17981750616475279043; }
        8 => { current_block_8 = 12821047218658804999; }
        _ => {
            free((*serv).domain as *mut libc::c_void);
            free(serv as *mut libc::c_void);
            return NULL_0 as *mut server
        }
    }
    match current_block_8 {
        643824972125085941 =>
        /* fall through */
        {
            p =
                p.offset(sprintf(p,
                                 b"%d.\x00" as *const u8 as
                                     *const libc::c_char,
                                 a >> 8 as libc::c_int &
                                     0xff as libc::c_int as libc::c_uint) as
                             isize);
            current_block_8 = 17981750616475279043;
        }
        _ => { }
    }
    match current_block_8 {
        17981750616475279043 =>
        /* fall through */
        {
            p =
                p.offset(sprintf(p,
                                 b"%d.\x00" as *const u8 as
                                     *const libc::c_char,
                                 a >> 16 as libc::c_int &
                                     0xff as libc::c_int as libc::c_uint) as
                             isize)
        }
        _ => { }
    }
    /* fall through */
    p =
        p.offset(sprintf(p, b"%d.\x00" as *const u8 as *const libc::c_char,
                         a >> 24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as
                     isize); /* strlen("32*<n.>ip6.arpa")+1 */
    p =
        p.offset(sprintf(p,
                         b"in-addr.arpa\x00" as *const u8 as
                             *const libc::c_char) as isize);
    (*serv).flags = SERV_HAS_DOMAIN;
    (*serv).next = (*dnsmasq_daemon).servers;
    (*dnsmasq_daemon).servers = serv;
    return serv;
}
#[c2rust::src_loc = "931:1"]
unsafe extern "C" fn add_rev6(mut addr: *mut in6_addr, mut msize: libc::c_int)
 -> *mut server {
    let mut serv =
        opt_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
            *mut server;
    let mut p = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    memset(serv as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<server>() as libc::c_ulong);
    (*serv).domain =
        opt_malloc(73 as libc::c_int as size_t) as *mut libc::c_char;
    p = (*serv).domain;
    i = msize - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut dig =
            *(addr as
                  *mut libc::c_uchar).offset((i >> 3 as libc::c_int) as isize)
                as libc::c_int;
        p =
            p.offset(sprintf(p,
                             b"%.1x.\x00" as *const u8 as *const libc::c_char,
                             if i >> 2 as libc::c_int & 1 as libc::c_int != 0
                                {
                                 (dig) & 15 as libc::c_int
                             } else { (dig) >> 4 as libc::c_int }) as isize);
        i -= 4 as libc::c_int
    }
    p =
        p.offset(sprintf(p,
                         b"ip6.arpa\x00" as *const u8 as *const libc::c_char)
                     as isize);
    (*serv).flags = SERV_HAS_DOMAIN;
    (*serv).next = (*dnsmasq_daemon).servers;
    (*dnsmasq_daemon).servers = serv;
    return serv;
}
#[c2rust::src_loc = "956:1"]
unsafe extern "C" fn is_tag_prefix(mut arg: *mut libc::c_char)
 -> libc::c_int {
    if !arg.is_null() &&
           (strstr(arg, b"net:\x00" as *const u8 as *const libc::c_char) ==
                arg ||
                strstr(arg, b"tag:\x00" as *const u8 as *const libc::c_char)
                    == arg) {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "964:1"]
unsafe extern "C" fn set_prefix(mut arg: *mut libc::c_char)
 -> *mut libc::c_char {
    if strstr(arg, b"set:\x00" as *const u8 as *const libc::c_char) == arg {
        return arg.offset(4 as libc::c_int as isize)
    }
    return arg;
}
#[c2rust::src_loc = "972:1"]
unsafe extern "C" fn dhcp_netid_create(mut net: *const libc::c_char,
                                       mut next: *mut dhcp_netid)
 -> *mut dhcp_netid {
    let mut tt = 0 as *mut dhcp_netid;
    tt =
        opt_malloc(::std::mem::size_of::<dhcp_netid>() as libc::c_ulong) as
            *mut dhcp_netid;
    (*tt).net = opt_string_alloc(net);
    (*tt).next = next;
    return tt;
}
#[c2rust::src_loc = "981:1"]
unsafe extern "C" fn dhcp_netid_free(mut nid: *mut dhcp_netid) {
    while !nid.is_null() {
        let mut tmp = nid;
        nid = (*nid).next;
        free((*tmp).net as *mut libc::c_void);
        free(tmp as *mut libc::c_void);
    };
}
/* Parse one or more tag:s before parameters.
 * Moves arg to the end of tags. */
#[c2rust::src_loc = "994:1"]
unsafe extern "C" fn dhcp_tags(mut arg: *mut *mut libc::c_char)
 -> *mut dhcp_netid {
    let mut id = NULL_0 as *mut dhcp_netid;
    while is_tag_prefix(*arg) != 0 {
        let mut comma = split(*arg);
        id = dhcp_netid_create((*arg).offset(4 as libc::c_int as isize), id);
        *arg = comma
    }
    if (*arg).is_null() {
        dhcp_netid_free(id);
        id = NULL_0 as *mut dhcp_netid
    }
    return id;
}
#[c2rust::src_loc = "1012:1"]
unsafe extern "C" fn dhcp_netid_list_free(mut netid: *mut dhcp_netid_list) {
    while !netid.is_null() {
        let mut tmplist = netid;
        netid = (*netid).next;
        dhcp_netid_free((*tmplist).list);
        free(tmplist as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1023:1"]
unsafe extern "C" fn dhcp_config_free(mut config: *mut dhcp_config) {
    if !config.is_null() {
        let mut hwaddr = (*config).hwaddr;
        while !hwaddr.is_null() {
            let mut tmp = hwaddr;
            hwaddr = (*hwaddr).next;
            free(tmp as *mut libc::c_void);
        }
        dhcp_netid_list_free((*config).netid);
        dhcp_netid_free((*config).filter);
        if (*config).flags & CONFIG_CLID as libc::c_uint != 0 {
            free((*config).clid as *mut libc::c_void);
        }
        if (*config).flags & CONFIG_ADDR6 as libc::c_uint != 0 {
            let mut addr = 0 as *mut addrlist;
            let mut tmp_0 = 0 as *mut addrlist;
            addr = (*config).addr6;
            while !addr.is_null() {
                tmp_0 = (*addr).next;
                free(addr as *mut libc::c_void);
                addr = tmp_0
            }
        }
        free(config as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1059:1"]
unsafe extern "C" fn dhcp_context_free(mut ctx: *mut dhcp_context) {
    if !ctx.is_null() {
        dhcp_netid_free((*ctx).filter);
        free((*ctx).netid.net as *mut libc::c_void);
        free((*ctx).template_interface as *mut libc::c_void);
        free(ctx as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1072:1"]
unsafe extern "C" fn dhcp_opt_free(mut opt: *mut dhcp_opt) {
    if (*opt).flags & DHOPT_VENDOR != 0 {
        free((*opt).u.vendor_class as *mut libc::c_void);
    }
    dhcp_netid_free((*opt).netid);
    free((*opt).val as *mut libc::c_void);
    free(opt as *mut libc::c_void);
}
/* This is too insanely large to keep in-line in the switch */
#[c2rust::src_loc = "1083:1"]
unsafe extern "C" fn parse_dhcp_opt(mut errstr: *mut libc::c_char,
                                    mut arg: *mut libc::c_char,
                                    mut flags: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut new =
        opt_malloc(::std::mem::size_of::<dhcp_opt>() as libc::c_ulong) as
            *mut dhcp_opt;
    let mut lenchar = 0 as libc::c_int as libc::c_char;
    let mut cp = 0 as *mut libc::c_char;
    let mut addrs: libc::c_int = 0;
    let mut digs: libc::c_int = 0;
    let mut is_addr: libc::c_int = 0;
    let mut is_addr6: libc::c_int = 0;
    let mut is_hex: libc::c_int = 0;
    let mut is_dec: libc::c_int = 0;
    let mut is_string: libc::c_int = 0;
    let mut dots: libc::c_int = 0;
    let mut comma = NULL_0 as *mut libc::c_char;
    let mut opt_len = 0 as libc::c_int as u16_0;
    let mut is6 = 0 as libc::c_int;
    let mut option_ok = 0 as libc::c_int;
    (*new).len = 0 as libc::c_int;
    (*new).flags = flags;
    (*new).netid = NULL_0 as *mut dhcp_netid;
    (*new).val = NULL_0 as *mut libc::c_uchar;
    (*new).opt = 0 as libc::c_int;
    while !arg.is_null() {
        comma = split(arg);
        cp = arg;
        while *cp != 0 {
            if (*cp as libc::c_int) < '0' as i32 ||
                   *cp as libc::c_int > '9' as i32 {
                break ;
            }
            cp = cp.offset(1)
        }
        if *cp == 0 {
            (*new).opt = atoi(arg);
            opt_len = 0 as libc::c_int as u16_0;
            option_ok = 1 as libc::c_int;
            break ;
        } else if strstr(arg,
                         b"option:\x00" as *const u8 as *const libc::c_char)
                      == arg {
            (*new).opt =
                lookup_dhcp_opt(AF_INET,
                                arg.offset(7 as libc::c_int as isize));
            if (*new).opt != -(1 as libc::c_int) {
                opt_len = lookup_dhcp_len(AF_INET, (*new).opt) as u16_0;
                /* option:<optname> must follow tag and vendor string. */
                if opt_len as libc::c_int & OT_INTERNAL == 0 ||
                       flags == DHOPT_MATCH {
                    option_ok = 1 as libc::c_int
                }
            }
            break ;
        } else if strstr(arg,
                         b"option6:\x00" as *const u8 as *const libc::c_char)
                      == arg {
            cp = arg.offset(8 as libc::c_int as isize);
            while *cp != 0 {
                if (*cp as libc::c_int) < '0' as i32 ||
                       *cp as libc::c_int > '9' as i32 {
                    break ;
                }
                cp = cp.offset(1)
            }
            if *cp == 0 {
                (*new).opt = atoi(arg.offset(8 as libc::c_int as isize));
                opt_len = 0 as libc::c_int as u16_0;
                option_ok = 1 as libc::c_int
            } else {
                (*new).opt =
                    lookup_dhcp_opt(AF_INET6,
                                    arg.offset(8 as libc::c_int as isize));
                if (*new).opt != -(1 as libc::c_int) {
                    opt_len = lookup_dhcp_len(AF_INET6, (*new).opt) as u16_0;
                    if opt_len as libc::c_int & OT_INTERNAL == 0 ||
                           flags == DHOPT_MATCH {
                        option_ok = 1 as libc::c_int
                    }
                }
            }
            /* option6:<opt>|<optname> must follow tag and vendor string. */
            is6 = 1 as libc::c_int;
            break ;
        } else {
            if strstr(arg, b"vendor:\x00" as *const u8 as *const libc::c_char)
                   == arg {
                (*new).u.vendor_class =
                    opt_string_alloc(arg.offset(7 as libc::c_int as isize)) as
                        *mut libc::c_uchar;
                (*new).flags |= DHOPT_VENDOR
            } else if strstr(arg,
                             b"encap:\x00" as *const u8 as
                                 *const libc::c_char) == arg {
                (*new).u.encap = atoi(arg.offset(6 as libc::c_int as isize));
                (*new).flags |= DHOPT_ENCAPSULATE
            } else if strstr(arg,
                             b"vi-encap:\x00" as *const u8 as
                                 *const libc::c_char) == arg {
                (*new).u.encap = atoi(arg.offset(9 as libc::c_int as isize));
                (*new).flags |= DHOPT_RFC3925;
                if flags == DHOPT_MATCH {
                    option_ok = 1 as libc::c_int;
                    break ;
                }
            } else {
                /* allow optional "net:" or "tag:" for consistency */
                let mut name: *const libc::c_char =
                    if is_tag_prefix(arg) != 0 {
                        arg.offset(4 as libc::c_int as isize)
                    } else { set_prefix(arg) };
                (*new).netid = dhcp_netid_create(name, (*new).netid)
            }
            arg = comma
        }
    }
    if is6 != 0 {
        if (*new).flags & (DHOPT_VENDOR | DHOPT_ENCAPSULATE) != 0 {
            strcpy(errstr,
                   b"unsupported encapsulation for IPv6 option\x00" as
                       *const u8 as *const libc::c_char);
            current_block = 14151404249770905673;
        } else {
            if opt_len as libc::c_int == 0 as libc::c_int &&
                   (*new).flags & DHOPT_RFC3925 == 0 {
                opt_len = lookup_dhcp_len(AF_INET6, (*new).opt) as u16_0
            }
            current_block = 317151059986244064;
        }
    } else {
        if opt_len as libc::c_int == 0 as libc::c_int &&
               (*new).flags &
                   (DHOPT_VENDOR | DHOPT_ENCAPSULATE | DHOPT_RFC3925) == 0 {
            opt_len = lookup_dhcp_len(AF_INET, (*new).opt) as u16_0
        }
        current_block = 317151059986244064;
    }
    match current_block {
        317151059986244064 =>
        /* option may be missing with rfc3925 match */
        {
            if option_ok == 0 {
                strcpy(errstr,
                       b"bad dhcp-option\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                if !comma.is_null() {
                    /* characterise the value */
                    let mut c: libc::c_char = 0;
                    let mut found_dig = 0 as libc::c_int;
                    let mut found_colon = 0 as libc::c_int;
                    is_string = 1 as libc::c_int;
                    is_dec = is_string;
                    is_hex = is_dec;
                    is_addr6 = is_hex;
                    is_addr = is_addr6;
                    digs = 1 as libc::c_int;
                    addrs = digs;
                    dots = 0 as libc::c_int;
                    cp = comma;
                    loop  {
                        c = *cp;
                        if !(c != 0) { break ; }
                        if c as libc::c_int == ',' as i32 {
                            addrs += 1;
                            is_hex = 0 as libc::c_int;
                            is_dec = is_hex
                        } else if c as libc::c_int == ':' as i32 {
                            digs += 1;
                            is_addr = 0 as libc::c_int;
                            is_dec = is_addr;
                            found_colon = 1 as libc::c_int
                        } else if c as libc::c_int == '/' as i32 {
                            is_hex = 0 as libc::c_int;
                            is_dec = is_hex;
                            is_addr6 = is_dec;
                            if cp == comma {
                                /* leading / means a pathname */
                                is_addr = 0 as libc::c_int
                            }
                        } else if c as libc::c_int == '.' as i32 {
                            is_hex = 0 as libc::c_int;
                            is_dec = is_hex;
                            dots += 1
                        } else if c as libc::c_int == '-' as i32 {
                            is_addr6 = 0 as libc::c_int;
                            is_addr = is_addr6;
                            is_hex = is_addr
                        } else if c as libc::c_int == ' ' as i32 {
                            is_hex = 0 as libc::c_int;
                            is_dec = is_hex
                        } else if !(c as libc::c_int >= '0' as i32 &&
                                        c as libc::c_int <= '9' as i32) {
                            is_addr = 0 as libc::c_int;
                            if *cp.offset(1 as libc::c_int as isize) as
                                   libc::c_int == 0 as libc::c_int &&
                                   is_dec != 0 &&
                                   (c as libc::c_int == 'b' as i32 ||
                                        c as libc::c_int == 's' as i32 ||
                                        c as libc::c_int == 'i' as i32) {
                                lenchar = c;
                                *cp = 0 as libc::c_int as libc::c_char
                            } else { is_dec = 0 as libc::c_int }
                            if !(c as libc::c_int >= 'A' as i32 &&
                                     c as libc::c_int <= 'F' as i32 ||
                                     c as libc::c_int >= 'a' as i32 &&
                                         c as libc::c_int <= 'f' as i32 ||
                                     c as libc::c_int == '*' as i32 &&
                                         flags & DHOPT_MATCH != 0) {
                                is_hex = 0 as libc::c_int;
                                if c as libc::c_int != '[' as i32 &&
                                       c as libc::c_int != ']' as i32 {
                                    is_addr6 = 0 as libc::c_int
                                }
                            }
                        } else { found_dig = 1 as libc::c_int }
                        cp = cp.offset(1)
                    }
                    if found_dig == 0 {
                        is_addr = 0 as libc::c_int;
                        is_dec = is_addr
                    }
                    if found_colon == 0 { is_addr6 = 0 as libc::c_int }
                    /* NTP server option takes hex, addresses or FQDN */
                    if is6 != 0 && (*new).opt == OPTION6_NTP_SERVER &&
                           is_hex == 0 {
                        opt_len =
                            (opt_len as libc::c_int |
                                 if is_addr6 != 0 {
                                     OT_ADDR_LIST
                                 } else { OT_RFC1035_NAME }) as u16_0
                    }
                    /* We know that some options take addresses */
                    if opt_len as libc::c_int & OT_ADDR_LIST != 0 {
                        is_hex = 0 as libc::c_int;
                        is_dec = is_hex;
                        is_string = is_dec;
                        if is6 == 0 &&
                               (is_addr == 0 || dots == 0 as libc::c_int) {
                            strcpy(errstr,
                                   b"bad IP address\x00" as *const u8 as
                                       *const libc::c_char);
                            current_block = 14151404249770905673;
                        } else if is6 != 0 && is_addr6 == 0 {
                            strcpy(errstr,
                                   b"bad IPv6 address\x00" as *const u8 as
                                       *const libc::c_char);
                            current_block = 14151404249770905673;
                        } else { current_block = 6407515180622463684; }
                    } else {
                        /* or names */
                        if opt_len as libc::c_int &
                               (OT_NAME | OT_RFC1035_NAME | OT_CSTRING) != 0 {
                            is_hex = 0 as libc::c_int;
                            is_dec = is_hex;
                            is_addr = is_dec;
                            is_addr6 = is_addr
                        }
                        current_block = 6407515180622463684;
                    }
                    match current_block {
                        14151404249770905673 => { }
                        _ => {
                            if found_dig != 0 &&
                                   opt_len as libc::c_int & OT_TIME != 0 &&
                                   strlen(comma) >
                                       0 as libc::c_int as libc::c_ulong {
                                let mut val: libc::c_int = 0;
                                let mut fac = 1 as libc::c_int;
                                let mut current_block_105: u64;
                                match *comma.offset(strlen(comma).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                                        as isize) as
                                          libc::c_int {
                                    119 | 87 => {
                                        fac *= 7 as libc::c_int;
                                        current_block_105 =
                                            13975322803090976125;
                                    }
                                    100 | 68 => {
                                        current_block_105 =
                                            13975322803090976125;
                                    }
                                    104 | 72 => {
                                        current_block_105 =
                                            16222678236269241473;
                                    }
                                    109 | 77 => {
                                        current_block_105 =
                                            15298318582559719273;
                                    }
                                    115 | 83 => {
                                        current_block_105 =
                                            10680264320049698114;
                                    }
                                    _ => {
                                        current_block_105 =
                                            7761909515536616543;
                                    }
                                }
                                match current_block_105 {
                                    13975322803090976125 =>
                                    /* fall through */
                                    {
                                        fac *= 24 as libc::c_int;
                                        current_block_105 =
                                            16222678236269241473;
                                    }
                                    _ => { }
                                }
                                match current_block_105 {
                                    16222678236269241473 =>
                                    /* fall through */
                                    {
                                        fac *= 60 as libc::c_int;
                                        current_block_105 =
                                            15298318582559719273;
                                    }
                                    _ => { }
                                }
                                match current_block_105 {
                                    15298318582559719273 =>
                                    /* fall through */
                                    {
                                        fac *= 60 as libc::c_int;
                                        current_block_105 =
                                            10680264320049698114;
                                    }
                                    _ => { }
                                }
                                match current_block_105 {
                                    10680264320049698114 =>
                                    /* fall through */
                                    {
                                        *comma.offset(strlen(comma).wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                          as isize) =
                                            0 as libc::c_int as libc::c_char
                                    }
                                    _ => { }
                                }
                                (*new).len = 4 as libc::c_int;
                                (*new).val =
                                    opt_malloc(4 as libc::c_int as size_t) as
                                        *mut libc::c_uchar;
                                val = atoi(comma);
                                *((*new).val as *mut libc::c_int) =
                                    __bswap_32((val * fac) as __uint32_t) as
                                        libc::c_int;
                                current_block = 15439134456549723682;
                            } else if is_hex != 0 && digs > 1 as libc::c_int {
                                (*new).len = digs;
                                (*new).val =
                                    opt_malloc((*new).len as size_t) as
                                        *mut libc::c_uchar;
                                parse_hex(comma, (*new).val, digs,
                                          if flags & DHOPT_MATCH != 0 {
                                              &mut (*new).u.wildcard_mask
                                          } else {
                                              NULL_0 as *mut libc::c_uint
                                          }, NULL_0 as *mut libc::c_int);
                                (*new).flags |= DHOPT_HEX;
                                current_block = 15439134456549723682;
                            } else if is_dec != 0 {
                                let mut i: libc::c_int = 0;
                                let mut val_0 = atoi(comma);
                                /* assume numeric arg is 1 byte except for
	     options where it is known otherwise.
	     For vendor class option, we have to hack. */
                                if opt_len as libc::c_int != 0 as libc::c_int
                                   {
                                    (*new).len = opt_len as libc::c_int
                                } else if val_0 as libc::c_uint &
                                              0xffff0000 as libc::c_uint != 0
                                 {
                                    (*new).len = 4 as libc::c_int
                                } else if val_0 & 0xff00 as libc::c_int != 0 {
                                    (*new).len = 2 as libc::c_int
                                } else { (*new).len = 1 as libc::c_int }
                                if lenchar as libc::c_int == 'b' as i32 {
                                    (*new).len = 1 as libc::c_int
                                } else if lenchar as libc::c_int == 's' as i32
                                 {
                                    (*new).len = 2 as libc::c_int
                                } else if lenchar as libc::c_int == 'i' as i32
                                 {
                                    (*new).len = 4 as libc::c_int
                                }
                                (*new).val =
                                    opt_malloc((*new).len as size_t) as
                                        *mut libc::c_uchar;
                                i = 0 as libc::c_int;
                                while i < (*new).len {
                                    *(*new).val.offset(i as isize) =
                                        (val_0 >>
                                             ((*new).len - i -
                                                  1 as libc::c_int) *
                                                 8 as libc::c_int) as
                                            libc::c_uchar;
                                    i += 1
                                }
                                current_block = 15439134456549723682;
                            } else if is_addr != 0 && is6 == 0 {
                                let mut in_0 = in_addr{s_addr: 0,};
                                let mut op = 0 as *mut libc::c_uchar;
                                let mut slash = 0 as *mut libc::c_char;
                                /* max length of address/subnet descriptor is five bytes,
	     add one for the option 120 enc byte too */
                                op =
                                    opt_malloc((5 as libc::c_int * addrs +
                                                    1 as libc::c_int) as
                                                   size_t) as
                                        *mut libc::c_uchar; /* RFC 3361 "enc byte" */
                                (*new).val = op;
                                (*new).flags |= DHOPT_ADDR;
                                if (*new).flags &
                                       (DHOPT_ENCAPSULATE | DHOPT_VENDOR |
                                            DHOPT_RFC3925) == 0 &&
                                       (*new).opt == OPTION_SIP_SERVER {
                                    let fresh6 = op;
                                    op = op.offset(1);
                                    *fresh6 =
                                        1 as libc::c_int as libc::c_uchar;
                                    (*new).flags &= !DHOPT_ADDR
                                }
                                loop  {
                                    let fresh7 = addrs;
                                    addrs = addrs - 1;
                                    if !(fresh7 != 0) {
                                        current_block = 1131197912709891142;
                                        break ;
                                    }
                                    cp = comma;
                                    comma = split(cp);
                                    slash =
                                        split_chr(cp,
                                                  '/' as i32 as libc::c_char);
                                    if inet_pton(AF_INET, cp,
                                                 &mut in_0 as *mut in_addr as
                                                     *mut libc::c_void) == 0 {
                                        strcpy(errstr,
                                               b"bad IPv4 address\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                                        current_block = 14151404249770905673;
                                        break ;
                                    } else if slash.is_null() {
                                        memcpy(op as *mut libc::c_void,
                                               &mut in_0 as *mut in_addr as
                                                   *const libc::c_void,
                                               INADDRSZ as libc::c_ulong);
                                        op = op.offset(INADDRSZ as isize)
                                    } else {
                                        let mut p =
                                            &mut in_0 as *mut in_addr as
                                                *mut libc::c_uchar;
                                        let mut netsize = atoi(slash);
                                        let fresh8 = op;
                                        op = op.offset(1);
                                        *fresh8 = netsize as libc::c_uchar;
                                        if netsize > 0 as libc::c_int {
                                            let fresh9 = p;
                                            p = p.offset(1);
                                            let fresh10 = op;
                                            op = op.offset(1);
                                            *fresh10 = *fresh9
                                        }
                                        if netsize > 8 as libc::c_int {
                                            let fresh11 = p;
                                            p = p.offset(1);
                                            let fresh12 = op;
                                            op = op.offset(1);
                                            *fresh12 = *fresh11
                                        }
                                        if netsize > 16 as libc::c_int {
                                            let fresh13 = p;
                                            p = p.offset(1);
                                            let fresh14 = op;
                                            op = op.offset(1);
                                            *fresh14 = *fresh13
                                        }
                                        if netsize > 24 as libc::c_int {
                                            let fresh15 = p;
                                            p = p.offset(1);
                                            let fresh16 = op;
                                            op = op.offset(1);
                                            *fresh16 = *fresh15
                                        }
                                        (*new).flags &= !DHOPT_ADDR
                                        /* cannot re-write descriptor format */
                                    }
                                }
                                match current_block {
                                    14151404249770905673 => { }
                                    _ => {
                                        (*new).len =
                                            op.wrapping_offset_from((*new).val)
                                                as libc::c_long as
                                                libc::c_int;
                                        current_block = 15439134456549723682;
                                    }
                                }
                            } else if is_addr6 != 0 && is6 != 0 {
                                let mut op_0 = 0 as *mut libc::c_uchar;
                                op_0 =
                                    opt_malloc((16 as libc::c_int * addrs) as
                                                   size_t) as
                                        *mut libc::c_uchar;
                                (*new).val = op_0;
                                (*new).flags |= DHOPT_ADDR6;
                                loop  {
                                    let fresh17 = addrs;
                                    addrs = addrs - 1;
                                    if !(fresh17 != 0) {
                                        current_block = 6930811285952240378;
                                        break ;
                                    }
                                    cp = comma;
                                    comma = split(cp);
                                    /* check for [1234::7] */
                                    if *cp as libc::c_int == '[' as i32 {
                                        cp = cp.offset(1)
                                    }
                                    if strlen(cp) >
                                           1 as libc::c_int as libc::c_ulong
                                           &&
                                           *cp.offset(strlen(cp).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                                                          as isize) as
                                               libc::c_int == ']' as i32 {
                                        *cp.offset(strlen(cp).wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong)
                                                       as isize) =
                                            0 as libc::c_int as libc::c_char
                                    }
                                    if inet_pton(AF_INET6, cp,
                                                 op_0 as *mut libc::c_void) !=
                                           0 {
                                        op_0 = op_0.offset(IN6ADDRSZ as isize)
                                    } else {
                                        strcpy(errstr,
                                               b"bad IPv6 address\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                                        current_block = 14151404249770905673;
                                        break ;
                                    }
                                }
                                match current_block {
                                    14151404249770905673 => { }
                                    _ => {
                                        (*new).len =
                                            op_0.wrapping_offset_from((*new).val)
                                                as libc::c_long as
                                                libc::c_int;
                                        current_block = 15439134456549723682;
                                    }
                                }
                            } else if is_string != 0 {
                                /* text arg */
                                if ((*new).opt == OPTION_DOMAIN_SEARCH ||
                                        (*new).opt == OPTION_SIP_SERVER) &&
                                       is6 == 0 &&
                                       (*new).flags &
                                           (DHOPT_ENCAPSULATE | DHOPT_VENDOR |
                                                DHOPT_RFC3925) == 0 {
                                    /* dns search, RFC 3397, or SIP, RFC 3361 */
                                    let mut q = 0 as *mut libc::c_uchar;
                                    let mut r = 0 as *mut libc::c_uchar;
                                    let mut tail = 0 as *mut libc::c_uchar;
                                    let mut p_0 = 0 as *mut libc::c_uchar;
                                    let mut m = NULL_0 as *mut libc::c_uchar;
                                    let mut newp = 0 as *mut libc::c_uchar;
                                    let mut newlen: size_t = 0;
                                    let mut len = 0 as libc::c_int as size_t;
                                    let mut header_size =
                                        if (*new).opt == OPTION_DOMAIN_SEARCH
                                           {
                                            0 as libc::c_int
                                        } else { 1 as libc::c_int };
                                    arg = comma;
                                    comma = split(arg);
                                    loop  {
                                        if !(!arg.is_null() &&
                                                 *arg as libc::c_int != 0) {
                                            current_block =
                                                5913497314667414582;
                                            break ;
                                        }
                                        let mut in_1 = 0 as *mut libc::c_char;
                                        let mut dom =
                                            NULL_0 as *mut libc::c_char;
                                        let mut domlen =
                                            1 as libc::c_int as size_t;
                                        /* Allow "." as an empty domain */
                                        if strcmp(arg,
                                                  b".\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                               0 as libc::c_int {
                                            dom = canonicalise_opt(arg);
                                            if dom.is_null() {
                                                strcpy(errstr,
                                                       b"bad domain in dhcp-option\x00"
                                                           as *const u8 as
                                                           *const libc::c_char);
                                                current_block =
                                                    14151404249770905673;
                                                break ;
                                            } else {
                                                domlen =
                                                    strlen(dom).wrapping_add(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                            }
                                        }
                                        newp =
                                            opt_malloc(len.wrapping_add(domlen).wrapping_add(header_size
                                                                                                 as
                                                                                                 libc::c_ulong))
                                                as *mut libc::c_uchar;
                                        if !m.is_null() {
                                            memcpy(newp as *mut libc::c_void,
                                                   m as *const libc::c_void,
                                                   (header_size as
                                                        libc::c_ulong).wrapping_add(len));
                                            free(m as *mut libc::c_void);
                                        }
                                        m = newp;
                                        p_0 = m.offset(header_size as isize);
                                        q = p_0.offset(len as isize);
                                        /* add string on the end in RFC1035 format */
                                        in_1 = dom;
                                        while !in_1.is_null() &&
                                                  *in_1 as libc::c_int != 0 {
                                            let fresh18 = q;
                                            q = q.offset(1);
                                            let mut cp_0 = fresh18;
                                            let mut j: libc::c_int = 0;
                                            j = 0 as libc::c_int;
                                            while *in_1 as libc::c_int != 0 &&
                                                      *in_1 as libc::c_int !=
                                                          '.' as i32 {
                                                let fresh19 = q;
                                                q = q.offset(1);
                                                *fresh19 =
                                                    *in_1 as libc::c_uchar;
                                                in_1 = in_1.offset(1);
                                                j += 1
                                            }
                                            *cp_0 = j as libc::c_uchar;
                                            if *in_1 != 0 {
                                                in_1 = in_1.offset(1)
                                            }
                                        }
                                        let fresh20 = q;
                                        q = q.offset(1);
                                        *fresh20 =
                                            0 as libc::c_int as libc::c_uchar;
                                        free(dom as *mut libc::c_void);
                                        /* Now tail-compress using earlier names. */
                                        newlen =
                                            q.wrapping_offset_from(p_0) as
                                                libc::c_long as size_t;
                                        tail = p_0.offset(len as isize);
                                        's_1219:
                                            while *tail != 0 {
                                                r = p_0;
                                                while (r.wrapping_offset_from(p_0)
                                                           as libc::c_long) <
                                                          len as libc::c_int
                                                              as libc::c_long
                                                      {
                                                    if strcmp(r as
                                                                  *mut libc::c_char,
                                                              tail as
                                                                  *mut libc::c_char)
                                                           == 0 as libc::c_int
                                                       {
                                                        let mut t_s =
                                                            (r.wrapping_offset_from(p_0)
                                                                 as
                                                                 libc::c_long
                                                                 |
                                                                 0xc000 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_long)
                                                                as u16_0;
                                                        let mut t_cp = tail;
                                                        let fresh21 = t_cp;
                                                        t_cp = t_cp.offset(1);
                                                        *fresh21 =
                                                            (t_s as
                                                                 libc::c_int
                                                                 >>
                                                                 8 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uchar;
                                                        *t_cp =
                                                            t_s as
                                                                libc::c_uchar;
                                                        tail =
                                                            tail.offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize);
                                                        newlen =
                                                            tail.wrapping_offset_from(p_0)
                                                                as
                                                                libc::c_long
                                                                as size_t;
                                                        break 's_1219 ;
                                                    } else {
                                                        r =
                                                            r.offset((*r as
                                                                          libc::c_int
                                                                          +
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize)
                                                    }
                                                }
                                                tail =
                                                    tail.offset((*tail as
                                                                     libc::c_int
                                                                     +
                                                                     1 as
                                                                         libc::c_int)
                                                                    as isize)
                                            }
                                        len = newlen;
                                        arg = comma;
                                        comma = split(arg)
                                    }
                                    match current_block {
                                        14151404249770905673 => { }
                                        _ => {
                                            /* RFC 3361, enc byte is zero for names */
                                            if (*new).opt == OPTION_SIP_SERVER
                                                   && !m.is_null() {
                                                *m.offset(0 as libc::c_int as
                                                              isize) =
                                                    0 as libc::c_int as
                                                        libc::c_uchar
                                            }
                                            (*new).len =
                                                len as libc::c_int +
                                                    header_size;
                                            (*new).val = m;
                                            current_block =
                                                15439134456549723682;
                                        }
                                    }
                                } else if !comma.is_null() &&
                                              opt_len as libc::c_int &
                                                  OT_CSTRING != 0 {
                                    /* length fields are two bytes so need 16 bits for each string */
                                    let mut i_0: libc::c_int = 0;
                                    let mut commas = 1 as libc::c_int;
                                    let mut p_1 = 0 as *mut libc::c_uchar;
                                    let mut newp_0 = 0 as *mut libc::c_uchar;
                                    i_0 = 0 as libc::c_int;
                                    while *comma.offset(i_0 as isize) != 0 {
                                        if *comma.offset(i_0 as isize) as
                                               libc::c_int == ',' as i32 {
                                            commas += 1
                                        }
                                        i_0 += 1
                                    }
                                    newp_0 =
                                        opt_malloc(strlen(comma).wrapping_add((2
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   commas)
                                                                                  as
                                                                                  libc::c_ulong))
                                            as *mut libc::c_uchar;
                                    p_1 = newp_0;
                                    arg = comma;
                                    comma = split(arg);
                                    while !arg.is_null() &&
                                              *arg as libc::c_int != 0 {
                                        let mut len_0 = strlen(arg) as u16_0;
                                        unhide_metas(arg);
                                        let mut t_s_0 = len_0;
                                        let mut t_cp_0 = p_1;
                                        let fresh22 = t_cp_0;
                                        t_cp_0 = t_cp_0.offset(1);
                                        *fresh22 =
                                            (t_s_0 as libc::c_int >>
                                                 8 as libc::c_int) as
                                                libc::c_uchar;
                                        *t_cp_0 = t_s_0 as libc::c_uchar;
                                        p_1 =
                                            p_1.offset(2 as libc::c_int as
                                                           isize);
                                        memcpy(p_1 as *mut libc::c_void,
                                               arg as *const libc::c_void,
                                               len_0 as libc::c_ulong);
                                        p_1 =
                                            p_1.offset(len_0 as libc::c_int as
                                                           isize);
                                        arg = comma;
                                        comma = split(arg)
                                    }
                                    (*new).val = newp_0;
                                    (*new).len =
                                        p_1.wrapping_offset_from(newp_0) as
                                            libc::c_long as libc::c_int;
                                    current_block = 15439134456549723682;
                                } else if !comma.is_null() &&
                                              opt_len as libc::c_int &
                                                  OT_RFC1035_NAME != 0 {
                                    let mut p_2 =
                                        NULL_0 as *mut libc::c_uchar;
                                    let mut q_0 = 0 as *mut libc::c_uchar;
                                    let mut newp_1 = 0 as *mut libc::c_uchar;
                                    let mut end = 0 as *mut libc::c_uchar;
                                    let mut len_1 = 0 as libc::c_int;
                                    let mut header_size_0 =
                                        if is6 != 0 &&
                                               (*new).opt ==
                                                   OPTION6_NTP_SERVER {
                                            4 as libc::c_int
                                        } else { 0 as libc::c_int };
                                    arg = comma;
                                    comma = split(arg);
                                    loop  {
                                        if !(!arg.is_null() &&
                                                 *arg as libc::c_int != 0) {
                                            current_block =
                                                7499465236084769340;
                                            break ;
                                        }
                                        let mut dom_0 = canonicalise_opt(arg);
                                        if dom_0.is_null() {
                                            strcpy(errstr,
                                                   b"bad domain in dhcp-option\x00"
                                                       as *const u8 as
                                                       *const libc::c_char);
                                            current_block =
                                                14151404249770905673;
                                            break ;
                                        } else {
                                            newp_1 =
                                                opt_malloc(((len_1 +
                                                                 header_size_0)
                                                                as
                                                                libc::c_ulong).wrapping_add(strlen(dom_0)).wrapping_add(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                    as *mut libc::c_uchar;
                                            if !p_2.is_null() {
                                                memcpy(newp_1 as
                                                           *mut libc::c_void,
                                                       p_2 as
                                                           *const libc::c_void,
                                                       len_1 as
                                                           libc::c_ulong);
                                                free(p_2 as
                                                         *mut libc::c_void);
                                            }
                                            p_2 = newp_1;
                                            q_0 = p_2.offset(len_1 as isize);
                                            end =
                                                do_rfc1035_name(q_0.offset(header_size_0
                                                                               as
                                                                               isize),
                                                                dom_0,
                                                                NULL_0 as
                                                                    *mut libc::c_char);
                                            let fresh23 = end;
                                            end = end.offset(1);
                                            *fresh23 =
                                                0 as libc::c_int as
                                                    libc::c_uchar;
                                            if is6 != 0 &&
                                                   (*new).opt ==
                                                       OPTION6_NTP_SERVER {
                                                let mut t_s_1 =
                                                    3 as libc::c_int as u16_0;
                                                let mut t_cp_1 = q_0;
                                                let fresh24 = t_cp_1;
                                                t_cp_1 = t_cp_1.offset(1);
                                                *fresh24 =
                                                    (t_s_1 as libc::c_int >>
                                                         8 as libc::c_int) as
                                                        libc::c_uchar;
                                                *t_cp_1 =
                                                    t_s_1 as libc::c_uchar;
                                                q_0 =
                                                    q_0.offset(2 as
                                                                   libc::c_int
                                                                   as isize);
                                                let mut t_s_2 =
                                                    (end.wrapping_offset_from(q_0)
                                                         as libc::c_long -
                                                         2 as libc::c_int as
                                                             libc::c_long) as
                                                        u16_0;
                                                let mut t_cp_2 = q_0;
                                                let fresh25 = t_cp_2;
                                                t_cp_2 = t_cp_2.offset(1);
                                                *fresh25 =
                                                    (t_s_2 as libc::c_int >>
                                                         8 as libc::c_int) as
                                                        libc::c_uchar;
                                                *t_cp_2 =
                                                    t_s_2 as libc::c_uchar;
                                                q_0 =
                                                    q_0.offset(2 as
                                                                   libc::c_int
                                                                   as isize)
                                            }
                                            len_1 =
                                                end.wrapping_offset_from(p_2)
                                                    as libc::c_long as
                                                    libc::c_int;
                                            free(dom_0 as *mut libc::c_void);
                                            arg = comma;
                                            comma = split(arg)
                                        }
                                    }
                                    match current_block {
                                        14151404249770905673 => { }
                                        _ => {
                                            (*new).val = p_2;
                                            (*new).len = len_1;
                                            current_block =
                                                15439134456549723682;
                                        }
                                    }
                                } else {
                                    (*new).len = strlen(comma) as libc::c_int;
                                    /* keep terminating zero on string */
                                    (*new).val =
                                        opt_string_alloc(comma) as
                                            *mut libc::c_uchar;
                                    (*new).flags |= DHOPT_STRING;
                                    current_block = 15439134456549723682;
                                }
                            } else { current_block = 15439134456549723682; }
                        }
                    }
                } else { current_block = 15439134456549723682; }
                match current_block {
                    14151404249770905673 => { }
                    _ => {
                        if is6 == 0 &&
                               ((*new).len > 255 as libc::c_int ||
                                    (*new).len > 253 as libc::c_int &&
                                        (*new).flags &
                                            (DHOPT_VENDOR | DHOPT_ENCAPSULATE)
                                            != 0 ||
                                    (*new).len > 250 as libc::c_int &&
                                        (*new).flags & DHOPT_RFC3925 != 0) {
                            strcpy(errstr,
                                   b"dhcp-option too long\x00" as *const u8 as
                                       *const libc::c_char);
                        } else {
                            if flags == DHOPT_MATCH {
                                if (*new).flags &
                                       (DHOPT_ENCAPSULATE | DHOPT_VENDOR) != 0
                                       || (*new).netid.is_null() ||
                                       !(*(*new).netid).next.is_null() {
                                    strcpy(errstr,
                                           b"illegal dhcp-match\x00" as
                                               *const u8 as
                                               *const libc::c_char);
                                    current_block = 14151404249770905673;
                                } else {
                                    if is6 != 0 {
                                        (*new).next =
                                            (*dnsmasq_daemon).dhcp_match6;
                                        (*dnsmasq_daemon).dhcp_match6 = new
                                    } else {
                                        (*new).next =
                                            (*dnsmasq_daemon).dhcp_match;
                                        (*dnsmasq_daemon).dhcp_match = new
                                    }
                                    current_block = 7621687701095090519;
                                }
                            } else {
                                if is6 != 0 {
                                    (*new).next =
                                        (*dnsmasq_daemon).dhcp_opts6;
                                    (*dnsmasq_daemon).dhcp_opts6 = new
                                } else {
                                    (*new).next = (*dnsmasq_daemon).dhcp_opts;
                                    (*dnsmasq_daemon).dhcp_opts = new
                                }
                                current_block = 7621687701095090519;
                            }
                            match current_block {
                                14151404249770905673 => { }
                                _ => { return 1 as libc::c_int }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    dhcp_opt_free(new);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1626:1"]
pub unsafe extern "C" fn set_option_bool(mut opt: libc::c_uint) {
    (*dnsmasq_daemon).options[(opt as
                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                                  as usize] |=
        (1 as libc::c_uint) <<
            (opt as
                 libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                  as
                                                  libc::c_ulong).wrapping_mul(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong));
}
#[no_mangle]
#[c2rust::src_loc = "1631:1"]
pub unsafe extern "C" fn reset_option_bool(mut opt: libc::c_uint) {
    (*dnsmasq_daemon).options[(opt as
                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                                  as usize] &=
        !((1 as libc::c_uint) <<
              (opt as
                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)));
}
#[c2rust::src_loc = "1636:1"]
unsafe extern "C" fn server_list_free(mut list: *mut server) {
    while !list.is_null() {
        let mut tmp = list;
        list = (*list).next;
        free(tmp as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1646:1"]
unsafe extern "C" fn one_opt(mut option: libc::c_int,
                             mut arg: *mut libc::c_char,
                             mut errstr: *mut libc::c_char,
                             mut gen_err: *mut libc::c_char,
                             mut command_line: libc::c_int,
                             mut servers_only: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut comma = 0 as *mut libc::c_char;
    if option == '?' as i32 {
        strcpy(errstr, gen_err);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while usage[i as usize].opt != 0 as libc::c_int {
        if usage[i as usize].opt == option {
            let mut rept = usage[i as usize].rept as libc::c_int;
            if command_line != 0 {
                /* command line */
                if rept == OPT_LAST + 2 as libc::c_int {
                    strcpy(errstr,
                           b"illegal repeated flag\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if rept == OPT_LAST + 1 as libc::c_int {
                    usage[i as usize].rept =
                        (OPT_LAST + 2 as libc::c_int) as libc::c_uint
                }
            } else {
                /* allow file to override command line */
                if rept == OPT_LAST + 3 as libc::c_int {
                    strcpy(errstr,
                           b"illegal repeated keyword\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if rept == OPT_LAST + 2 as libc::c_int ||
                       rept == OPT_LAST + 1 as libc::c_int {
                    usage[i as usize].rept =
                        (OPT_LAST + 3 as libc::c_int) as libc::c_uint
                }
            }
            if rept != ARG_DUP && rept != OPT_LAST + 1 as libc::c_int &&
                   rept != OPT_LAST + 2 as libc::c_int {
                set_option_bool(rept as libc::c_uint);
                return 1 as libc::c_int
            }
            break ;
        } else { i += 1 }
    }
    match option {
        67 => {
            /* --conf-file */
            let mut file = opt_string_alloc(arg);
            if !file.is_null() {
                one_file(file, 0 as libc::c_int);
                free(file as *mut libc::c_void);
            }
            current_block = 7879481898411272068;
        }
        55 => {
            /* --conf-dir */
            let mut dir_stream = 0 as *mut DIR;
            let mut ent = 0 as *mut dirent;
            let mut directory = 0 as *mut libc::c_char;
            let mut path = 0 as *mut libc::c_char;
            let mut ignore_suffix = NULL_0 as *mut list;
            let mut match_suffix = NULL_0 as *mut list;
            let mut files = NULL_0 as *mut list;
            let mut li = 0 as *mut list;
            comma = split(arg);
            directory = opt_string_alloc(arg);
            if directory.is_null() {
                current_block = 7879481898411272068;
            } else {
                arg = comma;
                while !arg.is_null() {
                    comma = split(arg);
                    if strlen(arg) != 0 as libc::c_int as libc::c_ulong {
                        li =
                            opt_malloc(::std::mem::size_of::<list>() as
                                           libc::c_ulong) as *mut list;
                        if *arg as libc::c_int == '*' as i32 {
                            /* "*" with no suffix is a no-op */
                            if *arg.offset(1 as libc::c_int as isize) as
                                   libc::c_int == 0 as libc::c_int {
                                free(li as *mut libc::c_void);
                            } else {
                                (*li).next = match_suffix;
                                match_suffix = li;
                                /* Have to copy: buffer is overwritten */
                                (*li).name =
                                    opt_string_alloc(arg.offset(1 as
                                                                    libc::c_int
                                                                    as isize))
                            }
                        } else {
                            (*li).next = ignore_suffix;
                            ignore_suffix = li;
                            /* Have to copy: buffer is overwritten */
                            (*li).name = opt_string_alloc(arg)
                        }
                    }
                    arg = comma
                }
                dir_stream = opendir(directory);
                if dir_stream.is_null() {
                    die(b"cannot access directory %s: %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        directory, EC_FILE);
                }
                loop  {
                    ent = readdir(dir_stream);
                    if ent.is_null() { break ; }
                    let mut len = strlen((*ent).d_name.as_mut_ptr());
                    let mut buf =
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
                    /* ignore emacs backups and dotfiles */
                    if len == 0 as libc::c_int as libc::c_ulong ||
                           (*ent).d_name[len.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                                             as usize] as libc::c_int ==
                               '~' as i32 ||
                           (*ent).d_name[0 as libc::c_int as usize] as
                               libc::c_int == '#' as i32 &&
                               (*ent).d_name[len.wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                                                 as usize] as libc::c_int ==
                                   '#' as i32 ||
                           (*ent).d_name[0 as libc::c_int as usize] as
                               libc::c_int == '.' as i32 {
                        continue ;
                    }
                    if !match_suffix.is_null() {
                        li = match_suffix;
                        while !li.is_null() {
                            /* check for required suffices */
                            let mut ls = strlen((*li).name);
                            if len > ls &&
                                   strcmp((*li).name,
                                          &mut *(*ent).d_name.as_mut_ptr().offset(len.wrapping_sub(ls)
                                                                                      as
                                                                                      isize))
                                       == 0 as libc::c_int {
                                break ;
                            }
                            li = (*li).next
                        }
                        if li.is_null() { continue ; }
                    }
                    li = ignore_suffix;
                    while !li.is_null() {
                        /* check for proscribed suffices */
                        let mut ls_0 = strlen((*li).name);
                        if len > ls_0 &&
                               strcmp((*li).name,
                                      &mut *(*ent).d_name.as_mut_ptr().offset(len.wrapping_sub(ls_0)
                                                                                  as
                                                                                  isize))
                                   == 0 as libc::c_int {
                            break ;
                        }
                        li = (*li).next
                    }
                    if !li.is_null() { continue ; }
                    path =
                        opt_malloc(strlen(directory).wrapping_add(len).wrapping_add(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
                            as *mut libc::c_char;
                    strcpy(path, directory);
                    strcat(path,
                           b"/\x00" as *const u8 as *const libc::c_char);
                    strcat(path, (*ent).d_name.as_mut_ptr());
                    /* files must be readable */
                    if stat(path, &mut buf) == -(1 as libc::c_int) {
                        die(b"cannot access %s: %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            path, EC_FILE);
                    }
                    /* only reg files allowed. */
                    if buf.st_mode & __S_IFMT as libc::c_uint ==
                           0o100000 as libc::c_int as libc::c_uint {
                        /* sort files into order. */
                        let mut up = 0 as *mut *mut list;
                        let mut new =
                            opt_malloc(::std::mem::size_of::<list>() as
                                           libc::c_ulong) as *mut list;
                        (*new).name = path;
                        up = &mut files;
                        li = files;
                        while !li.is_null() {
                            if strcmp((*li).name, path) >= 0 as libc::c_int {
                                break ;
                            }
                            up = &mut (*li).next;
                            li = (*li).next
                        }
                        (*new).next = li;
                        *up = new
                    }
                }
                li = files;
                while !li.is_null() {
                    one_file((*li).name, 0 as libc::c_int);
                    li = (*li).next
                }
                closedir(dir_stream);
                free(directory as *mut libc::c_void);
                while !ignore_suffix.is_null() {
                    li = (*ignore_suffix).next;
                    free((*ignore_suffix).name as *mut libc::c_void);
                    free(ignore_suffix as *mut libc::c_void);
                    ignore_suffix = li
                }
                while !match_suffix.is_null() {
                    li = (*match_suffix).next;
                    free((*match_suffix).name as *mut libc::c_void);
                    free(match_suffix as *mut libc::c_void);
                    match_suffix = li
                }
                while !files.is_null() {
                    li = (*files).next;
                    free((*files).name as *mut libc::c_void);
                    free(files as *mut libc::c_void);
                    files = li
                }
                current_block = 7879481898411272068;
            }
        }
        LOPT_ADD_SBNET => {
            /* --add-subnet */
            set_option_bool(OPT_CLIENT_SUBNET as libc::c_uint);
            if !arg.is_null() {
                let mut err = 0 as *mut libc::c_char;
                let mut end = 0 as *mut libc::c_char;
                comma = split(arg);
                let mut new_0 =
                    opt_malloc(::std::mem::size_of::<mysubnet>() as
                                   libc::c_ulong) as *mut mysubnet;
                end = split_chr(arg, '/' as i32 as libc::c_char);
                if !end.is_null() {
                    /* has subnet+len */
                    err = parse_mysockaddr(arg, &mut (*new_0).addr);
                    if !err.is_null() {
                        strcpy(errstr, err);
                        free(new_0 as *mut libc::c_void);
                        return 0 as libc::c_int
                    }
                    if atoi_check(end, &mut (*new_0).mask) == 0 {
                        strcpy(errstr, gen_err);
                        free(new_0 as *mut libc::c_void);
                        return 0 as libc::c_int
                    }
                    (*new_0).addr_used = 1 as libc::c_int
                } else if atoi_check(arg, &mut (*new_0).mask) == 0 {
                    strcpy(errstr, gen_err);
                    free(new_0 as *mut libc::c_void);
                    return 0 as libc::c_int
                }
                (*dnsmasq_daemon).add_subnet4 = new_0;
                if !comma.is_null() {
                    new_0 =
                        opt_malloc(::std::mem::size_of::<mysubnet>() as
                                       libc::c_ulong) as *mut mysubnet;
                    end = split_chr(comma, '/' as i32 as libc::c_char);
                    if !end.is_null() {
                        /* has subnet+len */
                        err = parse_mysockaddr(comma, &mut (*new_0).addr);
                        if !err.is_null() {
                            strcpy(errstr, err);
                            free(new_0 as *mut libc::c_void);
                            return 0 as libc::c_int
                        }
                        if atoi_check(end, &mut (*new_0).mask) == 0 {
                            strcpy(errstr, gen_err);
                            free(new_0 as *mut libc::c_void);
                            return 0 as libc::c_int
                        }
                        (*new_0).addr_used = 1 as libc::c_int
                    } else if atoi_check(comma, &mut (*new_0).mask) == 0 {
                        strcpy(errstr, gen_err);
                        free(new_0 as *mut libc::c_void);
                        return 0 as libc::c_int
                    }
                    (*dnsmasq_daemon).add_subnet6 = new_0
                }
            }
            current_block = 7879481898411272068;
        }
        49 => {
            /* --enable-dbus */
            set_option_bool(OPT_DBUS as libc::c_uint);
            if !arg.is_null() {
                (*dnsmasq_daemon).dbus_name = opt_string_alloc(arg)
            } else {
                (*dnsmasq_daemon).dbus_name = DNSMASQ_SERVICE.as_mut_ptr()
            }
            current_block = 7879481898411272068;
        }
        LOPT_UBUS => {
            /* --enable-ubus */
            set_option_bool(OPT_UBUS as libc::c_uint);
            if !arg.is_null() {
                (*dnsmasq_daemon).ubus_name = opt_string_alloc(arg)
            } else {
                (*dnsmasq_daemon).ubus_name = DNSMASQ_UBUS_NAME.as_mut_ptr()
            }
            current_block = 7879481898411272068;
        }
        56 => {
            /* --log-facility */
            /* may be a filename */
            if !strchr(arg, '/' as i32).is_null() ||
                   strcmp(arg, b"-\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                (*dnsmasq_daemon).log_file = opt_string_alloc(arg)
            } else {
                i = 0 as libc::c_int;
                while !facilitynames[i as usize].c_name.is_null() {
                    if hostname_isequal(facilitynames[i as usize].c_name, arg)
                           != 0 {
                        break ;
                    }
                    i += 1
                }
                if !facilitynames[i as usize].c_name.is_null() {
                    (*dnsmasq_daemon).log_fac =
                        facilitynames[i as usize].c_val
                } else {
                    strcpy(errstr,
                           b"bad log facility\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
            }
            current_block = 7879481898411272068;
        }
        120 => {
            /* --pid-file */
            (*dnsmasq_daemon).runfile = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        114 => {
            /* --resolv-file */
            let mut name = opt_string_alloc(arg);
            let mut new_1 = 0 as *mut resolvc;
            let mut list_0 = (*dnsmasq_daemon).resolv_files;
            if !list_0.is_null() && (*list_0).is_default != 0 {
                /* replace default resolv file - possibly with nothing */
                if !name.is_null() {
                    (*list_0).is_default = 0 as libc::c_int;
                    (*list_0).name = name
                } else { list_0 = NULL_0 as *mut resolvc }
            } else if !name.is_null() {
                new_1 =
                    opt_malloc(::std::mem::size_of::<resolvc>() as
                                   libc::c_ulong) as *mut resolvc;
                (*new_1).next = list_0;
                (*new_1).name = name;
                (*new_1).is_default = 0 as libc::c_int;
                (*new_1).mtime = 0 as libc::c_int as time_t;
                (*new_1).logged = 0 as libc::c_int;
                list_0 = new_1
            }
            (*dnsmasq_daemon).resolv_files = list_0;
            current_block = 7879481898411272068;
        }
        LOPT_SERVERS_FILE => {
            (*dnsmasq_daemon).servers_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        109 => {
            /* --mx-host */
            let mut pref = 1 as libc::c_int; /* may be NULL */
            let mut new_2 = 0 as *mut mx_srv_record;
            let mut name_0 = 0 as *mut libc::c_char;
            let mut target = NULL_0 as *mut libc::c_char;
            comma = split(arg);
            if !comma.is_null() {
                let mut prefstr = 0 as *mut libc::c_char;
                prefstr = split(comma);
                if !prefstr.is_null() && atoi_check16(prefstr, &mut pref) == 0
                   {
                    strcpy(errstr,
                           b"bad MX preference\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
            }
            name_0 = canonicalise_opt(arg);
            if name_0.is_null() ||
                   !comma.is_null() &&
                       { target = canonicalise_opt(comma); target.is_null() }
               {
                strcpy(errstr,
                       b"bad MX name\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            new_2 =
                opt_malloc(::std::mem::size_of::<mx_srv_record>() as
                               libc::c_ulong) as *mut mx_srv_record;
            (*new_2).next = (*dnsmasq_daemon).mxnames;
            (*dnsmasq_daemon).mxnames = new_2;
            (*new_2).issrv = 0 as libc::c_int;
            (*new_2).name = name_0;
            (*new_2).target = target;
            (*new_2).weight = pref;
            current_block = 7879481898411272068;
        }
        116 => {
            /*  --mx-target */
            (*dnsmasq_daemon).mxtarget = canonicalise_opt(arg);
            if (*dnsmasq_daemon).mxtarget.is_null() {
                strcpy(errstr,
                       b"bad MX target\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_DUMPFILE => {
            /* --dumpfile */
            (*dnsmasq_daemon).dump_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        LOPT_DUMPMASK => {
            /* --dumpmask */
            (*dnsmasq_daemon).dump_mask =
                strtol(arg, NULL_0 as *mut *mut libc::c_char,
                       0 as libc::c_int) as libc::c_int;
            current_block = 7879481898411272068;
        }
        108 => {
            /* --dhcp-leasefile */
            (*dnsmasq_daemon).lease_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        54 | LOPT_LUASCRIPT => {
            /* --dhcp-script */
            /* --dhcp-luascript */
            if option == LOPT_LUASCRIPT {
                strcpy(errstr,
                       b"recompile with HAVE_LUASCRIPT defined to enable Lua scripts\x00"
                           as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            } else {
                (*dnsmasq_daemon).lease_change_command = opt_string_alloc(arg)
            }
            current_block = 7879481898411272068;
        }
        LOPT_DHCP_HOST => {
            /* --dhcp-hostsfile */
            current_block = 12010070245366740438;
        }
        LOPT_DHCP_OPTS => { current_block = 12010070245366740438; }
        LOPT_DHCP_INOTIFY => { current_block = 2812646229686797995; }
        LOPT_DHOPT_INOTIFY => { current_block = 10566976656908717602; }
        LOPT_HOST_INOTIFY => { current_block = 2602045500541335152; }
        72 => { current_block = 4533671380017093834; }
        LOPT_AUTHSERV => {
            /* --auth-server */
            comma = split(arg);
            (*dnsmasq_daemon).authserver = opt_string_alloc(arg);
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut new_4 =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                (*new_4).name = NULL_0 as *mut libc::c_char;
                unhide_metas(arg);
                if inet_pton(AF_INET, arg,
                             &mut (*new_4).addr.in_0.sin_addr as *mut in_addr
                                 as *mut libc::c_void) > 0 as libc::c_int {
                    (*new_4).addr.sa.sa_family = AF_INET as sa_family_t
                } else if inet_pton(AF_INET6, arg,
                                    &mut (*new_4).addr.in6.sin6_addr as
                                        *mut in6_addr as *mut libc::c_void) >
                              0 as libc::c_int {
                    (*new_4).addr.sa.sa_family = AF_INET6 as sa_family_t
                } else {
                    let mut fam = split_chr(arg, '/' as i32 as libc::c_char);
                    (*new_4).name = opt_string_alloc(arg);
                    (*new_4).addr.sa.sa_family =
                        0 as libc::c_int as sa_family_t;
                    if !fam.is_null() {
                        if strcmp(fam,
                                  b"4\x00" as *const u8 as
                                      *const libc::c_char) == 0 as libc::c_int
                           {
                            (*new_4).addr.sa.sa_family =
                                AF_INET as sa_family_t
                        } else if strcmp(fam,
                                         b"6\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 as libc::c_int {
                            (*new_4).addr.sa.sa_family =
                                AF_INET6 as sa_family_t
                        } else {
                            free((*new_4).name as *mut libc::c_void);
                            strcpy(errstr, gen_err);
                            free(new_4 as *mut libc::c_void);
                            return 0 as libc::c_int
                        }
                    }
                }
                (*new_4).next = (*dnsmasq_daemon).authinterface;
                (*dnsmasq_daemon).authinterface = new_4
            }
            current_block = 7879481898411272068;
        }
        LOPT_AUTHSFS => {
            /* --auth-sec-servers */
            let mut new_5 = 0 as *mut name_list;
            loop  {
                comma = split(arg);
                new_5 =
                    opt_malloc(::std::mem::size_of::<name_list>() as
                                   libc::c_ulong) as *mut name_list;
                (*new_5).name = opt_string_alloc(arg);
                (*new_5).next = (*dnsmasq_daemon).secondary_forward_server;
                (*dnsmasq_daemon).secondary_forward_server = new_5;
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        LOPT_AUTHZONE => {
            /* --auth-zone */
            let mut new_6 = 0 as *mut auth_zone;
            comma = split(arg);
            new_6 =
                opt_malloc(::std::mem::size_of::<auth_zone>() as
                               libc::c_ulong) as *mut auth_zone;
            (*new_6).domain = opt_string_alloc(arg);
            (*new_6).subnet = NULL_0 as *mut addrlist;
            (*new_6).exclude = NULL_0 as *mut addrlist;
            (*new_6).interface_names = NULL_0 as *mut auth_name_list;
            (*new_6).next = (*dnsmasq_daemon).auth_zones;
            (*dnsmasq_daemon).auth_zones = new_6;
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut prefixlen = 0 as libc::c_int;
                let mut is_exclude = 0 as libc::c_int;
                let mut prefix = 0 as *mut libc::c_char;
                let mut subnet = NULL_0 as *mut addrlist;
                let mut addr = all_addr{addr4: in_addr{s_addr: 0,},};
                comma = split(arg);
                prefix = split_chr(arg, '/' as i32 as libc::c_char);
                if !prefix.is_null() &&
                       atoi_check(prefix, &mut prefixlen) == 0 {
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                }
                if strstr(arg,
                          b"exclude:\x00" as *const u8 as *const libc::c_char)
                       == arg {
                    is_exclude = 1 as libc::c_int;
                    arg = arg.offset(8 as libc::c_int as isize)
                }
                if inet_pton(AF_INET, arg,
                             &mut addr.addr4 as *mut in_addr as
                                 *mut libc::c_void) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<addrlist>() as
                                       libc::c_ulong) as *mut addrlist;
                    (*subnet).prefixlen =
                        if prefixlen == 0 as libc::c_int {
                            24 as libc::c_int
                        } else { prefixlen };
                    (*subnet).flags = ADDRLIST_LITERAL
                } else if inet_pton(AF_INET6, arg,
                                    &mut addr.addr6 as *mut in6_addr as
                                        *mut libc::c_void) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<addrlist>() as
                                       libc::c_ulong) as *mut addrlist;
                    (*subnet).prefixlen =
                        if prefixlen == 0 as libc::c_int {
                            64 as libc::c_int
                        } else { prefixlen };
                    (*subnet).flags = ADDRLIST_LITERAL | ADDRLIST_IPV6
                } else {
                    let mut name_1 =
                        opt_malloc(::std::mem::size_of::<auth_name_list>() as
                                       libc::c_ulong) as *mut auth_name_list;
                    (*name_1).name = opt_string_alloc(arg);
                    (*name_1).flags = AUTH4 | AUTH6;
                    (*name_1).next = (*new_6).interface_names;
                    (*new_6).interface_names = name_1;
                    if !prefix.is_null() {
                        if prefixlen == 4 as libc::c_int {
                            (*name_1).flags &= !AUTH6
                        } else if prefixlen == 6 as libc::c_int {
                            (*name_1).flags &= !AUTH4
                        } else {
                            strcpy(errstr, gen_err);
                            return 0 as libc::c_int
                        }
                    }
                }
                if !subnet.is_null() {
                    (*subnet).addr = addr;
                    if is_exclude != 0 {
                        (*subnet).next = (*new_6).exclude;
                        (*new_6).exclude = subnet
                    } else {
                        (*subnet).next = (*new_6).subnet;
                        (*new_6).subnet = subnet
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        LOPT_AUTHSOA => {
            /* --auth-soa */
            comma = split(arg);
            (*dnsmasq_daemon).soa_sn = atoi(arg) as u32_0 as libc::c_ulong;
            if !comma.is_null() {
                let mut cp = 0 as *mut libc::c_char;
                arg = comma;
                comma = split(arg);
                (*dnsmasq_daemon).hostmaster = opt_string_alloc(arg);
                cp = (*dnsmasq_daemon).hostmaster;
                while *cp != 0 {
                    if *cp as libc::c_int == '@' as i32 {
                        *cp = '.' as i32 as libc::c_char
                    }
                    cp = cp.offset(1)
                }
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    (*dnsmasq_daemon).soa_refresh =
                        atoi(arg) as u32_0 as libc::c_ulong;
                    if !comma.is_null() {
                        arg = comma;
                        comma = split(arg);
                        (*dnsmasq_daemon).soa_retry =
                            atoi(arg) as u32_0 as libc::c_ulong;
                        if !comma.is_null() {
                            (*dnsmasq_daemon).soa_expiry =
                                atoi(comma) as u32_0 as libc::c_ulong
                        }
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        115 | LOPT_SYNTH => {
            /* --domain */
            /* --synth-domain */
            if strcmp(arg, b"#\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                set_option_bool(OPT_RESOLV_DOMAIN as libc::c_uint);
            } else {
                let mut d = 0 as *mut libc::c_char;
                comma = split(arg);
                d = canonicalise_opt(arg);
                if d.is_null() {
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                } else {
                    if !comma.is_null() {
                        let mut new_7 =
                            opt_malloc(::std::mem::size_of::<cond_domain>() as
                                           libc::c_ulong) as *mut cond_domain;
                        let mut netpart = 0 as *mut libc::c_char;
                        (*new_7).prefix = NULL_0 as *mut libc::c_char;
                        (*new_7).indexed = 0 as libc::c_int;
                        unhide_metas(comma);
                        netpart =
                            split_chr(comma, '/' as i32 as libc::c_char);
                        if !netpart.is_null() {
                            let mut msize: libc::c_int = 0;
                            arg = split(netpart);
                            if atoi_check(netpart, &mut msize) == 0 {
                                strcpy(errstr, gen_err);
                                free(new_7 as *mut libc::c_void);
                                return 0 as libc::c_int
                            } else {
                                if inet_pton(AF_INET, comma,
                                             &mut (*new_7).start as
                                                 *mut in_addr as
                                                 *mut libc::c_void) != 0 {
                                    let mut mask =
                                        ((1 as libc::c_int) <<
                                             32 as libc::c_int - msize) -
                                            1 as libc::c_int;
                                    (*new_7).is6 = 0 as libc::c_int;
                                    (*new_7).start.s_addr =
                                        __bswap_32(__bswap_32((*new_7).start.s_addr)
                                                       &
                                                       !mask as libc::c_uint);
                                    (*new_7).end.s_addr =
                                        (*new_7).start.s_addr |
                                            __bswap_32(mask as __uint32_t);
                                    if !arg.is_null() {
                                        if option != 's' as i32 {
                                            (*new_7).prefix =
                                                canonicalise_opt(arg);
                                            if (*new_7).prefix.is_null() ||
                                                   strlen((*new_7).prefix) >
                                                       (MAXLABEL -
                                                            INET_ADDRSTRLEN)
                                                           as libc::c_ulong {
                                                strcpy(errstr,
                                                       b"bad prefix\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                                free(new_7 as
                                                         *mut libc::c_void);
                                                return 0 as libc::c_int
                                            }
                                        } else if strcmp(arg,
                                                         b"local\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                      != 0 as libc::c_int ||
                                                      msize !=
                                                          8 as libc::c_int &&
                                                          msize !=
                                                              16 as
                                                                  libc::c_int
                                                          &&
                                                          msize !=
                                                              24 as
                                                                  libc::c_int
                                         {
                                            strcpy(errstr, gen_err);
                                            free(new_7 as *mut libc::c_void);
                                            return 0 as libc::c_int
                                        } else {
                                            /* generate the equivalent of
				      local=/xxx.yyy.zzz.in-addr.arpa/ */
                                            let mut serv =
                                                add_rev4((*new_7).start,
                                                         msize);
                                            if serv.is_null() {
                                                strcpy(errstr,
                                                       b"bad prefix\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                                free(new_7 as
                                                         *mut libc::c_void);
                                                return 0 as libc::c_int
                                            }
                                            (*serv).flags |= SERV_NO_ADDR;
                                            /* local=/<domain>/ */
                                            serv =
                                                opt_malloc(::std::mem::size_of::<server>()
                                                               as
                                                               libc::c_ulong)
                                                    as *mut server;
                                            memset(serv as *mut libc::c_void,
                                                   0 as libc::c_int,
                                                   ::std::mem::size_of::<server>()
                                                       as libc::c_ulong);
                                            (*serv).domain = d;
                                            (*serv).flags =
                                                SERV_HAS_DOMAIN |
                                                    SERV_NO_ADDR;
                                            (*serv).next =
                                                (*dnsmasq_daemon).servers;
                                            (*dnsmasq_daemon).servers = serv
                                        }
                                    }
                                } else if inet_pton(AF_INET6, comma,
                                                    &mut (*new_7).start6 as
                                                        *mut in6_addr as
                                                        *mut libc::c_void) !=
                                              0 {
                                    let mut mask_0 =
                                        ((1 as libc::c_ulonglong) <<
                                             128 as libc::c_int -
                                                 msize).wrapping_sub(1 as
                                                                         libc::c_ulonglong);
                                    let mut addrpart =
                                        addr6part(&mut (*new_7).start6);
                                    (*new_7).is6 = 1 as libc::c_int;
                                    /* prefix==64 overflows the mask calculation above */
                                    if msize == 64 as libc::c_int {
                                        mask_0 =
                                            -(1 as libc::c_longlong) as u64_0
                                    }
                                    (*new_7).end6 = (*new_7).start6;
                                    setaddr6part(&mut (*new_7).start6,
                                                 addrpart & !mask_0);
                                    setaddr6part(&mut (*new_7).end6,
                                                 addrpart | mask_0);
                                    if msize < 64 as libc::c_int {
                                        strcpy(errstr, gen_err);
                                        free(new_7 as *mut libc::c_void);
                                        return 0 as libc::c_int
                                    } else {
                                        if !arg.is_null() {
                                            if option != 's' as i32 {
                                                (*new_7).prefix =
                                                    canonicalise_opt(arg);
                                                if (*new_7).prefix.is_null()
                                                       ||
                                                       strlen((*new_7).prefix)
                                                           >
                                                           (MAXLABEL -
                                                                INET6_ADDRSTRLEN)
                                                               as
                                                               libc::c_ulong {
                                                    strcpy(errstr,
                                                           b"bad prefix\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                                    free(new_7 as
                                                             *mut libc::c_void);
                                                    return 0 as libc::c_int
                                                }
                                            } else if strcmp(arg,
                                                             b"local\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)
                                                          != 0 as libc::c_int
                                                          ||
                                                          msize &
                                                              4 as libc::c_int
                                                              !=
                                                              0 as libc::c_int
                                             {
                                                strcpy(errstr, gen_err);
                                                free(new_7 as
                                                         *mut libc::c_void);
                                                return 0 as libc::c_int
                                            } else {
                                                /* generate the equivalent of
				     local=/xxx.yyy.zzz.ip6.arpa/ */
                                                let mut serv_0 =
                                                    add_rev6(&mut (*new_7).start6,
                                                             msize);
                                                (*serv_0).flags |=
                                                    SERV_NO_ADDR;
                                                /* local=/<domain>/ */
                                                serv_0 =
                                                    opt_malloc(::std::mem::size_of::<server>()
                                                                   as
                                                                   libc::c_ulong)
                                                        as *mut server;
                                                memset(serv_0 as
                                                           *mut libc::c_void,
                                                       0 as libc::c_int,
                                                       ::std::mem::size_of::<server>()
                                                           as libc::c_ulong);
                                                (*serv_0).domain = d;
                                                (*serv_0).flags =
                                                    SERV_HAS_DOMAIN |
                                                        SERV_NO_ADDR;
                                                (*serv_0).next =
                                                    (*dnsmasq_daemon).servers;
                                                (*dnsmasq_daemon).servers =
                                                    serv_0
                                            }
                                        }
                                    }
                                } else {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0 as libc::c_int
                                }
                            }
                        } else {
                            let mut prefstr_0 = 0 as *mut libc::c_char;
                            arg = split(comma);
                            prefstr_0 = split(arg);
                            if inet_pton(AF_INET, comma,
                                         &mut (*new_7).start as *mut in_addr
                                             as *mut libc::c_void) != 0 {
                                (*new_7).is6 = 0 as libc::c_int;
                                if arg.is_null() {
                                    (*new_7).end.s_addr =
                                        (*new_7).start.s_addr
                                } else if inet_pton(AF_INET, arg,
                                                    &mut (*new_7).end as
                                                        *mut in_addr as
                                                        *mut libc::c_void) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0 as libc::c_int
                                }
                            } else if inet_pton(AF_INET6, comma,
                                                &mut (*new_7).start6 as
                                                    *mut in6_addr as
                                                    *mut libc::c_void) != 0 {
                                (*new_7).is6 = 1 as libc::c_int;
                                if arg.is_null() {
                                    memcpy(&mut (*new_7).end6 as *mut in6_addr
                                               as *mut libc::c_void,
                                           &mut (*new_7).start6 as
                                               *mut in6_addr as
                                               *const libc::c_void,
                                           IN6ADDRSZ as libc::c_ulong);
                                } else if inet_pton(AF_INET6, arg,
                                                    &mut (*new_7).end6 as
                                                        *mut in6_addr as
                                                        *mut libc::c_void) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0 as libc::c_int
                                }
                            } else {
                                strcpy(errstr, gen_err);
                                free(new_7 as *mut libc::c_void);
                                return 0 as libc::c_int
                            }
                            if option != 's' as i32 && !prefstr_0.is_null() {
                                (*new_7).prefix = canonicalise_opt(prefstr_0);
                                if (*new_7).prefix.is_null() ||
                                       strlen((*new_7).prefix) >
                                           (MAXLABEL - INET_ADDRSTRLEN) as
                                               libc::c_ulong {
                                    strcpy(errstr,
                                           b"bad prefix\x00" as *const u8 as
                                               *const libc::c_char);
                                    free(new_7 as *mut libc::c_void);
                                    return 0 as libc::c_int
                                }
                            }
                        }
                        (*new_7).domain = d;
                        if option == 's' as i32 {
                            (*new_7).next = (*dnsmasq_daemon).cond_domain;
                            (*dnsmasq_daemon).cond_domain = new_7
                        } else {
                            let mut star = 0 as *mut libc::c_char;
                            (*new_7).next = (*dnsmasq_daemon).synth_domains;
                            (*dnsmasq_daemon).synth_domains = new_7;
                            if !(*new_7).prefix.is_null() &&
                                   {
                                       star =
                                           strrchr((*new_7).prefix,
                                                   '*' as i32);
                                       !star.is_null()
                                   } &&
                                   *star.offset(1 as libc::c_int as isize) as
                                       libc::c_int == 0 as libc::c_int {
                                *star = 0 as libc::c_int as libc::c_char;
                                (*new_7).indexed = 1 as libc::c_int
                            }
                        }
                    } else if option == 's' as i32 {
                        (*dnsmasq_daemon).domain_suffix = d
                    } else {
                        strcpy(errstr, gen_err);
                        return 0 as libc::c_int
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        LOPT_CPE_ID => {
            /* --add-dns-client */
            if !arg.is_null() {
                (*dnsmasq_daemon).dns_client_id = opt_string_alloc(arg)
            }
            current_block = 7879481898411272068;
        }
        LOPT_ADD_MAC => {
            /* --add-mac */
            if arg.is_null() {
                set_option_bool(OPT_ADD_MAC as libc::c_uint);
            } else {
                unhide_metas(arg);
                if strcmp(arg,
                          b"base64\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    set_option_bool(OPT_MAC_B64 as libc::c_uint);
                } else if strcmp(arg,
                                 b"text\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    set_option_bool(OPT_MAC_HEX as libc::c_uint);
                } else { strcpy(errstr, gen_err); return 0 as libc::c_int }
            }
            current_block = 7879481898411272068;
        }
        117 => {
            /* --user */
            (*dnsmasq_daemon).username = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        103 => {
            /* --group */
            (*dnsmasq_daemon).groupname = opt_string_alloc(arg);
            (*dnsmasq_daemon).group_set = 1 as libc::c_int;
            current_block = 7879481898411272068;
        }
        LOPT_SCRIPTUSR => {
            /* --scriptuser */
            (*dnsmasq_daemon).scriptuser = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        105 => {
            loop 
                 /* --interface */
                 {
                let mut new_8 =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                (*new_8).next = (*dnsmasq_daemon).if_names;
                (*dnsmasq_daemon).if_names = new_8;
                /* new->name may be NULL if someone does
	   "interface=" to disable all interfaces except loop. */
                (*new_8).name = opt_string_alloc(arg);
                (*new_8).used = 0 as libc::c_int;
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        LOPT_TFTP => {
            /* --enable-tftp */
            set_option_bool(OPT_TFTP as libc::c_uint);
            if arg.is_null() {
                current_block = 7879481898411272068;
            } else {
                /* fall through */
                current_block = 887445304002143054;
            }
        }
        73 | 50 => { current_block = 887445304002143054; }
        66 => {
            /* --bogus-nxdomain */
            current_block = 2926860427235594157;
        }
        LOPT_IGNORE_ADDR => { current_block = 2926860427235594157; }
        97 | LOPT_AUTHPEER => {
            /* --listen-address */
            loop 
                 /* --auth-peer */
                 {
                let mut new_10 =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                unhide_metas(arg);
                if !arg.is_null() &&
                       inet_pton(AF_INET, arg,
                                 &mut (*new_10).addr.in_0.sin_addr as
                                     *mut in_addr as *mut libc::c_void) >
                           0 as libc::c_int {
                    (*new_10).addr.sa.sa_family = AF_INET as sa_family_t;
                    (*new_10).addr.in_0.sin_port =
                        0 as libc::c_int as in_port_t
                } else if !arg.is_null() &&
                              inet_pton(AF_INET6, arg,
                                        &mut (*new_10).addr.in6.sin6_addr as
                                            *mut in6_addr as
                                            *mut libc::c_void) >
                                  0 as libc::c_int {
                    (*new_10).addr.sa.sa_family = AF_INET6 as sa_family_t;
                    (*new_10).addr.in6.sin6_flowinfo =
                        0 as libc::c_int as uint32_t;
                    (*new_10).addr.in6.sin6_scope_id =
                        0 as libc::c_int as uint32_t;
                    (*new_10).addr.in6.sin6_port =
                        0 as libc::c_int as in_port_t
                } else {
                    strcpy(errstr, gen_err);
                    free(new_10 as *mut libc::c_void);
                    return 0 as libc::c_int
                }
                (*new_10).used = 0 as libc::c_int;
                if option == 'a' as i32 {
                    (*new_10).next = (*dnsmasq_daemon).if_addrs;
                    (*dnsmasq_daemon).if_addrs = new_10
                } else {
                    (*new_10).next = (*dnsmasq_daemon).auth_peers;
                    (*dnsmasq_daemon).auth_peers = new_10
                }
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        83 => {
            /*  --server */
            current_block = 9676380469790025234;
        }
        LOPT_LOCAL => { current_block = 9676380469790025234; }
        65 => { current_block = 6480954168551069607; }
        LOPT_NO_REBIND => { current_block = 14399141444697241811; }
        LOPT_REV_SERV => {
            /* --rev-server */
            let mut string = 0 as *mut libc::c_char;
            let mut size: libc::c_int = 0;
            let mut serv_2 = 0 as *mut server;
            let mut addr4 = in_addr{s_addr: 0,};
            let mut addr6 =
                in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            unhide_metas(arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            comma = split(arg);
            string = split_chr(arg, '/' as i32 as libc::c_char);
            if string.is_null() || atoi_check(string, &mut size) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            if inet_pton(AF_INET, arg,
                         &mut addr4 as *mut in_addr as *mut libc::c_void) != 0
               {
                serv_2 = add_rev4(addr4, size);
                if serv_2.is_null() {
                    strcpy(errstr,
                           b"bad prefix\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
            } else if inet_pton(AF_INET6, arg,
                                &mut addr6 as *mut in6_addr as
                                    *mut libc::c_void) != 0 {
                serv_2 = add_rev6(&mut addr6, size)
            } else { strcpy(errstr, gen_err); return 0 as libc::c_int }
            string =
                parse_server(comma, &mut (*serv_2).addr,
                             &mut (*serv_2).source_addr,
                             (*serv_2).interface.as_mut_ptr(),
                             &mut (*serv_2).flags);
            if !string.is_null() {
                strcpy(errstr, string);
                return 0 as libc::c_int
            }
            if servers_only != 0 { (*serv_2).flags |= SERV_FROM_FILE }
            current_block = 7879481898411272068;
        }
        LOPT_IPSET => {
            /* --ipset */
            let mut ipsets_head =
                ipsets{sets: 0 as *mut *mut libc::c_char,
                       domain: 0 as *mut libc::c_char,
                       next: 0 as *mut ipsets,};
            let mut ipsets: *mut ipsets = &mut ipsets_head;
            let mut size_0: libc::c_int = 0;
            let mut end_1 = 0 as *mut libc::c_char;
            let mut sets = 0 as *mut *mut libc::c_char;
            let mut sets_pos = 0 as *mut *mut libc::c_char;
            memset(ipsets as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<ipsets>() as libc::c_ulong);
            unhide_metas(arg);
            if !arg.is_null() && *arg as libc::c_int == '/' as i32 {
                arg = arg.offset(1);
                loop  {
                    end_1 = split_chr(arg, '/' as i32 as libc::c_char);
                    if end_1.is_null() { break ; }
                    let mut domain_0 = NULL_0 as *mut libc::c_char;
                    /* elide leading dots - they are implied in the search algorithm */
                    while *arg as libc::c_int == '.' as i32 {
                        arg = arg.offset(1)
                    }
                    /* # matches everything and becomes a zero length domain string */
                    if strcmp(arg,
                              b"#\x00" as *const u8 as *const libc::c_char) ==
                           0 as libc::c_int || *arg == 0 {
                        domain_0 =
                            b"\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char
                    } else if strlen(arg) != 0 as libc::c_int as libc::c_ulong
                                  &&
                                  {
                                      domain_0 = canonicalise_opt(arg);
                                      domain_0.is_null()
                                  } {
                        strcpy(errstr, gen_err);
                        return 0 as libc::c_int
                    }
                    (*ipsets).next =
                        opt_malloc(::std::mem::size_of::<ipsets>() as
                                       libc::c_ulong) as *mut ipsets;
                    ipsets = (*ipsets).next;
                    memset(ipsets as *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<ipsets>() as libc::c_ulong);
                    (*ipsets).domain = domain_0;
                    arg = end_1
                }
            } else {
                (*ipsets).next =
                    opt_malloc(::std::mem::size_of::<ipsets>() as
                                   libc::c_ulong) as *mut ipsets;
                ipsets = (*ipsets).next;
                memset(ipsets as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<ipsets>() as libc::c_ulong);
                (*ipsets).domain =
                    b"\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            if arg.is_null() || *arg == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            size_0 = 2 as libc::c_int;
            end_1 = arg;
            while *end_1 != 0 {
                if *end_1 as libc::c_int == ',' as i32 { size_0 += 1 }
                end_1 = end_1.offset(1)
            }
            sets_pos =
                opt_malloc((::std::mem::size_of::<*mut libc::c_char>() as
                                libc::c_ulong).wrapping_mul(size_0 as
                                                                libc::c_ulong))
                    as *mut *mut libc::c_char;
            sets = sets_pos;
            loop  {
                end_1 = split(arg);
                let fresh27 = sets_pos;
                sets_pos = sets_pos.offset(1);
                *fresh27 = opt_string_alloc(arg);
                arg = end_1;
                if end_1.is_null() { break ; }
            }
            *sets_pos = 0 as *mut libc::c_char;
            ipsets = &mut ipsets_head;
            while !(*ipsets).next.is_null() {
                (*(*ipsets).next).sets = sets;
                ipsets = (*ipsets).next
            }
            (*ipsets).next = (*dnsmasq_daemon).ipsets;
            (*dnsmasq_daemon).ipsets = ipsets_head.next;
            current_block = 7879481898411272068;
        }
        99 => {
            /* --cache-size */
            let mut size_1: libc::c_int = 0;
            if atoi_check(arg, &mut size_1) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                /* zero is OK, and means no caching. */
                if size_1 < 0 as libc::c_int { size_1 = 0 as libc::c_int }
                /* Note that for very large cache sizes, the malloc()
	       will overflow. For the size of the cache record
	       at the time this was noted, the value of "very large"
               was 46684428. Limit to an order of magnitude less than
	       that to be safe from changes to the cache record. */
                if size_1 > 5000000 as libc::c_int {
                    size_1 = 5000000 as libc::c_int
                }
                (*dnsmasq_daemon).cachesize = size_1
            }
            current_block = 7879481898411272068;
        }
        112 => {
            /* --port */
            if atoi_check16(arg, &mut (*dnsmasq_daemon).port) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_MINPORT => {
            /* --min-port */
            if atoi_check16(arg, &mut (*dnsmasq_daemon).min_port) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_MAXPORT => {
            /* --max-port */
            if atoi_check16(arg, &mut (*dnsmasq_daemon).max_port) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        48 => {
            /* --dns-forward-max */
            if atoi_check(arg, &mut (*dnsmasq_daemon).ftabsize) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        113 => {
            /* --log-queries */
            set_option_bool(OPT_LOG as libc::c_uint); /* default */
            if !arg.is_null() &&
                   strcmp(arg,
                          b"extra\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                set_option_bool(OPT_EXTRALOG as libc::c_uint);
            }
            current_block = 7879481898411272068;
        }
        LOPT_MAX_LOGS => {
            /* --log-async */
            (*dnsmasq_daemon).max_logs = LOG_MAX;
            if !arg.is_null() &&
                   atoi_check(arg, &mut (*dnsmasq_daemon).max_logs) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                if (*dnsmasq_daemon).max_logs > 100 as libc::c_int {
                    (*dnsmasq_daemon).max_logs = 100 as libc::c_int
                }
            }
            current_block = 7879481898411272068;
        }
        80 => {
            /* --edns-packet-max */
            let mut i_0: libc::c_int = 0;
            if atoi_check(arg, &mut i_0) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            (*dnsmasq_daemon).edns_pktsz = i_0 as libc::c_ushort;
            current_block = 7879481898411272068;
        }
        81 => {
            /* --query-port */
            if atoi_check16(arg, &mut (*dnsmasq_daemon).query_port) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            /* if explicitly set to zero, use single OS ephemeral port
	 and disable random ports */
            if (*dnsmasq_daemon).query_port == 0 as libc::c_int {
                (*dnsmasq_daemon).osport = 1 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        84 => {
            /* --local-ttl */
            current_block = 15489771604880449635;
        }
        LOPT_NEGTTL => { current_block = 15489771604880449635; }
        LOPT_MAXTTL => { current_block = 6082976577402880686; }
        LOPT_MINCTTL => { current_block = 16916584745428150692; }
        LOPT_MAXCTTL => { current_block = 13094692781038244044; }
        LOPT_AUTHTTL => { current_block = 13035992208579083528; }
        LOPT_DHCPTTL => { current_block = 5893551302610724882; }
        88 => {
            /* --dhcp-lease-max */
            if atoi_check(arg, &mut (*dnsmasq_daemon).dhcp_max) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_TFTP_MAX => {
            /*  --tftp-max */
            if atoi_check(arg, &mut (*dnsmasq_daemon).tftp_max) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_TFTP_MTU => {
            /*  --tftp-mtu */
            if atoi_check(arg, &mut (*dnsmasq_daemon).tftp_mtu) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_PREFIX => {
            /* --tftp-prefix */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_11 =
                    opt_malloc(::std::mem::size_of::<tftp_prefix>() as
                                   libc::c_ulong) as *mut tftp_prefix;
                (*new_11).interface = opt_string_alloc(comma);
                (*new_11).prefix = opt_string_alloc(arg);
                (*new_11).next = (*dnsmasq_daemon).if_prefix;
                (*dnsmasq_daemon).if_prefix = new_11
            } else { (*dnsmasq_daemon).tftp_prefix = opt_string_alloc(arg) }
            current_block = 7879481898411272068;
        }
        LOPT_TFTPPORTS => {
            /* --tftp-port-range */
            comma = split(arg);
            if comma.is_null() ||
                   atoi_check16(arg, &mut (*dnsmasq_daemon).start_tftp_port)
                       == 0 ||
                   atoi_check16(comma, &mut (*dnsmasq_daemon).end_tftp_port)
                       == 0 {
                strcpy(errstr,
                       b"bad port range\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            if (*dnsmasq_daemon).start_tftp_port >
                   (*dnsmasq_daemon).end_tftp_port {
                let mut tmp = (*dnsmasq_daemon).start_tftp_port;
                (*dnsmasq_daemon).start_tftp_port =
                    (*dnsmasq_daemon).end_tftp_port;
                (*dnsmasq_daemon).end_tftp_port = tmp
            }
            current_block = 7879481898411272068;
        }
        LOPT_APREF => {
            /* --tftp-unique-root */
            if arg.is_null() ||
                   strcasecmp(arg,
                              b"ip\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                set_option_bool(OPT_TFTP_APREF_IP as libc::c_uint);
            } else if strcasecmp(arg,
                                 b"mac\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
             {
                set_option_bool(OPT_TFTP_APREF_MAC as libc::c_uint);
            } else { strcpy(errstr, gen_err); return 0 as libc::c_int }
            current_block = 7879481898411272068;
        }
        LOPT_BRIDGE => {
            /* --bridge-interface */
            let mut new_12 = 0 as *mut dhcp_bridge;
            comma = split(arg);
            if comma.is_null() ||
                   strlen(arg) >
                       (IF_NAMESIZE - 1 as libc::c_int) as libc::c_ulong {
                strcpy(errstr,
                       b"bad bridge-interface\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            new_12 = (*dnsmasq_daemon).bridges;
            while !new_12.is_null() {
                if strcmp((*new_12).iface.as_mut_ptr(), arg) ==
                       0 as libc::c_int {
                    break ;
                }
                new_12 = (*new_12).next
            }
            if new_12.is_null() {
                new_12 =
                    opt_malloc(::std::mem::size_of::<dhcp_bridge>() as
                                   libc::c_ulong) as *mut dhcp_bridge;
                strcpy((*new_12).iface.as_mut_ptr(), arg);
                (*new_12).alias = NULL_0 as *mut dhcp_bridge;
                (*new_12).next = (*dnsmasq_daemon).bridges;
                (*dnsmasq_daemon).bridges = new_12
            }
            loop  {
                arg = comma;
                comma = split(arg);
                if strlen(arg) != 0 as libc::c_int as libc::c_ulong &&
                       strlen(arg) <=
                           (IF_NAMESIZE - 1 as libc::c_int) as libc::c_ulong {
                    let mut b =
                        opt_malloc(::std::mem::size_of::<dhcp_bridge>() as
                                       libc::c_ulong) as *mut dhcp_bridge;
                    (*b).next = (*new_12).alias;
                    (*new_12).alias = b;
                    strcpy((*b).iface.as_mut_ptr(), arg);
                }
                if comma.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        LOPT_SHARED_NET => {
            /* --shared-network */
            let mut new_13 =
                opt_malloc(::std::mem::size_of::<shared_network>() as
                               libc::c_ulong) as *mut shared_network;
            (*new_13).shared_addr.s_addr = 0 as libc::c_int as in_addr_t;
            (*new_13).if_index = 0 as libc::c_int;
            comma = split(arg);
            if comma.is_null() {
                current_block = 3177757304694473968;
            } else {
                if inet_pton(AF_INET, comma,
                             &mut (*new_13).shared_addr as *mut in_addr as
                                 *mut libc::c_void) != 0 {
                    if inet_pton(AF_INET, arg,
                                 &mut (*new_13).match_addr as *mut in_addr as
                                     *mut libc::c_void) == 0 &&
                           {
                               (*new_13).if_index =
                                   if_nametoindex(arg) as libc::c_int;
                               ((*new_13).if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else if inet_pton(AF_INET6, comma,
                                    &mut (*new_13).shared_addr6 as
                                        *mut in6_addr as *mut libc::c_void) !=
                              0 {
                    if inet_pton(AF_INET6, arg,
                                 &mut (*new_13).match_addr6 as *mut in6_addr
                                     as *mut libc::c_void) == 0 &&
                           {
                               (*new_13).if_index =
                                   if_nametoindex(arg) as libc::c_int;
                               ((*new_13).if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else { current_block = 3177757304694473968; }
                match current_block {
                    3177757304694473968 => { }
                    _ => {
                        (*new_13).next = (*dnsmasq_daemon).shared_networks;
                        (*dnsmasq_daemon).shared_networks = new_13;
                        current_block = 7879481898411272068;
                    }
                }
            }
            match current_block {
                7879481898411272068 => { }
                _ => {
                    free(new_13 as *mut libc::c_void);
                    strcpy(errstr,
                           b"bad shared-network\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
            }
        }
        70 => {
            /* --dhcp-range */
            let mut k: libc::c_int = 0;
            let mut leasepos = 2 as libc::c_int;
            let mut cp_0 = 0 as *mut libc::c_char;
            let mut a: [*mut libc::c_char; 8] =
                [NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char];
            let mut new_14 =
                opt_malloc(::std::mem::size_of::<dhcp_context>() as
                               libc::c_ulong) as *mut dhcp_context;
            memset(new_14 as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<dhcp_context>() as libc::c_ulong);
            loop  {
                cp_0 = arg;
                while *cp_0 != 0 {
                    if !(*cp_0 as libc::c_int == ' ' as i32 ||
                             *cp_0 as libc::c_int == '.' as i32 ||
                             *cp_0 as libc::c_int == ':' as i32 ||
                             *cp_0 as libc::c_int >= 'a' as i32 &&
                                 *cp_0 as libc::c_int <= 'f' as i32 ||
                             *cp_0 as libc::c_int >= 'A' as i32 &&
                                 *cp_0 as libc::c_int <= 'F' as i32 ||
                             *cp_0 as libc::c_int >= '0' as i32 &&
                                 *cp_0 as libc::c_int <= '9' as i32) {
                        break ;
                    }
                    cp_0 = cp_0.offset(1)
                }
                if *cp_0 as libc::c_int != ',' as i32 &&
                       { comma = split(arg); !comma.is_null() } {
                    if is_tag_prefix(arg) != 0 {
                        /* ignore empty tag */
                        if *arg.offset(4 as libc::c_int as isize) != 0 {
                            (*new_14).filter =
                                dhcp_netid_create(arg.offset(4 as libc::c_int
                                                                 as isize),
                                                  (*new_14).filter)
                        }
                    } else if !(*new_14).netid.net.is_null() {
                        dhcp_context_free(new_14); /* default */
                        strcpy(errstr,
                               b"only one tag allowed\x00" as *const u8 as
                                   *const libc::c_char);
                        return 0 as libc::c_int
                    } else {
                        (*new_14).netid.net =
                            opt_string_alloc(set_prefix(arg))
                    }
                    arg = comma
                } else { a[0 as libc::c_int as usize] = arg; break ; }
            }
            k = 1 as libc::c_int;
            while k < 8 as libc::c_int {
                a[k as usize] = split(a[(k - 1 as libc::c_int) as usize]);
                if a[k as usize].is_null() { break ; }
                k += 1
            }
            if k < 2 as libc::c_int {
                dhcp_context_free(new_14);
                strcpy(errstr,
                       b"bad dhcp-range\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            if inet_pton(AF_INET, a[0 as libc::c_int as usize],
                         &mut (*new_14).start as *mut in_addr as
                             *mut libc::c_void) != 0 {
                (*new_14).next = (*dnsmasq_daemon).dhcp;
                (*new_14).lease_time = DEFLEASE as libc::c_uint;
                (*dnsmasq_daemon).dhcp = new_14;
                (*new_14).end = (*new_14).start;
                if strcmp(a[1 as libc::c_int as usize],
                          b"static\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint | CONTEXT_STATIC) as
                            libc::c_int
                } else if strcmp(a[1 as libc::c_int as usize],
                                 b"proxy\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint | CONTEXT_PROXY) as
                            libc::c_int
                } else if inet_pton(AF_INET, a[1 as libc::c_int as usize],
                                    &mut (*new_14).end as *mut in_addr as
                                        *mut libc::c_void) == 0 {
                    dhcp_context_free(new_14);
                    strcpy(errstr,
                           b"bad dhcp-range\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if __bswap_32((*new_14).start.s_addr) >
                       __bswap_32((*new_14).end.s_addr) {
                    let mut tmp_0 = (*new_14).start;
                    (*new_14).start = (*new_14).end;
                    (*new_14).end = tmp_0
                }
                if k >= 3 as libc::c_int &&
                       !strchr(a[2 as libc::c_int as usize],
                               '.' as i32).is_null() &&
                       inet_pton(AF_INET, a[2 as libc::c_int as usize],
                                 &mut (*new_14).netmask as *mut in_addr as
                                     *mut libc::c_void) > 0 as libc::c_int {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint | CONTEXT_NETMASK) as
                            libc::c_int;
                    leasepos = 3 as libc::c_int;
                    if is_same_net((*new_14).start, (*new_14).end,
                                   (*new_14).netmask) == 0 {
                        dhcp_context_free(new_14);
                        strcpy(errstr,
                               b"inconsistent DHCP range\x00" as *const u8 as
                                   *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    if k >= 4 as libc::c_int &&
                           !strchr(a[3 as libc::c_int as usize],
                                   '.' as i32).is_null() &&
                           inet_pton(AF_INET, a[3 as libc::c_int as usize],
                                     &mut (*new_14).broadcast as *mut in_addr
                                         as *mut libc::c_void) >
                               0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 CONTEXT_BRDCAST) as libc::c_int;
                        leasepos = 4 as libc::c_int
                    }
                }
            } else if inet_pton(AF_INET6, a[0 as libc::c_int as usize],
                                &mut (*new_14).start6 as *mut in6_addr as
                                    *mut libc::c_void) != 0 {
                let mut err_1 = NULL_0 as *const libc::c_char;
                (*new_14).flags =
                    ((*new_14).flags as libc::c_uint | CONTEXT_V6) as
                        libc::c_int;
                (*new_14).prefix = 64 as libc::c_int;
                (*new_14).end6 = (*new_14).start6;
                (*new_14).lease_time = DEFLEASE6 as libc::c_uint;
                (*new_14).next = (*dnsmasq_daemon).dhcp6;
                (*dnsmasq_daemon).dhcp6 = new_14;
                leasepos = 1 as libc::c_int;
                while leasepos < k {
                    if strcmp(a[leasepos as usize],
                              b"static\x00" as *const u8 as
                                  *const libc::c_char) == 0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (CONTEXT_STATIC | CONTEXT_DHCP)) as
                                libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-only\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int ||
                                  strcmp(a[leasepos as usize],
                                         b"slaac\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint | CONTEXT_RA) as
                                libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-names\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (CONTEXT_RA_NAME | CONTEXT_RA)) as
                                libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-advrouter\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (CONTEXT_RA_ROUTER | CONTEXT_RA)) as
                                libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-stateless\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (CONTEXT_RA_STATELESS | CONTEXT_DHCP |
                                      CONTEXT_RA)) as libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"off-link\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 CONTEXT_RA_OFF_LINK) as libc::c_int
                    } else if leasepos == 1 as libc::c_int &&
                                  inet_pton(AF_INET6, a[leasepos as usize],
                                            &mut (*new_14).end6 as
                                                *mut in6_addr as
                                                *mut libc::c_void) != 0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint | CONTEXT_DHCP)
                                as libc::c_int
                    } else {
                        if !(strstr(a[leasepos as usize],
                                    b"constructor:\x00" as *const u8 as
                                        *const libc::c_char) ==
                                 a[leasepos as usize]) {
                            break ;
                        }
                        (*new_14).template_interface =
                            opt_string_alloc(a[leasepos as
                                                   usize].offset(12 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 CONTEXT_TEMPLATE) as libc::c_int
                    }
                    leasepos += 1
                }
                /* bare integer < 128 is prefix value */
                if leasepos < k {
                    let mut pref_0: libc::c_int = 0;
                    cp_0 = a[leasepos as usize];
                    while *cp_0 != 0 {
                        if !(*cp_0 as libc::c_int >= '0' as i32 &&
                                 *cp_0 as libc::c_int <= '9' as i32) {
                            break ;
                        }
                        cp_0 = cp_0.offset(1)
                    }
                    if *cp_0 == 0 &&
                           {
                               pref_0 = atoi(a[leasepos as usize]);
                               (pref_0) <= 128 as libc::c_int
                           } {
                        (*new_14).prefix = pref_0;
                        leasepos += 1
                    }
                }
                if (*new_14).prefix > 64 as libc::c_int {
                    if (*new_14).flags as libc::c_uint & CONTEXT_RA != 0 {
                        err_1 =
                            b"prefix length must be exactly 64 for RA subnets\x00"
                                as *const u8 as *const libc::c_char
                    } else if (*new_14).flags as libc::c_uint &
                                  CONTEXT_TEMPLATE != 0 {
                        err_1 =
                            b"prefix length must be exactly 64 for subnet constructors\x00"
                                as *const u8 as *const libc::c_char
                    }
                } else if (*new_14).prefix < 64 as libc::c_int {
                    err_1 =
                        b"prefix length must be at least 64\x00" as *const u8
                            as *const libc::c_char
                }
                if err_1.is_null() &&
                       is_same_net6(&mut (*new_14).start6,
                                    &mut (*new_14).end6, (*new_14).prefix) ==
                           0 {
                    err_1 =
                        b"inconsistent DHCPv6 range\x00" as *const u8 as
                            *const libc::c_char
                }
                if !err_1.is_null() {
                    dhcp_context_free(new_14);
                    strcpy(errstr, err_1);
                    return 0 as libc::c_int
                }
                /* dhcp-range=:: enables DHCP stateless on any interface */
                if ({
                        let mut __a =
                            &mut (*new_14).start6 as *mut in6_addr as
                                *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint) as
                            libc::c_int
                    }) != 0 &&
                       (*new_14).flags as libc::c_uint & CONTEXT_TEMPLATE == 0
                   {
                    (*new_14).prefix = 0 as libc::c_int
                }
                if (*new_14).flags as libc::c_uint & CONTEXT_TEMPLATE != 0 {
                    let mut zero =
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    memset(&mut zero as *mut in6_addr as *mut libc::c_void,
                           0 as libc::c_int,
                           ::std::mem::size_of::<in6_addr>() as
                               libc::c_ulong);
                    if is_same_net6(&mut zero, &mut (*new_14).start6,
                                    (*new_14).prefix) == 0 {
                        dhcp_context_free(new_14);
                        strcpy(errstr,
                               b"prefix must be zero with \"constructor:\" argument\x00"
                                   as *const u8 as *const libc::c_char);
                        return 0 as libc::c_int
                    }
                }
                if addr6part(&mut (*new_14).start6) >
                       addr6part(&mut (*new_14).end6) {
                    let mut tmp_1 = (*new_14).start6;
                    (*new_14).start6 = (*new_14).end6;
                    (*new_14).end6 = tmp_1
                }
            } else {
                dhcp_context_free(new_14);
                strcpy(errstr,
                       b"bad dhcp-range\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            if leasepos < k {
                if leasepos != k - 1 as libc::c_int {
                    dhcp_context_free(new_14);
                    strcpy(errstr,
                           b"bad dhcp-range\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if strcmp(a[leasepos as usize],
                          b"infinite\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    (*new_14).lease_time = 0xffffffff as libc::c_uint;
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint | CONTEXT_SETLEASE)
                            as libc::c_int
                } else if strcmp(a[leasepos as usize],
                                 b"deprecated\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint | CONTEXT_DEPRECATE)
                            as libc::c_int
                } else {
                    let mut fac = 1 as libc::c_int;
                    if strlen(a[leasepos as usize]) >
                           0 as libc::c_int as libc::c_ulong {
                        let mut current_block_1049: u64;
                        match *a[leasepos as
                                     usize].offset(strlen(a[leasepos as
                                                                usize]).wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                       as isize) as
                                  libc::c_int {
                            119 | 87 => {
                                fac *= 7 as libc::c_int;
                                current_block_1049 = 9610247714461258384;
                            }
                            100 | 68 => {
                                current_block_1049 = 9610247714461258384;
                            }
                            104 | 72 => {
                                current_block_1049 = 9280197982685904555;
                            }
                            109 | 77 => {
                                current_block_1049 = 17378754114849407475;
                            }
                            115 | 83 => {
                                current_block_1049 = 8582955123963743225;
                            }
                            _ => { current_block_1049 = 8758648760486203175; }
                        }
                        match current_block_1049 {
                            9610247714461258384 =>
                            /* fall through */
                            {
                                fac *= 24 as libc::c_int;
                                current_block_1049 = 9280197982685904555;
                            }
                            _ => { }
                        }
                        match current_block_1049 {
                            9280197982685904555 =>
                            /* fall through */
                            {
                                fac *= 60 as libc::c_int;
                                current_block_1049 = 17378754114849407475;
                            }
                            _ => { }
                        }
                        match current_block_1049 {
                            17378754114849407475 =>
                            /* fall through */
                            {
                                fac *= 60 as libc::c_int;
                                current_block_1049 = 8582955123963743225;
                            }
                            _ => { }
                        }
                        match current_block_1049 {
                            8582955123963743225 =>
                            /* fall through */
                            {
                                *a[leasepos as
                                       usize].offset(strlen(a[leasepos as
                                                                  usize]).wrapping_sub(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)
                                                         as isize) =
                                    0 as libc::c_int as libc::c_char
                            }
                            _ => { }
                        }
                        cp_0 = a[leasepos as usize];
                        while *cp_0 != 0 {
                            if !(*cp_0 as libc::c_int >= '0' as i32 &&
                                     *cp_0 as libc::c_int <= '9' as i32) {
                                break ;
                            }
                            cp_0 = cp_0.offset(1)
                        }
                        if *cp_0 as libc::c_int != 0 ||
                               (leasepos + 1 as libc::c_int) < k {
                            strcpy(errstr,
                                   b"bad dhcp-range\x00" as *const u8 as
                                       *const libc::c_char);
                            free(new_14 as *mut libc::c_void);
                            return 0 as libc::c_int
                        }
                        (*new_14).lease_time =
                            (atoi(a[leasepos as usize]) * fac) as
                                libc::c_uint;
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 CONTEXT_SETLEASE) as libc::c_int;
                        /* Leases of a minute or less confuse
		       some clients, notably Apple's */
                        if (*new_14).lease_time <
                               120 as libc::c_int as libc::c_uint {
                            (*new_14).lease_time =
                                120 as libc::c_int as libc::c_uint
                        }
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        LOPT_BANK | 71 => {
            /* --dhcp-host */
            let mut new_15 = 0 as *mut dhcp_config;
            let mut in_0 = in_addr{s_addr: 0,};
            new_15 =
                opt_malloc(::std::mem::size_of::<dhcp_config>() as
                               libc::c_ulong) as *mut dhcp_config;
            (*new_15).next = (*dnsmasq_daemon).dhcp_conf;
            (*new_15).flags =
                if option == LOPT_BANK {
                    CONFIG_BANK
                } else { 0 as libc::c_int } as libc::c_uint;
            (*new_15).hwaddr = NULL_0 as *mut hwaddr_config;
            (*new_15).netid = NULL_0 as *mut dhcp_netid_list;
            (*new_15).filter = NULL_0 as *mut dhcp_netid;
            (*new_15).clid = NULL_0 as *mut libc::c_uchar;
            (*new_15).addr6 = NULL_0 as *mut addrlist;
            while !arg.is_null() {
                comma = split(arg);
                if !strchr(arg, ':' as i32).is_null() {
                    /* ethernet address, netid or binary CLID */
                    if (*arg.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'i' as i32 ||
                            *arg.offset(0 as libc::c_int as isize) as
                                libc::c_int == 'I' as i32) &&
                           (*arg.offset(1 as libc::c_int as isize) as
                                libc::c_int == 'd' as i32 ||
                                *arg.offset(1 as libc::c_int as isize) as
                                    libc::c_int == 'D' as i32) &&
                           *arg.offset(2 as libc::c_int as isize) as
                               libc::c_int == ':' as i32 {
                        if *arg.offset(3 as libc::c_int as isize) as
                               libc::c_int == '*' as i32 {
                            (*new_15).flags |= CONFIG_NOCLID as libc::c_uint
                        } else {
                            let mut len_0: libc::c_int = 0; /* dump id: */
                            arg = arg.offset(3 as libc::c_int as isize);
                            if !strchr(arg, ':' as i32).is_null() {
                                len_0 =
                                    parse_hex(arg, arg as *mut libc::c_uchar,
                                              -(1 as libc::c_int),
                                              NULL_0 as *mut libc::c_uint,
                                              NULL_0 as *mut libc::c_int)
                            } else {
                                unhide_metas(arg);
                                len_0 = strlen(arg) as libc::c_int
                            }
                            if len_0 == -(1 as libc::c_int) {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       b"bad hex constant\x00" as *const u8 as
                                           *const libc::c_char);
                                return 0 as libc::c_int
                            } else {
                                (*new_15).clid =
                                    opt_malloc(len_0 as size_t) as
                                        *mut libc::c_uchar;
                                if !(*new_15).clid.is_null() {
                                    (*new_15).flags |=
                                        CONFIG_CLID as libc::c_uint;
                                    (*new_15).clid_len = len_0;
                                    memcpy((*new_15).clid as
                                               *mut libc::c_void,
                                           arg as *const libc::c_void,
                                           len_0 as libc::c_ulong);
                                }
                            }
                        }
                    } else if strstr(arg,
                                     b"net:\x00" as *const u8 as
                                         *const libc::c_char) == arg ||
                                  strstr(arg,
                                         b"set:\x00" as *const u8 as
                                             *const libc::c_char) == arg {
                        let mut newlist_0 =
                            opt_malloc(::std::mem::size_of::<dhcp_netid_list>()
                                           as libc::c_ulong) as
                                *mut dhcp_netid_list;
                        (*newlist_0).next = (*new_15).netid;
                        (*new_15).netid = newlist_0;
                        (*newlist_0).list =
                            dhcp_netid_create(arg.offset(4 as libc::c_int as
                                                             isize),
                                              NULL_0 as *mut dhcp_netid)
                    } else if strstr(arg,
                                     b"tag:\x00" as *const u8 as
                                         *const libc::c_char) == arg {
                        (*new_15).filter =
                            dhcp_netid_create(arg.offset(4 as libc::c_int as
                                                             isize),
                                              (*new_15).filter)
                    } else if *arg.offset(0 as libc::c_int as isize) as
                                  libc::c_int == '[' as i32 &&
                                  *arg.offset(strlen(arg).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                                  as isize) as libc::c_int ==
                                      ']' as i32 {
                        let mut pref_1 = 0 as *mut libc::c_char;
                        let mut in6 =
                            in6_addr{__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},};
                        let mut new_addr = 0 as *mut addrlist;
                        *arg.offset(strlen(arg).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                        as isize) =
                            0 as libc::c_int as libc::c_char;
                        arg = arg.offset(1);
                        pref_1 = split_chr(arg, '/' as i32 as libc::c_char);
                        if inet_pton(AF_INET6, arg,
                                     &mut in6 as *mut in6_addr as
                                         *mut libc::c_void) == 0 {
                            dhcp_config_free(new_15);
                            strcpy(errstr,
                                   b"bad IPv6 address\x00" as *const u8 as
                                       *const libc::c_char);
                            return 0 as libc::c_int
                        }
                        new_addr =
                            opt_malloc(::std::mem::size_of::<addrlist>() as
                                           libc::c_ulong) as *mut addrlist;
                        (*new_addr).next = (*new_15).addr6;
                        (*new_addr).flags = 0 as libc::c_int;
                        (*new_addr).addr.addr6 = in6;
                        (*new_15).addr6 = new_addr;
                        if !pref_1.is_null() {
                            let mut addrpart_0 = addr6part(&mut in6);
                            if atoi_check(pref_1, &mut (*new_addr).prefixlen)
                                   == 0 ||
                                   (*new_addr).prefixlen > 128 as libc::c_int
                                   ||
                                   ((1 as libc::c_int as u64_0) <<
                                        128 as libc::c_int -
                                            (*new_addr).prefixlen).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulonglong)
                                       & addrpart_0 !=
                                       0 as libc::c_int as libc::c_ulonglong {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       b"bad IPv6 prefix\x00" as *const u8 as
                                           *const libc::c_char);
                                return 0 as libc::c_int
                            }
                            (*new_addr).flags |= ADDRLIST_PREFIX
                        }
                        i = 0 as libc::c_int;
                        while i < 8 as libc::c_int {
                            if in6.__in6_u.__u6_addr8[i as usize] as
                                   libc::c_int != 0 as libc::c_int {
                                break ;
                            }
                            i += 1
                        }
                        /* dhcp-host has strange backwards-compat needs. */
                        /* set WILDCARD if network part all zeros */
                        if i == 8 as libc::c_int {
                            (*new_addr).flags |= ADDRLIST_WILDCARD
                        }
                        (*new_15).flags |= CONFIG_ADDR6 as libc::c_uint
                    } else {
                        let mut newhw =
                            opt_malloc(::std::mem::size_of::<hwaddr_config>()
                                           as libc::c_ulong) as
                                *mut hwaddr_config;
                        (*newhw).hwaddr_len =
                            parse_hex(arg, (*newhw).hwaddr.as_mut_ptr(),
                                      DHCP_CHADDR_MAX,
                                      &mut (*newhw).wildcard_mask,
                                      &mut (*newhw).hwaddr_type);
                        if (*newhw).hwaddr_len == -(1 as libc::c_int) {
                            free(newhw as *mut libc::c_void);
                            dhcp_config_free(new_15);
                            strcpy(errstr,
                                   b"bad hex constant\x00" as *const u8 as
                                       *const libc::c_char);
                            return 0 as libc::c_int
                        } else {
                            (*newhw).next = (*new_15).hwaddr;
                            (*new_15).hwaddr = newhw
                        }
                    }
                } else if !strchr(arg, '.' as i32).is_null() &&
                              inet_pton(AF_INET, arg,
                                        &mut in_0 as *mut in_addr as
                                            *mut libc::c_void) >
                                  0 as libc::c_int {
                    let mut configs = 0 as *mut dhcp_config;
                    (*new_15).addr = in_0;
                    (*new_15).flags |= CONFIG_ADDR as libc::c_uint;
                    /* If the same IP appears in more than one host config, then DISCOVER
		   for one of the hosts will get the address, but REQUEST will be NAKed,
		   since the address is reserved by the other one -> protocol loop. */
                    configs = (*dnsmasq_daemon).dhcp_conf;
                    while !configs.is_null() {
                        if (*configs).flags & CONFIG_ADDR as libc::c_uint != 0
                               && (*configs).addr.s_addr == in_0.s_addr {
                            sprintf(errstr,
                                    b"duplicate dhcp-host IP address %s\x00"
                                        as *const u8 as *const libc::c_char,
                                    inet_ntoa(in_0));
                            return 0 as libc::c_int
                        }
                        configs = (*configs).next
                    }
                } else {
                    let mut cp_1 = 0 as *mut libc::c_char;
                    let mut lastp = NULL_0 as *mut libc::c_char;
                    let mut last = 0 as libc::c_int as libc::c_char;
                    let mut fac_0 = 1 as libc::c_int;
                    let mut isdig = 0 as libc::c_int;
                    if strlen(arg) > 1 as libc::c_int as libc::c_ulong {
                        lastp =
                            arg.offset(strlen(arg) as
                                           isize).offset(-(1 as libc::c_int as
                                                               isize));
                        last = *lastp;
                        let mut current_block_1169: u64;
                        match last as libc::c_int {
                            119 | 87 => {
                                fac_0 *= 7 as libc::c_int;
                                current_block_1169 = 16827258629745096341;
                            }
                            100 | 68 => {
                                current_block_1169 = 16827258629745096341;
                            }
                            104 | 72 => {
                                current_block_1169 = 1699689399587118340;
                            }
                            109 | 77 => {
                                current_block_1169 = 9134426092733397760;
                            }
                            115 | 83 => {
                                current_block_1169 = 13003683363839989667;
                            }
                            _ => {
                                current_block_1169 = 14492088476923213239;
                            }
                        }
                        match current_block_1169 {
                            16827258629745096341 =>
                            /* fall through */
                            {
                                fac_0 *= 24 as libc::c_int;
                                current_block_1169 = 1699689399587118340;
                            }
                            _ => { }
                        }
                        match current_block_1169 {
                            1699689399587118340 =>
                            /* fall through */
                            {
                                fac_0 *= 60 as libc::c_int;
                                current_block_1169 = 9134426092733397760;
                            }
                            _ => { }
                        }
                        match current_block_1169 {
                            9134426092733397760 =>
                            /* fall through */
                            {
                                fac_0 *= 60 as libc::c_int;
                                current_block_1169 = 13003683363839989667;
                            }
                            _ => { }
                        }
                        match current_block_1169 {
                            13003683363839989667 =>
                            /* fall through */
                            {
                                *lastp = 0 as libc::c_int as libc::c_char
                            }
                            _ => { }
                        }
                    }
                    cp_1 = arg;
                    while *cp_1 != 0 {
                        if *(*__ctype_b_loc()).offset(*cp_1 as libc::c_uchar
                                                          as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISdigit as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            isdig = 1 as libc::c_int
                        } else if *cp_1 as libc::c_int != ' ' as i32 {
                            break ;
                        }
                        cp_1 = cp_1.offset(1)
                    }
                    if *cp_1 != 0 {
                        if !lastp.is_null() { *lastp = last }
                        if strcmp(arg,
                                  b"infinite\x00" as *const u8 as
                                      *const libc::c_char) == 0 as libc::c_int
                           {
                            (*new_15).lease_time = 0xffffffff as libc::c_uint;
                            (*new_15).flags |= CONFIG_TIME as libc::c_uint
                        } else if strcmp(arg,
                                         b"ignore\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 as libc::c_int {
                            (*new_15).flags |= CONFIG_DISABLE as libc::c_uint
                        } else {
                            (*new_15).hostname = canonicalise_opt(arg);
                            if (*new_15).hostname.is_null() ||
                                   legal_hostname((*new_15).hostname) == 0 {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       b"bad DHCP host name\x00" as *const u8
                                           as *const libc::c_char);
                                return 0 as libc::c_int
                            }
                            (*new_15).flags |= CONFIG_NAME as libc::c_uint;
                            (*new_15).domain =
                                strip_hostname((*new_15).hostname)
                        }
                    } else if isdig != 0 {
                        (*new_15).lease_time =
                            (atoi(arg) * fac_0) as libc::c_uint;
                        /* Leases of a minute or less confuse
		       some clients, notably Apple's */
                        if (*new_15).lease_time <
                               120 as libc::c_int as libc::c_uint {
                            (*new_15).lease_time =
                                120 as libc::c_int as libc::c_uint
                        }
                        (*new_15).flags |= CONFIG_TIME as libc::c_uint
                    }
                }
                arg = comma
            }
            (*dnsmasq_daemon).dhcp_conf = new_15;
            current_block = 7879481898411272068;
        }
        LOPT_TAG_IF => {
            /* --tag-if */
            let mut new_16 =
                opt_malloc(::std::mem::size_of::<tag_if>() as libc::c_ulong)
                    as *mut tag_if;
            (*new_16).tag = NULL_0 as *mut dhcp_netid;
            (*new_16).set = NULL_0 as *mut dhcp_netid_list;
            (*new_16).next = NULL_0 as *mut tag_if;
            /* preserve order */
            if (*dnsmasq_daemon).tag_if.is_null() {
                (*dnsmasq_daemon).tag_if = new_16
            } else {
                let mut tmp_2 = 0 as *mut tag_if;
                tmp_2 = (*dnsmasq_daemon).tag_if;
                while !(*tmp_2).next.is_null() { tmp_2 = (*tmp_2).next }
                (*tmp_2).next = new_16
            }
            while !arg.is_null() {
                let mut len_1: size_t = 0;
                comma = split(arg);
                len_1 = strlen(arg);
                if len_1 < 5 as libc::c_int as libc::c_ulong {
                    (*new_16).set = NULL_0 as *mut dhcp_netid_list;
                    break ;
                } else {
                    let mut newtag =
                        dhcp_netid_create(arg.offset(4 as libc::c_int as
                                                         isize),
                                          NULL_0 as *mut dhcp_netid);
                    if strstr(arg,
                              b"set:\x00" as *const u8 as *const libc::c_char)
                           == arg {
                        let mut newlist_1 =
                            opt_malloc(::std::mem::size_of::<dhcp_netid_list>()
                                           as libc::c_ulong) as
                                *mut dhcp_netid_list;
                        (*newlist_1).next = (*new_16).set;
                        (*new_16).set = newlist_1;
                        (*newlist_1).list = newtag
                    } else if strstr(arg,
                                     b"tag:\x00" as *const u8 as
                                         *const libc::c_char) == arg {
                        (*newtag).next = (*new_16).tag;
                        (*new_16).tag = newtag
                    } else {
                        (*new_16).set = NULL_0 as *mut dhcp_netid_list;
                        dhcp_netid_free(newtag);
                        break ;
                    }
                    arg = comma
                }
            }
            if (*new_16).set.is_null() {
                dhcp_netid_free((*new_16).tag);
                dhcp_netid_list_free((*new_16).set);
                strcpy(errstr,
                       b"bad tag-if\x00" as *const u8 as *const libc::c_char);
                free(new_16 as *mut libc::c_void);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        79 => {
            /* --dhcp-option */
            current_block = 18295461473828413614;
        }
        LOPT_FORCE | LOPT_OPTS | LOPT_MATCH => {
            current_block = 18295461473828413614;
        }
        LOPT_NAME_MATCH => {
            /* --dhcp-name-match */
            let mut new_17 =
                opt_malloc(::std::mem::size_of::<dhcp_match_name>() as
                               libc::c_ulong) as *mut dhcp_match_name;
            let mut id =
                opt_malloc(::std::mem::size_of::<dhcp_netid>() as
                               libc::c_ulong) as *mut dhcp_netid;
            let mut len_2: ssize_t = 0;
            comma = split(arg);
            if comma.is_null() ||
                   {
                       len_2 = strlen(comma) as ssize_t;
                       (len_2) == 0 as libc::c_int as libc::c_long
                   } {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            (*new_17).wildcard = 0 as libc::c_int;
            (*new_17).netid = id;
            (*id).net = opt_string_alloc(set_prefix(arg));
            if *comma.offset((len_2 - 1 as libc::c_int as libc::c_long) as
                                 isize) as libc::c_int == '*' as i32 {
                *comma.offset((len_2 - 1 as libc::c_int as libc::c_long) as
                                  isize) = 0 as libc::c_int as libc::c_char;
                (*new_17).wildcard = 1 as libc::c_int
            }
            (*new_17).name = opt_string_alloc(comma);
            (*new_17).next = (*dnsmasq_daemon).dhcp_name_match;
            (*dnsmasq_daemon).dhcp_name_match = new_17;
            current_block = 7879481898411272068;
        }
        77 => {
            /* --dhcp-boot */
            let mut id_0 = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                let mut dhcp_file = 0 as *mut libc::c_char;
                let mut dhcp_sname = NULL_0 as *mut libc::c_char;
                let mut tftp_sname = NULL_0 as *mut libc::c_char;
                let mut dhcp_next_server = in_addr{s_addr: 0,};
                let mut new_18 = 0 as *mut dhcp_boot;
                comma = split(arg);
                dhcp_file = opt_string_alloc(arg);
                dhcp_next_server.s_addr = 0 as libc::c_int as in_addr_t;
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    dhcp_sname = opt_string_alloc(arg);
                    if !comma.is_null() {
                        unhide_metas(comma);
                        if !(inet_pton(AF_INET, comma,
                                       &mut dhcp_next_server as *mut in_addr
                                           as *mut libc::c_void) >
                                 0 as libc::c_int) {
                            /*
			 * The user may have specified the tftp hostname here.
			 * save it so that it can be resolved/looked up during
			 * actual dhcp_reply().
			 */
                            tftp_sname = opt_string_alloc(comma);
                            dhcp_next_server.s_addr =
                                0 as libc::c_int as in_addr_t
                        }
                    }
                }
                new_18 =
                    opt_malloc(::std::mem::size_of::<dhcp_boot>() as
                                   libc::c_ulong) as *mut dhcp_boot;
                (*new_18).file = dhcp_file;
                (*new_18).sname = dhcp_sname;
                (*new_18).tftp_sname = tftp_sname;
                (*new_18).next_server = dhcp_next_server;
                (*new_18).netid = id_0;
                (*new_18).next = (*dnsmasq_daemon).boot_config;
                (*dnsmasq_daemon).boot_config = new_18
            }
            current_block = 7879481898411272068;
        }
        LOPT_REPLY_DELAY => {
            /* --dhcp-reply-delay */
            let mut id_1 = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                let mut new_19 = 0 as *mut delay_config;
                let mut delay: libc::c_int = 0;
                if atoi_check(arg, &mut delay) == 0 {
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                }
                new_19 =
                    opt_malloc(::std::mem::size_of::<delay_config>() as
                                   libc::c_ulong) as *mut delay_config;
                (*new_19).delay = delay;
                (*new_19).netid = id_1;
                (*new_19).next = (*dnsmasq_daemon).delay_conf;
                (*dnsmasq_daemon).delay_conf = new_19
            }
            current_block = 7879481898411272068;
        }
        LOPT_PXE_PROMT => {
            /* --pxe-prompt */
            let mut new_20 =
                opt_malloc(::std::mem::size_of::<dhcp_opt>() as libc::c_ulong)
                    as *mut dhcp_opt; /* PXE_MENU_PROMPT */
            let mut timeout: libc::c_int = 0;
            (*new_20).netid = NULL_0 as *mut dhcp_netid;
            (*new_20).opt = 10 as libc::c_int;
            (*new_20).netid = dhcp_tags(&mut arg);
            if arg.is_null() {
                dhcp_opt_free(new_20);
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                comma = split(arg);
                unhide_metas(arg);
                (*new_20).len =
                    strlen(arg).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as
                        libc::c_int;
                (*new_20).val =
                    opt_malloc((*new_20).len as size_t) as *mut libc::c_uchar;
                memcpy((*new_20).val.offset(1 as libc::c_int as isize) as
                           *mut libc::c_void, arg as *const libc::c_void,
                       ((*new_20).len - 1 as libc::c_int) as libc::c_ulong);
                (*new_20).u.vendor_class = NULL_0 as *mut libc::c_uchar;
                (*new_20).flags = DHOPT_VENDOR | DHOPT_VENDOR_PXE;
                if !comma.is_null() && atoi_check(comma, &mut timeout) != 0 {
                    *(*new_20).val = timeout as libc::c_uchar
                } else {
                    *(*new_20).val = 255 as libc::c_int as libc::c_uchar
                }
                (*new_20).next = (*dnsmasq_daemon).dhcp_opts;
                (*dnsmasq_daemon).dhcp_opts = new_20;
                (*dnsmasq_daemon).enable_pxe = 1 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_PXE_SERV => {
            /* --pxe-service */
            let mut new_21 =
                opt_malloc(::std::mem::size_of::<pxe_service>() as
                               libc::c_ulong) as
                    *mut pxe_service; /* local boot */
            let mut CSA: [*mut libc::c_char; 13] =
                [b"x86PC\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"PC98\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"IA64_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Alpha\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Arc_x86\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Intel_Lean_Client\x00" as *const u8 as *const libc::c_char
                     as *mut libc::c_char,
                 b"IA32_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"x86-64_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Xscale_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"BC_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"ARM32_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"ARM64_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char, NULL_0 as *mut libc::c_char];
            static mut boottype: libc::c_int = 32768 as libc::c_int;
            (*new_21).netid = NULL_0 as *mut dhcp_netid;
            (*new_21).sname = NULL_0 as *mut libc::c_char;
            (*new_21).server.s_addr = 0 as libc::c_int as in_addr_t;
            (*new_21).netid = dhcp_tags(&mut arg);
            if !arg.is_null() && { comma = split(arg); !comma.is_null() } {
                i = 0 as libc::c_int;
                while !CSA[i as usize].is_null() {
                    if strcasecmp(CSA[i as usize], arg) == 0 as libc::c_int {
                        break ;
                    }
                    i += 1
                }
                if !CSA[i as usize].is_null() || atoi_check(arg, &mut i) != 0
                   {
                    arg = comma;
                    comma = split(arg);
                    (*new_21).CSA = i as libc::c_ushort;
                    (*new_21).menu = opt_string_alloc(arg);
                    if comma.is_null() {
                        (*new_21).type_0 = 0 as libc::c_int as libc::c_ushort;
                        (*new_21).basename = NULL_0 as *mut libc::c_char
                    } else {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check(arg, &mut i) != 0 {
                            (*new_21).type_0 = i as libc::c_ushort;
                            (*new_21).basename = NULL_0 as *mut libc::c_char
                        } else {
                            let fresh28 = boottype;
                            boottype = boottype + 1;
                            (*new_21).type_0 = fresh28 as libc::c_ushort;
                            (*new_21).basename = opt_string_alloc(arg)
                        }
                        if !comma.is_null() {
                            if inet_pton(AF_INET, comma,
                                         &mut (*new_21).server as *mut in_addr
                                             as *mut libc::c_void) == 0 {
                                (*new_21).server.s_addr =
                                    0 as libc::c_int as in_addr_t;
                                (*new_21).sname = opt_string_alloc(comma)
                            }
                        }
                    }
                    /* Order matters */
                    (*new_21).next = NULL_0 as *mut pxe_service;
                    if (*dnsmasq_daemon).pxe_services.is_null() {
                        (*dnsmasq_daemon).pxe_services = new_21
                    } else {
                        let mut s = 0 as *mut pxe_service;
                        s = (*dnsmasq_daemon).pxe_services;
                        while !(*s).next.is_null() { s = (*s).next }
                        (*s).next = new_21
                    }
                    (*dnsmasq_daemon).enable_pxe = 1 as libc::c_int;
                    current_block = 7879481898411272068;
                } else { current_block = 6421703339113101262; }
            } else { current_block = 6421703339113101262; }
            match current_block {
                7879481898411272068 => { }
                _ => { strcpy(errstr, gen_err); return 0 as libc::c_int }
            }
        }
        52 => {
            /* --dhcp-mac */
            comma = split(arg);
            if comma.is_null() {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                let mut new_22 =
                    opt_malloc(::std::mem::size_of::<dhcp_mac>() as
                                   libc::c_ulong) as *mut dhcp_mac;
                (*new_22).netid.net = opt_string_alloc(set_prefix(arg));
                unhide_metas(comma);
                (*new_22).hwaddr_len =
                    parse_hex(comma, (*new_22).hwaddr.as_mut_ptr(),
                              DHCP_CHADDR_MAX, &mut (*new_22).mask,
                              &mut (*new_22).hwaddr_type);
                if (*new_22).hwaddr_len == -(1 as libc::c_int) {
                    free((*new_22).netid.net as *mut libc::c_void);
                    strcpy(errstr, gen_err);
                    free(new_22 as *mut libc::c_void);
                    return 0 as libc::c_int
                } else {
                    (*new_22).next = (*dnsmasq_daemon).dhcp_macs;
                    (*dnsmasq_daemon).dhcp_macs = new_22
                }
            }
            current_block = 7879481898411272068;
        }
        85 => {
            /* --dhcp-vendorclass */
            current_block = 10375845272849059847;
        }
        106 => { current_block = 10375845272849059847; }
        LOPT_CIRCUIT => { current_block = 17332795835978703980; }
        LOPT_REMOTE => { current_block = 15503158355981179141; }
        LOPT_SUBSCR => { current_block = 9763990383449182594; }
        LOPT_ALTPORT => {
            /* --dhcp-alternate-port */
            if arg.is_null() {
                (*dnsmasq_daemon).dhcp_server_port = DHCP_SERVER_ALTPORT;
                (*dnsmasq_daemon).dhcp_client_port = DHCP_CLIENT_ALTPORT
            } else {
                comma = split(arg);
                if atoi_check16(arg, &mut (*dnsmasq_daemon).dhcp_server_port)
                       == 0 ||
                       !comma.is_null() &&
                           atoi_check16(comma,
                                        &mut (*dnsmasq_daemon).dhcp_client_port)
                               == 0 {
                    strcpy(errstr,
                           b"invalid port number\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if comma.is_null() {
                    (*dnsmasq_daemon).dhcp_client_port =
                        (*dnsmasq_daemon).dhcp_server_port + 1 as libc::c_int
                }
            }
            current_block = 7879481898411272068;
        }
        74 => {
            /* --dhcp-ignore */
            current_block = 8728755645494476224;
        }
        LOPT_NO_NAMES => { current_block = 8728755645494476224; }
        LOPT_BROADCAST => { current_block = 9783966086509161201; }
        51 => { current_block = 9535337827963792624; }
        LOPT_GEN_NAMES => { current_block = 8762260891357387630; }
        LOPT_PROXY => {
            /* --dhcp-proxy */
            (*dnsmasq_daemon).override_0 = 1 as libc::c_int;
            while !arg.is_null() {
                let mut new_25 =
                    opt_malloc(::std::mem::size_of::<addr_list>() as
                                   libc::c_ulong) as *mut addr_list;
                comma = split(arg);
                if !(inet_pton(AF_INET, arg,
                               &mut (*new_25).addr as *mut in_addr as
                                   *mut libc::c_void) > 0 as libc::c_int) {
                    strcpy(errstr,
                           b"bad dhcp-proxy address\x00" as *const u8 as
                               *const libc::c_char);
                    free(new_25 as *mut libc::c_void);
                    return 0 as libc::c_int
                }
                (*new_25).next = (*dnsmasq_daemon).override_relays;
                (*dnsmasq_daemon).override_relays = new_25;
                arg = comma
            }
            current_block = 7879481898411272068;
        }
        LOPT_PXE_VENDOR => {
            /* --dhcp-pxe-vendor */
            while !arg.is_null() {
                let mut new_26 =
                    opt_malloc(::std::mem::size_of::<dhcp_pxe_vendor>() as
                                   libc::c_ulong) as *mut dhcp_pxe_vendor;
                comma = split(arg);
                (*new_26).data = opt_string_alloc(arg);
                (*new_26).next = (*dnsmasq_daemon).dhcp_pxe_vendors;
                (*dnsmasq_daemon).dhcp_pxe_vendors = new_26;
                arg = comma
            }
            current_block = 7879481898411272068;
        }
        LOPT_RELAY => {
            /* --dhcp-relay */
            let mut new_27 =
                opt_malloc(::std::mem::size_of::<dhcp_relay>() as
                               libc::c_ulong) as *mut dhcp_relay;
            comma = split(arg);
            (*new_27).interface = opt_string_alloc(split(comma));
            (*new_27).iface_index = 0 as libc::c_int;
            if inet_pton(AF_INET, arg,
                         &mut (*new_27).local as *mut all_addr as
                             *mut libc::c_void) != 0 &&
                   inet_pton(AF_INET, comma,
                             &mut (*new_27).server as *mut all_addr as
                                 *mut libc::c_void) != 0 {
                (*new_27).next = (*dnsmasq_daemon).relay4;
                (*dnsmasq_daemon).relay4 = new_27
            } else if inet_pton(AF_INET6, arg,
                                &mut (*new_27).local as *mut all_addr as
                                    *mut libc::c_void) != 0 &&
                          inet_pton(AF_INET6, comma,
                                    &mut (*new_27).server as *mut all_addr as
                                        *mut libc::c_void) != 0 {
                (*new_27).next = (*dnsmasq_daemon).relay6;
                (*dnsmasq_daemon).relay6 = new_27
            } else {
                free((*new_27).interface as *mut libc::c_void);
                strcpy(errstr,
                       b"Bad dhcp-relay\x00" as *const u8 as
                           *const libc::c_char);
                free(new_27 as *mut libc::c_void);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_RA_PARAM => {
            /* --ra-param */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_28 =
                    opt_malloc(::std::mem::size_of::<ra_interface>() as
                                   libc::c_ulong) as *mut ra_interface;
                (*new_28).lifetime = -(1 as libc::c_int);
                (*new_28).prio = 0 as libc::c_int;
                (*new_28).mtu = 0 as libc::c_int;
                (*new_28).mtu_name = NULL_0 as *mut libc::c_char;
                (*new_28).name = opt_string_alloc(arg);
                if strcasestr(comma,
                              b"mtu:\x00" as *const u8 as *const libc::c_char)
                       == comma {
                    arg = comma.offset(4 as libc::c_int as isize);
                    comma = split(comma);
                    if comma.is_null() {
                        current_block = 14730872864422895907;
                    } else if strcasecmp(arg,
                                         b"off\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                        (*new_28).mtu = -(1 as libc::c_int);
                        current_block = 1840194652026069277;
                    } else if atoi_check(arg, &mut (*new_28).mtu) == 0 {
                        (*new_28).mtu_name = opt_string_alloc(arg);
                        current_block = 1840194652026069277;
                    } else if (*new_28).mtu < 1280 as libc::c_int {
                        current_block = 14730872864422895907;
                    } else { current_block = 1840194652026069277; }
                } else { current_block = 1840194652026069277; }
                match current_block {
                    1840194652026069277 => {
                        if strcasestr(comma,
                                      b"high\x00" as *const u8 as
                                          *const libc::c_char) == comma ||
                               strcasestr(comma,
                                          b"low\x00" as *const u8 as
                                              *const libc::c_char) == comma {
                            if *comma as libc::c_int == 'l' as i32 ||
                                   *comma as libc::c_int == 'L' as i32 {
                                (*new_28).prio = 0x18 as libc::c_int
                            } else { (*new_28).prio = 0x8 as libc::c_int }
                            comma = split(comma)
                        }
                        arg = split(comma);
                        if atoi_check(comma, &mut (*new_28).interval) == 0 ||
                               !arg.is_null() &&
                                   atoi_check(arg, &mut (*new_28).lifetime) ==
                                       0 {
                            current_block = 14730872864422895907;
                        } else {
                            (*new_28).next = (*dnsmasq_daemon).ra_interfaces;
                            (*dnsmasq_daemon).ra_interfaces = new_28;
                            current_block = 7879481898411272068;
                        }
                    }
                    _ => { }
                }
                match current_block {
                    7879481898411272068 => { }
                    _ => {
                        free((*new_28).name as *mut libc::c_void);
                        strcpy(errstr,
                               b"bad RA-params\x00" as *const u8 as
                                   *const libc::c_char);
                        free(new_28 as *mut libc::c_void);
                        return 0 as libc::c_int
                    }
                }
            } else { current_block = 7879481898411272068; }
        }
        LOPT_DUID => {
            /* --dhcp-duid */
            comma = split(arg);
            if comma.is_null() ||
                   atoi_check(arg,
                              &mut (*dnsmasq_daemon).duid_enterprise as
                                  *mut libc::c_uint as *mut libc::c_int) == 0
               {
                strcpy(errstr,
                       b"bad DUID\x00" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            } else {
                (*dnsmasq_daemon).duid_config_len =
                    parse_hex(comma, comma as *mut libc::c_uchar,
                              strlen(comma) as libc::c_int,
                              NULL_0 as *mut libc::c_uint,
                              NULL_0 as *mut libc::c_int) as libc::c_uint;
                (*dnsmasq_daemon).duid_config =
                    opt_malloc((*dnsmasq_daemon).duid_config_len as size_t) as
                        *mut libc::c_uchar;
                memcpy((*dnsmasq_daemon).duid_config as *mut libc::c_void,
                       comma as *const libc::c_void,
                       (*dnsmasq_daemon).duid_config_len as libc::c_ulong);
            }
            current_block = 7879481898411272068;
        }
        86 => {
            /* --alias */
            let mut dash = 0 as *mut libc::c_char;
            let mut a_0: [*mut libc::c_char; 3] =
                [NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char];
            let mut k_0 = 0 as libc::c_int;
            let mut new_29 =
                opt_malloc(::std::mem::size_of::<doctor>() as libc::c_ulong)
                    as *mut doctor;
            (*new_29).next = (*dnsmasq_daemon).doctors;
            (*dnsmasq_daemon).doctors = new_29;
            (*new_29).mask.s_addr = 0xffffffff as libc::c_uint;
            (*new_29).end.s_addr = 0 as libc::c_int as in_addr_t;
            a_0[0 as libc::c_int as usize] = arg;
            if !a_0[0 as libc::c_int as usize].is_null() {
                k_0 = 1 as libc::c_int;
                while k_0 < 3 as libc::c_int {
                    a_0[k_0 as usize] =
                        split(a_0[(k_0 - 1 as libc::c_int) as usize]);
                    if a_0[k_0 as usize].is_null() { break ; }
                    unhide_metas(a_0[k_0 as usize]);
                    k_0 += 1
                }
            }
            dash =
                split_chr(a_0[0 as libc::c_int as usize],
                          '-' as i32 as libc::c_char);
            if k_0 < 2 as libc::c_int ||
                   !(inet_pton(AF_INET, a_0[0 as libc::c_int as usize],
                               &mut (*new_29).in_0 as *mut in_addr as
                                   *mut libc::c_void) > 0 as libc::c_int) ||
                   !(inet_pton(AF_INET, a_0[1 as libc::c_int as usize],
                               &mut (*new_29).out as *mut in_addr as
                                   *mut libc::c_void) > 0 as libc::c_int) ||
                   k_0 == 3 as libc::c_int &&
                       inet_pton(AF_INET, a_0[2 as libc::c_int as usize],
                                 &mut (*new_29).mask as *mut in_addr as
                                     *mut libc::c_void) == 0 {
                strcpy(errstr,
                       b"missing address in alias\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            if !dash.is_null() &&
                   (!(inet_pton(AF_INET, dash,
                                &mut (*new_29).end as *mut in_addr as
                                    *mut libc::c_void) > 0 as libc::c_int) ||
                        is_same_net((*new_29).in_0, (*new_29).end,
                                    (*new_29).mask) == 0 ||
                        __bswap_32((*new_29).in_0.s_addr) >
                            __bswap_32((*new_29).end.s_addr)) {
                strcpy(errstr,
                       b"invalid alias range\x00" as *const u8 as
                           *const libc::c_char);
                free(new_29 as *mut libc::c_void);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        LOPT_INTNAME => {
            /* --interface-name */
            let mut new_30 = 0 as *mut interface_name;
            let mut up_0 = 0 as *mut *mut interface_name;
            let mut domain_1 = NULL_0 as *mut libc::c_char;
            comma = split(arg);
            if comma.is_null() ||
                   { domain_1 = canonicalise_opt(arg); domain_1.is_null() } {
                strcpy(errstr,
                       b"bad interface name\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            new_30 =
                opt_malloc(::std::mem::size_of::<interface_name>() as
                               libc::c_ulong) as *mut interface_name;
            (*new_30).next = NULL_0 as *mut interface_name;
            (*new_30).addr = NULL_0 as *mut addrlist;
            /* Add to the end of the list, so that first name
	   of an interface is used for PTR lookups. */
            up_0 = &mut (*dnsmasq_daemon).int_names;
            while !(*up_0).is_null() { up_0 = &mut (**up_0).next }
            *up_0 = new_30;
            (*new_30).name = domain_1;
            (*new_30).family = 0 as libc::c_int;
            arg = split_chr(comma, '/' as i32 as libc::c_char);
            if !arg.is_null() {
                if strcmp(arg, b"4\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    (*new_30).family = AF_INET
                } else if strcmp(arg,
                                 b"6\x00" as *const u8 as *const libc::c_char)
                              == 0 as libc::c_int {
                    (*new_30).family = AF_INET6
                } else {
                    strcpy(errstr, gen_err);
                    free(new_30 as *mut libc::c_void);
                    return 0 as libc::c_int
                }
            }
            (*new_30).intr = opt_string_alloc(comma);
            current_block = 7879481898411272068;
        }
        LOPT_CNAME => {
            /* --cname */
            let mut new_31 = 0 as *mut cname;
            let mut alias = 0 as *mut libc::c_char;
            let mut target_0 = 0 as *mut libc::c_char;
            let mut last_0 = 0 as *mut libc::c_char;
            let mut pen = 0 as *mut libc::c_char;
            let mut ttl_0 = -(1 as libc::c_int);
            pen = NULL_0 as *mut libc::c_char;
            last_0 = pen;
            comma = arg;
            while !comma.is_null() {
                pen = last_0;
                last_0 = comma;
                comma = split(comma)
            }
            if pen.is_null() {
                strcpy(errstr,
                       b"bad CNAME\x00" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
            if pen != arg && atoi_check(last_0, &mut ttl_0) != 0 {
                last_0 = pen
            }
            target_0 = canonicalise_opt(last_0);
            while arg != last_0 {
                let mut arglen = strlen(arg) as libc::c_int;
                alias = canonicalise_opt(arg);
                if alias.is_null() || target_0.is_null() {
                    free(target_0 as *mut libc::c_void);
                    free(alias as *mut libc::c_void);
                    strcpy(errstr,
                           b"bad CNAME\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                new_31 = (*dnsmasq_daemon).cnames;
                while !new_31.is_null() {
                    if hostname_isequal((*new_31).alias, alias) != 0 {
                        free(target_0 as *mut libc::c_void);
                        free(alias as *mut libc::c_void);
                        strcpy(errstr,
                               b"duplicate CNAME\x00" as *const u8 as
                                   *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    new_31 = (*new_31).next
                }
                new_31 =
                    opt_malloc(::std::mem::size_of::<cname>() as
                                   libc::c_ulong) as *mut cname;
                (*new_31).next = (*dnsmasq_daemon).cnames;
                (*dnsmasq_daemon).cnames = new_31;
                (*new_31).alias = alias;
                (*new_31).target = target_0;
                (*new_31).ttl = ttl_0;
                arg = arg.offset((arglen + 1 as libc::c_int) as isize);
                while *arg as libc::c_int != 0 &&
                          *(*__ctype_b_loc()).offset(*arg as libc::c_int as
                                                         isize) as libc::c_int
                              &
                              _ISspace as libc::c_int as libc::c_ushort as
                                  libc::c_int != 0 {
                    arg = arg.offset(1)
                }
            }
            current_block = 7879481898411272068;
        }
        LOPT_PTR => {
            /* --ptr-record */
            let mut new_32 = 0 as *mut ptr_record;
            let mut dom = 0 as *mut libc::c_char;
            let mut target_1 = NULL_0 as *mut libc::c_char;
            comma = split(arg);
            dom = canonicalise_opt(arg);
            if dom.is_null() ||
                   !comma.is_null() &&
                       {
                           target_1 = canonicalise_opt(comma);
                           target_1.is_null()
                       } {
                free(dom as *mut libc::c_void);
                free(target_1 as *mut libc::c_void);
                strcpy(errstr,
                       b"bad PTR record\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            } else {
                new_32 =
                    opt_malloc(::std::mem::size_of::<ptr_record>() as
                                   libc::c_ulong) as *mut ptr_record;
                (*new_32).next = (*dnsmasq_daemon).ptr;
                (*dnsmasq_daemon).ptr = new_32;
                (*new_32).name = dom;
                (*new_32).ptr = target_1
            }
            current_block = 7879481898411272068;
        }
        LOPT_NAPTR => {
            /* --naptr-record */
            let mut a_1: [*mut libc::c_char; 7] =
                [NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char, NULL_0 as *mut libc::c_char,
                 NULL_0 as *mut libc::c_char];
            let mut k_1 = 0 as libc::c_int;
            let mut new_33 = 0 as *mut naptr;
            let mut order: libc::c_int = 0;
            let mut pref_2: libc::c_int = 0;
            let mut name_2 = NULL_0 as *mut libc::c_char;
            let mut replace = NULL_0 as *mut libc::c_char;
            a_1[0 as libc::c_int as usize] = arg;
            if !a_1[0 as libc::c_int as usize].is_null() {
                k_1 = 1 as libc::c_int;
                while k_1 < 7 as libc::c_int {
                    a_1[k_1 as usize] =
                        split(a_1[(k_1 - 1 as libc::c_int) as usize]);
                    if a_1[k_1 as usize].is_null() { break ; }
                    k_1 += 1
                }
            }
            if k_1 < 6 as libc::c_int ||
                   {
                       name_2 =
                           canonicalise_opt(a_1[0 as libc::c_int as usize]);
                       name_2.is_null()
                   } ||
                   atoi_check16(a_1[1 as libc::c_int as usize], &mut order) ==
                       0 ||
                   atoi_check16(a_1[2 as libc::c_int as usize], &mut pref_2)
                       == 0 ||
                   k_1 == 7 as libc::c_int &&
                       {
                           replace =
                               canonicalise_opt(a_1[6 as libc::c_int as
                                                        usize]);
                           replace.is_null()
                       } {
                free(name_2 as *mut libc::c_void);
                free(replace as *mut libc::c_void);
                strcpy(errstr,
                       b"bad NAPTR record\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            } else {
                new_33 =
                    opt_malloc(::std::mem::size_of::<naptr>() as
                                   libc::c_ulong) as *mut naptr;
                (*new_33).next = (*dnsmasq_daemon).naptr;
                (*dnsmasq_daemon).naptr = new_33;
                (*new_33).name = name_2;
                (*new_33).flags =
                    opt_string_alloc(a_1[3 as libc::c_int as usize]);
                (*new_33).services =
                    opt_string_alloc(a_1[4 as libc::c_int as usize]);
                (*new_33).regexp =
                    opt_string_alloc(a_1[5 as libc::c_int as usize]);
                (*new_33).replace = replace;
                (*new_33).order = order as libc::c_uint;
                (*new_33).pref = pref_2 as libc::c_uint
            }
            current_block = 7879481898411272068;
        }
        LOPT_RR => {
            /* dns-rr */
            let mut new_34 = 0 as *mut txt_record;
            let mut len_3 = 0 as libc::c_int as size_t;
            let mut data = 0 as *mut libc::c_char;
            let mut class: libc::c_int = 0;
            comma = split(arg);
            data = split(comma);
            new_34 =
                opt_malloc(::std::mem::size_of::<txt_record>() as
                               libc::c_ulong) as *mut txt_record;
            (*new_34).name = NULL_0 as *mut libc::c_char;
            if atoi_check(comma, &mut class) == 0 ||
                   {
                       (*new_34).name = canonicalise_opt(arg);
                       (*new_34).name.is_null()
                   } ||
                   !data.is_null() &&
                       {
                           len_3 =
                               parse_hex(data, data as *mut libc::c_uchar,
                                         -(1 as libc::c_int),
                                         NULL_0 as *mut libc::c_uint,
                                         NULL_0 as *mut libc::c_int) as
                                   size_t;
                           (len_3) ==
                               (1 as libc::c_uint).wrapping_neg() as
                                   libc::c_ulong
                       } {
                free((*new_34).name as *mut libc::c_void);
                strcpy(errstr,
                       b"bad RR record\x00" as *const u8 as
                           *const libc::c_char);
                free(new_34 as *mut libc::c_void);
                return 0 as libc::c_int
            }
            (*new_34).len = 0 as libc::c_int as libc::c_ushort;
            (*new_34).class = class as libc::c_ushort;
            (*new_34).next = (*dnsmasq_daemon).rr;
            (*dnsmasq_daemon).rr = new_34;
            if !data.is_null() {
                (*new_34).txt = opt_malloc(len_3) as *mut libc::c_uchar;
                (*new_34).len = len_3 as libc::c_ushort;
                memcpy((*new_34).txt as *mut libc::c_void,
                       data as *const libc::c_void, len_3);
            }
            current_block = 7879481898411272068;
        }
        LOPT_CAA => {
            /* --caa-record */
            let mut new_35 = 0 as *mut txt_record;
            let mut tag = 0 as *mut libc::c_char;
            let mut value = 0 as *mut libc::c_char;
            let mut flags: libc::c_int = 0;
            comma = split(arg);
            tag = split(comma);
            value = split(tag);
            new_35 =
                opt_malloc(::std::mem::size_of::<txt_record>() as
                               libc::c_ulong) as *mut txt_record;
            (*new_35).next = (*dnsmasq_daemon).rr;
            (*dnsmasq_daemon).rr = new_35;
            if atoi_check(comma, &mut flags) == 0 || tag.is_null() ||
                   value.is_null() ||
                   {
                       (*new_35).name = canonicalise_opt(arg);
                       (*new_35).name.is_null()
                   } {
                strcpy(errstr,
                       b"bad CAA record\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            unhide_metas(tag);
            unhide_metas(value);
            (*new_35).len =
                strlen(tag).wrapping_add(strlen(value)).wrapping_add(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                    as libc::c_ushort;
            (*new_35).txt =
                opt_malloc((*new_35).len as size_t) as *mut libc::c_uchar;
            *(*new_35).txt.offset(0 as libc::c_int as isize) =
                flags as libc::c_uchar;
            *(*new_35).txt.offset(1 as libc::c_int as isize) =
                strlen(tag) as libc::c_uchar;
            memcpy(&mut *(*new_35).txt.offset(2 as libc::c_int as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   tag as *const libc::c_void, strlen(tag));
            memcpy(&mut *(*new_35).txt.offset((2 as libc::c_int as
                                                   libc::c_ulong).wrapping_add((strlen
                                                                                    as
                                                                                    unsafe extern "C" fn(_:
                                                                                                             *const libc::c_char)
                                                                                        ->
                                                                                            libc::c_ulong)(tag))
                                                  as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   value as *const libc::c_void, strlen(value));
            (*new_35).class = T_CAA as libc::c_ushort;
            current_block = 7879481898411272068;
        }
        89 => {
            /* --txt-record */
            let mut new_36 = 0 as *mut txt_record; /* room for extra counts */
            let mut p_0 = 0 as *mut libc::c_uchar;
            let mut cnt = 0 as *mut libc::c_uchar;
            let mut len_4: size_t = 0;
            comma = split(arg);
            new_36 =
                opt_malloc(::std::mem::size_of::<txt_record>() as
                               libc::c_ulong) as *mut txt_record;
            (*new_36).class = C_IN as libc::c_ushort;
            (*new_36).stat = 0 as libc::c_int;
            (*new_36).name = canonicalise_opt(arg);
            if (*new_36).name.is_null() {
                strcpy(errstr,
                       b"bad TXT record\x00" as *const u8 as
                           *const libc::c_char);
                free(new_36 as *mut libc::c_void);
                return 0 as libc::c_int
            }
            (*new_36).next = (*dnsmasq_daemon).txt;
            (*dnsmasq_daemon).txt = new_36;
            len_4 =
                if !comma.is_null() {
                    strlen(comma)
                } else { 0 as libc::c_int as libc::c_ulong };
            len_4 =
                (len_4 as
                     libc::c_ulong).wrapping_add(len_4.wrapping_div(255 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_add(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                    as size_t as size_t;
            p_0 = opt_malloc(len_4) as *mut libc::c_uchar;
            (*new_36).txt = p_0;
            let fresh29 = p_0;
            p_0 = p_0.offset(1);
            cnt = fresh29;
            *cnt = 0 as libc::c_int as libc::c_uchar;
            while !comma.is_null() && *comma as libc::c_int != 0 {
                let fresh30 = comma;
                comma = comma.offset(1);
                let mut c = *fresh30 as libc::c_uchar;
                if c as libc::c_int == ',' as i32 ||
                       *cnt as libc::c_int == 255 as libc::c_int {
                    if c as libc::c_int != ',' as i32 {
                        comma = comma.offset(-1)
                    }
                    let fresh31 = p_0;
                    p_0 = p_0.offset(1);
                    cnt = fresh31;
                    *cnt = 0 as libc::c_int as libc::c_uchar
                } else {
                    let fresh32 = p_0;
                    p_0 = p_0.offset(1);
                    *fresh32 =
                        unhide_meta(c as libc::c_char) as libc::c_uchar;
                    *cnt = (*cnt).wrapping_add(1)
                }
            }
            (*new_36).len =
                p_0.wrapping_offset_from((*new_36).txt) as libc::c_long as
                    libc::c_ushort;
            current_block = 7879481898411272068;
        }
        87 => {
            /* --srv-host */
            let mut port = 1 as libc::c_int;
            let mut priority = 0 as libc::c_int;
            let mut weight = 0 as libc::c_int;
            let mut name_3 = 0 as *mut libc::c_char;
            let mut target_2 = NULL_0 as *mut libc::c_char;
            let mut new_37 = 0 as *mut mx_srv_record;
            comma = split(arg);
            name_3 = canonicalise_opt(arg);
            if name_3.is_null() {
                strcpy(errstr,
                       b"bad SRV record\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            if !comma.is_null() {
                arg = comma;
                comma = split(arg);
                target_2 = canonicalise_opt(arg);
                if target_2.is_null() {
                    strcpy(errstr,
                           b"bad SRV target\x00" as *const u8 as
                               *const libc::c_char);
                    free(name_3 as *mut libc::c_void);
                    return 0 as libc::c_int
                }
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    if atoi_check16(arg, &mut port) == 0 {
                        free(name_3 as *mut libc::c_void);
                        strcpy(errstr,
                               b"invalid port number\x00" as *const u8 as
                                   *const libc::c_char);
                        free(target_2 as *mut libc::c_void);
                        return 0 as libc::c_int
                    }
                    if !comma.is_null() {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check16(arg, &mut priority) == 0 {
                            free(name_3 as *mut libc::c_void);
                            strcpy(errstr,
                                   b"invalid priority\x00" as *const u8 as
                                       *const libc::c_char);
                            free(target_2 as *mut libc::c_void);
                            return 0 as libc::c_int
                        }
                        if !comma.is_null() &&
                               atoi_check16(comma, &mut weight) == 0 {
                            free(name_3 as *mut libc::c_void);
                            strcpy(errstr,
                                   b"invalid weight\x00" as *const u8 as
                                       *const libc::c_char);
                            free(target_2 as *mut libc::c_void);
                            return 0 as libc::c_int
                        }
                    }
                }
            }
            new_37 =
                opt_malloc(::std::mem::size_of::<mx_srv_record>() as
                               libc::c_ulong) as *mut mx_srv_record;
            (*new_37).next = (*dnsmasq_daemon).mxnames;
            (*dnsmasq_daemon).mxnames = new_37;
            (*new_37).issrv = 1 as libc::c_int;
            (*new_37).name = name_3;
            (*new_37).target = target_2;
            (*new_37).srvport = port;
            (*new_37).priority = priority;
            (*new_37).weight = weight;
            current_block = 7879481898411272068;
        }
        LOPT_HOST_REC => {
            /* --host-record */
            let mut new_38 = 0 as *mut host_record;
            if arg.is_null() || { comma = split(arg); comma.is_null() } {
                strcpy(errstr,
                       b"Bad host-record\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            new_38 =
                opt_malloc(::std::mem::size_of::<host_record>() as
                               libc::c_ulong) as *mut host_record;
            memset(new_38 as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<host_record>() as libc::c_ulong);
            (*new_38).ttl = -(1 as libc::c_int);
            (*new_38).flags = 0 as libc::c_int;
            while !arg.is_null() {
                let mut addr_1 = all_addr{addr4: in_addr{s_addr: 0,},};
                let mut dig_0 = 0 as *mut libc::c_char;
                dig_0 = arg;
                while *dig_0 as libc::c_int != 0 as libc::c_int {
                    if (*dig_0 as libc::c_int) < '0' as i32 ||
                           *dig_0 as libc::c_int > '9' as i32 {
                        break ;
                    }
                    dig_0 = dig_0.offset(1)
                }
                if *dig_0 as libc::c_int == 0 as libc::c_int {
                    (*new_38).ttl = atoi(arg)
                } else if inet_pton(AF_INET, arg,
                                    &mut addr_1.addr4 as *mut in_addr as
                                        *mut libc::c_void) != 0 {
                    (*new_38).addr = addr_1.addr4;
                    (*new_38).flags |= HR_4
                } else if inet_pton(AF_INET6, arg,
                                    &mut addr_1.addr6 as *mut in6_addr as
                                        *mut libc::c_void) != 0 {
                    (*new_38).addr6 = addr_1.addr6;
                    (*new_38).flags |= HR_6
                } else {
                    let mut nomem: libc::c_int = 0;
                    let mut canon = canonicalise(arg, &mut nomem);
                    let mut nl = 0 as *mut name_list;
                    if canon.is_null() {
                        let mut tmp_3 = (*new_38).names;
                        let mut next = 0 as *mut name_list;
                        tmp_3 = (*new_38).names;
                        while !tmp_3.is_null() {
                            next = (*tmp_3).next;
                            free(tmp_3 as *mut libc::c_void);
                            tmp_3 = next
                        }
                        strcpy(errstr,
                               b"Bad name in host-record\x00" as *const u8 as
                                   *const libc::c_char);
                        free(new_38 as *mut libc::c_void);
                        return 0 as libc::c_int
                    }
                    nl =
                        opt_malloc(::std::mem::size_of::<name_list>() as
                                       libc::c_ulong) as *mut name_list;
                    (*nl).name = canon;
                    /* keep order, so that PTR record goes to first name */
                    (*nl).next = NULL_0 as *mut name_list;
                    if (*new_38).names.is_null() {
                        (*new_38).names = nl
                    } else {
                        let mut tmp_4 = 0 as *mut name_list;
                        tmp_4 = (*new_38).names;
                        while !(*tmp_4).next.is_null() {
                            tmp_4 = (*tmp_4).next
                        }
                        (*tmp_4).next = nl
                    }
                }
                arg = comma;
                comma = split(arg)
            }
            /* Keep list order */
            if (*dnsmasq_daemon).host_records_tail.is_null() {
                (*dnsmasq_daemon).host_records = new_38
            } else { (*(*dnsmasq_daemon).host_records_tail).next = new_38 }
            (*new_38).next = NULL_0 as *mut host_record;
            (*dnsmasq_daemon).host_records_tail = new_38;
            current_block = 7879481898411272068;
        }
        _ => {
            strcpy(errstr,
                   b"unsupported option (check that dnsmasq was compiled with DHCP/TFTP/DNSSEC/DBus support)\x00"
                       as *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    match current_block {
        2926860427235594157 =>
        /* --ignore-address */
        {
            let mut addr_0 = in_addr{s_addr: 0,}; /* error */
            unhide_metas(arg);
            if !arg.is_null() &&
                   inet_pton(AF_INET, arg,
                             &mut addr_0 as *mut in_addr as *mut libc::c_void)
                       > 0 as libc::c_int {
                let mut baddr =
                    opt_malloc(::std::mem::size_of::<bogus_addr>() as
                                   libc::c_ulong) as *mut bogus_addr;
                if option == 'B' as i32 {
                    (*baddr).next = (*dnsmasq_daemon).bogus_addr;
                    (*dnsmasq_daemon).bogus_addr = baddr
                } else {
                    (*baddr).next = (*dnsmasq_daemon).ignore_addr;
                    (*dnsmasq_daemon).ignore_addr = baddr
                }
                (*baddr).addr = addr_0
            } else { strcpy(errstr, gen_err); return 0 as libc::c_int }
            current_block = 7879481898411272068;
        }
        12010070245366740438 =>
        /* --dhcp-optsfile */
        {
            current_block = 2812646229686797995;
        }
        887445304002143054 =>
        /* --except-interface */
        {
            loop 
                 /* --no-dhcp-interface */
                 {
                let mut new_9 =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                (*new_9).name = opt_string_alloc(arg);
                if option == 'I' as i32 {
                    (*new_9).next = (*dnsmasq_daemon).if_except;
                    (*dnsmasq_daemon).if_except = new_9
                } else if option == LOPT_TFTP {
                    (*new_9).next = (*dnsmasq_daemon).tftp_interfaces;
                    (*dnsmasq_daemon).tftp_interfaces = new_9
                } else {
                    (*new_9).next = (*dnsmasq_daemon).dhcp_except;
                    (*dnsmasq_daemon).dhcp_except = new_9
                }
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        9676380469790025234 =>
        /*  --local */
        {
            current_block = 6480954168551069607;
        }
        15489771604880449635 =>
        /* --neg-ttl */
        {
            current_block = 6082976577402880686;
        }
        18295461473828413614 =>
        /* --dhcp-option-force */
        /* --dhcp-match */
        {
            return parse_dhcp_opt(errstr, arg,
                                  if option == LOPT_FORCE {
                                      DHOPT_FORCE
                                  } else if option == LOPT_MATCH {
                                      DHOPT_MATCH
                                  } else if option == LOPT_OPTS {
                                      DHOPT_BANK
                                  } else { 0 as libc::c_int })
        }
        10375845272849059847 =>
        /* --dhcp-userclass */
        {
            current_block = 17332795835978703980;
        }
        8728755645494476224 =>
        /* --dhcp-ignore-names */
        {
            current_block = 9783966086509161201;
        }
        _ => { }
    }
    match current_block {
        2812646229686797995 =>
        /* --dhcp-hostsdir */
        {
            current_block = 10566976656908717602;
        }
        6480954168551069607 =>
        /*  --address */
        {
            current_block = 14399141444697241811;
        }
        6082976577402880686 =>
        /* --max-ttl */
        {
            current_block = 16916584745428150692;
        }
        17332795835978703980 =>
        /* --dhcp-circuitid */
        {
            current_block = 15503158355981179141;
        }
        9783966086509161201 =>
        /* --dhcp-broadcast */
        {
            current_block = 9535337827963792624;
        }
        _ => { }
    }
    match current_block {
        14399141444697241811 =>
        /*  --rebind-domain-ok */
        {
            let mut serv_1 = 0 as *mut server;
            let mut newlist = NULL_0 as *mut server;
            unhide_metas(arg);
            if !arg.is_null() &&
                   (*arg as libc::c_int == '/' as i32 ||
                        option == LOPT_NO_REBIND) {
                let mut rebind =
                    !(*arg as libc::c_int == '/' as i32) as libc::c_int;
                let mut end_0 = NULL_0 as *mut libc::c_char;
                if rebind == 0 { arg = arg.offset(1) }
                while rebind != 0 ||
                          {
                              end_0 =
                                  split_chr(arg, '/' as i32 as libc::c_char);
                              !end_0.is_null()
                          } {
                    let mut domain = NULL_0 as *mut libc::c_char;
                    /* elide leading dots - they are implied in the search algorithm */
                    while *arg as libc::c_int == '.' as i32 {
                        arg = arg.offset(1)
                    }
                    /* # matches everything and becomes a zero length domain string */
                    if strcmp(arg,
                              b"#\x00" as *const u8 as *const libc::c_char) ==
                           0 as libc::c_int {
                        domain =
                            b"\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char
                    } else if strlen(arg) != 0 as libc::c_int as libc::c_ulong
                                  &&
                                  {
                                      domain = canonicalise_opt(arg);
                                      domain.is_null()
                                  } {
                        strcpy(errstr, gen_err);
                        return 0 as libc::c_int
                    }
                    serv_1 =
                        opt_malloc(::std::mem::size_of::<server>() as
                                       libc::c_ulong) as *mut server;
                    memset(serv_1 as *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<server>() as libc::c_ulong);
                    (*serv_1).next = newlist;
                    newlist = serv_1;
                    (*serv_1).domain = domain;
                    (*serv_1).flags =
                        if !domain.is_null() {
                            SERV_HAS_DOMAIN
                        } else { SERV_FOR_NODOTS };
                    arg = end_0;
                    if rebind != 0 { break ; }
                }
                if newlist.is_null() {
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                }
            } else {
                newlist =
                    opt_malloc(::std::mem::size_of::<server>() as
                                   libc::c_ulong) as *mut server;
                memset(newlist as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<server>() as libc::c_ulong);
                (*newlist).uid = rand32()
            }
            if servers_only != 0 && option == 'S' as i32 {
                (*newlist).flags |= SERV_FROM_FILE
            }
            if option == 'A' as i32 {
                (*newlist).flags |= SERV_LITERAL_ADDRESS;
                if (*newlist).flags & SERV_TYPE == 0 {
                    server_list_free(newlist);
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                }
            } else if option == LOPT_NO_REBIND {
                (*newlist).flags |= SERV_NO_REBIND
            }
            if arg.is_null() || *arg == 0 {
                if (*newlist).flags & SERV_NO_REBIND == 0 {
                    (*newlist).flags |= SERV_NO_ADDR
                }
                /* no server */
            } else if strcmp(arg,
                             b"#\x00" as *const u8 as *const libc::c_char) ==
                          0 as libc::c_int {
                (*newlist).flags |= SERV_USE_RESOLV
            } else {
                let mut err_0 =
                    parse_server(arg, &mut (*newlist).addr,
                                 &mut (*newlist).source_addr,
                                 (*newlist).interface.as_mut_ptr(),
                                 &mut (*newlist).flags); /* treat in ordinary way */
                if !err_0.is_null() {
                    server_list_free(newlist);
                    strcpy(errstr, err_0);
                    return 0 as libc::c_int
                }
            }
            serv_1 = newlist;
            while !(*serv_1).next.is_null() {
                (*(*serv_1).next).flags |=
                    (*serv_1).flags & !(SERV_HAS_DOMAIN | SERV_FOR_NODOTS);
                (*(*serv_1).next).addr = (*serv_1).addr;
                (*(*serv_1).next).source_addr = (*serv_1).source_addr;
                strcpy((*(*serv_1).next).interface.as_mut_ptr(),
                       (*serv_1).interface.as_mut_ptr());
                serv_1 = (*serv_1).next
            }
            (*serv_1).next = (*dnsmasq_daemon).servers;
            (*dnsmasq_daemon).servers = newlist;
            current_block = 7879481898411272068;
        }
        10566976656908717602 =>
        /* --dhcp-optsdir */
        {
            current_block = 2602045500541335152;
        }
        16916584745428150692 =>
        /* --min-cache-ttl */
        {
            current_block = 13094692781038244044;
        }
        15503158355981179141 =>
        /* --dhcp-remoteid */
        {
            current_block = 9763990383449182594;
        }
        9535337827963792624 =>
        /* --bootp-dynamic */
        {
            current_block = 8762260891357387630;
        }
        _ => { }
    }
    match current_block {
        8762260891357387630 =>
        /* --dhcp-generate-names */
        {
            let mut new_24 =
                opt_malloc(::std::mem::size_of::<dhcp_netid_list>() as
                               libc::c_ulong) as *mut dhcp_netid_list;
            let mut list_1 = NULL_0 as *mut dhcp_netid;
            if option == 'J' as i32 {
                (*new_24).next = (*dnsmasq_daemon).dhcp_ignore;
                (*dnsmasq_daemon).dhcp_ignore = new_24
            } else if option == LOPT_BROADCAST {
                (*new_24).next = (*dnsmasq_daemon).force_broadcast;
                (*dnsmasq_daemon).force_broadcast = new_24
            } else if option == '3' as i32 {
                (*new_24).next = (*dnsmasq_daemon).bootp_dynamic;
                (*dnsmasq_daemon).bootp_dynamic = new_24
            } else if option == LOPT_GEN_NAMES {
                (*new_24).next = (*dnsmasq_daemon).dhcp_gen_names;
                (*dnsmasq_daemon).dhcp_gen_names = new_24
            } else {
                (*new_24).next = (*dnsmasq_daemon).dhcp_ignore_names;
                (*dnsmasq_daemon).dhcp_ignore_names = new_24
            }
            while !arg.is_null() {
                comma = split(arg);
                list_1 =
                    dhcp_netid_create(if is_tag_prefix(arg) != 0 {
                                          arg.offset(4 as libc::c_int as
                                                         isize)
                                      } else { arg }, list_1);
                arg = comma
            }
            (*new_24).list = list_1;
            current_block = 7879481898411272068;
        }
        9763990383449182594 =>
        /* --dhcp-subscrid */
        {
            let mut p = 0 as *mut libc::c_uchar;
            let mut dig = 0 as libc::c_int;
            let mut new_23 =
                opt_malloc(::std::mem::size_of::<dhcp_vendor>() as
                               libc::c_ulong) as *mut dhcp_vendor;
            comma = split(arg);
            if comma.is_null() {
                strcpy(errstr, gen_err);
                free(new_23 as *mut libc::c_void);
                return 0 as libc::c_int
            }
            (*new_23).netid.net = opt_string_alloc(set_prefix(arg));
            /* check for hex string - must digits may include : must not have nothing else, 
	    only allowed for agent-options. */
            arg = comma;
            comma = split(arg);
            if !comma.is_null() {
                if option != 'U' as i32 ||
                       strstr(arg,
                              b"enterprise:\x00" as *const u8 as
                                  *const libc::c_char) != arg {
                    free((*new_23).netid.net as *mut libc::c_void);
                    strcpy(errstr, gen_err);
                    free(new_23 as *mut libc::c_void);
                    return 0 as libc::c_int
                } else {
                    (*new_23).enterprise =
                        atoi(arg.offset(11 as libc::c_int as isize)) as
                            libc::c_uint
                }
            } else { comma = arg }
            p = comma as *mut libc::c_uchar;
            while *p != 0 {
                if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                       libc::c_int &
                       _ISxdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    dig = 1 as libc::c_int
                } else if *p as libc::c_int != ':' as i32 { break ; }
                p = p.offset(1)
            }
            unhide_metas(comma);
            if option == 'U' as i32 || option == 'j' as i32 ||
                   *p as libc::c_int != 0 || dig == 0 {
                (*new_23).len = strlen(comma) as libc::c_int;
                (*new_23).data =
                    opt_malloc((*new_23).len as size_t) as *mut libc::c_char;
                memcpy((*new_23).data as *mut libc::c_void,
                       comma as *const libc::c_void,
                       (*new_23).len as libc::c_ulong);
            } else {
                (*new_23).len =
                    parse_hex(comma, comma as *mut libc::c_uchar,
                              strlen(comma) as libc::c_int,
                              NULL_0 as *mut libc::c_uint,
                              NULL_0 as *mut libc::c_int);
                (*new_23).data =
                    opt_malloc((*new_23).len as size_t) as *mut libc::c_char;
                memcpy((*new_23).data as *mut libc::c_void,
                       comma as *const libc::c_void,
                       (*new_23).len as libc::c_ulong);
            }
            match option {
                106 => { (*new_23).match_type = MATCH_USER }
                85 => { (*new_23).match_type = MATCH_VENDOR }
                LOPT_CIRCUIT => { (*new_23).match_type = MATCH_CIRCUIT }
                LOPT_REMOTE => { (*new_23).match_type = MATCH_REMOTE }
                LOPT_SUBSCR => { (*new_23).match_type = MATCH_SUBSCRIBER }
                _ => { }
            }
            (*new_23).next = (*dnsmasq_daemon).dhcp_vendors;
            (*dnsmasq_daemon).dhcp_vendors = new_23;
            current_block = 7879481898411272068;
        }
        2602045500541335152 =>
        /* --hostsdir */
        {
            current_block = 4533671380017093834;
        }
        13094692781038244044 =>
        /* --max-cache-ttl */
        {
            current_block = 13035992208579083528;
        }
        _ => { }
    }
    match current_block {
        4533671380017093834 =>
        /* --addn-hosts */
        {
            let mut new_3 =
                opt_malloc(::std::mem::size_of::<hostsfile>() as
                               libc::c_ulong) as *mut hostsfile;
            static mut hosts_index: libc::c_uint = SRC_AH as libc::c_uint;
            (*new_3).fname = opt_string_alloc(arg);
            let fresh26 = hosts_index;
            hosts_index = hosts_index.wrapping_add(1);
            (*new_3).index = fresh26;
            (*new_3).flags = 0 as libc::c_int;
            if option == 'H' as i32 {
                (*new_3).next = (*dnsmasq_daemon).addn_hosts;
                (*dnsmasq_daemon).addn_hosts = new_3
            } else if option == LOPT_DHCP_HOST {
                (*new_3).next = (*dnsmasq_daemon).dhcp_hosts_file;
                (*dnsmasq_daemon).dhcp_hosts_file = new_3
            } else if option == LOPT_DHCP_OPTS {
                (*new_3).next = (*dnsmasq_daemon).dhcp_opts_file;
                (*dnsmasq_daemon).dhcp_opts_file = new_3
            } else {
                (*new_3).next = (*dnsmasq_daemon).dynamic_dirs;
                (*dnsmasq_daemon).dynamic_dirs = new_3;
                if option == LOPT_DHCP_INOTIFY {
                    (*new_3).flags |= AH_DHCP_HST
                } else if option == LOPT_DHOPT_INOTIFY {
                    (*new_3).flags |= AH_DHCP_OPT
                } else if option == LOPT_HOST_INOTIFY {
                    (*new_3).flags |= AH_HOSTS
                }
            }
            current_block = 7879481898411272068;
        }
        13035992208579083528 =>
        /* --auth-ttl */
        {
            current_block = 5893551302610724882;
        }
        _ => { }
    }
    match current_block {
        5893551302610724882 =>
        /* --dhcp-ttl */
        {
            let mut ttl: libc::c_int = 0;
            if atoi_check(arg, &mut ttl) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                if option == LOPT_NEGTTL {
                    (*dnsmasq_daemon).neg_ttl = ttl as libc::c_ulong
                } else if option == LOPT_MAXTTL {
                    (*dnsmasq_daemon).max_ttl = ttl as libc::c_ulong
                } else if option == LOPT_MINCTTL {
                    if ttl > TTL_FLOOR_LIMIT { ttl = TTL_FLOOR_LIMIT }
                    (*dnsmasq_daemon).min_cache_ttl = ttl as libc::c_ulong
                } else if option == LOPT_MAXCTTL {
                    (*dnsmasq_daemon).max_cache_ttl = ttl as libc::c_ulong
                } else if option == LOPT_AUTHTTL {
                    (*dnsmasq_daemon).auth_ttl = ttl as libc::c_ulong
                } else if option == LOPT_DHCPTTL {
                    (*dnsmasq_daemon).dhcp_ttl = ttl as libc::c_ulong;
                    (*dnsmasq_daemon).use_dhcp_ttl =
                        1 as libc::c_int as libc::c_ulong
                } else { (*dnsmasq_daemon).local_ttl = ttl as libc::c_ulong }
            }
        }
        _ => { }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "4557:1"]
unsafe extern "C" fn read_file(mut file: *mut libc::c_char, mut f: *mut FILE,
                               mut hard_opt: libc::c_int) {
    let mut lineno = 0 as libc::c_int;
    let mut buff = (*dnsmasq_daemon).namebuff;
    let mut current_block_66: u64;
    while !fgets(buff, MAXDNAME, f).is_null() {
        let mut white: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut option =
            if hard_opt == LOPT_REV_SERV {
                0 as libc::c_int
            } else { hard_opt };
        let mut errmess = 0 as *mut libc::c_char;
        let mut p = 0 as *mut libc::c_char;
        let mut arg = 0 as *mut libc::c_char;
        let mut start = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        /* Memory allocation failure longjmps here if mem_recover == 1 */
        if option != 0 as libc::c_int || hard_opt == LOPT_REV_SERV {
            if _setjmp(mem_jmp.as_mut_ptr()) != 0 { continue ; }
            ::std::ptr::write_volatile(&mut mem_recover as *mut libc::c_int,
                                       1 as libc::c_int)
        }
        arg = NULL_0 as *mut libc::c_char;
        ::std::ptr::write_volatile(&mut lineno as *mut libc::c_int,
                                   ::std::ptr::read_volatile::<libc::c_int>(&lineno
                                                                                as
                                                                                *const libc::c_int)
                                       + 1);
        errmess = NULL_0 as *mut libc::c_char;
        /* Implement quotes, inside quotes we allow \\ \" \n and \t 
	 metacharacters get hidden also strip comments */
        white = 1 as libc::c_int;
        p = buff;
        loop  {
            if !(*p != 0) { current_block_66 = 12199444798915819164; break ; }
            if *p as libc::c_int == '\"' as i32 {
                memmove(p as *mut libc::c_void,
                        p.offset(1 as libc::c_int as isize) as
                            *const libc::c_void,
                        strlen(p.offset(1 as libc::c_int as
                                            isize)).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong));
                while *p as libc::c_int != 0 &&
                          *p as libc::c_int != '\"' as i32 {
                    if *p as libc::c_int == '\\' as i32 &&
                           !strchr(b"\"tnebr\\\x00" as *const u8 as
                                       *const libc::c_char,
                                   *p.offset(1 as libc::c_int as isize) as
                                       libc::c_int).is_null() {
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int
                               == 't' as i32 {
                            *p.offset(1 as libc::c_int as isize) =
                                '\t' as i32 as libc::c_char
                        } else if *p.offset(1 as libc::c_int as isize) as
                                      libc::c_int == 'n' as i32 {
                            *p.offset(1 as libc::c_int as isize) =
                                '\n' as i32 as libc::c_char
                        } else if *p.offset(1 as libc::c_int as isize) as
                                      libc::c_int == 'b' as i32 {
                            *p.offset(1 as libc::c_int as isize) =
                                '\u{8}' as i32 as libc::c_char
                        } else if *p.offset(1 as libc::c_int as isize) as
                                      libc::c_int == 'r' as i32 {
                            *p.offset(1 as libc::c_int as isize) =
                                '\r' as i32 as libc::c_char
                        } else if *p.offset(1 as libc::c_int as isize) as
                                      libc::c_int == 'e' as i32 {
                            /* escape */
                            *p.offset(1 as libc::c_int as isize) =
                                '\u{1b}' as i32 as libc::c_char
                        }
                        memmove(p as *mut libc::c_void,
                                p.offset(1 as libc::c_int as isize) as
                                    *const libc::c_void,
                                strlen(p.offset(1 as libc::c_int as
                                                    isize)).wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong));
                    }
                    *p = hide_meta(*p);
                    p = p.offset(1)
                }
                if *p as libc::c_int == 0 as libc::c_int {
                    errmess =
                        b"missing \"\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    current_block_66 = 15635431839692940240;
                    break ;
                } else {
                    memmove(p as *mut libc::c_void,
                            p.offset(1 as libc::c_int as isize) as
                                *const libc::c_void,
                            strlen(p.offset(1 as libc::c_int as
                                                isize)).wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                }
            }
            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                   libc::c_int &
                   _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                *p = ' ' as i32 as libc::c_char;
                white = 1 as libc::c_int
            } else if white != 0 && *p as libc::c_int == '#' as i32 {
                *p = 0 as libc::c_int as libc::c_char;
                current_block_66 = 12199444798915819164;
                break ;
            } else { white = 0 as libc::c_int }
            p = p.offset(1)
        }
        match current_block_66 {
            12199444798915819164 => {
                /* strip leading spaces */
                start = buff;
                while *start as libc::c_int != 0 &&
                          *start as libc::c_int == ' ' as i32 {
                    start = start.offset(1)
                }
                /* strip trailing spaces */
                len = strlen(start);
                while len != 0 as libc::c_int as libc::c_ulong &&
                          *start.offset(len.wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                                            isize) as libc::c_int ==
                              ' ' as i32 {
                    len = len.wrapping_sub(1)
                }
                if len == 0 as libc::c_int as libc::c_ulong { continue ; }
                *start.offset(len as isize) =
                    0 as libc::c_int as libc::c_char;
                if option != 0 as libc::c_int {
                    arg = start
                } else {
                    p = strchr(start, '=' as i32);
                    if !p.is_null() {
                        /* allow spaces around "=" */
                        arg = p.offset(1 as libc::c_int as isize);
                        while *arg as libc::c_int == ' ' as i32 {
                            arg = arg.offset(1)
                        }
                        while p >= start &&
                                  (*p as libc::c_int == ' ' as i32 ||
                                       *p as libc::c_int == '=' as i32) {
                            *p = 0 as libc::c_int as libc::c_char;
                            p = p.offset(-1)
                        }
                    } else { arg = NULL_0 as *mut libc::c_char }
                }
                if option == 0 as libc::c_int {
                    ::std::ptr::write_volatile(&mut option as
                                                   *mut libc::c_int,
                                               0 as libc::c_int);
                    i = 0 as libc::c_int;
                    while !opts[i as usize].name.is_null() {
                        if strcmp(opts[i as usize].name, start) ==
                               0 as libc::c_int {
                            ::std::ptr::write_volatile(&mut option as
                                                           *mut libc::c_int,
                                                       opts[i as usize].val);
                            break ;
                        } else { i += 1 }
                    }
                    if option == 0 {
                        errmess =
                            b"bad option\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if opts[i as usize].has_arg == 0 as libc::c_int &&
                                  !arg.is_null() {
                        errmess =
                            b"extraneous parameter\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if opts[i as usize].has_arg == 1 as libc::c_int &&
                                  arg.is_null() {
                        errmess =
                            b"missing parameter\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if hard_opt == LOPT_REV_SERV &&
                                  option != 'S' as i32 &&
                                  option != LOPT_REV_SERV {
                        errmess =
                            b"illegal option\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    }
                }
            }
            _ => { }
        }
        if !errmess.is_null() { strcpy((*dnsmasq_daemon).namebuff, errmess); }
        if !errmess.is_null() ||
               one_opt(option, arg, (*dnsmasq_daemon).namebuff,
                       b"error\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char, 0 as libc::c_int,
                       (hard_opt == LOPT_REV_SERV) as libc::c_int) == 0 {
            sprintf((*dnsmasq_daemon).namebuff.offset(strlen((*dnsmasq_daemon).namebuff)
                                                          as isize),
                    b" at line %d of %s\x00" as *const u8 as
                        *const libc::c_char, lineno, file);
            if hard_opt != 0 as libc::c_int {
                my_syslog(LOG_ERR,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).namebuff);
            } else {
                die(b"%s\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, (*dnsmasq_daemon).namebuff,
                    EC_BADCONF);
            }
        }
    }
    ::std::ptr::write_volatile(&mut mem_recover as *mut libc::c_int,
                               0 as libc::c_int);
    fclose(f);
}
#[no_mangle]
#[c2rust::src_loc = "4695:1"]
pub unsafe extern "C" fn option_read_dynfile(mut file: *mut libc::c_char,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    my_syslog(MS_DHCP | LOG_INFO,
              b"read %s\x00" as *const u8 as *const libc::c_char, file);
    if flags & AH_DHCP_HST != 0 {
        return one_file(file, LOPT_BANK)
    } else {
        if flags & AH_DHCP_OPT != 0 { return one_file(file, LOPT_OPTS) }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "4708:1"]
unsafe extern "C" fn one_file(mut file: *mut libc::c_char,
                              mut hard_opt: libc::c_int) -> libc::c_int {
    let mut f = 0 as *mut FILE;
    let mut nofile_ok = 0 as libc::c_int;
    static mut read_stdin: libc::c_int = 0 as libc::c_int;
    static mut filesread: *mut fileread = NULL_0 as *mut fileread;
    if hard_opt == '7' as i32 {
        /* default conf-file reading */
        hard_opt = 0 as libc::c_int;
        nofile_ok = 1 as libc::c_int
    }
    if hard_opt == 0 as libc::c_int &&
           strcmp(file, b"-\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        if read_stdin == 1 as libc::c_int { return 1 as libc::c_int }
        read_stdin = 1 as libc::c_int;
        file =
            b"stdin\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        f = stdin
    } else {
        /* ignore repeated files. */
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
                 __glibc_reserved: [0; 3],}; /* No conffile, all done. */
        if hard_opt == 0 as libc::c_int &&
               stat(file, &mut statbuf) == 0 as libc::c_int {
            let mut r = 0 as *mut fileread;
            r = filesread;
            while !r.is_null() {
                if (*r).dev == statbuf.st_dev && (*r).ino == statbuf.st_ino {
                    return 1 as libc::c_int
                }
                r = (*r).next
            }
            r =
                safe_malloc(::std::mem::size_of::<fileread>() as
                                libc::c_ulong) as *mut fileread;
            (*r).next = filesread;
            filesread = r;
            (*r).dev = statbuf.st_dev;
            (*r).ino = statbuf.st_ino
        }
        f = fopen(file, b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            if errno == ENOENT && nofile_ok != 0 {
                return 1 as libc::c_int
            } else {
                let mut str =
                    b"cannot read %s: %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char;
                if hard_opt != 0 as libc::c_int {
                    my_syslog(LOG_ERR, str, file, strerror(errno));
                    return 0 as libc::c_int
                } else { die(str, file, EC_FILE); }
            }
        }
    }
    read_file(file, f, hard_opt);
    return 1 as libc::c_int;
}
/* expand any name which is a directory */
#[no_mangle]
#[c2rust::src_loc = "4777:1"]
pub unsafe extern "C" fn expand_filelist(mut list: *mut hostsfile)
 -> *mut hostsfile {
    let mut i: libc::c_uint = 0;
    let mut ah = 0 as *mut hostsfile;
    /* find largest used index */
    i = SRC_AH as libc::c_uint;
    ah = list;
    while !ah.is_null() {
        if i <= (*ah).index {
            i = (*ah).index.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
        if (*ah).flags & AH_DIR != 0 {
            (*ah).flags |= AH_INACTIVE
        } else { (*ah).flags &= !AH_INACTIVE }
        ah = (*ah).next
    }
    ah = list;
    while !ah.is_null() {
        if (*ah).flags & AH_INACTIVE == 0 {
            let mut buf =
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
            if stat((*ah).fname, &mut buf) != -(1 as libc::c_int) &&
                   buf.st_mode & __S_IFMT as libc::c_uint ==
                       0o40000 as libc::c_int as libc::c_uint {
                let mut dir_stream = 0 as *mut DIR;
                let mut ent = 0 as *mut dirent;
                /* don't read this as a file */
                (*ah).flags |= AH_INACTIVE;
                dir_stream = opendir((*ah).fname);
                if dir_stream.is_null() {
                    my_syslog(LOG_ERR,
                              b"cannot access directory %s: %s\x00" as
                                  *const u8 as *const libc::c_char,
                              (*ah).fname, strerror(errno));
                } else {
                    loop  {
                        ent = readdir(dir_stream);
                        if ent.is_null() { break ; }
                        let mut lendir = strlen((*ah).fname);
                        let mut lenfile = strlen((*ent).d_name.as_mut_ptr());
                        let mut ah1 = 0 as *mut hostsfile;
                        let mut path = 0 as *mut libc::c_char;
                        /* ignore emacs backups and dotfiles */
                        if lenfile == 0 as libc::c_int as libc::c_ulong ||
                               (*ent).d_name[lenfile.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                                                 as usize] as libc::c_int ==
                                   '~' as i32 ||
                               (*ent).d_name[0 as libc::c_int as usize] as
                                   libc::c_int == '#' as i32 &&
                                   (*ent).d_name[lenfile.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                                     as usize] as libc::c_int
                                       == '#' as i32 ||
                               (*ent).d_name[0 as libc::c_int as usize] as
                                   libc::c_int == '.' as i32 {
                            continue ;
                        }
                        /* see if we have an existing record.
		       dir is ah->fname 
		       file is ent->d_name
		       path to match is ah1->fname */
                        ah1 = list;
                        while !ah1.is_null() {
                            if lendir < strlen((*ah1).fname) &&
                                   strstr((*ah1).fname, (*ah).fname) ==
                                       (*ah1).fname &&
                                   *(*ah1).fname.offset(lendir as isize) as
                                       libc::c_int == '/' as i32 &&
                                   strcmp((*ah1).fname.offset(lendir as
                                                                  isize).offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                          (*ent).d_name.as_mut_ptr()) ==
                                       0 as libc::c_int {
                                (*ah1).flags &= !AH_INACTIVE;
                                break ;
                            } else { ah1 = (*ah1).next }
                        }
                        /* make new record */
                        if ah1.is_null() {
                            ah1 =
                                whine_malloc(::std::mem::size_of::<hostsfile>()
                                                 as libc::c_ulong) as
                                    *mut hostsfile;
                            if ah1.is_null() { continue ; }
                            path =
                                whine_malloc(lendir.wrapping_add(lenfile).wrapping_add(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong))
                                    as *mut libc::c_char;
                            if path.is_null() {
                                free(ah1 as *mut libc::c_void);
                                continue ;
                            } else {
                                strcpy(path, (*ah).fname);
                                strcat(path,
                                       b"/\x00" as *const u8 as
                                           *const libc::c_char);
                                strcat(path, (*ent).d_name.as_mut_ptr());
                                (*ah1).fname = path;
                                let fresh33 = i;
                                i = i.wrapping_add(1);
                                (*ah1).index = fresh33;
                                (*ah1).flags = AH_DIR;
                                (*ah1).next = list;
                                list = ah1
                            }
                        }
                        /* inactivate record if not regular file */
                        if (*ah1).flags & AH_DIR != 0 &&
                               stat((*ah1).fname, &mut buf) !=
                                   -(1 as libc::c_int) &&
                               !(buf.st_mode & __S_IFMT as libc::c_uint ==
                                     0o100000 as libc::c_int as libc::c_uint)
                           {
                            (*ah1).flags |= AH_INACTIVE
                        }
                    }
                    closedir(dir_stream);
                }
            }
        }
        ah = (*ah).next
    }
    return list;
}
#[no_mangle]
#[c2rust::src_loc = "4877:1"]
pub unsafe extern "C" fn read_servers_file() {
    let mut f = 0 as *mut FILE;
    f =
        fopen((*dnsmasq_daemon).servers_file,
              b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        my_syslog(LOG_ERR,
                  b"cannot read %s: %s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).servers_file,
                  strerror(errno));
        return
    }
    mark_servers(SERV_FROM_FILE);
    cleanup_servers();
    read_file((*dnsmasq_daemon).servers_file, f, LOPT_REV_SERV);
}
#[c2rust::src_loc = "4895:1"]
unsafe extern "C" fn clear_dynamic_conf() {
    let mut configs = 0 as *mut dhcp_config;
    let mut cp = 0 as *mut dhcp_config;
    let mut up = 0 as *mut *mut dhcp_config;
    /* remove existing... */
    up = &mut (*dnsmasq_daemon).dhcp_conf;
    configs = (*dnsmasq_daemon).dhcp_conf;
    while !configs.is_null() {
        cp = (*configs).next;
        if (*configs).flags & CONFIG_BANK as libc::c_uint != 0 {
            let mut mac = 0 as *mut hwaddr_config;
            let mut tmp = 0 as *mut hwaddr_config;
            let mut list = 0 as *mut dhcp_netid_list;
            let mut tmplist = 0 as *mut dhcp_netid_list;
            mac = (*configs).hwaddr;
            while !mac.is_null() {
                tmp = (*mac).next;
                free(mac as *mut libc::c_void);
                mac = tmp
            }
            if (*configs).flags & CONFIG_CLID as libc::c_uint != 0 {
                free((*configs).clid as *mut libc::c_void);
            }
            list = (*configs).netid;
            while !list.is_null() {
                free((*list).list as *mut libc::c_void);
                tmplist = (*list).next;
                free(list as *mut libc::c_void);
                list = tmplist
            }
            if (*configs).flags & CONFIG_NAME as libc::c_uint != 0 {
                free((*configs).hostname as *mut libc::c_void);
            }
            *up = (*configs).next;
            free(configs as *mut libc::c_void);
        } else { up = &mut (*configs).next }
        configs = cp
    };
}
#[c2rust::src_loc = "4936:1"]
unsafe extern "C" fn clear_dynamic_opt() {
    let mut opts_0 = 0 as *mut dhcp_opt;
    let mut cp = 0 as *mut dhcp_opt;
    let mut up = 0 as *mut *mut dhcp_opt;
    let mut id = 0 as *mut dhcp_netid;
    let mut next = 0 as *mut dhcp_netid;
    up = &mut (*dnsmasq_daemon).dhcp_opts;
    opts_0 = (*dnsmasq_daemon).dhcp_opts;
    while !opts_0.is_null() {
        cp = (*opts_0).next;
        if (*opts_0).flags & DHOPT_BANK != 0 {
            if (*opts_0).flags & DHOPT_VENDOR != 0 {
                free((*opts_0).u.vendor_class as *mut libc::c_void);
            }
            free((*opts_0).val as *mut libc::c_void);
            id = (*opts_0).netid;
            while !id.is_null() {
                next = (*id).next;
                free((*id).net as *mut libc::c_void);
                free(id as *mut libc::c_void);
                id = next
            }
            *up = (*opts_0).next;
            free(opts_0 as *mut libc::c_void);
        } else { up = &mut (*opts_0).next }
        opts_0 = cp
    };
}
#[no_mangle]
#[c2rust::src_loc = "4964:1"]
pub unsafe extern "C" fn reread_dhcp() {
    let mut hf = 0 as *mut hostsfile;
    /* Do these even if there is no daemon->dhcp_hosts_file or
      daemon->dhcp_opts_file since entries may have been created by the
      inotify dynamic file reading system. */
    clear_dynamic_conf();
    clear_dynamic_opt();
    if !(*dnsmasq_daemon).dhcp_hosts_file.is_null() {
        (*dnsmasq_daemon).dhcp_hosts_file =
            expand_filelist((*dnsmasq_daemon).dhcp_hosts_file);
        hf = (*dnsmasq_daemon).dhcp_hosts_file;
        while !hf.is_null() {
            if (*hf).flags & AH_INACTIVE == 0 {
                if one_file((*hf).fname, LOPT_BANK) != 0 {
                    my_syslog(MS_DHCP | LOG_INFO,
                              b"read %s\x00" as *const u8 as
                                  *const libc::c_char, (*hf).fname);
                }
            }
            hf = (*hf).next
        }
    }
    if !(*dnsmasq_daemon).dhcp_opts_file.is_null() {
        (*dnsmasq_daemon).dhcp_opts_file =
            expand_filelist((*dnsmasq_daemon).dhcp_opts_file);
        hf = (*dnsmasq_daemon).dhcp_opts_file;
        while !hf.is_null() {
            if (*hf).flags & AH_INACTIVE == 0 {
                if one_file((*hf).fname, LOPT_OPTS) != 0 {
                    my_syslog(MS_DHCP | LOG_INFO,
                              b"read %s\x00" as *const u8 as
                                  *const libc::c_char, (*hf).fname);
                }
            }
            hf = (*hf).next
        }
    }
    /* Setup notify and read pre-existing files. */
    set_dynamic_inotify(AH_DHCP_HST | AH_DHCP_OPT, 0 as libc::c_int,
                        NULL_0 as *mut *mut crec, 0 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "5004:1"]
pub unsafe extern "C" fn read_opts(mut argc: libc::c_int,
                                   mut argv: *mut *mut libc::c_char,
                                   mut compile_opts: *mut libc::c_char) {
    let mut argbuf_size = MAXDNAME as size_t;
    let mut argbuf = opt_malloc(argbuf_size) as *mut libc::c_char;
    let mut buff = opt_malloc(MAXDNAME as size_t) as *mut libc::c_char;
    let mut option: libc::c_int = 0;
    let mut testmode = 0 as libc::c_int;
    let mut arg = 0 as *mut libc::c_char;
    let mut conffile = NULL_0 as *mut libc::c_char;
    opterr = 0 as libc::c_int;
    dnsmasq_daemon =
        opt_malloc(::std::mem::size_of::<dnsmasq_daemon>() as libc::c_ulong)
            as *mut dnsmasq_daemon;
    memset(dnsmasq_daemon as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dnsmasq_daemon>() as libc::c_ulong);
    (*dnsmasq_daemon).namebuff = buff;
    /* Set defaults - everything else is zero or NULL */
    (*dnsmasq_daemon).cachesize = CACHESIZ;
    (*dnsmasq_daemon).ftabsize = FTABSIZ;
    (*dnsmasq_daemon).port = NAMESERVER_PORT;
    (*dnsmasq_daemon).dhcp_client_port = DHCP_CLIENT_PORT;
    (*dnsmasq_daemon).dhcp_server_port = DHCP_SERVER_PORT;
    (*dnsmasq_daemon).default_resolv.is_default = 1 as libc::c_int;
    (*dnsmasq_daemon).default_resolv.name = RESOLVFILE.as_mut_ptr();
    (*dnsmasq_daemon).resolv_files = &mut (*dnsmasq_daemon).default_resolv;
    (*dnsmasq_daemon).username = CHUSER.as_mut_ptr();
    (*dnsmasq_daemon).runfile = RUNFILE.as_mut_ptr();
    (*dnsmasq_daemon).dhcp_max = MAXLEASES;
    (*dnsmasq_daemon).tftp_max = TFTP_MAX_CONNECTIONS;
    (*dnsmasq_daemon).edns_pktsz = EDNS_PKTSZ as libc::c_ushort;
    (*dnsmasq_daemon).log_fac = -(1 as libc::c_int);
    (*dnsmasq_daemon).auth_ttl = AUTH_TTL as libc::c_ulong;
    (*dnsmasq_daemon).soa_refresh = SOA_REFRESH as libc::c_ulong;
    (*dnsmasq_daemon).soa_retry = SOA_RETRY as libc::c_ulong;
    (*dnsmasq_daemon).soa_expiry = SOA_EXPIRY as libc::c_ulong;
    (*dnsmasq_daemon).max_port = MAX_PORT as libc::c_int;
    (*dnsmasq_daemon).min_port = MIN_PORT;
    add_txt(b"version.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"dnsmasq-2.84rc2\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as libc::c_int);
    add_txt(b"authors.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"Simon Kelley\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as libc::c_int);
    add_txt(b"copyright.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, COPYRIGHT.as_mut_ptr(), 0 as libc::c_int);
    add_txt(b"cachesize.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_CACHESIZE);
    add_txt(b"insertions.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_INSERTS);
    add_txt(b"evictions.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_EVICTIONS);
    add_txt(b"misses.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_MISSES);
    add_txt(b"hits.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_HITS);
    add_txt(b"auth.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_AUTH);
    add_txt(b"servers.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, NULL_0 as *mut libc::c_char,
            TXT_STAT_SERVERS);
    loop  {
        option =
            getopt_long(argc, argv, OPTSTRING.as_ptr(), opts.as_ptr(),
                        NULL_0 as *mut libc::c_int);
        if option == -(1 as libc::c_int) {
            while optind < argc {
                let mut c =
                    *argv.offset(optind as isize) as *mut libc::c_uchar;
                while *c as libc::c_int != 0 as libc::c_int {
                    if *(*__ctype_b_loc()).offset(*c as libc::c_int as isize)
                           as libc::c_int &
                           _ISspace as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        die(b"junk found in command line\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            NULL_0 as *mut libc::c_char, EC_BADCONF);
                    }
                    c = c.offset(1)
                }
                optind += 1
            }
            break ;
        } else {
            /* Copy optarg so that argv doesn't get changed */
            if !optarg.is_null() {
                if strlen(optarg) >= argbuf_size {
                    free(argbuf as *mut libc::c_void);
                    argbuf_size =
                        strlen(optarg).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong);
                    argbuf = opt_malloc(argbuf_size) as *mut libc::c_char
                }
                safe_strncpy(argbuf, optarg, argbuf_size);
                arg = argbuf
            } else { arg = NULL_0 as *mut libc::c_char }
            /* command-line only stuff */
            if option == LOPT_TEST {
                testmode = 1 as libc::c_int
            } else if option == 'w' as i32 {
                if argc == 3 as libc::c_int &&
                       strcmp(*argv.offset(2 as libc::c_int as isize),
                              b"dhcp\x00" as *const u8 as *const libc::c_char)
                           == 0 as libc::c_int {
                    display_opts();
                } else if argc == 3 as libc::c_int &&
                              strcmp(*argv.offset(2 as libc::c_int as isize),
                                     b"dhcp6\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                    display_opts6();
                } else { do_usage(); }
                exit(0 as libc::c_int);
            } else {
                if option == 'v' as i32 {
                    printf(b"Dnsmasq version %s  %s\n\x00" as *const u8 as
                               *const libc::c_char, VERSION.as_ptr(),
                           COPYRIGHT.as_ptr());
                    printf(b"Compile time options: %s\n\n\x00" as *const u8 as
                               *const libc::c_char, compile_opts);
                    printf(b"This software comes with ABSOLUTELY NO WARRANTY.\n\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"Dnsmasq is free software, and you are welcome to redistribute it\n\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"under the terms of the GNU General Public License, version 2 or 3.\n\x00"
                               as *const u8 as *const libc::c_char);
                    exit(0 as libc::c_int);
                } else {
                    if option == 'C' as i32 {
                        if conffile.is_null() {
                            conffile = opt_string_alloc(arg)
                        } else {
                            let mut extra = opt_string_alloc(arg);
                            one_file(extra, 0 as libc::c_int);
                            free(extra as *mut libc::c_void);
                        }
                    } else if one_opt(option, arg, (*dnsmasq_daemon).namebuff,
                                      b"try --help\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char, 1 as libc::c_int,
                                      0 as libc::c_int) == 0 {
                        die(b"bad command line options: %s\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*dnsmasq_daemon).namebuff, EC_BADCONF);
                    }
                }
            }
        }
    }
    free(argbuf as *mut libc::c_void);
    if !conffile.is_null() {
        one_file(conffile, 0 as libc::c_int);
        free(conffile as *mut libc::c_void);
    } else { one_file(CONFFILE.as_mut_ptr(), '7' as i32); }
    /* port might not be known when the address is parsed - fill in here */
    if !(*dnsmasq_daemon).servers.is_null() {
        let mut tmp = 0 as *mut server;
        tmp = (*dnsmasq_daemon).servers;
        while !tmp.is_null() {
            if (*tmp).flags & SERV_HAS_SOURCE == 0 {
                if (*tmp).source_addr.sa.sa_family as libc::c_int == AF_INET {
                    (*tmp).source_addr.in_0.sin_port =
                        __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
                } else if (*tmp).source_addr.sa.sa_family as libc::c_int ==
                              AF_INET6 {
                    (*tmp).source_addr.in6.sin6_port =
                        __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
                }
            }
            tmp = (*tmp).next
        }
    }
    if !(*dnsmasq_daemon).host_records.is_null() {
        let mut hr = 0 as *mut host_record;
        hr = (*dnsmasq_daemon).host_records;
        while !hr.is_null() {
            if (*hr).ttl == -(1 as libc::c_int) {
                (*hr).ttl = (*dnsmasq_daemon).local_ttl as libc::c_int
            }
            hr = (*hr).next
        }
    }
    if !(*dnsmasq_daemon).cnames.is_null() {
        let mut cn = 0 as *mut cname;
        let mut cn2 = 0 as *mut cname;
        let mut cn3 = 0 as *mut cname;
        /* Fill in TTL for CNAMES now we have local_ttl.
	 Also prepare to do loop detection. */
        cn = (*dnsmasq_daemon).cnames;
        while !cn.is_null() {
            if (*cn).ttl == -(1 as libc::c_int) {
                (*cn).ttl = (*dnsmasq_daemon).local_ttl as libc::c_int
            }
            (*cn).flag = 0 as libc::c_int;
            (*cn).targetp = NULL_0 as *mut cname;
            cn2 = (*dnsmasq_daemon).cnames;
            while !cn2.is_null() {
                if hostname_isequal((*cn).target, (*cn2).alias) != 0 {
                    (*cn).targetp = cn2;
                    break ;
                } else { cn2 = (*cn2).next }
            }
            cn = (*cn).next
        }
        /* Find any CNAME loops.*/
        cn = (*dnsmasq_daemon).cnames;
        while !cn.is_null() {
            cn2 = (*cn).targetp;
            while !cn2.is_null() {
                if (*cn2).flag == NOLOOP { break ; }
                if (*cn2).flag == TESTLOOP {
                    die(b"CNAME loop involving %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*cn).alias, EC_BADCONF);
                }
                (*cn2).flag = TESTLOOP;
                cn2 = (*cn2).targetp
            }
            cn3 = (*cn).targetp;
            while cn3 != cn2 { (*cn3).flag = NOLOOP; cn3 = (*cn3).targetp }
            cn = (*cn).next
        }
    }
    if !(*dnsmasq_daemon).if_addrs.is_null() {
        let mut tmp_0 = 0 as *mut iname;
        tmp_0 = (*dnsmasq_daemon).if_addrs;
        while !tmp_0.is_null() {
            if (*tmp_0).addr.sa.sa_family as libc::c_int == AF_INET {
                (*tmp_0).addr.in_0.sin_port =
                    __bswap_16((*dnsmasq_daemon).port as __uint16_t)
            } else if (*tmp_0).addr.sa.sa_family as libc::c_int == AF_INET6 {
                (*tmp_0).addr.in6.sin6_port =
                    __bswap_16((*dnsmasq_daemon).port as __uint16_t)
            }
            tmp_0 = (*tmp_0).next
        }
    }
    /* create default, if not specified */
    if !(*dnsmasq_daemon).authserver.is_null() &&
           (*dnsmasq_daemon).hostmaster.is_null() {
        strcpy(buff, b"hostmaster.\x00" as *const u8 as *const libc::c_char);
        strcat(buff, (*dnsmasq_daemon).authserver);
        (*dnsmasq_daemon).hostmaster = opt_string_alloc(buff)
    }
    if (*dnsmasq_daemon).dhcp_pxe_vendors.is_null() {
        (*dnsmasq_daemon).dhcp_pxe_vendors =
            opt_malloc(::std::mem::size_of::<dhcp_pxe_vendor>() as
                           libc::c_ulong) as *mut dhcp_pxe_vendor;
        (*(*dnsmasq_daemon).dhcp_pxe_vendors).data =
            opt_string_alloc(DHCP_PXE_DEF_VENDOR.as_ptr());
        (*(*dnsmasq_daemon).dhcp_pxe_vendors).next =
            NULL_0 as *mut dhcp_pxe_vendor
    }
    /* only one of these need be specified: the other defaults to the host-name */
    if (*dnsmasq_daemon).options[(10 as libc::c_int as
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
           != 0 || !(*dnsmasq_daemon).mxnames.is_null() ||
           !(*dnsmasq_daemon).mxtarget.is_null() {
        let mut mx = 0 as *mut mx_srv_record;
        if gethostname(buff, MAXDNAME as size_t) == -(1 as libc::c_int) {
            die(b"cannot get host-name: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                NULL_0 as *mut libc::c_char, EC_MISC);
        }
        mx = (*dnsmasq_daemon).mxnames;
        while !mx.is_null() {
            if (*mx).issrv == 0 && hostname_isequal((*mx).name, buff) != 0 {
                break ;
            }
            mx = (*mx).next
        }
        if (!(*dnsmasq_daemon).mxtarget.is_null() ||
                (*dnsmasq_daemon).options[(10 as libc::c_int as
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
                    != 0) && mx.is_null() {
            mx =
                opt_malloc(::std::mem::size_of::<mx_srv_record>() as
                               libc::c_ulong) as *mut mx_srv_record;
            (*mx).next = (*dnsmasq_daemon).mxnames;
            (*mx).issrv = 0 as libc::c_int;
            (*mx).target = NULL_0 as *mut libc::c_char;
            (*mx).name = opt_string_alloc(buff);
            (*dnsmasq_daemon).mxnames = mx
        }
        if (*dnsmasq_daemon).mxtarget.is_null() {
            (*dnsmasq_daemon).mxtarget = opt_string_alloc(buff)
        }
        mx = (*dnsmasq_daemon).mxnames;
        while !mx.is_null() {
            if (*mx).issrv == 0 && (*mx).target.is_null() {
                (*mx).target = (*dnsmasq_daemon).mxtarget
            }
            mx = (*mx).next
        }
    }
    if (*dnsmasq_daemon).options[(8 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (8 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 && !(*dnsmasq_daemon).resolv_files.is_null() &&
           !(*(*dnsmasq_daemon).resolv_files).next.is_null() &&
           (*dnsmasq_daemon).options[(5 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (5 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        die(b"only one resolv.conf file allowed in no-poll mode.\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            NULL_0 as *mut libc::c_char, EC_BADCONF);
    }
    if (*dnsmasq_daemon).options[(15 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (15 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut line = 0 as *mut libc::c_char;
        let mut f = 0 as *mut FILE;
        if (*dnsmasq_daemon).options[(8 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (8 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || (*dnsmasq_daemon).resolv_files.is_null() ||
               !(*(*dnsmasq_daemon).resolv_files).next.is_null() {
            die(b"must have exactly one resolv.conf to read domain from.\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                NULL_0 as *mut libc::c_char, EC_BADCONF);
        }
        f =
            fopen((*(*dnsmasq_daemon).resolv_files).name,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            die(b"failed to read %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*(*dnsmasq_daemon).resolv_files).name, EC_FILE);
        }
        loop  {
            line = fgets(buff, MAXDNAME, f);
            if line.is_null() { break ; }
            let mut token =
                strtok(line,
                       b" \t\n\r\x00" as *const u8 as *const libc::c_char);
            if token.is_null() ||
                   strcmp(token,
                          b"search\x00" as *const u8 as *const libc::c_char)
                       != 0 as libc::c_int {
                continue ;
            }
            token =
                strtok(NULL_0 as *mut libc::c_char,
                       b" \t\n\r\x00" as *const u8 as *const libc::c_char);
            if !token.is_null() &&
                   {
                       (*dnsmasq_daemon).domain_suffix =
                           canonicalise_opt(token);
                       !(*dnsmasq_daemon).domain_suffix.is_null()
                   } {
                break ;
            }
        }
        fclose(f);
        if (*dnsmasq_daemon).domain_suffix.is_null() {
            die(b"no search directive found in %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*(*dnsmasq_daemon).resolv_files).name, EC_MISC);
        }
    }
    if !(*dnsmasq_daemon).domain_suffix.is_null() {
        /* add domain for any srv record without one. */
        let mut srv = 0 as *mut mx_srv_record;
        srv = (*dnsmasq_daemon).mxnames;
        while !srv.is_null() {
            if (*srv).issrv != 0 && !strchr((*srv).name, '.' as i32).is_null()
                   &&
                   strchr((*srv).name, '.' as i32) ==
                       strrchr((*srv).name, '.' as i32) {
                strcpy(buff, (*srv).name);
                strcat(buff, b".\x00" as *const u8 as *const libc::c_char);
                strcat(buff, (*dnsmasq_daemon).domain_suffix);
                free((*srv).name as *mut libc::c_void);
                (*srv).name = opt_string_alloc(buff)
            }
            srv = (*srv).next
        }
    } else if (*dnsmasq_daemon).options[(20 as libc::c_int as
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
        die(b"there must be a default domain when --dhcp-fqdn is set\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            NULL_0 as *mut libc::c_char, EC_BADCONF);
    }
    /* If there's access-control config, then ignore --local-service, it's intended
     as a system default to keep otherwise unconfigured installations safe. */
    if !(*dnsmasq_daemon).if_names.is_null() ||
           !(*dnsmasq_daemon).if_except.is_null() ||
           !(*dnsmasq_daemon).if_addrs.is_null() ||
           !(*dnsmasq_daemon).authserver.is_null() {
        reset_option_bool(OPT_LOCAL_SERVICE as libc::c_uint);
    }
    if testmode != 0 {
        fprintf(stderr,
                b"dnsmasq: %s.\n\x00" as *const u8 as *const libc::c_char,
                b"syntax check OK\x00" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    };
}
#[c2rust::src_loc = "5176:9"]
pub const NOLOOP: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "5177:9"]
pub const TESTLOOP: libc::c_int = 2 as libc::c_int;
