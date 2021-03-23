#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __xstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                 __stat_buf: *mut stat64) -> libc::c_int;
    #[no_mangle]
    fn __fxstat64(__ver: libc::c_int, __fildes: libc::c_int,
                  __stat_buf: *mut stat64) -> libc::c_int;
    #[no_mangle]
    fn __fxstatat(__ver: libc::c_int, __fildes: libc::c_int,
                  __filename: *const libc::c_char, __stat_buf: *mut stat,
                  __flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __fxstatat64(__ver: libc::c_int, __fildes: libc::c_int,
                    __filename: *const libc::c_char, __stat_buf: *mut stat64,
                    __flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __lxstat(__ver: libc::c_int, __filename: *const libc::c_char,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __lxstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                  __stat_buf: *mut stat64) -> libc::c_int;
    #[no_mangle]
    fn __xmknod(__ver: libc::c_int, __path: *const libc::c_char,
                __mode: __mode_t, __dev: *mut __dev_t) -> libc::c_int;
    #[no_mangle]
    fn __xmknodat(__ver: libc::c_int, __fd: libc::c_int,
                  __path: *const libc::c_char, __mode: __mode_t,
                  __dev: *mut __dev_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __uflow(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __strtol_internal(__nptr: *const libc::c_char,
                         __endptr: *mut *mut libc::c_char,
                         __base: libc::c_int, __group: libc::c_int)
     -> libc::c_long;
    #[no_mangle]
    fn __strtoul_internal(__nptr: *const libc::c_char,
                          __endptr: *mut *mut libc::c_char,
                          __base: libc::c_int, __group: libc::c_int)
     -> libc::c_ulong;
    #[no_mangle]
    fn __wcstol_internal(__nptr: *const __gwchar_t,
                         __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
                         __group: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn __wcstoul_internal(__nptr: *const __gwchar_t,
                          __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
                          __group: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    static mut dnsmasq_daemon: *mut dnsmasq_daemon;
    #[no_mangle]
    fn expand_buf(iov: *mut iovec, size: size_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct C2RustUnnamed_0 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub target: *mut blockdata,
    pub targetlen: libc::c_ushort,
    pub srvport: libc::c_ushort,
    pub priority: libc::c_ushort,
    pub weight: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockdata {
    pub next: *mut blockdata,
    pub key: [libc::c_uchar; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub target: C2RustUnnamed_5,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub cache: *mut crec,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub union C2RustUnnamed_6 {
    pub sname: [libc::c_char; 50],
    pub bname: *mut bigname,
    pub namep: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union bigname {
    pub name: [libc::c_char; 1025],
    pub next: *mut bigname,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bogus_addr {
    pub addr: in_addr,
    pub next: *mut bogus_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct doctor {
    pub in_0: in_addr,
    pub end: in_addr,
    pub out: in_addr,
    pub mask: in_addr,
    pub next: *mut doctor,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct ptr_record {
    pub name: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub next: *mut ptr_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct addrlist {
    pub addr: all_addr,
    pub flags: libc::c_int,
    pub prefixlen: libc::c_int,
    pub decline_time: time_t,
    pub next: *mut addrlist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_zone {
    pub domain: *mut libc::c_char,
    pub interface_names: *mut auth_name_list,
    pub subnet: *mut addrlist,
    pub exclude: *mut addrlist,
    pub next: *mut auth_zone,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_name_list {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub next: *mut auth_name_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct name_list {
    pub name: *mut libc::c_char,
    pub next: *mut name_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface_name {
    pub name: *mut libc::c_char,
    pub intr: *mut libc::c_char,
    pub family: libc::c_int,
    pub addr: *mut addrlist,
    pub next: *mut interface_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mysockaddr {
    pub sa: sockaddr,
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct randfd {
    pub fd: libc::c_int,
    pub refcount: libc::c_ushort,
    pub family: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct ipsets {
    pub sets: *mut *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub next: *mut ipsets,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct iname {
    pub name: *mut libc::c_char,
    pub addr: mysockaddr,
    pub used: libc::c_int,
    pub next: *mut iname,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mysubnet {
    pub addr: mysockaddr,
    pub addr_used: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct hostsfile {
    pub next: *mut hostsfile,
    pub flags: libc::c_int,
    pub fname: *mut libc::c_char,
    pub wd: libc::c_int,
    pub index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct dhcp_netid {
    pub net: *mut libc::c_char,
    pub next: *mut dhcp_netid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_netid_list {
    pub list: *mut dhcp_netid,
    pub next: *mut dhcp_netid_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag_if {
    pub set: *mut dhcp_netid_list,
    pub tag: *mut dhcp_netid,
    pub next: *mut tag_if,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delay_config {
    pub delay: libc::c_int,
    pub netid: *mut dhcp_netid,
    pub next: *mut delay_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwaddr_config {
    pub hwaddr_len: libc::c_int,
    pub hwaddr_type: libc::c_int,
    pub hwaddr: [libc::c_uchar; 16],
    pub wildcard_mask: libc::c_uint,
    pub next: *mut hwaddr_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub union C2RustUnnamed_7 {
    pub encap: libc::c_int,
    pub wildcard_mask: libc::c_uint,
    pub vendor_class: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct dhcp_match_name {
    pub name: *mut libc::c_char,
    pub wildcard: libc::c_int,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_match_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct dhcp_pxe_vendor {
    pub data: *mut libc::c_char,
    pub next: *mut dhcp_pxe_vendor,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct dhcp_bridge {
    pub iface: [libc::c_char; 16],
    pub alias: *mut dhcp_bridge,
    pub next: *mut dhcp_bridge,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct ping_result {
    pub addr: in_addr,
    pub time: time_t,
    pub hash: libc::c_uint,
    pub next: *mut ping_result,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct addr_list {
    pub addr: in_addr,
    pub next: *mut addr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_prefix {
    pub interface: *mut libc::c_char,
    pub prefix: *mut libc::c_char,
    pub missing: libc::c_int,
    pub next: *mut tftp_prefix,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
                (__bsx as libc::c_int & 0xff as libc::c_int) <<
                    8 as libc::c_int) as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
               (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
               (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
               (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong &
                 0xff00000000000000 as libc::c_ulonglong) >> 56 as libc::c_int
                |
                (__bsx as libc::c_ulonglong &
                     0xff000000000000 as libc::c_ulonglong) >>
                    40 as libc::c_int |
                (__bsx as libc::c_ulonglong &
                     0xff0000000000 as libc::c_ulonglong) >> 24 as libc::c_int
                |
                (__bsx as libc::c_ulonglong &
                     0xff00000000 as libc::c_ulonglong) >> 8 as libc::c_int |
                (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
                    << 8 as libc::c_int |
                (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
                    << 24 as libc::c_int |
                (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) <<
                    40 as libc::c_int |
                (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) <<
                    56 as libc::c_int) as __uint64_t;
}
#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: *mut msghdr,
                                   mut __cmsg: *mut cmsghdr) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
       {
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
                *mut libc::c_uchar).offset((*__mhdr).msg_controllen as isize)
           ||
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
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat)
 -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                            mut __statbuf: *mut stat64) -> libc::c_int {
    return __xstat64(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat64(mut __fd: libc::c_int,
                             mut __statbuf: *mut stat64) -> libc::c_int {
    return __fxstat64(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(mut __fd: libc::c_int,
                             mut __filename: *const libc::c_char,
                             mut __statbuf: *mut stat,
                             mut __flag: libc::c_int) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn fstatat64(mut __fd: libc::c_int,
                               mut __filename: *const libc::c_char,
                               mut __statbuf: *mut stat64,
                               mut __flag: libc::c_int) -> libc::c_int {
    return __fxstatat64(1 as libc::c_int, __fd, __filename, __statbuf,
                        __flag);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
                           mut __statbuf: *mut stat) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
                             mut __statbuf: *mut stat64) -> libc::c_int {
    return __lxstat64(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
                           mut __mode: __mode_t, mut __dev: __dev_t)
 -> libc::c_int {
    return __xmknod(0 as libc::c_int, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn mknodat(mut __fd: libc::c_int,
                             mut __path: *const libc::c_char,
                             mut __mode: __mode_t, mut __dev: __dev_t)
 -> libc::c_int {
    return __xmknodat(0 as libc::c_int, __fd, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: ::std::ffi::VaList) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as
                  libc::c_long != 0 {
               __uflow(__fp)
           } else {
               let fresh0 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh0 as *mut libc::c_uchar) as libc::c_int
           };
}
#[inline]
unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as libc::c_int
                  as libc::c_long != 0 {
               __uflow(stdin)
           } else {
               let fresh1 = (*stdin)._IO_read_ptr;
               (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
               *(fresh1 as *mut libc::c_uchar) as libc::c_int
           };
}
#[inline]
unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as
                  libc::c_long != 0 {
               __uflow(__fp)
           } else {
               let fresh2 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh2 as *mut libc::c_uchar) as libc::c_int
           };
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
                                    mut __stream: *mut FILE) -> libc::c_int {
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
unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
                                   mut __stream: *mut FILE) -> libc::c_int {
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
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as
                  libc::c_int as libc::c_long != 0 {
               __overflow(stdout, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh5 = (*stdout)._IO_write_ptr;
               (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
               *fresh5 = __c as libc::c_char;
               *fresh5 as libc::c_uchar as libc::c_int
           };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as
               libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as
               libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int);
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
 -> libc::c_longlong {
    return strtoll(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                   10 as libc::c_int);
}
#[inline]
unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                             mut __base: *const libc::c_void,
                             mut __nmemb: size_t, mut __size: size_t,
                             mut __compar: __compar_fn_t)
 -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
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
                                                 isize) as *mut libc::c_void;
        __comparison =
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                   __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else { return __p as *mut libc::c_void }
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
               *(*__ctype_tolower_loc()).offset(__c as isize)
           } else { __c };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
               *(*__ctype_toupper_loc()).offset(__c as isize)
           } else { __c };
}
#[inline]
unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                             mut __n: *mut size_t, mut __stream: *mut FILE)
 -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
                               mut endptr: *mut *mut libc::c_char,
                               mut base: libc::c_int) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
                               mut endptr: *mut *mut libc::c_char,
                               mut base: libc::c_int) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
                               mut endptr: *mut *mut __gwchar_t,
                               mut base: libc::c_int) -> intmax_t {
    return __wcstol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                               mut endptr: *mut *mut __gwchar_t,
                               mut base: libc::c_int) -> uintmax_t {
    return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
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
static mut outpacket_counter: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn end_opt6(mut container: libc::c_int) {
    let mut p: *mut libc::c_void =
        (*dnsmasq_daemon).outpacket.iov_base.offset(container as
                                                        isize).offset(2 as
                                                                          libc::c_int
                                                                          as
                                                                          isize);
    let mut len: u16_0 =
        outpacket_counter.wrapping_sub(container as
                                           libc::c_ulong).wrapping_sub(4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
            as u16_0;
    let mut t_s: u16_0 = len;
    let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
    let fresh6 = t_cp;
    t_cp = t_cp.offset(1);
    *fresh6 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    *t_cp = t_s as libc::c_uchar;
    p = p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn reset_counter() {
    /* Clear out buffer when starting from beginning */
    if !(*dnsmasq_daemon).outpacket.iov_base.is_null() {
        memset((*dnsmasq_daemon).outpacket.iov_base, 0 as libc::c_int,
               (*dnsmasq_daemon).outpacket.iov_len);
    }
    save_counter(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn save_counter(mut newval: libc::c_int)
 -> libc::c_int {
    let mut ret: libc::c_int = outpacket_counter as libc::c_int;
    if newval != -(1 as libc::c_int) { outpacket_counter = newval as size_t }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn expand(mut headroom: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if expand_buf(&mut (*dnsmasq_daemon).outpacket,
                  outpacket_counter.wrapping_add(headroom)) != 0 {
        ret =
            (*dnsmasq_daemon).outpacket.iov_base.offset(outpacket_counter as
                                                            isize);
        outpacket_counter =
            (outpacket_counter as libc::c_ulong).wrapping_add(headroom) as
                size_t as size_t;
        return ret
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn new_opt6(mut opt: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = outpacket_counter as libc::c_int;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(4 as libc::c_int as size_t);
    if !p.is_null() {
        let mut t_s: u16_0 = opt as u16_0;
        let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh7 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh7 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_s_0: u16_0 = 0 as libc::c_int as u16_0;
        let mut t_cp_0: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh8 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh8 = (t_s_0 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_0 = t_s_0 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6(mut data: *mut libc::c_void,
                                  mut len: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(len);
    if !p.is_null() && !data.is_null() { memcpy(p, data, len); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_long(mut val: libc::c_uint) {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(4 as libc::c_int as size_t);
    if !p.is_null() {
        let mut t_l: u32_0 = val;
        let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh9 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh9 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
        let fresh10 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh10 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
        let fresh11 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh11 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_l as libc::c_uchar;
        p = p.offset(4 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_short(mut val: libc::c_uint) {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(2 as libc::c_int as size_t);
    if !p.is_null() {
        let mut t_s: u16_0 = val as u16_0;
        let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh12 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh12 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_char(mut val: libc::c_uint) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = expand(1 as libc::c_int as size_t) as *mut libc::c_uchar;
    if !p.is_null() { *p = val as libc::c_uchar };
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_string(mut s: *mut libc::c_char) {
    put_opt6(s as *mut libc::c_void, strlen(s));
}
