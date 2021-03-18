#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                 __buf: *mut libc::c_char, __len: socklen_t)
     -> *const libc::c_char;
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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __uflow(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
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
    fn a_record_from_hosts(name: *mut libc::c_char, now: time_t) -> in_addr;
    #[no_mangle]
    fn get_domain(addr: in_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn rand16() -> libc::c_ushort;
    #[no_mangle]
    fn legal_hostname(name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn do_rfc1035_name(p: *mut libc::c_uchar, sval: *mut libc::c_char,
                       limit: *mut libc::c_char) -> *mut libc::c_uchar;
    #[no_mangle]
    fn safe_strncpy(dest: *mut libc::c_char, src: *const libc::c_char,
                    size: size_t);
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> libc::c_int;
    #[no_mangle]
    fn prettyprint_time(buf: *mut libc::c_char, t: libc::c_uint);
    #[no_mangle]
    fn memcmp_masked(a: *mut libc::c_uchar, b: *mut libc::c_uchar,
                     len: libc::c_int, mask: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn expand_buf(iov: *mut iovec, size: size_t) -> libc::c_int;
    #[no_mangle]
    fn print_mac(buff: *mut libc::c_char, mac: *mut libc::c_uchar,
                 len: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn option_string(prot: libc::c_int, opt: libc::c_uint,
                     val: *mut libc::c_uchar, opt_len: libc::c_int,
                     buf: *mut libc::c_char, buf_len: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn enumerate_interfaces(reset: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn address_available(context: *mut dhcp_context, taddr: in_addr,
                         netids: *mut dhcp_netid) -> *mut dhcp_context;
    #[no_mangle]
    fn narrow_context(context: *mut dhcp_context, taddr: in_addr,
                      netids: *mut dhcp_netid) -> *mut dhcp_context;
    #[no_mangle]
    fn do_icmp_ping(now: time_t, addr: in_addr, hash: libc::c_uint,
                    loopback: libc::c_int) -> *mut ping_result;
    #[no_mangle]
    fn address_allocate(context: *mut dhcp_context, addrp: *mut in_addr,
                        hwaddr: *mut libc::c_uchar, hw_len: libc::c_int,
                        netids: *mut dhcp_netid, now: time_t,
                        loopback: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn config_find_by_address(configs: *mut dhcp_config, addr: in_addr)
     -> *mut dhcp_config;
    #[no_mangle]
    fn host_from_dns(addr: in_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn lease4_allocate(addr: in_addr) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease_set_hwaddr(lease: *mut dhcp_lease, hwaddr: *const libc::c_uchar,
                        clid: *const libc::c_uchar, hw_len: libc::c_int,
                        hw_type: libc::c_int, clid_len: libc::c_int,
                        now: time_t, force: libc::c_int);
    #[no_mangle]
    fn lease_set_hostname(lease: *mut dhcp_lease, name: *const libc::c_char,
                          auth: libc::c_int, domain: *mut libc::c_char,
                          config_domain: *mut libc::c_char);
    #[no_mangle]
    fn lease_set_expires(lease: *mut dhcp_lease, len: libc::c_uint,
                         now: time_t);
    #[no_mangle]
    fn lease_set_interface(lease: *mut dhcp_lease, interface: libc::c_int,
                           now: time_t);
    #[no_mangle]
    fn lease_find_by_client(hwaddr: *mut libc::c_uchar, hw_len: libc::c_int,
                            hw_type: libc::c_int, clid: *mut libc::c_uchar,
                            clid_len: libc::c_int) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease_find_by_addr(addr: in_addr) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease_prune(target: *mut dhcp_lease, now: time_t);
    #[no_mangle]
    fn lease_add_extradata(lease: *mut dhcp_lease, data: *mut libc::c_uchar,
                           len: libc::c_uint, delim: libc::c_int);
    #[no_mangle]
    fn match_netid(check: *mut dhcp_netid, pool: *mut dhcp_netid,
                   tagnotneeded: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn option_filter(tags: *mut dhcp_netid, context_tags: *mut dhcp_netid,
                     opts: *mut dhcp_opt) -> *mut dhcp_netid;
    #[no_mangle]
    fn log_tags(netid: *mut dhcp_netid, xid: u32_0);
    #[no_mangle]
    fn run_tag_if(tags: *mut dhcp_netid) -> *mut dhcp_netid;
    #[no_mangle]
    fn config_has_mac(config: *mut dhcp_config, hwaddr: *mut libc::c_uchar,
                      len: libc::c_int, type_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn delay_dhcp(start: time_t, sec: libc::c_int, fd: libc::c_int,
                  addr: uint32_t, id: libc::c_ushort) -> libc::c_int;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn find_config(configs: *mut dhcp_config, context: *mut dhcp_context,
                   clid: *mut libc::c_uchar, clid_len: libc::c_int,
                   hwaddr: *mut libc::c_uchar, hw_len: libc::c_int,
                   hw_type: libc::c_int, hostname: *mut libc::c_char,
                   filter: *mut dhcp_netid) -> *mut dhcp_config;
    #[no_mangle]
    fn strip_hostname(hostname: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn match_bytes(o: *mut dhcp_opt, p: *mut libc::c_uchar, len: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type ssize_t = __ssize_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const __METRIC_MAX: C2RustUnnamed_0 = 20;
pub const METRIC_LEASES_PRUNED_6: C2RustUnnamed_0 = 19;
pub const METRIC_LEASES_ALLOCATED_6: C2RustUnnamed_0 = 18;
pub const METRIC_LEASES_PRUNED_4: C2RustUnnamed_0 = 17;
pub const METRIC_LEASES_ALLOCATED_4: C2RustUnnamed_0 = 16;
pub const METRIC_NOANSWER: C2RustUnnamed_0 = 15;
pub const METRIC_DHCPREQUEST: C2RustUnnamed_0 = 14;
pub const METRIC_DHCPRELEASE: C2RustUnnamed_0 = 13;
pub const METRIC_DHCPOFFER: C2RustUnnamed_0 = 12;
pub const METRIC_DHCPNAK: C2RustUnnamed_0 = 11;
pub const METRIC_DHCPINFORM: C2RustUnnamed_0 = 10;
pub const METRIC_DHCPDISCOVER: C2RustUnnamed_0 = 9;
pub const METRIC_DHCPDECLINE: C2RustUnnamed_0 = 8;
pub const METRIC_DHCPACK: C2RustUnnamed_0 = 7;
pub const METRIC_PXE: C2RustUnnamed_0 = 6;
pub const METRIC_BOOTP: C2RustUnnamed_0 = 5;
pub const METRIC_DNS_LOCAL_ANSWERED: C2RustUnnamed_0 = 4;
pub const METRIC_DNS_AUTH_ANSWERED: C2RustUnnamed_0 = 3;
pub const METRIC_DNS_QUERIES_FORWARDED: C2RustUnnamed_0 = 2;
pub const METRIC_DNS_CACHE_LIVE_FREED: C2RustUnnamed_0 = 1;
pub const METRIC_DNS_CACHE_INSERTED: C2RustUnnamed_0 = 0;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct C2RustUnnamed_2 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub target: C2RustUnnamed_7,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub name: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
pub struct slaac_address {
    pub addr: in6_addr,
    pub ping_time: time_t,
    pub backoff: libc::c_int,
    pub next: *mut slaac_address,
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
    pub u: C2RustUnnamed_9,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                             mut __n: *mut size_t, mut __stream: *mut FILE)
 -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_reply(mut context: *mut dhcp_context,
                                    mut iface_name: *mut libc::c_char,
                                    mut int_index: libc::c_int,
                                    mut sz: size_t, mut now: time_t,
                                    mut unicast_dest: libc::c_int,
                                    mut loopback: libc::c_int,
                                    mut is_inform: *mut libc::c_int,
                                    mut pxe: libc::c_int,
                                    mut fallback: in_addr,
                                    mut recvtime: time_t) -> size_t {
    let mut opt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut clid: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ltmp: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut vendor: *mut dhcp_vendor = 0 as *mut dhcp_vendor;
    let mut mac: *mut dhcp_mac = 0 as *mut dhcp_mac;
    let mut id_list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
    let mut clid_len: libc::c_int = 0 as libc::c_int;
    let mut ignore: libc::c_int = 0 as libc::c_int;
    let mut do_classes: libc::c_int = 0 as libc::c_int;
    let mut rapid_commit: libc::c_int = 0 as libc::c_int;
    let mut selecting: libc::c_int = 0 as libc::c_int;
    let mut pxearch: libc::c_int = -(1 as libc::c_int);
    let mut pxevendor: *const libc::c_char = 0 as *const libc::c_char;
    let mut mess: *mut dhcp_packet =
        (*dnsmasq_daemon).dhcp_packet.iov_base as *mut dhcp_packet;
    let mut end: *mut libc::c_uchar =
        mess.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut real_end: *mut libc::c_uchar =
        mess.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offer_hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostname_auth: libc::c_int = 0 as libc::c_int;
    let mut borken_opt: libc::c_int = 0 as libc::c_int;
    let mut req_options: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut time: libc::c_uint = 0;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut netid: *mut dhcp_netid = 0 as *mut dhcp_netid;
    let mut tagif_netid: *mut dhcp_netid = 0 as *mut dhcp_netid;
    let mut subnet_addr: in_addr = in_addr{s_addr: 0,};
    let mut override_0: in_addr = in_addr{s_addr: 0,};
    let mut fuzz: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut mess_type: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut fqdn_flags: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut agent_id: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut uuid: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut emac: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vendor_class_len: libc::c_int = 0 as libc::c_int;
    let mut emac_len: libc::c_int = 0 as libc::c_int;
    let mut known_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut iface_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut cpewan_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut o: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut pxe_uuid: [libc::c_uchar; 17] = [0; 17];
    let mut oui: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut serial: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut class: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    override_0.s_addr = 0 as libc::c_int as in_addr_t;
    subnet_addr.s_addr = override_0.s_addr;
    /* set tag with name == interface */
    iface_id.net = iface_name;
    iface_id.next = 0 as *mut dhcp_netid;
    netid = &mut iface_id;
    if (*mess).op as libc::c_int != 1 as libc::c_int ||
           (*mess).hlen as libc::c_int > 16 as libc::c_int {
        return 0 as libc::c_int as size_t
    }
    if (*mess).htype as libc::c_int == 0 as libc::c_int &&
           (*mess).hlen as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int as size_t
    }
    /* check for DHCP rather than BOOTP */
    opt = option_find(mess, sz, 53 as libc::c_int, 1 as libc::c_int);
    if !opt.is_null() {
        let mut cookie: u32_0 =
            __bswap_32(0x63825363 as libc::c_int as __uint32_t);
        /* only insist on a cookie for DHCP. */
        if memcmp((*mess).options.as_mut_ptr() as *const libc::c_void,
                  &mut cookie as *mut u32_0 as *const libc::c_void,
                  ::std::mem::size_of::<u32_0>() as libc::c_ulong) !=
               0 as libc::c_int {
            return 0 as libc::c_int as size_t
        }
        mess_type = option_uint(opt, 0 as libc::c_int, 1 as libc::c_int);
        /* two things to note here: expand_buf may move the packet,
	 so reassign mess from daemon->packet. Also, the size
	 sent includes the IP and UDP headers, hence the magic "-28" */
        opt = option_find(mess, sz, 57 as libc::c_int, 2 as libc::c_int);
        if !opt.is_null() {
            let mut size: size_t =
                (option_uint(opt, 0 as libc::c_int, 2 as libc::c_int) as
                     size_t).wrapping_sub(28 as libc::c_int as libc::c_ulong);
            if size > 16384 as libc::c_int as libc::c_ulong {
                size = 16384 as libc::c_int as size_t
            } else if size <
                          ::std::mem::size_of::<dhcp_packet>() as
                              libc::c_ulong {
                size = ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong
            }
            if expand_buf(&mut (*dnsmasq_daemon).dhcp_packet, size) != 0 {
                mess =
                    (*dnsmasq_daemon).dhcp_packet.iov_base as
                        *mut dhcp_packet;
                end = (mess as *mut libc::c_uchar).offset(size as isize);
                real_end = end
            }
        }
        /* Some buggy clients set ciaddr when they shouldn't, so clear that here since
	 it can affect the context-determination code. */
        if !option_find(mess, sz, 50 as libc::c_int,
                        4 as libc::c_int).is_null() ||
               mess_type == 1 as libc::c_int as libc::c_uint {
            (*mess).ciaddr.s_addr = 0 as libc::c_int as in_addr_t
        }
        /* search for device identity from CPEWAN devices, we pass this through to the script */
        opt = option_find(mess, sz, 125 as libc::c_int, 5 as libc::c_int);
        if !opt.is_null() {
            let mut elen: libc::c_uint = 0;
            let mut offset: libc::c_uint = 0;
            let mut len: libc::c_uint =
                *opt.offset(1 as libc::c_int as isize) as libc::c_int as
                    libc::c_uint;
            offset = 0 as libc::c_int as libc::c_uint;
            while offset < len.wrapping_sub(5 as libc::c_int as libc::c_uint)
                  {
                elen =
                    option_uint(opt,
                                offset.wrapping_add(4 as libc::c_int as
                                                        libc::c_uint) as
                                    libc::c_int, 1 as libc::c_int);
                if option_uint(opt, offset as libc::c_int, 4 as libc::c_int)
                       == 3561 as libc::c_int as libc::c_uint &&
                       offset.wrapping_add(elen).wrapping_add(5 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                           <= len {
                    let mut x: *mut libc::c_uchar =
                        &mut *opt.offset((2 as
                                              libc::c_uint).wrapping_add(offset.wrapping_add(5
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint))
                                             as isize) as *mut libc::c_uchar
                            as *mut libc::c_void as *mut libc::c_uchar;
                    let mut y: *mut libc::c_uchar =
                        &mut *opt.offset((2 as
                                              libc::c_uint).wrapping_add(offset.wrapping_add(elen).wrapping_add(5
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint))
                                             as isize) as *mut libc::c_uchar
                            as *mut libc::c_void as *mut libc::c_uchar;
                    oui =
                        option_find1(x, y, 1 as libc::c_int,
                                     1 as libc::c_int);
                    serial =
                        option_find1(x, y, 2 as libc::c_int,
                                     1 as libc::c_int);
                    class =
                        option_find1(x, y, 3 as libc::c_int,
                                     1 as libc::c_int);
                    /* If TR069-id is present set the tag "cpewan-id" to facilitate echoing 
		     the gateway id back. Note that the device class is optional */
                    if !oui.is_null() && !serial.is_null() {
                        cpewan_id.net =
                            b"cpewan-id\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char;
                        cpewan_id.next = netid;
                        netid = &mut cpewan_id
                    }
                    break ;
                } else {
                    offset =
                        offset.wrapping_add(elen.wrapping_add(5 as libc::c_int
                                                                  as
                                                                  libc::c_uint))
                }
            }
        }
        opt = option_find(mess, sz, 82 as libc::c_int, 1 as libc::c_int);
        if !opt.is_null() {
            /* Any agent-id needs to be copied back out, verbatim, as the last option
	     in the packet. Here, we shift it to the very end of the buffer, if it doesn't
	     get overwritten, then it will be shuffled back at the end of processing.
	     Note that the incoming options must not be overwritten here, so there has to 
	     be enough free space at the end of the packet to copy the option. */
            let mut sopt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut total: libc::c_uint =
                (*opt.offset(1 as libc::c_int as isize) as libc::c_int +
                     2 as libc::c_int) as libc::c_uint;
            let mut last_opt: *mut libc::c_uchar =
                option_find1((&mut *(*mess).options.as_mut_ptr().offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                  as
                                  *mut u8_0).offset(::std::mem::size_of::<u32_0>()
                                                        as libc::c_ulong as
                                                        isize),
                             (mess as *mut libc::c_uchar).offset(sz as isize),
                             255 as libc::c_int, 0 as libc::c_int);
            if !last_opt.is_null() && last_opt < end.offset(-(total as isize))
               {
                end = end.offset(-(total as isize));
                agent_id = end;
                memcpy(agent_id as *mut libc::c_void,
                       opt as *const libc::c_void, total as libc::c_ulong);
            }
            /* look for RFC3527 Link selection sub-option */
            sopt =
                option_find1(&mut *opt.offset((2 as
                                                   libc::c_uint).wrapping_add(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                  as isize) as
                                 *mut libc::c_uchar as *mut libc::c_void as
                                 *mut libc::c_uchar,
                             &mut *opt.offset((2 as
                                                   libc::c_uint).wrapping_add(*opt.offset(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                  as isize) as
                                 *mut libc::c_uchar as *mut libc::c_void as
                                 *mut libc::c_uchar, 5 as libc::c_int,
                             4 as libc::c_int);
            if !sopt.is_null() { subnet_addr = option_addr(sopt) }
            /* look for RFC5107 server-identifier-override */
            sopt =
                option_find1(&mut *opt.offset((2 as
                                                   libc::c_uint).wrapping_add(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                  as isize) as
                                 *mut libc::c_uchar as *mut libc::c_void as
                                 *mut libc::c_uchar,
                             &mut *opt.offset((2 as
                                                   libc::c_uint).wrapping_add(*opt.offset(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                  as isize) as
                                 *mut libc::c_uchar as *mut libc::c_void as
                                 *mut libc::c_uchar, 11 as libc::c_int,
                             4 as libc::c_int);
            if !sopt.is_null() { override_0 = option_addr(sopt) }
            let mut current_block_52: u64;
            /* if a circuit-id or remote-is option is provided, exact-match to options. */
            vendor = (*dnsmasq_daemon).dhcp_vendors;
            while !vendor.is_null() {
                let mut search: libc::c_int = 0;
                if (*vendor).match_type == 3 as libc::c_int {
                    search = 1 as libc::c_int;
                    current_block_52 = 7385833325316299293;
                } else if (*vendor).match_type == 4 as libc::c_int {
                    search = 2 as libc::c_int;
                    current_block_52 = 7385833325316299293;
                } else if (*vendor).match_type == 5 as libc::c_int {
                    search = 6 as libc::c_int;
                    current_block_52 = 7385833325316299293;
                } else { current_block_52 = 15970011996474399071; }
                match current_block_52 {
                    7385833325316299293 => {
                        sopt =
                            option_find1(&mut *opt.offset((2 as
                                                               libc::c_uint).wrapping_add(0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)
                                                              as isize) as
                                             *mut libc::c_uchar as
                                             *mut libc::c_void as
                                             *mut libc::c_uchar,
                                         &mut *opt.offset((2 as
                                                               libc::c_uint).wrapping_add(*opt.offset(1
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize)
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)
                                                              as isize) as
                                             *mut libc::c_uchar as
                                             *mut libc::c_void as
                                             *mut libc::c_uchar, search,
                                         1 as libc::c_int);
                        if !sopt.is_null() &&
                               (*vendor).len ==
                                   *sopt.offset(1 as libc::c_int as isize) as
                                       libc::c_int &&
                               memcmp(&mut *sopt.offset((2 as
                                                             libc::c_uint).wrapping_add(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint)
                                                            as isize) as
                                          *mut libc::c_uchar as
                                          *mut libc::c_void,
                                      (*vendor).data as *const libc::c_void,
                                      (*vendor).len as libc::c_ulong) ==
                                   0 as libc::c_int {
                            (*vendor).netid.next = netid;
                            netid = &mut (*vendor).netid
                        }
                    }
                    _ => { }
                }
                vendor = (*vendor).next
            }
        }
        /* Check for RFC3011 subnet selector - only if RFC3527 one not present */
        if subnet_addr.s_addr == 0 as libc::c_int as libc::c_uint &&
               {
                   opt =
                       option_find(mess, sz, 118 as libc::c_int,
                                   4 as libc::c_int);
                   !opt.is_null()
               } {
            subnet_addr = option_addr(opt)
        }
        /* If there is no client identifier option, use the hardware address */
        if (*dnsmasq_daemon).options[(59 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (59 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 &&
               {
                   opt =
                       option_find(mess, sz, 61 as libc::c_int,
                                   1 as libc::c_int);
                   !opt.is_null()
               } {
            clid_len = *opt.offset(1 as libc::c_int as isize) as libc::c_int;
            clid =
                &mut *opt.offset((2 as
                                      libc::c_uint).wrapping_add(0 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                     as isize) as *mut libc::c_uchar as
                    *mut libc::c_void as *mut libc::c_uchar
        }
        /* do we have a lease in store? */
        lease =
            lease_find_by_client((*mess).chaddr.as_mut_ptr(),
                                 (*mess).hlen as libc::c_int,
                                 (*mess).htype as libc::c_int, clid,
                                 clid_len);
        /* If this request is missing a clid, but we've seen one before, 
	 use it again for option matching etc. */
        if !lease.is_null() && clid.is_null() && !(*lease).clid.is_null() {
            clid_len = (*lease).clid_len;
            clid = (*lease).clid
        }
        /* find mac to use for logging and hashing */
        emac =
            extended_hwaddr((*mess).htype as libc::c_int,
                            (*mess).hlen as libc::c_int,
                            (*mess).chaddr.as_mut_ptr(), clid_len, clid,
                            &mut emac_len)
    }
    mac = (*dnsmasq_daemon).dhcp_macs;
    while !mac.is_null() {
        if (*mac).hwaddr_len == (*mess).hlen as libc::c_int &&
               ((*mac).hwaddr_type == (*mess).htype as libc::c_int ||
                    (*mac).hwaddr_type == 0 as libc::c_int) &&
               memcmp_masked((*mac).hwaddr.as_mut_ptr(),
                             (*mess).chaddr.as_mut_ptr(),
                             (*mess).hlen as libc::c_int, (*mac).mask) != 0 {
            (*mac).netid.next = netid;
            netid = &mut (*mac).netid
        }
        mac = (*mac).next
    }
    /* Determine network for this packet. Our caller will have already linked all the 
     contexts which match the addresses of the receiving interface but if the 
     machine has an address already, or came via a relay, or we have a subnet selector, 
     we search again. If we don't have have a giaddr or explicit subnet selector, 
     use the ciaddr. This is necessary because a  machine which got a lease via a 
     relay won't use the relay to renew. If matching a ciaddr fails but we have a context 
     from the physical network, continue using that to allow correct DHCPNAK generation later. */
    if (*mess).giaddr.s_addr != 0 || subnet_addr.s_addr != 0 ||
           (*mess).ciaddr.s_addr != 0 {
        let mut context_tmp: *mut dhcp_context = 0 as *mut dhcp_context;
        let mut context_new: *mut dhcp_context = 0 as *mut dhcp_context;
        let mut share: *mut shared_network = 0 as *mut shared_network;
        let mut addr: in_addr = in_addr{s_addr: 0,};
        let mut force: libc::c_int = 0 as libc::c_int;
        let mut via_relay: libc::c_int = 0 as libc::c_int;
        if subnet_addr.s_addr != 0 {
            addr = subnet_addr;
            force = 1 as libc::c_int
        } else if (*mess).giaddr.s_addr != 0 {
            addr = (*mess).giaddr;
            force = 1 as libc::c_int;
            via_relay = 1 as libc::c_int
        } else {
            /* If ciaddr is in the hardware derived set of contexts, leave that unchanged */
            addr = (*mess).ciaddr;
            context_tmp = context;
            while !context_tmp.is_null() {
                if (*context_tmp).netmask.s_addr != 0 &&
                       is_same_net(addr, (*context_tmp).start,
                                   (*context_tmp).netmask) != 0 &&
                       is_same_net(addr, (*context_tmp).end,
                                   (*context_tmp).netmask) != 0 {
                    context_new = context;
                    break ;
                } else { context_tmp = (*context_tmp).current }
            }
        }
        if context_new.is_null() {
            context_tmp = (*dnsmasq_daemon).dhcp;
            while !context_tmp.is_null() {
                let mut netmask: in_addr = (*context_tmp).netmask;
                /* guess the netmask for relayed networks */
                if (*context_tmp).flags as libc::c_uint &
                       (1 as libc::c_uint) << 1 as libc::c_int == 0 &&
                       (*context_tmp).netmask.s_addr ==
                           0 as libc::c_int as libc::c_uint {
                    if __bswap_32((*context_tmp).start.s_addr) &
                           0x80000000 as libc::c_uint ==
                           0 as libc::c_int as libc::c_uint &&
                           __bswap_32((*context_tmp).end.s_addr) &
                               0x80000000 as libc::c_uint ==
                               0 as libc::c_int as libc::c_uint {
                        netmask.s_addr =
                            __bswap_32(0xff000000 as libc::c_uint)
                    } else if __bswap_32((*context_tmp).start.s_addr) &
                                  0xc0000000 as libc::c_uint ==
                                  0x80000000 as libc::c_uint &&
                                  __bswap_32((*context_tmp).end.s_addr) &
                                      0xc0000000 as libc::c_uint ==
                                      0x80000000 as libc::c_uint {
                        netmask.s_addr =
                            __bswap_32(0xffff0000 as libc::c_uint)
                    } else if __bswap_32((*context_tmp).start.s_addr) &
                                  0xe0000000 as libc::c_uint ==
                                  0xc0000000 as libc::c_uint &&
                                  __bswap_32((*context_tmp).end.s_addr) &
                                      0xe0000000 as libc::c_uint ==
                                      0xc0000000 as libc::c_uint {
                        netmask.s_addr =
                            __bswap_32(0xffffff00 as libc::c_uint)
                    }
                }
                /* check to see is a context is OK because of a shared address on
		 the relayed subnet. */
                if via_relay != 0 {
                    share = (*dnsmasq_daemon).shared_networks;
                    while !share.is_null() {
                        if !((*share).shared_addr.s_addr ==
                                 0 as libc::c_int as libc::c_uint) {
                            if !((*share).if_index != 0 as libc::c_int ||
                                     (*share).match_addr.s_addr !=
                                         (*mess).giaddr.s_addr) {
                                if netmask.s_addr !=
                                       0 as libc::c_int as libc::c_uint &&
                                       is_same_net((*share).shared_addr,
                                                   (*context_tmp).start,
                                                   netmask) != 0 &&
                                       is_same_net((*share).shared_addr,
                                                   (*context_tmp).end,
                                                   netmask) != 0 {
                                    break ;
                                }
                            }
                        }
                        share = (*share).next
                    }
                }
                /* This section fills in context mainly when a client which is on a remote (relayed)
		 network renews a lease without using the relay, after dnsmasq has restarted. */
                if !share.is_null() ||
                       netmask.s_addr != 0 as libc::c_int as libc::c_uint &&
                           is_same_net(addr, (*context_tmp).start, netmask) !=
                               0 &&
                           is_same_net(addr, (*context_tmp).end, netmask) != 0
                   {
                    (*context_tmp).netmask = netmask;
                    if (*context_tmp).local.s_addr ==
                           0 as libc::c_int as libc::c_uint {
                        (*context_tmp).local = fallback
                    }
                    if (*context_tmp).router.s_addr ==
                           0 as libc::c_int as libc::c_uint && share.is_null()
                       {
                        (*context_tmp).router = (*mess).giaddr
                    }
                    /* fill in missing broadcast addresses for relayed ranges */
                    if (*context_tmp).flags as libc::c_uint &
                           (1 as libc::c_uint) << 2 as libc::c_int == 0 &&
                           (*context_tmp).broadcast.s_addr ==
                               0 as libc::c_int as libc::c_uint {
                        (*context_tmp).broadcast.s_addr =
                            (*context_tmp).start.s_addr |
                                !(*context_tmp).netmask.s_addr
                    }
                    (*context_tmp).current = context_new;
                    context_new = context_tmp
                }
                context_tmp = (*context_tmp).next
            }
        }
        if !context_new.is_null() || force != 0 { context = context_new }
    }
    if context.is_null() {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
                  b"no address range available for DHCP request %s %s\x00" as
                      *const u8 as *const libc::c_char,
                  if subnet_addr.s_addr != 0 {
                      b"with subnet selector\x00" as *const u8 as
                          *const libc::c_char
                  } else { b"via\x00" as *const u8 as *const libc::c_char },
                  if subnet_addr.s_addr != 0 {
                      inet_ntoa(subnet_addr)
                  } else if (*mess).giaddr.s_addr != 0 {
                      inet_ntoa((*mess).giaddr)
                  } else { iface_name });
        return 0 as libc::c_int as size_t
    }
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
        let mut context_tmp_0: *mut dhcp_context = 0 as *mut dhcp_context;
        context_tmp_0 = context;
        while !context_tmp_0.is_null() {
            strcpy((*dnsmasq_daemon).namebuff,
                   inet_ntoa((*context_tmp_0).start));
            if (*context_tmp_0).flags as libc::c_uint &
                   ((1 as libc::c_uint) << 0 as libc::c_int |
                        (1 as libc::c_uint) << 3 as libc::c_int) != 0 {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"%u available DHCP subnet: %s/%s\x00" as *const u8
                              as *const libc::c_char, __bswap_32((*mess).xid),
                          (*dnsmasq_daemon).namebuff,
                          inet_ntoa((*context_tmp_0).netmask));
            } else {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"%u available DHCP range: %s -- %s\x00" as
                              *const u8 as *const libc::c_char,
                          __bswap_32((*mess).xid), (*dnsmasq_daemon).namebuff,
                          inet_ntoa((*context_tmp_0).end));
            }
            context_tmp_0 = (*context_tmp_0).current
        }
    }
    let mut current_block_146: u64;
    /* dhcp-match. If we have hex-and-wildcards, look for a left-anchored match.
     Otherwise assume the option is an array, and look for a matching element. 
     If no data given, existence of the option is enough. This code handles 
     rfc3925 V-I classes too. */
    o = (*dnsmasq_daemon).dhcp_match;
    while !o.is_null() {
        let mut len_0: libc::c_uint = 0;
        let mut elen_0: libc::c_uint = 0;
        let mut match_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut offset_0: size_t = 0;
        let mut o2: size_t = 0;
        if (*o).flags & 2048 as libc::c_int != 0 {
            opt = option_find(mess, sz, 124 as libc::c_int, 5 as libc::c_int);
            if opt.is_null() {
                current_block_146 = 18316056106135622027;
            } else {
                offset_0 = 0 as libc::c_int as size_t;
                while offset_0 <
                          (*opt.offset(1 as libc::c_int as isize) as
                               libc::c_int as
                               libc::c_uint).wrapping_sub(5 as libc::c_uint)
                              as libc::c_ulong {
                    len_0 =
                        option_uint(opt,
                                    offset_0.wrapping_add(4 as libc::c_int as
                                                              libc::c_ulong)
                                        as libc::c_int, 1 as libc::c_int);
                    /* Need to take care that bad data can't run us off the end of the packet */
                    if offset_0.wrapping_add(len_0 as
                                                 libc::c_ulong).wrapping_add(5
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                           <=
                           *opt.offset(1 as libc::c_int as isize) as
                               libc::c_int as libc::c_uint as libc::c_ulong &&
                           option_uint(opt, offset_0 as libc::c_int,
                                       4 as libc::c_int) ==
                               (*o).u.encap as libc::c_uint {
                        o2 =
                            offset_0.wrapping_add(5 as libc::c_int as
                                                      libc::c_ulong);
                        while o2 <
                                  offset_0.wrapping_add(len_0 as
                                                            libc::c_ulong).wrapping_add(5
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong)
                              {
                            elen_0 =
                                option_uint(opt, o2 as libc::c_int,
                                            1 as libc::c_int);
                            if o2.wrapping_add(elen_0 as
                                                   libc::c_ulong).wrapping_add(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                   <=
                                   *opt.offset(1 as libc::c_int as isize) as
                                       libc::c_int as libc::c_uint as
                                       libc::c_ulong &&
                                   {
                                       match_0 =
                                           match_bytes(o,
                                                       &mut *opt.offset((2 as
                                                                             libc::c_uint).wrapping_add(o2.wrapping_add(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong)
                                                                                                            as
                                                                                                            libc::c_uint)
                                                                            as
                                                                            isize)
                                                           as
                                                           *mut libc::c_uchar
                                                           as
                                                           *mut libc::c_void
                                                           as
                                                           *mut libc::c_uchar,
                                                       elen_0 as libc::c_int)
                                               as libc::c_uint;
                                       (match_0) != 0
                                   } {
                                break ;
                            }
                            o2 =
                                (o2 as
                                     libc::c_ulong).wrapping_add(elen_0.wrapping_add(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint)
                                                                     as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                    }
                    if match_0 != 0 { break ; }
                    offset_0 =
                        (offset_0 as
                             libc::c_ulong).wrapping_add(len_0.wrapping_add(5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                             as libc::c_ulong)
                            as size_t as size_t
                }
                current_block_146 = 10720305954121010852;
            }
        } else {
            opt = option_find(mess, sz, (*o).opt, 1 as libc::c_int);
            if opt.is_null() {
                current_block_146 = 18316056106135622027;
            } else {
                match_0 =
                    match_bytes(o,
                                &mut *opt.offset((2 as
                                                      libc::c_uint).wrapping_add(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                                     as isize) as
                                    *mut libc::c_uchar as *mut libc::c_void as
                                    *mut libc::c_uchar,
                                *opt.offset(1 as libc::c_int as isize) as
                                    libc::c_int) as libc::c_uint;
                current_block_146 = 10720305954121010852;
            }
        }
        match current_block_146 {
            10720305954121010852 => {
                if match_0 != 0 {
                    (*(*o).netid).next = netid;
                    netid = (*o).netid
                }
            }
            _ => { }
        }
        o = (*o).next
    }
    /* user-class options are, according to RFC3004, supposed to contain
     a set of counted strings. Here we check that this is so (by seeing
     if the counts are consistent with the overall option length) and if
     so zero the counts so that we don't get spurious matches between 
     the vendor string and the counts. If the lengths don't add up, we
     assume that the option is a single string and non RFC3004 compliant 
     and just do the substring match. dhclient provides these broken options.
     The code, later, which sends user-class data to the lease-change script
     relies on the transformation done here.
  */
    opt = option_find(mess, sz, 77 as libc::c_int, 1 as libc::c_int);
    if !opt.is_null() {
        let mut ucp: *mut libc::c_uchar =
            &mut *opt.offset((2 as
                                  libc::c_uint).wrapping_add(0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                 as isize) as *mut libc::c_uchar as
                *mut libc::c_void as *mut libc::c_uchar;
        let mut tmp: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < *opt.offset(1 as libc::c_int as isize) as libc::c_int {
            j += *ucp.offset(j as isize) as libc::c_int + 1 as libc::c_int
        }
        if j == *opt.offset(1 as libc::c_int as isize) as libc::c_int {
            j = 0 as libc::c_int;
            while j < *opt.offset(1 as libc::c_int as isize) as libc::c_int {
                tmp =
                    j + *ucp.offset(j as isize) as libc::c_int +
                        1 as libc::c_int;
                *ucp.offset(j as isize) = 0 as libc::c_int as libc::c_uchar;
                j = tmp
            }
        }
    }
    let mut current_block_167: u64;
    vendor = (*dnsmasq_daemon).dhcp_vendors;
    while !vendor.is_null() {
        let mut mopt: libc::c_int = 0;
        if (*vendor).match_type == 1 as libc::c_int {
            mopt = 60 as libc::c_int;
            current_block_167 = 13526015532137226550;
        } else if (*vendor).match_type == 2 as libc::c_int {
            mopt = 77 as libc::c_int;
            current_block_167 = 13526015532137226550;
        } else { current_block_167 = 12227374774078719326; }
        match current_block_167 {
            13526015532137226550 => {
                opt = option_find(mess, sz, mopt, 1 as libc::c_int);
                if !opt.is_null() {
                    let mut i: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i <=
                              *opt.offset(1 as libc::c_int as isize) as
                                  libc::c_int - (*vendor).len {
                        if memcmp((*vendor).data as *const libc::c_void,
                                  &mut *opt.offset((2 as
                                                        libc::c_uint).wrapping_add(i
                                                                                       as
                                                                                       libc::c_uint)
                                                       as isize) as
                                      *mut libc::c_uchar as *mut libc::c_void,
                                  (*vendor).len as libc::c_ulong) ==
                               0 as libc::c_int {
                            (*vendor).netid.next = netid;
                            netid = &mut (*vendor).netid;
                            break ;
                        } else { i += 1 }
                    }
                }
            }
            _ => { }
        }
        vendor = (*vendor).next
    }
    /* mark vendor-encapsulated options which match the client-supplied vendor class,
     save client-supplied vendor class */
    opt = option_find(mess, sz, 60 as libc::c_int, 1 as libc::c_int);
    if !opt.is_null() {
        memcpy((*dnsmasq_daemon).dhcp_buff3 as *mut libc::c_void,
               &mut *opt.offset((2 as
                                     libc::c_uint).wrapping_add(0 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as isize) as *mut libc::c_uchar as
                   *mut libc::c_void,
               *opt.offset(1 as libc::c_int as isize) as libc::c_int as
                   libc::c_ulong);
        vendor_class_len =
            *opt.offset(1 as libc::c_int as isize) as libc::c_int
    }
    match_vendor_opts(opt, (*dnsmasq_daemon).dhcp_opts);
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
        if sanitise(opt, (*dnsmasq_daemon).namebuff) != 0 {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u vendor class: %s\x00" as *const u8 as
                          *const libc::c_char, __bswap_32((*mess).xid),
                      (*dnsmasq_daemon).namebuff);
        }
        if sanitise(option_find(mess, sz, 77 as libc::c_int,
                                1 as libc::c_int), (*dnsmasq_daemon).namebuff)
               != 0 {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u user class: %s\x00" as *const u8 as
                          *const libc::c_char, __bswap_32((*mess).xid),
                      (*dnsmasq_daemon).namebuff);
        }
    }
    (*mess).op = 2 as libc::c_int as u8_0;
    config =
        find_config((*dnsmasq_daemon).dhcp_conf, context, clid, clid_len,
                    (*mess).chaddr.as_mut_ptr(), (*mess).hlen as libc::c_int,
                    (*mess).htype as libc::c_int, 0 as *mut libc::c_char,
                    run_tag_if(netid));
    /* set "known" tag for known hosts */
    if !config.is_null() {
        known_id.net =
            b"known\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        known_id.next = netid;
        netid = &mut known_id
    } else if !find_config((*dnsmasq_daemon).dhcp_conf,
                           0 as *mut dhcp_context, clid, clid_len,
                           (*mess).chaddr.as_mut_ptr(),
                           (*mess).hlen as libc::c_int,
                           (*mess).htype as libc::c_int,
                           0 as *mut libc::c_char,
                           run_tag_if(netid)).is_null() {
        known_id.net =
            b"known-othernet\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        known_id.next = netid;
        netid = &mut known_id
    }
    if mess_type == 0 as libc::c_int as libc::c_uint && pxe == 0 {
        /* BOOTP request */
        let mut id: dhcp_netid =
            dhcp_netid{net: 0 as *mut libc::c_char,
                       next: 0 as *mut dhcp_netid,};
        let mut bootp_id: dhcp_netid =
            dhcp_netid{net: 0 as *mut libc::c_char,
                       next: 0 as *mut dhcp_netid,};
        let mut logaddr: *mut in_addr = 0 as *mut in_addr;
        /* must have a MAC addr for bootp */
        if (*mess).htype as libc::c_int == 0 as libc::c_int ||
               (*mess).hlen as libc::c_int == 0 as libc::c_int ||
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 3 as libc::c_int != 0 {
            return 0 as libc::c_int as size_t
        } /* BOOTP vend area is only 64 bytes */
        if !config.is_null() &&
               (*config).flags & 1 as libc::c_int as libc::c_uint != 0 {
            message =
                b"disabled\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        end = (*mess).options.as_mut_ptr().offset(64 as libc::c_int as isize);
        if !config.is_null() &&
               (*config).flags & 16 as libc::c_int as libc::c_uint != 0 {
            hostname = (*config).hostname;
            domain = (*config).domain
        }
        if !config.is_null() {
            let mut list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
            list = (*config).netid;
            while !list.is_null() {
                (*(*list).list).next = netid;
                netid = (*list).list;
                list = (*list).next
            }
        }
        /* Match incoming filename field as a netid. */
        if (*mess).file[0 as libc::c_int as usize] != 0 {
            memcpy((*dnsmasq_daemon).dhcp_buff2 as *mut libc::c_void,
                   (*mess).file.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<[u8_0; 128]>() as
                       libc::c_ulong); /* ensure zero term. */
            *(*dnsmasq_daemon).dhcp_buff2.offset((::std::mem::size_of::<[u8_0; 128]>()
                                                      as
                                                      libc::c_ulong).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                                                     as isize) =
                0 as libc::c_int as libc::c_char;
            id.net = (*dnsmasq_daemon).dhcp_buff2;
            id.next = netid;
            netid = &mut id
        }
        /* Add "bootp" as a tag to allow different options, address ranges etc
	 for BOOTP clients */
        bootp_id.net =
            b"bootp\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        bootp_id.next = netid;
        netid = &mut bootp_id;
        tagif_netid = run_tag_if(netid);
        id_list = (*dnsmasq_daemon).dhcp_ignore;
        while !id_list.is_null() {
            if match_netid((*id_list).list, tagif_netid, 0 as libc::c_int) !=
                   0 {
                message =
                    b"ignored\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            id_list = (*id_list).next
        }
        if message.is_null() {
            let mut nailed: libc::c_int = 0 as libc::c_int;
            if !config.is_null() &&
                   (*config).flags & 32 as libc::c_int as libc::c_uint != 0 {
                nailed = 1 as libc::c_int;
                logaddr = &mut (*config).addr;
                (*mess).yiaddr = (*config).addr;
                lease = lease_find_by_addr((*config).addr);
                if !lease.is_null() &&
                       ((*lease).hwaddr_len != (*mess).hlen as libc::c_int ||
                            (*lease).hwaddr_type !=
                                (*mess).htype as libc::c_int ||
                            memcmp((*lease).hwaddr.as_mut_ptr() as
                                       *const libc::c_void,
                                   (*mess).chaddr.as_mut_ptr() as
                                       *const libc::c_void,
                                   (*lease).hwaddr_len as libc::c_ulong) !=
                                0 as libc::c_int) {
                    message =
                        b"address in use\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char
                }
            } else {
                lease =
                    lease_find_by_client((*mess).chaddr.as_mut_ptr(),
                                         (*mess).hlen as libc::c_int,
                                         (*mess).htype as libc::c_int,
                                         0 as *mut libc::c_uchar,
                                         0 as libc::c_int);
                if lease.is_null() ||
                       address_available(context, (*lease).addr,
                                         tagif_netid).is_null() {
                    if !lease.is_null() {
                        /* lease exists, wrong network. */
                        lease_prune(lease, now);
                        lease = 0 as *mut dhcp_lease
                    }
                    if address_allocate(context, &mut (*mess).yiaddr,
                                        (*mess).chaddr.as_mut_ptr(),
                                        (*mess).hlen as libc::c_int,
                                        tagif_netid, now, loopback) == 0 {
                        message =
                            b"no address available\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    }
                } else { (*mess).yiaddr = (*lease).addr }
            }
            if message.is_null() &&
                   {
                       context =
                           narrow_context(context, (*mess).yiaddr, netid);
                       context.is_null()
                   } {
                message =
                    b"wrong network\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
            } else if !(*context).netid.net.is_null() {
                (*context).netid.next = netid;
                tagif_netid = run_tag_if(&mut (*context).netid)
            }
            log_tags(tagif_netid, __bswap_32((*mess).xid));
            if message.is_null() && nailed == 0 {
                id_list = (*dnsmasq_daemon).bootp_dynamic;
                while !id_list.is_null() {
                    if (*id_list).list.is_null() ||
                           match_netid((*id_list).list, tagif_netid,
                                       0 as libc::c_int) != 0 {
                        break ;
                    }
                    id_list = (*id_list).next
                }
                if id_list.is_null() {
                    message =
                        b"no address configured\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char
                }
            }
            if message.is_null() && lease.is_null() &&
                   {
                       lease = lease4_allocate((*mess).yiaddr);
                       lease.is_null()
                   } {
                message =
                    b"no leases left\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
            }
            if message.is_null() {
                logaddr = &mut (*mess).yiaddr;
                lease_set_hwaddr(lease, (*mess).chaddr.as_mut_ptr(),
                                 0 as *const libc::c_uchar,
                                 (*mess).hlen as libc::c_int,
                                 (*mess).htype as libc::c_int,
                                 0 as libc::c_int, now, 1 as libc::c_int);
                if !hostname.is_null() {
                    lease_set_hostname(lease, hostname, 1 as libc::c_int,
                                       get_domain((*lease).addr), domain);
                }
                /* infinite lease unless nailed in dhcp-host line. */
                lease_set_expires(lease,
                                  if !config.is_null() &&
                                         (*config).flags &
                                             8 as libc::c_int as libc::c_uint
                                             != 0 {
                                      (*config).lease_time
                                  } else { 0xffffffff as libc::c_uint }, now);
                lease_set_interface(lease, int_index, now);
                clear_packet(mess, end);
                do_options(context, mess, end, 0 as *mut libc::c_uchar,
                           hostname, get_domain((*mess).yiaddr), netid,
                           subnet_addr, 0 as libc::c_int as libc::c_uchar,
                           0 as libc::c_int, -(1 as libc::c_int),
                           0 as *mut libc::c_uchar, vendor_class_len, now,
                           0xffffffff as libc::c_uint,
                           0 as libc::c_int as libc::c_ushort,
                           0 as *const libc::c_char);
            }
        }
        (*dnsmasq_daemon).metrics[METRIC_BOOTP as libc::c_int as usize] =
            (*dnsmasq_daemon).metrics[METRIC_BOOTP as libc::c_int as
                                          usize].wrapping_add(1);
        log_packet(b"BOOTP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char, logaddr as *mut libc::c_void,
                   (*mess).chaddr.as_mut_ptr(), (*mess).hlen as libc::c_int,
                   iface_name, 0 as *mut libc::c_char, message, (*mess).xid);
        return if !message.is_null() {
                   0 as libc::c_int as libc::c_ulong
               } else { dhcp_packet_size(mess, agent_id, real_end) }
    }
    opt = option_find(mess, sz, 81 as libc::c_int, 3 as libc::c_int);
    if !opt.is_null() {
        /* http://tools.ietf.org/wg/dhc/draft-ietf-dhc-fqdn-option/draft-ietf-dhc-fqdn-option-10.txt */
        let mut len_1: libc::c_int =
            *opt.offset(1 as libc::c_int as isize) as libc::c_int;
        let mut pq: *mut libc::c_char = (*dnsmasq_daemon).dhcp_buff;
        let mut pp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut op: *mut libc::c_uchar =
            &mut *opt.offset((2 as
                                  libc::c_uint).wrapping_add(0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                 as isize) as *mut libc::c_uchar as
                *mut libc::c_void as *mut libc::c_uchar;
        fqdn_flags = *op;
        len_1 -= 3 as libc::c_int;
        op = op.offset(3 as libc::c_int as isize);
        pp = op;
        /* NB, the following always sets at least one bit */
        if (*dnsmasq_daemon).options[(36 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (36 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            if fqdn_flags as libc::c_int & 0x1 as libc::c_int != 0 {
                fqdn_flags =
                    (fqdn_flags as libc::c_int | 0x2 as libc::c_int) as
                        libc::c_uchar;
                fqdn_flags =
                    (fqdn_flags as libc::c_int & !(0x1 as libc::c_int)) as
                        libc::c_uchar /* set O */
                /* clear S */
            }
            fqdn_flags =
                (fqdn_flags as libc::c_int | 0x8 as libc::c_int) as
                    libc::c_uchar
            /* set N */
        } else {
            if fqdn_flags as libc::c_int & 0x1 as libc::c_int == 0 {
                fqdn_flags =
                    (fqdn_flags as libc::c_int | 0x3 as libc::c_int) as
                        libc::c_uchar
            }
            fqdn_flags =
                (fqdn_flags as libc::c_int & !(0x8 as libc::c_int)) as
                    libc::c_uchar /* set S and O */
            /* clear N */
        }
        if fqdn_flags as libc::c_int & 0x4 as libc::c_int != 0 {
            while *op as libc::c_int != 0 as libc::c_int &&
                      (op.offset(*op as libc::c_int as
                                     isize).wrapping_offset_from(pp) as
                           libc::c_long) < len_1 as libc::c_long {
                memcpy(pq as *mut libc::c_void,
                       op.offset(1 as libc::c_int as isize) as
                           *const libc::c_void, *op as libc::c_ulong);
                pq = pq.offset(*op as libc::c_int as isize);
                op =
                    op.offset((*op as libc::c_int + 1 as libc::c_int) as
                                  isize);
                let fresh6 = pq;
                pq = pq.offset(1);
                *fresh6 = '.' as i32 as libc::c_char
            }
        } else {
            memcpy(pq as *mut libc::c_void, op as *const libc::c_void,
                   len_1 as libc::c_ulong);
            if len_1 > 0 as libc::c_int &&
                   *op.offset((len_1 - 1 as libc::c_int) as isize) as
                       libc::c_int == 0 as libc::c_int {
                borken_opt = 1 as libc::c_int
            }
            pq = pq.offset((len_1 + 1 as libc::c_int) as isize)
        }
        if pq != (*dnsmasq_daemon).dhcp_buff { pq = pq.offset(-1) }
        *pq = 0 as libc::c_int as libc::c_char;
        if legal_hostname((*dnsmasq_daemon).dhcp_buff) != 0 {
            client_hostname = (*dnsmasq_daemon).dhcp_buff;
            offer_hostname = client_hostname
        }
    } else {
        opt = option_find(mess, sz, 12 as libc::c_int, 1 as libc::c_int);
        if !opt.is_null() {
            let mut len_2: libc::c_int =
                *opt.offset(1 as libc::c_int as isize) as libc::c_int;
            memcpy((*dnsmasq_daemon).dhcp_buff as *mut libc::c_void,
                   &mut *opt.offset((2 as
                                         libc::c_uint).wrapping_add(0 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                        as isize) as *mut libc::c_uchar as
                       *mut libc::c_void, len_2 as libc::c_ulong);
            /* Microsoft clients are broken, and need zero-terminated strings
	 in options. We detect this state here, and do the same in
	 any options we send */
            if len_2 > 0 as libc::c_int &&
                   *(*dnsmasq_daemon).dhcp_buff.offset((len_2 -
                                                            1 as libc::c_int)
                                                           as isize) as
                       libc::c_int == 0 as libc::c_int {
                borken_opt = 1 as libc::c_int
            } else {
                *(*dnsmasq_daemon).dhcp_buff.offset(len_2 as isize) =
                    0 as libc::c_int as libc::c_char
            }
            if legal_hostname((*dnsmasq_daemon).dhcp_buff) != 0 {
                client_hostname = (*dnsmasq_daemon).dhcp_buff
            }
        }
    }
    if !client_hostname.is_null() {
        let mut m: *mut dhcp_match_name = 0 as *mut dhcp_match_name;
        let mut nl: size_t = strlen(client_hostname);
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
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u client provides name: %s\x00" as *const u8 as
                          *const libc::c_char, __bswap_32((*mess).xid),
                      client_hostname);
        }
        m = (*dnsmasq_daemon).dhcp_name_match;
        while !m.is_null() {
            let mut ml: size_t = strlen((*m).name);
            let mut save: libc::c_char = 0 as libc::c_int as libc::c_char;
            if !(nl < ml) {
                if nl > ml {
                    save = *client_hostname.offset(ml as isize);
                    *client_hostname.offset(ml as isize) =
                        0 as libc::c_int as libc::c_char
                }
                if hostname_isequal(client_hostname, (*m).name) != 0 &&
                       (save as libc::c_int == 0 as libc::c_int ||
                            (*m).wildcard != 0) {
                    (*(*m).netid).next = netid;
                    netid = (*m).netid
                }
                if save as libc::c_int != 0 as libc::c_int {
                    *client_hostname.offset(ml as isize) = save
                }
            }
            m = (*m).next
        }
    }
    if !config.is_null() &&
           (*config).flags & 16 as libc::c_int as libc::c_uint != 0 {
        hostname = (*config).hostname;
        domain = (*config).domain;
        hostname_auth = 1 as libc::c_int;
        /* be careful not to send an OFFER with a hostname not matching the DISCOVER. */
        if fqdn_flags as libc::c_int != 0 as libc::c_int ||
               client_hostname.is_null() ||
               hostname_isequal(hostname, client_hostname) != 0 {
            offer_hostname = hostname
        }
    } else if !client_hostname.is_null() {
        domain = strip_hostname(client_hostname);
        if strlen(client_hostname) != 0 as libc::c_int as libc::c_ulong {
            hostname = client_hostname;
            if config.is_null() {
                /* Search again now we have a hostname. 
		 Only accept configs without CLID and HWADDR here, (they won't match)
		 to avoid impersonation by name. */
                let mut new: *mut dhcp_config =
                    find_config((*dnsmasq_daemon).dhcp_conf, context,
                                0 as *mut libc::c_uchar, 0 as libc::c_int,
                                (*mess).chaddr.as_mut_ptr(),
                                (*mess).hlen as libc::c_int,
                                (*mess).htype as libc::c_int, hostname,
                                run_tag_if(netid));
                if !new.is_null() &&
                       !(!new.is_null() &&
                             (*new).flags & 2 as libc::c_int as libc::c_uint
                                 != 0) && (*new).hwaddr.is_null() {
                    config = new;
                    /* set "known" tag for known hosts */
                    known_id.net =
                        b"known\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char;
                    known_id.next = netid;
                    netid = &mut known_id
                }
            }
        }
    }
    if !config.is_null() {
        let mut list_0: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
        list_0 = (*config).netid;
        while !list_0.is_null() {
            (*(*list_0).list).next = netid;
            netid = (*list_0).list;
            list_0 = (*list_0).next
        }
    }
    tagif_netid = run_tag_if(netid);
    /* if all the netids in the ignore list are present, ignore this client */
    id_list = (*dnsmasq_daemon).dhcp_ignore;
    while !id_list.is_null() {
        if match_netid((*id_list).list, tagif_netid, 0 as libc::c_int) != 0 {
            ignore = 1 as libc::c_int
        }
        id_list = (*id_list).next
    }
    /* If configured, we can override the server-id to be the address of the relay, 
     so that all traffic goes via the relay and can pick up agent-id info. This can be
     configured for all relays, or by address. */
    if (*dnsmasq_daemon).override_0 != 0 &&
           (*mess).giaddr.s_addr != 0 as libc::c_int as libc::c_uint &&
           override_0.s_addr == 0 as libc::c_int as libc::c_uint {
        if (*dnsmasq_daemon).override_relays.is_null() {
            override_0 = (*mess).giaddr
        } else {
            let mut l: *mut addr_list = 0 as *mut addr_list;
            l = (*dnsmasq_daemon).override_relays;
            while !l.is_null() {
                if (*l).addr.s_addr == (*mess).giaddr.s_addr { break ; }
                l = (*l).next
            }
            if !l.is_null() { override_0 = (*mess).giaddr }
        }
    }
    /* Can have setting to ignore the client ID for a particular MAC address or hostname */
    if !config.is_null() &&
           (*config).flags & 128 as libc::c_int as libc::c_uint != 0 {
        clid = 0 as *mut libc::c_uchar
    }
    /* Check if client is PXE client. */
    if (*dnsmasq_daemon).enable_pxe != 0 &&
           is_pxe_client(mess, sz, &mut pxevendor) != 0 {
        opt = option_find(mess, sz, 97 as libc::c_int, 17 as libc::c_int);
        if !opt.is_null() {
            memcpy(pxe_uuid.as_mut_ptr() as *mut libc::c_void,
                   &mut *opt.offset((2 as
                                         libc::c_uint).wrapping_add(0 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                        as isize) as *mut libc::c_uchar as
                       *mut libc::c_void, 17 as libc::c_int as libc::c_ulong);
            uuid = pxe_uuid.as_mut_ptr()
        }
        /* Check if this is really a PXE bootserver request, and handle specially if so. */
        if (mess_type == 3 as libc::c_int as libc::c_uint ||
                mess_type == 8 as libc::c_int as libc::c_uint) &&
               {
                   opt =
                       option_find(mess, sz, 43 as libc::c_int,
                                   1 as libc::c_int);
                   !opt.is_null()
               } &&
               {
                   opt =
                       option_find1(&mut *opt.offset((2 as
                                                          libc::c_uint).wrapping_add(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint)
                                                         as isize) as
                                        *mut libc::c_uchar as
                                        *mut libc::c_void as
                                        *mut libc::c_uchar,
                                    &mut *opt.offset((2 as
                                                          libc::c_uint).wrapping_add(*opt.offset(1
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint)
                                                         as isize) as
                                        *mut libc::c_uchar as
                                        *mut libc::c_void as
                                        *mut libc::c_uchar, 71 as libc::c_int,
                                    4 as libc::c_int);
                   !opt.is_null()
               } {
            let mut service: *mut pxe_service = 0 as *mut pxe_service;
            let mut type_0: libc::c_int =
                option_uint(opt, 0 as libc::c_int, 2 as libc::c_int) as
                    libc::c_int;
            let mut layer: libc::c_int =
                option_uint(opt, 2 as libc::c_int, 2 as libc::c_int) as
                    libc::c_int;
            let mut save71: [libc::c_uchar; 4] = [0; 4];
            let mut opt71: dhcp_opt =
                dhcp_opt{opt: 0,
                         len: 0,
                         flags: 0,
                         u: C2RustUnnamed_9{encap: 0,},
                         val: 0 as *mut libc::c_uchar,
                         netid: 0 as *mut dhcp_netid,
                         next: 0 as *mut dhcp_opt,};
            if ignore != 0 { return 0 as libc::c_int as size_t }
            if layer & 0x8000 as libc::c_int != 0 {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              3 as libc::c_int,
                          b"PXE BIS not supported\x00" as *const u8 as
                              *const libc::c_char);
                return 0 as libc::c_int as size_t
            }
            memcpy(save71.as_mut_ptr() as *mut libc::c_void,
                   &mut *opt.offset((2 as
                                         libc::c_uint).wrapping_add(0 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                        as isize) as *mut libc::c_uchar as
                       *mut libc::c_void, 4 as libc::c_int as libc::c_ulong);
            service = (*dnsmasq_daemon).pxe_services;
            while !service.is_null() {
                if (*service).type_0 as libc::c_int == type_0 { break ; }
                service = (*service).next
            }
            while !context.is_null() {
                if match_netid((*context).filter, tagif_netid,
                               1 as libc::c_int) != 0 &&
                       is_same_net((*mess).ciaddr, (*context).start,
                                   (*context).netmask) != 0 {
                    break ;
                }
                context = (*context).current
            }
            if service.is_null() || (*service).basename.is_null() ||
                   context.is_null() {
                return 0 as libc::c_int as size_t
            }
            clear_packet(mess, end);
            (*mess).yiaddr = (*mess).ciaddr;
            (*mess).ciaddr.s_addr = 0 as libc::c_int as in_addr_t;
            if !(*service).sname.is_null() {
                (*mess).siaddr = a_record_from_hosts((*service).sname, now)
            } else if (*service).server.s_addr !=
                          0 as libc::c_int as libc::c_uint {
                (*mess).siaddr = (*service).server
            } else { (*mess).siaddr = (*context).local }
            if !strchr((*service).basename, '.' as i32).is_null() {
                snprintf((*mess).file.as_mut_ptr() as *mut libc::c_char,
                         ::std::mem::size_of::<[u8_0; 128]>() as
                             libc::c_ulong,
                         b"%s\x00" as *const u8 as *const libc::c_char,
                         (*service).basename);
            } else {
                snprintf((*mess).file.as_mut_ptr() as *mut libc::c_char,
                         ::std::mem::size_of::<[u8_0; 128]>() as
                             libc::c_ulong,
                         b"%s.%d\x00" as *const u8 as *const libc::c_char,
                         (*service).basename, layer);
            }
            option_put(mess, end, 53 as libc::c_int, 1 as libc::c_int,
                       5 as libc::c_int as libc::c_uint);
            option_put(mess, end, 54 as libc::c_int, 4 as libc::c_int,
                       __bswap_32((*context).local.s_addr));
            pxe_misc(mess, end, uuid, pxevendor);
            prune_vendor_opts(tagif_netid);
            opt71.val = save71.as_mut_ptr();
            opt71.opt = 71 as libc::c_int;
            opt71.len = 4 as libc::c_int;
            opt71.flags = 1024 as libc::c_int;
            opt71.netid = 0 as *mut dhcp_netid;
            opt71.next = (*dnsmasq_daemon).dhcp_opts;
            do_encap_opts(&mut opt71, 43 as libc::c_int, 1024 as libc::c_int,
                          mess, end, 0 as libc::c_int);
            log_packet(b"PXE\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                       &mut (*mess).yiaddr as *mut in_addr as
                           *mut libc::c_void, emac, emac_len, iface_name,
                       (*mess).file.as_mut_ptr() as *mut libc::c_char,
                       0 as *mut libc::c_char, (*mess).xid);
            log_tags(tagif_netid, __bswap_32((*mess).xid));
            return dhcp_packet_size(mess, agent_id, real_end)
        }
        opt = option_find(mess, sz, 93 as libc::c_int, 2 as libc::c_int);
        if !opt.is_null() {
            pxearch =
                option_uint(opt, 0 as libc::c_int, 2 as libc::c_int) as
                    libc::c_int;
            /* proxy DHCP here. */
            if mess_type == 1 as libc::c_int as libc::c_uint ||
                   pxe != 0 && mess_type == 3 as libc::c_int as libc::c_uint {
                let mut tmp_0: *mut dhcp_context = 0 as *mut dhcp_context;
                let mut workaround: libc::c_int = 0 as libc::c_int;
                tmp_0 = context;
                while !tmp_0.is_null() {
                    if (*tmp_0).flags as libc::c_uint &
                           (1 as libc::c_uint) << 3 as libc::c_int != 0 &&
                           match_netid((*tmp_0).filter, tagif_netid,
                                       1 as libc::c_int) != 0 {
                        break ;
                    }
                    tmp_0 = (*tmp_0).current
                }
                if !tmp_0.is_null() {
                    let mut boot: *mut dhcp_boot = 0 as *mut dhcp_boot;
                    let mut redirect4011: libc::c_int = 0 as libc::c_int;
                    if !(*tmp_0).netid.net.is_null() {
                        (*tmp_0).netid.next = netid;
                        tagif_netid = run_tag_if(&mut (*tmp_0).netid)
                    }
                    boot = find_boot(tagif_netid);
                    (*mess).yiaddr.s_addr = 0 as libc::c_int as in_addr_t;
                    if mess_type == 1 as libc::c_int as libc::c_uint ||
                           (*mess).ciaddr.s_addr ==
                               0 as libc::c_int as libc::c_uint {
                        (*mess).ciaddr.s_addr = 0 as libc::c_int as in_addr_t;
                        (*mess).flags =
                            ((*mess).flags as libc::c_int |
                                 __bswap_16(0x8000 as libc::c_int as
                                                __uint16_t) as libc::c_int) as
                                u16_0
                        /* broadcast */
                    }
                    clear_packet(mess, end);
                    /* Redirect EFI clients to port 4011 */
                    if pxearch >= 6 as libc::c_int {
                        redirect4011 = 1 as libc::c_int;
                        (*mess).siaddr = (*tmp_0).local
                    }
                    /* Returns true if only one matching service is available. On port 4011, 
		     it also inserts the boot file and server name. */
                    workaround =
                        pxe_uefi_workaround(pxearch, tagif_netid, mess,
                                            (*tmp_0).local, now, pxe);
                    if workaround == 0 && !boot.is_null() {
                        /* Provide the bootfile here, for iPXE, and in case we have no menu items
			 and set discovery_control = 8 */
                        if (*boot).next_server.s_addr != 0 {
                            (*mess).siaddr = (*boot).next_server
                        } else if !(*boot).tftp_sname.is_null() {
                            (*mess).siaddr =
                                a_record_from_hosts((*boot).tftp_sname, now)
                        }
                        if !(*boot).file.is_null() {
                            safe_strncpy((*mess).file.as_mut_ptr() as
                                             *mut libc::c_char, (*boot).file,
                                         ::std::mem::size_of::<[u8_0; 128]>()
                                             as libc::c_ulong);
                        }
                    }
                    option_put(mess, end, 53 as libc::c_int, 1 as libc::c_int,
                               if mess_type ==
                                      1 as libc::c_int as libc::c_uint {
                                   2 as libc::c_int
                               } else { 5 as libc::c_int } as libc::c_uint);
                    option_put(mess, end, 54 as libc::c_int, 4 as libc::c_int,
                               __bswap_32((*tmp_0).local.s_addr));
                    pxe_misc(mess, end, uuid, pxevendor);
                    prune_vendor_opts(tagif_netid);
                    if pxe != 0 && workaround == 0 || redirect4011 == 0 {
                        do_encap_opts(pxe_opts(pxearch, tagif_netid,
                                               (*tmp_0).local, now),
                                      43 as libc::c_int, 1024 as libc::c_int,
                                      mess, end, 0 as libc::c_int);
                    }
                    (*dnsmasq_daemon).metrics[METRIC_PXE as libc::c_int as
                                                  usize] =
                        (*dnsmasq_daemon).metrics[METRIC_PXE as libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"PXE\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               0 as *mut libc::c_void, emac, emac_len,
                               iface_name,
                               if ignore != 0 {
                                   b"proxy-ignored\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"proxy\x00" as *const u8 as
                                       *const libc::c_char
                               } as *mut libc::c_char, 0 as *mut libc::c_char,
                               (*mess).xid);
                    log_tags(tagif_netid, __bswap_32((*mess).xid));
                    if ignore == 0 {
                        apply_delay((*mess).xid, recvtime, tagif_netid);
                    }
                    return if ignore != 0 {
                               0 as libc::c_int as libc::c_ulong
                           } else {
                               dhcp_packet_size(mess, agent_id, real_end)
                           }
                }
            }
        }
    }
    /* if we're just a proxy server, go no further */
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 3 as libc::c_int != 0 || pxe != 0 {
        return 0 as libc::c_int as size_t
    }
    opt = option_find(mess, sz, 55 as libc::c_int, 0 as libc::c_int);
    if !opt.is_null() {
        req_options = (*dnsmasq_daemon).dhcp_buff2 as *mut libc::c_uchar;
        memcpy(req_options as *mut libc::c_void,
               &mut *opt.offset((2 as
                                     libc::c_uint).wrapping_add(0 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as isize) as *mut libc::c_uchar as
                   *mut libc::c_void,
               *opt.offset(1 as libc::c_int as isize) as libc::c_int as
                   libc::c_ulong);
        *req_options.offset(*opt.offset(1 as libc::c_int as isize) as
                                libc::c_int as isize) =
            255 as libc::c_int as libc::c_uchar
    }
    's_3991:
        {
            match mess_type {
                4 => {
                    opt =
                        option_find(mess, sz, 54 as libc::c_int,
                                    4 as libc::c_int);
                    if opt.is_null() ||
                           option_addr(opt).s_addr !=
                               server_id(context, override_0, fallback).s_addr
                       {
                        return 0 as libc::c_int as size_t
                    }
                    /* sanitise any message. Paranoid? Moi? */
                    sanitise(option_find(mess, sz, 56 as libc::c_int,
                                         1 as libc::c_int),
                             (*dnsmasq_daemon).dhcp_buff);
                    opt =
                        option_find(mess, sz, 50 as libc::c_int,
                                    4 as libc::c_int);
                    if opt.is_null() { return 0 as libc::c_int as size_t }
                    (*dnsmasq_daemon).metrics[METRIC_DHCPDECLINE as
                                                  libc::c_int as usize] =
                        (*dnsmasq_daemon).metrics[METRIC_DHCPDECLINE as
                                                      libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"DHCPDECLINE\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               &mut *opt.offset((2 as
                                                     libc::c_uint).wrapping_add(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                    as isize) as
                                   *mut libc::c_uchar as *mut libc::c_void,
                               emac, emac_len, iface_name,
                               0 as *mut libc::c_char,
                               (*dnsmasq_daemon).dhcp_buff, (*mess).xid);
                    if !lease.is_null() &&
                           (*lease).addr.s_addr == option_addr(opt).s_addr {
                        lease_prune(lease, now);
                    }
                    if !config.is_null() &&
                           (*config).flags & 32 as libc::c_int as libc::c_uint
                               != 0 &&
                           (*config).addr.s_addr == option_addr(opt).s_addr {
                        prettyprint_time((*dnsmasq_daemon).dhcp_buff,
                                         600 as libc::c_int as libc::c_uint);
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      4 as libc::c_int,
                                  b"disabling DHCP static address %s for %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  inet_ntoa((*config).addr),
                                  (*dnsmasq_daemon).dhcp_buff);
                        (*config).flags |=
                            1024 as libc::c_int as libc::c_uint;
                        (*config).decline_time = now
                    } else {
                        /* make sure this host gets a different address next time. */
                        while !context.is_null() {
                            (*context).addr_epoch =
                                (*context).addr_epoch.wrapping_add(1);
                            context = (*context).current
                        }
                    }
                    return 0 as libc::c_int as size_t
                }
                7 => {
                    context =
                        narrow_context(context, (*mess).ciaddr, tagif_netid);
                    if context.is_null() ||
                           {
                               opt =
                                   option_find(mess, sz, 54 as libc::c_int,
                                               4 as libc::c_int);
                               opt.is_null()
                           } ||
                           option_addr(opt).s_addr !=
                               server_id(context, override_0, fallback).s_addr
                       {
                        return 0 as libc::c_int as size_t
                    }
                    if !lease.is_null() &&
                           (*lease).addr.s_addr == (*mess).ciaddr.s_addr {
                        lease_prune(lease, now);
                    } else {
                        message =
                            b"unknown lease\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    }
                    (*dnsmasq_daemon).metrics[METRIC_DHCPRELEASE as
                                                  libc::c_int as usize] =
                        (*dnsmasq_daemon).metrics[METRIC_DHCPRELEASE as
                                                      libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"DHCPRELEASE\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               &mut (*mess).ciaddr as *mut in_addr as
                                   *mut libc::c_void, emac, emac_len,
                               iface_name, 0 as *mut libc::c_char, message,
                               (*mess).xid);
                    return 0 as libc::c_int as size_t
                }
                1 => {
                    if ignore != 0 ||
                           !config.is_null() &&
                               (*config).flags &
                                   1 as libc::c_int as libc::c_uint != 0 {
                        if (*dnsmasq_daemon).options[(42 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (42 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               != 0 {
                            return 0 as libc::c_int as size_t
                        }
                        message =
                            b"ignored\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        opt = 0 as *mut libc::c_uchar
                    } else {
                        let mut addr_0: in_addr = in_addr{s_addr: 0,};
                        let mut conf: in_addr = in_addr{s_addr: 0,};
                        conf.s_addr = 0 as libc::c_int as in_addr_t;
                        addr_0.s_addr = conf.s_addr;
                        opt =
                            option_find(mess, sz, 50 as libc::c_int,
                                        4 as libc::c_int);
                        if !opt.is_null() { addr_0 = option_addr(opt) }
                        if !config.is_null() &&
                               (*config).flags &
                                   32 as libc::c_int as libc::c_uint != 0 {
                            let mut addrs: *mut libc::c_char =
                                inet_ntoa((*config).addr);
                            ltmp = lease_find_by_addr((*config).addr);
                            if !ltmp.is_null() && ltmp != lease &&
                                   config_has_mac(config,
                                                  (*ltmp).hwaddr.as_mut_ptr(),
                                                  (*ltmp).hwaddr_len,
                                                  (*ltmp).hwaddr_type) == 0 {
                                let mut len_3: libc::c_int = 0;
                                let mut mac_0: *mut libc::c_uchar =
                                    extended_hwaddr((*ltmp).hwaddr_type,
                                                    (*ltmp).hwaddr_len,
                                                    (*ltmp).hwaddr.as_mut_ptr(),
                                                    (*ltmp).clid_len,
                                                    (*ltmp).clid, &mut len_3);
                                my_syslog((3 as libc::c_int) <<
                                              3 as libc::c_int |
                                              4 as libc::c_int,
                                          b"not using configured address %s because it is leased to %s\x00"
                                              as *const u8 as
                                              *const libc::c_char, addrs,
                                          print_mac((*dnsmasq_daemon).namebuff,
                                                    mac_0, len_3));
                            } else {
                                let mut tmp_1: *mut dhcp_context =
                                    0 as *mut dhcp_context;
                                tmp_1 = context;
                                while !tmp_1.is_null() {
                                    if (*context).router.s_addr ==
                                           (*config).addr.s_addr {
                                        break ;
                                    }
                                    tmp_1 = (*tmp_1).current
                                }
                                if !tmp_1.is_null() {
                                    my_syslog((3 as libc::c_int) <<
                                                  3 as libc::c_int |
                                                  4 as libc::c_int,
                                              b"not using configured address %s because it is in use by the server or relay\x00"
                                                  as *const u8 as
                                                  *const libc::c_char, addrs);
                                } else if !config.is_null() &&
                                              (*config).flags &
                                                  1024 as libc::c_int as
                                                      libc::c_uint != 0 &&
                                              difftime(now,
                                                       (*config).decline_time)
                                                  <
                                                  600 as libc::c_int as
                                                      libc::c_float as
                                                      libc::c_double {
                                    my_syslog((3 as libc::c_int) <<
                                                  3 as libc::c_int |
                                                  4 as libc::c_int,
                                              b"not using configured address %s because it was previously declined\x00"
                                                  as *const u8 as
                                                  *const libc::c_char, addrs);
                                } else { conf = (*config).addr }
                            }
                        }
                        if conf.s_addr != 0 {
                            (*mess).yiaddr = conf
                        } else if !lease.is_null() &&
                                      !address_available(context,
                                                         (*lease).addr,
                                                         tagif_netid).is_null()
                                      &&
                                      config_find_by_address((*dnsmasq_daemon).dhcp_conf,
                                                             (*lease).addr).is_null()
                         {
                            (*mess).yiaddr = (*lease).addr
                        } else if !opt.is_null() &&
                                      !address_available(context, addr_0,
                                                         tagif_netid).is_null()
                                      && lease_find_by_addr(addr_0).is_null()
                                      &&
                                      config_find_by_address((*dnsmasq_daemon).dhcp_conf,
                                                             addr_0).is_null()
                                      &&
                                      !do_icmp_ping(now, addr_0,
                                                    0 as libc::c_int as
                                                        libc::c_uint,
                                                    loopback).is_null() {
                            (*mess).yiaddr = addr_0
                        } else if emac_len == 0 as libc::c_int {
                            message =
                                b"no unique-id\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char
                        } else if address_allocate(context,
                                                   &mut (*mess).yiaddr, emac,
                                                   emac_len, tagif_netid, now,
                                                   loopback) == 0 {
                            message =
                                b"no address available\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char
                        }
                    }
                    (*dnsmasq_daemon).metrics[METRIC_DHCPDISCOVER as
                                                  libc::c_int as usize] =
                        (*dnsmasq_daemon).metrics[METRIC_DHCPDISCOVER as
                                                      libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"DHCPDISCOVER\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               if !opt.is_null() {
                                   &mut *opt.offset((2 as
                                                         libc::c_uint).wrapping_add(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                                        as isize) as
                                       *mut libc::c_uchar as *mut libc::c_void
                               } else { 0 as *mut libc::c_void }, emac,
                               emac_len, iface_name, 0 as *mut libc::c_char,
                               message, (*mess).xid);
                    if !message.is_null() ||
                           {
                               context =
                                   narrow_context(context, (*mess).yiaddr,
                                                  tagif_netid);
                               context.is_null()
                           } {
                        return 0 as libc::c_int as size_t
                    }
                    if !(*context).netid.net.is_null() {
                        (*context).netid.next = netid;
                        tagif_netid = run_tag_if(&mut (*context).netid)
                    }
                    log_tags(tagif_netid, __bswap_32((*mess).xid));
                    apply_delay((*mess).xid, recvtime, tagif_netid);
                    if (*dnsmasq_daemon).options[(57 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (57 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 &&
                           !option_find(mess, sz, 80 as libc::c_int,
                                        0 as libc::c_int).is_null() {
                        rapid_commit = 1 as libc::c_int
                    } else {
                        (*dnsmasq_daemon).metrics[METRIC_DHCPOFFER as
                                                      libc::c_int as usize] =
                            (*dnsmasq_daemon).metrics[METRIC_DHCPOFFER as
                                                          libc::c_int as
                                                          usize].wrapping_add(1);
                        log_packet(b"DHCPOFFER\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                                   &mut (*mess).yiaddr as *mut in_addr as
                                       *mut libc::c_void, emac, emac_len,
                                   iface_name, 0 as *mut libc::c_char,
                                   0 as *mut libc::c_char, (*mess).xid);
                        time =
                            calc_time(context, config,
                                      option_find(mess, sz, 51 as libc::c_int,
                                                  4 as libc::c_int));
                        clear_packet(mess, end);
                        option_put(mess, end, 53 as libc::c_int,
                                   1 as libc::c_int,
                                   2 as libc::c_int as libc::c_uint);
                        option_put(mess, end, 54 as libc::c_int,
                                   4 as libc::c_int,
                                   __bswap_32(server_id(context, override_0,
                                                        fallback).s_addr));
                        option_put(mess, end, 51 as libc::c_int,
                                   4 as libc::c_int, time);
                        /* T1 and T2 are required in DHCPOFFER by HP's wacky Jetdirect client. */
                        do_options(context, mess, end, req_options,
                                   offer_hostname, get_domain((*mess).yiaddr),
                                   netid, subnet_addr, fqdn_flags, borken_opt,
                                   pxearch, uuid, vendor_class_len, now, time,
                                   fuzz, pxevendor);
                        return dhcp_packet_size(mess, agent_id, real_end)
                    }
                }
                3 => {
                    if ignore != 0 ||
                           !config.is_null() &&
                               (*config).flags &
                                   1 as libc::c_int as libc::c_uint != 0 {
                        return 0 as libc::c_int as size_t
                    }
                    opt =
                        option_find(mess, sz, 50 as libc::c_int,
                                    4 as libc::c_int);
                    if !opt.is_null() {
                        /* SELECTING  or INIT_REBOOT */
                        (*mess).yiaddr = option_addr(opt);
                        /* send vendor and user class info for new or recreated lease */
                        do_classes = 1 as libc::c_int;
                        opt =
                            option_find(mess, sz, 54 as libc::c_int,
                                        4 as libc::c_int);
                        if !opt.is_null() {
                            /* SELECTING */
                            selecting = 1 as libc::c_int;
                            if override_0.s_addr !=
                                   0 as libc::c_int as libc::c_uint {
                                if option_addr(opt).s_addr !=
                                       override_0.s_addr {
                                    return 0 as libc::c_int as size_t
                                }
                            } else {
                                while !context.is_null() {
                                    if (*context).local.s_addr ==
                                           option_addr(opt).s_addr {
                                        break ;
                                    }
                                    context = (*context).current
                                }
                                if context.is_null() {
                                    /* Handle very strange configs where clients have more than one route to the server.
			 If a clients idea of its server-id matches any of our DHCP interfaces, we let it pass.
			 Have to set override to make sure we echo back the correct server-id */
                                    let mut intr: *mut irec = 0 as *mut irec;
                                    enumerate_interfaces(0 as libc::c_int);
                                    intr = (*dnsmasq_daemon).interfaces;
                                    while !intr.is_null() {
                                        if (*intr).addr.sa.sa_family as
                                               libc::c_int == 2 as libc::c_int
                                               &&
                                               (*intr).addr.in_0.sin_addr.s_addr
                                                   == option_addr(opt).s_addr
                                               && (*intr).tftp_ok != 0 {
                                            break ;
                                        }
                                        intr = (*intr).next
                                    }
                                    if !intr.is_null() {
                                        override_0 =
                                            (*intr).addr.in_0.sin_addr
                                    } else {
                                        /* In auth mode, a REQUEST sent to the wrong server
			     should be faulted, so that the client establishes 
			     communication with us, otherwise, silently ignore. */
                                        if (*dnsmasq_daemon).options[(17 as
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
                                                   (17 as libc::c_int as
                                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_ulong))
                                               == 0 {
                                            return 0 as libc::c_int as size_t
                                        }
                                        message =
                                            b"wrong server-ID\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char
                                    }
                                }
                            }
                            /* If a lease exists for this host and another address, squash it. */
                            if !lease.is_null() &&
                                   (*lease).addr.s_addr !=
                                       (*mess).yiaddr.s_addr {
                                lease_prune(lease, now);
                                lease = 0 as *mut dhcp_lease
                            }
                        } else {
                            /* INIT-REBOOT */
                            if lease.is_null() &&
                                   (*dnsmasq_daemon).options[(17 as
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
                                           (17 as libc::c_int as
                                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong))
                                       == 0 {
                                return 0 as libc::c_int as size_t
                            }
                            if !lease.is_null() &&
                                   (*lease).addr.s_addr !=
                                       (*mess).yiaddr.s_addr {
                                message =
                                    b"wrong address\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            }
                        }
                    } else {
                        /* RENEWING or REBINDING */ 
	  /* Check existing lease for this address.
	     We allow it to be missing if dhcp-authoritative mode
	     as long as we can allocate the lease now - checked below.
	     This makes for a smooth recovery from a lost lease DB */
                        if !lease.is_null() &&
                               (*mess).ciaddr.s_addr != (*lease).addr.s_addr
                               ||
                               lease.is_null() &&
                                   (*dnsmasq_daemon).options[(17 as
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
                                           (17 as libc::c_int as
                                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong))
                                       == 0 {
                            /* A client rebinding will broadcast the request, so we may see it even 
		 if the lease is held by another server. Just ignore it in that case. 
		 If the request is unicast to us, then somethings wrong, NAK */
                            if unicast_dest == 0 {
                                return 0 as libc::c_int as size_t
                            }
                            message =
                                b"lease not found\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char;
                            /* ensure we broadcast NAK */
                            unicast_dest = 0 as libc::c_int
                        }
                        /* desynchronise renewals */
                        fuzz = rand16();
                        (*mess).yiaddr = (*mess).ciaddr
                    }
                    (*dnsmasq_daemon).metrics[METRIC_DHCPREQUEST as
                                                  libc::c_int as usize] =
                        (*dnsmasq_daemon).metrics[METRIC_DHCPREQUEST as
                                                      libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"DHCPREQUEST\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               &mut (*mess).yiaddr as *mut in_addr as
                                   *mut libc::c_void, emac, emac_len,
                               iface_name, 0 as *mut libc::c_char,
                               0 as *mut libc::c_char, (*mess).xid);
                }
                8 => {
                    if ignore != 0 ||
                           !config.is_null() &&
                               (*config).flags &
                                   1 as libc::c_int as libc::c_uint != 0 {
                        message =
                            b"ignored\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char
                    }
                    (*dnsmasq_daemon).metrics[METRIC_DHCPINFORM as libc::c_int
                                                  as usize] =
                        (*dnsmasq_daemon).metrics[METRIC_DHCPINFORM as
                                                      libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"DHCPINFORM\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               &mut (*mess).ciaddr as *mut in_addr as
                                   *mut libc::c_void, emac, emac_len,
                               iface_name, message, 0 as *mut libc::c_char,
                               (*mess).xid);
                    if !message.is_null() ||
                           (*mess).ciaddr.s_addr ==
                               0 as libc::c_int as libc::c_uint {
                        return 0 as libc::c_int as size_t
                    }
                    /* For DHCPINFORM only, cope without a valid context */
                    context =
                        narrow_context(context, (*mess).ciaddr, tagif_netid);
                    /* Find a least based on IP address if we didn't
	 get one from MAC address/client-d */
                    if lease.is_null() &&
                           {
                               lease = lease_find_by_addr((*mess).ciaddr);
                               !lease.is_null()
                           } && !(*lease).hostname.is_null() {
                        hostname = (*lease).hostname
                    }
                    if hostname.is_null() {
                        hostname = host_from_dns((*mess).ciaddr)
                    }
                    if !context.is_null() && !(*context).netid.net.is_null() {
                        (*context).netid.next = netid;
                        tagif_netid = run_tag_if(&mut (*context).netid)
                    }
                    log_tags(tagif_netid, __bswap_32((*mess).xid));
                    (*dnsmasq_daemon).metrics[METRIC_DHCPACK as libc::c_int as
                                                  usize] =
                        (*dnsmasq_daemon).metrics[METRIC_DHCPACK as
                                                      libc::c_int as
                                                      usize].wrapping_add(1);
                    log_packet(b"DHCPACK\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               &mut (*mess).ciaddr as *mut in_addr as
                                   *mut libc::c_void, emac, emac_len,
                               iface_name, hostname, 0 as *mut libc::c_char,
                               (*mess).xid);
                    if !lease.is_null() {
                        lease_set_interface(lease, int_index, now);
                        if override_0.s_addr !=
                               0 as libc::c_int as libc::c_uint {
                            (*lease).override_0 = override_0
                        } else { override_0 = (*lease).override_0 }
                    }
                    clear_packet(mess, end);
                    option_put(mess, end, 53 as libc::c_int, 1 as libc::c_int,
                               5 as libc::c_int as libc::c_uint);
                    option_put(mess, end, 54 as libc::c_int, 4 as libc::c_int,
                               __bswap_32(server_id(context, override_0,
                                                    fallback).s_addr));
                    /* RFC 2131 says that DHCPINFORM shouldn't include lease-time parameters, but 
	 we supply a utility which makes DHCPINFORM requests to get this information.
	 Only include lease time if OPTION_LEASE_TIME is in the parameter request list,
	 which won't be true for ordinary clients, but will be true for the 
	 dhcp_lease_time utility. */
                    if !lease.is_null() &&
                           in_list(req_options, 51 as libc::c_int) != 0 {
                        if (*lease).expires ==
                               0 as libc::c_int as libc::c_long {
                            time = 0xffffffff as libc::c_uint
                        } else {
                            time =
                                difftime((*lease).expires, now) as
                                    libc::c_uint
                        } /* handle reply differently */
                        option_put(mess, end, 51 as libc::c_int,
                                   4 as libc::c_int, time);
                    }
                    do_options(context, mess, end, req_options, hostname,
                               get_domain((*mess).ciaddr), netid, subnet_addr,
                               fqdn_flags, borken_opt, pxearch, uuid,
                               vendor_class_len, now,
                               0xffffffff as libc::c_uint,
                               0 as libc::c_int as libc::c_ushort, pxevendor);
                    *is_inform = 1 as libc::c_int;
                    return dhcp_packet_size(mess, agent_id, real_end)
                }
                _ => { break 's_3991 ; }
            }
            if message.is_null() {
                let mut addr_config: *mut dhcp_config = 0 as *mut dhcp_config;
                let mut tmp_2: *mut dhcp_context = 0 as *mut dhcp_context;
                if !config.is_null() &&
                       (*config).flags & 32 as libc::c_int as libc::c_uint !=
                           0 {
                    tmp_2 = context;
                    while !tmp_2.is_null() {
                        if (*context).router.s_addr == (*config).addr.s_addr {
                            break ;
                        }
                        tmp_2 = (*tmp_2).current
                    }
                }
                context =
                    narrow_context(context, (*mess).yiaddr, tagif_netid);
                if context.is_null() {
                    /* If a machine moves networks whilst it has a lease, we catch that here. */
                    message =
                        b"wrong network\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char;
                    /* ensure we broadcast NAK */
                    unicast_dest = 0 as libc::c_int
                } else if address_available(context, (*mess).yiaddr,
                                            tagif_netid).is_null() &&
                              (!(!config.is_null() &&
                                     (*config).flags &
                                         32 as libc::c_int as libc::c_uint !=
                                         0) ||
                                   (*config).addr.s_addr !=
                                       (*mess).yiaddr.s_addr) {
                    message =
                        b"address not available\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char
                } else if tmp_2.is_null() && selecting == 0 &&
                              (!config.is_null() &&
                                   (*config).flags &
                                       32 as libc::c_int as libc::c_uint != 0)
                              &&
                              (!(!config.is_null() &&
                                     (*config).flags &
                                         1024 as libc::c_int as libc::c_uint
                                         != 0) ||
                                   difftime(now, (*config).decline_time) >
                                       600 as libc::c_int as libc::c_float as
                                           libc::c_double) &&
                              (*config).addr.s_addr != (*mess).yiaddr.s_addr
                              &&
                              {
                                  ltmp = lease_find_by_addr((*config).addr);
                                  (ltmp.is_null()) || ltmp == lease
                              } {
                    message =
                        b"static lease available\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char
                } else {
                    /* Check for renewal of a lease which is outside the allowed range. */
                    /* Check if a new static address has been configured. Be very sure that
	     when the client does DISCOVER, it will get the static address, otherwise
	     an endless protocol loop will ensue. */
                    /* Check to see if the address is reserved as a static address for another host */
                    addr_config =
                        config_find_by_address((*dnsmasq_daemon).dhcp_conf,
                                               (*mess).yiaddr);
                    if !addr_config.is_null() && addr_config != config {
                        message =
                            b"address reserved\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if lease.is_null() &&
                                  {
                                      ltmp =
                                          lease_find_by_addr((*mess).yiaddr);
                                      !ltmp.is_null()
                                  } {
                        /* If a host is configured with more than one MAC address, it's OK to 'nix 
		 a lease from one of it's MACs to give the address to another. */
                        if !config.is_null() &&
                               config_has_mac(config,
                                              (*ltmp).hwaddr.as_mut_ptr(),
                                              (*ltmp).hwaddr_len,
                                              (*ltmp).hwaddr_type) != 0 {
                            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                          6 as libc::c_int,
                                      b"abandoning lease to %s of %s\x00" as
                                          *const u8 as *const libc::c_char,
                                      print_mac((*dnsmasq_daemon).namebuff,
                                                (*ltmp).hwaddr.as_mut_ptr(),
                                                (*ltmp).hwaddr_len),
                                      inet_ntoa((*ltmp).addr));
                            lease = ltmp
                        } else {
                            message =
                                b"address in use\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char
                        }
                    }
                }
                if message.is_null() {
                    if emac_len == 0 as libc::c_int {
                        message =
                            b"no unique-id\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if lease.is_null() {
                        lease = lease4_allocate((*mess).yiaddr);
                        if !lease.is_null() {
                            do_classes = 1 as libc::c_int
                        } else {
                            message =
                                b"no leases left\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char
                        }
                    }
                }
            }
            if !message.is_null() {
                (*dnsmasq_daemon).metrics[if rapid_commit != 0 {
                                              METRIC_NOANSWER as libc::c_int
                                          } else {
                                              METRIC_DHCPNAK as libc::c_int
                                          } as usize] =
                    (*dnsmasq_daemon).metrics[if rapid_commit != 0 {
                                                  METRIC_NOANSWER as
                                                      libc::c_int
                                              } else {
                                                  METRIC_DHCPNAK as
                                                      libc::c_int
                                              } as usize].wrapping_add(1);
                log_packet(if rapid_commit != 0 {
                               b"NOANSWER\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"DHCPNAK\x00" as *const u8 as
                                   *const libc::c_char
                           } as *mut libc::c_char,
                           &mut (*mess).yiaddr as *mut in_addr as
                               *mut libc::c_void, emac, emac_len, iface_name,
                           0 as *mut libc::c_char, message, (*mess).xid);
                /* rapid commit case: lease allocate failed but don't send DHCPNAK */
                if rapid_commit != 0 { return 0 as libc::c_int as size_t }
                (*mess).yiaddr.s_addr = 0 as libc::c_int as in_addr_t;
                clear_packet(mess, end);
                option_put(mess, end, 53 as libc::c_int, 1 as libc::c_int,
                           6 as libc::c_int as libc::c_uint);
                option_put(mess, end, 54 as libc::c_int, 4 as libc::c_int,
                           __bswap_32(server_id(context, override_0,
                                                fallback).s_addr));
                option_put_string(mess, end, 56 as libc::c_int, message,
                                  borken_opt);
                /* This fixes a problem with the DHCP spec, broadcasting a NAK to a host on 
	     a distant subnet which unicast a REQ to us won't work. */
                if unicast_dest == 0 ||
                       (*mess).giaddr.s_addr !=
                           0 as libc::c_int as libc::c_uint ||
                       (*mess).ciaddr.s_addr ==
                           0 as libc::c_int as libc::c_uint ||
                       is_same_net((*context).local, (*mess).ciaddr,
                                   (*context).netmask) != 0 {
                    (*mess).flags =
                        ((*mess).flags as libc::c_int |
                             __bswap_16(0x8000 as libc::c_int as __uint16_t)
                                 as libc::c_int) as u16_0; /* broadcast */
                    (*mess).ciaddr.s_addr = 0 as libc::c_int as in_addr_t
                }
            } else {
                if !(*context).netid.net.is_null() {
                    (*context).netid.next = netid;
                    tagif_netid = run_tag_if(&mut (*context).netid)
                }
                log_tags(tagif_netid, __bswap_32((*mess).xid));
                if do_classes != 0 {
                    /* pick up INIT-REBOOT events. */
                    (*lease).flags |= 2 as libc::c_int;
                    if !(*dnsmasq_daemon).lease_change_command.is_null() {
                        let mut n: *mut dhcp_netid = 0 as *mut dhcp_netid;
                        if (*mess).giaddr.s_addr != 0 {
                            (*lease).giaddr = (*mess).giaddr
                        }
                        free((*lease).extradata as *mut libc::c_void);
                        (*lease).extradata = 0 as *mut libc::c_uchar;
                        (*lease).extradata_len =
                            0 as libc::c_int as libc::c_uint;
                        (*lease).extradata_size = (*lease).extradata_len;
                        add_extradata_opt(lease,
                                          option_find(mess, sz,
                                                      60 as libc::c_int,
                                                      1 as libc::c_int));
                        add_extradata_opt(lease,
                                          option_find(mess, sz,
                                                      12 as libc::c_int,
                                                      1 as libc::c_int));
                        add_extradata_opt(lease, oui);
                        add_extradata_opt(lease, serial);
                        add_extradata_opt(lease, class);
                        opt =
                            option_find(mess, sz, 82 as libc::c_int,
                                        1 as libc::c_int);
                        if !opt.is_null() {
                            add_extradata_opt(lease,
                                              option_find1(&mut *opt.offset((2
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(0
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                                as
                                                                                isize)
                                                               as
                                                               *mut libc::c_uchar
                                                               as
                                                               *mut libc::c_void
                                                               as
                                                               *mut libc::c_uchar,
                                                           &mut *opt.offset((2
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(*opt.offset(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            isize)
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                                as
                                                                                isize)
                                                               as
                                                               *mut libc::c_uchar
                                                               as
                                                               *mut libc::c_void
                                                               as
                                                               *mut libc::c_uchar,
                                                           1 as libc::c_int,
                                                           1 as libc::c_int));
                            add_extradata_opt(lease,
                                              option_find1(&mut *opt.offset((2
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(0
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                                as
                                                                                isize)
                                                               as
                                                               *mut libc::c_uchar
                                                               as
                                                               *mut libc::c_void
                                                               as
                                                               *mut libc::c_uchar,
                                                           &mut *opt.offset((2
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(*opt.offset(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            isize)
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                                as
                                                                                isize)
                                                               as
                                                               *mut libc::c_uchar
                                                               as
                                                               *mut libc::c_void
                                                               as
                                                               *mut libc::c_uchar,
                                                           6 as libc::c_int,
                                                           1 as libc::c_int));
                            add_extradata_opt(lease,
                                              option_find1(&mut *opt.offset((2
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(0
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                                as
                                                                                isize)
                                                               as
                                                               *mut libc::c_uchar
                                                               as
                                                               *mut libc::c_void
                                                               as
                                                               *mut libc::c_uchar,
                                                           &mut *opt.offset((2
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(*opt.offset(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            isize)
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                                as
                                                                                isize)
                                                               as
                                                               *mut libc::c_uchar
                                                               as
                                                               *mut libc::c_void
                                                               as
                                                               *mut libc::c_uchar,
                                                           2 as libc::c_int,
                                                           1 as libc::c_int));
                        } else {
                            add_extradata_opt(lease, 0 as *mut libc::c_uchar);
                            add_extradata_opt(lease, 0 as *mut libc::c_uchar);
                            add_extradata_opt(lease, 0 as *mut libc::c_uchar);
                        }
                        /* DNSMASQ_REQUESTED_OPTIONS */
                        opt =
                            option_find(mess, sz, 55 as libc::c_int,
                                        1 as libc::c_int);
                        if !opt.is_null() {
                            let mut len_4: libc::c_int =
                                *opt.offset(1 as libc::c_int as isize) as
                                    libc::c_int;
                            let mut rop: *mut libc::c_uchar =
                                &mut *opt.offset((2 as
                                                      libc::c_uint).wrapping_add(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                                     as isize) as
                                    *mut libc::c_uchar as *mut libc::c_void as
                                    *mut libc::c_uchar;
                            let mut q: *mut libc::c_char =
                                (*dnsmasq_daemon).namebuff;
                            let mut i_0: libc::c_int = 0;
                            i_0 = 0 as libc::c_int;
                            while i_0 < len_4 {
                                q =
                                    q.offset(snprintf(q,
                                                      (1025 as libc::c_int as
                                                           libc::c_long -
                                                           q.wrapping_offset_from((*dnsmasq_daemon).namebuff)
                                                               as
                                                               libc::c_long)
                                                          as libc::c_ulong,
                                                      b"%d%s\x00" as *const u8
                                                          as
                                                          *const libc::c_char,
                                                      *rop.offset(i_0 as
                                                                      isize)
                                                          as libc::c_int,
                                                      if i_0 +
                                                             1 as libc::c_int
                                                             == len_4 {
                                                          b"\x00" as *const u8
                                                              as
                                                              *const libc::c_char
                                                      } else {
                                                          b",\x00" as
                                                              *const u8 as
                                                              *const libc::c_char
                                                      }) as isize);
                                i_0 += 1
                            }
                            lease_add_extradata(lease,
                                                (*dnsmasq_daemon).namebuff as
                                                    *mut libc::c_uchar,
                                                q.wrapping_offset_from((*dnsmasq_daemon).namebuff)
                                                    as libc::c_long as
                                                    libc::c_uint,
                                                0 as libc::c_int);
                        } else {
                            add_extradata_opt(lease, 0 as *mut libc::c_uchar);
                        }
                        /* space-concat tag set */
                        if tagif_netid.is_null() {
                            add_extradata_opt(lease, 0 as *mut libc::c_uchar);
                        } else {
                            n = tagif_netid;
                            while !n.is_null() {
                                let mut n1: *mut dhcp_netid =
                                    0 as *mut dhcp_netid;
                                /* kill dupes */
                                n1 = (*n).next;
                                while !n1.is_null() {
                                    if strcmp((*n).net, (*n1).net) ==
                                           0 as libc::c_int {
                                        break ;
                                    }
                                    n1 = (*n1).next
                                }
                                if n1.is_null() {
                                    lease_add_extradata(lease,
                                                        (*n).net as
                                                            *mut libc::c_uchar,
                                                        strlen((*n).net) as
                                                            libc::c_uint,
                                                        if !(*n).next.is_null()
                                                           {
                                                            ' ' as i32
                                                        } else {
                                                            0 as libc::c_int
                                                        });
                                }
                                n = (*n).next
                            }
                        }
                        opt =
                            option_find(mess, sz, 77 as libc::c_int,
                                        1 as libc::c_int);
                        if !opt.is_null() {
                            let mut len_5: libc::c_int =
                                *opt.offset(1 as libc::c_int as isize) as
                                    libc::c_int;
                            let mut ucp_0: *mut libc::c_uchar =
                                &mut *opt.offset((2 as
                                                      libc::c_uint).wrapping_add(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                                     as isize) as
                                    *mut libc::c_uchar as *mut libc::c_void as
                                    *mut libc::c_uchar;
                            /* If the user-class option started as counted strings, the first byte will be zero. */
                            if len_5 != 0 as libc::c_int &&
                                   *ucp_0.offset(0 as libc::c_int as isize) as
                                       libc::c_int == 0 as libc::c_int {
                                ucp_0 = ucp_0.offset(1);
                                len_5 -= 1
                            }
                            lease_add_extradata(lease, ucp_0,
                                                len_5 as libc::c_uint,
                                                -(1 as libc::c_int));
                        }
                    }
                }
                if hostname_auth == 0 &&
                       {
                           client_hostname = host_from_dns((*mess).yiaddr);
                           !client_hostname.is_null()
                       } {
                    domain = get_domain((*mess).yiaddr);
                    hostname = client_hostname;
                    hostname_auth = 1 as libc::c_int
                }
                time =
                    calc_time(context, config,
                              option_find(mess, sz, 51 as libc::c_int,
                                          4 as libc::c_int));
                lease_set_hwaddr(lease, (*mess).chaddr.as_mut_ptr(), clid,
                                 (*mess).hlen as libc::c_int,
                                 (*mess).htype as libc::c_int, clid_len, now,
                                 do_classes);
                /* if all the netids in the ignore_name list are present, ignore client-supplied name */
                if hostname_auth == 0 {
                    id_list = (*dnsmasq_daemon).dhcp_ignore_names;
                    while !id_list.is_null() {
                        if (*id_list).list.is_null() ||
                               match_netid((*id_list).list, tagif_netid,
                                           0 as libc::c_int) != 0 {
                            break ;
                        }
                        id_list = (*id_list).next
                    }
                    if !id_list.is_null() {
                        hostname = 0 as *mut libc::c_char
                    }
                }
                /* Last ditch, if configured, generate hostname from mac address */
                if hostname.is_null() && emac_len != 0 as libc::c_int {
                    id_list = (*dnsmasq_daemon).dhcp_gen_names;
                    while !id_list.is_null() {
                        if (*id_list).list.is_null() ||
                               match_netid((*id_list).list, tagif_netid,
                                           0 as libc::c_int) != 0 {
                            break ;
                        }
                        id_list = (*id_list).next
                    }
                    if !id_list.is_null() {
                        let mut i_1: libc::c_int = 0;
                        hostname = (*dnsmasq_daemon).dhcp_buff;
                        /* buffer is 256 bytes, 3 bytes per octet */
                        i_1 = 0 as libc::c_int;
                        while i_1 < emac_len && i_1 < 80 as libc::c_int {
                            hostname =
                                hostname.offset(sprintf(hostname,
                                                        b"%.2x%s\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        *emac.offset(i_1 as
                                                                         isize)
                                                            as libc::c_int,
                                                        if i_1 ==
                                                               emac_len -
                                                                   1 as
                                                                       libc::c_int
                                                           {
                                                            b"\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                        } else {
                                                            b"-\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                        }) as isize);
                            i_1 += 1
                        }
                        hostname = (*dnsmasq_daemon).dhcp_buff
                    }
                }
                if !hostname.is_null() {
                    lease_set_hostname(lease, hostname, hostname_auth,
                                       get_domain((*lease).addr), domain);
                }
                lease_set_expires(lease, time, now);
                lease_set_interface(lease, int_index, now);
                if override_0.s_addr != 0 as libc::c_int as libc::c_uint {
                    (*lease).override_0 = override_0
                } else { override_0 = (*lease).override_0 }
                (*dnsmasq_daemon).metrics[METRIC_DHCPACK as libc::c_int as
                                              usize] =
                    (*dnsmasq_daemon).metrics[METRIC_DHCPACK as libc::c_int as
                                                  usize].wrapping_add(1);
                log_packet(b"DHCPACK\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char,
                           &mut (*mess).yiaddr as *mut in_addr as
                               *mut libc::c_void, emac, emac_len, iface_name,
                           hostname, 0 as *mut libc::c_char, (*mess).xid);
                clear_packet(mess, end);
                option_put(mess, end, 53 as libc::c_int, 1 as libc::c_int,
                           5 as libc::c_int as libc::c_uint);
                option_put(mess, end, 54 as libc::c_int, 4 as libc::c_int,
                           __bswap_32(server_id(context, override_0,
                                                fallback).s_addr));
                option_put(mess, end, 51 as libc::c_int, 4 as libc::c_int,
                           time);
                if rapid_commit != 0 {
                    option_put(mess, end, 80 as libc::c_int, 0 as libc::c_int,
                               0 as libc::c_int as libc::c_uint);
                }
                do_options(context, mess, end, req_options, hostname,
                           get_domain((*mess).yiaddr), netid, subnet_addr,
                           fqdn_flags, borken_opt, pxearch, uuid,
                           vendor_class_len, now, time, fuzz, pxevendor);
            }
            return dhcp_packet_size(mess, agent_id, real_end)
        }
    return 0 as libc::c_int as size_t;
}
/* find a good value to use as MAC address for logging and address-allocation hashing.
   This is normally just the chaddr field from the DHCP packet,
   but eg Firewire will have hlen == 0 and use the client-id instead. 
   This could be anything, but will normally be EUI64 for Firewire.
   We assume that if the first byte of the client-id equals the htype byte
   then the client-id is using the usual encoding and use the rest of the 
   client-id: if not we can use the whole client-id. This should give
   sane MAC address logs. */
#[no_mangle]
pub unsafe extern "C" fn extended_hwaddr(mut hwtype: libc::c_int,
                                         mut hwlen: libc::c_int,
                                         mut hwaddr: *mut libc::c_uchar,
                                         mut clid_len: libc::c_int,
                                         mut clid: *mut libc::c_uchar,
                                         mut len_out: *mut libc::c_int)
 -> *mut libc::c_uchar {
    if hwlen == 0 as libc::c_int && !clid.is_null() &&
           clid_len > 3 as libc::c_int {
        if *clid.offset(0 as libc::c_int as isize) as libc::c_int == hwtype {
            *len_out = clid_len - 1 as libc::c_int; /* sanity */
            return clid.offset(1 as libc::c_int as isize)
        } /* add terminator */
        if *clid.offset(0 as libc::c_int as isize) as libc::c_int ==
               27 as libc::c_int && hwtype == 24 as libc::c_int {
            *len_out = clid_len - 1 as libc::c_int;
            return clid.offset(1 as libc::c_int as isize)
        }
        *len_out = clid_len;
        return clid
    }
    *len_out = hwlen;
    return hwaddr;
}
unsafe extern "C" fn calc_time(mut context: *mut dhcp_context,
                               mut config: *mut dhcp_config,
                               mut opt: *mut libc::c_uchar) -> libc::c_uint {
    let mut time: libc::c_uint =
        if !config.is_null() &&
               (*config).flags & 8 as libc::c_int as libc::c_uint != 0 {
            (*config).lease_time
        } else { (*context).lease_time };
    if !opt.is_null() {
        let mut req_time: libc::c_uint =
            option_uint(opt, 0 as libc::c_int, 4 as libc::c_int);
        if req_time < 120 as libc::c_int as libc::c_uint {
            req_time = 120 as libc::c_int as libc::c_uint
        }
        if time == 0xffffffff as libc::c_uint ||
               req_time != 0xffffffff as libc::c_uint && req_time < time {
            time = req_time
        }
    }
    return time;
}
unsafe extern "C" fn server_id(mut context: *mut dhcp_context,
                               mut override_0: in_addr, mut fallback: in_addr)
 -> in_addr {
    if override_0.s_addr != 0 as libc::c_int as libc::c_uint {
        return override_0
    } else if !context.is_null() &&
                  (*context).local.s_addr != 0 as libc::c_int as libc::c_uint
     {
        return (*context).local
    } else { return fallback };
}
unsafe extern "C" fn sanitise(mut opt: *mut libc::c_uchar,
                              mut buf: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    *buf = 0 as libc::c_int as libc::c_char;
    if opt.is_null() { return 0 as libc::c_int }
    p =
        &mut *opt.offset((2 as
                              libc::c_uint).wrapping_add(0 as libc::c_int as
                                                             libc::c_uint) as
                             isize) as *mut libc::c_uchar as *mut libc::c_void
            as *mut libc::c_char;
    i = *opt.offset(1 as libc::c_int as isize) as libc::c_int;
    while i > 0 as libc::c_int {
        let fresh7 = p;
        p = p.offset(1);
        let mut c: libc::c_char = *fresh7;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
               libc::c_int &
               _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            let fresh8 = buf;
            buf = buf.offset(1);
            *fresh8 = c
        }
        i -= 1
    }
    *buf = 0 as libc::c_int as libc::c_char;
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_extradata_opt(mut lease: *mut dhcp_lease,
                                       mut opt: *mut libc::c_uchar) {
    if opt.is_null() {
        lease_add_extradata(lease, 0 as *mut libc::c_uchar,
                            0 as libc::c_int as libc::c_uint,
                            0 as libc::c_int);
    } else {
        lease_add_extradata(lease,
                            &mut *opt.offset((2 as
                                                  libc::c_uint).wrapping_add(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                                                 as isize) as
                                *mut libc::c_uchar as *mut libc::c_void as
                                *mut libc::c_uchar,
                            *opt.offset(1 as libc::c_int as isize) as
                                libc::c_int as libc::c_uint,
                            0 as libc::c_int);
    };
}
unsafe extern "C" fn log_packet(mut type_0: *mut libc::c_char,
                                mut addr: *mut libc::c_void,
                                mut ext_mac: *mut libc::c_uchar,
                                mut mac_len: libc::c_int,
                                mut interface: *mut libc::c_char,
                                mut string: *mut libc::c_char,
                                mut err: *mut libc::c_char, mut xid: u32_0) {
    let mut a: in_addr = in_addr{s_addr: 0,};
    if err.is_null() &&
           (*dnsmasq_daemon).options[(28 as libc::c_int as
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
               == 0 &&
           (*dnsmasq_daemon).options[(42 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (42 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        return
    }
    /* addr may be misaligned */
    if !addr.is_null() {
        memcpy(&mut a as *mut in_addr as *mut libc::c_void, addr,
               ::std::mem::size_of::<in_addr>() as
                   libc::c_ulong); /* malformed packet */
    } /* malformed packet */
    print_mac((*dnsmasq_daemon).namebuff, ext_mac, mac_len);
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
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u %s(%s) %s%s%s %s%s\x00" as *const u8 as
                      *const libc::c_char, __bswap_32(xid), type_0, interface,
                  if !addr.is_null() {
                      inet_ntoa(a) as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  if !addr.is_null() {
                      b" \x00" as *const u8 as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  (*dnsmasq_daemon).namebuff,
                  if !string.is_null() {
                      string as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  if !err.is_null() {
                      err as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char });
    } else {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%s(%s) %s%s%s %s%s\x00" as *const u8 as
                      *const libc::c_char, type_0, interface,
                  if !addr.is_null() {
                      inet_ntoa(a) as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  if !addr.is_null() {
                      b" \x00" as *const u8 as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  (*dnsmasq_daemon).namebuff,
                  if !string.is_null() {
                      string as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  if !err.is_null() {
                      err as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char });
    };
}
unsafe extern "C" fn log_options(mut start: *mut libc::c_uchar,
                                 mut xid: u32_0) {
    while *start as libc::c_int != 255 as libc::c_int {
        let mut optname: *mut libc::c_char =
            option_string(2 as libc::c_int,
                          *start.offset(0 as libc::c_int as isize) as
                              libc::c_uint,
                          &mut *start.offset((2 as
                                                  libc::c_uint).wrapping_add(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                                                 as isize) as
                              *mut libc::c_uchar as *mut libc::c_void as
                              *mut libc::c_uchar,
                          *start.offset(1 as libc::c_int as isize) as
                              libc::c_int, (*dnsmasq_daemon).namebuff,
                          1025 as libc::c_int);
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u sent size:%3d option:%3d %s  %s\x00" as *const u8 as
                      *const libc::c_char, __bswap_32(xid),
                  *start.offset(1 as libc::c_int as isize) as libc::c_int,
                  *start.offset(0 as libc::c_int as isize) as libc::c_int,
                  optname, (*dnsmasq_daemon).namebuff);
        start =
            start.offset((*start.offset(1 as libc::c_int as isize) as
                              libc::c_int + 2 as libc::c_int) as isize)
    };
}
unsafe extern "C" fn option_find1(mut p: *mut libc::c_uchar,
                                  mut end: *mut libc::c_uchar,
                                  mut opt: libc::c_int,
                                  mut minsize: libc::c_int)
 -> *mut libc::c_uchar {
    loop  {
        if p >= end {
            return 0 as *mut libc::c_uchar
        } else {
            if *p as libc::c_int == 255 as libc::c_int {
                return if opt == 255 as libc::c_int {
                           p
                       } else { 0 as *mut libc::c_uchar }
            } else {
                if *p as libc::c_int == 0 as libc::c_int {
                    p = p.offset(1)
                } else {
                    let mut opt_len: libc::c_int = 0;
                    if p > end.offset(-(2 as libc::c_int as isize)) {
                        return 0 as *mut libc::c_uchar
                    }
                    opt_len =
                        *p.offset(1 as libc::c_int as isize) as libc::c_int;
                    if p >
                           end.offset(-((2 as libc::c_int + opt_len) as
                                            isize)) {
                        return 0 as *mut libc::c_uchar
                    }
                    if *p as libc::c_int == opt && opt_len >= minsize {
                        return p
                    }
                    p = p.offset((opt_len + 2 as libc::c_int) as isize)
                }
            }
        }
    };
}
unsafe extern "C" fn option_find(mut mess: *mut dhcp_packet, mut size: size_t,
                                 mut opt_type: libc::c_int,
                                 mut minsize: libc::c_int)
 -> *mut libc::c_uchar {
    let mut ret: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut overload: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* skip over DHCP cookie; */
    ret =
        option_find1((&mut *(*mess).options.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                          as
                          *mut u8_0).offset(::std::mem::size_of::<u32_0>() as
                                                libc::c_ulong as isize),
                     (mess as *mut libc::c_uchar).offset(size as isize),
                     opt_type, minsize);
    if !ret.is_null() { return ret }
    /* look for overload option. */
    overload =
        option_find1((&mut *(*mess).options.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                          as
                          *mut u8_0).offset(::std::mem::size_of::<u32_0>() as
                                                libc::c_ulong as isize),
                     (mess as *mut libc::c_uchar).offset(size as isize),
                     52 as libc::c_int, 1 as libc::c_int);
    if overload.is_null() { return 0 as *mut libc::c_uchar }
    /* Can we look in filename area ? */
    if *overload.offset(2 as libc::c_int as isize) as libc::c_int &
           1 as libc::c_int != 0 &&
           {
               ret =
                   option_find1(&mut *(*mess).file.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                                &mut *(*mess).file.as_mut_ptr().offset(128 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                                opt_type, minsize);
               !ret.is_null()
           } {
        return ret
    }
    /* finally try sname area */
    if *overload.offset(2 as libc::c_int as isize) as libc::c_int &
           2 as libc::c_int != 0 &&
           {
               ret =
                   option_find1(&mut *(*mess).sname.as_mut_ptr().offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                &mut *(*mess).sname.as_mut_ptr().offset(64 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                opt_type, minsize);
               !ret.is_null()
           } {
        return ret
    }
    return 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn option_addr(mut opt: *mut libc::c_uchar) -> in_addr {
    /* this worries about unaligned data in the option. */
  /* struct in_addr is network byte order */
    let mut ret: in_addr = in_addr{s_addr: 0,};
    memcpy(&mut ret as *mut in_addr as *mut libc::c_void,
           &mut *opt.offset((2 as
                                 libc::c_uint).wrapping_add(0 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as isize) as *mut libc::c_uchar as
               *mut libc::c_void, 4 as libc::c_int as libc::c_ulong);
    return ret;
}
unsafe extern "C" fn option_uint(mut opt: *mut libc::c_uchar,
                                 mut offset: libc::c_int,
                                 mut size: libc::c_int) -> libc::c_uint {
    /* this worries about unaligned data and byte order */
    let mut ret: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_uchar =
        &mut *opt.offset((2 as
                              libc::c_uint).wrapping_add(offset as
                                                             libc::c_uint) as
                             isize) as *mut libc::c_uchar as *mut libc::c_void
            as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < size {
        let fresh9 = p;
        p = p.offset(1);
        ret = ret << 8 as libc::c_int | *fresh9 as libc::c_uint;
        i += 1
    }
    return ret;
}
unsafe extern "C" fn dhcp_skip_opts(mut start: *mut libc::c_uchar)
 -> *mut libc::c_uchar {
    while *start as libc::c_int != 0 as libc::c_int {
        start =
            start.offset((*start.offset(1 as libc::c_int as isize) as
                              libc::c_int + 2 as libc::c_int) as isize)
    }
    return start;
}
/* only for use when building packet: doesn't check for bad data. */
unsafe extern "C" fn find_overload(mut mess: *mut dhcp_packet)
 -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar =
        (&mut *(*mess).options.as_mut_ptr().offset(0 as libc::c_int as isize)
             as
             *mut u8_0).offset(::std::mem::size_of::<u32_0>() as libc::c_ulong
                                   as isize);
    while *p as libc::c_int != 0 as libc::c_int {
        if *p as libc::c_int == 52 as libc::c_int { return p }
        p =
            p.offset((*p.offset(1 as libc::c_int as isize) as libc::c_int +
                          2 as libc::c_int) as isize)
    }
    return 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn dhcp_packet_size(mut mess: *mut dhcp_packet,
                                      mut agent_id: *mut libc::c_uchar,
                                      mut real_end: *mut libc::c_uchar)
 -> size_t {
    let mut p: *mut libc::c_uchar =
        dhcp_skip_opts((&mut *(*mess).options.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                            as
                            *mut u8_0).offset(::std::mem::size_of::<u32_0>()
                                                  as libc::c_ulong as isize));
    let mut overload: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: size_t = 0;
    /* move agent_id back down to the end of the packet */
    if !agent_id.is_null() {
        memmove(p as *mut libc::c_void, agent_id as *const libc::c_void,
                real_end.wrapping_offset_from(agent_id) as libc::c_long as
                    libc::c_ulong);
        p =
            p.offset(real_end.wrapping_offset_from(agent_id) as libc::c_long
                         as isize);
        memset(p as *mut libc::c_void, 0 as libc::c_int,
               real_end.wrapping_offset_from(p) as libc::c_long as
                   libc::c_ulong);
        /* in case of overlap */
    }
    /* add END options to the regions. */
    overload = find_overload(mess);
    if !overload.is_null() &&
           option_uint(overload, 0 as libc::c_int, 1 as libc::c_int) &
               1 as libc::c_int as libc::c_uint != 0 {
        *dhcp_skip_opts((*mess).file.as_mut_ptr()) =
            255 as libc::c_int as libc::c_uchar;
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
            log_options((*mess).file.as_mut_ptr(), (*mess).xid);
        }
    } else if (*dnsmasq_daemon).options[(28 as libc::c_int as
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
                  != 0 &&
                  strlen((*mess).file.as_mut_ptr() as *mut libc::c_char) !=
                      0 as libc::c_int as libc::c_ulong {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u bootfile name: %s\x00" as *const u8 as
                      *const libc::c_char, __bswap_32((*mess).xid),
                  (*mess).file.as_mut_ptr() as *mut libc::c_char);
    }
    if !overload.is_null() &&
           option_uint(overload, 0 as libc::c_int, 1 as libc::c_int) &
               2 as libc::c_int as libc::c_uint != 0 {
        *dhcp_skip_opts((*mess).sname.as_mut_ptr()) =
            255 as libc::c_int as libc::c_uchar;
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
            log_options((*mess).sname.as_mut_ptr(), (*mess).xid);
        }
    } else if (*dnsmasq_daemon).options[(28 as libc::c_int as
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
                  != 0 &&
                  strlen((*mess).sname.as_mut_ptr() as *mut libc::c_char) !=
                      0 as libc::c_int as libc::c_ulong {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u server name: %s\x00" as *const u8 as
                      *const libc::c_char, __bswap_32((*mess).xid),
                  (*mess).sname.as_mut_ptr() as *mut libc::c_char);
    }
    let fresh10 = p;
    p = p.offset(1);
    *fresh10 = 255 as libc::c_int as libc::c_uchar;
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
        if (*mess).siaddr.s_addr != 0 as libc::c_int as libc::c_uint {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u next server: %s\x00" as *const u8 as
                          *const libc::c_char, __bswap_32((*mess).xid),
                      inet_ntoa((*mess).siaddr));
        }
        if (*mess).flags as libc::c_int &
               __bswap_16(0x8000 as libc::c_int as __uint16_t) as libc::c_int
               != 0 &&
               (*mess).ciaddr.s_addr == 0 as libc::c_int as libc::c_uint {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u broadcast response\x00" as *const u8 as
                          *const libc::c_char, __bswap_32((*mess).xid));
        }
        log_options((&mut *(*mess).options.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                         as
                         *mut u8_0).offset(::std::mem::size_of::<u32_0>() as
                                               libc::c_ulong as isize),
                    (*mess).xid);
    }
    ret =
        p.wrapping_offset_from(mess as *mut libc::c_uchar) as libc::c_long as
            size_t;
    if ret < 300 as libc::c_int as libc::c_ulong {
        ret = 300 as libc::c_int as size_t
    }
    return ret;
}
unsafe extern "C" fn free_space(mut mess: *mut dhcp_packet,
                                mut end: *mut libc::c_uchar,
                                mut opt: libc::c_int, mut len: libc::c_int)
 -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar =
        dhcp_skip_opts((&mut *(*mess).options.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                            as
                            *mut u8_0).offset(::std::mem::size_of::<u32_0>()
                                                  as libc::c_ulong as isize));
    if p.offset(len as isize).offset(3 as libc::c_int as isize) >= end {
        /* not enough space in options area, try and use overload, if poss */
        let mut overload: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        overload = find_overload(mess);
        if overload.is_null() &&
               ((*mess).file[0 as libc::c_int as usize] as libc::c_int ==
                    0 as libc::c_int ||
                    (*mess).sname[0 as libc::c_int as usize] as libc::c_int ==
                        0 as libc::c_int) {
            /* attempt to overload fname and sname areas, we've reserved space for the
	     overflow option previuously. */
            overload = p;
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = 52 as libc::c_int as libc::c_uchar;
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = 1 as libc::c_int as libc::c_uchar
        }
        p = 0 as *mut libc::c_uchar;
        /* using filename field ? */
        if !overload.is_null() {
            if (*mess).file[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                let ref mut fresh13 =
                    *overload.offset(2 as libc::c_int as isize);
                *fresh13 =
                    (*fresh13 as libc::c_int | 1 as libc::c_int) as
                        libc::c_uchar
            }
            if *overload.offset(2 as libc::c_int as isize) as libc::c_int &
                   1 as libc::c_int != 0 {
                p = dhcp_skip_opts((*mess).file.as_mut_ptr());
                if p.offset(len as isize).offset(3 as libc::c_int as isize) >=
                       (*mess).file.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 128]>()
                                                            as libc::c_ulong
                                                            as isize) {
                    p = 0 as *mut libc::c_uchar
                }
            }
            if p.is_null() {
                /* try to bring sname into play (it may be already) */
                if (*mess).sname[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int {
                    let ref mut fresh14 =
                        *overload.offset(2 as libc::c_int as isize);
                    *fresh14 =
                        (*fresh14 as libc::c_int | 2 as libc::c_int) as
                            libc::c_uchar
                }
                if *overload.offset(2 as libc::c_int as isize) as libc::c_int
                       & 2 as libc::c_int != 0 {
                    p = dhcp_skip_opts((*mess).sname.as_mut_ptr());
                    if p.offset(len as
                                    isize).offset(3 as libc::c_int as isize)
                           >=
                           (*mess).sname.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 64]>()
                                                                 as
                                                                 libc::c_ulong
                                                                 as isize) {
                        p = 0 as *mut libc::c_uchar
                    }
                }
            }
        }
        if p.is_null() {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          4 as libc::c_int,
                      b"cannot send DHCP/BOOTP option %d: no space left in packet\x00"
                          as *const u8 as *const libc::c_char, opt);
        }
    }
    if !p.is_null() {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = opt as libc::c_uchar;
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = len as libc::c_uchar
    }
    return p;
}
unsafe extern "C" fn option_put(mut mess: *mut dhcp_packet,
                                mut end: *mut libc::c_uchar,
                                mut opt: libc::c_int, mut len: libc::c_int,
                                mut val: libc::c_uint) {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = free_space(mess, end, opt, len);
    if !p.is_null() {
        i = 0 as libc::c_int;
        while i < len {
            let fresh17 = p;
            p = p.offset(1);
            *fresh17 =
                (val >> 8 as libc::c_int * (len - (i + 1 as libc::c_int))) as
                    libc::c_uchar;
            i += 1
        }
    };
}
unsafe extern "C" fn option_put_string(mut mess: *mut dhcp_packet,
                                       mut end: *mut libc::c_uchar,
                                       mut opt: libc::c_int,
                                       mut string: *const libc::c_char,
                                       mut null_term: libc::c_int) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = strlen(string);
    if null_term != 0 && len != 255 as libc::c_int as libc::c_ulong {
        len = len.wrapping_add(1)
    }
    p = free_space(mess, end, opt, len as libc::c_int);
    if !p.is_null() {
        memcpy(p as *mut libc::c_void, string as *const libc::c_void, len);
    };
}
/* return length, note this only does the data part */
unsafe extern "C" fn do_opt(mut opt: *mut dhcp_opt, mut p: *mut libc::c_uchar,
                            mut context: *mut dhcp_context,
                            mut null_term: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = (*opt).len;
    if (*opt).flags & 2 as libc::c_int != 0 && null_term != 0 &&
           len != 255 as libc::c_int {
        len += 1
    }
    if !p.is_null() && len != 0 as libc::c_int {
        if !context.is_null() && (*opt).flags & 1 as libc::c_int != 0 {
            let mut j: libc::c_int = 0;
            let mut a: *mut in_addr = (*opt).val as *mut in_addr;
            j = 0 as libc::c_int;
            while j < (*opt).len {
                /* zero means "self" (but not in vendorclass options.) */
                if (*a).s_addr == 0 as libc::c_int as libc::c_uint {
                    memcpy(p as *mut libc::c_void,
                           &mut (*context).local as *mut in_addr as
                               *const libc::c_void,
                           4 as libc::c_int as libc::c_ulong);
                } else {
                    memcpy(p as *mut libc::c_void, a as *const libc::c_void,
                           4 as libc::c_int as libc::c_ulong);
                }
                p = p.offset(4 as libc::c_int as isize);
                j += 4 as libc::c_int;
                a = a.offset(1)
            }
        } else {
            /* empty string may be extended to "\0" by null_term */
            memcpy(p as *mut libc::c_void,
                   if !(*opt).val.is_null() {
                       (*opt).val
                   } else {
                       b"\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_uchar
                   } as *const libc::c_void, len as libc::c_ulong);
        }
    }
    return len;
}
unsafe extern "C" fn in_list(mut list: *mut libc::c_uchar,
                             mut opt: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* If no requested options, send everything, not nothing. */
    if list.is_null() { return 1 as libc::c_int }
    i = 0 as libc::c_int;
    while *list.offset(i as isize) as libc::c_int != 255 as libc::c_int {
        if opt == *list.offset(i as isize) as libc::c_int {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn option_find2(mut opt: libc::c_int) -> *mut dhcp_opt {
    let mut opts: *mut dhcp_opt = 0 as *mut dhcp_opt;
    opts = (*dnsmasq_daemon).dhcp_opts;
    while !opts.is_null() {
        if (*opts).opt == opt && (*opts).flags & 4096 as libc::c_int != 0 {
            return opts
        }
        opts = (*opts).next
    }
    return 0 as *mut dhcp_opt;
}
/* mark vendor-encapsulated options which match the client-supplied  or
   config-supplied vendor class */
unsafe extern "C" fn match_vendor_opts(mut opt: *mut libc::c_uchar,
                                       mut dopt: *mut dhcp_opt) {
    while !dopt.is_null() {
        (*dopt).flags &= !(1024 as libc::c_int);
        if !opt.is_null() && (*dopt).flags & 256 as libc::c_int != 0 {
            let mut pv: *const dhcp_pxe_vendor = 0 as *const dhcp_pxe_vendor;
            let mut dummy_vendor: dhcp_pxe_vendor =
                {
                    let mut init =
                        dhcp_pxe_vendor{data:
                                            (*dopt).u.vendor_class as
                                                *mut libc::c_char,
                                        next: 0 as *mut dhcp_pxe_vendor,};
                    init
                };
            if (*dopt).flags & 16384 as libc::c_int != 0 {
                pv = (*dnsmasq_daemon).dhcp_pxe_vendors
            } else { pv = &mut dummy_vendor }
            while !pv.is_null() {
                let mut i: libc::c_int = 0;
                let mut len: libc::c_int = 0 as libc::c_int;
                let mut matched: libc::c_int = 0 as libc::c_int;
                if !(*pv).data.is_null() {
                    len = strlen((*pv).data) as libc::c_int
                }
                i = 0 as libc::c_int;
                while i <=
                          *opt.offset(1 as libc::c_int as isize) as
                              libc::c_int - len {
                    if len == 0 as libc::c_int ||
                           memcmp((*pv).data as *const libc::c_void,
                                  &mut *opt.offset((2 as
                                                        libc::c_uint).wrapping_add(i
                                                                                       as
                                                                                       libc::c_uint)
                                                       as isize) as
                                      *mut libc::c_uchar as *mut libc::c_void,
                                  len as libc::c_ulong) == 0 as libc::c_int {
                        matched = 1 as libc::c_int;
                        break ;
                    } else { i += 1 }
                }
                if matched != 0 {
                    (*dopt).flags |= 1024 as libc::c_int;
                    break ;
                } else { pv = (*pv).next }
            }
        }
        dopt = (*dopt).next
    };
}
unsafe extern "C" fn do_encap_opts(mut opt: *mut dhcp_opt,
                                   mut encap: libc::c_int,
                                   mut flag: libc::c_int,
                                   mut mess: *mut dhcp_packet,
                                   mut end: *mut libc::c_uchar,
                                   mut null_term: libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut enc_len: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut start: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* find size in advance */
    enc_len = 0 as libc::c_int;
    start = opt;
    while !opt.is_null() {
        if (*opt).flags & flag != 0 {
            let mut new: libc::c_int =
                do_opt(opt, 0 as *mut libc::c_uchar, 0 as *mut dhcp_context,
                       null_term) + 2 as libc::c_int;
            ret = 1 as libc::c_int;
            if enc_len + new <= 255 as libc::c_int {
                enc_len += new
            } else {
                p = free_space(mess, end, encap, enc_len);
                while !start.is_null() && start != opt {
                    if !p.is_null() && (*start).flags & flag != 0 {
                        len =
                            do_opt(start, p.offset(2 as libc::c_int as isize),
                                   0 as *mut dhcp_context, null_term);
                        let fresh18 = p;
                        p = p.offset(1);
                        *fresh18 = (*start).opt as libc::c_uchar;
                        let fresh19 = p;
                        p = p.offset(1);
                        *fresh19 = len as libc::c_uchar;
                        p = p.offset(len as isize)
                    }
                    start = (*start).next
                }
                enc_len = new;
                start = opt
            }
        }
        opt = (*opt).next
    }
    if enc_len != 0 as libc::c_int &&
           {
               p = free_space(mess, end, encap, enc_len + 1 as libc::c_int);
               !p.is_null()
           } {
        while !start.is_null() {
            if (*start).flags & flag != 0 {
                len =
                    do_opt(start, p.offset(2 as libc::c_int as isize),
                           0 as *mut dhcp_context, null_term);
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = (*start).opt as libc::c_uchar;
                let fresh21 = p;
                p = p.offset(1);
                *fresh21 = len as libc::c_uchar;
                p = p.offset(len as isize)
            }
            start = (*start).next
        }
        *p = 255 as libc::c_int as libc::c_uchar
    }
    return ret;
}
unsafe extern "C" fn pxe_misc(mut mess: *mut dhcp_packet,
                              mut end: *mut libc::c_uchar,
                              mut uuid: *mut libc::c_uchar,
                              mut pxevendor: *const libc::c_char) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if pxevendor.is_null() {
        pxevendor = b"PXEClient\x00" as *const u8 as *const libc::c_char
    }
    option_put_string(mess, end, 60 as libc::c_int, pxevendor,
                      0 as libc::c_int);
    if !uuid.is_null() &&
           {
               p =
                   free_space(mess, end, 97 as libc::c_int,
                              17 as libc::c_int);
               !p.is_null()
           } {
        memcpy(p as *mut libc::c_void, uuid as *const libc::c_void,
               17 as libc::c_int as libc::c_ulong);
    };
}
unsafe extern "C" fn prune_vendor_opts(mut netid: *mut dhcp_netid)
 -> libc::c_int {
    let mut force: libc::c_int = 0 as libc::c_int;
    let mut opt: *mut dhcp_opt = 0 as *mut dhcp_opt;
    /* prune vendor-encapsulated options based on netid, and look if we're forcing them to be sent */
    opt = (*dnsmasq_daemon).dhcp_opts;
    while !opt.is_null() {
        if (*opt).flags & 1024 as libc::c_int != 0 {
            if match_netid((*opt).netid, netid, 1 as libc::c_int) == 0 {
                (*opt).flags &= !(1024 as libc::c_int)
            } else if (*opt).flags & 16 as libc::c_int != 0 {
                force = 1 as libc::c_int
            }
        }
        opt = (*opt).next
    }
    return force;
}
/* Many UEFI PXE implementations have badly broken menu code.
   If there's exactly one relevant menu item, we abandon the menu system,
   and jamb the data direct into the DHCP file, siaddr and sname fields.
   Note that in this case, we have to assume that layer zero would be requested
   by the client PXE stack. */
unsafe extern "C" fn pxe_uefi_workaround(mut pxe_arch: libc::c_int,
                                         mut netid: *mut dhcp_netid,
                                         mut mess: *mut dhcp_packet,
                                         mut local: in_addr, mut now: time_t,
                                         mut pxe: libc::c_int)
 -> libc::c_int {
    let mut service: *mut pxe_service = 0 as *mut pxe_service;
    let mut found: *mut pxe_service = 0 as *mut pxe_service;
    /* Only workaround UEFI archs. */
    if pxe_arch < 6 as libc::c_int {
        return 0 as libc::c_int
    } /* More than one relevant menu item */
    found = 0 as *mut pxe_service; /* No relevant menu items. */
    service = (*dnsmasq_daemon).pxe_services;
    while !service.is_null() {
        if pxe_arch == (*service).CSA as libc::c_int &&
               !(*service).basename.is_null() &&
               match_netid((*service).netid, netid, 1 as libc::c_int) != 0 {
            if !found.is_null() { return 0 as libc::c_int }
            found = service
        }
        service = (*service).next
    }
    if found.is_null() { return 0 as libc::c_int }
    if pxe == 0 { return 1 as libc::c_int }
    if !(*found).sname.is_null() {
        (*mess).siaddr = a_record_from_hosts((*found).sname, now);
        snprintf((*mess).sname.as_mut_ptr() as *mut libc::c_char,
                 ::std::mem::size_of::<[u8_0; 64]>() as libc::c_ulong,
                 b"%s\x00" as *const u8 as *const libc::c_char,
                 (*found).sname);
    } else {
        if (*found).server.s_addr != 0 as libc::c_int as libc::c_uint {
            (*mess).siaddr = (*found).server
        } else { (*mess).siaddr = local }
        inet_ntop(2 as libc::c_int,
                  &mut (*mess).siaddr as *mut in_addr as *const libc::c_void,
                  (*mess).sname.as_mut_ptr() as *mut libc::c_char,
                  16 as libc::c_int as socklen_t);
    }
    snprintf((*mess).file.as_mut_ptr() as *mut libc::c_char,
             ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong,
             if !strchr((*found).basename, '.' as i32).is_null() {
                 b"%s\x00" as *const u8 as *const libc::c_char
             } else { b"%s.0\x00" as *const u8 as *const libc::c_char },
             (*found).basename);
    return 1 as libc::c_int;
}
unsafe extern "C" fn pxe_opts(mut pxe_arch: libc::c_int,
                              mut netid: *mut dhcp_netid, mut local: in_addr,
                              mut now: time_t) -> *mut dhcp_opt {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut service: *mut pxe_service = 0 as *mut pxe_service;
    static mut o: *mut dhcp_opt = 0 as *const dhcp_opt as *mut dhcp_opt;
    static mut ret: *mut dhcp_opt = 0 as *const dhcp_opt as *mut dhcp_opt;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 4 as libc::c_int - 1 as libc::c_int;
    let mut boot_server: in_addr = in_addr{s_addr: 0,};
    /* We pass back references to these, hence they are declared static */
    static mut discovery_control: libc::c_uchar = 0;
    static mut fake_prompt: [libc::c_uchar; 4] =
        [0 as libc::c_int as libc::c_uchar, 'P' as i32 as libc::c_uchar,
         'X' as i32 as libc::c_uchar, 'E' as i32 as libc::c_uchar];
    static mut fake_opts: *mut dhcp_opt =
        0 as *const dhcp_opt as *mut dhcp_opt;
    /* Disable multicast, since we don't support it, and broadcast
     unless we need it */
    discovery_control = 3 as libc::c_int as libc::c_uchar;
    ret = (*dnsmasq_daemon).dhcp_opts;
    if fake_opts.is_null() &&
           {
               fake_opts =
                   whine_malloc((4 as libc::c_int as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<dhcp_opt>()
                                                                     as
                                                                     libc::c_ulong))
                       as *mut dhcp_opt;
               fake_opts.is_null()
           } {
        return ret
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*fake_opts.offset(i as isize)).flags = 1024 as libc::c_int;
        let ref mut fresh22 = (*fake_opts.offset(i as isize)).netid;
        *fresh22 = 0 as *mut dhcp_netid;
        let ref mut fresh23 = (*fake_opts.offset(i as isize)).next;
        *fresh23 =
            if i == 4 as libc::c_int - 1 as libc::c_int {
                ret
            } else {
                &mut *fake_opts.offset((i + 1 as libc::c_int) as isize) as
                    *mut dhcp_opt
            };
        i += 1
    }
    /* create the data for the PXE_MENU and PXE_SERVERS options. */
    p = (*dnsmasq_daemon).dhcp_buff as *mut libc::c_uchar;
    q = (*dnsmasq_daemon).dhcp_buff3 as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    service = (*dnsmasq_daemon).pxe_services;
    while !service.is_null() {
        if pxe_arch == (*service).CSA as libc::c_int &&
               match_netid((*service).netid, netid, 1 as libc::c_int) != 0 {
            's_185:
                {
                    let mut current_block_29: u64;
                    let mut len: size_t = strlen((*service).menu);
                    /* opt 43 max size is 255. encapsulated option has type and length
	   bytes, so its max size is 253. */
                    if (p.wrapping_offset_from((*dnsmasq_daemon).dhcp_buff as
                                                   *mut libc::c_uchar) as
                            libc::c_long as
                            libc::c_ulong).wrapping_add(len).wrapping_add(3 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                           < 253 as libc::c_int as libc::c_ulong {
                        let fresh24 = p;
                        p = p.offset(1);
                        *fresh24 =
                            ((*service).type_0 as libc::c_int >>
                                 8 as libc::c_int) as libc::c_uchar;
                        let fresh25 = p;
                        p = p.offset(1);
                        *fresh25 = (*service).type_0 as libc::c_uchar;
                        let fresh26 = p;
                        p = p.offset(1);
                        *fresh26 = len as libc::c_uchar;
                        memcpy(p as *mut libc::c_void,
                               (*service).menu as *const libc::c_void, len);
                        p = p.offset(len as isize);
                        i += 1;
                        boot_server =
                            if !(*service).basename.is_null() {
                                local
                            } else if !(*service).sname.is_null() {
                                a_record_from_hosts((*service).sname, now)
                            } else { (*service).server };
                        if boot_server.s_addr !=
                               0 as libc::c_int as libc::c_uint {
                            if q.wrapping_offset_from((*dnsmasq_daemon).dhcp_buff3
                                                          as
                                                          *mut libc::c_uchar)
                                   as libc::c_long +
                                   3 as libc::c_int as libc::c_long +
                                   4 as libc::c_int as libc::c_long >=
                                   253 as libc::c_int as libc::c_long {
                                current_block_29 = 14257305712396241914;
                            } else {
                                /* Boot service with known address - give it */
                                let fresh27 = q;
                                q = q.offset(1);
                                *fresh27 =
                                    ((*service).type_0 as libc::c_int >>
                                         8 as libc::c_int) as libc::c_uchar;
                                let fresh28 = q;
                                q = q.offset(1);
                                *fresh28 = (*service).type_0 as libc::c_uchar;
                                let fresh29 = q;
                                q = q.offset(1);
                                *fresh29 = 1 as libc::c_int as libc::c_uchar;
                                /* dest misaligned */
                                memcpy(q as *mut libc::c_void,
                                       &mut boot_server.s_addr as
                                           *mut in_addr_t as
                                           *const libc::c_void,
                                       4 as libc::c_int as libc::c_ulong);
                                q = q.offset(4 as libc::c_int as isize);
                                current_block_29 = 6450636197030046351;
                            }
                        } else {
                            if (*service).type_0 as libc::c_int !=
                                   0 as libc::c_int {
                                /* We don't know the server for a service type, so we'll
	     allow the client to broadcast for it */
                                discovery_control =
                                    2 as libc::c_int as libc::c_uchar
                            }
                            current_block_29 = 6450636197030046351;
                        }
                        match current_block_29 {
                            14257305712396241914 => { }
                            _ => { break 's_185 ; }
                        }
                    }
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  3 as libc::c_int,
                              b"PXE menu too large\x00" as *const u8 as
                                  *const libc::c_char);
                    return (*dnsmasq_daemon).dhcp_opts
                }
        }
        service = (*service).next
    }
    /* if no prompt, wait forever if there's a choice */
    fake_prompt[0 as libc::c_int as usize] =
        if i > 1 as libc::c_int {
            255 as libc::c_int
        } else { 0 as libc::c_int } as
            libc::c_uchar; /* no menu - just use use mess->filename */
    if i == 0 as libc::c_int {
        discovery_control = 8 as libc::c_int as libc::c_uchar
    } else {
        let fresh30 = j;
        j = j - 1;
        ret = &mut *fake_opts.offset(fresh30 as isize) as *mut dhcp_opt;
        (*ret).len =
            p.wrapping_offset_from((*dnsmasq_daemon).dhcp_buff as
                                       *mut libc::c_uchar) as libc::c_long as
                libc::c_int;
        (*ret).val = (*dnsmasq_daemon).dhcp_buff as *mut libc::c_uchar;
        (*ret).opt = 9 as libc::c_int;
        if q.wrapping_offset_from((*dnsmasq_daemon).dhcp_buff3 as
                                      *mut libc::c_uchar) as libc::c_long !=
               0 as libc::c_int as libc::c_long {
            let fresh31 = j;
            j = j - 1;
            ret = &mut *fake_opts.offset(fresh31 as isize) as *mut dhcp_opt;
            (*ret).len =
                q.wrapping_offset_from((*dnsmasq_daemon).dhcp_buff3 as
                                           *mut libc::c_uchar) as libc::c_long
                    as libc::c_int;
            (*ret).val = (*dnsmasq_daemon).dhcp_buff3 as *mut libc::c_uchar;
            (*ret).opt = 8 as libc::c_int
        }
    }
    o = (*dnsmasq_daemon).dhcp_opts;
    while !o.is_null() {
        if (*o).flags & 1024 as libc::c_int != 0 &&
               (*o).opt == 10 as libc::c_int {
            break ;
        }
        o = (*o).next
    }
    if o.is_null() {
        let fresh32 = j;
        j = j - 1;
        ret = &mut *fake_opts.offset(fresh32 as isize) as *mut dhcp_opt;
        (*ret).len =
            ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as
                libc::c_int;
        (*ret).val = fake_prompt.as_mut_ptr();
        (*ret).opt = 10 as libc::c_int
    }
    let fresh33 = j;
    j = j - 1;
    ret = &mut *fake_opts.offset(fresh33 as isize) as *mut dhcp_opt;
    (*ret).len = 1 as libc::c_int;
    (*ret).opt = 6 as libc::c_int;
    (*ret).val = &mut discovery_control;
    return ret;
}
unsafe extern "C" fn clear_packet(mut mess: *mut dhcp_packet,
                                  mut end: *mut libc::c_uchar) {
    memset((*mess).sname.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[u8_0; 64]>() as libc::c_ulong);
    memset((*mess).file.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong);
    memset((&mut *(*mess).options.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
                *mut u8_0).offset(::std::mem::size_of::<u32_0>() as
                                      libc::c_ulong as isize) as
               *mut libc::c_void, 0 as libc::c_int,
           end.wrapping_offset_from((&mut *(*mess).options.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                         as
                                         *mut u8_0).offset(::std::mem::size_of::<u32_0>()
                                                               as
                                                               libc::c_ulong
                                                               as isize)) as
               libc::c_long as libc::c_ulong);
    (*mess).siaddr.s_addr = 0 as libc::c_int as in_addr_t;
}
#[no_mangle]
pub unsafe extern "C" fn find_boot(mut netid: *mut dhcp_netid)
 -> *mut dhcp_boot {
    let mut boot: *mut dhcp_boot = 0 as *mut dhcp_boot;
    /* decide which dhcp-boot option we're using */
    boot = (*dnsmasq_daemon).boot_config;
    while !boot.is_null() {
        if match_netid((*boot).netid, netid, 0 as libc::c_int) != 0 {
            break ;
        }
        boot = (*boot).next
    }
    if boot.is_null() {
        /* No match, look for one without a netid */
        boot = (*dnsmasq_daemon).boot_config;
        while !boot.is_null() {
            if match_netid((*boot).netid, netid, 1 as libc::c_int) != 0 {
                break ;
            }
            boot = (*boot).next
        }
    }
    return boot;
}
unsafe extern "C" fn is_pxe_client(mut mess: *mut dhcp_packet, mut sz: size_t,
                                   mut pxe_vendor: *mut *const libc::c_char)
 -> libc::c_int {
    let mut opt: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut conf_len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut conf: *const dhcp_pxe_vendor = (*dnsmasq_daemon).dhcp_pxe_vendors;
    opt = option_find(mess, sz, 60 as libc::c_int, 0 as libc::c_int);
    if opt.is_null() { return 0 as libc::c_int }
    while !conf.is_null() {
        conf_len = strlen((*conf).data) as ssize_t;
        if !((*(opt as *mut libc::c_uchar).offset(1 as libc::c_int as isize)
                  as libc::c_int as libc::c_long) < conf_len) {
            if strncmp(&mut *(opt as
                                  *mut libc::c_uchar).offset((2 as
                                                                  libc::c_uint).wrapping_add(0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)
                                                                 as isize) as
                           *mut libc::c_uchar as *mut libc::c_void as
                           *const libc::c_char, (*conf).data,
                       conf_len as libc::c_ulong) == 0 as libc::c_int {
                if !pxe_vendor.is_null() { *pxe_vendor = (*conf).data }
                return 1 as libc::c_int
            }
        }
        conf = (*conf).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_options(mut context: *mut dhcp_context,
                                mut mess: *mut dhcp_packet,
                                mut end: *mut libc::c_uchar,
                                mut req_options: *mut libc::c_uchar,
                                mut hostname: *mut libc::c_char,
                                mut domain: *mut libc::c_char,
                                mut netid: *mut dhcp_netid,
                                mut subnet_addr: in_addr,
                                mut fqdn_flags: libc::c_uchar,
                                mut null_term: libc::c_int,
                                mut pxe_arch: libc::c_int,
                                mut uuid: *mut libc::c_uchar,
                                mut vendor_class_len: libc::c_int,
                                mut now: time_t, mut lease_time: libc::c_uint,
                                mut fuzz: libc::c_ushort,
                                mut pxevendor: *const libc::c_char) {
    let mut opt: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut config_opts: *mut dhcp_opt = (*dnsmasq_daemon).dhcp_opts;
    let mut boot: *mut dhcp_boot = 0 as *mut dhcp_boot;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut force_encap: libc::c_int = 0 as libc::c_int;
    let mut f0: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut s0: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut done_file: libc::c_int = 0 as libc::c_int;
    let mut done_server: libc::c_int = 0 as libc::c_int;
    let mut done_vendor_class: libc::c_int = 0 as libc::c_int;
    let mut tagif: *mut dhcp_netid = 0 as *mut dhcp_netid;
    let mut id_list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
    /* filter options based on tags, those we want get DHOPT_TAGOK bit set */
    if !context.is_null() { (*context).netid.next = 0 as *mut dhcp_netid }
    tagif =
        option_filter(netid,
                      if !context.is_null() && !(*context).netid.net.is_null()
                         {
                          &mut (*context).netid
                      } else { 0 as *mut dhcp_netid }, config_opts);
    /* logging */
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
           != 0 && !req_options.is_null() {
        let mut q: *mut libc::c_char =
            (*dnsmasq_daemon).namebuff; /* force broadcast */
        i = 0 as libc::c_int;
        while *req_options.offset(i as isize) as libc::c_int !=
                  255 as libc::c_int {
            let mut s: *mut libc::c_char =
                option_string(2 as libc::c_int,
                              *req_options.offset(i as isize) as libc::c_uint,
                              0 as *mut libc::c_uchar, 0 as libc::c_int,
                              0 as *mut libc::c_char, 0 as libc::c_int);
            q =
                q.offset(snprintf(q,
                                  (1025 as libc::c_int as libc::c_long -
                                       q.wrapping_offset_from((*dnsmasq_daemon).namebuff)
                                           as libc::c_long) as libc::c_ulong,
                                  b"%d%s%s%s\x00" as *const u8 as
                                      *const libc::c_char,
                                  *req_options.offset(i as isize) as
                                      libc::c_int,
                                  if strlen(s) !=
                                         0 as libc::c_int as libc::c_ulong {
                                      b":\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b"\x00" as *const u8 as
                                          *const libc::c_char
                                  }, s,
                                  if *req_options.offset((i +
                                                              1 as
                                                                  libc::c_int)
                                                             as isize) as
                                         libc::c_int == 255 as libc::c_int {
                                      b"\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b", \x00" as *const u8 as
                                          *const libc::c_char
                                  }) as isize);
            if *req_options.offset((i + 1 as libc::c_int) as isize) as
                   libc::c_int == 255 as libc::c_int ||
                   q.wrapping_offset_from((*dnsmasq_daemon).namebuff) as
                       libc::c_long > 40 as libc::c_int as libc::c_long {
                q = (*dnsmasq_daemon).namebuff;
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"%u requested options: %s\x00" as *const u8 as
                              *const libc::c_char, __bswap_32((*mess).xid),
                          (*dnsmasq_daemon).namebuff);
            }
            i += 1
        }
    }
    id_list = (*dnsmasq_daemon).force_broadcast;
    while !id_list.is_null() {
        if (*id_list).list.is_null() ||
               match_netid((*id_list).list, netid, 0 as libc::c_int) != 0 {
            break ;
        }
        id_list = (*id_list).next
    }
    if !id_list.is_null() {
        (*mess).flags =
            ((*mess).flags as libc::c_int |
                 __bswap_16(0x8000 as libc::c_int as __uint16_t) as
                     libc::c_int) as u16_0
    }
    if !context.is_null() { (*mess).siaddr = (*context).local }
    /* See if we can send the boot stuff as options.
     To do this we need a requested option list, BOOTP
     and very old DHCP clients won't have this, we also 
     provide a manual option to disable it.
     Some PXE ROMs have bugs (surprise!) and need zero-terminated 
     names, so we always send those.  */
    boot = find_boot(tagif);
    if !boot.is_null() {
        if !(*boot).sname.is_null() {
            if (*dnsmasq_daemon).options[(30 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (30 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 && !req_options.is_null() &&
                   in_list(req_options, 66 as libc::c_int) != 0 {
                option_put_string(mess, end, 66 as libc::c_int, (*boot).sname,
                                  1 as libc::c_int);
            } else {
                safe_strncpy((*mess).sname.as_mut_ptr() as *mut libc::c_char,
                             (*boot).sname,
                             ::std::mem::size_of::<[u8_0; 64]>() as
                                 libc::c_ulong);
            }
        }
        if !(*boot).file.is_null() {
            if (*dnsmasq_daemon).options[(30 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (30 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 && !req_options.is_null() &&
                   in_list(req_options, 67 as libc::c_int) != 0 {
                option_put_string(mess, end, 67 as libc::c_int, (*boot).file,
                                  1 as libc::c_int);
            } else {
                safe_strncpy((*mess).file.as_mut_ptr() as *mut libc::c_char,
                             (*boot).file,
                             ::std::mem::size_of::<[u8_0; 128]>() as
                                 libc::c_ulong);
            }
        }
        if (*boot).next_server.s_addr != 0 {
            (*mess).siaddr = (*boot).next_server
        } else if !(*boot).tftp_sname.is_null() {
            (*mess).siaddr = a_record_from_hosts((*boot).tftp_sname, now)
        }
    } else {
        /* Use the values of the relevant options if no dhcp-boot given and
       they're not explicitly asked for as options. OPTION_END is used
       as an internal way to specify siaddr without using dhcp-boot, for use in
       dhcp-optsfile. */
        if (req_options.is_null() ||
                in_list(req_options, 67 as libc::c_int) == 0) &&
               { opt = option_find2(67 as libc::c_int); !opt.is_null() } &&
               (*opt).flags & 16 as libc::c_int == 0 {
            safe_strncpy((*mess).file.as_mut_ptr() as *mut libc::c_char,
                         (*opt).val as *mut libc::c_char,
                         ::std::mem::size_of::<[u8_0; 128]>() as
                             libc::c_ulong);
            done_file = 1 as libc::c_int
        }
        if (req_options.is_null() ||
                in_list(req_options, 66 as libc::c_int) == 0) &&
               { opt = option_find2(66 as libc::c_int); !opt.is_null() } &&
               (*opt).flags & 16 as libc::c_int == 0 {
            safe_strncpy((*mess).sname.as_mut_ptr() as *mut libc::c_char,
                         (*opt).val as *mut libc::c_char,
                         ::std::mem::size_of::<[u8_0; 64]>() as
                             libc::c_ulong);
            done_server = 1 as libc::c_int
        }
        opt = option_find2(255 as libc::c_int);
        if !opt.is_null() {
            (*mess).siaddr.s_addr = (*((*opt).val as *mut in_addr)).s_addr
        }
    }
    /* We don't want to do option-overload for BOOTP, so make the file and sname
     fields look like they are in use, even when they aren't. This gets restored
     at the end of this function. */
    if req_options.is_null() ||
           (*dnsmasq_daemon).options[(30 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (30 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        f0 = (*mess).file[0 as libc::c_int as usize];
        (*mess).file[0 as libc::c_int as usize] = 1 as libc::c_int as u8_0;
        s0 = (*mess).sname[0 as libc::c_int as usize];
        (*mess).sname[0 as libc::c_int as usize] = 1 as libc::c_int as u8_0
    }
    /* At this point, if mess->sname or mess->file are zeroed, they are available
     for option overload, reserve space for the overload option. */
    if (*mess).file[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int ||
           (*mess).sname[0 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
        end = end.offset(-(3 as libc::c_int as isize))
    }
    /* rfc3011 says this doesn't need to be in the requested options list. */
    if subnet_addr.s_addr != 0 {
        option_put(mess, end, 118 as libc::c_int, 4 as libc::c_int,
                   __bswap_32(subnet_addr.s_addr));
    }
    if lease_time != 0xffffffff as libc::c_uint {
        let mut t1val: libc::c_uint =
            lease_time.wrapping_div(2 as libc::c_int as libc::c_uint);
        let mut t2val: libc::c_uint =
            lease_time.wrapping_mul(7 as libc::c_int as
                                        libc::c_uint).wrapping_div(8 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
        let mut hval: libc::c_uint = 0;
        /* If set by user, sanity check, so not longer than lease. */
        opt = option_find2(58 as libc::c_int);
        if !opt.is_null() {
            hval = __bswap_32(*((*opt).val as *mut libc::c_uint));
            if hval < lease_time && hval > 2 as libc::c_int as libc::c_uint {
                t1val = hval
            }
        }
        opt = option_find2(59 as libc::c_int);
        if !opt.is_null() {
            hval = __bswap_32(*((*opt).val as *mut libc::c_uint));
            if hval < lease_time && hval > 2 as libc::c_int as libc::c_uint {
                t2val = hval
            }
        }
        /* ensure T1 is still < T2 */
        if t2val <= t1val {
            t1val = t2val.wrapping_sub(1 as libc::c_int as libc::c_uint)
        }
        while fuzz as libc::c_uint >
                  t1val.wrapping_div(8 as libc::c_int as libc::c_uint) {
            fuzz = (fuzz as libc::c_int / 2 as libc::c_int) as libc::c_ushort
        }
        t1val = t1val.wrapping_sub(fuzz as libc::c_uint);
        t2val = t2val.wrapping_sub(fuzz as libc::c_uint);
        option_put(mess, end, 58 as libc::c_int, 4 as libc::c_int, t1val);
        option_put(mess, end, 59 as libc::c_int, 4 as libc::c_int, t2val);
    }
    /* replies to DHCPINFORM may not have a valid context */
    if !context.is_null() {
        if option_find2(1 as libc::c_int).is_null() {
            option_put(mess, end, 1 as libc::c_int, 4 as libc::c_int,
                       __bswap_32((*context).netmask.s_addr));
        }
        /* May not have a "guessed" broadcast address if we got no packets via a relay
	 from this net yet (ie just unicast renewals after a restart */
        if (*context).broadcast.s_addr != 0 &&
               option_find2(28 as libc::c_int).is_null() {
            option_put(mess, end, 28 as libc::c_int, 4 as libc::c_int,
                       __bswap_32((*context).broadcast.s_addr));
        }
        /* Same comments as broadcast apply, and also may not be able to get a sensible
	 default when using subnet select.  User must configure by steam in that case. */
        if (*context).router.s_addr != 0 &&
               in_list(req_options, 3 as libc::c_int) != 0 &&
               option_find2(3 as libc::c_int).is_null() {
            option_put(mess, end, 3 as libc::c_int, 4 as libc::c_int,
                       __bswap_32((*context).router.s_addr));
        }
        if (*dnsmasq_daemon).port == 53 as libc::c_int &&
               in_list(req_options, 6 as libc::c_int) != 0 &&
               option_find2(6 as libc::c_int).is_null() {
            option_put(mess, end, 6 as libc::c_int, 4 as libc::c_int,
                       __bswap_32((*context).local.s_addr));
        }
    }
    if !domain.is_null() && in_list(req_options, 15 as libc::c_int) != 0 &&
           option_find2(15 as libc::c_int).is_null() {
        option_put_string(mess, end, 15 as libc::c_int, domain, null_term);
    }
    /* Note that we ignore attempts to set the fqdn using --dhc-option=81,<name> */
    if !hostname.is_null() {
        if in_list(req_options, 12 as libc::c_int) != 0 &&
               option_find2(12 as libc::c_int).is_null() {
            option_put_string(mess, end, 12 as libc::c_int, hostname,
                              null_term); /* MBZ bits to zero */
        }
        if fqdn_flags as libc::c_int != 0 as libc::c_int {
            len =
                strlen(hostname).wrapping_add(3 as libc::c_int as
                                                  libc::c_ulong) as
                    libc::c_int;
            if fqdn_flags as libc::c_int & 0x4 as libc::c_int != 0 {
                len += 2 as libc::c_int
            } else if null_term != 0 { len += 1 }
            if !domain.is_null() {
                len =
                    (len as
                         libc::c_ulong).wrapping_add(strlen(domain).wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
                        as libc::c_int as libc::c_int
            } else if fqdn_flags as libc::c_int & 0x4 as libc::c_int != 0 {
                len -= 1
            }
            p = free_space(mess, end, 81 as libc::c_int, len);
            if !p.is_null() {
                let fresh34 = p;
                p = p.offset(1);
                *fresh34 =
                    (fqdn_flags as libc::c_int & 0xf as libc::c_int) as
                        libc::c_uchar;
                let fresh35 = p;
                p = p.offset(1);
                *fresh35 = 255 as libc::c_int as libc::c_uchar;
                let fresh36 = p;
                p = p.offset(1);
                *fresh36 = 255 as libc::c_int as libc::c_uchar;
                if fqdn_flags as libc::c_int & 0x4 as libc::c_int != 0 {
                    p = do_rfc1035_name(p, hostname, 0 as *mut libc::c_char);
                    if !domain.is_null() {
                        p =
                            do_rfc1035_name(p, domain,
                                            0 as *mut libc::c_char);
                        let fresh37 = p;
                        p = p.offset(1);
                        *fresh37 = 0 as libc::c_int as libc::c_uchar
                    }
                } else {
                    memcpy(p as *mut libc::c_void,
                           hostname as *const libc::c_void, strlen(hostname));
                    p = p.offset(strlen(hostname) as isize);
                    if !domain.is_null() {
                        let fresh38 = p;
                        p = p.offset(1);
                        *fresh38 = '.' as i32 as libc::c_uchar;
                        memcpy(p as *mut libc::c_void,
                               domain as *const libc::c_void, strlen(domain));
                        p = p.offset(strlen(domain) as isize)
                    }
                    if null_term != 0 {
                        let fresh39 = p;
                        p = p.offset(1);
                        *fresh39 = 0 as libc::c_int as libc::c_uchar
                    }
                }
            }
        }
    }
    opt = config_opts;
    while !opt.is_null() {
        let mut optno: libc::c_int = (*opt).opt;
        /* netids match and not encapsulated? */
        if !((*opt).flags & 4096 as libc::c_int == 0) {
            /* was it asked for, or are we sending it anyway? */
            if !((*opt).flags & 16 as libc::c_int == 0 &&
                     in_list(req_options, optno) == 0) {
                /* prohibit some used-internally options. T1 and T2 already handled. */
                if !(optno == 81 as libc::c_int || optno == 57 as libc::c_int
                         || optno == 52 as libc::c_int ||
                         optno == 0 as libc::c_int ||
                         optno == 255 as libc::c_int ||
                         optno == 58 as libc::c_int ||
                         optno == 59 as libc::c_int) {
                    if !(optno == 66 as libc::c_int && done_server != 0) {
                        if !(optno == 67 as libc::c_int && done_file != 0) {
                            /* For the options we have default values on
	 dhc-option=<optionno> means "don't include this option"
	 not "include a zero-length option" */
                            if !((*opt).len == 0 as libc::c_int &&
                                     (optno == 1 as libc::c_int ||
                                          optno == 28 as libc::c_int ||
                                          optno == 3 as libc::c_int ||
                                          optno == 6 as libc::c_int ||
                                          optno == 15 as libc::c_int ||
                                          optno == 12 as libc::c_int)) {
                                /* vendor-class comes from elsewhere for PXE */
                                if !(pxe_arch != -(1 as libc::c_int) &&
                                         optno == 60 as libc::c_int) {
                                    /* always force null-term for filename and servername - buggy PXE again. */
                                    len =
                                        do_opt(opt, 0 as *mut libc::c_uchar,
                                               context,
                                               if optno == 66 as libc::c_int
                                                      ||
                                                      optno ==
                                                          67 as libc::c_int {
                                                   1 as libc::c_int
                                               } else { null_term });
                                    p = free_space(mess, end, optno, len);
                                    if !p.is_null() {
                                        do_opt(opt, p, context,
                                               if optno == 66 as libc::c_int
                                                      ||
                                                      optno ==
                                                          67 as libc::c_int {
                                                   1 as libc::c_int
                                               } else { null_term });
                                        /* If we send a vendor-id, revisit which vendor-ops we consider 
	     it appropriate to send. */
                                        if optno == 60 as libc::c_int {
                                            match_vendor_opts(p.offset(-(2 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)),
                                                              config_opts);
                                            done_vendor_class =
                                                1 as libc::c_int
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        opt = (*opt).next
    }
    /* Now send options to be encapsulated in arbitrary options, 
     eg dhcp-option=encap:172,17,.......
     Also handle vendor-identifying vendor-encapsulated options,
     dhcp-option = vi-encap:13,17,.......
     The may be more that one "outer" to do, so group
     all the options which match each outer in turn. */
    opt = config_opts;
    while !opt.is_null() {
        (*opt).flags &= !(64 as libc::c_int);
        opt = (*opt).next
    }
    opt = config_opts;
    while !opt.is_null() {
        let mut flags: libc::c_int = 0;
        flags = (*opt).flags & (4 as libc::c_int | 2048 as libc::c_int);
        if flags != 0 {
            let mut found: libc::c_int = 0 as libc::c_int;
            let mut o: *mut dhcp_opt = 0 as *mut dhcp_opt;
            if !((*opt).flags & 64 as libc::c_int != 0) {
                len = 0 as libc::c_int;
                o = config_opts;
                while !o.is_null() {
                    let mut outer: libc::c_int =
                        if flags & 4 as libc::c_int != 0 {
                            (*o).u.encap
                        } else { 125 as libc::c_int };
                    (*o).flags &= !(8 as libc::c_int);
                    if !((*o).flags & flags == 0 ||
                             (*opt).u.encap != (*o).u.encap) {
                        (*o).flags |= 64 as libc::c_int;
                        if match_netid((*o).netid, tagif, 1 as libc::c_int) !=
                               0 &&
                               ((*o).flags & 16 as libc::c_int != 0 ||
                                    in_list(req_options, outer) != 0) {
                            (*o).flags |= 8 as libc::c_int;
                            found = 1 as libc::c_int;
                            len +=
                                do_opt(o, 0 as *mut libc::c_uchar,
                                       0 as *mut dhcp_context,
                                       0 as libc::c_int) + 2 as libc::c_int
                        }
                    }
                    o = (*o).next
                }
                if found != 0 {
                    if flags & 4 as libc::c_int != 0 {
                        do_encap_opts(config_opts, (*opt).u.encap,
                                      8 as libc::c_int, mess, end, null_term);
                    } else if len > 250 as libc::c_int {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      4 as libc::c_int,
                                  b"cannot send RFC3925 option: too many options for enterprise number %d\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*opt).u.encap);
                    } else {
                        p =
                            free_space(mess, end, 125 as libc::c_int,
                                       len + 5 as libc::c_int);
                        if !p.is_null() {
                            let mut swap_ent: libc::c_int =
                                __bswap_32((*opt).u.encap as __uint32_t) as
                                    libc::c_int;
                            memcpy(p as *mut libc::c_void,
                                   &mut swap_ent as *mut libc::c_int as
                                       *const libc::c_void,
                                   4 as libc::c_int as libc::c_ulong);
                            p = p.offset(4 as libc::c_int as isize);
                            let fresh40 = p;
                            p = p.offset(1);
                            *fresh40 = len as libc::c_uchar;
                            o = config_opts;
                            while !o.is_null() {
                                if (*o).flags & 8 as libc::c_int != 0 {
                                    len =
                                        do_opt(o,
                                               p.offset(2 as libc::c_int as
                                                            isize),
                                               0 as *mut dhcp_context,
                                               0 as libc::c_int);
                                    let fresh41 = p;
                                    p = p.offset(1);
                                    *fresh41 = (*o).opt as libc::c_uchar;
                                    let fresh42 = p;
                                    p = p.offset(1);
                                    *fresh42 = len as libc::c_uchar;
                                    p = p.offset(len as isize)
                                }
                                o = (*o).next
                            }
                        }
                    }
                }
            }
        }
        opt = (*opt).next
    }
    force_encap = prune_vendor_opts(tagif);
    if !context.is_null() && pxe_arch != -(1 as libc::c_int) {
        pxe_misc(mess, end, uuid, pxevendor);
        if pxe_uefi_workaround(pxe_arch, tagif, mess, (*context).local, now,
                               0 as libc::c_int) == 0 {
            config_opts = pxe_opts(pxe_arch, tagif, (*context).local, now)
        }
    }
    if (force_encap != 0 || in_list(req_options, 43 as libc::c_int) != 0) &&
           do_encap_opts(config_opts, 43 as libc::c_int, 1024 as libc::c_int,
                         mess, end, null_term) != 0 &&
           pxe_arch == -(1 as libc::c_int) && done_vendor_class == 0 &&
           vendor_class_len != 0 as libc::c_int &&
           {
               p = free_space(mess, end, 60 as libc::c_int, vendor_class_len);
               !p.is_null()
           } {
        /* If we send vendor encapsulated options, and haven't already sent option 60,
       echo back the value we got from the client. */
        memcpy(p as *mut libc::c_void,
               (*dnsmasq_daemon).dhcp_buff3 as *const libc::c_void,
               vendor_class_len as libc::c_ulong);
    }
    /* restore BOOTP anti-overload hack */
    if req_options.is_null() ||
           (*dnsmasq_daemon).options[(30 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (30 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        (*mess).file[0 as libc::c_int as usize] = f0;
        (*mess).sname[0 as libc::c_int as usize] = s0
    };
}
unsafe extern "C" fn apply_delay(mut xid: u32_0, mut recvtime: time_t,
                                 mut netid: *mut dhcp_netid) {
    let mut delay_conf: *mut delay_config = 0 as *mut delay_config;
    /* Decide which delay_config option we're using */
    delay_conf = (*dnsmasq_daemon).delay_conf;
    while !delay_conf.is_null() {
        if match_netid((*delay_conf).netid, netid, 0 as libc::c_int) != 0 {
            break ;
        }
        delay_conf = (*delay_conf).next
    }
    if delay_conf.is_null() {
        /* No match, look for one without a netid */
        delay_conf = (*dnsmasq_daemon).delay_conf;
        while !delay_conf.is_null() {
            if match_netid((*delay_conf).netid, netid, 1 as libc::c_int) != 0
               {
                break ;
            }
            delay_conf = (*delay_conf).next
        }
    }
    if !delay_conf.is_null() {
        if (*dnsmasq_daemon).options[(42 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (42 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u reply delay: %d\x00" as *const u8 as
                          *const libc::c_char, __bswap_32(xid),
                      (*delay_conf).delay);
        }
        delay_dhcp(recvtime, (*delay_conf).delay, -(1 as libc::c_int),
                   0 as libc::c_int as uint32_t,
                   0 as libc::c_int as libc::c_ushort);
    };
}
