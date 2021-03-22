#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]

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
use crate::defines::{dnsmasq_daemon, irec, iname, msghdr};
use crate::util::expand_buf;
use socket2::Socket;
use std::io;

pub fn dhcp_common_init(daemon: &mut dnsmasq_daemon) {
    /* These each hold a DHCP option max size 255
     and get a terminating zero added */
    daemon.dhcp_buff = Vec::new();
    daemon.dhcp_buff2 = Vec::new();
    daemon.dhcp_buff3 = Vec::new();
    /* dhcp_packet is used by v4 and v6, outpacket only by v6 
     sizeof(struct dhcp_packet) is as good an initial size as any,
     even for v6 */
    daemon.dhcp_packet = Default::default();
    // expand_buf(&mut daemon.dhcp_packet,
    //            ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong);
    // if !daemon.dhcp6.is_null() {
    //     expand_buf(&mut daemon.outpacket,
    //                ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong);
    // };
}

#[no_mangle]
pub unsafe extern "C" fn recv_dhcp_packet(mut fd: libc::c_int,
                                          mut msg: &mut msghdr) -> ssize_t {
    let mut sz: ssize_t = 0;
    let mut new_sz: ssize_t = 0;
    loop  {
        (*msg).msg_flags = 0 as libc::c_int;
        loop  {
            sz =
                recvmsg(fd, msg,
                        MSG_PEEK as libc::c_int | MSG_TRUNC as libc::c_int);
            if !(sz == -(1 as libc::c_int) as libc::c_long &&
                     *__errno_location() == 4 as libc::c_int) {
                break ;
            }
        }
        if sz == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t
        }
        if (*msg).msg_flags & MSG_TRUNC as libc::c_int == 0 { break ; }
        /* Very new Linux kernels return the actual size needed, 
	 older ones always return truncated size */
        if sz as size_t == (*(*msg).msg_iov).iov_len {
            if expand_buf((*msg).msg_iov,
                          (sz + 100 as libc::c_int as libc::c_long) as size_t)
                   == 0 {
                return -(1 as libc::c_int) as ssize_t
            }
        } else { expand_buf((*msg).msg_iov, sz as size_t); break ; }
    }
    loop  {
        new_sz = recvmsg(fd, msg, 0 as libc::c_int);
        if !(new_sz == -(1 as libc::c_int) as libc::c_long &&
                 *__errno_location() == 4 as libc::c_int) {
            break ;
        }
    }
    /* Some kernels seem to ignore MSG_PEEK, and dequeue the packet anyway. 
     If that happens we get EAGAIN here because the socket is non-blocking.
     Use the result of the original testing recvmsg as long as the buffer
     was big enough. There's a small race here that may lose the odd packet,
     but it's UDP anyway. */
    if new_sz == -(1 as libc::c_int) as libc::c_long &&
           (*__errno_location() == 11 as libc::c_int ||
                *__errno_location() == 11 as libc::c_int) {
        new_sz = sz
    }
    return if (*msg).msg_flags & MSG_TRUNC as libc::c_int != 0 {
               -(1 as libc::c_int) as libc::c_long
           } else { new_sz };
}
#[no_mangle]
pub unsafe extern "C" fn run_tag_if(mut tags: *mut dhcp_netid)
 -> *mut dhcp_netid {
    let mut exprs: *mut tag_if = 0 as *mut tag_if;
    let mut list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
    exprs = (*dnsmasq_daemon).tag_if;
    while !exprs.is_null() {
        if match_netid((*exprs).tag, tags, 1 as libc::c_int) != 0 {
            list = (*exprs).set;
            while !list.is_null() {
                (*(*list).list).next = tags;
                tags = (*list).list;
                list = (*list).next
            }
        }
        exprs = (*exprs).next
    }
    return tags;
}
#[no_mangle]
pub unsafe extern "C" fn option_filter(mut tags: *mut dhcp_netid,
                                       mut context_tags: *mut dhcp_netid,
                                       mut opts: *mut dhcp_opt)
 -> *mut dhcp_netid {
    let mut tagif: *mut dhcp_netid = run_tag_if(tags);
    let mut opt: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut tmp: *mut dhcp_opt = 0 as *mut dhcp_opt;
    /* flag options which are valid with the current tag set (sans context tags) */
    opt = opts;
    while !opt.is_null() {
        (*opt).flags &= !(4096 as libc::c_int);
        if (*opt).flags &
               (4 as libc::c_int | 256 as libc::c_int | 2048 as libc::c_int)
               == 0 && match_netid((*opt).netid, tagif, 0 as libc::c_int) != 0
           {
            (*opt).flags |= 4096 as libc::c_int
        }
        opt = (*opt).next
    }
    /* now flag options which are valid, including the context tags,
     otherwise valid options are inhibited if we found a higher priority one above */
    if !context_tags.is_null() {
        let mut last_tag: *mut dhcp_netid = 0 as *mut dhcp_netid;
        last_tag = context_tags;
        while !(*last_tag).next.is_null() { last_tag = (*last_tag).next }
        (*last_tag).next = tags;
        tagif = run_tag_if(context_tags);
        /* reset stuff with tag:!<tag> which now matches. */
        opt = opts;
        while !opt.is_null() {
            if (*opt).flags &
                   (4 as libc::c_int | 256 as libc::c_int |
                        2048 as libc::c_int) == 0 &&
                   (*opt).flags & 4096 as libc::c_int != 0 &&
                   match_netid((*opt).netid, tagif, 0 as libc::c_int) == 0 {
                (*opt).flags &= !(4096 as libc::c_int)
            }
            opt = (*opt).next
        }
        opt = opts;
        while !opt.is_null() {
            if (*opt).flags &
                   (4 as libc::c_int | 256 as libc::c_int |
                        2048 as libc::c_int | 4096 as libc::c_int) == 0 &&
                   match_netid((*opt).netid, tagif, 0 as libc::c_int) != 0 {
                let mut tmp_0: *mut dhcp_opt = 0 as *mut dhcp_opt;
                tmp_0 = opts;
                while !tmp_0.is_null() {
                    if (*tmp_0).opt == (*opt).opt && !(*opt).netid.is_null()
                           && (*tmp_0).flags & 4096 as libc::c_int != 0 {
                        break ;
                    }
                    tmp_0 = (*tmp_0).next
                }
                if tmp_0.is_null() { (*opt).flags |= 4096 as libc::c_int }
            }
            opt = (*opt).next
        }
    }
    /* now flag untagged options which are not overridden by tagged ones */
    opt = opts;
    while !opt.is_null() {
        if (*opt).flags &
               (4 as libc::c_int | 256 as libc::c_int | 2048 as libc::c_int |
                    4096 as libc::c_int) == 0 && (*opt).netid.is_null() {
            tmp = opts;
            while !tmp.is_null() {
                if (*tmp).opt == (*opt).opt &&
                       (*tmp).flags & 4096 as libc::c_int != 0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() {
                (*opt).flags |= 4096 as libc::c_int
            } else if (*tmp).netid.is_null() {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"Ignoring duplicate dhcp-option %d\x00" as
                              *const u8 as *const libc::c_char, (*tmp).opt);
            }
        }
        opt = (*opt).next
    }
    /* Finally, eliminate duplicate options later in the chain, and therefore earlier in the config file. */
    opt = opts;
    while !opt.is_null() {
        if (*opt).flags & 4096 as libc::c_int != 0 {
            tmp = (*opt).next;
            while !tmp.is_null() {
                if (*tmp).opt == (*opt).opt {
                    (*tmp).flags &= !(4096 as libc::c_int)
                }
                tmp = (*tmp).next
            }
        }
        opt = (*opt).next
    }
    return tagif;
}
/* Is every member of check matched by a member of pool? 
   If tagnotneeded, untagged is OK */
#[no_mangle]
pub unsafe extern "C" fn match_netid(mut check: *mut dhcp_netid,
                                     mut pool: *mut dhcp_netid,
                                     mut tagnotneeded: libc::c_int)
 -> libc::c_int {
    let mut tmp1: *mut dhcp_netid = 0 as *mut dhcp_netid;
    if check.is_null() && tagnotneeded == 0 { return 0 as libc::c_int }
    while !check.is_null() {
        /* '#' for not is for backwards compat. */
        if *(*check).net.offset(0 as libc::c_int as isize) as libc::c_int !=
               '!' as i32 &&
               *(*check).net.offset(0 as libc::c_int as isize) as libc::c_int
                   != '#' as i32 {
            tmp1 = pool;
            while !tmp1.is_null() {
                if strcmp((*check).net, (*tmp1).net) == 0 as libc::c_int {
                    break ;
                }
                tmp1 = (*tmp1).next
            }
            if tmp1.is_null() { return 0 as libc::c_int }
        } else {
            tmp1 = pool;
            while !tmp1.is_null() {
                if strcmp((*check).net.offset(1 as libc::c_int as isize),
                          (*tmp1).net) == 0 as libc::c_int {
                    return 0 as libc::c_int
                }
                tmp1 = (*tmp1).next
            }
        }
        check = (*check).next
    }
    return 1 as libc::c_int;
}
/* return domain or NULL if none. */
#[no_mangle]
pub unsafe extern "C" fn strip_hostname(mut hostname: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut dot: *mut libc::c_char =
        strchr(hostname, '.' as i32); /* truncate */
    if dot.is_null() { return 0 as *mut libc::c_char }
    *dot = 0 as libc::c_int as libc::c_char;
    if strlen(dot.offset(1 as libc::c_int as isize)) !=
           0 as libc::c_int as libc::c_ulong {
        return dot.offset(1 as libc::c_int as isize)
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn log_tags(mut netid: *mut dhcp_netid,
                                  mut xid: u32_0) {
    if !netid.is_null() &&
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
               != 0 {
        let mut s: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
        *s = 0 as libc::c_int as libc::c_char;
        while !netid.is_null() {
            /* kill dupes. */
            let mut n: *mut dhcp_netid = 0 as *mut dhcp_netid;
            n = (*netid).next;
            while !n.is_null() {
                if strcmp((*netid).net, (*n).net) == 0 as libc::c_int {
                    break ;
                }
                n = (*n).next
            }
            if n.is_null() {
                strncat(s, (*netid).net,
                        ((1025 as libc::c_int - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen(s)));
                if !(*netid).next.is_null() {
                    strncat(s, b", \x00" as *const u8 as *const libc::c_char,
                            ((1025 as libc::c_int - 1 as libc::c_int) as
                                 libc::c_ulong).wrapping_sub(strlen(s)));
                }
            }
            netid = (*netid).next
        }
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u tags: %s\x00" as *const u8 as *const libc::c_char, xid,
                  s);
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_bytes(mut o: *mut dhcp_opt,
                                     mut p: *mut libc::c_uchar,
                                     mut len: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*o).len > len { return 0 as libc::c_int }
    if (*o).len == 0 as libc::c_int { return 1 as libc::c_int }
    if (*o).flags & 512 as libc::c_int != 0 {
        if memcmp_masked((*o).val, p, (*o).len, (*o).u.wildcard_mask) != 0 {
            return 1 as libc::c_int
        }
    } else {
        i = 0 as libc::c_int;
        while i <= len - (*o).len {
            if memcmp((*o).val as *const libc::c_void,
                      p.offset(i as isize) as *const libc::c_void,
                      (*o).len as libc::c_ulong) == 0 as libc::c_int {
                return 1 as libc::c_int
            }
            if (*o).flags & 2 as libc::c_int != 0 {
                i += 1
            } else { i += (*o).len }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn config_has_mac(mut config: *mut dhcp_config,
                                        mut hwaddr: *mut libc::c_uchar,
                                        mut len: libc::c_int,
                                        mut type_0: libc::c_int)
 -> libc::c_int {
    let mut conf_addr: *mut hwaddr_config = 0 as *mut hwaddr_config;
    conf_addr = (*config).hwaddr;
    while !conf_addr.is_null() {
        if (*conf_addr).wildcard_mask == 0 as libc::c_int as libc::c_uint &&
               (*conf_addr).hwaddr_len == len &&
               ((*conf_addr).hwaddr_type == type_0 ||
                    (*conf_addr).hwaddr_type == 0 as libc::c_int) &&
               memcmp((*conf_addr).hwaddr.as_mut_ptr() as *const libc::c_void,
                      hwaddr as *const libc::c_void, len as libc::c_ulong) ==
                   0 as libc::c_int {
            return 1 as libc::c_int
        }
        conf_addr = (*conf_addr).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_config_in_context(mut context: *mut dhcp_context,
                                          mut config: *mut dhcp_config)
 -> libc::c_int {
    if context.is_null() {
        /* called via find_config() from lease_update_from_configs() */
        return 1 as libc::c_int
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 17 as libc::c_int != 0 {
        let mut addr_list: *mut addrlist = 0 as *mut addrlist;
        if (*config).flags & 4096 as libc::c_int as libc::c_uint == 0 {
            return 1 as libc::c_int
        }
        while !context.is_null() {
            addr_list = (*config).addr6;
            while !addr_list.is_null() {
                if (*addr_list).flags & 16 as libc::c_int != 0 &&
                       (*context).prefix == 64 as libc::c_int {
                    return 1 as libc::c_int
                }
                if is_same_net6(&mut (*addr_list).addr.addr6,
                                &mut (*context).start6, (*context).prefix) !=
                       0 {
                    return 1 as libc::c_int
                }
                addr_list = (*addr_list).next
            }
            context = (*context).current
        }
    } else {
        if (*config).flags & 32 as libc::c_int as libc::c_uint == 0 {
            return 1 as libc::c_int
        }
        while !context.is_null() {
            if (*config).flags & 32 as libc::c_int as libc::c_uint != 0 &&
                   is_same_net((*config).addr, (*context).start,
                               (*context).netmask) != 0 {
                return 1 as libc::c_int
            }
            context = (*context).current
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_config_match(mut configs: *mut dhcp_config,
                                       mut context: *mut dhcp_context,
                                       mut clid: *mut libc::c_uchar,
                                       mut clid_len: libc::c_int,
                                       mut hwaddr: *mut libc::c_uchar,
                                       mut hw_len: libc::c_int,
                                       mut hw_type: libc::c_int,
                                       mut hostname: *mut libc::c_char,
                                       mut tags: *mut dhcp_netid,
                                       mut tag_not_needed: libc::c_int)
 -> *mut dhcp_config {
    let mut count: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut candidate: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut conf_addr: *mut hwaddr_config = 0 as *mut hwaddr_config;
    if !clid.is_null() {
        config = configs;
        while !config.is_null() {
            if (*config).flags & 2 as libc::c_int as libc::c_uint != 0 {
                if (*config).clid_len == clid_len &&
                       memcmp((*config).clid as *const libc::c_void,
                              clid as *const libc::c_void,
                              clid_len as libc::c_ulong) == 0 as libc::c_int
                       && is_config_in_context(context, config) != 0 &&
                       match_netid((*config).filter, tags, tag_not_needed) !=
                           0 {
                    return config
                }
                /* dhcpcd prefixes ASCII client IDs by zero which is wrong, but we try and
	     cope with that here. This is IPv4 only. context==NULL implies IPv4, 
	     see lease_update_from_configs() */
                if (context.is_null() ||
                        (*context).flags as libc::c_uint &
                            (1 as libc::c_uint) << 17 as libc::c_int == 0) &&
                       *clid as libc::c_int == 0 as libc::c_int &&
                       (*config).clid_len == clid_len - 1 as libc::c_int &&
                       memcmp((*config).clid as *const libc::c_void,
                              clid.offset(1 as libc::c_int as isize) as
                                  *const libc::c_void,
                              (clid_len - 1 as libc::c_int) as libc::c_ulong)
                           == 0 as libc::c_int &&
                       is_config_in_context(context, config) != 0 &&
                       match_netid((*config).filter, tags, tag_not_needed) !=
                           0 {
                    return config
                }
            }
            config = (*config).next
        }
    }
    if !hwaddr.is_null() {
        config = configs;
        while !config.is_null() {
            if config_has_mac(config, hwaddr, hw_len, hw_type) != 0 &&
                   is_config_in_context(context, config) != 0 &&
                   match_netid((*config).filter, tags, tag_not_needed) != 0 {
                return config
            }
            config = (*config).next
        }
    }
    if !hostname.is_null() && !context.is_null() {
        config = configs;
        while !config.is_null() {
            if (*config).flags & 16 as libc::c_int as libc::c_uint != 0 &&
                   hostname_isequal((*config).hostname, hostname) != 0 &&
                   is_config_in_context(context, config) != 0 &&
                   match_netid((*config).filter, tags, tag_not_needed) != 0 {
                return config
            }
            config = (*config).next
        }
    }
    if hwaddr.is_null() { return 0 as *mut dhcp_config }
    /* use match with fewest wildcard octets */
    candidate = 0 as *mut dhcp_config;
    count = 0 as libc::c_int;
    config = configs;
    while !config.is_null() {
        if is_config_in_context(context, config) != 0 &&
               match_netid((*config).filter, tags, tag_not_needed) != 0 {
            conf_addr = (*config).hwaddr;
            while !conf_addr.is_null() {
                if (*conf_addr).wildcard_mask !=
                       0 as libc::c_int as libc::c_uint &&
                       (*conf_addr).hwaddr_len == hw_len &&
                       ((*conf_addr).hwaddr_type == hw_type ||
                            (*conf_addr).hwaddr_type == 0 as libc::c_int) &&
                       {
                           new =
                               memcmp_masked((*conf_addr).hwaddr.as_mut_ptr(),
                                             hwaddr, hw_len,
                                             (*conf_addr).wildcard_mask);
                           (new) > count
                       } {
                    count = new;
                    candidate = config
                }
                conf_addr = (*conf_addr).next
            }
        }
        config = (*config).next
    }
    return candidate;
}
/* Find tagged configs first. */
#[no_mangle]
pub unsafe extern "C" fn find_config(mut configs: *mut dhcp_config,
                                     mut context: *mut dhcp_context,
                                     mut clid: *mut libc::c_uchar,
                                     mut clid_len: libc::c_int,
                                     mut hwaddr: *mut libc::c_uchar,
                                     mut hw_len: libc::c_int,
                                     mut hw_type: libc::c_int,
                                     mut hostname: *mut libc::c_char,
                                     mut tags: *mut dhcp_netid)
 -> *mut dhcp_config {
    let mut ret: *mut dhcp_config =
        find_config_match(configs, context, clid, clid_len, hwaddr, hw_len,
                          hw_type, hostname, tags, 0 as libc::c_int);
    if ret.is_null() {
        ret =
            find_config_match(configs, context, clid, clid_len, hwaddr,
                              hw_len, hw_type, hostname, tags,
                              1 as libc::c_int)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_update_configs(mut configs: *mut dhcp_config) {
    /* Some people like to keep all static IP addresses in /etc/hosts.
     This goes through /etc/hosts and sets static addresses for any DHCP config
     records which don't have an address and whose name matches. 
     We take care to maintain the invariant that any IP address can appear
     in at most one dhcp-host. Since /etc/hosts can be re-read by SIGHUP, 
     restore the status-quo ante first. */
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut conf_tmp: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut crec: *mut crec = 0 as *mut crec;
    let mut prot: libc::c_int = 2 as libc::c_int;
    config = configs;
    while !config.is_null() {
        if (*config).flags & 512 as libc::c_int as libc::c_uint != 0 {
            (*config).flags &=
                !(32 as libc::c_int | 512 as libc::c_int) as libc::c_uint
        }
        if (*config).flags & 16384 as libc::c_int as libc::c_uint != 0 {
            (*config).flags &=
                !(4096 as libc::c_int | 16384 as libc::c_int) as libc::c_uint
        }
        config = (*config).next
    }
    loop  {
        if (*dnsmasq_daemon).port != 0 as libc::c_int {
            let mut current_block_27: u64;
            config = configs;
            while !config.is_null() {
                let mut conflags: libc::c_int = 32 as libc::c_int;
                let mut cacheflags: libc::c_int =
                    ((1 as libc::c_uint) << 7 as libc::c_int) as libc::c_int;
                if prot == 10 as libc::c_int {
                    conflags = 4096 as libc::c_int;
                    cacheflags =
                        ((1 as libc::c_uint) << 8 as libc::c_int) as
                            libc::c_int
                }
                if (*config).flags & conflags as libc::c_uint == 0 &&
                       (*config).flags & 16 as libc::c_int as libc::c_uint !=
                           0 &&
                       {
                           crec =
                               cache_find_by_name(0 as *mut crec,
                                                  (*config).hostname,
                                                  0 as libc::c_int as time_t,
                                                  cacheflags as libc::c_uint);
                           !crec.is_null()
                       } &&
                       (*crec).flags & (1 as libc::c_uint) << 6 as libc::c_int
                           != 0 {
                    if !cache_find_by_name(crec, (*config).hostname,
                                           0 as libc::c_int as time_t,
                                           cacheflags as
                                               libc::c_uint).is_null() {
                        /* use primary (first) address */
                        while !crec.is_null() &&
                                  (*crec).flags &
                                      (1 as libc::c_uint) << 2 as libc::c_int
                                      == 0 {
                            crec =
                                cache_find_by_name(crec, (*config).hostname,
                                                   0 as libc::c_int as time_t,
                                                   cacheflags as libc::c_uint)
                        } /* should be never */
                        if crec.is_null() {
                            current_block_27 = 3640593987805443782;
                        } else {
                            inet_ntop(prot,
                                      &mut (*crec).addr as *mut all_addr as
                                          *const libc::c_void,
                                      (*dnsmasq_daemon).addrbuff,
                                      46 as libc::c_int as socklen_t);
                            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                          4 as libc::c_int,
                                      b"%s has more than one address in hostsfile, using %s for DHCP\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*config).hostname,
                                      (*dnsmasq_daemon).addrbuff);
                            current_block_27 = 1109700713171191020;
                        }
                    } else { current_block_27 = 1109700713171191020; }
                    match current_block_27 {
                        3640593987805443782 => { }
                        _ => {
                            if prot == 2 as libc::c_int &&
                                   {
                                       conf_tmp =
                                           config_find_by_address(configs,
                                                                  (*crec).addr.addr4);
                                       (conf_tmp.is_null()) ||
                                           conf_tmp == config
                                   } {
                                (*config).addr = (*crec).addr.addr4;
                                (*config).flags |=
                                    (32 as libc::c_int | 512 as libc::c_int)
                                        as libc::c_uint
                            } else if prot == 10 as libc::c_int &&
                                          {
                                              conf_tmp =
                                                  config_find_by_address6(configs,
                                                                          0 as
                                                                              *mut in6_addr,
                                                                          0 as
                                                                              libc::c_int,
                                                                          &mut (*crec).addr.addr6);
                                              (conf_tmp.is_null()) ||
                                                  conf_tmp == config
                                          } {
                                /* host must have exactly one address if comming from /etc/hosts. */
                                if (*config).addr6.is_null() &&
                                       {
                                           (*config).addr6 =
                                               whine_malloc(::std::mem::size_of::<addrlist>()
                                                                as
                                                                libc::c_ulong)
                                                   as *mut addrlist;
                                           !(*config).addr6.is_null()
                                       } {
                                    (*(*config).addr6).next =
                                        0 as *mut addrlist;
                                    (*(*config).addr6).flags =
                                        0 as libc::c_int
                                }
                                if !(*config).addr6.is_null() &&
                                       (*(*config).addr6).next.is_null() &&
                                       (*(*config).addr6).flags &
                                           (16 as libc::c_int |
                                                8 as libc::c_int) == 0 {
                                    memcpy(&mut (*(*config).addr6).addr.addr6
                                               as *mut in6_addr as
                                               *mut libc::c_void,
                                           &mut (*crec).addr.addr6 as
                                               *mut in6_addr as
                                               *const libc::c_void,
                                           16 as libc::c_int as
                                               libc::c_ulong);
                                    (*config).flags |=
                                        (4096 as libc::c_int |
                                             16384 as libc::c_int) as
                                            libc::c_uint
                                }
                            } else {
                                inet_ntop(prot,
                                          &mut (*crec).addr as *mut all_addr
                                              as *const libc::c_void,
                                          (*dnsmasq_daemon).addrbuff,
                                          46 as libc::c_int as socklen_t);
                                my_syslog((3 as libc::c_int) <<
                                              3 as libc::c_int |
                                              4 as libc::c_int,
                                          b"duplicate IP address %s (%s) in dhcp-config directive\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*dnsmasq_daemon).addrbuff,
                                          (*config).hostname);
                            }
                        }
                    }
                }
                config = (*config).next
            }
        }
        if !(prot == 2 as libc::c_int) { break ; }
        prot = 10 as libc::c_int
    };
}

pub fn whichdevice(daemon: &mut dnsmasq_daemon) -> Option<String> {
    /* If we are doing DHCP on exactly one interface, and running linux, do SO_BINDTODEVICE
     to that device. This is for the use case of  (eg) OpenStack, which runs a new
     dnsmasq instance for each VLAN interface it creates. Without the BINDTODEVICE, 
     individual processes don't always see the packets they should.
     SO_BINDTODEVICE is only available Linux. 

     Note that if wildcards are used in --interface, or --interface is not used at all,
     or a configured interface doesn't yet exist, then more interfaces may arrive later, 
     so we can't safely assert there is only one interface and proceed.
*/
    let mut iface: irec = Default::default();
    let mut found: irec = Default::default();
    let mut if_tmp: iname = Default::default();
    // if daemon.if_names.is_null() { return 0 as *mut libc::c_char }
    if_tmp = daemon.if_names;
    while !if_tmp.is_null() {
        if !(*if_tmp).name.is_null() &&
               ((*if_tmp).used == 0 ||
                    !strchr((*if_tmp).name, '*' as i32).is_null()) {
            return None;
        }
        if_tmp = (*if_tmp).next
    }
    found = 0;
    iface = daemon.interfaces;
    while !iface.is_null() {
        if iface.dhcp_ok != 0 {
            if found.is_null() {
                found = iface
            } else if strcmp((*found).name, iface.name) != 0 as libc::c_int
             {
                return None;
            }
            /* more than one. */
        }
        iface = iface.next
    }
    if !found.is_null() { return (*found).name }
    return None;
}

pub fn bind_to_device(device: &String, fd: &mut Socket) -> io::Result<()> {
    return fd.bind_device(device.as_bytes())
}

static mut opttab: [opttab_t; 74] =
    [{
         let mut init =
             opttab_t{name:
                          b"netmask\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 1 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"time-offset\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 2 as libc::c_int as u16_0,
                      size: 4 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"router\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 3 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"dns-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 6 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"log-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 7 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"lpr-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 9 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"hostname\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 12 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x1000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"boot-file-size\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 13 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"domain-name\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 15 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"swap-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 16 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"root-path\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 17 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"extension-path\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 18 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ip-forward-enable\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 19 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"non-local-source-routing\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 20 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"policy-filter\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 21 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"max-datagram-reassembly\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 22 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"default-ttl\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 23 as libc::c_int as u16_0,
                      size:
                          (1 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"mtu\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 26 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"all-subnets-local\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 27 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"broadcast\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 28 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x8000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"router-discovery\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 31 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"router-solicitation\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 32 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"static-route\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 33 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"trailer-encapsulation\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 34 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"arp-timeout\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 35 as libc::c_int as u16_0,
                      size:
                          (4 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ethernet-encap\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 36 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tcp-ttl\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 37 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tcp-keepalive\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 38 as libc::c_int as u16_0,
                      size:
                          (4 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 40 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 41 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 42 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-encap\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 43 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-ns\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 44 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-dd\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 45 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-nodetype\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 46 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-scope\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 47 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"x-windows-fs\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 48 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"x-windows-dm\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 49 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"requested-address\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 50 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x8000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"lease-time\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 51 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x200 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"option-overload\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 52 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"message-type\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 53 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"server-identifier\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 54 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x8000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"parameter-request\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 55 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"message\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 56 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"max-message-size\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 57 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"T1\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 58 as libc::c_int as u16_0,
                      size: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"T2\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 59 as libc::c_int as u16_0,
                      size: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 60 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 61 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 64 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 65 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tftp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 66 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"bootfile-name\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 67 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"mobile-ip-home\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 68 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"smtp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 69 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"pop3-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 70 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 71 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"irc-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 74 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"user-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 77 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"rapid-commit\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 80 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"FQDN\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 81 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"agent-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 82 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-arch\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 93 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-interface-id\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 94 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-machine-id\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 97 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"subnet-select\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 118 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"domain-search\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 119 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sip-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 120 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"classless-static-route\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 121 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-id-encap\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 125 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tftp-server-address\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 150 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"server-ip-address\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 255 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name: 0 as *const libc::c_char as *mut libc::c_char,
                      val: 0 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     }];
static mut opttab6: [opttab_t; 28] =
    [{
         let mut init =
             opttab_t{name:
                          b"client-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 1 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"server-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 2 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ia-na\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 3 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ia-ta\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 4 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"iaaddr\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 5 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"oro\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 6 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"preference\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 7 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"unicast\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 12 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"status\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 13 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"rapid-commit\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 14 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"user-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 15 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x800 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 16 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x800 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-opts\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 17 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sip-server-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 21 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sip-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 22 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"dns-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 23 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"domain-search\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 24 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 27 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 28 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 29 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 30 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 31 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"information-refresh-time\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 32 as libc::c_int as u16_0,
                      size: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"FQDN\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 39 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x4000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 56 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"bootfile-url\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 59 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"bootfile-param\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 60 as libc::c_int as u16_0,
                      size: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name: 0 as *const libc::c_char as *mut libc::c_char,
                      val: 0 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn display_opts() {
    let mut i: libc::c_int = 0;
    printf(b"Known DHCP options:\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while !opttab[i as usize].name.is_null() {
        if opttab[i as usize].size as libc::c_int & 0x2000 as libc::c_int == 0
           {
            printf(b"%3d %s\n\x00" as *const u8 as *const libc::c_char,
                   opttab[i as usize].val as libc::c_int,
                   opttab[i as usize].name);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn display_opts6() {
    let mut i: libc::c_int = 0;
    printf(b"Known DHCPv6 options:\n\x00" as *const u8 as
               *const libc::c_char);
    i = 0 as libc::c_int;
    while !opttab6[i as usize].name.is_null() {
        if opttab6[i as usize].size as libc::c_int & 0x2000 as libc::c_int ==
               0 {
            printf(b"%3d %s\n\x00" as *const u8 as *const libc::c_char,
                   opttab6[i as usize].val as libc::c_int,
                   opttab6[i as usize].name);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn lookup_dhcp_opt(mut prot: libc::c_int,
                                         mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut t: *const opttab_t = 0 as *const opttab_t;
    let mut i: libc::c_int = 0;
    if prot == 10 as libc::c_int {
        t = opttab6.as_ptr()
    } else { t = opttab.as_ptr() }
    i = 0 as libc::c_int;
    while !(*t.offset(i as isize)).name.is_null() {
        if strcasecmp((*t.offset(i as isize)).name, name) == 0 as libc::c_int
           {
            return (*t.offset(i as isize)).val as libc::c_int
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookup_dhcp_len(mut prot: libc::c_int,
                                         mut val: libc::c_int)
 -> libc::c_int {
    let mut t: *const opttab_t = 0 as *const opttab_t;
    let mut i: libc::c_int = 0;
    if prot == 10 as libc::c_int {
        t = opttab6.as_ptr()
    } else { t = opttab.as_ptr() }
    i = 0 as libc::c_int;
    while !(*t.offset(i as isize)).name.is_null() {
        if val == (*t.offset(i as isize)).val as libc::c_int {
            return (*t.offset(i as isize)).size as libc::c_int &
                       !(0x400 as libc::c_int)
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn option_string(mut prot: libc::c_int,
                                       mut opt: libc::c_uint,
                                       mut val: *mut libc::c_uchar,
                                       mut opt_len: libc::c_int,
                                       mut buf: *mut libc::c_char,
                                       mut buf_len: libc::c_int)
 -> *mut libc::c_char {
    let mut o: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nodecode: libc::c_int = 0 as libc::c_int;
    let mut ot: *const opttab_t = opttab.as_ptr();
    if prot == 10 as libc::c_int { ot = opttab6.as_ptr() }
    o = 0 as libc::c_int;
    while !(*ot.offset(o as isize)).name.is_null() {
        if (*ot.offset(o as isize)).val as libc::c_uint == opt {
            if !buf.is_null() {
                memset(buf as *mut libc::c_void, 0 as libc::c_int,
                       buf_len as libc::c_ulong);
                if (*ot.offset(o as isize)).size as libc::c_int &
                       0x8000 as libc::c_int != 0 {
                    let mut addr: all_addr =
                        all_addr{addr4: in_addr{s_addr: 0,},};
                    let mut addr_len: libc::c_int = 4 as libc::c_int;
                    if prot == 10 as libc::c_int {
                        addr_len = 16 as libc::c_int
                    }
                    *buf.offset(0 as libc::c_int as isize) =
                        0 as libc::c_int as libc::c_char;
                    i = 0 as libc::c_int;
                    while i <= opt_len - addr_len {
                        if i != 0 as libc::c_int {
                            strncat(buf,
                                    b", \x00" as *const u8 as
                                        *const libc::c_char,
                                    (buf_len as
                                         libc::c_ulong).wrapping_sub(strlen(buf)));
                        }
                        /* align */
                        memcpy(&mut addr as *mut all_addr as
                                   *mut libc::c_void,
                               &mut *val.offset(i as isize) as
                                   *mut libc::c_uchar as *const libc::c_void,
                               addr_len as libc::c_ulong);
                        inet_ntop(prot,
                                  &mut *val.offset(i as isize) as
                                      *mut libc::c_uchar as
                                      *const libc::c_void,
                                  (*dnsmasq_daemon).addrbuff,
                                  46 as libc::c_int as socklen_t);
                        strncat(buf, (*dnsmasq_daemon).addrbuff,
                                (buf_len as
                                     libc::c_ulong).wrapping_sub(strlen(buf)));
                        i += addr_len
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              0x1000 as libc::c_int != 0 {
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while i < opt_len && j < buf_len {
                        let mut c: libc::c_char =
                            *val.offset(i as isize) as libc::c_char;
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISprint as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            let fresh6 = j;
                            j = j + 1;
                            *buf.offset(fresh6 as isize) = c
                        }
                        i += 1
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              0x4000 as libc::c_int != 0 &&
                              prot == 10 as libc::c_int {
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while i < opt_len &&
                              *val.offset(i as isize) as libc::c_int !=
                                  0 as libc::c_int {
                        let mut k: libc::c_int = 0;
                        let mut l: libc::c_int =
                            i + *val.offset(i as isize) as libc::c_int +
                                1 as libc::c_int;
                        k = i + 1 as libc::c_int;
                        while k < opt_len && k < l && j < buf_len {
                            let mut c_0: libc::c_char =
                                *val.offset(k as isize) as libc::c_char;
                            if *(*__ctype_b_loc()).offset(c_0 as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISprint as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                let fresh7 = j;
                                j = j + 1;
                                *buf.offset(fresh7 as isize) = c_0
                            }
                            k += 1
                        }
                        i = l;
                        if *val.offset(i as isize) as libc::c_int !=
                               0 as libc::c_int && j < buf_len {
                            let fresh8 = j;
                            j = j + 1;
                            *buf.offset(fresh8 as isize) =
                                '.' as i32 as libc::c_char
                        }
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              0x800 as libc::c_int != 0 {
                    let mut k_0: libc::c_int = 0;
                    let mut len: libc::c_int = 0;
                    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    loop  {
                        p =
                            &mut *val.offset(i as isize) as
                                *mut libc::c_uchar;
                        let mut t_cp: *mut libc::c_uchar = p;
                        len =
                            (*t_cp.offset(0 as libc::c_int as isize) as u16_0
                                 as libc::c_int) << 8 as libc::c_int |
                                *t_cp.offset(1 as libc::c_int as isize) as
                                    u16_0 as libc::c_int;
                        p = p.offset(2 as libc::c_int as isize);
                        k_0 = 0 as libc::c_int;
                        while k_0 < len && j < buf_len {
                            let fresh9 = p;
                            p = p.offset(1);
                            let mut c_1: libc::c_char =
                                *fresh9 as libc::c_char;
                            if *(*__ctype_b_loc()).offset(c_1 as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISprint as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                let fresh10 = j;
                                j = j + 1;
                                *buf.offset(fresh10 as isize) = c_1
                            }
                            k_0 += 1
                        }
                        i += len + 2 as libc::c_int;
                        if i >= opt_len { break ; }
                        if j < buf_len {
                            let fresh11 = j;
                            j = j + 1;
                            *buf.offset(fresh11 as isize) =
                                ',' as i32 as libc::c_char
                        }
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              (0x400 as libc::c_int | 0x200 as libc::c_int) !=
                              0 && opt_len != 0 as libc::c_int {
                    let mut dec: libc::c_uint =
                        0 as libc::c_int as libc::c_uint;
                    i = 0 as libc::c_int;
                    while i < opt_len {
                        dec =
                            dec << 8 as libc::c_int |
                                *val.offset(i as isize) as libc::c_uint;
                        i += 1
                    }
                    if (*ot.offset(o as isize)).size as libc::c_int &
                           0x200 as libc::c_int != 0 {
                        prettyprint_time(buf, dec);
                    } else {
                        sprintf(buf,
                                b"%u\x00" as *const u8 as *const libc::c_char,
                                dec);
                    }
                } else { nodecode = 1 as libc::c_int }
            }
            break ;
        } else { o += 1 }
    }
    if opt_len != 0 as libc::c_int && !buf.is_null() &&
           ((*ot.offset(o as isize)).name.is_null() || nodecode != 0) {
        let mut trunc: libc::c_int = 0 as libc::c_int;
        if opt_len > 14 as libc::c_int {
            trunc = 1 as libc::c_int;
            opt_len = 14 as libc::c_int
        }
        print_mac(buf, val, opt_len);
        if trunc != 0 {
            strncat(buf, b"...\x00" as *const u8 as *const libc::c_char,
                    (buf_len as libc::c_ulong).wrapping_sub(strlen(buf)));
        }
    }
    return if !(*ot.offset(o as isize)).name.is_null() {
               (*ot.offset(o as isize)).name as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char } as
               *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn log_context(mut family: libc::c_int,
                                     mut context: *mut dhcp_context) {
    /* We don't handle compressed rfc1035 names, so no good in IPv4 land */
    /* Cannot use dhcp_buff* for RA contexts */
    let mut start: *mut libc::c_void =
        &mut (*context).start as *mut in_addr as *mut libc::c_void;
    let mut end: *mut libc::c_void =
        &mut (*context).end as *mut in_addr as *mut libc::c_void;
    let mut template: *mut libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut p: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    *p = 0 as libc::c_int as libc::c_char;
    if family == 10 as libc::c_int {
        let mut subnet: in6_addr = (*context).start6;
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 10 as libc::c_int == 0 {
            setaddr6part(&mut subnet, 0 as libc::c_int as u64_0);
        }
        inet_ntop(10 as libc::c_int,
                  &mut subnet as *mut in6_addr as *const libc::c_void,
                  (*dnsmasq_daemon).addrbuff, 46 as libc::c_int as socklen_t);
        start = &mut (*context).start6 as *mut in6_addr as *mut libc::c_void;
        end = &mut (*context).end6 as *mut in6_addr as *mut libc::c_void
    }
    if family != 2 as libc::c_int &&
           (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        strcpy((*dnsmasq_daemon).namebuff,
               b", prefix deprecated\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        p =
            p.offset(sprintf(p,
                             b", lease time \x00" as *const u8 as
                                 *const libc::c_char) as isize);
        prettyprint_time(p, (*context).lease_time);
        p = p.offset(strlen(p) as isize)
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 11 as libc::c_int != 0 {
        let mut ifrn_name: [libc::c_char; 16] = [0; 16];
        template = p;
        p =
            p.offset(sprintf(p, b", \x00" as *const u8 as *const libc::c_char)
                         as isize);
        if indextoname((*dnsmasq_daemon).icmp6fd, (*context).if_index,
                       ifrn_name.as_mut_ptr()) != 0 {
            sprintf(p, b"%s for %s\x00" as *const u8 as *const libc::c_char,
                    if (*context).flags as libc::c_uint &
                           (1 as libc::c_uint) << 16 as libc::c_int != 0 {
                        b"old prefix\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"constructed\x00" as *const u8 as *const libc::c_char
                    }, ifrn_name.as_mut_ptr());
        }
    } else if (*context).flags as libc::c_uint &
                  (1 as libc::c_uint) << 10 as libc::c_int != 0 &&
                  (*context).flags as libc::c_uint &
                      (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        template = p;
        p =
            p.offset(sprintf(p, b", \x00" as *const u8 as *const libc::c_char)
                         as isize);
        sprintf(p, b"template for %s\x00" as *const u8 as *const libc::c_char,
                (*context).template_interface);
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 16 as libc::c_int == 0 &&
           ((*context).flags as libc::c_uint &
                (1 as libc::c_uint) << 8 as libc::c_int != 0 ||
                family == 2 as libc::c_int) {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 7 as libc::c_int != 0 {
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                strncpy((*dnsmasq_daemon).dhcp_buff,
                        (*context).template_interface,
                        256 as libc::c_int as libc::c_ulong);
            } else {
                strcpy((*dnsmasq_daemon).dhcp_buff,
                       (*dnsmasq_daemon).addrbuff);
            }
        } else {
            inet_ntop(family, start, (*dnsmasq_daemon).dhcp_buff,
                      256 as libc::c_int as socklen_t);
        }
        inet_ntop(family, end, (*dnsmasq_daemon).dhcp_buff3,
                  256 as libc::c_int as socklen_t);
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  if (*context).flags as libc::c_uint &
                         (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                      b"%s stateless on %s%.0s%.0s%s\x00" as *const u8 as
                          *const libc::c_char
                  } else if (*context).flags as libc::c_uint &
                                (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                      b"%s, static leases only on %.0s%s%s%.0s\x00" as
                          *const u8 as *const libc::c_char
                  } else if (*context).flags as libc::c_uint &
                                (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                      b"%s, proxy on subnet %.0s%s%.0s%.0s\x00" as *const u8
                          as *const libc::c_char
                  } else {
                      b"%s, IP range %s -- %s%s%.0s\x00" as *const u8 as
                          *const libc::c_char
                  },
                  if family != 2 as libc::c_int {
                      b"DHCPv6\x00" as *const u8 as *const libc::c_char
                  } else { b"DHCP\x00" as *const u8 as *const libc::c_char },
                  (*dnsmasq_daemon).dhcp_buff, (*dnsmasq_daemon).dhcp_buff3,
                  (*dnsmasq_daemon).namebuff, template);
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 10 as libc::c_int != 0 {
        strcpy((*dnsmasq_daemon).addrbuff, (*context).template_interface);
        template =
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
           (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 16 as libc::c_int == 0 {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCPv4-derived IPv6 names on %s%s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  template);
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 13 as libc::c_int != 0 ||
           (*dnsmasq_daemon).options[(37 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (37 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 8 as libc::c_int != 0 &&
               family == 10 as libc::c_int {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"router advertisement on %s%s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  template);
    };
}
#[no_mangle]
pub unsafe extern "C" fn log_relay(mut family: libc::c_int,
                                   mut relay: *mut dhcp_relay) {
    inet_ntop(family,
              &mut (*relay).local as *mut all_addr as *const libc::c_void,
              (*dnsmasq_daemon).addrbuff, 46 as libc::c_int as socklen_t);
    inet_ntop(family,
              &mut (*relay).server as *mut all_addr as *const libc::c_void,
              (*dnsmasq_daemon).namebuff, 46 as libc::c_int as socklen_t);
    if !(*relay).interface.is_null() {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCP relay from %s to %s via %s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  (*dnsmasq_daemon).namebuff, (*relay).interface);
    } else {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCP relay from %s to %s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  (*dnsmasq_daemon).namebuff);
    };
}
