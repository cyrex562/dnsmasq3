
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
static mut leases: *mut dhcp_lease =
    0 as *const dhcp_lease as *mut dhcp_lease;
static mut old_leases: *mut dhcp_lease =
    0 as *const dhcp_lease as *mut dhcp_lease;
static mut dns_dirty: libc::c_int = 0;
static mut file_dirty: libc::c_int = 0;
static mut leases_left: libc::c_int = 0;
unsafe extern "C" fn read_leases(mut now: time_t, mut leasestream: *mut FILE)
 -> libc::c_int {
    let mut ei: libc::c_ulong = 0;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut clid_len: libc::c_int = 0;
    let mut hw_len: libc::c_int = 0;
    let mut hw_type: libc::c_int = 0;
    let mut items: libc::c_int = 0;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
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
                          130 as libc::c_int, 0 as *mut libc::c_uint,
                          0 as *mut libc::c_int);
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
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          4 as libc::c_int,
                      b"ignoring invalid line in lease database: %s %s %s %s ...\x00"
                          as *const u8 as *const libc::c_char,
                      (*dnsmasq_daemon).dhcp_buff3,
                      (*dnsmasq_daemon).dhcp_buff2,
                      (*dnsmasq_daemon).namebuff,
                      (*dnsmasq_daemon).dhcp_buff);
        } else {
            if inet_pton(2 as libc::c_int, (*dnsmasq_daemon).namebuff,
                         &mut addr.addr4 as *mut in_addr as *mut libc::c_void)
                   != 0 {
                lease = lease4_allocate(addr.addr4);
                if !lease.is_null() { domain = get_domain((*lease).addr) }
                hw_len =
                    parse_hex((*dnsmasq_daemon).dhcp_buff2,
                              (*dnsmasq_daemon).dhcp_buff2 as
                                  *mut libc::c_uchar, 16 as libc::c_int,
                              0 as *mut libc::c_uint, &mut hw_type);
                /* For backwards compatibility, no explicit MAC address type means ether. */
                if hw_type == 0 as libc::c_int && hw_len != 0 as libc::c_int {
                    hw_type = 1 as libc::c_int
                }
            } else if inet_pton(10 as libc::c_int, (*dnsmasq_daemon).namebuff,
                                &mut addr.addr6 as *mut in6_addr as
                                    *mut libc::c_void) != 0 {
                let mut s: *mut libc::c_char = (*dnsmasq_daemon).dhcp_buff2;
                let mut lease_type: libc::c_int = 32 as libc::c_int;
                if *s.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'T' as i32 {
                    lease_type = 64 as libc::c_int;
                    s = s.offset(1)
                }
                lease = lease6_allocate(&mut addr.addr6, lease_type);
                if !lease.is_null() {
                    lease_set_iaid(lease,
                                   strtoul(s, 0 as *mut *mut libc::c_char,
                                           10 as libc::c_int) as
                                       libc::c_uint);
                    domain = get_domain6(&mut (*lease).addr6)
                }
            } else {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"ignoring invalid line in lease database, bad address: %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).namebuff);
                continue ;
            }
            if lease.is_null() {
                die(b"too many stored leases\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_char, 5 as libc::c_int);
            }
            if strcmp((*dnsmasq_daemon).packet,
                      b"*\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                clid_len =
                    parse_hex((*dnsmasq_daemon).packet,
                              (*dnsmasq_daemon).packet as *mut libc::c_uchar,
                              255 as libc::c_int, 0 as *mut libc::c_uint,
                              0 as *mut libc::c_int)
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
                                   0 as *mut libc::c_char);
            }
            ei = atol((*dnsmasq_daemon).dhcp_buff3) as libc::c_ulong;
            /* strictly time_t is opaque, but this hack should work on all sane systems,
	   even when sizeof(time_t) == 8 */
            (*lease).expires = ei as time_t;
            /* set these correctly: the "old" events are generated later from
	   the startup synthesised SIGHUP. */
            (*lease).flags &= !(1 as libc::c_int | 2 as libc::c_int);
            *(*dnsmasq_daemon).dhcp_buff2 = '\u{0}' as i32 as libc::c_char;
            *(*dnsmasq_daemon).dhcp_buff3 = *(*dnsmasq_daemon).dhcp_buff2
        }
    }
    return (items == 0 as libc::c_int || items == -(1 as libc::c_int)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lease_init(mut now: time_t) {
    let mut leasestream: *mut FILE = 0 as *mut FILE;
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
                (*dnsmasq_daemon).lease_file, 3 as libc::c_int);
        }
        /* a+ mode leaves pointer at end. */
        rewind(leasestream);
    }
    if !leasestream.is_null() {
        if read_leases(now, leasestream) == 0 {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          3 as libc::c_int,
                      b"failed to parse lease database cleanly\x00" as
                          *const u8 as *const libc::c_char);
        }
        if ferror(leasestream) != 0 {
            die(b"failed to read lease file %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_file, 3 as libc::c_int);
        }
    }
    if (*dnsmasq_daemon).lease_stream.is_null() {
        let mut rc: libc::c_int = 0 as libc::c_int;
        /* shell returns 127 for "command not found", 126 for bad permissions. */
        if leasestream.is_null() ||
               { rc = pclose(leasestream); (rc) == -(1 as libc::c_int) } ||
               (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   127 as libc::c_int ||
               (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   126 as libc::c_int {
            if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   127 as libc::c_int {
                *__errno_location() = 2 as libc::c_int
            } else if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                          126 as libc::c_int {
                *__errno_location() = 13 as libc::c_int
            }
            die(b"cannot run lease-init script %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_change_command, 3 as libc::c_int);
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
                    10 as libc::c_int);
        }
    }
    /* Some leases may have expired */
    file_dirty = 0 as libc::c_int;
    lease_prune(0 as *mut dhcp_lease, now);
    dns_dirty = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_from_configs() {
    /* changes to the config may change current leases. */
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0) {
            config =
                find_config((*dnsmasq_daemon).dhcp_conf,
                            0 as *mut dhcp_context, (*lease).clid,
                            (*lease).clid_len, (*lease).hwaddr.as_mut_ptr(),
                            (*lease).hwaddr_len, (*lease).hwaddr_type,
                            0 as *mut libc::c_char, 0 as *mut dhcp_netid);
            if !config.is_null() &&
                   (*config).flags & 16 as libc::c_int as libc::c_uint != 0 &&
                   ((*config).flags & 32 as libc::c_int as libc::c_uint == 0
                        || (*config).addr.s_addr == (*lease).addr.s_addr) {
                lease_set_hostname(lease, (*config).hostname,
                                   1 as libc::c_int,
                                   get_domain((*lease).addr),
                                   0 as *mut libc::c_char);
            } else {
                name = host_from_dns((*lease).addr);
                if !name.is_null() {
                    lease_set_hostname(lease, name, 1 as libc::c_int,
                                       get_domain((*lease).addr),
                                       0 as *mut libc::c_char);
                }
            }
        }
        lease = (*lease).next
    };
    /* updates auth flag only */
}
unsafe extern "C" fn ourprintf(mut errp: *mut libc::c_int,
                               mut format: *mut libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if *errp == 0 &&
           vfprintf((*dnsmasq_daemon).lease_stream, format, ap.as_va_list()) <
               0 as libc::c_int {
        *errp = *__errno_location()
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_file(mut now: time_t) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut next_event: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    if file_dirty != 0 as libc::c_int &&
           !(*dnsmasq_daemon).lease_stream.is_null() {
        *__errno_location() = 0 as libc::c_int;
        rewind((*dnsmasq_daemon).lease_stream);
        if *__errno_location() != 0 as libc::c_int ||
               ftruncate(fileno((*dnsmasq_daemon).lease_stream),
                         0 as libc::c_int as __off64_t) != 0 as libc::c_int {
            err = *__errno_location()
        }
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) !=
                     0) {
                ourprintf(&mut err as *mut libc::c_int,
                          b"%lu \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*lease).expires as libc::c_ulong);
                if (*lease).hwaddr_type != 1 as libc::c_int ||
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
                inet_ntop(2 as libc::c_int,
                          &mut (*lease).addr as *mut in_addr as
                              *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                          46 as libc::c_int as socklen_t);
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
                if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int)
                         == 0) {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%lu \x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              (*lease).expires as libc::c_ulong);
                    inet_ntop(10 as libc::c_int,
                              &mut (*lease).addr6 as *mut in6_addr as
                                  *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%s%u %s \x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              if (*lease).flags & 64 as libc::c_int != 0 {
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
            err = *__errno_location()
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
               difftime(next_event, 60 as libc::c_int as libc::c_long + now) >
                   0.0f64 {
            next_event = 60 as libc::c_int as libc::c_long + now
        }
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 3 as libc::c_int,
                  b"failed to write %s: %s (retry in %us)\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).lease_file,
                  strerror(err), difftime(next_event, now) as libc::c_uint);
    }
    send_alarm(next_event, now);
}
unsafe extern "C" fn find_interface_v4(mut local: in_addr,
                                       mut if_index: libc::c_int,
                                       mut label: *mut libc::c_char,
                                       mut netmask: in_addr,
                                       mut broadcast: in_addr,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut prefix: libc::c_int = netmask_length(netmask);
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) == 0 &&
               is_same_net(local, (*lease).addr, netmask) != 0 &&
               prefix > (*lease).new_prefixlen {
            (*lease).new_interface = if_index;
            (*lease).new_prefixlen = prefix
        }
        lease = (*lease).next
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_interface_v6(mut local: *mut in6_addr,
                                       mut prefix: libc::c_int,
                                       mut scope: libc::c_int,
                                       mut if_index: libc::c_int,
                                       mut flags: libc::c_int,
                                       mut preferred: libc::c_int,
                                       mut valid: libc::c_int,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 {
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
pub unsafe extern "C" fn lease_update_slaac(mut now: time_t) {
    /* Called when we construct a new RA-names context, to add putative
     new SLAAC addresses to existing leases. */
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
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
pub unsafe extern "C" fn lease_find_interfaces(mut now: time_t) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).new_interface = 0 as libc::c_int;
        (*lease).new_prefixlen = (*lease).new_interface;
        lease = (*lease).next
    }
    iface_enumerate(2 as libc::c_int,
                    &mut now as *mut time_t as *mut libc::c_void,
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
    iface_enumerate(10 as libc::c_int,
                    &mut now as *mut time_t as *mut libc::c_void,
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
pub unsafe extern "C" fn lease_make_duid(mut now: time_t) {
    /* If we're not doing DHCPv6, and there are not v6 leases, don't add the DUID to the database */
    if (*dnsmasq_daemon).duid.is_null() && (*dnsmasq_daemon).doing_dhcp6 != 0
       {
        file_dirty = 1 as libc::c_int;
        make_duid(now);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_dns(mut force: libc::c_int) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if (*dnsmasq_daemon).port != 0 as libc::c_int &&
           (dns_dirty != 0 || force != 0) {
        /* force transfer to authoritative secondaries */
        (*dnsmasq_daemon).soa_sn =
            (*dnsmasq_daemon).soa_sn.wrapping_add(1); /* unlink */
        cache_unhash_dhcp();
        lease = leases;
        while !lease.is_null() {
            let mut prot: libc::c_int = 2 as libc::c_int;
            if (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 {
                prot = 10 as libc::c_int
            } else if !(*lease).hostname.is_null() || !(*lease).fqdn.is_null()
             {
                let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
                slaac = (*lease).slaac_address;
                while !slaac.is_null() {
                    if (*slaac).backoff == 0 as libc::c_int {
                        if !(*lease).fqdn.is_null() {
                            cache_add_dhcp_entry((*lease).fqdn,
                                                 10 as libc::c_int,
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
                            cache_add_dhcp_entry((*lease).hostname,
                                                 10 as libc::c_int,
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
                                     if prot == 2 as libc::c_int {
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
                                     if prot == 2 as libc::c_int {
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
pub unsafe extern "C" fn lease_prune(mut target: *mut dhcp_lease,
                                     mut now: time_t) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut tmp: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut up: *mut *mut dhcp_lease = 0 as *mut *mut dhcp_lease;
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
pub unsafe extern "C" fn lease_find_by_client(mut hwaddr: *mut libc::c_uchar,
                                              mut hw_len: libc::c_int,
                                              mut hw_type: libc::c_int,
                                              mut clid: *mut libc::c_uchar,
                                              mut clid_len: libc::c_int)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if !clid.is_null() {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) !=
                     0) {
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
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0) {
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
    return 0 as *mut dhcp_lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease_find_by_addr(mut addr: in_addr)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0) {
            if (*lease).addr.s_addr == addr.s_addr { return lease }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
/* find address for {CLID, IAID, address} */
#[no_mangle]
pub unsafe extern "C" fn lease6_find(mut clid: *mut libc::c_uchar,
                                     mut clid_len: libc::c_int,
                                     mut lease_type: libc::c_int,
                                     mut iaid: libc::c_uint,
                                     mut addr: *mut in6_addr)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & lease_type == 0 || (*lease).iaid != iaid) {
            if !(({
                      let mut __a: *const in6_addr =
                          &mut (*lease).addr6 as *mut in6_addr as
                              *const in6_addr;
                      let mut __b: *const in6_addr = addr as *const in6_addr;
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
    return 0 as *mut dhcp_lease;
}
/* reset "USED flags */
#[no_mangle]
pub unsafe extern "C" fn lease6_reset() {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).flags &= !(16 as libc::c_int);
        lease = (*lease).next
    };
}
/* enumerate all leases belonging to {CLID, IAID} */
#[no_mangle]
pub unsafe extern "C" fn lease6_find_by_client(mut first: *mut dhcp_lease,
                                               mut lease_type: libc::c_int,
                                               mut clid: *mut libc::c_uchar,
                                               mut clid_len: libc::c_int,
                                               mut iaid: libc::c_uint)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if first.is_null() { first = leases } else { first = (*first).next }
    lease = first;
    while !lease.is_null() {
        if !((*lease).flags & 16 as libc::c_int != 0) {
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
    return 0 as *mut dhcp_lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease6_find_by_addr(mut net: *mut in6_addr,
                                             mut prefix: libc::c_int,
                                             mut addr: u64_0)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) == 0) {
            if is_same_net6(&mut (*lease).addr6, net, prefix) != 0 &&
                   (prefix == 128 as libc::c_int ||
                        addr6part(&mut (*lease).addr6) == addr) {
                return lease
            }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
/* Find largest assigned address in context */
#[no_mangle]
pub unsafe extern "C" fn lease_find_max_addr6(mut context: *mut dhcp_context)
 -> u64_0 {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut addr: u64_0 = addr6part(&mut (*context).start6);
    if (*context).flags as libc::c_uint &
           ((1 as libc::c_uint) << 0 as libc::c_int |
                (1 as libc::c_uint) << 3 as libc::c_int) == 0 {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) ==
                     0) {
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
pub unsafe extern "C" fn lease_find_max_addr(mut context: *mut dhcp_context)
 -> in_addr {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease; /* illegal value */
    let mut addr: in_addr = (*context).start;
    if (*context).flags as libc::c_uint &
           ((1 as libc::c_uint) << 0 as libc::c_int |
                (1 as libc::c_uint) << 3 as libc::c_int) == 0 {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) !=
                     0) {
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
unsafe extern "C" fn lease_allocate() -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if leases_left == 0 ||
           {
               lease =
                   whine_malloc(::std::mem::size_of::<dhcp_lease>() as
                                    libc::c_ulong) as *mut dhcp_lease;
               lease.is_null()
           } {
        return 0 as *mut dhcp_lease
    }
    memset(lease as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dhcp_lease>() as libc::c_ulong);
    (*lease).flags = 1 as libc::c_int;
    (*lease).expires = 1 as libc::c_int as time_t;
    (*lease).hwaddr_len = 256 as libc::c_int;
    (*lease).next = leases;
    leases = lease;
    file_dirty = 1 as libc::c_int;
    leases_left -= 1;
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease4_allocate(mut addr: in_addr)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = lease_allocate();
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
pub unsafe extern "C" fn lease6_allocate(mut addrp: *mut in6_addr,
                                         mut lease_type: libc::c_int)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = lease_allocate();
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
        (*lease).flags |= 4 as libc::c_int | 256 as libc::c_int;
        file_dirty = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_iaid(mut lease: *mut dhcp_lease,
                                        mut iaid: libc::c_uint) {
    if (*lease).iaid != iaid {
        (*lease).iaid = iaid;
        (*lease).flags |= 2 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_hwaddr(mut lease: *mut dhcp_lease,
                                          mut hwaddr: *const libc::c_uchar,
                                          mut clid: *const libc::c_uchar,
                                          mut hw_len: libc::c_int,
                                          mut hw_type: libc::c_int,
                                          mut clid_len: libc::c_int,
                                          mut now: time_t,
                                          mut force: libc::c_int) {
    let mut change: libc::c_int = force;
    (*lease).flags |= 128 as libc::c_int;
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
        (*lease).flags |= 2 as libc::c_int;
        file_dirty = 1 as libc::c_int
        /* run script on change */
    }
    /* only update clid when one is available, stops packets
     without a clid removing the record. Lease init uses
     clid_len == 0 for no clid. */
    if clid_len != 0 as libc::c_int && !clid.is_null() {
        if (*lease).clid.is_null() { (*lease).clid_len = 0 as libc::c_int }
        if (*lease).clid_len != clid_len {
            (*lease).flags |= 4 as libc::c_int;
            file_dirty = 1 as libc::c_int;
            free((*lease).clid as *mut libc::c_void);
            (*lease).clid =
                whine_malloc(clid_len as size_t) as *mut libc::c_uchar;
            if (*lease).clid.is_null() { return }
            change = 1 as libc::c_int
        } else if memcmp((*lease).clid as *const libc::c_void,
                         clid as *const libc::c_void,
                         clid_len as libc::c_ulong) != 0 as libc::c_int {
            (*lease).flags |= 4 as libc::c_int;
            file_dirty = 1 as libc::c_int;
            change = 1 as libc::c_int
        }
        (*lease).clid_len = clid_len;
        memcpy((*lease).clid as *mut libc::c_void,
               clid as *const libc::c_void, clid_len as libc::c_ulong);
    }
    if change != 0 { slaac_add_addrs(lease, now, force); };
}
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
    (*lease).fqdn = 0 as *mut libc::c_char;
    (*lease).hostname = (*lease).fqdn;
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_hostname(mut lease: *mut dhcp_lease,
                                            mut name: *const libc::c_char,
                                            mut auth: libc::c_int,
                                            mut domain: *mut libc::c_char,
                                            mut config_domain:
                                                *mut libc::c_char) {
    let mut lease_tmp: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_fqdn: *mut libc::c_char = 0 as *mut libc::c_char;
    if !config_domain.is_null() &&
           (domain.is_null() || hostname_isequal(domain, config_domain) == 0)
       {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
                  b"Ignoring domain %s for DHCP host name %s\x00" as *const u8
                      as *const libc::c_char, config_domain, name);
    }
    if !(*lease).hostname.is_null() && !name.is_null() &&
           hostname_isequal((*lease).hostname, name) != 0 {
        if auth != 0 { (*lease).flags |= 8 as libc::c_int }
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
                    if (*lease).flags &
                           (64 as libc::c_int | 32 as libc::c_int) != 0 {
                        if (*lease_tmp).flags &
                               (64 as libc::c_int | 32 as libc::c_int) == 0 {
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
                    } else if (*lease_tmp).flags &
                                  (64 as libc::c_int | 32 as libc::c_int) != 0
                     {
                        current_block_23 = 17833034027772472439;
                    } else { current_block_23 = 1608152415753874203; }
                    match current_block_23 {
                        17833034027772472439 => { }
                        _ => {
                            if (*lease_tmp).flags & 8 as libc::c_int != 0 &&
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
    if auth != 0 { (*lease).flags |= 8 as libc::c_int }
    file_dirty = 1 as libc::c_int;
    dns_dirty = 1 as libc::c_int;
    (*lease).flags |= 2 as libc::c_int;
    /* another lease for the same DUID is OK for IPv6 */
    /* run script on change */
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_interface(mut lease: *mut dhcp_lease,
                                             mut interface: libc::c_int,
                                             mut now: time_t) {
    if (*lease).last_interface == interface { return }
    (*lease).last_interface = interface;
    (*lease).flags |= 2 as libc::c_int;
    slaac_add_addrs(lease, now, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rerun_scripts() {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).flags |= 2 as libc::c_int;
        lease = (*lease).next
    };
}
/* deleted leases get transferred to the old_leases list.
   remove them here, after calling the lease change
   script. Also run the lease change script on new/modified leases.

   Return zero if nothing to do. */
#[no_mangle]
pub unsafe extern "C" fn do_script_run(mut now: time_t) -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if !old_leases.is_null() {
        lease = old_leases;
        /* If the lease still has an old_hostname, do the "old" action on that first */
        if !(*lease).old_hostname.is_null() {
            queue_script(2 as libc::c_int, lease, (*lease).old_hostname, now);
            free((*lease).old_hostname as *mut libc::c_void);
            (*lease).old_hostname = 0 as *mut libc::c_char;
            return 1 as libc::c_int
        } else {
            let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
            let mut tmp: *mut slaac_address = 0 as *mut slaac_address;
            slaac = (*lease).slaac_address;
            while !slaac.is_null() {
                tmp = (*slaac).next;
                free(slaac as *mut libc::c_void);
                slaac = tmp
            }
            kill_name(lease);
            queue_script(1 as libc::c_int, lease, (*lease).old_hostname, now);
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
            queue_script(2 as libc::c_int, lease, (*lease).old_hostname, now);
            free((*lease).old_hostname as *mut libc::c_void);
            (*lease).old_hostname = 0 as *mut libc::c_char;
            return 1 as libc::c_int
        }
        lease = (*lease).next
    }
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (1 as libc::c_int | 2 as libc::c_int) != 0 ||
               (*lease).flags & 4 as libc::c_int != 0 &&
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
               (*lease).flags & 256 as libc::c_int != 0 &&
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
            queue_script(if (*lease).flags & 1 as libc::c_int != 0 {
                             4 as libc::c_int
                         } else { 3 as libc::c_int }, lease,
                         if !(*lease).fqdn.is_null() {
                             (*lease).fqdn
                         } else { (*lease).hostname }, now);
            (*lease).flags &=
                !(1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int |
                      256 as libc::c_int);
            /* this is used for the "add" call, then junked, since they're not in the database */
            free((*lease).extradata as *mut libc::c_void);
            (*lease).extradata = 0 as *mut libc::c_uchar;
            return 1 as libc::c_int
        }
        lease = (*lease).next
    }
    return 0 as libc::c_int;
    /* nothing to do */
}
/* delim == -1 -> delim = 0, but embedded 0s, creating extra records, are OK. */
#[no_mangle]
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
        let mut newsz: size_t =
            (*lease).extradata_len.wrapping_add(len).wrapping_add(100 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                as size_t;
        let mut new: *mut libc::c_uchar =
            whine_malloc(newsz) as *mut libc::c_uchar;
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
