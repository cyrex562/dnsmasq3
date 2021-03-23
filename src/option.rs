#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
// #![register_tool(c2rust)]
// #![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
//            ptr_wrapping_offset_from, register_tool)]

#[no_mangle]
pub static mut prioritynames: [CODE; 13] =
    [{
         let mut init =
             _code{c_name:
                       b"alert\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"crit\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"debug\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"emerg\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"err\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"error\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"info\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"none\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 0x10 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"notice\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"panic\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"warn\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"warning\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name: 0 as *const libc::c_char as *mut libc::c_char,
                   c_val: -(1 as libc::c_int),};
         init
     }];
#[no_mangle]
pub static mut facilitynames: [CODE; 23] =
    [{
         let mut init =
             _code{c_name:
                       b"auth\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (4 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"authpriv\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (10 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"cron\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (9 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"daemon\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (3 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"ftp\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (11 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"kern\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (0 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"lpr\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (6 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"mail\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (2 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"mark\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val:
                       (24 as libc::c_int) << 3 as libc::c_int |
                           0 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"news\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (7 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"security\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (4 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"syslog\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (5 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"user\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (1 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"uucp\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (8 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local0\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (16 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local1\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (17 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local2\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (18 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local3\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (19 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local4\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (20 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local5\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (21 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local6\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (22 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name:
                       b"local7\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   c_val: (23 as libc::c_int) << 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             _code{c_name: 0 as *const libc::c_char as *mut libc::c_char,
                   c_val: -(1 as libc::c_int),};
         init
     }];
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
static mut mem_recover: libc::c_int = 0 as libc::c_int;
static mut mem_jmp: jmp_buf =
    [__jmp_buf_tag{__jmpbuf: [0; 8],
                   __mask_was_saved: 0,
                   __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
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
                    val: 333 as libc::c_int,};
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
                    val: 335 as libc::c_int,};
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
                    val: 338 as libc::c_int,};
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
                    val: 332 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"local\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 286 as libc::c_int,};
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
                    val: 342 as libc::c_int,};
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
                    val: 356 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"dns-rr\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 310 as libc::c_int,};
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
                    val: 354 as libc::c_int,};
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
                    val: 360 as libc::c_int,};
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
                    val: 256 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-ignore-names\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 257 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"enable-tftp\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 258 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-secure\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 259 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-no-fail\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 344 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-unique-root\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 274 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-root\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 260 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"tftp-max\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 263 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"tftp-mtu\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 349 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-lowercase\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 309 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-single-port\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 359 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"ptr-record\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 261 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"naptr-record\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 287 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bridge-interface\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 262 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"shared-network\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 357 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-option-force\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 264 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-no-blocksize\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 265 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"log-dhcp\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 266 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"log-async\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 267 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-circuitid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 268 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-remoteid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 269 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-subscrid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 270 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-pxe-vendor\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 361 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"interface-name\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 271 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-hostsfile\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 273 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-optsfile\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 280 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-hostsdir\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 340 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-optsdir\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 341 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-no-override\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 275 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"tftp-port-range\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 276 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"stop-dns-rebind\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 277 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"rebind-domain-ok\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 298 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"all-servers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 278 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-match\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 281 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-name-match\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 355 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-broadcast\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 282 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"neg-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 283 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"max-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 297 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"min-cache-ttl\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 339 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"max-cache-ttl\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 312 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-alternate-port\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 284 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-scriptuser\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 285 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"min-port\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 288 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"max-port\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 345 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-fqdn\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 289 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"cname\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 290 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"pxe-prompt\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 291 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"pxe-service\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 292 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"test\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 293 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"tag-if\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 294 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-proxy\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 295 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-generate-names\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 296 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"rebind-localhost-ok\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 299 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"add-mac\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 300 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"add-subnet\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 325 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"add-cpe-id\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 346 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"proxy-dnssec\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 301 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-sequential-ip\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 302 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"conntrack\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 303 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-client-update\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 304 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-luascript\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 305 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"enable-ra\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 306 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-duid\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 307 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"host-record\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 308 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"bind-dynamic\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 311 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-zone\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 313 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-server\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 314 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"auth-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 315 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"auth-soa\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 316 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-sec-servers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 317 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"auth-peer\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 318 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"ipset\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 319 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"synth-domain\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 320 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"dnssec\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 329 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"trust-anchor\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 330 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-debug\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 331 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-check-unsigned\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 334 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-no-timecheck\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 336 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dnssec-timestamp\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 343 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-relay\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 323 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"ra-param\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 324 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"quiet-dhcp\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 326 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"quiet-dhcp6\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 327 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"quiet-ra\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 328 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dns-loop-detect\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 337 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"script-arp\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 347 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"dhcp-ttl\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 348 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-reply-delay\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 350 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-rapid-commit\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 351 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"dumpfile\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 352 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: b"dumpmask\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 353 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name:
                        b"dhcp-ignore-clid\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 358 as libc::c_int,};
         init
     },
     {
         let mut init =
             option{name: 0 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,};
         init
     }];
static mut usage: [C2RustUnnamed_9; 165] =
    [{
         let mut init =
             C2RustUnnamed_9{opt: 'a' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify local address(es) to listen on.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'A' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Return ipaddr for all hosts in specified domains.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'b' as i32,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Fake reverse lookups for RFC1918 private address ranges.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'B' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Treat ipaddr as NXDOMAIN (defeats Verisign wildcard).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'c' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
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
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify configuration file (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/dnsmasq.conf\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'd' as i32,
                             rept: 6 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT fork into the background: run in debug mode.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'D' as i32,
                             rept: 12 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT forward queries with no domain part.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'e' as i32,
                             rept: 3 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Return self-pointing MX records for local hosts.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'E' as i32,
                             rept: 9 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Expand simple names in /etc/hosts with domain-suffix.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'f' as i32,
                             rept: 1 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Don\'t forward spurious DNS requests from Windows hosts.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'F' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>,...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable DHCP in the range given with lease duration.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'g' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<groupname>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Change to this group after startup (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"dip\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'G' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<hostspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set address or hostname for a specified machine.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 273 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP host specs from file.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 280 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP option specs from file.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 340 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP host specs from a directory.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 341 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read DHCP options from a directory.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 294 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"tag-expression\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Evaluate conditional tag expression.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'h' as i32,
                             rept: 4 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT load %s file.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg:
                                 b"/etc/hosts\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'H' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify a hosts file to be read in addition to %s.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/hosts\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 342 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read hosts files from a directory.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'i' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify interface(s) to listen on.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'I' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify interface(s) NOT to listen on.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'j' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<class>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map DHCP user class to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 268 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<circuit>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map RFC3046 circuit-id to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 269 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<remote>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map RFC3046 remote-id to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 270 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<remote>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map RFC3993 subscriber-id to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 361 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<vendor>[,...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify vendor class to match for PXE requests.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'J' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"tag:<tag>...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Don\'t do DHCP for hosts with tag set.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 282 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Force broadcast replies for hosts with tag set.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'k' as i32,
                             rept: 16 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT fork into the background, do NOT run in debug mode.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'K' as i32,
                             rept: 17 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Assume we are the only DHCP server on the local network.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'l' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify where to store DHCP leases (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/var/lib/misc/dnsmasq.leases\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'L' as i32,
                             rept: 10 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Return MX records for local hosts.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'm' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<host_name>,<target>,<pref>\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify an MX record.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'M' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<bootp opts>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify BOOTP options to DHCP server.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'n' as i32,
                             rept: 5 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT poll %s file, reload only on SIGHUP.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/resolv.conf\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'N' as i32,
                             rept: 11 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT cache failed search results.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'o' as i32,
                             rept: 7 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Use nameservers strictly in the order given in %s.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/resolv.conf\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'O' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<optspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify options to be sent to DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 264 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<optspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"DHCP option sent even if the client does not request it.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'p' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify port to listen for DNS requests on (defaults to 53).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'P' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
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
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Log DNS queries.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'Q' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Force the originating port for upstream DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'R' as i32,
                             rept: 8 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT read resolv.conf.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'r' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify path to resolv.conf (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/resolv.conf\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 333 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify path to file with server= options\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'S' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify address(es) of upstream servers with optional domains.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 332 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<addr>/<prefix>,<ipaddr>\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify address of upstream servers for reverse address queries\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 286 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Never forward queries to specified domains.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 's' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<domain>[,<range>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify the domain to be assigned in DHCP leases.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 't' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<host_name>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify default target in an MX record.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'T' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live in seconds for replies from /etc/hosts.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 283 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live in seconds for negative caching.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 297 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live in seconds for maximum TTL to send to clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 312 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live ceiling for cache.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 339 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify time-to-live floor for cache.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'u' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<username>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Change to this user after startup. (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"nobody\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'U' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<class>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Map DHCP vendor class to tag.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'v' as i32,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Display dnsmasq version and copyright information.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'V' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>,<ipaddr>,<netmask>\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Translate IPv4 addresses from upstream servers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'W' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<target>,...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify a SRV record.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'w' as i32,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Display this message. Use --help dhcp or --help dhcp6 for known DHCP options.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'x' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify path of PID file (defaults to %s).\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/var/run/dnsmasq.pid\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'X' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
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
                             rept: 18 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Answer DNS queries based on the interface a query was sent to.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'Y' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<txt>[,<txt]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify TXT DNS record.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 261 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<target>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify PTR DNS record.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 271 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Give DNS name to IPv4 address of interface.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'z' as i32,
                             rept: 13 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Bind only to interfaces in use.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 'Z' as i32,
                             rept: 14 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Read DHCP static host information from %s.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/ethers\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '1' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<busname>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable the DBus interface for setting upstream servers, etc.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 354 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<busname>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable the UBus interface.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '2' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Do not provide DHCP on this interface, only provide DNS.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '3' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>]...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable dynamic address allocation for bootp.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '4' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<mac address>\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Map MAC address (with wildcards) to option set.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 262 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<iface>,<alias>..\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Treat DHCP requests on aliases as arriving from interface.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 357 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<iface>|<addr>,<addr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify extra networks sharing a broadcast domain for DHCP\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '5' as i32,
                             rept: 21 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Disable ICMP echo address checking in the DHCP server.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '6' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Shell script to run on DHCP lease creation and destruction.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 305 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"path\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Lua script to run on DHCP lease creation and destruction.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 285 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<username>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Run lease-change scripts as this user.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 347 as libc::c_int,
                             rept: 53 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Call dhcp-script with changes to local ARP table.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '7' as i32,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Read configuration from all the files in this directory.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '8' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<facility>|<file>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Log to this syslog facility or file. (defaults to DAEMON)\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '9' as i32,
                             rept: 22 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do not use leasefile.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: '0' as i32,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
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
             C2RustUnnamed_9{opt: 256 as libc::c_int,
                             rept: 24 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Clear DNS cache when reloading %s.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 b"/etc/resolv.conf\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 257 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>]...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Ignore hostnames provided by DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 275 as libc::c_int,
                             rept: 30 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do NOT reuse filename and server fields for extra DHCP options.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 258 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=<intr>[,<intr>]]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable integrated read-only TFTP server.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 260 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<dir>[,<iface>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Export files by TFTP only from the specified subtree.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 274 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=ip|mac]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add client IP or hardware address to tftp-root.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 259 as libc::c_int,
                             rept: 26 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Allow access only to files owned by the user running dnsmasq.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 344 as libc::c_int,
                             rept: 52 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do not terminate the service if TFTP directories are inaccessible.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 263 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
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
             C2RustUnnamed_9{opt: 349 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Maximum MTU to use for TFTP transfers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 265 as libc::c_int,
                             rept: 27 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Disable the TFTP blocksize extension.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 309 as libc::c_int,
                             rept: 38 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Convert TFTP filenames to lowercase\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 276 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<start>,<end>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Ephemeral port range for use by TFTP transfers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 359 as libc::c_int,
                             rept: 60 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Use only one port for TFTP server.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 266 as libc::c_int,
                             rept: 28 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Extra logging for DHCP.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 267 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<integer>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Enable async. logging; optionally set queue length.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 277 as libc::c_int,
                             rept: 31 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Stop DNS rebinding. Filter private IP ranges when resolving.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 299 as libc::c_int,
                             rept: 25 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Allow rebinding of 127.0.0.0/8, for RBL servers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 298 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"/<domain>/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Inhibit DNS-rebind protection on this domain.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 278 as libc::c_int,
                             rept: 23 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Always perform DNS queries to all servers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 281 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<optspec>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set tag if client includes matching option in request.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 355 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"set:<tag>,<string>[*]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set tag if client provides given name.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 284 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"[=<ports>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Use alternative ports for DHCP.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 287 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<naptr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify NAPTR DNS record.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 288 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<port>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify lowest port available for DNS query transmission.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 345 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<port>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify highest port available for DNS query transmission.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 289 as libc::c_int,
                             rept: 20 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Use only fully qualified domain names for DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 296 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=tag:<tag>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Generate hostnames based on MAC address for nameless clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 295 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=<ipaddr>]...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Use these DHCP relays as full proxies.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 323 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<local-addr>,<server>[,<iface>]\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Relay DHCP requests to a remote server\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 290 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<alias>,<target>[,<ttl>]\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify alias name for LOCAL DNS name.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 291 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<prompt>,[<timeout>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Prompt to send to PXE clients.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 292 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<service>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Boot service for PXE menu.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 293 as libc::c_int,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Check configuration syntax.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 300 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"[=base64|text]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add requestor\'s MAC address to forwarded DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 325 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<v4 pref>[,<v6 pref>]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add specified IP subnet to forwarded DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 346 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<text>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Add client identification to forwarded DNS queries.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 301 as libc::c_int,
                             rept: 33 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Proxy DNSSEC validation results from upstream nameservers.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 302 as libc::c_int,
                             rept: 34 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Attempt to allocate sequential IP addresses to DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 358 as libc::c_int,
                             rept: 59 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Ignore client identifier option sent by DHCP clients.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 303 as libc::c_int,
                             rept: 35 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Copy connection-track mark from queries to upstream connections.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 304 as libc::c_int,
                             rept: 36 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Allow DHCP clients to do their own DDNS updates.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 306 as libc::c_int,
                             rept: 37 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Send router-advertisements for interfaces doing DHCPv6\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 307 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<enterprise>,<duid>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify DUID_EN-type DHCPv6 server DUID\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 308 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<address>[,<ttl>]\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify host (A/AAAA and PTR) records\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 356 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<flags>,<tag>,<value>\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify certification authority authorization record\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 310 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<name>,<RR-number>,[<data>]\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify arbitrary DNS resource record\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 311 as libc::c_int,
                             rept: 39 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Bind to interfaces in use - check for new interfaces\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 314 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<NS>,<interface>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Export local names to global DNS\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 313 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<domain>,[<subnet>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Domain to export to global DNS\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 315 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set TTL for authoritative replies\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 316 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<serial>[,...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set authoritative zone information\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 317 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<NS>[,<NS>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Secondary authoritative nameservers for forward domains\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 318 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>[,<ipaddr>...]\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Peers which are allowed to do zone transfer\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 319 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"/<domain>[/<domain>...]/<ipset>...\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify ipsets to which matching domains should be added\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 320 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<domain>,<range>,[<prefix>]\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Specify a domain and address range for synthesised names\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 329 as libc::c_int,
                             rept: 45 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Activate DNSSEC validation\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 330 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<domain>,[<class>],...\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Specify trust anchor key digest.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 331 as libc::c_int,
                             rept: 47 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Disable upstream checking for DNSSEC debugging.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 334 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Ensure answers without DNSSEC are in unsigned zones.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 336 as libc::c_int,
                             rept: 46 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Don\'t check DNSSEC signature timestamps until first cache-reload\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 343 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Timestamp file to verify system clock for DNSSEC\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 324 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<iface>,[mtu:<value>|<interface>|off,][<prio>,]<intval>[,<lifetime>]\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Set MTU, priority, resend-interval and router-lifetime\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 326 as libc::c_int,
                             rept: 42 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do not log routine DHCP.\x00" as *const u8
                                     as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 327 as libc::c_int,
                             rept: 43 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do not log routine DHCPv6.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 328 as libc::c_int,
                             rept: 44 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Do not log RA.\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 335 as libc::c_int,
                             rept: 49 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Accept queries only from directly-connected networks.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 337 as libc::c_int,
                             rept: 50 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Detect and remove DNS forwarding loops.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 338 as libc::c_int,
                             rept: 62 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 b"<ipaddr>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Ignore DNS responses containing ipaddr.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 348 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<ttl>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Set TTL in DNS responses with DHCP-derived addresses.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 350 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<integer>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Delay DHCP replies for at least number of seconds.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 351 as libc::c_int,
                             rept: 57 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Enables DHCPv4 Rapid Commit option.\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 352 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<path>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Path to debug packet dump file\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 353 as libc::c_int,
                             rept:
                                 (62 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_uint,
                             flagdesc:
                                 b"<hex>\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             desc:
                                 b"Mask which packets to dump\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 360 as libc::c_int,
                             rept: 61 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 b"Call dhcp-script when lease expiry changes.\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_9{opt: 0 as libc::c_int,
                             rept: 0 as libc::c_int as libc::c_uint,
                             flagdesc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             desc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             arg:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,};
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
static mut meta: [libc::c_char; 33] =
    unsafe {
        *::std::mem::transmute::<&[u8; 33],
                                 &[libc::c_char; 33]>(b"\x00123456 \x08\t\n78\r90abcdefABCDE\x1bF:,.\x00")
    };
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
unsafe extern "C" fn unhide_meta(mut cr: libc::c_char) -> libc::c_char {
    let mut c: libc::c_uint = cr as libc::c_uint;
    if (c as libc::c_ulong) <
           (::std::mem::size_of::<[libc::c_char; 33]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        cr = meta[c as usize]
    }
    return cr;
}
unsafe extern "C" fn unhide_metas(mut cp: *mut libc::c_char) {
    if !cp.is_null() {
        while *cp != 0 { *cp = unhide_meta(*cp); cp = cp.offset(1) }
    };
}
unsafe extern "C" fn opt_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if mem_recover != 0 {
        ret = whine_malloc(size);
        if ret.is_null() { longjmp(mem_jmp.as_mut_ptr(), 1 as libc::c_int); }
    } else { ret = safe_malloc(size) }
    return ret;
}
unsafe extern "C" fn opt_string_alloc(mut cp: *const libc::c_char)
 -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
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
unsafe extern "C" fn split_chr(mut s: *mut libc::c_char, mut c: libc::c_char)
 -> *mut libc::c_char {
    let mut comma: *mut libc::c_char =
        0 as *mut libc::c_char; /* strlen("xxx.yyy.zzz.ttt.in-addr.arpa")+1 */
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() || { comma = strchr(s, c as libc::c_int); comma.is_null() }
       {
        return 0 as *mut libc::c_char
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
unsafe extern "C" fn split(mut s: *mut libc::c_char) -> *mut libc::c_char {
    return split_chr(s, ',' as i32 as libc::c_char);
}
unsafe extern "C" fn canonicalise_opt(mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
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
                0 as *mut libc::c_char, 4 as libc::c_int);
        }
    }
    return ret;
}
unsafe extern "C" fn atoi_check(mut a: *mut libc::c_char,
                                mut res: *mut libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
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
unsafe extern "C" fn atoi_check16(mut a: *mut libc::c_char,
                                  mut res: *mut libc::c_int) -> libc::c_int {
    if atoi_check(a, res) == 0 || *res < 0 as libc::c_int ||
           *res > 0xffff as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_txt(mut name: *mut libc::c_char,
                             mut txt: *mut libc::c_char,
                             mut stat_0: libc::c_int) {
    let mut r: *mut txt_record =
        opt_malloc(::std::mem::size_of::<txt_record>() as libc::c_ulong) as
            *mut txt_record;
    if !txt.is_null() {
        let mut len: size_t = strlen(txt);
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
    (*r).class = 3 as libc::c_int as libc::c_ushort;
}
unsafe extern "C" fn do_usage() {
    let mut buff: [libc::c_char; 100] = [0; 100];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tab: [C2RustUnnamed_10; 6] =
        [{
             let mut init =
                 C2RustUnnamed_10{handle: '$' as i32 as libc::c_char,
                                  val: 150 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '*' as i32 as libc::c_char,
                                  val: 4096 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '&' as i32 as libc::c_char,
                                  val: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '!' as i32 as libc::c_char,
                                  val: 150 as libc::c_int,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_10{handle: '#' as i32 as libc::c_char,
                                  val: 50 as libc::c_int,};
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
        let mut desc: *mut libc::c_char = usage[i as usize].flagdesc;
        let mut eq: *mut libc::c_char =
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
unsafe extern "C" fn parse_mysockaddr(mut arg: *mut libc::c_char,
                                      mut addr: *mut mysockaddr)
 -> *mut libc::c_char {
    if inet_pton(2 as libc::c_int, arg,
                 &mut (*addr).in_0.sin_addr as *mut in_addr as
                     *mut libc::c_void) > 0 as libc::c_int {
        (*addr).sa.sa_family = 2 as libc::c_int as sa_family_t
    } else if inet_pton(10 as libc::c_int, arg,
                        &mut (*addr).in6.sin6_addr as *mut in6_addr as
                            *mut libc::c_void) > 0 as libc::c_int {
        (*addr).sa.sa_family = 10 as libc::c_int as sa_family_t
    } else {
        return b"bad address\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn parse_server(mut arg: *mut libc::c_char,
                                      mut addr: *mut mysockaddr,
                                      mut source_addr: *mut mysockaddr,
                                      mut interface: *mut libc::c_char,
                                      mut flags: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut source_port: libc::c_int = 0 as libc::c_int;
    let mut serv_port: libc::c_int = 53 as libc::c_int;
    let mut portno: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut source: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut interface_opt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scope_index: libc::c_int = 0 as libc::c_int;
    let mut scope_id: *mut libc::c_char = 0 as *mut libc::c_char;
    if arg.is_null() || strlen(arg) == 0 as libc::c_int as libc::c_ulong {
        *flags |= 2 as libc::c_int;
        *interface = 0 as libc::c_int as libc::c_char;
        return 0 as *mut libc::c_char
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
            safe_strncpy(interface, interface_opt,
                         16 as libc::c_int as size_t);
        }
    }
    if inet_pton(2 as libc::c_int, arg,
                 &mut (*addr).in_0.sin_addr as *mut in_addr as
                     *mut libc::c_void) > 0 as libc::c_int {
        (*addr).in_0.sin_port = __bswap_16(serv_port as __uint16_t);
        (*source_addr).sa.sa_family = 2 as libc::c_int as sa_family_t;
        (*addr).sa.sa_family = (*source_addr).sa.sa_family;
        (*source_addr).in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
        (*source_addr).in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        if !source.is_null() {
            if !flags.is_null() { *flags |= 16 as libc::c_int }
            (*source_addr).in_0.sin_port =
                __bswap_16(source_port as __uint16_t);
            if !(inet_pton(2 as libc::c_int, source,
                           &mut (*source_addr).in_0.sin_addr as *mut in_addr
                               as *mut libc::c_void) > 0 as libc::c_int) {
                if !interface_opt.is_null() {
                    return b"interface can only be specified once\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char
                }
                (*source_addr).in_0.sin_addr.s_addr =
                    0 as libc::c_int as in_addr_t;
                safe_strncpy(interface, source, 16 as libc::c_int as size_t);
            }
        }
    } else if inet_pton(10 as libc::c_int, arg,
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
        (*source_addr).sa.sa_family = 10 as libc::c_int as sa_family_t;
        (*addr).sa.sa_family = (*source_addr).sa.sa_family;
        (*source_addr).in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
        (*addr).in6.sin6_flowinfo = (*source_addr).in6.sin6_flowinfo;
        if !source.is_null() {
            if !flags.is_null() { *flags |= 16 as libc::c_int }
            (*source_addr).in6.sin6_port =
                __bswap_16(source_port as __uint16_t);
            if inet_pton(10 as libc::c_int, source,
                         &mut (*source_addr).in6.sin6_addr as *mut in6_addr as
                             *mut libc::c_void) == 0 as libc::c_int {
                if !interface_opt.is_null() {
                    return b"interface can only be specified once\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char
                }
                (*source_addr).in6.sin6_addr = in6addr_any;
                safe_strncpy(interface, source, 16 as libc::c_int as size_t);
            }
        }
    } else {
        return b"bad address\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn add_rev4(mut addr: in_addr, mut msize: libc::c_int)
 -> *mut server {
    let mut serv: *mut server =
        opt_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
            *mut server;
    let mut a: in_addr_t = __bswap_32(addr.s_addr);
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
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
            return 0 as *mut server
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
    (*serv).flags = 8 as libc::c_int;
    (*serv).next = (*dnsmasq_daemon).servers;
    (*dnsmasq_daemon).servers = serv;
    return serv;
}
unsafe extern "C" fn add_rev6(mut addr: *mut in6_addr, mut msize: libc::c_int)
 -> *mut server {
    let mut serv: *mut server =
        opt_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
            *mut server;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    memset(serv as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<server>() as libc::c_ulong);
    (*serv).domain =
        opt_malloc(73 as libc::c_int as size_t) as *mut libc::c_char;
    p = (*serv).domain;
    i = msize - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut dig: libc::c_int =
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
    (*serv).flags = 8 as libc::c_int;
    (*serv).next = (*dnsmasq_daemon).servers;
    (*dnsmasq_daemon).servers = serv;
    return serv;
}
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
unsafe extern "C" fn set_prefix(mut arg: *mut libc::c_char)
 -> *mut libc::c_char {
    if strstr(arg, b"set:\x00" as *const u8 as *const libc::c_char) == arg {
        return arg.offset(4 as libc::c_int as isize)
    }
    return arg;
}
unsafe extern "C" fn dhcp_netid_create(mut net: *const libc::c_char,
                                       mut next: *mut dhcp_netid)
 -> *mut dhcp_netid {
    let mut tt: *mut dhcp_netid = 0 as *mut dhcp_netid;
    tt =
        opt_malloc(::std::mem::size_of::<dhcp_netid>() as libc::c_ulong) as
            *mut dhcp_netid;
    (*tt).net = opt_string_alloc(net);
    (*tt).next = next;
    return tt;
}
unsafe extern "C" fn dhcp_netid_free(mut nid: *mut dhcp_netid) {
    while !nid.is_null() {
        let mut tmp: *mut dhcp_netid = nid;
        nid = (*nid).next;
        free((*tmp).net as *mut libc::c_void);
        free(tmp as *mut libc::c_void);
    };
}
/* Parse one or more tag:s before parameters.
 * Moves arg to the end of tags. */
unsafe extern "C" fn dhcp_tags(mut arg: *mut *mut libc::c_char)
 -> *mut dhcp_netid {
    let mut id: *mut dhcp_netid = 0 as *mut dhcp_netid;
    while is_tag_prefix(*arg) != 0 {
        let mut comma: *mut libc::c_char = split(*arg);
        id = dhcp_netid_create((*arg).offset(4 as libc::c_int as isize), id);
        *arg = comma
    }
    if (*arg).is_null() { dhcp_netid_free(id); id = 0 as *mut dhcp_netid }
    return id;
}
unsafe extern "C" fn dhcp_netid_list_free(mut netid: *mut dhcp_netid_list) {
    while !netid.is_null() {
        let mut tmplist: *mut dhcp_netid_list = netid;
        netid = (*netid).next;
        dhcp_netid_free((*tmplist).list);
        free(tmplist as *mut libc::c_void);
    };
}
unsafe extern "C" fn dhcp_config_free(mut config: *mut dhcp_config) {
    if !config.is_null() {
        let mut hwaddr: *mut hwaddr_config = (*config).hwaddr;
        while !hwaddr.is_null() {
            let mut tmp: *mut hwaddr_config = hwaddr;
            hwaddr = (*hwaddr).next;
            free(tmp as *mut libc::c_void);
        }
        dhcp_netid_list_free((*config).netid);
        dhcp_netid_free((*config).filter);
        if (*config).flags & 2 as libc::c_int as libc::c_uint != 0 {
            free((*config).clid as *mut libc::c_void);
        }
        if (*config).flags & 4096 as libc::c_int as libc::c_uint != 0 {
            let mut addr: *mut addrlist = 0 as *mut addrlist;
            let mut tmp_0: *mut addrlist = 0 as *mut addrlist;
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
unsafe extern "C" fn dhcp_context_free(mut ctx: *mut dhcp_context) {
    if !ctx.is_null() {
        dhcp_netid_free((*ctx).filter);
        free((*ctx).netid.net as *mut libc::c_void);
        free((*ctx).template_interface as *mut libc::c_void);
        free(ctx as *mut libc::c_void);
    };
}
unsafe extern "C" fn dhcp_opt_free(mut opt: *mut dhcp_opt) {
    if (*opt).flags & 256 as libc::c_int != 0 {
        free((*opt).u.vendor_class as *mut libc::c_void);
    }
    dhcp_netid_free((*opt).netid);
    free((*opt).val as *mut libc::c_void);
    free(opt as *mut libc::c_void);
}
/* This is too insanely large to keep in-line in the switch */
unsafe extern "C" fn parse_dhcp_opt(mut errstr: *mut libc::c_char,
                                    mut arg: *mut libc::c_char,
                                    mut flags: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut new: *mut dhcp_opt =
        opt_malloc(::std::mem::size_of::<dhcp_opt>() as libc::c_ulong) as
            *mut dhcp_opt;
    let mut lenchar: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addrs: libc::c_int = 0;
    let mut digs: libc::c_int = 0;
    let mut is_addr: libc::c_int = 0;
    let mut is_addr6: libc::c_int = 0;
    let mut is_hex: libc::c_int = 0;
    let mut is_dec: libc::c_int = 0;
    let mut is_string: libc::c_int = 0;
    let mut dots: libc::c_int = 0;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opt_len: u16_0 = 0 as libc::c_int as u16_0;
    let mut is6: libc::c_int = 0 as libc::c_int;
    let mut option_ok: libc::c_int = 0 as libc::c_int;
    (*new).len = 0 as libc::c_int;
    (*new).flags = flags;
    (*new).netid = 0 as *mut dhcp_netid;
    (*new).val = 0 as *mut libc::c_uchar;
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
                lookup_dhcp_opt(2 as libc::c_int,
                                arg.offset(7 as libc::c_int as isize));
            if (*new).opt != -(1 as libc::c_int) {
                opt_len =
                    lookup_dhcp_len(2 as libc::c_int, (*new).opt) as u16_0;
                /* option:<optname> must follow tag and vendor string. */
                if opt_len as libc::c_int & 0x2000 as libc::c_int == 0 ||
                       flags == 128 as libc::c_int {
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
                    lookup_dhcp_opt(10 as libc::c_int,
                                    arg.offset(8 as libc::c_int as isize));
                if (*new).opt != -(1 as libc::c_int) {
                    opt_len =
                        lookup_dhcp_len(10 as libc::c_int, (*new).opt) as
                            u16_0;
                    if opt_len as libc::c_int & 0x2000 as libc::c_int == 0 ||
                           flags == 128 as libc::c_int {
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
                (*new).flags |= 256 as libc::c_int
            } else if strstr(arg,
                             b"encap:\x00" as *const u8 as
                                 *const libc::c_char) == arg {
                (*new).u.encap = atoi(arg.offset(6 as libc::c_int as isize));
                (*new).flags |= 4 as libc::c_int
            } else if strstr(arg,
                             b"vi-encap:\x00" as *const u8 as
                                 *const libc::c_char) == arg {
                (*new).u.encap = atoi(arg.offset(9 as libc::c_int as isize));
                (*new).flags |= 2048 as libc::c_int;
                if flags == 128 as libc::c_int {
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
        if (*new).flags & (256 as libc::c_int | 4 as libc::c_int) != 0 {
            strcpy(errstr,
                   b"unsupported encapsulation for IPv6 option\x00" as
                       *const u8 as *const libc::c_char);
            current_block = 14151404249770905673;
        } else {
            if opt_len as libc::c_int == 0 as libc::c_int &&
                   (*new).flags & 2048 as libc::c_int == 0 {
                opt_len =
                    lookup_dhcp_len(10 as libc::c_int, (*new).opt) as u16_0
            }
            current_block = 317151059986244064;
        }
    } else {
        if opt_len as libc::c_int == 0 as libc::c_int &&
               (*new).flags &
                   (256 as libc::c_int | 4 as libc::c_int |
                        2048 as libc::c_int) == 0 {
            opt_len = lookup_dhcp_len(2 as libc::c_int, (*new).opt) as u16_0
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
                    let mut found_dig: libc::c_int = 0 as libc::c_int;
                    let mut found_colon: libc::c_int = 0 as libc::c_int;
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
                                         flags & 128 as libc::c_int != 0) {
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
                    if is6 != 0 && (*new).opt == 56 as libc::c_int &&
                           is_hex == 0 {
                        opt_len =
                            (opt_len as libc::c_int |
                                 if is_addr6 != 0 {
                                     0x8000 as libc::c_int
                                 } else { 0x4000 as libc::c_int }) as u16_0
                    }
                    /* We know that some options take addresses */
                    if opt_len as libc::c_int & 0x8000 as libc::c_int != 0 {
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
                               (0x1000 as libc::c_int | 0x4000 as libc::c_int
                                    | 0x800 as libc::c_int) != 0 {
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
                                   opt_len as libc::c_int &
                                       0x200 as libc::c_int != 0 &&
                                   strlen(comma) >
                                       0 as libc::c_int as libc::c_ulong {
                                let mut val: libc::c_int = 0;
                                let mut fac: libc::c_int = 1 as libc::c_int;
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
                                          if flags & 128 as libc::c_int != 0 {
                                              &mut (*new).u.wildcard_mask
                                          } else { 0 as *mut libc::c_uint },
                                          0 as *mut libc::c_int);
                                (*new).flags |= 512 as libc::c_int;
                                current_block = 15439134456549723682;
                            } else if is_dec != 0 {
                                let mut i: libc::c_int = 0;
                                let mut val_0: libc::c_int = atoi(comma);
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
                                let mut in_0: in_addr = in_addr{s_addr: 0,};
                                let mut op: *mut libc::c_uchar =
                                    0 as *mut libc::c_uchar;
                                let mut slash: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                /* max length of address/subnet descriptor is five bytes,
	     add one for the option 120 enc byte too */
                                op =
                                    opt_malloc((5 as libc::c_int * addrs +
                                                    1 as libc::c_int) as
                                                   size_t) as
                                        *mut libc::c_uchar; /* RFC 3361 "enc byte" */
                                (*new).val = op;
                                (*new).flags |= 1 as libc::c_int;
                                if (*new).flags &
                                       (4 as libc::c_int | 256 as libc::c_int
                                            | 2048 as libc::c_int) == 0 &&
                                       (*new).opt == 120 as libc::c_int {
                                    let fresh6 = op;
                                    op = op.offset(1);
                                    *fresh6 =
                                        1 as libc::c_int as libc::c_uchar;
                                    (*new).flags &= !(1 as libc::c_int)
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
                                    if inet_pton(2 as libc::c_int, cp,
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
                                               4 as libc::c_int as
                                                   libc::c_ulong);
                                        op =
                                            op.offset(4 as libc::c_int as
                                                          isize)
                                    } else {
                                        let mut p: *mut libc::c_uchar =
                                            &mut in_0 as *mut in_addr as
                                                *mut libc::c_uchar;
                                        let mut netsize: libc::c_int =
                                            atoi(slash);
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
                                        (*new).flags &= !(1 as libc::c_int)
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
                                let mut op_0: *mut libc::c_uchar =
                                    0 as *mut libc::c_uchar;
                                op_0 =
                                    opt_malloc((16 as libc::c_int * addrs) as
                                                   size_t) as
                                        *mut libc::c_uchar;
                                (*new).val = op_0;
                                (*new).flags |= 8192 as libc::c_int;
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
                                    if inet_pton(10 as libc::c_int, cp,
                                                 op_0 as *mut libc::c_void) !=
                                           0 {
                                        op_0 =
                                            op_0.offset(16 as libc::c_int as
                                                            isize)
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
                                if ((*new).opt == 119 as libc::c_int ||
                                        (*new).opt == 120 as libc::c_int) &&
                                       is6 == 0 &&
                                       (*new).flags &
                                           (4 as libc::c_int |
                                                256 as libc::c_int |
                                                2048 as libc::c_int) == 0 {
                                    /* dns search, RFC 3397, or SIP, RFC 3361 */
                                    let mut q: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut r: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut tail: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut p_0: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut m: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newp: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newlen: size_t = 0;
                                    let mut len: size_t =
                                        0 as libc::c_int as size_t;
                                    let mut header_size: libc::c_int =
                                        if (*new).opt == 119 as libc::c_int {
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
                                        let mut in_1: *mut libc::c_char =
                                            0 as *mut libc::c_char;
                                        let mut dom: *mut libc::c_char =
                                            0 as *mut libc::c_char;
                                        let mut domlen: size_t =
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
                                            let mut cp_0: *mut libc::c_uchar =
                                                fresh18;
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
                                                        let mut t_s: u16_0 =
                                                            (r.wrapping_offset_from(p_0)
                                                                 as
                                                                 libc::c_long
                                                                 |
                                                                 0xc000 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_long)
                                                                as u16_0;
                                                        let mut t_cp:
                                                                *mut libc::c_uchar =
                                                            tail;
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
                                            if (*new).opt ==
                                                   120 as libc::c_int &&
                                                   !m.is_null() {
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
                                                  0x800 as libc::c_int != 0 {
                                    /* length fields are two bytes so need 16 bits for each string */
                                    let mut i_0: libc::c_int = 0;
                                    let mut commas: libc::c_int =
                                        1 as libc::c_int;
                                    let mut p_1: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newp_0: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
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
                                        let mut len_0: u16_0 =
                                            strlen(arg) as u16_0;
                                        unhide_metas(arg);
                                        let mut t_s_0: u16_0 = len_0;
                                        let mut t_cp_0: *mut libc::c_uchar =
                                            p_1;
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
                                                  0x4000 as libc::c_int != 0 {
                                    let mut p_2: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut q_0: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newp_1: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut end: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut len_1: libc::c_int =
                                        0 as libc::c_int;
                                    let mut header_size_0: libc::c_int =
                                        if is6 != 0 &&
                                               (*new).opt == 56 as libc::c_int
                                           {
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
                                        let mut dom_0: *mut libc::c_char =
                                            canonicalise_opt(arg);
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
                                                                0 as
                                                                    *mut libc::c_char);
                                            let fresh23 = end;
                                            end = end.offset(1);
                                            *fresh23 =
                                                0 as libc::c_int as
                                                    libc::c_uchar;
                                            if is6 != 0 &&
                                                   (*new).opt ==
                                                       56 as libc::c_int {
                                                let mut t_s_1: u16_0 =
                                                    3 as libc::c_int as u16_0;
                                                let mut t_cp_1:
                                                        *mut libc::c_uchar =
                                                    q_0;
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
                                                let mut t_s_2: u16_0 =
                                                    (end.wrapping_offset_from(q_0)
                                                         as libc::c_long -
                                                         2 as libc::c_int as
                                                             libc::c_long) as
                                                        u16_0;
                                                let mut t_cp_2:
                                                        *mut libc::c_uchar =
                                                    q_0;
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
                                    (*new).flags |= 2 as libc::c_int;
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
                                            (256 as libc::c_int |
                                                 4 as libc::c_int) != 0 ||
                                    (*new).len > 250 as libc::c_int &&
                                        (*new).flags & 2048 as libc::c_int !=
                                            0) {
                            strcpy(errstr,
                                   b"dhcp-option too long\x00" as *const u8 as
                                       *const libc::c_char);
                        } else {
                            if flags == 128 as libc::c_int {
                                if (*new).flags &
                                       (4 as libc::c_int | 256 as libc::c_int)
                                       != 0 || (*new).netid.is_null() ||
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
unsafe extern "C" fn server_list_free(mut list: *mut server) {
    while !list.is_null() {
        let mut tmp: *mut server = list;
        list = (*list).next;
        free(tmp as *mut libc::c_void);
    };
}
unsafe extern "C" fn one_opt(mut option: libc::c_int,
                             mut arg: *mut libc::c_char,
                             mut errstr: *mut libc::c_char,
                             mut gen_err: *mut libc::c_char,
                             mut command_line: libc::c_int,
                             mut servers_only: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    if option == '?' as i32 {
        strcpy(errstr, gen_err);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while usage[i as usize].opt != 0 as libc::c_int {
        if usage[i as usize].opt == option {
            let mut rept: libc::c_int = usage[i as usize].rept as libc::c_int;
            if command_line != 0 {
                /* command line */
                if rept == 62 as libc::c_int + 2 as libc::c_int {
                    strcpy(errstr,
                           b"illegal repeated flag\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if rept == 62 as libc::c_int + 1 as libc::c_int {
                    usage[i as usize].rept =
                        (62 as libc::c_int + 2 as libc::c_int) as libc::c_uint
                }
            } else {
                /* allow file to override command line */
                if rept == 62 as libc::c_int + 3 as libc::c_int {
                    strcpy(errstr,
                           b"illegal repeated keyword\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
                if rept == 62 as libc::c_int + 2 as libc::c_int ||
                       rept == 62 as libc::c_int + 1 as libc::c_int {
                    usage[i as usize].rept =
                        (62 as libc::c_int + 3 as libc::c_int) as libc::c_uint
                }
            }
            if rept != 62 as libc::c_int &&
                   rept != 62 as libc::c_int + 1 as libc::c_int &&
                   rept != 62 as libc::c_int + 2 as libc::c_int {
                set_option_bool(rept as libc::c_uint);
                return 1 as libc::c_int
            }
            break ;
        } else { i += 1 }
    }
    match option {
        67 => {
            /* --conf-file */
            let mut file: *mut libc::c_char = opt_string_alloc(arg);
            if !file.is_null() {
                one_file(file, 0 as libc::c_int);
                free(file as *mut libc::c_void);
            }
            current_block = 7879481898411272068;
        }
        55 => {
            /* --conf-dir */
            let mut dir_stream: *mut DIR = 0 as *mut DIR;
            let mut ent: *mut dirent = 0 as *mut dirent;
            let mut directory: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ignore_suffix: *mut list = 0 as *mut list;
            let mut match_suffix: *mut list = 0 as *mut list;
            let mut files: *mut list = 0 as *mut list;
            let mut li: *mut list = 0 as *mut list;
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
                        directory, 3 as libc::c_int);
                }
                loop  {
                    ent = readdir(dir_stream);
                    if ent.is_null() { break ; }
                    let mut len: size_t = strlen((*ent).d_name.as_mut_ptr());
                    let mut buf: stat =
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
                            let mut ls: size_t = strlen((*li).name);
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
                        let mut ls_0: size_t = strlen((*li).name);
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
                            path, 3 as libc::c_int);
                    }
                    /* only reg files allowed. */
                    if buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                           == 0o100000 as libc::c_int as libc::c_uint {
                        /* sort files into order. */
                        let mut up: *mut *mut list = 0 as *mut *mut list;
                        let mut new: *mut list =
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
        325 => {
            /* --add-subnet */
            set_option_bool(41 as libc::c_int as libc::c_uint);
            if !arg.is_null() {
                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                comma = split(arg);
                let mut new_0: *mut mysubnet =
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
            set_option_bool(19 as libc::c_int as libc::c_uint);
            if !arg.is_null() {
                (*dnsmasq_daemon).dbus_name = opt_string_alloc(arg)
            } else {
                (*dnsmasq_daemon).dbus_name =
                    b"uk.org.thekelleys.dnsmasq\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char
            }
            current_block = 7879481898411272068;
        }
        354 => {
            /* --enable-ubus */
            set_option_bool(58 as libc::c_int as libc::c_uint);
            if !arg.is_null() {
                (*dnsmasq_daemon).ubus_name = opt_string_alloc(arg)
            } else {
                (*dnsmasq_daemon).ubus_name =
                    b"dnsmasq\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
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
            let mut name: *mut libc::c_char = opt_string_alloc(arg);
            let mut new_1: *mut resolvc = 0 as *mut resolvc;
            let mut list_0: *mut resolvc = (*dnsmasq_daemon).resolv_files;
            if !list_0.is_null() && (*list_0).is_default != 0 {
                /* replace default resolv file - possibly with nothing */
                if !name.is_null() {
                    (*list_0).is_default = 0 as libc::c_int;
                    (*list_0).name = name
                } else { list_0 = 0 as *mut resolvc }
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
        333 => {
            (*dnsmasq_daemon).servers_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        109 => {
            /* --mx-host */
            let mut pref: libc::c_int = 1 as libc::c_int; /* may be NULL */
            let mut new_2: *mut mx_srv_record = 0 as *mut mx_srv_record;
            let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
            comma = split(arg);
            if !comma.is_null() {
                let mut prefstr: *mut libc::c_char = 0 as *mut libc::c_char;
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
        352 => {
            /* --dumpfile */
            (*dnsmasq_daemon).dump_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        353 => {
            /* --dumpmask */
            (*dnsmasq_daemon).dump_mask =
                strtol(arg, 0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            current_block = 7879481898411272068;
        }
        108 => {
            /* --dhcp-leasefile */
            (*dnsmasq_daemon).lease_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        54 | 305 => {
            /* --dhcp-script */
            /* --dhcp-luascript */
            if option == 305 as libc::c_int {
                strcpy(errstr,
                       b"recompile with HAVE_LUASCRIPT defined to enable Lua scripts\x00"
                           as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            } else {
                (*dnsmasq_daemon).lease_change_command = opt_string_alloc(arg)
            }
            current_block = 7879481898411272068;
        }
        273 => {
            /* --dhcp-hostsfile */
            current_block = 12010070245366740438;
        }
        280 => { current_block = 12010070245366740438; }
        340 => { current_block = 2812646229686797995; }
        341 => { current_block = 10566976656908717602; }
        342 => { current_block = 2602045500541335152; }
        72 => { current_block = 4533671380017093834; }
        314 => {
            /* --auth-server */
            comma = split(arg);
            (*dnsmasq_daemon).authserver = opt_string_alloc(arg);
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut new_4: *mut iname =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                (*new_4).name = 0 as *mut libc::c_char;
                unhide_metas(arg);
                if inet_pton(2 as libc::c_int, arg,
                             &mut (*new_4).addr.in_0.sin_addr as *mut in_addr
                                 as *mut libc::c_void) > 0 as libc::c_int {
                    (*new_4).addr.sa.sa_family =
                        2 as libc::c_int as sa_family_t
                } else if inet_pton(10 as libc::c_int, arg,
                                    &mut (*new_4).addr.in6.sin6_addr as
                                        *mut in6_addr as *mut libc::c_void) >
                              0 as libc::c_int {
                    (*new_4).addr.sa.sa_family =
                        10 as libc::c_int as sa_family_t
                } else {
                    let mut fam: *mut libc::c_char =
                        split_chr(arg, '/' as i32 as libc::c_char);
                    (*new_4).name = opt_string_alloc(arg);
                    (*new_4).addr.sa.sa_family =
                        0 as libc::c_int as sa_family_t;
                    if !fam.is_null() {
                        if strcmp(fam,
                                  b"4\x00" as *const u8 as
                                      *const libc::c_char) == 0 as libc::c_int
                           {
                            (*new_4).addr.sa.sa_family =
                                2 as libc::c_int as sa_family_t
                        } else if strcmp(fam,
                                         b"6\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 as libc::c_int {
                            (*new_4).addr.sa.sa_family =
                                10 as libc::c_int as sa_family_t
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
        317 => {
            /* --auth-sec-servers */
            let mut new_5: *mut name_list = 0 as *mut name_list;
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
        313 => {
            /* --auth-zone */
            let mut new_6: *mut auth_zone = 0 as *mut auth_zone;
            comma = split(arg);
            new_6 =
                opt_malloc(::std::mem::size_of::<auth_zone>() as
                               libc::c_ulong) as *mut auth_zone;
            (*new_6).domain = opt_string_alloc(arg);
            (*new_6).subnet = 0 as *mut addrlist;
            (*new_6).exclude = 0 as *mut addrlist;
            (*new_6).interface_names = 0 as *mut auth_name_list;
            (*new_6).next = (*dnsmasq_daemon).auth_zones;
            (*dnsmasq_daemon).auth_zones = new_6;
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut prefixlen: libc::c_int = 0 as libc::c_int;
                let mut is_exclude: libc::c_int = 0 as libc::c_int;
                let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut subnet: *mut addrlist = 0 as *mut addrlist;
                let mut addr: all_addr =
                    all_addr{addr4: in_addr{s_addr: 0,},};
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
                if inet_pton(2 as libc::c_int, arg,
                             &mut addr.addr4 as *mut in_addr as
                                 *mut libc::c_void) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<addrlist>() as
                                       libc::c_ulong) as *mut addrlist;
                    (*subnet).prefixlen =
                        if prefixlen == 0 as libc::c_int {
                            24 as libc::c_int
                        } else { prefixlen };
                    (*subnet).flags = 1 as libc::c_int
                } else if inet_pton(10 as libc::c_int, arg,
                                    &mut addr.addr6 as *mut in6_addr as
                                        *mut libc::c_void) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<addrlist>() as
                                       libc::c_ulong) as *mut addrlist;
                    (*subnet).prefixlen =
                        if prefixlen == 0 as libc::c_int {
                            64 as libc::c_int
                        } else { prefixlen };
                    (*subnet).flags = 1 as libc::c_int | 2 as libc::c_int
                } else {
                    let mut name_1: *mut auth_name_list =
                        opt_malloc(::std::mem::size_of::<auth_name_list>() as
                                       libc::c_ulong) as *mut auth_name_list;
                    (*name_1).name = opt_string_alloc(arg);
                    (*name_1).flags = 2 as libc::c_int | 1 as libc::c_int;
                    (*name_1).next = (*new_6).interface_names;
                    (*new_6).interface_names = name_1;
                    if !prefix.is_null() {
                        if prefixlen == 4 as libc::c_int {
                            (*name_1).flags &= !(1 as libc::c_int)
                        } else if prefixlen == 6 as libc::c_int {
                            (*name_1).flags &= !(2 as libc::c_int)
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
        316 => {
            /* --auth-soa */
            comma = split(arg);
            (*dnsmasq_daemon).soa_sn = atoi(arg) as u32_0 as libc::c_ulong;
            if !comma.is_null() {
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
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
        115 | 320 => {
            /* --domain */
            /* --synth-domain */
            if strcmp(arg, b"#\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                set_option_bool(15 as libc::c_int as libc::c_uint);
            } else {
                let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
                comma = split(arg);
                d = canonicalise_opt(arg);
                if d.is_null() {
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                } else {
                    if !comma.is_null() {
                        let mut new_7: *mut cond_domain =
                            opt_malloc(::std::mem::size_of::<cond_domain>() as
                                           libc::c_ulong) as *mut cond_domain;
                        let mut netpart: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        (*new_7).prefix = 0 as *mut libc::c_char;
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
                                if inet_pton(2 as libc::c_int, comma,
                                             &mut (*new_7).start as
                                                 *mut in_addr as
                                                 *mut libc::c_void) != 0 {
                                    let mut mask: libc::c_int =
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
                                                       (63 as libc::c_int -
                                                            16 as libc::c_int)
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
                                            let mut serv: *mut server =
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
                                            (*serv).flags |= 2 as libc::c_int;
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
                                                8 as libc::c_int |
                                                    2 as libc::c_int;
                                            (*serv).next =
                                                (*dnsmasq_daemon).servers;
                                            (*dnsmasq_daemon).servers = serv
                                        }
                                    }
                                } else if inet_pton(10 as libc::c_int, comma,
                                                    &mut (*new_7).start6 as
                                                        *mut in6_addr as
                                                        *mut libc::c_void) !=
                                              0 {
                                    let mut mask_0: u64_0 =
                                        ((1 as libc::c_ulonglong) <<
                                             128 as libc::c_int -
                                                 msize).wrapping_sub(1 as
                                                                         libc::c_ulonglong);
                                    let mut addrpart: u64_0 =
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
                                                           (63 as libc::c_int
                                                                -
                                                                46 as
                                                                    libc::c_int)
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
                                                let mut serv_0: *mut server =
                                                    add_rev6(&mut (*new_7).start6,
                                                             msize);
                                                (*serv_0).flags |=
                                                    2 as libc::c_int;
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
                                                    8 as libc::c_int |
                                                        2 as libc::c_int;
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
                            let mut prefstr_0: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            arg = split(comma);
                            prefstr_0 = split(arg);
                            if inet_pton(2 as libc::c_int, comma,
                                         &mut (*new_7).start as *mut in_addr
                                             as *mut libc::c_void) != 0 {
                                (*new_7).is6 = 0 as libc::c_int;
                                if arg.is_null() {
                                    (*new_7).end.s_addr =
                                        (*new_7).start.s_addr
                                } else if inet_pton(2 as libc::c_int, arg,
                                                    &mut (*new_7).end as
                                                        *mut in_addr as
                                                        *mut libc::c_void) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0 as libc::c_int
                                }
                            } else if inet_pton(10 as libc::c_int, comma,
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
                                           16 as libc::c_int as
                                               libc::c_ulong);
                                } else if inet_pton(10 as libc::c_int, arg,
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
                                           (63 as libc::c_int -
                                                16 as libc::c_int) as
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
                            let mut star: *mut libc::c_char =
                                0 as *mut libc::c_char;
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
        346 => {
            /* --add-dns-client */
            if !arg.is_null() {
                (*dnsmasq_daemon).dns_client_id = opt_string_alloc(arg)
            }
            current_block = 7879481898411272068;
        }
        300 => {
            /* --add-mac */
            if arg.is_null() {
                set_option_bool(32 as libc::c_int as libc::c_uint);
            } else {
                unhide_metas(arg);
                if strcmp(arg,
                          b"base64\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    set_option_bool(54 as libc::c_int as libc::c_uint);
                } else if strcmp(arg,
                                 b"text\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    set_option_bool(55 as libc::c_int as libc::c_uint);
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
        285 => {
            /* --scriptuser */
            (*dnsmasq_daemon).scriptuser = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        105 => {
            loop 
                 /* --interface */
                 {
                let mut new_8: *mut iname =
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
        258 => {
            /* --enable-tftp */
            set_option_bool(40 as libc::c_int as libc::c_uint);
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
        338 => { current_block = 2926860427235594157; }
        97 | 318 => {
            /* --listen-address */
            loop 
                 /* --auth-peer */
                 {
                let mut new_10: *mut iname =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                unhide_metas(arg);
                if !arg.is_null() &&
                       inet_pton(2 as libc::c_int, arg,
                                 &mut (*new_10).addr.in_0.sin_addr as
                                     *mut in_addr as *mut libc::c_void) >
                           0 as libc::c_int {
                    (*new_10).addr.sa.sa_family =
                        2 as libc::c_int as sa_family_t;
                    (*new_10).addr.in_0.sin_port =
                        0 as libc::c_int as in_port_t
                } else if !arg.is_null() &&
                              inet_pton(10 as libc::c_int, arg,
                                        &mut (*new_10).addr.in6.sin6_addr as
                                            *mut in6_addr as
                                            *mut libc::c_void) >
                                  0 as libc::c_int {
                    (*new_10).addr.sa.sa_family =
                        10 as libc::c_int as sa_family_t;
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
        286 => { current_block = 9676380469790025234; }
        65 => { current_block = 6480954168551069607; }
        298 => { current_block = 14399141444697241811; }
        332 => {
            /* --rev-server */
            let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut size: libc::c_int = 0;
            let mut serv_2: *mut server = 0 as *mut server;
            let mut addr4: in_addr = in_addr{s_addr: 0,};
            let mut addr6: in6_addr =
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
            if inet_pton(2 as libc::c_int, arg,
                         &mut addr4 as *mut in_addr as *mut libc::c_void) != 0
               {
                serv_2 = add_rev4(addr4, size);
                if serv_2.is_null() {
                    strcpy(errstr,
                           b"bad prefix\x00" as *const u8 as
                               *const libc::c_char);
                    return 0 as libc::c_int
                }
            } else if inet_pton(10 as libc::c_int, arg,
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
            if servers_only != 0 { (*serv_2).flags |= 4096 as libc::c_int }
            current_block = 7879481898411272068;
        }
        319 => {
            /* --ipset */
            let mut ipsets_head: ipsets =
                ipsets{sets: 0 as *mut *mut libc::c_char,
                       domain: 0 as *mut libc::c_char,
                       next: 0 as *mut ipsets,};
            let mut ipsets: *mut ipsets = &mut ipsets_head;
            let mut size_0: libc::c_int = 0;
            let mut end_1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut sets: *mut *mut libc::c_char =
                0 as *mut *mut libc::c_char;
            let mut sets_pos: *mut *mut libc::c_char =
                0 as *mut *mut libc::c_char;
            memset(ipsets as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<ipsets>() as libc::c_ulong);
            unhide_metas(arg);
            if !arg.is_null() && *arg as libc::c_int == '/' as i32 {
                arg = arg.offset(1);
                loop  {
                    end_1 = split_chr(arg, '/' as i32 as libc::c_char);
                    if end_1.is_null() { break ; }
                    let mut domain_0: *mut libc::c_char =
                        0 as *mut libc::c_char;
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
        288 => {
            /* --min-port */
            if atoi_check16(arg, &mut (*dnsmasq_daemon).min_port) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        345 => {
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
            set_option_bool(2 as libc::c_int as libc::c_uint); /* default */
            if !arg.is_null() &&
                   strcmp(arg,
                          b"extra\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                set_option_bool(51 as libc::c_int as libc::c_uint);
            }
            current_block = 7879481898411272068;
        }
        267 => {
            /* --log-async */
            (*dnsmasq_daemon).max_logs = 5 as libc::c_int;
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
        283 => { current_block = 15489771604880449635; }
        297 => { current_block = 6082976577402880686; }
        339 => { current_block = 16916584745428150692; }
        312 => { current_block = 13094692781038244044; }
        315 => { current_block = 13035992208579083528; }
        348 => { current_block = 5893551302610724882; }
        88 => {
            /* --dhcp-lease-max */
            if atoi_check(arg, &mut (*dnsmasq_daemon).dhcp_max) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        263 => {
            /*  --tftp-max */
            if atoi_check(arg, &mut (*dnsmasq_daemon).tftp_max) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        349 => {
            /*  --tftp-mtu */
            if atoi_check(arg, &mut (*dnsmasq_daemon).tftp_mtu) == 0 {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            }
            current_block = 7879481898411272068;
        }
        260 => {
            /* --tftp-prefix */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_11: *mut tftp_prefix =
                    opt_malloc(::std::mem::size_of::<tftp_prefix>() as
                                   libc::c_ulong) as *mut tftp_prefix;
                (*new_11).interface = opt_string_alloc(comma);
                (*new_11).prefix = opt_string_alloc(arg);
                (*new_11).next = (*dnsmasq_daemon).if_prefix;
                (*dnsmasq_daemon).if_prefix = new_11
            } else { (*dnsmasq_daemon).tftp_prefix = opt_string_alloc(arg) }
            current_block = 7879481898411272068;
        }
        276 => {
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
                let mut tmp: libc::c_int = (*dnsmasq_daemon).start_tftp_port;
                (*dnsmasq_daemon).start_tftp_port =
                    (*dnsmasq_daemon).end_tftp_port;
                (*dnsmasq_daemon).end_tftp_port = tmp
            }
            current_block = 7879481898411272068;
        }
        274 => {
            /* --tftp-unique-root */
            if arg.is_null() ||
                   strcasecmp(arg,
                              b"ip\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                set_option_bool(29 as libc::c_int as libc::c_uint);
            } else if strcasecmp(arg,
                                 b"mac\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
             {
                set_option_bool(56 as libc::c_int as libc::c_uint);
            } else { strcpy(errstr, gen_err); return 0 as libc::c_int }
            current_block = 7879481898411272068;
        }
        262 => {
            /* --bridge-interface */
            let mut new_12: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
            comma = split(arg);
            if comma.is_null() ||
                   strlen(arg) >
                       (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
               {
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
                (*new_12).alias = 0 as *mut dhcp_bridge;
                (*new_12).next = (*dnsmasq_daemon).bridges;
                (*dnsmasq_daemon).bridges = new_12
            }
            loop  {
                arg = comma;
                comma = split(arg);
                if strlen(arg) != 0 as libc::c_int as libc::c_ulong &&
                       strlen(arg) <=
                           (16 as libc::c_int - 1 as libc::c_int) as
                               libc::c_ulong {
                    let mut b: *mut dhcp_bridge =
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
        357 => {
            /* --shared-network */
            let mut new_13: *mut shared_network =
                opt_malloc(::std::mem::size_of::<shared_network>() as
                               libc::c_ulong) as *mut shared_network;
            (*new_13).shared_addr.s_addr = 0 as libc::c_int as in_addr_t;
            (*new_13).if_index = 0 as libc::c_int;
            comma = split(arg);
            if comma.is_null() {
                current_block = 3177757304694473968;
            } else {
                if inet_pton(2 as libc::c_int, comma,
                             &mut (*new_13).shared_addr as *mut in_addr as
                                 *mut libc::c_void) != 0 {
                    if inet_pton(2 as libc::c_int, arg,
                                 &mut (*new_13).match_addr as *mut in_addr as
                                     *mut libc::c_void) == 0 &&
                           {
                               (*new_13).if_index =
                                   if_nametoindex(arg) as libc::c_int;
                               ((*new_13).if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else if inet_pton(10 as libc::c_int, comma,
                                    &mut (*new_13).shared_addr6 as
                                        *mut in6_addr as *mut libc::c_void) !=
                              0 {
                    if inet_pton(10 as libc::c_int, arg,
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
            let mut leasepos: libc::c_int = 2 as libc::c_int;
            let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut a: [*mut libc::c_char; 8] =
                [0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char];
            let mut new_14: *mut dhcp_context =
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
            if inet_pton(2 as libc::c_int, a[0 as libc::c_int as usize],
                         &mut (*new_14).start as *mut in_addr as
                             *mut libc::c_void) != 0 {
                (*new_14).next = (*dnsmasq_daemon).dhcp;
                (*new_14).lease_time = 3600 as libc::c_int as libc::c_uint;
                (*dnsmasq_daemon).dhcp = new_14;
                (*new_14).end = (*new_14).start;
                if strcmp(a[1 as libc::c_int as usize],
                          b"static\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 0 as libc::c_int) as
                            libc::c_int
                } else if strcmp(a[1 as libc::c_int as usize],
                                 b"proxy\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 3 as libc::c_int) as
                            libc::c_int
                } else if inet_pton(2 as libc::c_int,
                                    a[1 as libc::c_int as usize],
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
                    let mut tmp_0: in_addr = (*new_14).start;
                    (*new_14).start = (*new_14).end;
                    (*new_14).end = tmp_0
                }
                if k >= 3 as libc::c_int &&
                       !strchr(a[2 as libc::c_int as usize],
                               '.' as i32).is_null() &&
                       inet_pton(2 as libc::c_int,
                                 a[2 as libc::c_int as usize],
                                 &mut (*new_14).netmask as *mut in_addr as
                                     *mut libc::c_void) > 0 as libc::c_int {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 1 as libc::c_int) as
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
                           inet_pton(2 as libc::c_int,
                                     a[3 as libc::c_int as usize],
                                     &mut (*new_14).broadcast as *mut in_addr
                                         as *mut libc::c_void) >
                               0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 2 as libc::c_int) as
                                libc::c_int;
                        leasepos = 4 as libc::c_int
                    }
                }
            } else if inet_pton(10 as libc::c_int,
                                a[0 as libc::c_int as usize],
                                &mut (*new_14).start6 as *mut in6_addr as
                                    *mut libc::c_void) != 0 {
                let mut err_1: *const libc::c_char = 0 as *const libc::c_char;
                (*new_14).flags =
                    ((*new_14).flags as libc::c_uint |
                         (1 as libc::c_uint) << 17 as libc::c_int) as
                        libc::c_int;
                (*new_14).prefix = 64 as libc::c_int;
                (*new_14).end6 = (*new_14).start6;
                (*new_14).lease_time =
                    (3600 as libc::c_int * 24 as libc::c_int) as libc::c_uint;
                (*new_14).next = (*dnsmasq_daemon).dhcp6;
                (*dnsmasq_daemon).dhcp6 = new_14;
                leasepos = 1 as libc::c_int;
                while leasepos < k {
                    if strcmp(a[leasepos as usize],
                              b"static\x00" as *const u8 as
                                  *const libc::c_char) == 0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 0 as libc::c_int |
                                      (1 as libc::c_uint) <<
                                          8 as libc::c_int)) as libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-only\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int ||
                                  strcmp(a[leasepos as usize],
                                         b"slaac\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 13 as libc::c_int) as
                                libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-names\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 6 as libc::c_int |
                                      (1 as libc::c_uint) <<
                                          13 as libc::c_int)) as libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-advrouter\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 4 as libc::c_int |
                                      (1 as libc::c_uint) <<
                                          13 as libc::c_int)) as libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-stateless\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 7 as libc::c_int |
                                      (1 as libc::c_uint) << 8 as libc::c_int
                                      |
                                      (1 as libc::c_uint) <<
                                          13 as libc::c_int)) as libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"off-link\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 18 as libc::c_int) as
                                libc::c_int
                    } else if leasepos == 1 as libc::c_int &&
                                  inet_pton(10 as libc::c_int,
                                            a[leasepos as usize],
                                            &mut (*new_14).end6 as
                                                *mut in6_addr as
                                                *mut libc::c_void) != 0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 8 as libc::c_int) as
                                libc::c_int
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
                                 (1 as libc::c_uint) << 10 as libc::c_int) as
                                libc::c_int
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
                    if (*new_14).flags as libc::c_uint &
                           (1 as libc::c_uint) << 13 as libc::c_int != 0 {
                        err_1 =
                            b"prefix length must be exactly 64 for RA subnets\x00"
                                as *const u8 as *const libc::c_char
                    } else if (*new_14).flags as libc::c_uint &
                                  (1 as libc::c_uint) << 10 as libc::c_int !=
                                  0 {
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
                        let mut __a: *const in6_addr =
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
                       (*new_14).flags as libc::c_uint &
                           (1 as libc::c_uint) << 10 as libc::c_int == 0 {
                    (*new_14).prefix = 0 as libc::c_int
                }
                if (*new_14).flags as libc::c_uint &
                       (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                    let mut zero: in6_addr =
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
                    let mut tmp_1: in6_addr = (*new_14).start6;
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
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 19 as libc::c_int) as
                            libc::c_int
                } else if strcmp(a[leasepos as usize],
                                 b"deprecated\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 9 as libc::c_int) as
                            libc::c_int
                } else {
                    let mut fac: libc::c_int = 1 as libc::c_int;
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
                                 (1 as libc::c_uint) << 19 as libc::c_int) as
                                libc::c_int;
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
        272 | 71 => {
            /* --dhcp-host */
            let mut new_15: *mut dhcp_config = 0 as *mut dhcp_config;
            let mut in_0: in_addr = in_addr{s_addr: 0,};
            new_15 =
                opt_malloc(::std::mem::size_of::<dhcp_config>() as
                               libc::c_ulong) as *mut dhcp_config;
            (*new_15).next = (*dnsmasq_daemon).dhcp_conf;
            (*new_15).flags =
                if option == 272 as libc::c_int {
                    2048 as libc::c_int
                } else { 0 as libc::c_int } as libc::c_uint;
            (*new_15).hwaddr = 0 as *mut hwaddr_config;
            (*new_15).netid = 0 as *mut dhcp_netid_list;
            (*new_15).filter = 0 as *mut dhcp_netid;
            (*new_15).clid = 0 as *mut libc::c_uchar;
            (*new_15).addr6 = 0 as *mut addrlist;
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
                            (*new_15).flags |=
                                128 as libc::c_int as libc::c_uint
                        } else {
                            let mut len_0: libc::c_int = 0; /* dump id: */
                            arg = arg.offset(3 as libc::c_int as isize);
                            if !strchr(arg, ':' as i32).is_null() {
                                len_0 =
                                    parse_hex(arg, arg as *mut libc::c_uchar,
                                              -(1 as libc::c_int),
                                              0 as *mut libc::c_uint,
                                              0 as *mut libc::c_int)
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
                                        2 as libc::c_int as libc::c_uint;
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
                        let mut newlist_0: *mut dhcp_netid_list =
                            opt_malloc(::std::mem::size_of::<dhcp_netid_list>()
                                           as libc::c_ulong) as
                                *mut dhcp_netid_list;
                        (*newlist_0).next = (*new_15).netid;
                        (*new_15).netid = newlist_0;
                        (*newlist_0).list =
                            dhcp_netid_create(arg.offset(4 as libc::c_int as
                                                             isize),
                                              0 as *mut dhcp_netid)
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
                        let mut pref_1: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        let mut in6: in6_addr =
                            in6_addr{__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},};
                        let mut new_addr: *mut addrlist = 0 as *mut addrlist;
                        *arg.offset(strlen(arg).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                        as isize) =
                            0 as libc::c_int as libc::c_char;
                        arg = arg.offset(1);
                        pref_1 = split_chr(arg, '/' as i32 as libc::c_char);
                        if inet_pton(10 as libc::c_int, arg,
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
                            let mut addrpart_0: u64_0 = addr6part(&mut in6);
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
                            (*new_addr).flags |= 8 as libc::c_int
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
                            (*new_addr).flags |= 16 as libc::c_int
                        }
                        (*new_15).flags |= 4096 as libc::c_int as libc::c_uint
                    } else {
                        let mut newhw: *mut hwaddr_config =
                            opt_malloc(::std::mem::size_of::<hwaddr_config>()
                                           as libc::c_ulong) as
                                *mut hwaddr_config;
                        (*newhw).hwaddr_len =
                            parse_hex(arg, (*newhw).hwaddr.as_mut_ptr(),
                                      16 as libc::c_int,
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
                              inet_pton(2 as libc::c_int, arg,
                                        &mut in_0 as *mut in_addr as
                                            *mut libc::c_void) >
                                  0 as libc::c_int {
                    let mut configs: *mut dhcp_config = 0 as *mut dhcp_config;
                    (*new_15).addr = in_0;
                    (*new_15).flags |= 32 as libc::c_int as libc::c_uint;
                    /* If the same IP appears in more than one host config, then DISCOVER
		   for one of the hosts will get the address, but REQUEST will be NAKed,
		   since the address is reserved by the other one -> protocol loop. */
                    configs = (*dnsmasq_daemon).dhcp_conf;
                    while !configs.is_null() {
                        if (*configs).flags &
                               32 as libc::c_int as libc::c_uint != 0 &&
                               (*configs).addr.s_addr == in_0.s_addr {
                            sprintf(errstr,
                                    b"duplicate dhcp-host IP address %s\x00"
                                        as *const u8 as *const libc::c_char,
                                    inet_ntoa(in_0));
                            return 0 as libc::c_int
                        }
                        configs = (*configs).next
                    }
                } else {
                    let mut cp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut lastp: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut last: libc::c_char =
                        0 as libc::c_int as libc::c_char;
                    let mut fac_0: libc::c_int = 1 as libc::c_int;
                    let mut isdig: libc::c_int = 0 as libc::c_int;
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
                            (*new_15).flags |=
                                8 as libc::c_int as libc::c_uint
                        } else if strcmp(arg,
                                         b"ignore\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 as libc::c_int {
                            (*new_15).flags |=
                                1 as libc::c_int as libc::c_uint
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
                            (*new_15).flags |=
                                16 as libc::c_int as libc::c_uint;
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
                        (*new_15).flags |= 8 as libc::c_int as libc::c_uint
                    }
                }
                arg = comma
            }
            (*dnsmasq_daemon).dhcp_conf = new_15;
            current_block = 7879481898411272068;
        }
        294 => {
            /* --tag-if */
            let mut new_16: *mut tag_if =
                opt_malloc(::std::mem::size_of::<tag_if>() as libc::c_ulong)
                    as *mut tag_if;
            (*new_16).tag = 0 as *mut dhcp_netid;
            (*new_16).set = 0 as *mut dhcp_netid_list;
            (*new_16).next = 0 as *mut tag_if;
            /* preserve order */
            if (*dnsmasq_daemon).tag_if.is_null() {
                (*dnsmasq_daemon).tag_if = new_16
            } else {
                let mut tmp_2: *mut tag_if = 0 as *mut tag_if;
                tmp_2 = (*dnsmasq_daemon).tag_if;
                while !(*tmp_2).next.is_null() { tmp_2 = (*tmp_2).next }
                (*tmp_2).next = new_16
            }
            while !arg.is_null() {
                let mut len_1: size_t = 0;
                comma = split(arg);
                len_1 = strlen(arg);
                if len_1 < 5 as libc::c_int as libc::c_ulong {
                    (*new_16).set = 0 as *mut dhcp_netid_list;
                    break ;
                } else {
                    let mut newtag: *mut dhcp_netid =
                        dhcp_netid_create(arg.offset(4 as libc::c_int as
                                                         isize),
                                          0 as *mut dhcp_netid);
                    if strstr(arg,
                              b"set:\x00" as *const u8 as *const libc::c_char)
                           == arg {
                        let mut newlist_1: *mut dhcp_netid_list =
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
                        (*new_16).set = 0 as *mut dhcp_netid_list;
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
        264 | 279 | 281 => { current_block = 18295461473828413614; }
        355 => {
            /* --dhcp-name-match */
            let mut new_17: *mut dhcp_match_name =
                opt_malloc(::std::mem::size_of::<dhcp_match_name>() as
                               libc::c_ulong) as *mut dhcp_match_name;
            let mut id: *mut dhcp_netid =
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
            let mut id_0: *mut dhcp_netid = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                let mut dhcp_file: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut dhcp_sname: *mut libc::c_char =
                    0 as *mut libc::c_char;
                let mut tftp_sname: *mut libc::c_char =
                    0 as *mut libc::c_char;
                let mut dhcp_next_server: in_addr = in_addr{s_addr: 0,};
                let mut new_18: *mut dhcp_boot = 0 as *mut dhcp_boot;
                comma = split(arg);
                dhcp_file = opt_string_alloc(arg);
                dhcp_next_server.s_addr = 0 as libc::c_int as in_addr_t;
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    dhcp_sname = opt_string_alloc(arg);
                    if !comma.is_null() {
                        unhide_metas(comma);
                        if !(inet_pton(2 as libc::c_int, comma,
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
        350 => {
            /* --dhcp-reply-delay */
            let mut id_1: *mut dhcp_netid = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0 as libc::c_int
            } else {
                let mut new_19: *mut delay_config = 0 as *mut delay_config;
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
        291 => {
            /* --pxe-prompt */
            let mut new_20: *mut dhcp_opt =
                opt_malloc(::std::mem::size_of::<dhcp_opt>() as libc::c_ulong)
                    as *mut dhcp_opt; /* PXE_MENU_PROMPT */
            let mut timeout: libc::c_int = 0;
            (*new_20).netid = 0 as *mut dhcp_netid;
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
                (*new_20).u.vendor_class = 0 as *mut libc::c_uchar;
                (*new_20).flags = 256 as libc::c_int | 16384 as libc::c_int;
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
        292 => {
            /* --pxe-service */
            let mut new_21: *mut pxe_service =
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
                     *mut libc::c_char, 0 as *mut libc::c_char];
            static mut boottype: libc::c_int = 32768 as libc::c_int;
            (*new_21).netid = 0 as *mut dhcp_netid;
            (*new_21).sname = 0 as *mut libc::c_char;
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
                        (*new_21).basename = 0 as *mut libc::c_char
                    } else {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check(arg, &mut i) != 0 {
                            (*new_21).type_0 = i as libc::c_ushort;
                            (*new_21).basename = 0 as *mut libc::c_char
                        } else {
                            let fresh28 = boottype;
                            boottype = boottype + 1;
                            (*new_21).type_0 = fresh28 as libc::c_ushort;
                            (*new_21).basename = opt_string_alloc(arg)
                        }
                        if !comma.is_null() {
                            if inet_pton(2 as libc::c_int, comma,
                                         &mut (*new_21).server as *mut in_addr
                                             as *mut libc::c_void) == 0 {
                                (*new_21).server.s_addr =
                                    0 as libc::c_int as in_addr_t;
                                (*new_21).sname = opt_string_alloc(comma)
                            }
                        }
                    }
                    /* Order matters */
                    (*new_21).next = 0 as *mut pxe_service;
                    if (*dnsmasq_daemon).pxe_services.is_null() {
                        (*dnsmasq_daemon).pxe_services = new_21
                    } else {
                        let mut s: *mut pxe_service = 0 as *mut pxe_service;
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
                let mut new_22: *mut dhcp_mac =
                    opt_malloc(::std::mem::size_of::<dhcp_mac>() as
                                   libc::c_ulong) as *mut dhcp_mac;
                (*new_22).netid.net = opt_string_alloc(set_prefix(arg));
                unhide_metas(comma);
                (*new_22).hwaddr_len =
                    parse_hex(comma, (*new_22).hwaddr.as_mut_ptr(),
                              16 as libc::c_int, &mut (*new_22).mask,
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
        268 => { current_block = 17332795835978703980; }
        269 => { current_block = 15503158355981179141; }
        270 => { current_block = 9763990383449182594; }
        284 => {
            /* --dhcp-alternate-port */
            if arg.is_null() {
                (*dnsmasq_daemon).dhcp_server_port = 1067 as libc::c_int;
                (*dnsmasq_daemon).dhcp_client_port = 1068 as libc::c_int
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
        257 => { current_block = 8728755645494476224; }
        282 => { current_block = 9783966086509161201; }
        51 => { current_block = 9535337827963792624; }
        296 => { current_block = 8762260891357387630; }
        295 => {
            /* --dhcp-proxy */
            (*dnsmasq_daemon).override_0 = 1 as libc::c_int;
            while !arg.is_null() {
                let mut new_25: *mut addr_list =
                    opt_malloc(::std::mem::size_of::<addr_list>() as
                                   libc::c_ulong) as *mut addr_list;
                comma = split(arg);
                if !(inet_pton(2 as libc::c_int, arg,
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
        361 => {
            /* --dhcp-pxe-vendor */
            while !arg.is_null() {
                let mut new_26: *mut dhcp_pxe_vendor =
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
        323 => {
            /* --dhcp-relay */
            let mut new_27: *mut dhcp_relay =
                opt_malloc(::std::mem::size_of::<dhcp_relay>() as
                               libc::c_ulong) as *mut dhcp_relay;
            comma = split(arg);
            (*new_27).interface = opt_string_alloc(split(comma));
            (*new_27).iface_index = 0 as libc::c_int;
            if inet_pton(2 as libc::c_int, arg,
                         &mut (*new_27).local as *mut all_addr as
                             *mut libc::c_void) != 0 &&
                   inet_pton(2 as libc::c_int, comma,
                             &mut (*new_27).server as *mut all_addr as
                                 *mut libc::c_void) != 0 {
                (*new_27).next = (*dnsmasq_daemon).relay4;
                (*dnsmasq_daemon).relay4 = new_27
            } else if inet_pton(10 as libc::c_int, arg,
                                &mut (*new_27).local as *mut all_addr as
                                    *mut libc::c_void) != 0 &&
                          inet_pton(10 as libc::c_int, comma,
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
        324 => {
            /* --ra-param */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_28: *mut ra_interface =
                    opt_malloc(::std::mem::size_of::<ra_interface>() as
                                   libc::c_ulong) as *mut ra_interface;
                (*new_28).lifetime = -(1 as libc::c_int);
                (*new_28).prio = 0 as libc::c_int;
                (*new_28).mtu = 0 as libc::c_int;
                (*new_28).mtu_name = 0 as *mut libc::c_char;
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
        307 => {
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
                              0 as *mut libc::c_uint, 0 as *mut libc::c_int)
                        as libc::c_uint;
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
            let mut dash: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut a_0: [*mut libc::c_char; 3] =
                [0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char];
            let mut k_0: libc::c_int = 0 as libc::c_int;
            let mut new_29: *mut doctor =
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
                   !(inet_pton(2 as libc::c_int,
                               a_0[0 as libc::c_int as usize],
                               &mut (*new_29).in_0 as *mut in_addr as
                                   *mut libc::c_void) > 0 as libc::c_int) ||
                   !(inet_pton(2 as libc::c_int,
                               a_0[1 as libc::c_int as usize],
                               &mut (*new_29).out as *mut in_addr as
                                   *mut libc::c_void) > 0 as libc::c_int) ||
                   k_0 == 3 as libc::c_int &&
                       inet_pton(2 as libc::c_int,
                                 a_0[2 as libc::c_int as usize],
                                 &mut (*new_29).mask as *mut in_addr as
                                     *mut libc::c_void) == 0 {
                strcpy(errstr,
                       b"missing address in alias\x00" as *const u8 as
                           *const libc::c_char);
                return 0 as libc::c_int
            }
            if !dash.is_null() &&
                   (!(inet_pton(2 as libc::c_int, dash,
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
        271 => {
            /* --interface-name */
            let mut new_30: *mut interface_name = 0 as *mut interface_name;
            let mut up_0: *mut *mut interface_name =
                0 as *mut *mut interface_name;
            let mut domain_1: *mut libc::c_char = 0 as *mut libc::c_char;
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
            (*new_30).next = 0 as *mut interface_name;
            (*new_30).addr = 0 as *mut addrlist;
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
                    (*new_30).family = 2 as libc::c_int
                } else if strcmp(arg,
                                 b"6\x00" as *const u8 as *const libc::c_char)
                              == 0 as libc::c_int {
                    (*new_30).family = 10 as libc::c_int
                } else {
                    strcpy(errstr, gen_err);
                    free(new_30 as *mut libc::c_void);
                    return 0 as libc::c_int
                }
            }
            (*new_30).intr = opt_string_alloc(comma);
            current_block = 7879481898411272068;
        }
        290 => {
            /* --cname */
            let mut new_31: *mut cname = 0 as *mut cname;
            let mut alias: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut last_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pen: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ttl_0: libc::c_int = -(1 as libc::c_int);
            pen = 0 as *mut libc::c_char;
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
                let mut arglen: libc::c_int = strlen(arg) as libc::c_int;
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
        261 => {
            /* --ptr-record */
            let mut new_32: *mut ptr_record = 0 as *mut ptr_record;
            let mut dom: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target_1: *mut libc::c_char = 0 as *mut libc::c_char;
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
        287 => {
            /* --naptr-record */
            let mut a_1: [*mut libc::c_char; 7] =
                [0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char];
            let mut k_1: libc::c_int = 0 as libc::c_int;
            let mut new_33: *mut naptr = 0 as *mut naptr;
            let mut order: libc::c_int = 0;
            let mut pref_2: libc::c_int = 0;
            let mut name_2: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut replace: *mut libc::c_char = 0 as *mut libc::c_char;
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
        310 => {
            /* dns-rr */
            let mut new_34: *mut txt_record = 0 as *mut txt_record;
            let mut len_3: size_t = 0 as libc::c_int as size_t;
            let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut class: libc::c_int = 0;
            comma = split(arg);
            data = split(comma);
            new_34 =
                opt_malloc(::std::mem::size_of::<txt_record>() as
                               libc::c_ulong) as *mut txt_record;
            (*new_34).name = 0 as *mut libc::c_char;
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
                                         0 as *mut libc::c_uint,
                                         0 as *mut libc::c_int) as size_t;
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
        356 => {
            /* --caa-record */
            let mut new_35: *mut txt_record = 0 as *mut txt_record;
            let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
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
            (*new_35).class = 257 as libc::c_int as libc::c_ushort;
            current_block = 7879481898411272068;
        }
        89 => {
            /* --txt-record */
            let mut new_36: *mut txt_record =
                0 as *mut txt_record; /* room for extra counts */
            let mut p_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut cnt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len_4: size_t = 0;
            comma = split(arg);
            new_36 =
                opt_malloc(::std::mem::size_of::<txt_record>() as
                               libc::c_ulong) as *mut txt_record;
            (*new_36).class = 1 as libc::c_int as libc::c_ushort;
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
                let mut c: libc::c_uchar = *fresh30 as libc::c_uchar;
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
            let mut port: libc::c_int = 1 as libc::c_int;
            let mut priority: libc::c_int = 0 as libc::c_int;
            let mut weight: libc::c_int = 0 as libc::c_int;
            let mut name_3: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target_2: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut new_37: *mut mx_srv_record = 0 as *mut mx_srv_record;
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
        308 => {
            /* --host-record */
            let mut new_38: *mut host_record = 0 as *mut host_record;
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
                let mut addr_1: all_addr =
                    all_addr{addr4: in_addr{s_addr: 0,},};
                let mut dig_0: *mut libc::c_char = 0 as *mut libc::c_char;
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
                } else if inet_pton(2 as libc::c_int, arg,
                                    &mut addr_1.addr4 as *mut in_addr as
                                        *mut libc::c_void) != 0 {
                    (*new_38).addr = addr_1.addr4;
                    (*new_38).flags |= 2 as libc::c_int
                } else if inet_pton(10 as libc::c_int, arg,
                                    &mut addr_1.addr6 as *mut in6_addr as
                                        *mut libc::c_void) != 0 {
                    (*new_38).addr6 = addr_1.addr6;
                    (*new_38).flags |= 1 as libc::c_int
                } else {
                    let mut nomem: libc::c_int = 0;
                    let mut canon: *mut libc::c_char =
                        canonicalise(arg, &mut nomem);
                    let mut nl: *mut name_list = 0 as *mut name_list;
                    if canon.is_null() {
                        let mut tmp_3: *mut name_list = (*new_38).names;
                        let mut next: *mut name_list = 0 as *mut name_list;
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
                    (*nl).next = 0 as *mut name_list;
                    if (*new_38).names.is_null() {
                        (*new_38).names = nl
                    } else {
                        let mut tmp_4: *mut name_list = 0 as *mut name_list;
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
            (*new_38).next = 0 as *mut host_record;
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
            let mut addr_0: in_addr = in_addr{s_addr: 0,}; /* error */
            unhide_metas(arg);
            if !arg.is_null() &&
                   inet_pton(2 as libc::c_int, arg,
                             &mut addr_0 as *mut in_addr as *mut libc::c_void)
                       > 0 as libc::c_int {
                let mut baddr: *mut bogus_addr =
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
                let mut new_9: *mut iname =
                    opt_malloc(::std::mem::size_of::<iname>() as
                                   libc::c_ulong) as *mut iname;
                comma = split(arg);
                (*new_9).name = opt_string_alloc(arg);
                if option == 'I' as i32 {
                    (*new_9).next = (*dnsmasq_daemon).if_except;
                    (*dnsmasq_daemon).if_except = new_9
                } else if option == 258 as libc::c_int {
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
                                  if option == 264 as libc::c_int {
                                      16 as libc::c_int
                                  } else if option == 281 as libc::c_int {
                                      128 as libc::c_int
                                  } else if option == 279 as libc::c_int {
                                      32 as libc::c_int
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
            let mut serv_1: *mut server = 0 as *mut server;
            let mut newlist: *mut server = 0 as *mut server;
            unhide_metas(arg);
            if !arg.is_null() &&
                   (*arg as libc::c_int == '/' as i32 ||
                        option == 298 as libc::c_int) {
                let mut rebind: libc::c_int =
                    !(*arg as libc::c_int == '/' as i32) as libc::c_int;
                let mut end_0: *mut libc::c_char = 0 as *mut libc::c_char;
                if rebind == 0 { arg = arg.offset(1) }
                while rebind != 0 ||
                          {
                              end_0 =
                                  split_chr(arg, '/' as i32 as libc::c_char);
                              !end_0.is_null()
                          } {
                    let mut domain: *mut libc::c_char =
                        0 as *mut libc::c_char;
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
                            8 as libc::c_int
                        } else { 32 as libc::c_int };
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
                (*newlist).flags |= 4096 as libc::c_int
            }
            if option == 'A' as i32 {
                (*newlist).flags |= 4 as libc::c_int;
                if (*newlist).flags & (8 as libc::c_int | 32 as libc::c_int)
                       == 0 {
                    server_list_free(newlist);
                    strcpy(errstr, gen_err);
                    return 0 as libc::c_int
                }
            } else if option == 298 as libc::c_int {
                (*newlist).flags |= 2048 as libc::c_int
            }
            if arg.is_null() || *arg == 0 {
                if (*newlist).flags & 2048 as libc::c_int == 0 {
                    (*newlist).flags |= 2 as libc::c_int
                }
                /* no server */
            } else if strcmp(arg,
                             b"#\x00" as *const u8 as *const libc::c_char) ==
                          0 as libc::c_int {
                (*newlist).flags |= 1024 as libc::c_int
            } else {
                let mut err_0: *mut libc::c_char =
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
                    (*serv_1).flags & !(8 as libc::c_int | 32 as libc::c_int);
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
            let mut new_24: *mut dhcp_netid_list =
                opt_malloc(::std::mem::size_of::<dhcp_netid_list>() as
                               libc::c_ulong) as *mut dhcp_netid_list;
            let mut list_1: *mut dhcp_netid = 0 as *mut dhcp_netid;
            if option == 'J' as i32 {
                (*new_24).next = (*dnsmasq_daemon).dhcp_ignore;
                (*dnsmasq_daemon).dhcp_ignore = new_24
            } else if option == 282 as libc::c_int {
                (*new_24).next = (*dnsmasq_daemon).force_broadcast;
                (*dnsmasq_daemon).force_broadcast = new_24
            } else if option == '3' as i32 {
                (*new_24).next = (*dnsmasq_daemon).bootp_dynamic;
                (*dnsmasq_daemon).bootp_dynamic = new_24
            } else if option == 296 as libc::c_int {
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
            let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut dig: libc::c_int = 0 as libc::c_int;
            let mut new_23: *mut dhcp_vendor =
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
                              0 as *mut libc::c_uint, 0 as *mut libc::c_int);
                (*new_23).data =
                    opt_malloc((*new_23).len as size_t) as *mut libc::c_char;
                memcpy((*new_23).data as *mut libc::c_void,
                       comma as *const libc::c_void,
                       (*new_23).len as libc::c_ulong);
            }
            match option {
                106 => { (*new_23).match_type = 2 as libc::c_int }
                85 => { (*new_23).match_type = 1 as libc::c_int }
                268 => { (*new_23).match_type = 3 as libc::c_int }
                269 => { (*new_23).match_type = 4 as libc::c_int }
                270 => { (*new_23).match_type = 5 as libc::c_int }
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
            let mut new_3: *mut hostsfile =
                opt_malloc(::std::mem::size_of::<hostsfile>() as
                               libc::c_ulong) as *mut hostsfile;
            static mut hosts_index: libc::c_uint =
                3 as libc::c_int as libc::c_uint;
            (*new_3).fname = opt_string_alloc(arg);
            let fresh26 = hosts_index;
            hosts_index = hosts_index.wrapping_add(1);
            (*new_3).index = fresh26;
            (*new_3).flags = 0 as libc::c_int;
            if option == 'H' as i32 {
                (*new_3).next = (*dnsmasq_daemon).addn_hosts;
                (*dnsmasq_daemon).addn_hosts = new_3
            } else if option == 273 as libc::c_int {
                (*new_3).next = (*dnsmasq_daemon).dhcp_hosts_file;
                (*dnsmasq_daemon).dhcp_hosts_file = new_3
            } else if option == 280 as libc::c_int {
                (*new_3).next = (*dnsmasq_daemon).dhcp_opts_file;
                (*dnsmasq_daemon).dhcp_opts_file = new_3
            } else {
                (*new_3).next = (*dnsmasq_daemon).dynamic_dirs;
                (*dnsmasq_daemon).dynamic_dirs = new_3;
                if option == 340 as libc::c_int {
                    (*new_3).flags |= 16 as libc::c_int
                } else if option == 341 as libc::c_int {
                    (*new_3).flags |= 32 as libc::c_int
                } else if option == 342 as libc::c_int {
                    (*new_3).flags |= 8 as libc::c_int
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
                if option == 283 as libc::c_int {
                    (*dnsmasq_daemon).neg_ttl = ttl as libc::c_ulong
                } else if option == 297 as libc::c_int {
                    (*dnsmasq_daemon).max_ttl = ttl as libc::c_ulong
                } else if option == 339 as libc::c_int {
                    if ttl > 3600 as libc::c_int { ttl = 3600 as libc::c_int }
                    (*dnsmasq_daemon).min_cache_ttl = ttl as libc::c_ulong
                } else if option == 312 as libc::c_int {
                    (*dnsmasq_daemon).max_cache_ttl = ttl as libc::c_ulong
                } else if option == 315 as libc::c_int {
                    (*dnsmasq_daemon).auth_ttl = ttl as libc::c_ulong
                } else if option == 348 as libc::c_int {
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
unsafe extern "C" fn read_file(mut file: *mut libc::c_char, mut f: *mut FILE,
                               mut hard_opt: libc::c_int) {
    let mut lineno: libc::c_int = 0 as libc::c_int;
    let mut buff: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    let mut current_block_66: u64;
    while !fgets(buff, 1025 as libc::c_int, f).is_null() {
        let mut white: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut option: libc::c_int =
            if hard_opt == 332 as libc::c_int {
                0 as libc::c_int
            } else { hard_opt };
        let mut errmess: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        /* Memory allocation failure longjmps here if mem_recover == 1 */
        if option != 0 as libc::c_int || hard_opt == 332 as libc::c_int {
            if _setjmp(mem_jmp.as_mut_ptr()) != 0 { continue ; }
            ::std::ptr::write_volatile(&mut mem_recover as *mut libc::c_int,
                                       1 as libc::c_int)
        }
        arg = 0 as *mut libc::c_char;
        ::std::ptr::write_volatile(&mut lineno as *mut libc::c_int,
                                   ::std::ptr::read_volatile::<libc::c_int>(&lineno
                                                                                as
                                                                                *const libc::c_int)
                                       + 1);
        errmess = 0 as *mut libc::c_char;
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
                    } else { arg = 0 as *mut libc::c_char }
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
                    } else if hard_opt == 332 as libc::c_int &&
                                  option != 'S' as i32 &&
                                  option != 332 as libc::c_int {
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
                       (hard_opt == 332 as libc::c_int) as libc::c_int) == 0 {
            sprintf((*dnsmasq_daemon).namebuff.offset(strlen((*dnsmasq_daemon).namebuff)
                                                          as isize),
                    b" at line %d of %s\x00" as *const u8 as
                        *const libc::c_char, lineno, file);
            if hard_opt != 0 as libc::c_int {
                my_syslog(3 as libc::c_int,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).namebuff);
            } else {
                die(b"%s\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, (*dnsmasq_daemon).namebuff,
                    1 as libc::c_int);
            }
        }
    }
    ::std::ptr::write_volatile(&mut mem_recover as *mut libc::c_int,
                               0 as libc::c_int);
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn option_read_dynfile(mut file: *mut libc::c_char,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
              b"read %s\x00" as *const u8 as *const libc::c_char, file);
    if flags & 16 as libc::c_int != 0 {
        return one_file(file, 272 as libc::c_int)
    } else {
        if flags & 32 as libc::c_int != 0 {
            return one_file(file, 279 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn one_file(mut file: *mut libc::c_char,
                              mut hard_opt: libc::c_int) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut nofile_ok: libc::c_int = 0 as libc::c_int;
    static mut read_stdin: libc::c_int = 0 as libc::c_int;
    static mut filesread: *mut fileread =
        0 as *const fileread as *mut fileread;
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
        let mut statbuf: stat =
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
            let mut r: *mut fileread = 0 as *mut fileread;
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
            if *__errno_location() == 2 as libc::c_int && nofile_ok != 0 {
                return 1 as libc::c_int
            } else {
                let mut str: *mut libc::c_char =
                    b"cannot read %s: %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char;
                if hard_opt != 0 as libc::c_int {
                    my_syslog(3 as libc::c_int, str, file,
                              strerror(*__errno_location()));
                    return 0 as libc::c_int
                } else { die(str, file, 3 as libc::c_int); }
            }
        }
    }
    read_file(file, f, hard_opt);
    return 1 as libc::c_int;
}
/* expand any name which is a directory */
#[no_mangle]
pub unsafe extern "C" fn expand_filelist(mut list: *mut hostsfile)
 -> *mut hostsfile {
    let mut i: libc::c_uint = 0;
    let mut ah: *mut hostsfile = 0 as *mut hostsfile;
    /* find largest used index */
    i = 3 as libc::c_int as libc::c_uint;
    ah = list;
    while !ah.is_null() {
        if i <= (*ah).index {
            i = (*ah).index.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
        if (*ah).flags & 1 as libc::c_int != 0 {
            (*ah).flags |= 2 as libc::c_int
        } else { (*ah).flags &= !(2 as libc::c_int) }
        ah = (*ah).next
    }
    ah = list;
    while !ah.is_null() {
        if (*ah).flags & 2 as libc::c_int == 0 {
            let mut buf: stat =
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
                   buf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                       0o40000 as libc::c_int as libc::c_uint {
                let mut dir_stream: *mut DIR = 0 as *mut DIR;
                let mut ent: *mut dirent = 0 as *mut dirent;
                /* don't read this as a file */
                (*ah).flags |= 2 as libc::c_int;
                dir_stream = opendir((*ah).fname);
                if dir_stream.is_null() {
                    my_syslog(3 as libc::c_int,
                              b"cannot access directory %s: %s\x00" as
                                  *const u8 as *const libc::c_char,
                              (*ah).fname, strerror(*__errno_location()));
                } else {
                    loop  {
                        ent = readdir(dir_stream);
                        if ent.is_null() { break ; }
                        let mut lendir: size_t = strlen((*ah).fname);
                        let mut lenfile: size_t =
                            strlen((*ent).d_name.as_mut_ptr());
                        let mut ah1: *mut hostsfile = 0 as *mut hostsfile;
                        let mut path: *mut libc::c_char =
                            0 as *mut libc::c_char;
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
                                (*ah1).flags &= !(2 as libc::c_int);
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
                                (*ah1).flags = 1 as libc::c_int;
                                (*ah1).next = list;
                                list = ah1
                            }
                        }
                        /* inactivate record if not regular file */
                        if (*ah1).flags & 1 as libc::c_int != 0 &&
                               stat((*ah1).fname, &mut buf) !=
                                   -(1 as libc::c_int) &&
                               !(buf.st_mode &
                                     0o170000 as libc::c_int as libc::c_uint
                                     ==
                                     0o100000 as libc::c_int as libc::c_uint)
                           {
                            (*ah1).flags |= 2 as libc::c_int
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
pub unsafe extern "C" fn read_servers_file() {
    let mut f: *mut FILE = 0 as *mut FILE;
    f =
        fopen((*dnsmasq_daemon).servers_file,
              b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        my_syslog(3 as libc::c_int,
                  b"cannot read %s: %s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).servers_file,
                  strerror(*__errno_location()));
        return
    }
    mark_servers(4096 as libc::c_int);
    cleanup_servers();
    read_file((*dnsmasq_daemon).servers_file, f, 332 as libc::c_int);
}
unsafe extern "C" fn clear_dynamic_conf() {
    let mut configs: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut cp: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut up: *mut *mut dhcp_config = 0 as *mut *mut dhcp_config;
    /* remove existing... */
    up = &mut (*dnsmasq_daemon).dhcp_conf;
    configs = (*dnsmasq_daemon).dhcp_conf;
    while !configs.is_null() {
        cp = (*configs).next;
        if (*configs).flags & 2048 as libc::c_int as libc::c_uint != 0 {
            let mut mac: *mut hwaddr_config = 0 as *mut hwaddr_config;
            let mut tmp: *mut hwaddr_config = 0 as *mut hwaddr_config;
            let mut list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
            let mut tmplist: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
            mac = (*configs).hwaddr;
            while !mac.is_null() {
                tmp = (*mac).next;
                free(mac as *mut libc::c_void);
                mac = tmp
            }
            if (*configs).flags & 2 as libc::c_int as libc::c_uint != 0 {
                free((*configs).clid as *mut libc::c_void);
            }
            list = (*configs).netid;
            while !list.is_null() {
                free((*list).list as *mut libc::c_void);
                tmplist = (*list).next;
                free(list as *mut libc::c_void);
                list = tmplist
            }
            if (*configs).flags & 16 as libc::c_int as libc::c_uint != 0 {
                free((*configs).hostname as *mut libc::c_void);
            }
            *up = (*configs).next;
            free(configs as *mut libc::c_void);
        } else { up = &mut (*configs).next }
        configs = cp
    };
}
unsafe extern "C" fn clear_dynamic_opt() {
    let mut opts_0: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut cp: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut up: *mut *mut dhcp_opt = 0 as *mut *mut dhcp_opt;
    let mut id: *mut dhcp_netid = 0 as *mut dhcp_netid;
    let mut next: *mut dhcp_netid = 0 as *mut dhcp_netid;
    up = &mut (*dnsmasq_daemon).dhcp_opts;
    opts_0 = (*dnsmasq_daemon).dhcp_opts;
    while !opts_0.is_null() {
        cp = (*opts_0).next;
        if (*opts_0).flags & 32 as libc::c_int != 0 {
            if (*opts_0).flags & 256 as libc::c_int != 0 {
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
pub unsafe extern "C" fn reread_dhcp() {
    let mut hf: *mut hostsfile = 0 as *mut hostsfile;
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
            if (*hf).flags & 2 as libc::c_int == 0 {
                if one_file((*hf).fname, 272 as libc::c_int) != 0 {
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  6 as libc::c_int,
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
            if (*hf).flags & 2 as libc::c_int == 0 {
                if one_file((*hf).fname, 279 as libc::c_int) != 0 {
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  6 as libc::c_int,
                              b"read %s\x00" as *const u8 as
                                  *const libc::c_char, (*hf).fname);
                }
            }
            hf = (*hf).next
        }
    }
    /* Setup notify and read pre-existing files. */
    set_dynamic_inotify(16 as libc::c_int | 32 as libc::c_int,
                        0 as libc::c_int, 0 as *mut *mut crec,
                        0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn read_opts(mut argc: libc::c_int,
                                   mut argv: *mut *mut libc::c_char,
                                   mut compile_opts: *mut libc::c_char) {
    let mut argbuf_size: size_t = 1025 as libc::c_int as size_t;
    let mut argbuf: *mut libc::c_char =
        opt_malloc(argbuf_size) as *mut libc::c_char;
    let mut buff: *mut libc::c_char =
        opt_malloc(1025 as libc::c_int as size_t) as *mut libc::c_char;
    let mut option: libc::c_int = 0;
    let mut testmode: libc::c_int = 0 as libc::c_int;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut conffile: *mut libc::c_char = 0 as *mut libc::c_char;
    opterr = 0 as libc::c_int;
    dnsmasq_daemon =
        opt_malloc(::std::mem::size_of::<dnsmasq_daemon>() as libc::c_ulong)
            as *mut dnsmasq_daemon;
    memset(dnsmasq_daemon as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dnsmasq_daemon>() as libc::c_ulong);
    (*dnsmasq_daemon).namebuff = buff;
    /* Set defaults - everything else is zero or NULL */
    (*dnsmasq_daemon).cachesize = 150 as libc::c_int;
    (*dnsmasq_daemon).ftabsize = 150 as libc::c_int;
    (*dnsmasq_daemon).port = 53 as libc::c_int;
    (*dnsmasq_daemon).dhcp_client_port = 68 as libc::c_int;
    (*dnsmasq_daemon).dhcp_server_port = 67 as libc::c_int;
    (*dnsmasq_daemon).default_resolv.is_default = 1 as libc::c_int;
    (*dnsmasq_daemon).default_resolv.name =
        b"/etc/resolv.conf\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*dnsmasq_daemon).resolv_files = &mut (*dnsmasq_daemon).default_resolv;
    (*dnsmasq_daemon).username =
        b"nobody\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*dnsmasq_daemon).runfile =
        b"/var/run/dnsmasq.pid\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*dnsmasq_daemon).dhcp_max = 1000 as libc::c_int;
    (*dnsmasq_daemon).tftp_max = 50 as libc::c_int;
    (*dnsmasq_daemon).edns_pktsz = 4096 as libc::c_int as libc::c_ushort;
    (*dnsmasq_daemon).log_fac = -(1 as libc::c_int);
    (*dnsmasq_daemon).auth_ttl = 600 as libc::c_int as libc::c_ulong;
    (*dnsmasq_daemon).soa_refresh = 1200 as libc::c_int as libc::c_ulong;
    (*dnsmasq_daemon).soa_retry = 180 as libc::c_int as libc::c_ulong;
    (*dnsmasq_daemon).soa_expiry = 1209600 as libc::c_int as libc::c_ulong;
    (*dnsmasq_daemon).max_port = 65535 as libc::c_uint as libc::c_int;
    (*dnsmasq_daemon).min_port = 1024 as libc::c_int;
    add_txt(b"version.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"dnsmasq-2.84rc2\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as libc::c_int);
    add_txt(b"authors.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"Simon Kelley\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as libc::c_int);
    add_txt(b"copyright.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"Copyright (c) 2000-2021 Simon Kelley\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char, 0 as libc::c_int);
    add_txt(b"cachesize.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 1 as libc::c_int);
    add_txt(b"insertions.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 2 as libc::c_int);
    add_txt(b"evictions.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 3 as libc::c_int);
    add_txt(b"misses.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 4 as libc::c_int);
    add_txt(b"hits.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 5 as libc::c_int);
    add_txt(b"auth.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 6 as libc::c_int);
    add_txt(b"servers.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 7 as libc::c_int);
    loop  {
        option =
            getopt_long(argc, argv,
                        b"951yZDNLERKzowefnbvhdkqr:m:p:c:l:s:i:t:u:g:a:x:S:C:A:T:H:Q:I:B:F:G:O:M:X:V:U:j:P:J:W:Y:2:4:6:7:8:0:3:\x00"
                            as *const u8 as *const libc::c_char,
                        opts.as_ptr(), 0 as *mut libc::c_int);
        if option == -(1 as libc::c_int) {
            while optind < argc {
                let mut c: *mut libc::c_uchar =
                    *argv.offset(optind as isize) as *mut libc::c_uchar;
                while *c as libc::c_int != 0 as libc::c_int {
                    if *(*__ctype_b_loc()).offset(*c as libc::c_int as isize)
                           as libc::c_int &
                           _ISspace as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        die(b"junk found in command line\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_char, 1 as libc::c_int);
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
            } else { arg = 0 as *mut libc::c_char }
            /* command-line only stuff */
            if option == 293 as libc::c_int {
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
                               *const libc::c_char,
                           b"2.84rc2\x00" as *const u8 as *const libc::c_char,
                           b"Copyright (c) 2000-2021 Simon Kelley\x00" as
                               *const u8 as *const libc::c_char);
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
                            let mut extra: *mut libc::c_char =
                                opt_string_alloc(arg);
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
                            (*dnsmasq_daemon).namebuff, 1 as libc::c_int);
                    }
                }
            }
        }
    }
    free(argbuf as *mut libc::c_void);
    if !conffile.is_null() {
        one_file(conffile, 0 as libc::c_int);
        free(conffile as *mut libc::c_void);
    } else {
        one_file(b"/etc/dnsmasq.conf\x00" as *const u8 as *const libc::c_char
                     as *mut libc::c_char, '7' as i32);
    }
    /* port might not be known when the address is parsed - fill in here */
    if !(*dnsmasq_daemon).servers.is_null() {
        let mut tmp: *mut server = 0 as *mut server;
        tmp = (*dnsmasq_daemon).servers;
        while !tmp.is_null() {
            if (*tmp).flags & 16 as libc::c_int == 0 {
                if (*tmp).source_addr.sa.sa_family as libc::c_int ==
                       2 as libc::c_int {
                    (*tmp).source_addr.in_0.sin_port =
                        __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
                } else if (*tmp).source_addr.sa.sa_family as libc::c_int ==
                              10 as libc::c_int {
                    (*tmp).source_addr.in6.sin6_port =
                        __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
                }
            }
            tmp = (*tmp).next
        }
    }
    if !(*dnsmasq_daemon).host_records.is_null() {
        let mut hr: *mut host_record = 0 as *mut host_record;
        hr = (*dnsmasq_daemon).host_records;
        while !hr.is_null() {
            if (*hr).ttl == -(1 as libc::c_int) {
                (*hr).ttl = (*dnsmasq_daemon).local_ttl as libc::c_int
            }
            hr = (*hr).next
        }
    }
    if !(*dnsmasq_daemon).cnames.is_null() {
        let mut cn: *mut cname = 0 as *mut cname;
        let mut cn2: *mut cname = 0 as *mut cname;
        let mut cn3: *mut cname = 0 as *mut cname;
        /* Fill in TTL for CNAMES now we have local_ttl.
	 Also prepare to do loop detection. */
        cn = (*dnsmasq_daemon).cnames;
        while !cn.is_null() {
            if (*cn).ttl == -(1 as libc::c_int) {
                (*cn).ttl = (*dnsmasq_daemon).local_ttl as libc::c_int
            }
            (*cn).flag = 0 as libc::c_int;
            (*cn).targetp = 0 as *mut cname;
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
                if (*cn2).flag == 1 as libc::c_int { break ; }
                if (*cn2).flag == 2 as libc::c_int {
                    die(b"CNAME loop involving %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*cn).alias, 1 as libc::c_int);
                }
                (*cn2).flag = 2 as libc::c_int;
                cn2 = (*cn2).targetp
            }
            cn3 = (*cn).targetp;
            while cn3 != cn2 {
                (*cn3).flag = 1 as libc::c_int;
                cn3 = (*cn3).targetp
            }
            cn = (*cn).next
        }
    }
    if !(*dnsmasq_daemon).if_addrs.is_null() {
        let mut tmp_0: *mut iname = 0 as *mut iname;
        tmp_0 = (*dnsmasq_daemon).if_addrs;
        while !tmp_0.is_null() {
            if (*tmp_0).addr.sa.sa_family as libc::c_int == 2 as libc::c_int {
                (*tmp_0).addr.in_0.sin_port =
                    __bswap_16((*dnsmasq_daemon).port as __uint16_t)
            } else if (*tmp_0).addr.sa.sa_family as libc::c_int ==
                          10 as libc::c_int {
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
            opt_string_alloc(b"PXEClient\x00" as *const u8 as
                                 *const libc::c_char);
        (*(*dnsmasq_daemon).dhcp_pxe_vendors).next = 0 as *mut dhcp_pxe_vendor
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
        let mut mx: *mut mx_srv_record = 0 as *mut mx_srv_record;
        if gethostname(buff, 1025 as libc::c_int as size_t) ==
               -(1 as libc::c_int) {
            die(b"cannot get host-name: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
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
            (*mx).target = 0 as *mut libc::c_char;
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
            0 as *mut libc::c_char, 1 as libc::c_int);
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
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut f: *mut FILE = 0 as *mut FILE;
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
                0 as *mut libc::c_char, 1 as libc::c_int);
        }
        f =
            fopen((*(*dnsmasq_daemon).resolv_files).name,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            die(b"failed to read %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*(*dnsmasq_daemon).resolv_files).name, 3 as libc::c_int);
        }
        loop  {
            line = fgets(buff, 1025 as libc::c_int, f);
            if line.is_null() { break ; }
            let mut token: *mut libc::c_char =
                strtok(line,
                       b" \t\n\r\x00" as *const u8 as *const libc::c_char);
            if token.is_null() ||
                   strcmp(token,
                          b"search\x00" as *const u8 as *const libc::c_char)
                       != 0 as libc::c_int {
                continue ;
            }
            token =
                strtok(0 as *mut libc::c_char,
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
                (*(*dnsmasq_daemon).resolv_files).name, 5 as libc::c_int);
        }
    }
    if !(*dnsmasq_daemon).domain_suffix.is_null() {
        /* add domain for any srv record without one. */
        let mut srv: *mut mx_srv_record = 0 as *mut mx_srv_record;
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
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    /* If there's access-control config, then ignore --local-service, it's intended
     as a system default to keep otherwise unconfigured installations safe. */
    if !(*dnsmasq_daemon).if_names.is_null() ||
           !(*dnsmasq_daemon).if_except.is_null() ||
           !(*dnsmasq_daemon).if_addrs.is_null() ||
           !(*dnsmasq_daemon).authserver.is_null() {
        reset_option_bool(49 as libc::c_int as libc::c_uint);
    }
    if testmode != 0 {
        fprintf(stderr,
                b"dnsmasq: %s.\n\x00" as *const u8 as *const libc::c_char,
                b"syntax check OK\x00" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    };
}
