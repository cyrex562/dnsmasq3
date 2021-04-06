use crate::defines::{__caddr_t, C2RustUnnamed_1A, Ifmap, InPktInfo};

extern "C" {
    #[no_mangle]
    fn cache_enumerate(init: i32) -> crec;


    #[no_mangle]
    fn add_resource_record(header: &mut dns_header, limit: &mut String,
                           truncp: , nameoffset: i32,
                           pp: , ttl: u32,
                           offset: , type_0: u16,
                           class: u16, format: &mut String,
                           _: ...) -> i32;
    #[no_mangle]
    fn in_arpa_name_2_addr(namein: &mut String, addrp:  &mut all_addr)
                           -> i32;







}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area:Vec<u8>,
    pub reg_save_area:Vec<u8>,
}























#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: &mut String,
    pub _IO_read_end: &mut String,
    pub _IO_read_base: &mut String,
    pub _IO_write_base: &mut String,
    pub _IO_write_ptr: &mut String,
    pub _IO_write_end: &mut String,
    pub _IO_buf_base: &mut String,
    pub _IO_buf_end: &mut String,
    pub _IO_save_base: &mut String,
    pub _IO_backup_base: &mut String,
    pub _IO_save_end: &mut String,
    pub _markers: &mut _IO_marker,
    pub _chain: &mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock:Vec<u8>,
    pub _offset: __off64_t,
    pub _codecvt: &mut _IO_codecvt,
    pub _wide_data: &mut _IO_wide_data,
    pub _freeres_list: &mut _IO_FILE,
    pub _freeres_buf:Vec<u8>,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

































#[derive(Copy, Clone)]
#[repr(C)]
pub struct iname {
    pub name: &mut String,
    pub addr: NetAddress,
    pub used: i32,
    pub next: &mut iname,
}



































#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: &mut stat) -> i32 {
    return __xstat(1, __path, __statbuf);
}



#[inline]












#[inline]
unsafe extern "C" fn getline(mut __lineptr: String,
                             mut __n: &mut size_t, mut __stream: &mut FILE)
                             -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}

#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: &mut FILE) -> i32 {
    return (__stream._flags & 0x20 != 0)  libc::c_int;
}




#[inline]
unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                             mut __base: *const libc::c_void,
                             mut __nmemb: usize, mut __size: usize,
                             mut __compar: __compar_fn_t)
                             ->Vec<u8> {
    let mut __l: usize = 0;
    let mut __u: usize = 0;
    let mut __idx: usize = 0;
    let mut __p: *const libc::c_void = 0;
    let mut __comparison: i32 = 0;
    __l = 0 ;
    __u = __nmemb;
    while __l < __u {
        __idx =
            __l.wrapping_add(__u).wrapping_div(2   );
        __p =
            (__base          *const libc::c_char).offset(__idx.wrapping_mul(__size)          isize);
        __comparison =
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                   __p);
        if __comparison < 0 {
            __u = __idx
        } else if __comparison > 0 {
            __l = __idx.wrapping_add(1)
        } else { return __p }
    }
    return 0;
}




#[inline]
unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
                               mut endptr: &mut __gwchar_t.
                               mut base: i32) -> intmax_t {
    return __wcstol_internal(nptr, endptr, base, 0);
}



extern "C" {









    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;






    #[no_mangle]
    fn free(__ptr:Vec<u8>);







    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;

    #[no_mangle]
    fn read_write(fd: i32, packet: mut Vec<u8>,
                  size: i32, rw: i32) -> i32;
}















































#[derive(Copy, Clone)]
#[repr(C)]
pub struct listener {
    pub fd: i32,
    pub tcpfd: i32,
    pub tftpfd: i32,
    pub used: i32,
    pub addr: NetAddress,
    pub iface: &mut irec,
    pub next: Listener,
}





































#[inline]
unsafe extern "C" fn fstat64(mut __fd: i32,
                             mut __statbuf: &mut stat64) -> i32 {
    return __fxstat64(1, __fd, __statbuf);
}



extern "C" {

    pub type __dirstream;











    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;

    #[no_mangle]
    fn strncpy(_: &mut String, _: *const libc::c_char, _: u32)
               -> &mut String;



    #[no_mangle]
    fn strtok(_: &mut String, _: *const libc::c_char)
              -> &mut String;

    #[no_mangle]
    fn if_indextoname(__ifindex: u32, __ifname: &mut String)
                      -> &mut String;
    #[no_mangle]
    fn close(__fd: i32) -> i32;
    #[no_mangle]
    fn read(__fd: i32, __buf:Vec<u8>, __nbytes: usize)
            -> ssize_t;
    #[no_mangle]
    fn write(__fd: i32, __buf: *const libc::c_void, __n: usize)
             -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: ) -> i32;





    #[no_mangle]
    fn calloc(_: u32, _: u32) ->Vec<u8>;
    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: i32, _: ...)
            -> i32;
    #[no_mangle]
    fn __ctype_b_loc() -> u16;


    #[no_mangle]
    fn time(__timer: &mut time::Instant) -> time::Instant;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining:  &mut timespec)  -> i32;
    #[no_mangle]
    fn __errno_location() -> ;




    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> DIR;
    #[no_mangle]
    fn closedir(__dirp:  &mut DIR)  -> i32;
    #[no_mangle]
    fn readdir(__dirp:  &mut DIR)  -> dirent;
    #[no_mangle]
    fn dirfd(__dirp:  &mut DIR)  -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn fix_fd(fd: i32) -> i32;
    #[no_mangle]
    fn uname(__name:  &mut utsname)  -> i32;
}







































#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}






extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;




















}





















#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: i32,
                                    mut __stream: &mut FILE) -> i32 {
    return if (__stream._IO_write_ptr >= __stream._IO_write_end)  libc::c_int != 0 {
        __overflow(__stream, __c)
    } else {
        let fresh3 = __stream._IO_write_ptr;
        __stream._IO_write_ptr =
            __stream._IO_write_ptr.offset(1);
        *fresh3 = __c;
        *fresh3
    };
}




#[inline]
unsafe extern "C" fn fstatat(mut __fd: i32,
                             mut __filename: *const libc::c_char,
                             mut __statbuf: &mut stat,
                             mut __flag: i32) -> i32 {
    return __fxstatat(1, __fd, __filename, __statbuf, __flag);
}









extern "C" {


    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> FILE;


    #[no_mangle]
    fn snprintf(_: &mut String, _: u32,
                _: *const libc::c_char, _: ...) -> i32;


    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;




    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;




    #[no_mangle]
    fn strerror(_: i32) -> &mut String;



    #[no_mangle]
    fn ungetc(__c: i32, __stream:  &mut FILE)  -> i32;




    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn __ctype_b_loc() -> u16;


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn ctime(__timer: *const time::Instant) -> &mut String;
    #[no_mangle]
    fn __errno_location() -> ;




    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;

    #[no_mangle]
    fn prettyprint_addr(addr: NetAddress, buf: &mut String)
                        -> i32;

    #[no_mangle]
    fn blockdata_free(blocks:  &mut blockdata) ;
    #[no_mangle]
    fn read_write(fd: i32, packet: mut Vec<u8>,
                  size: i32, rw: i32) -> i32;
    #[no_mangle]
    fn blockdata_write(block: &mut blockdata, len: usize, fd: i32);
    #[no_mangle]
    fn blockdata_read(fd: i32, len: usize) -> blockdata;
    #[no_mangle]
    fn set_dynamic_inotify(flag: i32, total_size: i32,
                           rhash: &mutcrec, revhashsz: i32);
    #[no_mangle]
    fn canonicalise(in_0: &mut String, nomem: )
                    -> &mut String;
    #[no_mangle]
    fn get_domain6(addr:  &mut in6_addr)  -> &mut String;
    #[no_mangle]
    fn get_domain(addr: in_addr) -> &mut String;
    #[no_mangle]
    fn expand_filelist(list:  &mut hostsfile)  -> hostsfile;
    #[no_mangle]
    fn blockdata_retrieve(block: &mut blockdata, len: usize,
                          data:Vec<u8>) ->Vec<u8>;

    #[no_mangle]
    fn blockdata_report();
}


































// pub const _ISALNUM: C2RustUnnamed_1 = 8;
// pub const _ISPUNCT: C2RustUnnamed_1 = 4;
// pub const _ISCNTRL: C2RustUnnamed_1 = 2;
// pub const _ISBLANK: C2RustUnnamed_1 = 1;
// pub const _ISGRAPH: C2RustUnnamed_1 = 32768;
// pub const _ISPRINT: C2RustUnnamed_1 = 16384;
// pub const _ISSPACE: C2RustUnnamed_1 = 8192;
// pub const _ISXDIGIT: C2RustUnnamed_1 = 4096;
// pub const _ISDIGIT: C2RustUnnamed_1 = 2048;
// pub const _ISALPHA: C2RustUnnamed_1 = 1024;
// pub const _ISLOWER: C2RustUnnamed_1 = 512;
// pub const _ISUPPER: C2RustUnnamed_1 = 256;






























#[derive(Copy, Clone)]
#[repr(C)]
pub struct irec {
    pub addr: NetAddress,
    pub netmask: in_addr,
    pub tftp_ok: i32,
    pub dhcp_ok: i32,
    pub mtu: i32,
    pub done: i32,
    pub warned: i32,
    pub dad: i32,
    pub dns_auth: i32,
    pub index: i32,
    pub multicast_done: i32,
    pub found: i32,
    pub label: i32,
    pub name: &mut String,
    pub next: &mut irec,
}

































/* type->string mapping: this is also used by the name-hash function as a mixing table. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_101 {
    pub type_0: u32,
    pub name: *const libc::c_char,
}







#[inline]

#[inline]
unsafe extern "C" fn fstatat64(mut __fd: i32,
                               mut __filename: *const libc::c_char,
                               mut __statbuf: &mut stat64,
                               mut __flag: i32) -> i32 {
    return __fxstatat64(1, __fd, __filename, __statbuf,
                        __flag);
}

extern "C" {


    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn bind(__fd: i32, __addr: __CONST_NetAddress_ARG, __len: socklen_t)
            -> i32;
    #[no_mangle]
    fn sendmsg(__fd: i32, __message: *const msghdr,
               __flags: i32) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;
    #[no_mangle]
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    #[no_mangle]
    fn inet_ntoa(__in: in_addr) -> &mut String;




    #[no_mangle]
    fn ioctl(__fd: i32, __request: u32, _: ...)
             -> i32;
    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;



    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;

    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> FILE;





    #[no_mangle]
    fn fgets(__s: &mut String, __n: i32, __stream:  &mut FILE)
             -> &mut String;




    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn __ctype_b_loc() -> u16;


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> ;







    #[no_mangle]
    fn get_domain(addr: in_addr) -> &mut String;
    #[no_mangle]
    fn legal_hostname(name: &mut String) -> i32;
    #[no_mangle]
    fn canonicalise(in_0: &mut String, nomem: )
                    -> &mut String;
    #[no_mangle]
    fn safe_strncpy(dest: &mut String, src: *const libc::c_char,
                    size: usize);
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;


    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn parse_hex(in_0: &mut String, out: mut Vec<u8>,
                 maxlen: i32, wildcard_mask: &mut libc::c_uint,
                 mac_type: ) -> i32;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> i32;
    #[no_mangle]
    fn wildcard_matchn(wildcard: *const libc::c_char,
                       match_0: *const libc::c_char, num: i32)
                       -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn send_from(fd: i32, nowild: i32,
                 packet: &mut String, len: usize, to: NetAddress,
                 source: &mut all_addr, iface: u32) -> i32;
    #[no_mangle]
    fn indextoname(fd: i32, index: i32,
                   name: &mut String) -> i32;
    #[no_mangle]
    fn iface_check(family: i32, addr: &mut all_addr,
                   name: &mut String, auth: )
                   -> i32;
    #[no_mangle]
    fn fix_fd(fd: i32) -> i32;
    #[no_mangle]
    fn lease_update_dns(force: i32);
    #[no_mangle]
    fn lease_update_file(now: time::Instant);
    #[no_mangle]
    fn dhcp_reply(context: &mut dhcp_context, iface_name: &mut String,
                  int_index: i32, sz: usize, now: time::Instant,
                  unicast_dest: i32, loopback: i32,
                  is_inform: , pxe: i32,
                  fallback: in_addr, recvtime: time::Instant) -> size_t;
    #[no_mangle]
    fn lease_prune(target: &mut dhcp_lease, now: time::Instant);
    #[no_mangle]
    fn iface_enumerate(family: i32, parm:Vec<u8>,
                       callback:
                       Option<unsafe extern "C" fn() -> i32>)
                       -> i32;
    #[no_mangle]
    fn recv_dhcp_packet(fd: i32, msg:  &mut msghdr)  -> ssize_t;
    #[no_mangle]
    fn match_netid(check: &mut dhcp_netid, pool: &mut dhcp_netid,
                   tagnotneeded: i32) -> i32;
    #[no_mangle]
    fn icmp_ping(addr: in_addr) -> i32;
    #[no_mangle]
    fn lease_find_by_addr(addr: in_addr) -> dhcp_lease;
    #[no_mangle]
    fn lease_find_max_addr(context:  &mut dhcp_context)  -> in_addr;
    #[no_mangle]
    fn strip_hostname(hostname: &mut String) -> &mut String;
}







pub type __suseconds_t = i32;


pub type __ssize_t = i32;
pub type __syscall_slong_t = i32;












































pub type C2RustUnnamed_4 = libc::c_uint;
// pub const _ISALNUM: C2RustUnnamed_4 = 8;
// pub const _ISPUNCT: C2RustUnnamed_4 = 4;
// pub const _ISCNTRL: C2RustUnnamed_4 = 2;
// pub const _ISBLANK: C2RustUnnamed_4 = 1;
// pub const _ISGRAPH: C2RustUnnamed_4 = 32768;
// pub const _ISPRINT: C2RustUnnamed_4 = 16384;
// pub const _ISSPACE: C2RustUnnamed_4 = 8192;
// pub const _ISXDIGIT: C2RustUnnamed_4 = 4096;
// pub const _ISDIGIT: C2RustUnnamed_4 = 2048;
// pub const _ISALPHA: C2RustUnnamed_4 = 1024;
// pub const _ISLOWER: C2RustUnnamed_4 = 512;
// pub const _ISUPPER: C2RustUnnamed_4 = 256;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct arpreq {
    pub arp_pa: NetAddress,
    pub arp_ha: NetAddress,
    pub arp_flags: i32,
    pub arp_netmask: NetAddress,
    pub arp_dev: [libc::c_char; 16],
}








#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_102 {
    pub cache: &mut crec,
    pub name: &mut String,
}




















#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipsets {
    pub sets: String,
    pub domain: &mut String,
    pub next: IpSets,
}




































#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_136 {
    pub align: cmsghdr,
    pub control: [libc::c_char; 32],
}


extern "C" {


    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn bind(__fd: i32, __addr: __CONST_NetAddress_ARG, __len: socklen_t)
            -> i32;
    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;



    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;











    #[no_mangle]
    fn free(__ptr:Vec<u8>);


    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining:  &mut timespec)  -> i32;
    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn rand64() -> u64_0;
    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;

    #[no_mangle]
    fn addr6part(addr:  &mut in6_addr)  -> u64_0;
    #[no_mangle]
    fn setaddr6part(addr: &mut in6_addr, host: u64_0);
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> i32;
    #[no_mangle]
    fn wildcard_matchn(wildcard: *const libc::c_char,
                       match_0: *const libc::c_char, num: i32)
                       -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn indextoname(fd: i32, index: i32,
                   name: &mut String) -> i32;
    #[no_mangle]
    fn iface_check(family: i32, addr: &mut all_addr,
                   name: &mut String, auth: )
                   -> i32;
    #[no_mangle]
    fn fix_fd(fd: i32) -> i32;
    #[no_mangle]
    fn set_ipv6pktinfo(fd: i32) -> i32;
    #[no_mangle]
    fn lease_update_file(now: time::Instant);
    #[no_mangle]
    fn lease_update_dns(force: i32);
    #[no_mangle]
    fn lease6_find_by_addr(net: &mut in6_addr, prefix: i32,
                           addr: u64_0) -> dhcp_lease;
    #[no_mangle]
    fn lease_find_max_addr6(context:  &mut dhcp_context)  -> u64_0;
    #[no_mangle]
    fn lease_update_slaac(now: time::Instant);
    #[no_mangle]
    fn lease_prune(target: &mut dhcp_lease, now: time::Instant);
    #[no_mangle]
    fn send_alarm(event: time::Instant, now: time::Instant);
    #[no_mangle]
    fn iface_enumerate(family: i32, parm:Vec<u8>,
                       callback:
                       Option<unsafe extern "C" fn() -> i32>)
                       -> i32;
    #[no_mangle]
    fn save_counter(newval: i32) -> i32;
    #[no_mangle]
    fn dhcp6_reply(context: &mut dhcp_context, interface: i32,
                   iface_name: &mut String, fallback: &mut in6_addr,
                   ll_addr: &mut in6_addr, ula_addr: &mut in6_addr,
                   sz: usize, client_addr: &mut in6_addr, now: time::Instant)
                   -> u16;
    #[no_mangle]
    fn relay_upstream6(relay: &mut dhcp_relay, sz: ssize_t,
                       peer_address: &mut in6_addr, scope_id: u32_0,
                       now: time::Instant);
    #[no_mangle]
    fn relay_reply6(peer: NetAddress_in6, sz: ssize_t,
                    arrival_interface: &mut String) -> u16;
    #[no_mangle]
    fn recv_dhcp_packet(fd: i32, msg:  &mut msghdr)  -> ssize_t;
    #[no_mangle]
    fn match_netid(check: &mut dhcp_netid, pool: &mut dhcp_netid,
                   tagnotneeded: i32) -> i32;
    #[no_mangle]
    fn periodic_ra(now: time::Instant) -> time::Instant;
    #[no_mangle]
    fn log_context(family: i32, context:  &mut dhcp_context) ;
    #[no_mangle]
    fn ra_start_unsolicited(now: time::Instant, context:  &mut dhcp_context) ;
    #[no_mangle]
    fn find_mac(addr: NetAddress, mac: mut Vec<u8>,
                lazy: i32, now: time::Instant) -> i32;
}








pub type __blkcnt_t = i32;
pub type __blkcnt64_t = i32;






































#[derive(Copy, Clone)]
#[repr(C)]
pub struct neigh_packet {
    pub type_0: u8_0,
    pub code: u8_0,
    pub checksum: u16_0,
    pub reserved: u16_0,
    pub target: in6_addr,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}







#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_103 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}

















#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub addr: NetAddress,
    pub source_addr: NetAddress,
    pub interface: [libc::c_char; 17],
    pub sfd:ServerFd,
    pub domain: &mut String,
    pub flags: i32,
    pub tcpfd: i32,
    pub edns_pktsz: i32,
    pub pktsz_reduced: time::Instant,
    pub queries: u32,
    pub failed_queries: u32,
    pub uid: u32_0,
    pub next: Server,
}


extern "C" {




    #[no_mangle]
    fn snprintf(_: &mut String, _: u32,
                _: *const libc::c_char, _: ...) -> i32;


    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;




    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn strncpy(_: &mut String, _: *const libc::c_char, _: u32)
               -> &mut String;
    #[no_mangle]
    fn strncat(_: &mut String, _: *const libc::c_char, _: u32)
               -> &mut String;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
              -> &mut String;














    #[no_mangle]
    fn addr6part(addr:  &mut in6_addr)  -> u64_0;


    #[no_mangle]
    fn setaddr6part(addr: &mut in6_addr, host: u64_0);
}












































#[derive(Copy, Clone)]
#[repr(C)]
pub struct randfd {
    pub fd: i32,
    pub refcount: u16,
    pub family: u16,
}




































// #[inline]
// unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
//     return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
//         (__bsx as libc::c_int & 0xff as libc::c_int) <<
//             8 as libc::c_int) as __uint16_t; /* prefix match fail */
// }






#[inline]
unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
                           mut __statbuf: &mut stat) -> i32 {
    return __lxstat(1, __path, __statbuf);
}

#[no_mangle]
pub unsafe extern "C" fn is_name_synthetic(mut flags: i32,
                                           mut name: &mut String,
                                           mut addr: &mut all_addr)
                                           -> i32 {
    let mut p: &mut String = 0 ;
    let mut c: cond_domain = 0 ;
    let mut prot: i32 =
        if flags & (1) << 8 !=
            0 {
            10
        } else { 2 };
    let mut current_block_57: u64;
    c = dnsmasq_daemon.synth_domains;
    while !c.is_null() {
        let mut found: i32 = 0;
        let mut tail: &mut String = 0 ;
        let mut pref: &mut String = 0 ;
        tail = name;
        pref = c.prefix;
        while *tail != 0 && !pref.is_null() &&
            *pref != 0 {
            let mut c1: u32 = *pref;
            let mut c2: u32 = *tail;
            if c1 >= 'A' as i32 &&
                c1 <= 'Z' as i32 {
                c1 =
                    c1.wrapping_add(('a' as i32 - 'A' as i32))
            }
            if c2 >= 'A' as i32 &&
                c2 <= 'Z' as i32 {
                c2 =
                    c2.wrapping_add(('a' as i32 - 'A' as i32))
            }
            if c1 != c2 { break ; }
            tail = tail.offset(1);
            pref = pref.offset(1)
        }
        if !(!pref.is_null() && *pref != 0) {
            if c.indexed != 0 {
                p = tail;
                while *p != 0 {
                    let mut c_0: libc::c_char = *p;
                    if (c_0) < '0' as i32 ||
                        c_0 > '9' as i32 {
                        break ;
                    }
                    p = p.offset(1)
                }
                if *p != '.' as i32 {
                    current_block_57 = 14916268686031723178;
                } else {
                    *p = 0;
                    if hostname_isequal(c.domain,
                                        p.offset(1))
                        != 0 {
                        if prot == 2 {
                            let mut index: u32 =
                                atoi(tail);
                            if c.is6 == 0 &&
                                index <=
                                    __bswap_32(c.end.s_addr).wrapping_sub(__bswap_32(c.start.s_addr))
                            {
                                addr.addr4.s_addr =
                                    __bswap_32(__bswap_32(c.start.s_addr).wrapping_add(index));
                                found = 1
                            }
                        } else {
                            let mut index_0: u64_0 = atoll(tail) as u64_0;
                            if c.is6 != 0 &&
                                index_0 <=
                                    addr6part(&mut c.end6).wrapping_sub(addr6part(&mut c.start6))
                            {
                                let mut start: u64_0 =
                                    addr6part(&mut c.start6);
                                addr.addr6 = c.start6;
                                setaddr6part(&mut addr.addr6,
                                             start.wrapping_add(index_0));
                                found = 1
                            }
                        }
                    }
                    current_block_57 = 13853033528615664019;
                }
            } else {
                /* NB, must not alter name if we return zero */
                p = tail;
                while *p != 0 {
                    let mut c_1: libc::c_char = *p;
                    if !(c_1 >= '0' as i32 &&
                        c_1 <= '9' as i32 ||
                        c_1 == '-' as i32) {
                        if !(prot == 10 &&
                            (c_1 >= 'A' as i32 &&
                                c_1 <= 'F' as i32 ||
                                c_1 >= 'a' as i32 &&
                                    c_1 <= 'f' as i32)) {
                            break ;
                        }
                    }
                    p = p.offset(1)
                }
                if *p != '.' as i32 {
                    current_block_57 = 14916268686031723178;
                } else {
                    *p = 0;
                    if prot == 10 &&
                        strstr(tail,
                               "--ffff-"                              *const libc::c_char) == tail {
                        /* special hack for v4-mapped. */
                        memcpy(tail,
                               "::ffff:"                              *const libc::c_char,
                               7);
                        p = tail.offset(7);
                        while *p != 0 {
                            if *p == '-' as i32 {
                                *p = '.'
                            }
                            p = p.offset(1)
                        }
                    } else {
                        /* swap . or : for - */
                        p = tail;
                        while *p != 0 {
                            if *p == '-' as i32 {
                                if prot == 2 {
                                    *p = '.'
                                } else { *p = ':'  }
                            }
                            p = p.offset(1)
                        }
                    }
                    if hostname_isequal(c.domain,
                                        p.offset(1))
                        != 0 &&
                        inet_pton(prot, tail, addr) !=
                            0 {
                        if prot == 2 {
                            if c.is6 == 0 &&
                                __bswap_32(addr.addr4.s_addr) >=
                                    __bswap_32(c.start.s_addr) &&
                                __bswap_32(addr.addr4.s_addr) <=
                                    __bswap_32(c.end.s_addr) {
                                found = 1
                            }
                        } else {
                            let mut addrpart: u64_0 =
                                addr6part(&mut addr.addr6);
                            if c.is6 != 0 &&
                                is_same_net6(&mut addr.addr6,
                                             &mut c.start6,
                                             64) != 0 &&
                                addrpart >= addr6part(&mut c.start6) &&
                                addrpart <= addr6part(&mut c.end6) {
                                found = 1
                            }
                        }
                    }
                    current_block_57 = 13853033528615664019;
                }
            }
            match current_block_57 {
                14916268686031723178 => { }
                _ => {
                    /* restore name */
                    p = tail;
                    while *p != 0 {
                        if *p == '.' as i32 ||
                            *p == ':' as i32 {
                            *p = '-'
                        }
                        p = p.offset(1)
                    }
                    *p = '.' ;
                    if found != 0 { return 1 }
                }
            }
        }
        c = c.next
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_rev_synth(mut flag: i32,
                                      mut addr: &mut all_addr,
                                      mut name: &mut String)
                                      -> i32 {
    let mut c: cond_domain = 0 ;
    if flag & (1) << 7 != 0 &&
        {
            c =
                search_domain(addr.addr4,
                              dnsmasq_daemon.synth_domains);
            !c.is_null()
        } {
        let mut p: &mut String = 0 ;
        *name = 0;
        if c.indexed != 0 {
            let mut index: u32 =
                __bswap_32(addr.addr4.s_addr).wrapping_sub(__bswap_32(c.start.s_addr));
            snprintf(name, 1025,
                     "%s%u" ,
                     if !c.prefix.is_null() {
                         c.prefix
                     } else { ""  },
                     index);
        } else {
            if !c.prefix.is_null() {
                strncpy(name, c.prefix,
                        (1025 - 46) );
            }
            inet_ntop(2,
                      &mut addr.addr4  ,
                      name.offset(strlen(name)),
                      46);
            p = name;
            while *p != 0 {
                if *p == '.' as i32 {
                    *p = '-'
                }
                p = p.offset(1)
            }
        }
        strncat(name, "." ,
                1025);
        strncat(name, c.domain, 1025);
        return 1
    }
    if flag & (1) << 8 != 0 &&
        {
            c =
                search_domain6(&mut addr.addr6,
                               dnsmasq_daemon.synth_domains);
            !c.is_null()
        } {
        let mut p_0: &mut String = 0 ;
        *name = 0;
        if c.indexed != 0 {
            let mut index_0: u64_0 =
                addr6part(&mut addr.addr6).wrapping_sub(addr6part(&mut c.start6));
            snprintf(name, 1025,
                     "%s%llu" ,
                     if !c.prefix.is_null() {
                         c.prefix
                     } else { ""  },
                     index_0);
        } else {
            if !c.prefix.is_null() {
                strncpy(name, c.prefix,
                        (1025 - 46) );
            }
            inet_ntop(10,
                      &mut addr.addr6 ,
                      name.offset(strlen(name)),
                      46);
            /* IPv6 presentation address can start with ":", but valid domain names
	      cannot start with "-" so prepend a zero in that case. */
            if c.prefix.is_null() && *name == ':' as i32 {
                *name = '0' ;
                inet_ntop(10,
                          &mut addr.addr6
                          name.offset(1),
                          46);
            }
            /* V4-mapped have periods.... */
            p_0 = name;
            while *p_0 != 0 {
                if *p_0 == ':' as i32 ||
                    *p_0 == '.' as i32 {
                    *p_0 = '-'
                }
                p_0 = p_0.offset(1)
            }
        }
        strncat(name, "." ,
                1025);
        strncat(name, c.domain, 1025);
        return 1
    }
    return 0;
}


extern "C" {










    #[no_mangle]
    fn gettimeofday(__tv: &mut timeval, __tz:Vec<u8>)
                    -> i32;
    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn lseek(__fd: i32, __offset: __off64_t, __whence: i32)
             -> __off64_t;


    #[no_mangle]
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> i32;



    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: i32, _: ...)
            -> i32;


    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn read_write(fd: i32, packet: mut Vec<u8>,
                  size: i32, rw: i32) -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

}













pub type mode_t = __mode_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time::Instant,
    pub tv_usec: __suseconds_t,
}



















#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
    #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: u16,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdr {
    pub ip6_ctlun: C2RustUnnamed_1,
    pub ip6_src: in6_addr,
    pub ip6_dst: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_111 {
    pub ip6_un1: ip6_hdrctl,
    pub ip6_un2_vfc: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdrctl {
    pub ip6_un1_flow: uint32_t,
    pub ip6_un1_plen: uint16_t,
    pub ip6_un1_nxt: uint8_t,
    pub ip6_un1_hlim: uint8_t,
}

























#[derive(Copy, Clone)]
#[repr(C)]
pub struct serverfd {
    pub fd: i32,
    pub source_addr: NetAddress,
    pub interface: [libc::c_char; 17],
    pub ifindex: u32,
    pub used: u32,
    pub preallocated: u32,
    pub next:ServerFd,
}





































/* data link type */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcaprec_hdr_s {
    pub ts_sec: u32_0,
    pub ts_usec: u32_0,
    pub incl_len: u32_0,
    pub orig_len: u32_0,
}
/* https://wiki.wireshark.org/Development/LibpcapFileFormat */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcap_hdr_s {
    pub magic_number: u32_0,
    pub version_major: u16_0,
    pub version_minor: u16_0,
    pub thiszone: u32_0,
    pub sigfigs: u32_0,
    pub snaplen: u32_0,
    pub network: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udphdr {
    pub uh_sport: u16_0,
    pub uh_dport: u16_0,
    pub uh_ulen: u16_0,
    pub uh_sum: u16_0,
}







#[inline]



#[inline]
unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
                             mut __statbuf: &mut stat64) -> i32 {
    return __lxstat64(1, __path, __statbuf);
}

extern "C" {










    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memmove(_:Vec<u8>, _: *const libc::c_void, _: u32)
               ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;






    #[no_mangle]
    fn find_mac(addr: NetAddress, mac: mut Vec<u8>,
                lazy: i32, now: time::Instant) -> i32;
    #[no_mangle]
    fn rrfilter(header: &mut dns_header, plen: usize, mode: i32)
                -> size_t;







    #[no_mangle]
    fn skip_name(ansp: mut Vec<u8>, header: &mut dns_header,
                 plen: usize, extrabytes: i32) -> mut Vec<u8>;
    #[no_mangle]
    fn skip_questions(header: &mut dns_header, plen: usize)
                      -> mut Vec<u8>;
    #[no_mangle]
    fn skip_section(ansp: mut Vec<u8>, count: i32,
                    header: &mut dns_header, plen: usize)
                    -> mut Vec<u8>;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn print_mac(buff: &mut String, mac: mut Vec<u8>,
                 len: i32) -> &mut String;
    #[no_mangle]
    fn free(__ptr:Vec<u8>);
}












































#[derive(Copy, Clone)]
#[repr(C)]
pub union NetAddress {
    pub sa: NetAddress,
    pub in_0: NetAddress_in,
    pub in6: NetAddress_in6,
}






































#[derive(Copy, Clone)]
#[repr(C)]
pub struct subnet_opt {
    pub family: u16_0,
    pub source_netmask: u8_0,
    pub scope_netmask: u8_0,
    pub addr: [u8_0; 16],
}







#[inline]




#[inline]
unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
                           mut __mode: __mode_t, mut __dev: __dev_t)
                           -> i32 {
    return __xmknod(0, __path, __mode, &mut __dev);
}

extern "C" {








    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn connect(__fd: i32, __addr: __CONST_NetAddress_ARG,
               __len: socklen_t) -> i32;
    #[no_mangle]
    fn getpeername(__fd: i32, __addr: __NetAddress_ARG,
                   __len:  &mut socklen_t)  -> i32;
    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn recvfrom(__fd: i32, __buf:Vec<u8>, __n: usize,
                __flags: i32, __addr: __NetAddress_ARG,
                __addr_len:  &mut socklen_t)  -> ssize_t;
    #[no_mangle]
    fn sendmsg(__fd: i32, __message: *const msghdr,
               __flags: i32) -> ssize_t;
    #[no_mangle]
    fn recvmsg(__fd: i32, __message: &mut msghdr,
               __flags: i32) -> ssize_t;



    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;



    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn close(__fd: i32) -> i32;







    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> ;




    #[no_mangle]
    fn check_for_local_domain(name: &mut String, now: time::Instant)
                              -> i32;
    #[no_mangle]
    fn resize_packet(header: &mut dns_header, plen: usize,
                     pheader: mut Vec<u8>, hlen: usize) -> size_t;
    #[no_mangle]
    fn answer_auth(header: &mut dns_header, limit: &mut String,
                   qlen: usize, now: time::Instant, peer_addr: NetAddress,
                   local_query: i32, do_bit: i32,
                   have_pseudoheader: i32) -> size_t;
    #[no_mangle]
    fn in_zone(zone: &mut auth_zone, name: &mut String,
               cut: String) -> i32;
    #[no_mangle]
    fn check_for_bogus_wildcard(header: &mut dns_header, qlen: usize,
                                name: &mut String,
                                baddr: &mut bogus_addr, now: time::Instant)
                                -> i32;
    #[no_mangle]
    fn hash_questions(header: &mut dns_header, plen: usize,
                      name: &mut String) -> mut Vec<u8>;
    #[no_mangle]
    fn rand16() -> u16;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn sa_len(addr: NetAddress) -> i32;




    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn prettyprint_addr(addr: NetAddress, buf: &mut String)
                        -> i32;
    #[no_mangle]
    fn read_write(fd: i32, packet: mut Vec<u8>,
                  size: i32, rw: i32) -> i32;

    #[no_mangle]
    fn check_log_writer(force: i32);
    #[no_mangle]
    fn answer_request(header: &mut dns_header, limit: &mut String,
                      qlen: usize, local_addr: in_addr,
                      local_netmask: in_addr, now: time::Instant,
                      ad_reqd: i32, do_bit: i32,
                      have_pseudoheader: i32) -> size_t;
    #[no_mangle]
    fn extract_addresses(header: &mut dns_header, qlen: usize,
                         name: &mut String, now: time::Instant,
                         ipsets: String, is_sign: i32,
                         check_rebind: i32,
                         no_cache_dnssec: i32, secure: i32,
                         doctored: ) -> i32;
    #[no_mangle]
    fn setup_reply(header: &mut dns_header, qlen: usize,
                   addrp: &mut all_addr, flags: u32,
                   ttl: u32) -> size_t;
    #[no_mangle]
    fn extract_request(header: &mut dns_header, qlen: usize,
                       name: &mut String, typep:  &mut u16)
                       -> libc::c_uint;



    #[no_mangle]
    fn random_sock(family: i32) -> i32;
    #[no_mangle]
    fn check_for_ignored_address(header: &mut dns_header, qlen: usize,
                                 baddr:  &mut bogus_addr)  -> i32;
    #[no_mangle]
    fn enumerate_interfaces(reset: i32) -> i32;
    #[no_mangle]
    fn label_exception(index: i32, family: i32,
                       addr:  &mut all_addr)  -> i32;
    #[no_mangle]
    fn loopback_exception(fd: i32, family: i32,
                          addr: &mut all_addr, name: &mut String)
                          -> i32;
    #[no_mangle]
    fn iface_check(family: i32, addr: &mut all_addr,
                   name: &mut String, auth: )
                   -> i32;
    #[no_mangle]
    fn indextoname(fd: i32, index: i32,
                   name: &mut String) -> i32;
    #[no_mangle]
    fn local_bind(fd: i32, addr: NetAddress,
                  intname: &mut String, ifindex: u32,
                  is_tcp: i32) -> i32;
    #[no_mangle]
    fn detect_loop(query: &mut String, type_0: i32)
                   -> i32;
    #[no_mangle]
    fn find_pseudoheader(header: &mut dns_header, plen: usize,
                         len: &mut size_t, p: ,
                         is_sign: , is_last: )
                         -> mut Vec<u8>;
    #[no_mangle]
    fn add_do_bit(header: &mut dns_header, plen: usize,
                  limit: mut Vec<u8>) -> size_t;
    #[no_mangle]
    fn add_edns0_config(header: &mut dns_header, plen: usize,
                        limit: mut Vec<u8>, source: NetAddress,
                        now: time::Instant, check_subnet: ,
                        cacheable: ) -> size_t;

    #[no_mangle]
    fn rrfilter(header: &mut dns_header, plen: usize, mode: i32)
                -> size_t;
    #[no_mangle]
    fn check_source(header: &mut dns_header, plen: usize,
                    pseudoheader: mut Vec<u8>, peer: NetAddress)
                    -> i32;
    #[no_mangle]
    fn dump_packet(mask: i32, packet:Vec<u8>, len: usize,
                   src: NetAddress, dst: NetAddress);
}





pub type __off_t = i32;
pub type __off64_t = i32;

















pub type C2RustUnnamed999 = libc::c_uint;

















// pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
// pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
// pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
// pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
// pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
// pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
// pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
// pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
// pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
// pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
// pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
// pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
// pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
// pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
// pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
// pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
// pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
// pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
// pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
// pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
// pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
// pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
// pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
// pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
// pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
// pub const IPPROTO_IP: C2RustUnnamed_1 = 0;

// pub type C2rustUnnamed2 = libc::c_uint;
// pub const __METRIC_MAX: C2rustUnnamed2 = 20;
// pub const METRIC_LEASES_PRUNED_6: C2rustUnnamed2 = 19;
// pub const METRIC_LEASES_ALLOCATED_6: C2rustUnnamed2 = 18;
// pub const METRIC_LEASES_PRUNED_4: C2rustUnnamed2 = 17;
// pub const METRIC_LEASES_ALLOCATED_4: C2rustUnnamed2 = 16;
// pub const METRIC_NOANSWER: C2rustUnnamed2 = 15;
// pub const METRIC_DHCPREQUEST: C2rustUnnamed2 = 14;
// pub const METRIC_DHCPRELEASE: C2rustUnnamed2 = 13;
// pub const METRIC_DHCPOFFER: C2rustUnnamed2 = 12;
// pub const METRIC_DHCPNAK: C2rustUnnamed2 = 11;
// pub const METRIC_DHCPINFORM: C2rustUnnamed2 = 10;
// pub const METRIC_DHCPDISCOVER: C2rustUnnamed2 = 9;
// pub const METRIC_DHCPDECLINE: C2rustUnnamed2 = 8;
// pub const METRIC_DHCPACK: C2rustUnnamed2 = 7;
// pub const METRIC_PXE: C2rustUnnamed2 = 6;
// pub const METRIC_BOOTP: C2rustUnnamed2 = 5;
// pub const METRIC_DNS_LOCAL_ANSWERED: C2rustUnnamed2 = 4;
// pub const METRIC_DNS_AUTH_ANSWERED: C2rustUnnamed2 = 3;
// pub const METRIC_DNS_QUERIES_FORWARDED: C2rustUnnamed2 = 2;
// pub const METRIC_DNS_CACHE_LIVE_FREED: C2rustUnnamed2 = 1;
// pub const METRIC_DNS_CACHE_INSERTED: C2rustUnnamed2 = 0;







// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct ifreq {
//     pub ifr_ifrn: C2RustUnnamed_4,
//     pub ifr_ifru: C2RustUnnamed_3,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub ifru_addr: NetAddress,
    pub ifru_dstaddr: NetAddress,
    pub ifru_broadaddr: NetAddress,
    pub ifru_netmask: NetAddress,
    pub ifru_hwaddr: NetAddress,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: i32,
    pub ifru_mtu: i32,
    pub ifru_map: Ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_42 {
    pub ifrn_name: [libc::c_char; 16],
}



#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr2 {
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
pub struct C2RustUnnamed_51 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_61 {
    pub target: &mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_71 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_81 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_91 {
    pub target: C2RustUnnamed_10,
    pub uid: u32,
    pub is_name_ptr: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_104 {
    pub cache: &mut crec,
    pub name: &mut String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec2 {
    pub next: &mut crec,
    pub prev: &mut crec,
    pub hash_next: &mut crec,
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_113 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}













#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface_name {
    pub name: &mut String,
    pub intr: &mut String,
    pub family: i32,
    pub addr: &mut addrlist,
    pub next: &mut interface_name,
}



















#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt2 {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_12,
    pub val: mut Vec<u8>,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_121 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: mut Vec<u8>,
}


















#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_131 {
    pub align: cmsghdr,
    pub control: [libc::c_char; 32],
    pub control6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_141 {
    pub c: mut Vec<u8>,
    pub p: &mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_151 {
    pub c: mut Vec<u8>,
    pub p: &mut InPktInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_161 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
    pub control: [libc::c_char; 32],
}



#[inline]
unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                            mut __statbuf: &mut stat64) -> i32 {
    return __xstat64(1, __path, __statbuf);
}

extern "C" {









    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;












    #[no_mangle]
    fn extract_name(header: &mut dns_header, plen: usize,
                    pp: , name: &mut String,
                    isExtract: i32, extrabytes: i32)
                    -> i32;
}


















extern "C" {


    #[no_mangle]
    fn fdopen(__fd: i32, __modes: *const libc::c_char) -> FILE;




    #[no_mangle]
    fn fgets(__s: &mut String, __n: i32, __stream:  &mut FILE)
             -> &mut String;

    #[no_mangle]
    fn inet_ntoa(__in: in_addr) -> &mut String;




    #[no_mangle]
    fn sigemptyset(__set:  &mut sigset_t)  -> i32;
    #[no_mangle]
    fn sigaction(__sig: i32, __act: *const sigaction,
                 __oact:  &mut sigaction)  -> i32;
    #[no_mangle]
    fn wait(__stat_loc: ) -> __pid_t;
    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memmove(_:Vec<u8>, _: *const libc::c_void, _: u32)
               ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;

    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: i32) -> &mut String;

    #[no_mangle]
    fn close(__fd: i32) -> i32;
    #[no_mangle]
    fn write(__fd: i32, __buf: *const libc::c_void, __n: usize)
             -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: ) -> i32;
    #[no_mangle]
    fn sleep(__seconds: u32) -> libc::c_uint;
    #[no_mangle]
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...)
             -> i32;
    #[no_mangle]
    fn _exit(_: i32) -> !;
    #[no_mangle]
    fn setuid(__uid: __uid_t) -> i32;
    #[no_mangle]
    fn setgid(__gid: __gid_t) -> i32;
    #[no_mangle]
    fn fork() -> __pid_t;



    #[no_mangle]
    fn send_event(fd: i32, event: i32, data: i32,
                  msg: &mut String);
    #[no_mangle]
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> ;
    #[no_mangle]
    fn setgroups(__n: usize, __groups: *const __gid_t) -> i32;





    #[no_mangle]
    fn legal_hostname(name: &mut String) -> i32;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn read_write(fd: i32, packet: mut Vec<u8>,
                  size: i32, rw: i32) -> i32;
    #[no_mangle]
    fn close_fds(max_fd: i32, spare1: i32,
                 spare2: i32, spare3: i32);
    #[no_mangle]
    fn indextoname(fd: i32, index: i32,
                   name: &mut String) -> i32;
    #[no_mangle]
    fn fix_fd(fd: i32) -> i32;


    #[no_mangle]
    fn unsetenv(__name: *const libc::c_char) -> i32;
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: i32) -> i32;
    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn malloc(_: u32) ->Vec<u8>;
}




pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;

pub type __pid_t = libc::c_int;
pub type __clock_t = i32;
pub type __time::Instant = i32;





pub type gid_t = __gid_t;

pub type off_t = __off64_t;
pub type pid_t = __pid_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;













#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr:Vec<u8>,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_01 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_199 {
    pub _call_addr:Vec<u8>,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_299 {
    pub si_band: i32,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_399 {
    pub si_addr:Vec<u8>,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_43 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_599 {
    pub _lower:Vec<u8>,
    pub _upper:Vec<u8>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_699 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_799 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_899 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_998 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_105 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: i32,
                                                  _: &mut siginfo_t,
                                                  _:Vec<u8>)
                                                  -> ()>,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr3 {
    pub addr4: in_addr,
    pub addr6: in6_addr,
    pub cname: C2RustUnnamed_15,
    pub key: C2RustUnnamed_14,
    pub ds: C2RustUnnamed_13,
    pub srv: C2RustUnnamed_12,
    pub log: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1199 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1299 {
    pub target: &mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_132 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1499 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_159 {
    pub target: C2RustUnnamed_16,
    pub uid: u32,
    pub is_name_ptr: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub cache: &mut crec,
    pub name: &mut String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec3 {
    pub next: &mut crec,
    pub prev: &mut crec,
    pub hash_next: &mut crec,
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}












#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_list {
    pub name: &mut String,
    pub next: &mut name_list,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt3 {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_18,
    pub val: mut Vec<u8>,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: mut Vec<u8>,
}


















#[derive(Copy, Clone)]
#[repr(C)]
pub struct script_data {
    pub flags: i32,
    pub action: i32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub clid_len: i32,
    pub hostname_len: i32,
    pub ed_len: i32,
    pub addr: in_addr,
    pub giaddr: in_addr,
    pub remaining_time: u32,
    pub expires: time::Instant,
    pub file_len: off_t,
    pub addr6: in6_addr,
    pub vendorclass_count: i32,
    pub iaid: u32,
    pub hwaddr: [libc::c_uchar; 16],
    pub interface: [libc::c_char; 16],
}

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
    return strtol(__nptr, 0 ,
                  10);
}




extern "C" {

    pub type __dirstream;












    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: i32) -> &mut String;

    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn read(__fd: i32, __buf:Vec<u8>, __nbytes: usize)
            -> ssize_t;
    #[no_mangle]
    fn readlink(__path: *const libc::c_char, __buf: &mut String,
                __len: usize) -> ssize_t;





    #[no_mangle]
    fn dhcp_update_configs(configs:  &mut dhcp_config) ;
    #[no_mangle]
    fn lease_update_from_configs();


    #[no_mangle]
    fn __errno_location() -> ;




    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> DIR;
    #[no_mangle]
    fn closedir(__dirp:  &mut DIR)  -> i32;
    #[no_mangle]
    fn readdir(__dirp:  &mut DIR)  -> dirent;

    #[no_mangle]
    fn read_hostsfile(filename: &mut String, index: u32,
                      cache_size: i32, rhash: &mut crec,
                      hashsz: i32) -> i32;
    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn option_read_dynfile(file: &mut String, flags: i32)
                           -> i32;
    #[no_mangle]
    fn lease_update_file(now: time::Instant);
    #[no_mangle]
    fn lease_update_dns(force: i32);
    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn inotify_init1(__flags: i32) -> i32;
    #[no_mangle]
    fn inotify_add_watch(__fd: i32, __name: *const libc::c_char,
                         __mask: uint32_t) -> i32;
}

















#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: u16,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;





















#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_record {
    pub ttl: i32,
    pub flags: i32,
    pub names: &mut name_list,
    pub addr: in_addr,
    pub addr6: in6_addr,
    pub next: &mut host_record,
}









































pub const IN_CLOEXEC: C2RustUnnamed_8 = 524288;
pub const IN_NONBLOCK: C2RustUnnamed_8 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: i32,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
}
pub type C2RustUnnamed_89 = libc::c_uint;




#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> i32 {
    return strtol(__nptr, 0 ,
                  10);
}



extern "C" {








    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn bind(__fd: i32, __addr: __CONST_NetAddress_ARG, __len: socklen_t)
            -> i32;
    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn getsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval:Vec<u8>,
                  __optlen:  &mut socklen_t)  -> i32;
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;



    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;


    #[no_mangle]
    fn strerror(_: i32) -> &mut String;







    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

}



























pub type __u16 = u16;
pub type __u32 = libc::c_uint;
pub type __be16 = __u16;





#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}




#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_79 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_name_list {
    pub name: &mut String,
    pub flags: i32,
    pub next: &mut auth_name_list,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt4 {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_8,
    pub val: mut Vec<u8>,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_opt,
}



















#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetAddress_nl {
    pub nl_family: __kernel_sa_family_t,
    pub nl_pad: u16,
    pub nl_pid: __u32,
    pub nl_groups: __u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_nlattr {
    pub nla_len: __u16,
    pub nla_type: __u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_nfgenmsg {
    pub nfgen_family: __u8,
    pub version: __u8,
    pub res_id: __be16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_set_req_adt {
    pub op: u32,
    pub index: uint16_t,
    pub ip: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_99 {
    pub name: [libc::c_char; 32],
    pub index: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_set_req_adt_get {
    pub op: u32,
    pub version: u32,
    pub set: C2RustUnnamed_9,
    pub typename: [libc::c_char; 32],
}







#[inline]









#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
                           -> libc::c_longlong {
    return strtoll(__nptr, 0 ,
                   10);
}










extern "C" {

    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;




    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;




    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn fsync(__fd: i32) -> i32;
    #[no_mangle]
    fn ftruncate(__fd: i32, __length: __off64_t) -> i32;


    #[no_mangle]
    fn fflush(__stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> FILE;


    #[no_mangle]
    fn fscanf(_: &mut FILE, _: *const libc::c_char, _: ...) -> i32;





    #[no_mangle]
    fn rewind(__stream:  &mut FILE) ;
    #[no_mangle]
    fn ferror(__stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn fileno(__stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char)
             -> FILE;



    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: String,
               _: i32) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr:Vec<u8>);


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn cache_add_dhcp_entry(host_name: &mut String, prot: i32,
                            host_address: &mut all_addr, ttd: time::Instant);
    #[no_mangle]
    fn cache_unhash_dhcp();
    #[no_mangle]
    fn pclose(__stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn get_domain(addr: in_addr) -> &mut String;
    #[no_mangle]
    fn get_domain6(addr:  &mut in6_addr)  -> &mut String;
    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;

    #[no_mangle]
    fn netmask_length(mask: in_addr) -> i32;


    #[no_mangle]
    fn addr6part(addr:  &mut in6_addr)  -> u64_0;
    #[no_mangle]
    fn parse_hex(in_0: &mut String, out: mut Vec<u8>,
                 maxlen: i32, wildcard_mask: &mut libc::c_uint,
                 mac_type: ) -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn host_from_dns(addr: in_addr) -> &mut String;
    #[no_mangle]
    fn send_alarm(event: time::Instant, now: time::Instant);
    #[no_mangle]
    fn periodic_ra(now: time::Instant) -> time::Instant;
    #[no_mangle]
    fn periodic_slaac(now: time::Instant, leases_0:  &mut dhcp_lease)  -> time::Instant;
    #[no_mangle]
    fn slaac_add_addrs(lease: &mut dhcp_lease, now: time::Instant,
                       force: i32);
    #[no_mangle]
    fn slaac_ping_reply(sender: &mut in6_addr, packet: mut Vec<u8>,
                        interface: &mut String,
                        leases_0:  &mut dhcp_lease) ;
    #[no_mangle]
    fn make_duid(now: time::Instant);
    #[no_mangle]
    fn find_config(configs: &mut dhcp_config, context: &mut dhcp_context,
                   clid: mut Vec<u8>, clid_len: i32,
                   hwaddr: mut Vec<u8>, hw_len: i32,
                   hw_type: i32, hostname: &mut String,
                   filter:  &mut dhcp_netid)  -> dhcp_config;
    #[no_mangle]
    fn queue_script(action: i32, lease: &mut dhcp_lease,
                    hostname: &mut String, now: time::Instant);
    #[no_mangle]
    fn iface_enumerate(family: i32, parm:Vec<u8>,
                       callback:
                       Option<unsafe extern "C" fn() -> i32>)
                       -> i32;
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr9 {
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
pub struct C2RustUnnamed_198 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_298 {
    pub target: &mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockdata {
    pub next: &mut blockdata,
    pub key: [libc::c_uchar; 40],
}




#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec99 {
    pub next: &mut crec,
    pub prev: &mut crec,
    pub hash_next: &mut crec,
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_7,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_zone {
    pub domain: &mut String,
    pub interface_names: &mut auth_name_list,
    pub subnet: &mut addrlist,
    pub exclude: &mut addrlist,
    pub next: &mut auth_zone,
}
























#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_config {
    pub flags: u32,
    pub clid_len: i32,
    pub clid: mut Vec<u8>,
    pub hostname: &mut String,
    pub domain: &mut String,
    pub netid: &mut dhcp_netid_list,
    pub filter: &mut dhcp_netid,
    pub addr6: &mut addrlist,
    pub addr: in_addr,
    pub decline_time: time::Instant,
    pub lease_time: u32,
    pub hwaddr: &mut hwaddr_config,
    pub next: &mut dhcp_config,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_interface {
    pub name: &mut String,
    pub mtu_name: &mut String,
    pub interval: i32,
    pub lifetime: i32,
    pub prio: i32,
    pub mtu: i32,
    pub next: &mut ra_interface,
}

extern "C" {




    #[no_mangle]
    static mut stderr: FILE;
    #[no_mangle]
    fn fprintf(_: &mut FILE, _: *const libc::c_char, _: ...) -> i32;


    #[no_mangle]
    fn vsnprintf(_: &mut String, _: u32,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
                 -> i32;

    #[no_mangle]
    fn fputc(__c: i32, __stream:  &mut FILE)  -> i32;


    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn connect(__fd: i32, __addr: __CONST_NetAddress_ARG,
               __len: socklen_t) -> i32;




    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn close(__fd: i32) -> i32;
    #[no_mangle]
    fn write(__fd: i32, __buf: *const libc::c_void, __n: usize)
             -> ssize_t;
    #[no_mangle]
    fn fchown(__fd: i32, __owner: __uid_t, __group: __gid_t)
              -> i32;
    #[no_mangle]
    fn dup(__fd: i32) -> i32;
    #[no_mangle]
    fn _exit(_: i32) -> !;
    #[no_mangle]
    fn getpid() -> __pid_t;





    #[no_mangle]
    fn malloc(_: u32) ->Vec<u8>;
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: i32, _: ...)
            -> i32;


    #[no_mangle]
    fn time(__timer: &mut time::Instant) -> time::Instant;
    #[no_mangle]
    fn ctime(__timer: *const time::Instant) -> &mut String;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining:  &mut timespec)  -> i32;
    #[no_mangle]
    fn __errno_location() -> ;




    #[no_mangle]
    fn openlog(__ident: *const libc::c_char, __option: i32,
               __facility: i32);
    #[no_mangle]
    fn vsyslog(__pri: i32, __fmt: *const libc::c_char,
               __ap: ::std::ffi::VaList);

    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn safe_strncpy(dest: &mut String, src: *const libc::c_char,
                    size: usize);
    #[no_mangle]
    fn send_event(fd: i32, event: i32, data: i32,
                  msg: &mut String);
    #[no_mangle]
    fn poll_listen(fd: i32, event: libc::c_short);
    #[no_mangle]
    fn poll_check(fd: i32, event: libc::c_short) -> i32;
}




























#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: &mut String,
    pub pw_passwd: &mut String,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: &mut String,
    pub pw_dir: &mut String,
    pub pw_shell: &mut String,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_197 {
    pub target: &mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_398 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
}












#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrlist {
    pub addr: all_addr,
    pub flags: i32,
    pub prefixlen: i32,
    pub decline_time: time::Instant,
    pub next: &mut addrlist,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwaddr_config {
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [libc::c_uchar; 16],
    pub wildcard_mask: u32,
    pub next: &mut hwaddr_config,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_domain {
    pub domain: &mut String,
    pub prefix: &mut String,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub is6: i32,
    pub indexed: i32,
    pub next: &mut cond_domain,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_entry {
    pub offset: i32,
    pub length: i32,
    pub pid: pid_t,
    pub next: &mut log_entry,
    pub payload: [libc::c_char; 1024],
}

extern "C" {








    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;




    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
              -> &mut String;







    #[no_mangle]
    fn __ctype_b_loc() -> u16;







    #[no_mangle]
    fn rand16() -> u16;
    #[no_mangle]
    fn sa_len(addr: NetAddress) -> i32;
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn allocate_rfd(family: i32) -> randfd;
    #[no_mangle]
    fn free_rfd(rfd:  &mut randfd) ;
    #[no_mangle]
    fn check_servers();
}























//pub const _ISPUNCT: C2RustUnnamed_0 = 4;








#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_698 {
    pub cache: &mut crec,
    pub name: &mut String,
}









#[derive(Copy, Clone)]
#[repr(C)]
pub struct cname {
    pub ttl: i32,
    pub flag: i32,
    pub alias: &mut String,
    pub target: &mut String,
    pub next: &mut cname,
    pub targetp: &mut cname,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct delay_config {
    pub delay: i32,
    pub netid: &mut dhcp_netid,
    pub next: &mut delay_config,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_bridge {
    pub iface: [libc::c_char; 16],
    pub alias: &mut dhcp_bridge,
    pub next: &mut dhcp_bridge,
}



#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: i32) -> i32 {
    return if (stdout._IO_write_ptr >= stdout._IO_write_end)  libc::c_int != 0 {
        __overflow(stdout, __c)
    } else {
        let fresh5 = stdout._IO_write_ptr;
        stdout._IO_write_ptr = stdout._IO_write_ptr.offset(1);
        *fresh5 = __c;
        *fresh5
    };
}



extern "C" {
    pub type NetAddress_x25;
    pub type NetAddress_ns;
    pub type NetAddress_iso;
    pub type NetAddress_ipx;
    pub type NetAddress_inarp;
    pub type NetAddress_eon;
    pub type NetAddress_dl;
    pub type NetAddress_ax25;
    pub type NetAddress_at;



    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn bind(__fd: i32, __addr: __CONST_NetAddress_ARG, __len: socklen_t)
            -> i32;
    #[no_mangle]
    fn getsockname(__fd: i32, __addr: __NetAddress_ARG,
                   __len:  &mut socklen_t)  -> i32;
    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn recvmsg(__fd: i32, __message: &mut msghdr,
               __flags: i32) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;



    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn sleep(__seconds: u32) -> libc::c_uint;









    #[no_mangle]
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;


    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn expand_buf(iov_0: &mut iovec, size: usize) -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn queue_event(event: i32);
}








pub type C2RustUnnamed997 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;






#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr2 {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_02{
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}









pub type __u8 = libc::c_uchar;

pub type __s32 = libc::c_int;






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptr_record {
    pub name: &mut String,
    pub ptr: &mut String,
    pub next: &mut ptr_record,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag_if {
    pub set: &mut dhcp_netid_list,
    pub tag: &mut dhcp_netid,
    pub next: &mut tag_if,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_mac {
    pub mask: u32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [libc::c_uchar; 16],
    pub netid: dhcp_netid,
    pub next: &mut dhcp_mac,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnsmasq_daemon {
    pub options: [libc::c_uint; 2],
    pub default_resolv: resolvc,
    pub resolv_files: &mut resolvc,
    pub last_resolv: time::Instant,
    pub servers_file: &mut String,
    pub mxnames: &mut mx_srv_record,
    pub naptr: &mut naptr,
    pub txt: &mut txt_record,
    pub rr: &mut txt_record,
    pub ptr: &mut ptr_record,
    pub host_records: &mut host_record,
    pub host_records_tail: &mut host_record,
    pub cnames: &mut cname,
    pub auth_zones: &mut auth_zone,
    pub int_names: &mut interface_name,
    pub mxtarget: &mut String,
    pub add_subnet4: &mut mysubnet,
    pub add_subnet6: &mut mysubnet,
    pub lease_file: &mut String,
    pub username: &mut String,
    pub groupname: &mut String,
    pub scriptuser: &mut String,
    pub luascript: &mut String,
    pub authserver: &mut String,
    pub hostmaster: &mut String,
    pub authinterface: &mut iname,
    pub secondary_forward_server: &mut name_list,
    pub group_set: i32,
    pub osport: i32,
    pub domain_suffix: &mut String,
    pub cond_domain: &mut cond_domain,
    pub synth_domains: &mut cond_domain,
    pub runfile: &mut String,
    pub lease_change_command: &mut String,
    pub if_names: &mut iname,
    pub if_addrs: &mut iname,
    pub if_except: &mut iname,
    pub dhcp_except: &mut iname,
    pub auth_peers: &mut iname,
    pub tftp_interfaces: &mut iname,
    pub bogus_addr: &mut bogus_addr,
    pub ignore_addr: &mut bogus_addr,
    pub servers: Server,
    pub ipsets: IpSets,
    pub log_fac: i32,
    pub log_file: &mut String,
    pub max_logs: i32,
    pub cachesize: i32,
    pub ftabsize: i32,
    pub port: i32,
    pub query_port: i32,
    pub min_port: i32,
    pub max_port: i32,
    pub local_ttl: u32,
    pub neg_ttl: u32,
    pub max_ttl: u32,
    pub min_cache_ttl: u32,
    pub max_cache_ttl: u32,
    pub auth_ttl: u32,
    pub dhcp_ttl: u32,
    pub use_dhcp_ttl: u32,
    pub dns_client_id: &mut String,
    pub addn_hosts: &mut hostsfile,
    pub dhcp: &mut dhcp_context,
    pub dhcp6: &mut dhcp_context,
    pub ra_interfaces: &mut ra_interface,
    pub dhcp_conf: &mut dhcp_config,
    pub dhcp_opts: &mut dhcp_opt,
    pub dhcp_match: &mut dhcp_opt,
    pub dhcp_opts6: &mut dhcp_opt,
    pub dhcp_match6: &mut dhcp_opt,
    pub dhcp_name_match: &mut dhcp_match_name,
    pub dhcp_pxe_vendors: &mut dhcp_pxe_vendor,
    pub dhcp_vendors: &mut dhcp_vendor,
    pub dhcp_macs: &mut dhcp_mac,
    pub boot_config: &mut dhcp_boot,
    pub pxe_services: &mut pxe_service,
    pub tag_if: &mut tag_if,
    pub override_relays: &mut addr_list,
    pub relay4: &mut dhcp_relay,
    pub relay6: &mut dhcp_relay,
    pub delay_conf: &mut delay_config,
    pub override_0: i32,
    pub enable_pxe: i32,
    pub doing_ra: i32,
    pub doing_dhcp6: i32,
    pub dhcp_ignore: &mut dhcp_netid_list,
    pub dhcp_ignore_names: &mut dhcp_netid_list,
    pub dhcp_gen_names: &mut dhcp_netid_list,
    pub force_broadcast: &mut dhcp_netid_list,
    pub bootp_dynamic: &mut dhcp_netid_list,
    pub dhcp_hosts_file: &mut hostsfile,
    pub dhcp_opts_file: &mut hostsfile,
    pub dynamic_dirs: &mut hostsfile,
    pub dhcp_max: i32,
    pub tftp_max: i32,
    pub tftp_mtu: i32,
    pub dhcp_server_port: i32,
    pub dhcp_client_port: i32,
    pub start_tftp_port: i32,
    pub end_tftp_port: i32,
    pub min_leasetime: u32,
    pub doctors: &mut doctor,
    pub edns_pktsz: u16,
    pub tftp_prefix: &mut String,
    pub if_prefix: &mut tftp_prefix,
    pub duid_enterprise: u32,
    pub duid_config_len: u32,
    pub duid_config: mut Vec<u8>,
    pub dbus_name: &mut String,
    pub ubus_name: &mut String,
    pub dump_file: &mut String,
    pub dump_mask: i32,
    pub soa_sn: u32,
    pub soa_refresh: u32,
    pub soa_retry: u32,
    pub soa_expiry: u32,
    pub metrics: [u32_0; 20],
    pub packet: &mut String,
    pub packet_buff_sz: i32,
    pub namebuff: &mut String,
    pub frec_list: &mut frec,
    pub free_frec_src: &mut frec_src,
    pub frec_src_count: i32,
    pub sfds:ServerFd,
    pub interfaces: &mut irec,
    pub listeners: Listener,
    pub last_server: Server,
    pub forwardtime: time::Instant,
    pub forwardcount: i32,
    pub srv_save: Server,
    pub packet_len: usize,
    pub rfd_save: &mut randfd,
    pub tcp_pids: [pid_t; 20],
    pub tcp_pipes: [libc::c_int; 20],
    pub pipe_to_parent: i32,
    pub randomsocks: [randfd; 64],
    pub v6pktinfo: i32,
    pub interface_addrs: &mut addrlist,
    pub log_id: i32,
    pub log_display_id: i32,
    pub log_source_addr: NetAddress,
    pub dhcpfd: i32,
    pub helperfd: i32,
    pub pxefd: i32,
    pub inotifyfd: i32,
    pub netlinkfd: i32,
    pub kernel_version: i32,
    pub dhcp_packet: iovec,
    pub dhcp_buff: &mut String,
    pub dhcp_buff2: &mut String,
    pub dhcp_buff3: &mut String,
    pub ping_results: &mut ping_result,
    pub lease_stream: &mut FILE,
    pub bridges: &mut dhcp_bridge,
    pub shared_networks: &mut shared_network,
    pub duid_len: i32,
    pub duid: mut Vec<u8>,
    pub outpacket: iovec,
    pub dhcp6fd: i32,
    pub icmp6fd: i32,
    pub dbus:Vec<u8>,
    pub tftp_trans: &mut tftp_transfer,
    pub tftp_done_trans: &mut tftp_transfer,
    pub addrbuff: &mut String,
    pub addrbuff2: &mut String,
    pub dumpfd: i32,
}

pub type __kernel_sa_family_t = u16;

pub const RTM_DELADDR: C2RustUnnamed_14 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_14 = 20;
pub const RT_TABLE_LOCAL: rt_class_t = 255;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtmsg {
    pub rtm_family: libc::c_uchar,
    pub rtm_dst_len: libc::c_uchar,
    pub rtm_src_len: libc::c_uchar,
    pub rtm_tos: libc::c_uchar,
    pub rtm_table: libc::c_uchar,
    pub rtm_protocol: libc::c_uchar,
    pub rtm_scope: libc::c_uchar,
    pub rtm_type: libc::c_uchar,
    pub rtm_flags: u32,
}
pub const RT_TABLE_MAIN: rt_class_t = 254;
pub const RT_SCOPE_LINK: rt_scope_t = 253;
pub const RTN_UNICAST: C2RustUnnamed_15 = 1;
pub const RTM_NEWROUTE: C2RustUnnamed_14 = 24;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsgerr {
    pub error: i32,
    pub msg: nlmsghdr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifinfomsg {
    pub ifi_family: libc::c_uchar,
    pub __ifi_pad: libc::c_uchar,
    pub ifi_type: u16,
    pub ifi_index: i32,
    pub ifi_flags: u32,
    pub ifi_change: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}
pub const IFLA_ADDRESS: C2RustUnnamed_11 = 1;
pub const RTM_NEWLINK: C2RustUnnamed_14 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndmsg {
    pub ndm_family: __u8,
    pub ndm_pad1: __u8,
    pub ndm_pad2: __u16,
    pub ndm_ifindex: __s32,
    pub ndm_state: __u16,
    pub ndm_flags: __u8,
    pub ndm_type: __u8,
}
pub const NDA_LLADDR: C2RustUnnamed_13 = 2;
pub const NDA_DST: C2RustUnnamed_13 = 1;
pub const RTM_NEWNEIGH: C2RustUnnamed_14 = 28;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrmsg {
    pub ifa_family: __u8,
    pub ifa_prefixlen: __u8,
    pub ifa_flags: __u8,
    pub ifa_scope: __u8,
    pub ifa_index: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifa_cacheinfo {
    pub ifa_prefered: __u32,
    pub ifa_valid: __u32,
    pub cstamp: __u32,
    pub tstamp: __u32,
}
pub const IFA_CACHEINFO: C2RustUnnamed_12 = 6;
pub const IFA_ADDRESS: C2RustUnnamed_12 = 1;
pub const IFA_LABEL: C2RustUnnamed_12 = 3;
pub const IFA_BROADCAST: C2RustUnnamed_12 = 4;
pub const IFA_LOCAL: C2RustUnnamed_12 = 2;
pub const RTM_GETADDR: C2RustUnnamed_14 = 22;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub nlh: nlmsghdr,
    pub g: rtgenmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtgenmsg {
    pub rtgen_family: libc::c_uchar,
}
pub const RTM_GETLINK: C2RustUnnamed_14 = 18;
pub const RTM_GETNEIGH: C2RustUnnamed_14 = 30;
pub type C2RustUnnamed_119 = libc::c_uint;
pub const __IFLA_MAX: C2RustUnnamed_11 = 52;
pub const IFLA_MAX_MTU: C2RustUnnamed_11 = 51;
pub const IFLA_MIN_MTU: C2RustUnnamed_11 = 50;
pub const IFLA_NEW_IFINDEX: C2RustUnnamed_11 = 49;
pub const IFLA_CARRIER_DOWN_COUNT: C2RustUnnamed_11 = 48;
pub const IFLA_CARRIER_UP_COUNT: C2RustUnnamed_11 = 47;
pub const IFLA_TARGET_NETNSID: C2RustUnnamed_11 = 46;
pub const IFLA_IF_NETNSID: C2RustUnnamed_11 = 46;
pub const IFLA_NEW_NETNSID: C2RustUnnamed_11 = 45;
pub const IFLA_EVENT: C2RustUnnamed_11 = 44;
pub const IFLA_XDP: C2RustUnnamed_11 = 43;
pub const IFLA_PAD: C2RustUnnamed_11 = 42;
pub const IFLA_GSO_MAX_SIZE: C2RustUnnamed_11 = 41;
pub const IFLA_GSO_MAX_SEGS: C2RustUnnamed_11 = 40;
pub const IFLA_PROTO_DOWN: C2RustUnnamed_11 = 39;
pub const IFLA_PHYS_PORT_NAME: C2RustUnnamed_11 = 38;
pub const IFLA_LINK_NETNSID: C2RustUnnamed_11 = 37;
pub const IFLA_PHYS_SWITCH_ID: C2RustUnnamed_11 = 36;
pub const IFLA_CARRIER_CHANGES: C2RustUnnamed_11 = 35;
pub const IFLA_PHYS_PORT_ID: C2RustUnnamed_11 = 34;
pub const IFLA_CARRIER: C2RustUnnamed_11 = 33;
pub const IFLA_NUM_RX_QUEUES: C2RustUnnamed_11 = 32;
pub const IFLA_NUM_TX_QUEUES: C2RustUnnamed_11 = 31;
pub const IFLA_PROMISCUITY: C2RustUnnamed_11 = 30;
pub const IFLA_EXT_MASK: C2RustUnnamed_11 = 29;
pub const IFLA_NET_NS_FD: C2RustUnnamed_11 = 28;
pub const IFLA_GROUP: C2RustUnnamed_11 = 27;
pub const IFLA_AF_SPEC: C2RustUnnamed_11 = 26;
pub const IFLA_PORT_SELF: C2RustUnnamed_11 = 25;
pub const IFLA_VF_PORTS: C2RustUnnamed_11 = 24;
pub const IFLA_STATS64: C2RustUnnamed_11 = 23;
pub const IFLA_VFINFO_LIST: C2RustUnnamed_11 = 22;
pub const IFLA_NUM_VF: C2RustUnnamed_11 = 21;
pub const IFLA_IFALIAS: C2RustUnnamed_11 = 20;
pub const IFLA_NET_NS_PID: C2RustUnnamed_11 = 19;
pub const IFLA_LINKINFO: C2RustUnnamed_11 = 18;
pub const IFLA_LINKMODE: C2RustUnnamed_11 = 17;
pub const IFLA_OPERSTATE: C2RustUnnamed_11 = 16;
pub const IFLA_WEIGHT: C2RustUnnamed_11 = 15;
pub const IFLA_MAP: C2RustUnnamed_11 = 14;
pub const IFLA_TXQLEN: C2RustUnnamed_11 = 13;
pub const IFLA_PROTINFO: C2RustUnnamed_11 = 12;
pub const IFLA_WIRELESS: C2RustUnnamed_11 = 11;
pub const IFLA_MASTER: C2RustUnnamed_11 = 10;
pub const IFLA_PRIORITY: C2RustUnnamed_11 = 9;
pub const IFLA_COST: C2RustUnnamed_11 = 8;
pub const IFLA_STATS: C2RustUnnamed_11 = 7;
pub const IFLA_QDISC: C2RustUnnamed_11 = 6;
pub const IFLA_LINK: C2RustUnnamed_11 = 5;
pub const IFLA_MTU: C2RustUnnamed_11 = 4;
pub const IFLA_IFNAME: C2RustUnnamed_11 = 3;
pub const IFLA_BROADCAST: C2RustUnnamed_11 = 2;
pub const IFLA_UNSPEC: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_129 = libc::c_uint;
pub const __IFA_MAX: C2RustUnnamed_12 = 11;
pub const IFA_TARGET_NETNSID: C2RustUnnamed_12 = 10;
pub const IFA_RT_PRIORITY: C2RustUnnamed_12 = 9;
pub const IFA_FLAGS: C2RustUnnamed_12 = 8;
pub const IFA_MULTICAST: C2RustUnnamed_12 = 7;
pub const IFA_ANYCAST: C2RustUnnamed_12 = 5;
pub const IFA_UNSPEC: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_133 = libc::c_uint;
pub const __NDA_MAX: C2RustUnnamed_13 = 13;
pub const NDA_PROTOCOL: C2RustUnnamed_13 = 12;
pub const NDA_SRC_VNI: C2RustUnnamed_13 = 11;
pub const NDA_LINK_NETNSID: C2RustUnnamed_13 = 10;
pub const NDA_MASTER: C2RustUnnamed_13 = 9;
pub const NDA_IFINDEX: C2RustUnnamed_13 = 8;
pub const NDA_VNI: C2RustUnnamed_13 = 7;
pub const NDA_PORT: C2RustUnnamed_13 = 6;
pub const NDA_VLAN: C2RustUnnamed_13 = 5;
pub const NDA_PROBES: C2RustUnnamed_13 = 4;
pub const NDA_CACHEINFO: C2RustUnnamed_13 = 3;
pub const NDA_UNSPEC: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_149 = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed_14 = 107;
pub const RTM_GETNEXTHOP: C2RustUnnamed_14 = 106;
pub const RTM_DELNEXTHOP: C2RustUnnamed_14 = 105;
pub const RTM_NEWNEXTHOP: C2RustUnnamed_14 = 104;
pub const RTM_GETCHAIN: C2RustUnnamed_14 = 102;
pub const RTM_DELCHAIN: C2RustUnnamed_14 = 101;
pub const RTM_NEWCHAIN: C2RustUnnamed_14 = 100;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_14 = 96;
pub const RTM_GETSTATS: C2RustUnnamed_14 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_14 = 92;
pub const RTM_GETNSID: C2RustUnnamed_14 = 90;
pub const RTM_DELNSID: C2RustUnnamed_14 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_14 = 88;
pub const RTM_GETMDB: C2RustUnnamed_14 = 86;
pub const RTM_DELMDB: C2RustUnnamed_14 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_14 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_14 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_14 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_14 = 80;
pub const RTM_SETDCB: C2RustUnnamed_14 = 79;
pub const RTM_GETDCB: C2RustUnnamed_14 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_14 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_14 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_14 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_14 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_14 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_14 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_14 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_14 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_14 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_14 = 52;
pub const RTM_GETACTION: C2RustUnnamed_14 = 50;
pub const RTM_DELACTION: C2RustUnnamed_14 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_14 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_14 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_14 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_14 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_14 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_14 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_14 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_14 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_14 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_14 = 36;
pub const RTM_GETRULE: C2RustUnnamed_14 = 34;
pub const RTM_DELRULE: C2RustUnnamed_14 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_14 = 32;
pub const RTM_DELNEIGH: C2RustUnnamed_14 = 29;
pub const RTM_GETROUTE: C2RustUnnamed_14 = 26;
pub const RTM_DELROUTE: C2RustUnnamed_14 = 25;
pub const RTM_SETLINK: C2RustUnnamed_14 = 19;
pub const RTM_DELLINK: C2RustUnnamed_14 = 17;
pub const RTM_BASE: C2RustUnnamed_14 = 16;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const __RTN_MAX: C2RustUnnamed_15 = 12;
pub const RTN_XRESOLVE: C2RustUnnamed_15 = 11;
pub const RTN_NAT: C2RustUnnamed_15 = 10;
pub const RTN_THROW: C2RustUnnamed_15 = 9;
pub const RTN_PROHIBIT: C2RustUnnamed_15 = 8;
pub const RTN_UNREACHABLE: C2RustUnnamed_15 = 7;
pub const RTN_BLACKHOLE: C2RustUnnamed_15 = 6;
pub const RTN_MULTICAST: C2RustUnnamed_15 = 5;
pub const RTN_ANYCAST: C2RustUnnamed_15 = 4;
pub const RTN_BROADCAST: C2RustUnnamed_15 = 3;
pub const RTN_LOCAL: C2RustUnnamed_15 = 2;
pub const RTN_UNSPEC: C2RustUnnamed_15 = 0;
pub type rt_scope_t = libc::c_uint;
pub const RT_SCOPE_NOWHERE: rt_scope_t = 255;
pub const RT_SCOPE_HOST: rt_scope_t = 254;
pub const RT_SCOPE_SITE: rt_scope_t = 200;
pub const RT_SCOPE_UNIVERSE: rt_scope_t = 0;
pub type rt_class_t = libc::c_uint;
pub const RT_TABLE_MAX: rt_class_t = 4294967295;
pub const RT_TABLE_DEFAULT: rt_class_t = 253;
pub const RT_TABLE_COMPAT: rt_class_t = 252;
pub const RT_TABLE_UNSPEC: rt_class_t = 0;
#[inline]
unsafe extern "C" fn __bswap_116(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx >> 8 & 0xff |
        (__bsx & 0xff) <<
            8) ; /* autobind */
}







#[inline]





#[inline]
unsafe extern "C" fn mknodat(mut __fd: i32,
                             mut __path: *const libc::c_char,
                             mut __mode: __mode_t, mut __dev: __dev_t)
                             -> i32 {
    return __xmknodat(0, __fd, __path, __mode, &mut __dev);
}

extern "C" {




    #[no_mangle]
    fn fclose(__stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> FILE;




    #[no_mangle]
    fn fgets(__s: &mut String, __n: i32, __stream:  &mut FILE)
             -> &mut String;

    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn bind(__fd: i32, __addr: __CONST_NetAddress_ARG, __len: socklen_t)
            -> i32;
    #[no_mangle]
    fn getsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval:Vec<u8>,
                  __optlen:  &mut socklen_t)  -> i32;
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;
    #[no_mangle]
    fn listen(__fd: i32, __n: i32) -> i32;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;




    #[no_mangle]
    fn ioctl(__fd: i32, __request: u32, _: ...)
             -> i32;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;



    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: u32) -> i32;

    #[no_mangle]
    fn strtok(_: &mut String, _: *const libc::c_char)
              -> &mut String;

    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn close(__fd: i32) -> i32;





    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;


    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn private_net(addr: in_addr, ban_localhost: i32) -> i32;
    #[no_mangle]
    fn rand16() -> u16;
    #[no_mangle]
    fn rand32() -> u32_0;
    #[no_mangle]
    fn safe_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn safe_strncpy(dest: &mut String, src: *const libc::c_char,
                    size: usize);
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn sa_len(addr: NetAddress) -> i32;


    #[no_mangle]
    fn prettyprint_addr(addr: NetAddress, buf: &mut String)
                        -> i32;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn server_gone(server: Server);
    #[no_mangle]
    fn loop_send_probes();
    #[no_mangle]
    fn iface_enumerate(family: i32, parm:Vec<u8>,
                       callback:
                       Option<unsafe extern "C" fn() -> i32>)
                       -> i32;
    #[no_mangle]
    fn lease_find_interfaces(now: time::Instant);
    #[no_mangle]
    fn dhcp_construct_contexts(now: time::Instant);
}



pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;








pub type dev_t = __dev_t;


























#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: u32,
}





pub const IFF_DYNAMIC: C2RustUnnamed_1 = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed_1 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_1 = 8192;
pub const IFF_MULTICAST: C2RustUnnamed_1 = 4096;
pub const IFF_SLAVE: C2RustUnnamed_1 = 2048;
pub const IFF_MASTER: C2RustUnnamed_1 = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed_1 = 512;
pub const IFF_PROMISC: C2RustUnnamed_1 = 256;
pub const IFF_NOARP: C2RustUnnamed_1 = 128;
pub const IFF_RUNNING: C2RustUnnamed_1 = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed_1 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_1 = 16;
pub const IFF_LOOPBACK: C2RustUnnamed_1 = 8;
pub const IFF_DEBUG: C2RustUnnamed_1 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_1 = 2;
pub const IFF_UP: C2RustUnnamed_1 = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq2 {
    pub ifr_ifrn: C2RustUnnamed_3,
    pub ifr_ifru: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_297 {
    pub ifru_addr: NetAddress,
    pub ifru_dstaddr: NetAddress,
    pub ifru_broadaddr: NetAddress,
    pub ifru_netmask: NetAddress,
    pub ifru_hwaddr: NetAddress,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: i32,
    pub ifru_mtu: i32,
    pub ifru_map: Ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_397 {
    pub ifrn_name: [libc::c_char; 16],
}




#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}







#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_106 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}





#[derive(Copy, Clone)]
#[repr(C)]
pub struct txt_record {
    pub name: &mut String,
    pub txt: mut Vec<u8>,
    pub class: u16,
    pub len: u16,
    pub stat: i32,
    pub next: &mut txt_record,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_netid_list {
    pub list: &mut dhcp_netid,
    pub next: &mut dhcp_netid_list,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_pxe_vendor {
    pub data: &mut String,
    pub next: &mut dhcp_pxe_vendor,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_relay {
    pub local: all_addr,
    pub server: all_addr,
    pub interface: &mut String,
    pub iface_index: i32,
    pub current: &mut dhcp_relay,
    pub next: &mut dhcp_relay,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct iface_param {
    pub spare: &mut addrlist,
    pub fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_128 {
    pub c: mut Vec<u8>,
    pub p: &mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_134 {
    pub c: mut Vec<u8>,
    pub p: &mut InPktInfo,
}





#[inline]







#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}

extern "C" {





    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;


















    #[no_mangle]
    fn expand_buf(iov: &mut iovec, size: usize) -> i32;
}


















#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr99 {
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
pub struct C2RustUnnamed_099 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_296 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub target: C2RustUnnamed_5,
    pub uid: u32,
    pub is_name_ptr: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_598 {
    pub cache: &mut crec,
    pub name: &mut String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec98 {
    pub next: &mut crec,
    pub prev: &mut crec,
    pub hash_next: &mut crec,
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_697 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}




#[derive(Copy, Clone)]
#[repr(C)]
pub struct naptr {
    pub name: &mut String,
    pub replace: &mut String,
    pub regexp: &mut String,
    pub services: &mut String,
    pub flags: &mut String,
    pub order: u32,
    pub pref: u32,
    pub next: &mut naptr,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_netid {
    pub net: &mut String,
    pub next: &mut dhcp_netid,
}





#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt99 {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_7,
    pub val: mut Vec<u8>,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_78 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: mut Vec<u8>,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_vendor {
    pub len: i32,
    pub match_type: i32,
    pub enterprise: u32,
    pub data: &mut String,
    pub netid: dhcp_netid,
    pub next: &mut dhcp_vendor,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_prefix {
    pub interface: &mut String,
    pub prefix: &mut String,
    pub missing: i32,
    pub next: &mut tftp_prefix,
}









#[inline]











#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: &mut FILE) -> i32 {
    return (__stream._flags & 0x10 != 0)  libc::c_int;
}

#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128) && __c < 256 {
        *(*__ctype_tolower_loc()).offset(__c)
    } else { __c };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128) && __c < 256 {
        *(*__ctype_toupper_loc()).offset(__c)
    } else { __c };
}

#[inline]
unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
                               mut endptr: String,
                               mut base: i32) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0);
}
#[inline]
unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
                               mut endptr: String,
                               mut base: i32) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0);
}




extern "C" {




    #[no_mangle]
    fn poll(__fds: &mut pollfd, __nfds: nfds_t, __timeout: i32)
            -> i32;
    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memmove(_:Vec<u8>, _: *const libc::c_void, _: u32)
               ->Vec<u8>;











    #[no_mangle]
    fn free(__ptr:Vec<u8>);






    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
}


pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}








#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return putc(__c, stdout);
}








#[inline]






#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: i32,
                                   mut __stream: &mut FILE) -> i32 {
    return if (__stream._IO_write_ptr >= __stream._IO_write_end)  libc::c_int != 0 {
        __overflow(__stream, __c)
    } else {
        let fresh4 = __stream._IO_write_ptr;
        __stream._IO_write_ptr =
            __stream._IO_write_ptr.offset(1);
        *fresh4 = __c;
        *fresh4
    };
}








extern "C" {



    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> FILE;




    #[no_mangle]
    fn fgets(__s: &mut String, __n: i32, __stream:  &mut FILE)
             -> &mut String;

    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;




    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn if_indextoname(__ifindex: u32, __ifname: &mut String)
                      -> &mut String;





    #[no_mangle]
    fn free(__ptr:Vec<u8>);


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;





    #[no_mangle]
    fn rand16() -> u16;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;

    #[no_mangle]
    fn addr6part(addr:  &mut in6_addr)  -> u64_0;
    #[no_mangle]
    fn setaddr6part(addr: &mut in6_addr, host: u64_0);
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> i32;
    #[no_mangle]
    fn expand_buf(iov: &mut iovec, size: usize) -> i32;
    #[no_mangle]
    fn print_mac(buff: &mut String, mac: mut Vec<u8>,
                 len: i32) -> &mut String;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> i32;
    #[no_mangle]
    fn wildcard_matchn(wildcard: *const libc::c_char,
                       match_0: *const libc::c_char, num: i32)
                       -> i32;
    #[no_mangle]
    fn die(message: &mut String, arg1: &mut String,
           exit_code: i32) -> !;

    #[no_mangle]
    fn indextoname(fd: i32, index: i32,
                   name: &mut String) -> i32;
    #[no_mangle]
    fn iface_check(family: i32, addr: &mut all_addr,
                   name: &mut String, auth: )
                   -> i32;
    #[no_mangle]
    fn fix_fd(fd: i32) -> i32;
    #[no_mangle]
    fn set_ipv6pktinfo(fd: i32) -> i32;
    #[no_mangle]
    fn lease_ping_reply(sender: &mut in6_addr, packet: mut Vec<u8>,
                        interface: &mut String);
    #[no_mangle]
    fn iface_enumerate(family: i32, parm:Vec<u8>,
                       callback:
                       Option<unsafe extern "C" fn() -> i32>)
                       -> i32;
    #[no_mangle]
    fn recv_dhcp_packet(fd: i32, msg:  &mut msghdr)  -> ssize_t;
    #[no_mangle]
    fn option_filter(tags: &mut dhcp_netid, context_tags: &mut dhcp_netid,
                     opts:  &mut dhcp_opt)  -> dhcp_netid;
    #[no_mangle]
    fn reset_counter();
    #[no_mangle]
    fn save_counter(newval: i32) -> i32;
    #[no_mangle]
    fn expand(headroom: usize) ->Vec<u8>;
    #[no_mangle]
    fn put_opt6(data:Vec<u8>, len: usize) ->Vec<u8>;
    #[no_mangle]
    fn put_opt6_long(val: u32);
    #[no_mangle]
    fn put_opt6_short(val: u32);
    #[no_mangle]
    fn put_opt6_char(val: u32);
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;
    #[no_mangle]
    fn getsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval:Vec<u8>,
                  __optlen:  &mut socklen_t)  -> i32;
    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;
}
























// pub const IPPROTO_MH: C2RustUnnamed_1 = 135;
// pub const IPPROTO_DSTOPTS: C2RustUnnamed_1 = 60;
// pub const IPPROTO_NONE: C2RustUnnamed_1 = 59;
// pub const IPPROTO_ICMPV6: C2RustUnnamed_1 = 58;
// pub const IPPROTO_FRAGMENT: C2RustUnnamed_1 = 44;
// pub const IPPROTO_ROUTING: C2RustUnnamed_1 = 43;
// pub const IPPROTO_HOPOPTS: C2RustUnnamed_1 = 0;






#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_packet {
    pub type_0: u8_0,
    pub code: u8_0,
    pub checksum: u16_0,
    pub hop_limit: u8_0,
    pub flags: u8_0,
    pub lifetime: u16_0,
    pub reachable_time: u32_0,
    pub retrans_time: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prefix_opt {
    pub type_0: u8_0,
    pub len: u8_0,
    pub prefix_len: u8_0,
    pub flags: u8_0,
    pub valid_lifetime: u32_0,
    pub preferred_lifetime: u32_0,
    pub reserved: u32_0,
    pub prefix: in6_addr,
}


















#[derive(Copy, Clone)]
#[repr(C)]
pub struct mx_srv_record {
    pub name: &mut String,
    pub target: &mut String,
    pub issrv: i32,
    pub srvport: i32,
    pub priority: i32,
    pub weight: i32,
    pub offset: u32,
    pub next: &mut mx_srv_record,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct frec_src {
    pub source: NetAddress,
    pub dest: all_addr,
    pub iface: u32,
    pub log_id: u32,
    pub fd: i32,
    pub orig_id: u16,
    pub next: &mut frec_src,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct pxe_service {
    pub CSA: u16,
    pub type_0: u16,
    pub menu: &mut String,
    pub basename: &mut String,
    pub sname: &mut String,
    pub server: in_addr,
    pub netid: &mut dhcp_netid,
    pub next: &mut pxe_service,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr_list {
    pub addr: in_addr,
    pub next: &mut addr_list,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_filter {
    pub icmp6_filt: [uint32_t; 8],
}

extern "C" {










    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memmove(_:Vec<u8>, _: *const libc::c_void, _: u32)
               ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;









    #[no_mangle]
    fn __ctype_b_loc() -> u16;







    #[no_mangle]
    fn next_uid(crecp:  &mut crec) ;
    // #[no_mangle]
    // fn log_query(flags: u32, name: &mut String,
    //              addr: &mut all_addr, arg: &mut String);
    #[no_mangle]
    fn record_source(index: u32) -> &mut String;
    #[no_mangle]
    fn querystr(desc: &mut String, type_0: u16)
                -> &mut String;
    #[no_mangle]
    fn cache_find_non_terminal(name: &mut String, now: time::Instant)
                               -> i32;
    #[no_mangle]
    fn cache_find_by_addr(crecp: &mut crec, addr: &mut all_addr, now: time::Instant,
                          prot: u32) -> crec;
    #[no_mangle]
    fn cache_find_by_name(crecp: &mut crec, name: &mut String,
                          now: time::Instant, prot: u32) -> crec;
    #[no_mangle]
    fn cache_end_insert();
    #[no_mangle]
    fn cache_start_insert();
    #[no_mangle]
    fn cache_insert(name: &mut String, addr: &mut all_addr,
                    class: u16, now: time::Instant, ttl: u32,
                    flags: u32) -> crec;
    #[no_mangle]
    fn cache_make_stat(t:  &mut txt_record)  -> i32;
    #[no_mangle]
    fn cache_get_name(crecp:  &mut crec)  -> &mut String;
    #[no_mangle]
    fn cache_get_cname_target(crecp:  &mut crec)  -> &mut String;
    #[no_mangle]
    fn blockdata_alloc(data: &mut String, len: usize)
                       -> blockdata;
    #[no_mangle]
    fn blockdata_retrieve(block: &mut blockdata, len: usize,
                          data:Vec<u8>) ->Vec<u8>;
    #[no_mangle]
    fn is_name_synthetic(flags: i32, name: &mut String,
                         addr:  &mut all_addr)  -> i32;
    #[no_mangle]
    fn is_rev_synth(flag: i32, addr: &mut all_addr,
                    name: &mut String) -> i32;
    #[no_mangle]
    fn do_rfc1035_name(p: mut Vec<u8>, sval: &mut String,
                       limit: &mut String) -> mut Vec<u8>;

    #[no_mangle]
    fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> i32;
    #[no_mangle]
    fn add_to_ipset(setname: *const libc::c_char, ipaddr: *const all_addr,
                    flags: i32, remove: i32) -> i32;

    #[no_mangle]
    fn add_pseudoheader(header: &mut dns_header, plen: usize,
                        limit: mut Vec<u8>, udp_sz: u16,
                        optno: i32, opt: mut Vec<u8>,
                        optlen: usize, set_do: i32,
                        replace: i32) -> size_t;
    #[no_mangle]
    fn enumerate_interfaces(reset: i32) -> i32;
    #[no_mangle]
    fn hostname_issubdomain(a: &mut String, b: &mut String)
                            -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];










pub type __socklen_t = libc::c_uint;
pub type ino_t = __ino64_t;




















pub type va_list = __builtin_va_list;



// pub const _ISALNUM: C2RustUnnamed_0 = 8;
//
// pub const _ISCNTRL: C2RustUnnamed_0 = 2;
// pub const _ISBLANK: C2RustUnnamed_0 = 1;
// pub const _ISGRAPH: C2RustUnnamed_0 = 32768;
// pub const _ISPRINT: C2RustUnnamed_0 = 16384;
// pub const _ISSPACE: C2RustUnnamed_0 = 8192;
// pub const _ISXDIGIT: C2RustUnnamed_0 = 4096;
// pub const _ISDIGIT: C2RustUnnamed_0 = 2048;
// pub const _ISALPHA: C2RustUnnamed_0 = 1024;
// pub const _ISLOWER: C2RustUnnamed_0 = 512;
// pub const _ISUPPER: C2RustUnnamed_0 = 256;





#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_596 {
    pub target: C2RustUnnamed_6,
    pub uid: u32,
    pub is_name_ptr: i32,
}





#[derive(Copy, Clone)]
#[repr(C)]
pub struct doctor {
    pub in_0: in_addr,
    pub end: in_addr,
    pub out: in_addr,
    pub mask: in_addr,
    pub next: &mut doctor,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct frec {
    pub frec_src: frec_src,
    pub sentto: Server,
    pub rfd4: &mut randfd,
    pub rfd6: &mut randfd,
    pub new_id: u16,
    pub forwardall: i32,
    pub flags: i32,
    pub time: time::Instant,
    pub hash: [mut Vec<u8>; 32],
    pub next: &mut frec,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_match_name {
    pub name: &mut String,
    pub wildcard: i32,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_match_name,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_transfer {
    pub sockfd: i32,
    pub timeout: time::Instant,
    pub backoff: i32,
    pub block: u32,
    pub blocksize: u32,
    pub expansion: u32,
    pub offset: off_t,
    pub peer: NetAddress,
    pub source: all_addr,
    pub if_index: i32,
    pub opt_blocksize: libc::c_char,
    pub opt_transize: libc::c_char,
    pub netascii: libc::c_char,
    pub carrylf: libc::c_char,
    pub file: &mut tftp_file,
    pub next: &mut tftp_transfer,
}









#[inline]








#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}




fn a_record_from_hosts(name: &mut String, now: time::Instant) -> in_addr;
#[no_mangle]
fn get_domain(addr: in_addr) -> &mut String;
#[no_mangle]
fn rand16() -> u16;
#[no_mangle]
fn legal_hostname(name: &mut String) -> i32;
#[no_mangle]
fn do_rfc1035_name(p: mut Vec<u8>, sval: &mut String,
                   limit: &mut String) -> mut Vec<u8>;
#[no_mangle]
fn safe_strncpy(dest: &mut String, src: *const libc::c_char,
                size: usize);
#[no_mangle]
fn whine_malloc(size: usize) ->Vec<u8>;
#[no_mangle]
fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
                    -> i32;
#[no_mangle]
fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> i32;
#[no_mangle]
fn prettyprint_time(buf: &mut String, t: u32);
#[no_mangle]
fn memcmp_masked(a: mut Vec<u8>, b: mut Vec<u8>,
                 len: i32, mask: u32) -> i32;
#[no_mangle]
fn expand_buf(iov: &mut iovec, size: usize) -> i32;
#[no_mangle]
fn print_mac(buff: &mut String, mac: mut Vec<u8>,
             len: i32) -> &mut String;
#[no_mangle]
fn my_syslog(priority: i32, format: *const libc::c_char, _: ...);
#[no_mangle]
fn option_string(prot: i32, opt: u32,
                 val: mut Vec<u8>, opt_len: i32,
                 buf: &mut String, buf_len: i32)
                 -> &mut String;
#[no_mangle]
fn enumerate_interfaces(reset: i32) -> i32;
#[no_mangle]
fn address_available(context: &mut dhcp_context, taddr: in_addr,
                     netids:  &mut dhcp_netid)  -> dhcp_context;
#[no_mangle]
fn narrow_context(context: &mut dhcp_context, taddr: in_addr,
                  netids:  &mut dhcp_netid)  -> dhcp_context;
#[no_mangle]
fn do_icmp_ping(now: time::Instant, addr: in_addr, hash: u32,
                loopback: i32) -> ping_result;
#[no_mangle]
fn address_allocate(context: &mut dhcp_context, addrp: &mut in_addr,
                    hwaddr: mut Vec<u8>, hw_len: i32,
                    netids: &mut dhcp_netid, now: time::Instant,
                    loopback: i32) -> i32;
#[no_mangle]
fn config_find_by_address(configs: &mut dhcp_config, addr: in_addr)
                          -> dhcp_config;
#[no_mangle]
fn host_from_dns(addr: in_addr) -> &mut String;
#[no_mangle]
fn lease4_allocate(addr: in_addr) -> dhcp_lease;
#[no_mangle]
fn lease_set_hwaddr(lease: &mut dhcp_lease, hwaddr: *const libc::c_uchar,
                    clid: *const libc::c_uchar, hw_len: i32,
                    hw_type: i32, clid_len: i32,
                    now: time::Instant, force: i32);
#[no_mangle]
fn lease_set_hostname(lease: &mut dhcp_lease, name: *const libc::c_char,
                      auth: i32, domain: &mut String,
                      config_domain: &mut String);
#[no_mangle]
fn lease_set_expires(lease: &mut dhcp_lease, len: u32,
                     now: time::Instant);
#[no_mangle]
fn lease_set_interface(lease: &mut dhcp_lease, interface: i32,
                       now: time::Instant);
#[no_mangle]
fn lease_find_by_client(hwaddr: mut Vec<u8>, hw_len: i32,
                        hw_type: i32, clid: mut Vec<u8>,
                        clid_len: i32) -> dhcp_lease;
#[no_mangle]
fn lease_find_by_addr(addr: in_addr) -> dhcp_lease;
#[no_mangle]
fn lease_prune(target: &mut dhcp_lease, now: time::Instant);
#[no_mangle]
fn lease_add_extradata(lease: &mut dhcp_lease, data: mut Vec<u8>,
                       len: u32, delim: i32);
#[no_mangle]
fn match_netid(check: &mut dhcp_netid, pool: &mut dhcp_netid,
               tagnotneeded: i32) -> i32;
#[no_mangle]
fn option_filter(tags: &mut dhcp_netid, context_tags: &mut dhcp_netid,
                 opts:  &mut dhcp_opt)  -> dhcp_netid;
#[no_mangle]
fn log_tags(netid: &mut dhcp_netid, xid: u32_0);
#[no_mangle]
fn run_tag_if(tags:  &mut dhcp_netid)  -> dhcp_netid;
#[no_mangle]
fn config_has_mac(config: &mut dhcp_config, hwaddr: mut Vec<u8>,
                  len: i32, type_0: i32) -> i32;
#[no_mangle]
fn delay_dhcp(start: time::Instant, sec: i32, fd: i32,
              addr: uint32_t, id: u16) -> i32;
#[no_mangle]
fn strtod(_: *const libc::c_char, _: String)
          -> libc::c_double;
#[no_mangle]
fn find_config(configs: &mut dhcp_config, context: &mut dhcp_context,
               clid: mut Vec<u8>, clid_len: i32,
               hwaddr: mut Vec<u8>, hw_len: i32,
               hw_type: i32, hostname: &mut String,
               filter:  &mut dhcp_netid)  -> dhcp_config;
#[no_mangle]
fn strip_hostname(hostname: &mut String) -> &mut String;
#[no_mangle]
fn match_bytes(o: &mut dhcp_opt, p: mut Vec<u8>, len: i32)
               -> i32;
#[no_mangle]
fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
#[no_mangle]
fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
           _: u32) -> i32;
#[no_mangle]
fn strchr(_: *const libc::c_char, _: i32) -> &mut String;
#[no_mangle]
fn strlen(_: *const libc::c_char) -> libc::c_ulong;





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




pub type C2RustUnnamed_196 = libc::c_uint;


#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr95 {
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
pub struct C2RustUnnamed_295 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_395 {
    pub target: &mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_595 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_695 {
    pub target: C2RustUnnamed_7,
    pub uid: u32,
    pub is_name_ptr: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_795 {
    pub cache: &mut crec,
    pub name: &mut String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec95 {
    pub next: &mut crec,
    pub prev: &mut crec,
    pub hash_next: &mut crec,
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_895 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bogus_addr {
    pub addr: in_addr,
    pub next: &mut bogus_addr,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostsfile {
    pub next: &mut hostsfile,
    pub flags: i32,
    pub fname: &mut String,
    pub wd: i32,
    pub index: u32,
}










#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt96 {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_9,
    pub val: mut Vec<u8>,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_94 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: mut Vec<u8>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_boot {
    pub file: &mut String,
    pub sname: &mut String,
    pub tftp_sname: &mut String,
    pub next_server: in_addr,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_boot,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_file {
    pub refcount: i32,
    pub fd: i32,
    pub size: off_t,
    pub dev: dev_t,
    pub inode: ino_t,
    pub filename: [libc::c_char; 0],
}




#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 );
}

extern "C" {

    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;
    #[no_mangle]
    fn inet_pton(__af: i32, __cp: *const libc::c_char,
                 __buf:Vec<u8>) -> i32;




    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u32) -> i32;
    #[no_mangle]
    fn strcat(_: &mut String, _: *const libc::c_char)
              -> &mut String;

    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: i32) -> &mut String;

    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;




    #[no_mangle]
    fn snprintf(_: &mut String, _: u32,
                _: *const libc::c_char, _: ...) -> i32;





    #[no_mangle]
    fn reset_counter();
    #[no_mangle]
    fn match_bytes(o: &mut dhcp_opt, p: mut Vec<u8>, len: i32)
                   -> i32;
    #[no_mangle]
    fn strip_hostname(hostname: &mut String) -> &mut String;
    #[no_mangle]
    fn find_config(configs: &mut dhcp_config, context: &mut dhcp_context,
                   clid: mut Vec<u8>, clid_len: i32,
                   hwaddr: mut Vec<u8>, hw_len: i32,
                   hw_type: i32, hostname: &mut String,
                   filter:  &mut dhcp_netid)  -> dhcp_config;
    #[no_mangle]
    fn put_opt6_char(val: u32);
    #[no_mangle]
    fn run_tag_if(tags:  &mut dhcp_netid)  -> dhcp_netid;
    #[no_mangle]
    fn match_netid(check: &mut dhcp_netid, pool: &mut dhcp_netid,
                   tagnotneeded: i32) -> i32;
    #[no_mangle]
    fn expand(headroom: usize) ->Vec<u8>;
    #[no_mangle]
    fn option_filter(tags: &mut dhcp_netid, context_tags: &mut dhcp_netid,
                     opts:  &mut dhcp_opt)  -> dhcp_netid;
    #[no_mangle]
    fn put_opt6_long(val: u32);
    #[no_mangle]
    fn put_opt6_short(val: u32);
    #[no_mangle]
    fn put_opt6_string(s: &mut String);
    #[no_mangle]
    fn log_tags(netid: &mut dhcp_netid, xid: u32_0);
    #[no_mangle]
    fn save_counter(newval: i32) -> i32;
    #[no_mangle]
    fn put_opt6(data:Vec<u8>, len: usize) ->Vec<u8>;
    #[no_mangle]
    fn end_opt6(container: i32);
    #[no_mangle]
    fn new_opt6(opt: i32) -> i32;


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;





    #[no_mangle]
    fn get_domain6(addr:  &mut in6_addr)  -> &mut String;
    #[no_mangle]
    fn rand16() -> u16;
    #[no_mangle]
    fn legal_hostname(name: &mut String) -> i32;
    #[no_mangle]
    fn do_rfc1035_name(p: mut Vec<u8>, sval: &mut String,
                       limit: &mut String) -> mut Vec<u8>;
    #[no_mangle]
    fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
                        -> i32;
    #[no_mangle]
    fn is_same_net6(a: &mut in6_addr, b: &mut in6_addr,
                    prefixlen: i32) -> i32;
    #[no_mangle]
    fn addr6part(addr:  &mut in6_addr)  -> u64_0;
    #[no_mangle]
    fn setaddr6part(addr: &mut in6_addr, host: u64_0);
    #[no_mangle]
    fn prettyprint_time(buf: &mut String, t: u32);
    #[no_mangle]
    fn memcmp_masked(a: mut Vec<u8>, b: mut Vec<u8>,
                     len: i32, mask: u32) -> i32;
    #[no_mangle]
    fn print_mac(buff: &mut String, mac: mut Vec<u8>,
                 len: i32) -> &mut String;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> i32;

    #[no_mangle]
    fn option_string(prot: i32, opt: u32,
                     val: mut Vec<u8>, opt_len: i32,
                     buf: &mut String, buf_len: i32)
                     -> &mut String;
    #[no_mangle]
    fn send_from(fd: i32, nowild: i32,
                 packet: &mut String, len: usize, to: NetAddress,
                 source: &mut all_addr, iface: u32) -> i32;
    #[no_mangle]
    fn lease6_allocate(addrp: &mut in6_addr, lease_type: i32)
                       -> dhcp_lease;
    #[no_mangle]
    fn lease6_find(clid: mut Vec<u8>, clid_len: i32,
                   lease_type: i32, iaid: u32,
                   addr:  &mut in6_addr)  -> dhcp_lease;
    #[no_mangle]
    fn lease6_reset();
    #[no_mangle]
    fn lease6_find_by_client(first: &mut dhcp_lease, lease_type: i32,
                             clid: mut Vec<u8>, clid_len: i32,
                             iaid: u32) -> dhcp_lease;
    #[no_mangle]
    fn lease6_find_by_addr(net: &mut in6_addr, prefix: i32,
                           addr: u64_0) -> dhcp_lease;
    #[no_mangle]
    fn lease_set_iaid(lease: &mut dhcp_lease, iaid: u32);
    #[no_mangle]
    fn lease_set_hwaddr(lease: &mut dhcp_lease, hwaddr: *const libc::c_uchar,
                        clid: *const libc::c_uchar, hw_len: i32,
                        hw_type: i32, clid_len: i32,
                        now: time::Instant, force: i32);
    #[no_mangle]
    fn lease_set_hostname(lease: &mut dhcp_lease, name: *const libc::c_char,
                          auth: i32, domain: &mut String,
                          config_domain: &mut String);
    #[no_mangle]
    fn lease_set_expires(lease: &mut dhcp_lease, len: u32,
                         now: time::Instant);
    #[no_mangle]
    fn lease_set_interface(lease: &mut dhcp_lease, interface: i32,
                           now: time::Instant);
    #[no_mangle]
    fn lease_prune(target: &mut dhcp_lease, now: time::Instant);
    #[no_mangle]
    fn lease_add_extradata(lease: &mut dhcp_lease, data: mut Vec<u8>,
                           len: u32, delim: i32);
    #[no_mangle]
    fn address6_allocate(context: &mut dhcp_context, clid: mut Vec<u8>,
                         clid_len: i32, temp_addr: i32,
                         iaid: u32, serial: i32,
                         netids: &mut dhcp_netid, plain_range: i32,
                         ans:  &mut in6_addr)  -> dhcp_context;
    #[no_mangle]
    fn address6_available(context: &mut dhcp_context, taddr: &mut in6_addr,
                          netids: &mut dhcp_netid, plain_range: i32)
                          -> dhcp_context;
    #[no_mangle]
    fn address6_valid(context: &mut dhcp_context, taddr: &mut in6_addr,
                      netids: &mut dhcp_netid, plain_range: i32)
                      -> dhcp_context;
    #[no_mangle]
    fn get_client_mac(client: &mut in6_addr, iface: i32,
                      mac: mut Vec<u8>, maclenp: &mut libc::c_uint,
                      mactypep: &mut libc::c_uint, now: time::Instant);



    #[no_mangle]
    fn free(__ptr:Vec<u8>);
}















pub type u64_0 = libc::c_ulonglong;








#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
}




#[derive(Copy, Clone)]
#[repr(C)]
pub union bigname {
    pub name: [libc::c_char; 1025],
    pub next: &mut bigname,
}






















#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolvc {
    pub next: &mut resolvc,
    pub is_default: i32,
    pub logged: i32,
    pub mtime: time::Instant,
    pub name: &mut String,
    pub wd: i32,
    pub file: &mut String,
}












#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_84 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: mut Vec<u8>,
}











#[derive(Copy, Clone)]
#[repr(C)]
pub struct ping_result {
    pub addr: in_addr,
    pub time: time::Instant,
    pub hash: u32,
    pub next: &mut ping_result,
}








extern "C" {










    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memmove(_:Vec<u8>, _: *const libc::c_void, _: u32)
               ->Vec<u8>;





    #[no_mangle]
    fn free(__ptr:Vec<u8>);






    #[no_mangle]
    fn skip_name(ansp: mut Vec<u8>, header: &mut dns_header,
                 plen: usize, extrabytes: i32) -> mut Vec<u8>;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
}









#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_header {
    pub id: u16_0,
    pub hb3: u8_0,
    pub hb4: u8_0,
    pub qdcount: u16_0,
    pub ancount: u16_0,
    pub nscount: u16_0,
    pub arcount: u16_0,
}







extern "C" {








    #[no_mangle]
    fn sendto(__fd: i32, __buf: *const libc::c_void, __n: usize,
              __flags: i32, __addr: __CONST_NetAddress_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn inet_ntop(__af: i32, __cp: *const libc::c_void,
                 __buf: &mut String, __len: socklen_t)
                 -> *const libc::c_char;



    #[no_mangle]
    fn memcpy(_:Vec<u8>, _: *const libc::c_void, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;





    #[no_mangle]
    fn free(__ptr:Vec<u8>);


    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> ;





    #[no_mangle]
    fn rand16() -> u16;
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;

    #[no_mangle]
    fn lease_update_dns(force: i32);
    #[no_mangle]
    fn reset_counter();
    #[no_mangle]
    fn save_counter(newval: i32) -> i32;
    #[no_mangle]
    fn expand(headroom: usize) ->Vec<u8>;
    #[no_mangle]
    fn ra_start_unsolicited(now: time::Instant, context:  &mut dhcp_context) ;
}














pub const IPPROTO_MH: C2RustUnnamed_0 = 135;
pub const IPPROTO_DSTOPTS: C2RustUnnamed_0 = 60;
pub const IPPROTO_NONE: C2RustUnnamed_0 = 59;
pub const IPPROTO_ICMPV6: C2RustUnnamed_0 = 58;
pub const IPPROTO_FRAGMENT: C2RustUnnamed_0 = 44;
pub const IPPROTO_ROUTING: C2RustUnnamed_0 = 43;
pub const IPPROTO_HOPOPTS: C2RustUnnamed_0 = 0;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = u16;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ping_packet {
    pub type_0: u8_0,
    pub code: u8_0,
    pub checksum: u16_0,
    pub identifier: u16_0,
    pub sequence_no: u16_0,
}



































#[derive(Copy, Clone)]
#[repr(C)]
pub struct mysubnet {
    pub addr: NetAddress,
    pub addr_used: i32,
    pub mask: i32,
}
























#[derive(Copy, Clone)]
#[repr(C)]
pub struct shared_network {
    pub if_index: i32,
    pub match_addr: in_addr,
    pub shared_addr: in_addr,
    pub match_addr6: in6_addr,
    pub shared_addr6: in6_addr,
    pub next: &mut shared_network,
}









#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}

#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: &mut stat)
                           -> i32 {
    return __fxstat(1, __fd, __statbuf);
}

#[inline]
unsafe extern "C" fn getc_unlocked2(mut __fp: &mut FILE) -> i32 {
    return if (__fp._IO_read_ptr >= __fp._IO_read_end)  i32 != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = __fp._IO_read_ptr;
        __fp._IO_read_ptr = __fp._IO_read_ptr.offset(1);
        *(fresh0)
    };
}
#[inline]
unsafe extern "C" fn getchar_unlocked2() -> i32 {
    return if (stdin._IO_read_ptr >= stdin._IO_read_end)
        != 0 {
        __uflow(stdin)
    } else {
        let fresh1 = stdin._IO_read_ptr;
        stdin._IO_read_ptr = stdin._IO_read_ptr.offset(1);
        *(fresh1)
    };
}
#[inline]
unsafe extern "C" fn fgetc_unlocked2(mut __fp: &mut FILE) -> i32 {
    return if (__fp._IO_read_ptr >= __fp._IO_read_end)  i32 != 0 {
        __uflow(__fp)
    } else {
        let fresh2 = __fp._IO_read_ptr;
        __fp._IO_read_ptr = __fp._IO_read_ptr.offset(1);
        *(fresh2)
    };
}






#[inline]
unsafe extern "C" fn fgetc_unlocked(mut __fp: &mut FILE) -> i32 {
    return if (__fp._IO_read_ptr >= __fp._IO_read_end)  i32 != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = __fp._IO_read_ptr;
        __fp._IO_read_ptr = __fp._IO_read_ptr.offset(1);
        *(fresh0)
    };
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: &mut FILE) -> i32 {
    return if (__fp._IO_read_ptr >= __fp._IO_read_end)  i32 != 0 {
        __uflow(__fp)
    } else {
        let fresh1 = __fp._IO_read_ptr;
        __fp._IO_read_ptr = __fp._IO_read_ptr.offset(1);
        *(fresh1)
    };
}
#[inline]
unsafe extern "C" fn getchar_unlocked() -> i32 {
    return if (stdin._IO_read_ptr >= stdin._IO_read_end)
        != 0 {
        __uflow(stdin)
    } else {
        let fresh2 = stdin._IO_read_ptr;
        stdin._IO_read_ptr = stdin._IO_read_ptr.offset(1);
        *(fresh2)
    };
}
#[inline]
unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                               mut endptr: &mut __gwchar_t.
                               mut base: i32) -> uintmax_t {
    return __wcstoul_internal(nptr, endptr, base, 0);
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx >> 8 & 0xff |
        (__bsx & 0xff) <<
            8) ;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000) >> 24 |
        (__bsx & 0xff0000) >> 8 |
        (__bsx & 0xff00) << 8 |
        (__bsx & 0xff) << 24;
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsxlong &
        0xff00000000000000long) >> 56
        |
        (__bsxlong &
            0xff000000000000long) >>
            40 |
        (__bsxlong &
            0xff0000000000long) >> 24
        |
        (__bsxlong &
            0xff00000000long) >> 8 |
        (__bsxlong & 0xff000000long)
            << 8 |
        (__bsxlong & 0xff0000long)
            << 24 |
        (__bsxlong & 0xff00long) <<
            40 |
        (__bsxlong & 0xfflong) <<
            56) as __uint64_t;
}



#[inline]
unsafe extern "C" fn __cmsg_nxthdr2(mut __mhdr: &mut msghdr,
                                   mut __cmsg: &mut cmsghdr) -> cmsghdr {
    if __cmsg.cmsg_len < ::std::mem::size_of::<cmsghdr>()
    {
        return 0
    }
    __cmsg =
        (__cmsg      mut Vec<u8>).offset((__cmsg.cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
            as
     ).wrapping_sub(1

            as
     )
            &
            !(::std::mem::size_of::<size_t>()
                  ).wrapping_sub(1

                  ))
           );
    if __cmsg.offset(1) >
        (__mhdr.msg_control      mut Vec<u8>).offset(__mhdr.msg_controllen)
        ||
        (__cmsg      mut Vec<u8>).offset((__cmsg.cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
            as
     ).wrapping_sub(1

            as
     )
            &
            !(::std::mem::size_of::<size_t>()
                  ).wrapping_sub(1

                  ))
           ) >
            (__mhdr.msg_control          mut Vec<u8>).offset(__mhdr.msg_controllen          isize) {
        return 0
    }
    return __cmsg;
}

extern "C" {


    #[no_mangle]
    static mut stdin: FILE;
    #[no_mangle]
    static mut stdout: FILE;
    #[no_mangle]
    fn sprintf(_: &mut String, _: *const libc::c_char, _: ...)
               -> i32;
    #[no_mangle]
    fn vfprintf(_: &mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
                -> i32;
    #[no_mangle]
    fn snprintf(_: &mut String, _: u32,
                _: *const libc::c_char, _: ...) -> i32;
    #[no_mangle]
    fn getc(__stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn putc(__c: i32, __stream:  &mut FILE)  -> i32;
    #[no_mangle]
    fn __getdelim(__lineptr: String, __n: &mut size_t,
                  __delimiter: i32, __stream:  &mut FILE)  -> __ssize_t;
    #[no_mangle]
    fn socket(__domain: i32, __type: i32,
              __protocol: i32) -> i32;
    #[no_mangle]
    fn bind(__fd: i32, __addr: __CONST_NetAddress_ARG, __len: socklen_t)
            -> i32;
    #[no_mangle]
    fn getsockname(__fd: i32, __addr: __NetAddress_ARG,
                   __len:  &mut socklen_t)  -> i32;
    #[no_mangle]
    fn recv(__fd: i32, __buf:Vec<u8>, __n: usize,
            __flags: i32) -> ssize_t;
    #[no_mangle]
    fn recvmsg(__fd: i32, __message: &mut msghdr,
               __flags: i32) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: i32, __level: i32,
                  __optname: i32, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> i32;
    #[no_mangle]
    fn __xstat(__ver: i32, __filename: *const libc::c_char,
               __stat_buf:  &mut stat)  -> i32;
    #[no_mangle]
    fn __fxstat(__ver: i32, __fildes: i32,
                __stat_buf:  &mut stat)  -> i32;
    #[no_mangle]
    fn __xstat64(__ver: i32, __filename: *const libc::c_char,
                 __stat_buf:  &mut stat64)  -> i32;
    #[no_mangle]
    fn __fxstat64(__ver: i32, __fildes: i32,
                  __stat_buf:  &mut stat64)  -> i32;
    #[no_mangle]
    fn __fxstatat(__ver: i32, __fildes: i32,
                  __filename: *const libc::c_char, __stat_buf: &mut stat,
                  __flag: i32) -> i32;
    #[no_mangle]
    fn __fxstatat64(__ver: i32, __fildes: i32,
                    __filename: *const libc::c_char, __stat_buf: &mut stat64,
                    __flag: i32) -> i32;
    #[no_mangle]
    fn __lxstat(__ver: i32, __filename: *const libc::c_char,
                __stat_buf:  &mut stat)  -> i32;
    #[no_mangle]
    fn __lxstat64(__ver: i32, __filename: *const libc::c_char,
                  __stat_buf:  &mut stat64)  -> i32;
    #[no_mangle]
    fn __xmknod(__ver: i32, __path: *const libc::c_char,
                __mode: __mode_t, __dev:  &mut __dev_t)  -> i32;
    #[no_mangle]
    fn __xmknodat(__ver: i32, __fd: i32,
                  __path: *const libc::c_char, __mode: __mode_t,
                  __dev:  &mut __dev_t)  -> i32;
    #[no_mangle]
    fn ioctl(__fd: i32, __request: u32, _: ...)
             -> i32;
    #[no_mangle]
    fn memmove(_:Vec<u8>, _: *const libc::c_void, _: u32)
               ->Vec<u8>;
    #[no_mangle]
    fn memset(_:Vec<u8>, _: i32, _: u32)
              ->Vec<u8>;
    #[no_mangle]
    fn strcpy(_: &mut String, _: *const libc::c_char)
              -> &mut String;
    #[no_mangle]
    fn strncat(_: &mut String, _: *const libc::c_char, _: u32)
               -> &mut String;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
              -> &mut String;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: i32) -> &mut String;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
                  -> i32;
    #[no_mangle]
    fn lseek(__fd: i32, __offset: __off64_t, __whence: i32)
             -> __off64_t;
    #[no_mangle]
    fn close(__fd: i32) -> i32;
    #[no_mangle]
    fn geteuid() -> __uid_t;
    #[no_mangle]
    fn __uflow(_:  &mut FILE)  -> i32;
    #[no_mangle]
    fn __overflow(_: &mut FILE, _: i32) -> i32;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: String)
              -> libc::c_double;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: String,
              _: i32) -> i32;
    #[no_mangle]
    fn strtoll(_: *const libc::c_char, _: String,
               _: i32) -> libc::c_longlong;
    #[no_mangle]
    fn free(__ptr:Vec<u8>);
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: i32, _: ...)
            -> i32;
    #[no_mangle]
    fn __ctype_b_loc() -> u16;
    #[no_mangle]
    fn __ctype_tolower_loc() -> __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> __int32_t;
    #[no_mangle]
    fn difftime(__time1: time::Instant, __time0: time::Instant) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> ;
    #[no_mangle]
    fn __strtol_internal(__nptr: *const libc::c_char,
                         __endptr: String,
                         __base: i32, __group: i32)
                         -> i32;
    #[no_mangle]
    fn __strtoul_internal(__nptr: *const libc::c_char,
                          __endptr: String,
                          __base: i32, __group: i32)
                          -> libc::c_ulong;
    #[no_mangle]
    fn __wcstol_internal(__nptr: *const __gwchar_t,
                         __endptr: &mut__gwchar_t, __base: i32,
                         __group: i32) -> i32;
    #[no_mangle]
    fn __wcstoul_internal(__nptr: *const __gwchar_t,
                          __endptr: &mut__gwchar_t, __base: i32,
                          __group: i32) -> libc::c_ulong;
    #[no_mangle]
    static mut dnsmasq_daemon: dnsmasq_daemon;
    #[no_mangle]
    fn safe_strncpy(dest: &mut String, src: *const libc::c_char,
                    size: usize);
    #[no_mangle]
    fn whine_malloc(size: usize) ->Vec<u8>;
    #[no_mangle]
    fn sa_len(addr: NetAddress) -> i32;
    #[no_mangle]
    fn NetAddress_isequal(s1: NetAddress, s2: NetAddress)
                        -> i32;
    #[no_mangle]
    fn prettyprint_addr(addr: NetAddress, buf: &mut String)
                        -> i32;
    #[no_mangle]
    fn read_write(fd: i32, packet: mut Vec<u8>,
                  size: i32, rw: i32) -> i32;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> i32;
    #[no_mangle]
    fn my_syslog(priority: i32, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn send_from(fd: i32, nowild: i32,
                 packet: &mut String, len: usize, to: NetAddress,
                 source: &mut all_addr, iface: u32) -> i32;
    #[no_mangle]
    fn indextoname(fd: i32, index: i32,
                   name: &mut String) -> i32;
    #[no_mangle]
    fn enumerate_interfaces(reset: i32) -> i32;
    #[no_mangle]
    fn iface_check(family: i32, addr: &mut all_addr,
                   name: &mut String, auth: )
                   -> i32;
    #[no_mangle]
    fn loopback_exception(fd: i32, family: i32,
                          addr: &mut all_addr, name: &mut String)
                          -> i32;
    #[no_mangle]
    fn label_exception(index: i32, family: i32,
                       addr:  &mut all_addr)  -> i32;
    #[no_mangle]
    fn fix_fd(fd: i32) -> i32;
    #[no_mangle]
    fn lease_find_by_addr(addr: in_addr) -> dhcp_lease;
    #[no_mangle]
    fn queue_tftp(file_len: off_t, filename: &mut String,
                  peer: NetAddress);
    #[no_mangle]
    fn find_mac(addr: NetAddress, mac: mut Vec<u8>,
                lazy: i32, now: time::Instant) -> i32;
    #[no_mangle]
    fn poll_check(fd: i32, event: libc::c_short) -> i32;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = u16;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = i32;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;




pub type __blksize_t = i32;


pub type uid_t = __uid_t;

pub type ssize_t = __ssize_t;
pub type time::Instant = __time::Instant;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time::Instant,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base:Vec<u8>,
    pub iov_len: usize,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetAddress {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name:Vec<u8>,
    pub msg_namelen: socklen_t,
    pub msg_iov: &mut iovec,
    pub msg_iovlen: usize,
    pub msg_control:Vec<u8>,
    pub msg_controllen: usize,
    pub msg_flags: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
    pub __cmsg_data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __NetAddress_ARG {
    pub __NetAddress__: NetAddress,
    pub __NetAddress_at__: NetAddress_at,
    pub __NetAddress_ax25__: NetAddress_ax25,
    pub __NetAddress_dl__: NetAddress_dl,
    pub __NetAddress_eon__: NetAddress_eon,
    pub __NetAddress_in__: NetAddress_in,
    pub __NetAddress_in6__: NetAddress_in6,
    pub __NetAddress_inarp__: NetAddress_inarp,
    pub __NetAddress_ipx__: NetAddress_ipx,
    pub __NetAddress_iso__: NetAddress_iso,
    pub __NetAddress_ns__: NetAddress_ns,
    pub __NetAddress_un__: NetAddress_un,
    pub __NetAddress_x25__: NetAddress_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetAddress_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetAddress_in6 {
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
pub struct NetAddress_in {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_NetAddress_ARG {
    pub __NetAddress__: *const NetAddress,
    pub __NetAddress_at__: *const NetAddress_at,
    pub __NetAddress_ax25__: *const NetAddress_ax25,
    pub __NetAddress_dl__: *const NetAddress_dl,
    pub __NetAddress_eon__: *const NetAddress_eon,
    pub __NetAddress_in__: *const NetAddress_in,
    pub __NetAddress_in6__: *const NetAddress_in6,
    pub __NetAddress_inarp__: *const NetAddress_inarp,
    pub __NetAddress_ipx__: *const NetAddress_ipx,
    pub __NetAddress_iso__: *const NetAddress_iso,
    pub __NetAddress_ns__: *const NetAddress_ns,
    pub __NetAddress_un__: *const NetAddress_un,
    pub __NetAddress_x25__: *const NetAddress_x25,
}

pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: u32,
}
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
    pub __pad0: i32,
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
    pub __pad0: i32,
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
pub union C2RustUnnamed_2 {
    pub ifrn_name: [libc::c_char; 16],
}

pub type __compar_fn_t
=
Option<unsafe extern "C" fn(_: *const libc::c_void,
                            _: *const libc::c_void) -> i32>;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct C2RustUnnamed_48 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub target: &mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub keydata: &mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub target: C2RustUnnamed_9,
    pub uid: u32,
    pub is_name_ptr: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub cache: &mut crec,
    pub name: &mut String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec {
    pub next: &mut crec,
    pub prev: &mut crec,
    pub hash_next: &mut crec,
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_107 {
    pub sname: [libc::c_char; 50],
    pub bname: &mut bigname,
    pub namep: &mut String,
}



























#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_lease {
    pub clid_len: i32,
    pub clid: mut Vec<u8>,
    pub hostname: &mut String,
    pub fqdn: &mut String,
    pub old_hostname: &mut String,
    pub flags: i32,
    pub expires: time::Instant,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [libc::c_uchar; 16],
    pub addr: in_addr,
    pub override_0: in_addr,
    pub giaddr: in_addr,
    pub extradata: mut Vec<u8>,
    pub extradata_len: u32,
    pub extradata_size: u32,
    pub last_interface: i32,
    pub new_interface: i32,
    pub new_prefixlen: i32,
    pub addr6: in6_addr,
    pub iaid: u32,
    pub slaac_address: &mut slaac_address,
    pub vendorclass_count: i32,
    pub next: &mut dhcp_lease,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slaac_address {
    pub addr: in6_addr,
    pub ping_time: time::Instant,
    pub backoff: i32,
    pub next: &mut slaac_address,
}






#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_11,
    pub val: mut Vec<u8>,
    pub netid: &mut dhcp_netid,
    pub next: &mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: mut Vec<u8>,
}









#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_context {
    pub lease_time: u32,
    pub addr_epoch: u32,
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub local: in_addr,
    pub router: in_addr,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub local6: in6_addr,
    pub prefix: i32,
    pub if_index: i32,
    pub valid: u32,
    pub preferred: u32,
    pub saved_valid: u32,
    pub ra_time: time::Instant,
    pub ra_short_period_start: time::Instant,
    pub address_lost_time: time::Instant,
    pub template_interface: &mut String,
    pub flags: i32,
    pub netid: dhcp_netid,
    pub filter: &mut dhcp_netid,
    pub next: &mut dhcp_context,
    pub current: &mut dhcp_context,
}








#[derive(Copy, Clone)]
#[repr(C)]
pub struct errmess {
    pub op: u16,
    pub err: u16,
    pub message: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct datamess {
    pub op: u16,
    pub block: u16,
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oackmess {
    pub op: u16,
    pub data: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ack {
    pub op: u16,
    pub block: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c: mut Vec<u8>,
    pub p: &mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_135 {
    pub c: mut Vec<u8>,
    pub p: &mut InPktInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
    pub control: [libc::c_char; 32],
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: &mut msghdr,
                                   mut __cmsg: &mut cmsghdr) -> cmsghdr {
    if __cmsg.cmsg_len < ::std::mem::size_of::<cmsghdr>()
    {
        return 0
    } /* may be zero to use ephemeral port */
    __cmsg =
        (__cmsg      mut Vec<u8>).offset((__cmsg.cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
            as
     ).wrapping_sub(1

            as
     )
            &
            !(::std::mem::size_of::<size_t>()
                  ).wrapping_sub(1

                  ))
           );
    if __cmsg.offset(1) >
        (__mhdr.msg_control      mut Vec<u8>).offset(__mhdr.msg_controllen)
        ||
        (__cmsg      mut Vec<u8>).offset((__cmsg.cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
            as
     ).wrapping_sub(1

            as
     )
            &
            !(::std::mem::size_of::<size_t>()
                  ).wrapping_sub(1

                  ))
           ) >
            (__mhdr.msg_control          mut Vec<u8>).offset(__mhdr.msg_controllen          isize) {
        return 0
    }
    return __cmsg;
}



#[inline]
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: ::std::ffi::VaList) -> i32 {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn getchar() -> i32 { return getc(stdin); }
















