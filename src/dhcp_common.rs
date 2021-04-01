
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
use crate::defines::{DnsmasqDaemon, Irec, Iname, MsgHdr, MSG_PEEK, MSG_TRUNC, DhcpNetId, TagIf, DhcpNetIdList, DhcpOpt, DhcpConfig, HwaddrConfig, DhcpContext, AddrList, Crec,  AllAddr, DhcpOptTblEntry, InAddr, _ISprint, DhcpRelay};
use crate::util::{expand_buf, memcmp_masked, is_same_net6, is_same_net, hostname_isequal,  prettyprint_time, print_mac, setaddr6part};
use libc::recvmsg;
use socket2::Socket;
use std::io;
use crate::dnsmasq_log::my_syslog;
use crate::cache::cache_find_by_name;
use crate::dhcp::config_find_by_address;
use crate::dhcp6::config_find_by_address6;
use crate::network::indextoname;

pub fn dhcp_common_init(daemon: &mut DnsmasqDaemon) {
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


pub fn recv_dhcp_packet(daemon: &mut DnsmasqDaemon, socket: UdpSocket, packet_buf: &mut Vec<u8>) -> Result<usize,  std::io::Error> {
    socket.recv(packet_buf)
}

pub fn run_tag_if(mut tags: &mut DhcpNetId)
 -> *mut DhcpNetId {
    let mut exprs: TagIf;
    let mut list: DhcpNetIdList;
    exprs = daemon.tag_if;

    for exprs in daemon.tag_if {
        if match_netid(exprs.tag, tags, 1) != 0 {
            // list = exprs.set;
            // for item in list {
            //     list.list = tags;
            //     tags = 
            // }
        }
        
    }

    // while !exprs.is_null() {
    //     if match_netid((*exprs).tag, tags, 1 ) != 0 {
    //         list = (*exprs).set;
    //         while !list.is_null() {
    //             (*(*list).list).next = tags;
    //             tags = (*list).list;
    //             list = (*list).next
    //         }
    //     }
    //     exprs = (*exprs).next
    // }
    return tags;
}

pub fn option_filter(mut tags: *mut DhcpNetId,
                                       mut context_tags: *mut DhcpNetId,
                                       mut opts: *mut DhcpOpt)
                                       -> *mut DhcpNetId {
    let mut tagif: *mut DhcpNetId = run_tag_if(tags);
    let mut opt: *mut DhcpOpt = 0 as *mut DhcpOpt;
    let mut tmp: *mut DhcpOpt = 0 as *mut DhcpOpt;
    /* flag options which are valid with the current tag set (sans context tags) */
    opt = opts;
    while !opt.is_null() {
        (*opt).flags &= !(4096 );
        if (*opt).flags &
               (4  | 256  | 2048 )
               == 0 && match_netid((*opt).netid, tagif, 0 ) != 0
           {
            (*opt).flags |= 4096 
        }
        opt = (*opt).next
    }
    /* now flag options which are valid, including the context tags,
     otherwise valid options are inhibited if we found a higher priority one above */
    if !context_tags.is_null() {
        let mut last_tag: *mut DhcpNetId = 0 as *mut DhcpNetId;
        last_tag = context_tags;
        while !(*last_tag).next.is_null() { last_tag = (*last_tag).next }
        (*last_tag).next = tags;
        tagif = run_tag_if(context_tags);
        /* reset stuff with tag:!<tag> which now matches. */
        opt = opts;
        while !opt.is_null() {
            if (*opt).flags &
                   (4  | 256  |
                        2048 ) == 0 &&
                   (*opt).flags & 4096  != 0 &&
                   match_netid((*opt).netid, tagif, 0 ) == 0 {
                (*opt).flags &= !(4096 )
            }
            opt = (*opt).next
        }
        opt = opts;
        while !opt.is_null() {
            if (*opt).flags &
                   (4  | 256  |
                        2048  | 4096 ) == 0 &&
                   match_netid((*opt).netid, tagif, 0 ) != 0 {
                let mut tmp_0: *mut DhcpOpt = 0 as *mut DhcpOpt;
                tmp_0 = opts;
                while !tmp_0.is_null() {
                    if (*tmp_0).opt == (*opt).opt && !(*opt).netid.is_null()
                           && (*tmp_0).flags & 4096  != 0 {
                        break ;
                    }
                    tmp_0 = (*tmp_0).next
                }
                if tmp_0.is_null() { (*opt).flags |= 4096  }
            }
            opt = (*opt).next
        }
    }
    /* now flag untagged options which are not overridden by tagged ones */
    opt = opts;
    while !opt.is_null() {
        if (*opt).flags &
               (4  | 256  | 2048  |
                    4096 ) == 0 && (*opt).netid.is_null() {
            tmp = opts;
            while !tmp.is_null() {
                if (*tmp).opt == (*opt).opt &&
                       (*tmp).flags & 4096  != 0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() {
                (*opt).flags |= 4096 
            } else if (*tmp).netid.is_null() {
                my_syslog((3 ) << 3  |
                              4 ,
                          "Ignoring duplicate dhcp-option %d" as
                              *const u8 , (*tmp).opt);
            }
        }
        opt = (*opt).next
    }
    /* Finally, eliminate duplicate options later in the chain, and therefore earlier in the config file. */
    opt = opts;
    while !opt.is_null() {
        if (*opt).flags & 4096  != 0 {
            tmp = (*opt).next;
            while !tmp.is_null() {
                if (*tmp).opt == (*opt).opt {
                    (*tmp).flags &= !(4096 )
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

pub fn match_netid(mut check: *mut DhcpNetId,
                                     mut pool: *mut DhcpNetId,
                                     mut tagnotneeded: libc::c_int)
                                     -> libc::c_int {
    let mut tmp1: *mut DhcpNetId = 0 as *mut DhcpNetId;
    if check.is_null() && tagnotneeded == 0 { return 0  }
    while !check.is_null() {
        /* '#' for not is for backwards compat. */
        if *(*check).net.offset(0  as isize)  !=
               '!' as i32 &&
               *(*check).net.offset(0  as isize) 
                   != '#' as i32 {
            tmp1 = pool;
            while !tmp1.is_null() {
                if strcmp((*check).net, (*tmp1).net) == 0  {
                    break ;
                }
                tmp1 = (*tmp1).next
            }
            if tmp1.is_null() { return 0  }
        } else {
            tmp1 = pool;
            while !tmp1.is_null() {
                if strcmp((*check).net.offset(1  as isize),
                          (*tmp1).net) == 0  {
                    return 0 
                }
                tmp1 = (*tmp1).next
            }
        }
        check = (*check).next
    }
    return 1 ;
}
/* return domain or NULL if none. */

pub fn strip_hostname(mut hostname: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut dot: *mut libc::c_char =
        strchr(hostname, '.' as i32); /* truncate */
    if dot.is_null() { return 0 as *mut libc::c_char }
    *dot = 0  as libc::c_char;
    if strlen(dot.offset(1  as isize)) !=
           0  as libc::c_ulong {
        return dot.offset(1  as isize)
    }
    return 0 as *mut libc::c_char;
}

pub fn log_tags(mut netid: *mut DhcpNetId,
                                  mut xid: u32) {
    if !netid.is_null() &&
           daemon.options[(28  as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (28  as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        let mut s: *mut libc::c_char = daemon.namebuff;
        *s = 0  as libc::c_char;
        while !netid.is_null() {
            /* kill dupes. */
            let mut n: *mut DhcpNetId = 0 as *mut DhcpNetId;
            n = (*netid).next;
            while !n.is_null() {
                if strcmp((*netid).net, (*n).net) == 0  {
                    break ;
                }
                n = (*n).next
            }
            if n.is_null() {
                strncat(s, (*netid).net,
                        ((1025  - 1 ) as
                             libc::c_ulong).wrapping_sub(strlen(s)));
                if !(*netid).next.is_null() {
                    strncat(s, ", " ,
                            ((1025  - 1 ) as
                                 libc::c_ulong).wrapping_sub(strlen(s)));
                }
            }
            netid = (*netid).next
        }
        my_syslog((3 ) << 3  | 6 ,
                  "%u tags: %s" , xid,
                  s);
    };
}

pub fn match_bytes(mut o: *mut DhcpOpt,
                                     mut p: *mut libc::c_uchar,
                                     mut len: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*o).len > len { return 0  }
    if (*o).len == 0  { return 1  }
    if (*o).flags & 512  != 0 {
        if memcmp_masked((*o).val, p, (*o).len, (*o).u.wildcard_mask) != 0 {
            return 1 
        }
    } else {
        i = 0 ;
        while i <= len - (*o).len {
            if memcmp((*o).val as *const libc::c_void,
                      p.offset(i as isize) as *const libc::c_void,
                      (*o).len as libc::c_ulong) == 0  {
                return 1 
            }
            if (*o).flags & 2  != 0 {
                i += 1
            } else { i += (*o).len }
        }
    }
    return 0 ;
}

pub fn config_has_mac(mut config: *mut DhcpConfig,
                                        mut hwaddr: *mut libc::c_uchar,
                                        mut len: libc::c_int,
                                        mut type_0: libc::c_int)
                                        -> libc::c_int {
    let mut conf_addr: *mut HwaddrConfig = 0 as *mut HwaddrConfig;
    conf_addr = (*config).hwaddr;
    while !conf_addr.is_null() {
        if (*conf_addr).wildcard_mask == 0  as libc::c_uint &&
               (*conf_addr).hwaddr_len == len &&
               ((*conf_addr).hwaddr_type == type_0 ||
                    (*conf_addr).hwaddr_type == 0 ) &&
               memcmp((*conf_addr).hwaddr.as_mut_ptr() as *const libc::c_void,
                      hwaddr as *const libc::c_void, len as libc::c_ulong) ==
                   0  {
            return 1 
        }
        conf_addr = (*conf_addr).next
    }
    return 0 ;
}
unsafe extern "C" fn is_config_in_context(mut context: *mut DhcpContext,
                                          mut config: *mut DhcpConfig)
                                          -> libc::c_int {
    if context.is_null() {
        /* called via find_config() from lease_update_from_configs() */
        return 1 
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 17  != 0 {
        let mut addr_list: *mut AddrList = 0 as *mut AddrList;
        if (*config).flags & 4096  as libc::c_uint == 0 {
            return 1 
        }
        while !context.is_null() {
            addr_list = (*config).addr6;
            while !addr_list.is_null() {
                if (*addr_list).flags & 16  != 0 &&
                       (*context).prefix == 64  {
                    return 1 
                }
                if is_same_net6(&mut (*addr_list).addr.addr6,
                                &mut (*context).start6, (*context).prefix) !=
                       0 {
                    return 1 
                }
                addr_list = (*addr_list).next
            }
            context = (*context).current
        }
    } else {
        if (*config).flags & 32  as libc::c_uint == 0 {
            return 1 
        }
        while !context.is_null() {
            if (*config).flags & 32  as libc::c_uint != 0 &&
                   is_same_net((*config).addr, (*context).start,
                               (*context).netmask) != 0 {
                return 1 
            }
            context = (*context).current
        }
    }
    return 0 ;
}
unsafe extern "C" fn find_config_match(mut configs: *mut DhcpConfig,
                                       mut context: *mut DhcpContext,
                                       mut clid: *mut libc::c_uchar,
                                       mut clid_len: libc::c_int,
                                       mut hwaddr: *mut libc::c_uchar,
                                       mut hw_len: libc::c_int,
                                       mut hw_type: libc::c_int,
                                       mut hostname: *mut libc::c_char,
                                       mut tags: *mut DhcpNetId,
                                       mut tag_not_needed: libc::c_int)
                                       -> *mut DhcpConfig {
    let mut count: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    let mut config: *mut DhcpConfig = 0 as *mut DhcpConfig;
    let mut candidate: *mut DhcpConfig = 0 as *mut DhcpConfig;
    let mut conf_addr: *mut HwaddrConfig = 0 as *mut HwaddrConfig;
    if !clid.is_null() {
        config = configs;
        while !config.is_null() {
            if (*config).flags & 2  as libc::c_uint != 0 {
                if (*config).clid_len == clid_len &&
                       memcmp((*config).clid as *const libc::c_void,
                              clid as *const libc::c_void,
                              clid_len as libc::c_ulong) == 0 
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
                            (1 as libc::c_uint) << 17  == 0) &&
                       *clid  == 0  &&
                       (*config).clid_len == clid_len - 1  &&
                       memcmp((*config).clid as *const libc::c_void,
                              clid.offset(1  as isize) as
                                  *const libc::c_void,
                              (clid_len - 1 ) as libc::c_ulong)
                           == 0  &&
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
            if (*config).flags & 16  as libc::c_uint != 0 &&
                   hostname_isequal((*config).hostname, hostname) != 0 &&
                   is_config_in_context(context, config) != 0 &&
                   match_netid((*config).filter, tags, tag_not_needed) != 0 {
                return config
            }
            config = (*config).next
        }
    }
    if hwaddr.is_null() { return 0 as *mut DhcpConfig }
    /* use match with fewest wildcard octets */
    candidate = 0 as *mut DhcpConfig;
    count = 0 ;
    config = configs;
    while !config.is_null() {
        if is_config_in_context(context, config) != 0 &&
               match_netid((*config).filter, tags, tag_not_needed) != 0 {
            conf_addr = (*config).hwaddr;
            while !conf_addr.is_null() {
                if (*conf_addr).wildcard_mask !=
                       0  as libc::c_uint &&
                       (*conf_addr).hwaddr_len == hw_len &&
                       ((*conf_addr).hwaddr_type == hw_type ||
                            (*conf_addr).hwaddr_type == 0 ) &&
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

pub fn find_config(mut configs: *mut DhcpConfig,
                                     mut context: *mut DhcpContext,
                                     mut clid: *mut libc::c_uchar,
                                     mut clid_len: libc::c_int,
                                     mut hwaddr: *mut libc::c_uchar,
                                     mut hw_len: libc::c_int,
                                     mut hw_type: libc::c_int,
                                     mut hostname: *mut libc::c_char,
                                     mut tags: *mut DhcpNetId)
                                     -> *mut DhcpConfig {
    let mut ret: *mut DhcpConfig =
        find_config_match(configs, context, clid, clid_len, hwaddr, hw_len,
                          hw_type, hostname, tags, 0 );
    if ret.is_null() {
        ret =
            find_config_match(configs, context, clid, clid_len, hwaddr,
                              hw_len, hw_type, hostname, tags,
                              1 )
    }
    return ret;
}

pub fn dhcp_update_configs(mut configs: *mut DhcpConfig) {
    /* Some people like to keep all static IP addresses in /etc/hosts.
     This goes through /etc/hosts and sets static addresses for any DHCP config
     records which don't have an address and whose name matches. 
     We take care to maintain the invariant that any IP address can appear
     in at most one dhcp-host. Since /etc/hosts can be re-read by SIGHUP, 
     restore the status-quo ante first. */
    let mut config: *mut DhcpConfig = 0 as *mut DhcpConfig;
    let mut conf_tmp: *mut DhcpConfig = 0 as *mut DhcpConfig;
    let mut crec: *mut Crec = 0 as *mut Crec;
    let mut prot: libc::c_int = 2 ;
    config = configs;
    while !config.is_null() {
        if (*config).flags & 512  as libc::c_uint != 0 {
            (*config).flags &=
                !(32  | 512 ) as libc::c_uint
        }
        if (*config).flags & 16384  as libc::c_uint != 0 {
            (*config).flags &=
                !(4096  | 16384 ) as libc::c_uint
        }
        config = (*config).next
    }
    loop  {
        if daemon.port != 0  {
            let mut current_block_27: u64;
            config = configs;
            while !config.is_null() {
                let mut conflags: libc::c_int = 32 ;
                let mut cacheflags: libc::c_int =
                    ((1 as libc::c_uint) << 7 ) ;
                if prot == 10  {
                    conflags = 4096 ;
                    cacheflags =
                        ((1 as libc::c_uint) << 8 ) as
                            libc::c_int
                }
                if (*config).flags & conflags as libc::c_uint == 0 &&
                       (*config).flags & 16  as libc::c_uint !=
                           0 &&
                       {
                           crec =
                               cache_find_by_name(0 as *mut Crec,
                                                  (*config).hostname,
                                                  0  as time_t,
                                                  cacheflags as libc::c_uint);
                           !crec.is_null()
                       } &&
                       (*crec).flags & (1 as libc::c_uint) << 6 
                           != 0 {
                    if !cache_find_by_name(crec, (*config).hostname,
                                           0  as time_t,
                                           cacheflags as
                                               libc::c_uint).is_null() {
                        /* use primary (first) address */
                        while !crec.is_null() &&
                                  (*crec).flags &
                                      (1 as libc::c_uint) << 2 
                                      == 0 {
                            crec =
                                cache_find_by_name(crec, (*config).hostname,
                                                   0  as time_t,
                                                   cacheflags as libc::c_uint)
                        } /* should be never */
                        if crec.is_null() {
                            current_block_27 = 3640593987805443782;
                        } else {
                            inet_ntop(prot,
                                      &mut (*crec).addr as *mut AllAddr as
                                          *const libc::c_void,
                                      daemon.addrbuff,
                                      46  as socklen_t);
                            my_syslog((3 ) << 3  |
                                          4 ,
                                      "%s has more than one address in hostsfile, using %s for DHCP"
                                          ,
                                      (*config).hostname,
                                      daemon.addrbuff);
                            current_block_27 = 1109700713171191020;
                        }
                    } else { current_block_27 = 1109700713171191020; }
                    match current_block_27 {
                        3640593987805443782 => { }
                        _ => {
                            if prot == 2  &&
                                   {
                                       conf_tmp =
                                           config_find_by_address(configs,
                                                                  (*crec).addr.addr4);
                                       (conf_tmp.is_null()) ||
                                           conf_tmp == config
                                   } {
                                (*config).addr = (*crec).addr.addr4;
                                (*config).flags |=
                                    (32  | 512 )
                                        as libc::c_uint
                            } else if prot == 10  &&
                                          {
                                              conf_tmp =
                                                  config_find_by_address6(configs,
                                                                          0 as
                                                                              *mut In6Addr,
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
                                               whine_m
                                           alloc(::std::mem::size_of::<AddrList>()
                                                                as
                                                                libc::c_ulong)
                                                   as *mut AddrList;
                                           !(*config).addr6.is_null()
                                       } {
                                    (*(*config).addr6).next =
                                        0 as *mut AddrList;
                                    (*(*config).addr6).flags =
                                        0 
                                }
                                if !(*config).addr6.is_null() &&
                                       (*(*config).addr6).next.is_null() &&
                                       (*(*config).addr6).flags &
                                           (16  |
                                                8 ) == 0 {
                                    memcpy(&mut (*(*config).addr6).addr.addr6
                                               as *mut In6Addr as
                                               *mut libc::c_void,
                                           &mut (*crec).addr.addr6 as
                                               *mut In6Addr as
                                               *const libc::c_void,
                                           16  as
                                               libc::c_ulong);
                                    (*config).flags |=
                                        (4096  |
                                             16384 ) as
                                            libc::c_uint
                                }
                            } else {
                                inet_ntop(prot,
                                          &mut (*crec).addr as *mut AllAddr
                                              as *const libc::c_void,
                                          daemon.addrbuff,
                                          46  as socklen_t);
                                my_syslog((3 ) <<
                                              3  |
                                              4 ,
                                          "duplicate IP address %s (%s) in dhcp-config directive"
                                              as
                                              *const libc::c_char,
                                          daemon.addrbuff,
                                          (*config).hostname);
                            }
                        }
                    }
                }
                config = (*config).next
            }
        }
        if !(prot == 2 ) { break ; }
        prot = 10 
    };
}

pub fn whichdevice(daemon: &mut DnsmasqDaemon) -> Option<String> {
    /* If we are doing DHCP on exactly one interface, and running linux, do SO_BINDTODEVICE
     to that device. This is for the use case of  (eg) OpenStack, which runs a new
     dnsmasq instance for each VLAN interface it creates. Without the BINDTODEVICE, 
     individual processes don't always see the packets they should.
     SO_BINDTODEVICE is only available Linux. 

     Note that if wildcards are used in --interface, or --interface is not used at all,
     or a configured interface doesn't yet exist, then more interfaces may arrive later, 
     so we can't safely assert there is only one interface and proceed.
*/
    let mut iface: Irec = Default::default();
    let mut found: Irec = Default::default();
    let mut if_tmp: Iname = Default::default();
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
            } else if strcmp((*found).name, iface.name) != 0 
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

static DHCPV4_OPT_TBL: Vec<DhcpOptTblEntry> = vec!
    [DhcpOptTblEntry {name: String::from("netmask"), val: 1  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("time-offset"), val: 2  , size: 4  ,},
     DhcpOptTblEntry {name: String::from("router"), val: 3  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("dns-server"), val: 6  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("log-server"), val: 7  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("lpr-server"), val: 9  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("hostname"), val: 12  , size: (0x2000  | 0x1000 ),},
     DhcpOptTblEntry {name: String::from("boot-file-size"), val: 13  , size: (2  | 0x400 ) as u16,},
     DhcpOptTblEntry {name: String::from("domain-name"), val: 15  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("swap-server"), val: 16  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("root-path"), val: 17  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("extension-path"), val: 18  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("ip-forward-enable"), val: 19  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("non-local-source-routing"), val: 20  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("policy-filter"), val: 21  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("max-datagram-reassembly"), val: 22  , size: (2  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("default-ttl"), val: 23  , size: (1  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("mtu"), val: 26  , size: (2  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("all-subnets-local"), val: 27  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("broadcast"), val: 28  , size: (0x2000  | 0x8000 ) ,},
     DhcpOptTblEntry {name: String::from("router-discovery"), val: 31  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("router-solicitation"), val: 32  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("static-route"), val: 33  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("trailer-encapsulation"), val: 34  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("arp-timeout"), val: 35  , size: (4  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("ethernet-encap"), val: 36  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("tcp-ttl"), val: 37  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("tcp-keepalive"), val: 38  , size: (4  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("nis-domain"), val: 40  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("nis-server"), val: 41  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("ntp-server"), val: 42  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("vendor-encap"), val: 43  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("netbios-ns"), val: 44  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("netbios-dd"), val: 45  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("netbios-nodetype"), val: 46  , size: 1  ,},
     DhcpOptTblEntry {name: String::from("netbios-scope"), val: 47  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("x-windows-fs"), val: 48  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("x-windows-dm"), val: 49  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("requested-address"), val: 50  , size: (0x2000  | 0x8000 ) ,},
     DhcpOptTblEntry {name: String::from("lease-time"), val: 51  , size: (0x2000  | 0x200 ) ,},
     DhcpOptTblEntry {name: String::from("option-overload"), val: 52  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("message-type"), val: 53  , size: (0x2000  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("server-identifier"), val: 54  , size: (0x2000  | 0x8000 ) ,},
     DhcpOptTblEntry {name: String::from("parameter-request"), val: 55  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("message"), val: 56  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("max-message-size"), val: 57  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("T1"), val: 58  , size: 0x200  ,},
     DhcpOptTblEntry {name: String::from("T2"), val: 59  , size: 0x200  ,},
     DhcpOptTblEntry {name: String::from("vendor-class"), val: 60  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("client-id"), val: 61  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("nis+-domain"), val: 64  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("nis+-server"), val: 65  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("tftp-server"), val: 66  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("bootfile-name"), val: 67  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("mobile-ip-home"), val: 68  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("smtp-server"), val: 69  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("pop3-server"), val: 70  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("nntp-server"), val: 71  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("irc-server"), val: 74  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("user-class"), val: 77  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("rapid-commit"), val: 80  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("FQDN"), val: 81  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("agent-id"), val: 82  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("client-arch"), val: 93  , size: (2  | 0x400 ) ,},
     DhcpOptTblEntry {name: String::from("client-interface-id"), val: 94  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("client-machine-id"), val: 97  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("subnet-select"), val: 118  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("domain-search"), val: 119  , size: 0x4000  ,},
     DhcpOptTblEntry {name: String::from("sip-server"), val: 120  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("classless-static-route"), val: 121  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("vendor-id-encap"), val: 125  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("tftp-server-address"), val: 150  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("server-ip-address"), val: 255  , size: 0x8000  ,},];



static DHCPV6_OPT_TBL: Vec<DhcpOptTblEntry> = vec![
    DhcpOptTblEntry {name: String::from("client-id"), val: 1, size: 0x2000,},
    DhcpOptTblEntry {name: String::from("server-id"), val: 2, size: 0x2000,},
    DhcpOptTblEntry {name: String::from("ia-na"), val: 3, size: 0x2000 ,},
    DhcpOptTblEntry {name: String::from("ia-ta"), val: 4  , size: 0x2000  ,},
    DhcpOptTblEntry {name: String::from("iaaddr"), val: 5  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("oro"), val: 6  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("preference"), val: 7  , size: (0x2000  | 0x400 ),},
     DhcpOptTblEntry {name: String::from("unicast"), val: 12  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("status"), val: 13  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("rapid-commit"), val: 14  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("user-class"), val: 15  , size: (0x2000  | 0x800 ),},
     DhcpOptTblEntry {name: String::from("vendor-class"), val: 16  , size: (0x2000  | 0x800 ),},
     DhcpOptTblEntry {name: String::from("vendor-opts"), val: 17  , size: 0x2000  ,},
     DhcpOptTblEntry {name: String::from("sip-server-domain"), val: 21  , size: 0x4000  ,},
     DhcpOptTblEntry {name: String::from("sip-server"), val: 22  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("dns-server"), val: 23  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("domain-search"), val: 24  , size: 0x4000  ,},
     DhcpOptTblEntry {name: String::from("nis-server"), val: 27  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("nis+-server"), val: 28  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("nis-domain"), val: 29  , size: 0x4000  ,},
     DhcpOptTblEntry {name: String::from("nis+-domain"), val: 30  , size: 0x4000  ,},
     DhcpOptTblEntry {name: String::from("sntp-server"), val: 31  , size: 0x8000  ,},
     DhcpOptTblEntry {name: String::from("information-refresh-time"), val: 32  , size: 0x200  ,},
     DhcpOptTblEntry {name: String::from("FQDN"), val: 39  , size: (0x2000  | 0x4000 ),},
     DhcpOptTblEntry {name: String::from("ntp-server"), val: 56  , size: 0  ,},
     DhcpOptTblEntry {name: String::from("bootfile-url"), val: 59  , size: 0x1000  ,},
     DhcpOptTblEntry {name: String::from("bootfile-param"), val: 60  , size: 0x800  ,},];

pub fn display_opts() {
    let mut i: libc::c_int = 0;
    printf("Known DHCP options:\n" );
    i = 0 ;
    while !DHCPV4_OPT_TBL[i as usize].name.is_null() {
        if DHCPV4_OPT_TBL[i as usize].size  & 0x2000  == 0
           {
            printf("%3d %s\n" ,
                   DHCPV4_OPT_TBL[i as usize].val,
                   DHCPV4_OPT_TBL[i as usize].name);
        }
        i += 1
    };
}

pub fn display_opts6() {
    let mut i: libc::c_int = 0;
    printf("Known DHCPv6 options:\n" as
               *const libc::c_char);
    i = 0 ;
    while !DHCPV6_OPT_TBL[i as usize].name.is_null() {
        if DHCPV6_OPT_TBL[i as usize].size  & 0x2000  ==
               0 {
            printf("%3d %s\n" ,
                   DHCPV6_OPT_TBL[i as usize].val,
                   DHCPV6_OPT_TBL[i as usize].name);
        }
        i += 1
    };
}

pub fn lookup_dhcp_opt(mut protocol: u32, mut name: &String)
                       -> Option<u16> {
    // let mut t: OptTab = Default::default();
    // let mut i: u32 = 0;

    if protocol == 10 {
        for entry in DHCPV6_OPT_TBL {
            if entry.name == name {
                return Some(entry.val)
            }
        }
    } else {
        for entry in DHCPV4_OPT_TBL {
            if entry.name == name {
                return Some(entry.val)
            }
        }
    }
    None
}

pub fn lookup_dhcp_len(mut protocol: u32, mut val: u16) -> Option<usize> {
    if protocol == 10 {
        for entry in DHCPV6_OPT_TBL.into_iter() {
            if entry.val == val {
                return Some(entry.size)
            }
        }
    } else {
        for entry in DHCPV4_OPT_TBL.into_iter() {
            if entry.val == val {
                return Some(entry.size)
            }
        }
    }
    return None
}

pub fn option_string(mut prot: libc::c_int,
                                       mut opt: libc::c_uint,
                                       mut val: *mut libc::c_uchar,
                                       mut opt_len: libc::c_int,
                                       mut buf: *mut libc::c_char,
                                       mut buf_len: libc::c_int)
 -> *mut libc::c_char {
    let mut o: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nodecode: libc::c_int = 0 ;
    let mut ot: *const DhcpOptTblEntry = DHCPV4_OPT_TBL.as_ptr();
    if prot == 10  { ot = DHCPV6_OPT_TBL.as_ptr() }
    o = 0 ;
    while !(*ot.offset(o as isize)).name.is_null() {
        if (*ot.offset(o as isize)).val as libc::c_uint == opt {
            if !buf.is_null() {
                memset(buf as *mut libc::c_void, 0 ,
                       buf_len as libc::c_ulong);
                if (*ot.offset(o as isize)).size  &
                       0x8000  != 0 {
                    let mut addr: AllAddr =
                        AllAddr {addr4: InAddr {s_addr: 0,},};
                    let mut addr_len: libc::c_int = 4 ;
                    if prot == 10  {
                        addr_len = 16 
                    }
                    *buf.offset(0  as isize) =
                        0  as libc::c_char;
                    i = 0 ;
                    while i <= opt_len - addr_len {
                        if i != 0  {
                            strncat(buf,
                                    ", " as
                                        *const libc::c_char,
                                    (buf_len as
                                         libc::c_ulong).wrapping_sub(strlen(buf)));
                        }
                        /* align */
                        memcpy(&mut addr as *mut AllAddr as
                                   *mut libc::c_void,
                               &mut *val.offset(i as isize) as
                                   *mut libc::c_uchar as *const libc::c_void,
                               addr_len as libc::c_ulong);
                        inet_ntop(prot,
                                  &mut *val.offset(i as isize) as
                                      *mut libc::c_uchar as
                                      *const libc::c_void,
                                  daemon.addrbuff,
                                  46  as socklen_t);
                        strncat(buf, daemon.addrbuff,
                                (buf_len as
                                     libc::c_ulong).wrapping_sub(strlen(buf)));
                        i += addr_len
                    }
                } else if (*ot.offset(o as isize)).size  &
                              0x1000  != 0 {
                    i = 0 ;
                    j = 0 ;
                    while i < opt_len && j < buf_len {
                        let mut c: libc::c_char =
                            *val.offset(i as isize) as libc::c_char;
                        if *(*__ctype_b_loc()).offset(c  as
                                                          isize) as
                               libc::c_int &
                               _ISprint  as libc::c_ushort as
                                   libc::c_int != 0 {
                            let fresh6 = j;
                            j = j + 1;
                            *buf.offset(fresh6 as isize) = c
                        }
                        i += 1
                    }
                } else if (*ot.offset(o as isize)).size  &
                              0x4000  != 0 &&
                              prot == 10  {
                    i = 0 ;
                    j = 0 ;
                    while i < opt_len &&
                              *val.offset(i as isize)  !=
                                  0  {
                        let mut k: libc::c_int = 0;
                        let mut l: libc::c_int =
                            i + *val.offset(i as isize)  +
                                1 ;
                        k = i + 1 ;
                        while k < opt_len && k < l && j < buf_len {
                            let mut c_0: libc::c_char =
                                *val.offset(k as isize) as libc::c_char;
                            if *(*__ctype_b_loc()).offset(c_0 
                                                              as isize) as
                                   libc::c_int &
                                   _ISprint  as libc::c_ushort
                                        != 0 {
                                let fresh7 = j;
                                j = j + 1;
                                *buf.offset(fresh7 as isize) = c_0
                            }
                            k += 1
                        }
                        i = l;
                        if *val.offset(i as isize)  !=
                               0  && j < buf_len {
                            let fresh8 = j;
                            j = j + 1;
                            *buf.offset(fresh8 as isize) =
                                '.' as i32 as libc::c_char
                        }
                    }
                } else if (*ot.offset(o as isize)).size  &
                              0x800  != 0 {
                    let mut k_0: libc::c_int = 0;
                    let mut len: libc::c_int = 0;
                    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    i = 0 ;
                    j = 0 ;
                    loop  {
                        p =
                            &mut *val.offset(i as isize) as
                                *mut libc::c_uchar;
                        let mut t_cp: *mut libc::c_uchar = p;
                        len =
                            (*t_cp.offset(0  as isize) 
                                 ) << 8  |
                                *t_cp.offset(1  as isize) as
                                    u16 ;
                        p = p.offset(2  as isize);
                        k_0 = 0 ;
                        while k_0 < len && j < buf_len {
                            let fresh9 = p;
                            p = p.offset(1);
                            let mut c_1: libc::c_char =
                                *fresh9 as libc::c_char;
                            if *(*__ctype_b_loc()).offset(c_1 
                                                              as isize) as
                                   libc::c_int &
                                   _ISprint  as libc::c_ushort
                                        != 0 {
                                let fresh10 = j;
                                j = j + 1;
                                *buf.offset(fresh10 as isize) = c_1
                            }
                            k_0 += 1
                        }
                        i += len + 2 ;
                        if i >= opt_len { break ; }
                        if j < buf_len {
                            let fresh11 = j;
                            j = j + 1;
                            *buf.offset(fresh11 as isize) =
                                ',' as i32 as libc::c_char
                        }
                    }
                } else if (*ot.offset(o as isize)).size  &
                              (0x400  | 0x200 ) !=
                              0 && opt_len != 0  {
                    let mut dec: libc::c_uint =
                        0  as libc::c_uint;
                    i = 0 ;
                    while i < opt_len {
                        dec =
                            dec << 8  |
                                *val.offset(i as isize) as libc::c_uint;
                        i += 1
                    }
                    if (*ot.offset(o as isize)).size  &
                           0x200  != 0 {
                        prettyprint_time(buf, dec);
                    } else {
                        sprintf(buf,
                                "%u" ,
                                dec);
                    }
                } else { nodecode = 1  }
            }
            break ;
        } else { o += 1 }
    }
    if opt_len != 0  && !buf.is_null() &&
           ((*ot.offset(o as isize)).name.is_null() || nodecode != 0) {
        let mut trunc: libc::c_int = 0 ;
        if opt_len > 14  {
            trunc = 1 ;
            opt_len = 14 
        }
        print_mac(buf, val, opt_len);
        if trunc != 0 {
            strncat(buf, "..." ,
                    (buf_len as libc::c_ulong).wrapping_sub(strlen(buf)));
        }
    }
    return if !(*ot.offset(o as isize)).name.is_null() {
               (*ot.offset(o as isize)).name 
           } else { ""  } as
               *mut libc::c_char;
}

pub fn dhcp_context_to_string(mut family: u32, mut context: &DhcpContext) -> String {
    /* We don't handle compressed rfc1035 names, so no good in IPv4 land */
    /* Cannot use dhcp_buff* for RA contexts */
    format!("{}", context).to_string()
}

pub fn log_relay(mut family: u32, mut relay: &DhcpRelay) -> String {
    format!("{}", relay).to_string()
}
