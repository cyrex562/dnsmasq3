
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
use std::time;

use crate::blockdata::{blockdata_free, blockdata_read, blockdata_report, blockdata_retrieve, blockdata_write};
use crate::defines::{_ISPRINT, _ISSPACE, BigName, C2rustUnnamed10, C2rustUnnamed8, Cname, Crec, DnsmasqDaemon, FILE, HostRecord, HostsFile, InterfaceName, MxSrvRecord, NameListEntry, NaPtr, NetAddress, PtrRecord, Server, socklen_t, TxtRecord, TYPESTR};
use crate::domain::{get_domain, get_domain6};
use crate::inotify::set_dynamic_inotify;
use crate::option::expand_filelist;
use crate::util::{canonicalise, hostname_isequal, NetAddress_isequal, prettyprint_addr};

pub fn next_uid(mut crecp: &mut Crec, uid: &mut u32) {
    if crecp.uid == 0 {
        *uid = *uid.wrapping_add(1);
        /* uid == 0 used to indicate CNAME to interface name. */
        if uid == 0 {
            *uid = *uid.wrapping_add(1)
        }
        crecp.uid = *uid
    };
}

pub fn cache_init() {
    let mut crecp: Crec = Default::default();
    let mut i: i32 = 0;
    bignames_left = daemon.cachesize / 10;
    if daemon.cachesize > 0 {
        i = 0;
        while i < daemon.cachesize {
            cache_link(crecp);
            crecp.flags = 0;
            crecp.uid = 0;
            i += 1;
            crecp = crecp.offset(1)
        }
    }
    /* create initial hash table*/
    rehash(daemon.cachesize);
}

/* In most cases, we create the hash table once here by calling this with (hash_table == NULL)
   but if the hosts file(s) are big (some people have 50000 ad-block entries), the table
   will be much too small, so the hosts reading code calls rehash every 1000 addresses, to
   expand the table. */
fn rehash(mut size: i32) {
    let mut new: Crec = Default::default();
    let mut old: Crec = Default::default();
    let mut p: Crec = Default::default();
    let mut tmp: Crec = Default::default();
    let mut i: i32 = 0;
    let mut new_size: i32 = 0;
    let mut old_size: i32 = 0;
    /* hash_size is a power of two. */
    new_size = 64;
    while new_size < size / 10 {
        new_size = new_size << 1
    }
    /* must succeed in getting first instance, failure later is non-fatal */
    if hash_table.is_null() {
    } else if new_size <= hash_size ||
                  {
                      false
                  } {
        return
    }

    i = 0;
    while i < new_size {
        let ref mut fresh6 = *new.offset(i);
        *fresh6 = 0 ;
        i += 1
    }

    old = hash_table;
    old_size = hash_size;
    hash_table = new;
    hash_size = new_size;
    if !old.is_null() {
        i = 0;
        while i < old_size {
            p = *old.offset(i);
            while !p.is_null() {
                tmp = p.hash_next;
                cache_hash(p);
                p = tmp
            }
            i += 1
        }
        free(old);
    };
}

fn hash_bucket(mut name: &String)
 -> {
    let mut c: u32 = 0;
    let mut val: u32 = 0o17465;
    let mut mix_tab: *const libc::c_uchar =
        TYPESTR.as_ptr() ;
    loop  {
        let fresh7 = name;
        name = name.offset(1);
        c = *fresh7;
        if !(c != 0) { break ; }
        /* don't use tolower and friends here - they may be messed up by LOCALE */
        if c >= 'A' as i32 && c <= 'Z' as i32
           {
            c = c.wrapping_add(('a' as i32 - 'A' as i32))
        }
        val =
            (val << 7 |
                 val >>
                     32 -
                         7                       libc::c_int).wrapping_add(*mix_tab.offset((val.wrapping_add(c)
                                                                            &
                                                                            0x3f
                                                                                                     libc::c_int
                                                                                                     libc::c_uint)
                                                     )
                                                           ^
                                                           c)
    }
    /* hash_size is a power of two */
    return hash_table.offset(((val ^ val >> 16) &
                                  (hash_size - 1)                                libc::c_uint));
}
fn cache_hash(mut crecp: Crec) {
    /* maintain an invariant that all entries with F_REVERSE set
     are at the start of the hash-chain  and all non-reverse
     immortal entries are at the end of the hash-chain.
     This allows reverse searches and garbage collection to be optimised */
    let mut up: Crec =
        hash_bucket(cache_get_name(crecp)); /* invalidate CNAMES pointing to this. */
    if crecp.flags & (1) << 2 == 0 {
        while !up.is_null() &&
                  (**up).flags & (1) << 2 != 0
              {
            up = &mut (**up).hash_next
        }
        if crecp.flags & (1) << 0 != 0 {
            while !up.is_null() &&
                      (**up).flags & (1) << 0
                          == 0 {
                up = &mut (**up).hash_next
            }
        }
    }
    crecp.hash_next = *up;
    *up = crecp;
}
fn cache_blockdata_free(mut crecp: Crec) {
    if crecp.flags & (1) << 5 == 0 {
        if crecp.flags & (1) << 30 != 0 {
            blockdata_free(crecp.addr.srv.target);
        }
    };
}
fn cache_free(mut crecp: Crec) {
    crecp.flags &= !((1) << 3);
    crecp.flags &= !((1) << 2);
    crecp.uid = 0;
    if !cache_tail.is_null() {
        cache_tail.next = crecp
    } else { cache_head = crecp }
    crecp.prev = cache_tail;
    crecp.next = 0 ;
    cache_tail = crecp;
    /* retrieve big name for further use. */
    if crecp.flags & (1) << 9 != 0 {
        (*crecp.name.bname).next = big_free;
        big_free = crecp.name.bname;
        crecp.flags &= !((1) << 9)
    }
    cache_blockdata_free(crecp);
}
/* insert a new cache entry at the head of the list (youngest entry) */
fn cache_link(mut crecp: Crec) {
    if !cache_head.is_null() {
        /* check needed for init code */
        cache_head.prev = crecp
    }
    crecp.next = cache_head;
    crecp.prev = 0 ;
    cache_head = crecp;
    if cache_tail.is_null() { cache_tail = crecp };
}
/* remove an arbitrary cache entry for promotion */
fn cache_unlink(mut crecp: Crec) {
    if !crecp.prev.is_null() {
        (*crecp.prev).next = crecp.next
    } else { cache_head = crecp.next }
    if !crecp.next.is_null() {
        (*crecp.next).prev = crecp.prev
    } else { cache_tail = crecp.prev };
}

pub fn cache_get_name(mut crecp: Crec)
 -> String {
    if crecp.flags & (1) << 9 != 0 {
        return (*crecp.name.bname).name.as_mut_ptr()
    } else {
        if crecp.flags & (1) << 1 != 0 {
            return crecp.name.namep
        }
    }
    return crecp.name.sname.as_mut_ptr();
}

pub fn cache_get_cname_target(mut crecp: Crec)
 -> String {
    if crecp.addr.cname.is_name_ptr != 0 {
        return crecp.addr.cname.target.name
    } else { return cache_get_name(crecp.addr.cname.target.cache) };
}

pub fn cache_enumerate(mut init: i32) -> Crec {
    static mut bucket: i32 = 0;
    static mut cache: Crec = 0 ;
    if init != 0 {
        bucket = 0;
        cache = 0
    } else if !cache.is_null() && !cache.hash_next.is_null() {
        cache = cache.hash_next
    } else {
        cache = 0 ;
        while bucket < hash_size {
            let fresh8 = bucket;
            bucket = bucket + 1;
            cache = hash_table.offset(fresh8);
            if !cache.is_null() { break ; }
        }
    }
    return cache;
}
fn is_outdated_cname_pointer(mut crecp: Crec)
 -> i32 {
    if crecp.flags & (1) << 11 == 0 ||
           crecp.addr.cname.is_name_ptr != 0 {
        return 0
    }
    /* NB. record may be reused as DS or DNSKEY, where uid is 
     overloaded for something completely different */
    if !crecp.addr.cname.target.cache.is_null() &&
           (*crecp.addr.cname.target.cache).flags &
               ((1) << 12 |
                    (1) << 14) == 0 &&
           crecp.addr.cname.uid == (*crecp.addr.cname.target.cache).uid
       {
        return 0
    }
    return 1;
}
fn is_expired(mut now: time::Instant, mut crecp: Crec)
 -> i32 {
    if crecp.flags & (1) << 0 != 0 {
        return 0
    }
    if difftime(now, crecp.ttd) < 0  {
        return 0
    }
    return 1;
}
fn cache_scan_free(mut name: &mut String,
                                     mut addr: &mut NetAddress,
                                     mut class: u16,
                                     mut now: time::Instant, mut flags: u32,
                                     mut target_crec: &mut Crec,
                                     mut target_uid: &mut libc::c_uint)
                                     -> Crec {
    /* Scan and remove old entries.
     If (flags & F_FORWARD) then remove any forward entries for name and any expired
     entries but only in the same hash bucket as name.
     If (flags & F_REVERSE) then remove any reverse entries for addr and any expired
     entries in the whole cache.
     If (flags == 0) remove any expired entries in the whole cache. 

     In the flags & F_FORWARD case, the return code is valid, and returns a non-NULL pointer
     to a cache entry if the name exists in the cache as a HOSTS or DHCP entry (these are never deleted)

     We take advantage of the fact that hash chains have stuff in the order <reverse>,<other>,<immortal>
     so that when we hit an entry which isn't reverse and is immortal, we're done. 

     If we free a crec which is a CNAME target, return the entry and uid in target_crec and target_uid.
     This entry will get re-used with the same name, to preserve CNAMEs. */
    let mut crecp: Crec = 0 ;
    let mut up: Crec = 0;
    if flags & (1) << 3 != 0 {
        let mut current_block_18: u64;
        up = hash_bucket(name);
        crecp = *up;
        while !crecp.is_null() {
            if crecp.flags & (1) << 3 != 0
                   && hostname_isequal(cache_get_name(crecp), name) != 0 {
                /* Don't delete DNSSEC in favour of a CNAME, they can co-exist */
                if flags & crecp.flags &
                       ((1) << 7 |
                            (1) << 8 |
                            (1) << 30) != 0 ||
                       (crecp.flags | flags) &
                           (1) << 11 != 0 &&
                           crecp.flags &
                               ((1) << 12 |
                                    (1) << 14)
                               == 0 {
                    if crecp.flags &
                           ((1) << 6 |
                                (1) << 4 |
                                (1) << 13) != 0
                       {
                        return crecp
                    }
                    *up = crecp.hash_next;
                    /* If this record is for the name we're inserting and is the target
		     of a CNAME record. Make the new record for the same name, in the same
		     crec, with the same uid to avoid breaking the existing CNAME. */
                    if crecp.uid != 0 {
                        if !target_crec.is_null() { *target_crec = crecp }
                        if !target_uid.is_null() {
                            *target_uid = crecp.uid
                        }
                    }
                    cache_unlink(crecp);
                    cache_free(crecp);
                    current_block_18 = 12675440807659640239;
                } else { current_block_18 = 4956146061682418353; }
            } else { current_block_18 = 4956146061682418353; }
            match current_block_18 {
                4956146061682418353 => {
                    if is_expired(now, crecp) != 0 ||
                           is_outdated_cname_pointer(crecp) != 0 {
                        *up = crecp.hash_next;
                        if crecp.flags &
                               ((1) << 6 |
                                    (1) << 4 |
                                    (1) << 13)
                               == 0 {
                            cache_unlink(crecp);
                            cache_free(crecp);
                        }
                    } else { up = &mut crecp.hash_next }
                }
                _ => { }
            }
            crecp = crecp.hash_next
        }
    } else {
        let mut i: i32 = 0;
        let mut addrlen: i32 =
            if flags & (1) << 8 != 0 {
                16
            } else { 4 };
        i = 0;
        while i < hash_size {
            crecp = hash_table.offset(i);
            up = &mut hash_table.offset(i);
            while !crecp.is_null() &&
                      (crecp.flags &
                           (1) << 2 != 0 ||
                           crecp.flags &
                               (1) << 0 == 0) {
                if is_expired(now, crecp) != 0 {
                    *up = crecp.hash_next;
                    if crecp.flags &
                           ((1) << 6 |
                                (1) << 4 |
                                (1) << 13) == 0
                       {
                        cache_unlink(crecp);
                        cache_free(crecp);
                    }
                } else if crecp.flags &
                              ((1) << 6 |
                                   (1) << 4 |
                                   (1) << 13)
                              == 0 &&
                              flags & crecp.flags &
                                  (1) << 2 != 0
                              &&
                              flags & crecp.flags &
                                  ((1) << 7 |
                                       (1) <<
                                           8) != 0 &&
                              memcmp(&mut crecp.addr
                                     addr,
                                     addrlen) ==
                                  0 {
                    *up = crecp.hash_next;
                    cache_unlink(crecp);
                    cache_free(crecp);
                } else { up = &mut crecp.hash_next }
                crecp = crecp.hash_next
            }
            i += 1
        }
    }
    return 0 ;
}
/* Note: The normal calling sequence is
   cache_start_insert
   cache_insert * n
   cache_end_insert

   but an abort can cause the cache_end_insert to be missed 
   in which can the next cache_start_insert cleans things up. */

pub fn cache_start_insert() {
    /* Free any entries which didn't get committed during the last
     insert due to error.
  */
    while !new_chain.is_null() {
        let mut tmp: Crec = new_chain.next;
        cache_free(new_chain);
        new_chain = tmp
    }
    new_chain = 0 ;
    insert_error = 0;
}

pub fn cache_insert(mut name: &mut String,
                                      mut addr: &mut NetAddress,
                                      mut class: u16,
                                      mut now: time::Instant, mut ttl: u32,
                                      mut flags: u32) -> Crec {
    /* Don't log DNSSEC records here, done elsewhere */
    // log_query(flags | (1) << 16, name, addr,
    //           0 );
    if daemon.max_cache_ttl != 0 &&
           daemon.max_cache_ttl < ttl {
        ttl = daemon.max_cache_ttl
    }
    if daemon.min_cache_ttl != 0 &&
           daemon.min_cache_ttl > ttl {
        ttl = daemon.min_cache_ttl
    }
    return really_insert(name, addr, class, now, ttl, flags);
}
fn really_insert(mut name: &mut String,
                                   mut addr: &mut NetAddress,
                                   mut class: u16, mut now: time::Instant,
                                   mut ttl: u32,
                                   mut flags: u32) -> Crec {
    let mut new: Crec = 0 ;
    let mut target_crec: Crec = 0 ;
    let mut big_name: BigName = 0;
    let mut freed_all: i32 =
        (flags & (1) << 2);
    let mut free_avail: i32 = 0;
    let mut target_uid: u32 = 0;
    /* if previous insertion failed give up now. */
    if insert_error != 0 { return 0  }
    /* we don't cache zero-TTL records. */
    if ttl == 0 {
        insert_error = 1;
        return 0
    }
    /* First remove any expired entries and entries for the name/address we
     are currently inserting. */
    new =
        cache_scan_free(name, addr, class, now, flags, &mut target_crec,
                        &mut target_uid);
    if !new.is_null() {
        /* We're trying to insert a record over one from 
	 /etc/hosts or DHCP, or other config. If the 
	 existing record is for an A or AAAA or CNAME and
	 the record we're trying to insert is the same, 
	 just drop the insert, but don't error the whole process. */
        if flags &
               ((1) << 7 |
                    (1) << 8) != 0 &&
               flags & (1) << 3 != 0 &&
               !addr.is_null() {
            if flags & (1) << 7 != 0 &&
                   new.flags & (1) << 7 != 0
                   && new.addr.addr4.s_addr == addr.addr4.s_addr {
                return new
            } else {
                if flags & (1) << 8 != 0 &&
                       new.flags & (1) << 8
                           != 0 &&
                       ({
                            let mut __a: *const In6Addr =
                                &mut new.addr.addr6;
                            let mut __b: *const In6Addr =
                                &mut addr.addr6;
                            (__a.__in6_u.__u6_addr32[0  usize] ==
                                 __b.__in6_u.__u6_addr32[0] &&
                                 __a.__in6_u.__u6_addr32[1] ==
                                     __b.__in6_u.__u6_addr32[1]
                                 &&
                                 __a.__in6_u.__u6_addr32[2] ==
                                     __b.__in6_u.__u6_addr32[2]
                                 &&
                                 __a.__in6_u.__u6_addr32[3] ==
                                     __b.__in6_u.__u6_addr32[3])

                        }) != 0 {
                    return new
                }
            }
        }
        insert_error = 1;
        return 0
    }
    /* Now get a cache entry from the end of the LRU list */
    if target_crec.is_null() {
        loop  {
            new = cache_tail;
            if new.is_null() {
                /* no entries left - cache is too small, bail */
                insert_error = 1;
                return 0
            }
            /* Free entry at end of LRU list, use it. */
            if new.flags &
                   ((1) << 3 |
                        (1) << 2) == 0 {
                break ;
            }
            /* End of LRU list is still in use: if we didn't scan all the hash
	 chains for expired entries do that now. If we already tried that
	 then it's time to start spilling things. */
            /* If free_avail set, we believe that an entry has been freed.
	 Bugs have been known to make this not true, resulting in
	 a tight loop here. If that happens, abandon the
	 insert. Once in this state, all inserts will probably fail. */
            if free_avail != 0 {
                static mut warned: i32 = 0;
                if warned == 0 {
                    my_syslog(3,
                              "Internal error in cache.");
                    warned = 1
                }
                insert_error = 1;
                return 0
            }
            if freed_all != 0 {
                /* For DNSSEC records, uid holds class. */
                free_avail = 1; /* Must be free space now. */
                cache_scan_free(cache_get_name(new), &mut new.addr,
                                new.uid , now,
                                new.flags, 0,
                                0);
                daemon.metrics[METRIC_DNS_CACHE_LIVE_FREED] =
                    daemon.metrics[METRIC_DNS_CACHE_LIVE_FREED].wrapping_add(1)
            } else {
                cache_scan_free(0 , 0 ,
                                class, now, 0,
                                0, 0);
                freed_all = 1
            }
        }
    }
    /* Check if we need to and can allocate extra memory for a long name.
     If that fails, give up now, always succeed for DNSSEC records. */
    if !name.is_null() &&
           strlen(name) >
               (50 - 1) {
        if !big_free.is_null() {
            big_name = big_free;
            big_free = big_free.next
        } else if bignames_left == 0 &&
                      flags &
                          ((1) << 14 |
                               (1) << 12) == 0
                      ||
                      {
                          big_name =
                              whine_malloc(::std::mem::size_of::<BigName>());
                          big_name.is_null()
                      } {
            insert_error = 1;
            return 0
        } else { if bignames_left != 0 { bignames_left -= 1 } }
    }
    /* If we freed a cache entry for our name which was a CNAME target, use that.
     and preserve the uid, so that existing CNAMES are not broken. */
    if !target_crec.is_null() { new = target_crec; new.uid = target_uid }
    /* Got the rest: finally grab entry. */
    cache_unlink(new);
    new.flags = flags;
    if !big_name.is_null() {
        new.name.bname = big_name;
        new.flags |= (1) << 9
    }
    if !name.is_null() {
        strcpy(cache_get_name(new), name);
    } else { *cache_get_name(new) = 0 }
    if !addr.is_null() { new.addr = *addr }
    new.ttd = now + ttl;
    new.next = new_chain;
    new_chain = new;
    return new;
}
/* after end of insertion, commit the new entries */

pub fn cache_end_insert() {
    if insert_error != 0 { return }
    while !new_chain.is_null() {
        let mut tmp: Crec = new_chain.next;
        /* drop CNAMEs which didn't find a target. */
        if is_outdated_cname_pointer(new_chain) != 0 {
            cache_free(new_chain);
        } else {
            cache_hash(new_chain);
            cache_link(new_chain);
            daemon.metrics[METRIC_DNS_CACHE_INSERTED
                                          ] =
                daemon.metrics[METRIC_DNS_CACHE_INSERTED                                        libc::c_int                                        usize].wrapping_add(1);
            /* If we're a child process, send this cache entry up the pipe to the master.
	     The marshalling process is rather nasty. */
            if daemon.pipe_to_parent != -(1) {
                let mut name: &mut String = cache_get_name(new_chain);
                let mut m: isize = strlen(name);
                let mut flags: u32 = new_chain.flags;
                read_write(daemon.pipe_to_parent,
                           &mut m ,
                           ::std::mem::size_of::<isize>()
                              , 0);
                read_write(daemon.pipe_to_parent,
                           name, m,
                           0);
                read_write(daemon.pipe_to_parent,
                           &mut new_chain.ttd,
                           ::std::mem::size_of::<time::Instant>(), 0);
                read_write(daemon.pipe_to_parent,
                           &mut flags,
                           ::std::mem::size_of::<libc::c_uint>(),
                           0);
                if flags &
                       ((1) << 7 |
                            (1) << 8 |
                            (1) << 12 |
                            (1) << 14 |
                            (1) << 30) != 0 {
                    read_write(daemon.pipe_to_parent,
                               &mut new_chain.addr,
                               ::std::mem::size_of::<NetAddress>(),
                               0);
                }
                if flags & (1) << 30 != 0 {
                    /* A negative SRV entry is possible and has no data, obviously. */
                    if flags & (1) << 5 == 0 {
                        blockdata_write(new_chain.addr.srv.target,
                                        new_chain.addr.srv.targetlen,
                                        daemon.pipe_to_parent);
                    }
                }
            }
        }
        new_chain = tmp
    }
    /* signal end of cache insert in master process */
    if daemon.pipe_to_parent != -(1) {
        let mut m_0: isize = -(1);
        read_write(daemon.pipe_to_parent,
                   &mut m_0 ,
                   ::std::mem::size_of::<isize>(), 0);
    }
    new_chain = 0 ;
}
/* A marshalled cache entry arrives on fd, read, unmarshall and insert into cache of master process. */

pub fn cache_recv_insert(mut now: time::Instant,
                                           mut fd: i32)
 -> i32 {
    let mut m: isize = 0;
    let mut addr: NetAddress = NetAddress {addr4: NetAddress {s_addr: 0,},};
    let mut ttl: u32 = 0;
    let mut ttd: time::Instant = 0;
    let mut flags: u32 = 0;
    let mut crecp: Crec = 0 ;
    cache_start_insert();
    loop  {
        if read_write(fd, &mut m ,
                      ::std::mem::size_of::<isize>(), 1) == 0 {
            return 0
        }
        if m == -(1) {
            cache_end_insert();
            return 1
        }
        if read_write(fd, daemon.namebuff,
                      m, 1) == 0 ||
               read_write(fd, &mut ttd,
                          ::std::mem::size_of::<time::Instant>(), 1) == 0 ||
               read_write(fd,
                          &mut flags,
                          ::std::mem::size_of::<libc::c_uint>(), 1)
                   == 0 {
            return 0
        }
        *daemon.namebuff.offset(m) =
            0;
        ttl = difftime(ttd, now);
        if flags &
               ((1) << 7 |
                    (1) << 8 |
                    (1) << 12 |
                    (1) << 14 |
                    (1) << 30) != 0 {
            let mut class: u16 =
                1 ;
            if read_write(fd,
                          &mut addr ,
                          ::std::mem::size_of::<NetAddress>()
                             , 1) == 0 {
                return 0
            }
            if flags & (1) << 30 != 0 &&
                   flags & (1) << 5 == 0 &&
                   {
                       addr.srv.target =
                           blockdata_read(fd, addr.srv.targetlen );
                       addr.srv.target.is_null()
                   } {
                return 0
            }
            crecp =
                really_insert(daemon.namebuff, &mut addr, class,
                              now, ttl, flags)
        } else if flags & (1) << 11 != 0 {
            let mut newc: Crec =
                really_insert(daemon.namebuff, 0 ,
                              1 , now, ttl,
                              flags);
            /* This relies on the fact that the target of a CNAME immediately precedes
	     it because of the order of extraction in extract_addresses, and
	     the order reversal on the new_chain. */
            if !newc.is_null() {
                newc.addr.cname.is_name_ptr = 0;
                if crecp.is_null() {
                    newc.addr.cname.target.cache = 0
                } else {
                    next_uid(crecp);
                    newc.addr.cname.target.cache = crecp;
                    newc.addr.cname.uid = crecp.uid
                }
            }
        }
    };
}

pub fn cache_find_non_terminal(mut name: &String, mut now: time::Instant)
 -> i32 {
    let mut crecp: Crec;
    crecp = *hash_bucket(name);
    while !crecp.is_null() {
        if is_outdated_cname_pointer(crecp) == 0 &&
               is_expired(now, crecp) == 0 &&
               crecp.flags & (1) << 3 != 0
               &&
               crecp.flags & (1) << 10 == 0
               && hostname_isequal(name, cache_get_name(crecp)) != 0 {
            return 1
        }
        crecp = crecp.hash_next
    }
    return 0;
}

pub fn cache_find_by_name(mut crecp: Option<Crec>,
                                            mut name: &String,
                                            mut now: time::Instant,
                                            mut prot: u32)
                                            -> Option<Crec> {
    let mut ans: Crec;
    let mut no_rr: u32 = (prot & (1 << 25));
    prot &= !((1) << 25);
    if !crecp.is_null() {
        /* iterating */
        ans = crecp.next
    } else {
        /* first search, look for relevant entries and push to top of list
	 also free anything which has expired */
        let mut next: Crec;
        let mut up: Crec;
        let mut insert: Crec;
        let mut chainp: Crec = ans.clone();
        let mut ins_flags: u32 = 0;
        up = hash_bucket(&name);
        crecp = *up;
        while !crecp.is_null() {
            next = crecp.hash_next;
            if is_expired(now, crecp.unwrap()) == 0 &&
                   is_outdated_cname_pointer(crecp) == 0 {
                if crecp.flags & (1) << 3 !=
                       0 && crecp.flags & prot != 0 &&
                       hostname_isequal(cache_get_name(crecp), name) != 0 {
                    if crecp.flags &
                           ((1) << 6 |
                                (1) << 4 |
                                (1) << 13) != 0
                       {
                        *chainp = crecp;
                        chainp = &mut crecp.next
                    } else { cache_unlink(crecp); cache_link(crecp); }
                    /* Move all but the first entry up the hash chain
		     this implements round-robin. 
		     Make sure that re-ordering doesn't break the hash-chain
		     order invariants. 
		  */
                    if !insert.is_null() &&
                           crecp.flags &
                               ((1) << 2 |
                                    (1) << 0)
                               == ins_flags {
                        *up = crecp.hash_next;
                        crecp.hash_next = *insert;
                        *insert = crecp;
                        insert = &mut crecp.hash_next
                    } else {
                        if insert.is_null() && no_rr == 0 {
                            insert = up;
                            ins_flags =
                                crecp.flags &
                                    ((1) << 2 |
                                         (1) <<
                                             0)
                        }
                        up = &mut crecp.hash_next
                    }
                } else {
                    /* case : not expired, incorrect entry. */
                    up = &mut crecp.hash_next
                }
            } else {
                /* expired entry, free it */
                *up = crecp.hash_next;
                if crecp.flags &
                       ((1) << 6 |
                            (1) << 4 |
                            (1) << 13) == 0 {
                    cache_unlink(crecp);
                    cache_free(crecp);
                }
            }
            crecp = next
        }
        *chainp = cache_head
    }
    if !ans.is_null() &&
           ans.flags & (1) << 3 != 0 &&
           ans.flags & prot != 0 &&
           hostname_isequal(cache_get_name(ans), name) != 0 {
        return ans
    }
    return 0 ;
}

pub fn cache_find_by_addr(mut crecp: Option<&mut Vec<Crec>>, mut addr: &mut NetAddress, mut now: time::Instant, mut prot: u32) -> Crec {
    let mut ans: Crec = Default::default();
    let mut addrlen: i32 = if prot == (1 << 8) {
            16
        } else { 4 };
    if crecp.is_some() {
        let mut crec_list = crecp.unwrap();
        for crec in crec_list {
            ans = crec.clone()
        }
    } else {
        /* first search, look for relevant entries and push to top of list
	 also free anything which has expired. All the reverse entries are at the
	 start of the hash chain, so we can give up when we find the first 
	 non-REVERSE one.  */
        let mut i: i32 = 0;
        let mut up: Crec;
        let mut chainp: Crec = ans;
        i = 0;
        while i < hash_size {
            crecp = hash_table.offset(i);
            up = hash_table.offset(i);
            while !crecp.is_null() &&
                      crecp.flags & (1) << 2
                          != 0 {
                if is_expired(now, crecp) == 0 {
                    if crecp.flags & prot != 0 &&
                           memcmp(&mut crecp.addr
                                  addr,
                                  addrlen) ==
                               0 {
                        if crecp.flags &
                               ((1) << 6 |
                                    (1) << 4 |
                                    (1) << 13)
                               != 0 {
                            *chainp = crecp;
                            chainp = &mut crecp.next
                        } else { cache_unlink(crecp); cache_link(crecp); }
                    }
                    up = &mut crecp.hash_next
                } else {
                    *up = crecp.hash_next;
                    if crecp.flags &
                           ((1) << 6 |
                                (1) << 4 |
                                (1) << 13) == 0
                       {
                        cache_unlink(crecp);
                        cache_free(crecp);
                    }
                }
                crecp = crecp.hash_next
            }
            i += 1
        }
        *chainp = cache_head
    }
    if !ans.is_null() &&
           ans.flags & (1) << 2 != 0 &&
           ans.flags & prot != 0 &&
           memcmp(&mut ans.addr ,
                  addr, addrlen) ==
               0 {
        return ans
    }
    return 0 ;
}
fn add_hosts_entry(mut cache: Crec,
                                     mut addr: &mut NetAddress,
                                     mut addrlen: i32,
                                     mut index: u32,
                                     mut rhash: &mut Crec,
                                     mut hashsz: i32) {
    let mut lookup: Crec =
        cache_find_by_name(0 , cache_get_name(cache),
                           0,
                           cache.flags &
                               ((1) << 7 |
                                    (1) << 8));
    let mut i: i32 = 0;
    let mut j: u32 = 0;
    /* Remove duplicates in hosts files. */
    if !lookup.is_null() &&
           lookup.flags & (1) << 6 != 0 &&
           memcmp(&mut lookup.addr ,
                  addr, addrlen) ==
               0 {
        free(cache);
        return
    }
    /* Ensure there is only one address -> name mapping (first one trumps) 
     We do this by steam here, The entries are kept in hash chains, linked
     by ->next (which is unused at this point) held in hash buckets in
     the array rhash, hashed on address. Note that rhash and the values
     in ->next are only valid  whilst reading hosts files: the buckets are
     then freed, and the ->next pointer used for other things. 

     Only insert each unique address once into this hashing structure.

     This complexity avoids O(n^2) divergent CPU use whilst reading
     large (10000 entry) hosts files. 

     Note that we only do this process when bulk-reading hosts files, 
     for incremental reads, rhash is NULL, and we use cache lookups
     instead.
  */
    if !rhash.is_null() {
        /* hash address */
        j = 0;
        i = 0;
        while i < addrlen {
            j =
                j.wrapping_mul(2                             libc::c_uint).wrapping_add(*(addr          mut Vec<u8>).offset(i              isize)
                                                                         libc::c_uint).wrapping_rem(hashsz          libc::c_uint);
            i += 1
        }
        lookup = *rhash.offset(j);
        while !lookup.is_null() {
            if lookup.flags & cache.flags &
                   ((1) << 7 |
                        (1) << 8) != 0 &&
                   memcmp(&mut lookup.addr
                          addr,
                          addrlen) == 0 {
                cache.flags &= !((1) << 2);
                break ;
            } else { lookup = lookup.next }
        }
        /* maintain address hash chain, insert new unique address */
        if lookup.is_null() {
            cache.next = *rhash.offset(j);
            let ref mut fresh9 = *rhash.offset(j);
            *fresh9 = cache
        }
    } else {
        /* incremental read, lookup in cache */
        lookup =
            cache_find_by_addr(0 , addr,
                               0,
                               cache.flags &
                                   ((1) << 7 |
                                        (1) <<
                                            8));
        if !lookup.is_null() &&
               lookup.flags & (1) << 6 != 0
           {
            cache.flags &= !((1) << 2)
        }
    }
    cache.uid = index;
    memcpy(&mut cache.addr ,
           addr, addrlen);
    cache_hash(cache);
    make_non_terminals(cache);
}
fn eatspace(mut f: &mut FILE) -> i32 {
    let mut c: i32 = 0;
    let mut nl: i32 = 0;
    loop  {
        c = getc(f);
        if c == '#' as i32 {
            while c != '\n' as i32 && c != -(1) { c = getc(f) }
        }
        if c == -(1) { return 1 }
        if *(*__ctype_b_loc()).offset(c) &
               _ISSPACE  == 0 {
            ungetc(c, f);
            return nl
        }
        if c == '\n' as i32 { nl += 1 }
    };
}
fn gettok(mut f: &mut FILE, mut token: &mut String)
 -> i32 {
    let mut c: i32 = 0;
    let mut count: i32 = 0;
    loop  {
        c = getc(f);
        if c == -(1) {
            return if count == 0 {
                       -(1)
                   } else { 1 }
        }
        if *(*__ctype_b_loc()).offset(c) &
               _ISSPACE  != 0
               || c == '#' as i32 {
            ungetc(c, f);
            return eatspace(f)
        }
        if count < 1025 - 1 {
            let fresh10 = count;
            count = count + 1;
            *token.offset(fresh10) = c;
            *token.offset(count) = 0
        }
    };
}

pub fn read_hostsfile(mut filename: &mut String,
                                        mut index: u32,
                                        mut cache_size: i32,
                                        mut rhash: &mut Crec,
                                        mut hashsz: i32)
                                        -> i32 {
    let mut f: FILE = fopen(filename, "r" );
    let mut token: &mut String = daemon.namebuff;
    let mut domain_suffix: &mut String = 0 ;
    let mut addr_count: i32 = 0;
    let mut name_count: i32 = cache_size;
    let mut lineno: i32 = 1;
    let mut flags: u32 = 0;
    let mut addr: NetAddress = NetAddress {addr4: NetAddress {s_addr: 0,},};
    let mut atnl: i32 = 0;
    let mut addrlen: i32 = 0;
    if f.is_null() {
        my_syslog(3,
                  "failed to load names from %s: %s", filename,
                  strerror(*__errno_location()));
        return cache_size
    }
    lineno += eatspace(f);
    loop  {
        atnl = gettok(f, token);
        if !(atnl != -(1)) { break ; }
        if inet_pton(2, token,
                     &mut addr ) >
               0 {
            flags =
                (1) << 6 |
                    (1) << 0 |
                    (1) << 3 |
                    (1) << 2 |
                    (1) << 7;
            addrlen = 4;
            domain_suffix = get_domain(addr.addr4)
        } else if inet_pton(10, token,
                            &mut addr ) >
                      0 {
            flags =
                (1) << 6 |
                    (1) << 0 |
                    (1) << 3 |
                    (1) << 2 |
                    (1) << 8;
            addrlen = 16;
            domain_suffix = get_domain6(&mut addr.addr6)
        } else {
            my_syslog(3,
                      "bad address at %s line %d", filename, lineno);
            while atnl == 0 { atnl = gettok(f, token) }
            lineno += atnl;
            continue ;
        }
        addr_count += 1;
        /* rehash every 1000 names. */
        if !rhash.is_null() && name_count - cache_size > 1000 {
            rehash(name_count);
            cache_size = name_count
        }
        while atnl == 0 {
            let mut cache: Crec = Crec::new();
            let mut fqdn: i32 = 0;
            let mut nomem: i32 = 0;
            let mut canon: String = String::new();
            atnl = gettok(f, token);
            if atnl == -(1) { break ; }
            fqdn = !strchr(token, '.' as i32).is_null();
            canon = canonicalise(&mut token, nomem).unwrap();
            if !canon.is_null() {
                /* If set, add a version of the name with a default domain appended */
                if daemon.options[9] != 0 && !domain_suffix.is_null() && fqdn == 0 &&
                       {
                           // cache = whine_malloc((::std::mem::size_of::<Crec>()  ).wrapping_sub(50).wrapping_add(strlen(canon)).wrapping_add(2).wrapping_add(strlen(domain_suffix)));
                           // !cache.is_null()
                           true
                       }
                {
                    cache.name.sname = canon.as_str();
                    cache.name.sname += ".";
                    cache.name.sname += domain_suffix;
                    cache.flags = flags;
                    cache.ttd = daemon.local_ttl;
                    add_hosts_entry(cache, &mut addr, addrlen, index, rhash, hashsz);
                    name_count += 1
                }
                // cache = whine_malloc((::std::mem::size_of::<Crec>()).wrapping_sub(50).wrapping_add(canon.len()).wrapping_add(1));
                if !cache.is_null() {
                    cache.name.sname = canon.as_str();
                    cache.flags = flags;
                    cache.ttd = daemon.local_ttl;
                    add_hosts_entry(cache, &mut addr, addrlen, index, rhash, hashsz);
                    name_count += 1
                }
                // free(canon);
            } else if nomem == 0 {
                // my_syslog(3, "bad name at %s line %d" , filename, lineno);
            }
        }
        lineno += atnl
    }
    fclose(f);
    if !rhash.is_null() { rehash(name_count); }
    // my_syslog(6, "read %s - %d addresses" , filename, addr_count);
    name_count
}

pub fn cache_reload(daemon: &mut DnsmasqDaemon) {
    let mut cache: Crec = Default::default();
    let mut up: Crec = Default::default();
    let mut tmp: Crec = Default::default();
    let mut revhashsz: i32 = 0;
    let mut i: i32 = 0;
    let mut total_size: usize = daemon.cachesize;
    let mut ah: HostsFile = Default::default();
    let mut hr: HostRecord = Default::default();
    let mut nl: NameListEntry = Default::default();
    let mut a: Cname = Default::default();
    let mut lrec: Crec = Default::default();
    let mut mx: MxSrvRecord = Default::default();
    let mut txt: TxtRecord = Default::default();
    let mut intr: InterfaceName = Default::default();
    let mut ptr: PtrRecord = Default::default();
    let mut naptr: NaPtr = Default::default();
    daemon.metrics[METRIC_DNS_CACHE_INSERTED ] = 0;
    daemon.metrics[METRIC_DNS_CACHE_LIVE_FREED ] = 0;
    i = 0;
    while i < hash_size {
        cache = hash_table.offset(i);
        up = &mut hash_table.offset(i);
        while !cache.is_null() {
            cache_blockdata_free(cache);
            tmp = cache.hash_next;
            if cache.flags & ((1) << 6 | (1) << 13) != 0 {
                *up = cache.hash_next;
                // free(cache);
            } else if cache.flags & (1) << 4 == 0 {
                *up = cache.hash_next;
                if cache.flags & (1) << 9 != 0 {
                    (*cache.name.bname).next = big_free;
                    big_free = cache.name.bname
                }
                cache.flags = 0
            } else { up = &mut cache.hash_next }
            cache = tmp
        }
        i += 1
    }
    /* Add locally-configured CNAMEs to the cache */
    for a in daemon.cnames {
        if a.alias.offset(1) != '*' as i32 && {
                   // cache = whine_malloc((::std::mem::size_of::<Crec>()).wrapping_add(::std::mem::size_of::<&mut String>()).wrapping_sub(50 ));
                   // !cache.is_null()
            true
               } {
            cache.flags = (1) << 3 |  (1) << 1 | (1) << 11 | (1) << 0 | (1) << 13;
            cache.ttd = a.ttl;
            cache.name.namep = a.alias;
            cache.addr.cname.target.name = a.target;
            cache.addr.cname.is_name_ptr = 1;
            cache.uid = 0;
            cache_hash(cache);
            make_non_terminals(cache);
        }
        // a = a.next
    }
    /* borrow the packet buffer for a temporary by-address hash */
    daemon.packet.clear();
    
    revhashsz = (daemon.packet_buff_sz).wrapping_div(::std::mem::size_of::<Crec>());
    /* we overwrote the buffer... */
    daemon.srv_save = Default::default();
    /* Do host_records in config. */
    for hr in daemon.host_records {
        for nl in hr.names {
            if hr.flags & 2 != 0 && {
                       // cache = whine_malloc((::std::mem::size_of::<Crec>() ).wrapping_add(::std::mem::size_of::<&mut String>()).wrapping_sub(50));
                       // !cache.is_null()
                        true
                   } {
                cache.name.namep = nl.name;
                cache.ttd = hr.ttl;
                cache.flags =
                    (1) << 6 |
                        (1) << 0 |
                        (1) << 3 |
                        (1) << 2 |
                        (1) << 7 |
                        (1) << 1 |
                        (1) << 13;
                add_hosts_entry(cache, &hr.addr, 4, 1, daemon.packet, revhashsz);
            }
            if hr.flags & 1 != 0 &&
                   {
                       // cache =
                       //     whine_malloc((::std::mem::size_of::<Crec>() ).wrapping_add(::std::mem::size_of::<&mut String>()).wrapping_sub(50))
                       //         ;
                       // !cache.is_null()
                       true
                   } {
                cache.name.namep = nl.name;
                cache.ttd = hr.ttl;
                cache.flags =
                    (1) << 6 |
                        (1) << 0 |
                        (1) << 3 |
                        (1) << 2 |
                        (1) << 8 |
                        (1) << 1 |
                        (1) << 13;
                add_hosts_entry(cache,
                                &mut hr.addr6, 16,
                                1,
                                daemon.packet,
                                revhashsz);
            }
            nl = nl.next
        }
        hr = hr.next
    }
    if daemon.options[(4).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                     ] &
           (1) <<
               (4).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
           != 0 && daemon.addn_hosts.is_null() {
        if daemon.cachesize > 0 {
            my_syslog(6,
                      "cleared cache");
        }
    } else {
        if daemon.options[(4 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                         ] &
               (1) <<
                   (4 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8))
               == 0 {
            total_size =
                read_hostsfile("/etc/hosts",
                               2, total_size,
                               daemon.packet,
                               revhashsz)
        }
        daemon.addn_hosts =
            expand_filelist(daemon.addn_hosts);
        ah = daemon.addn_hosts;
        while !ah.is_null() {
            if ah.flags & 2 == 0 {
                total_size =
                    read_hostsfile(ah.fname, ah.index, total_size,
                                   daemon.packet,
                                   revhashsz)
            }
            ah = ah.next
        }
    }
    /* Make non-terminal records for all locally-define RRs */
    lrec.flags =
        (1) << 3 |
            (1) << 13 |
            (1) << 1 |
            (1) << 0;
    txt = daemon.txt;
    while !txt.is_null() {
        lrec.name.namep = txt.name;
        make_non_terminals(&mut lrec);
        txt = txt.next
    }
    naptr = daemon.naptr;
    while !naptr.is_null() {
        lrec.name.namep = naptr.name;
        make_non_terminals(&mut lrec);
        naptr = naptr.next
    }
    mx = daemon.mxnames;
    while !mx.is_null() {
        lrec.name.namep = mx.name;
        make_non_terminals(&mut lrec);
        mx = mx.next
    }
    intr = daemon.int_names;
    while !intr.is_null() {
        lrec.name.namep = intr.name;
        make_non_terminals(&mut lrec);
        intr = intr.next
    }
    ptr = daemon.ptr;
    while !ptr.is_null() {
        lrec.name.namep = ptr.name;
        make_non_terminals(&mut lrec);
        ptr = ptr.next
    }
    set_dynamic_inotify(8, total_size,
                        daemon.packet,
                        revhashsz);
}

pub fn a_record_from_hosts(mut name: &mut String,
                                             mut now: time::Instant) -> NetAddress {
    let mut crecp: Crec = 0 ;
    let mut ret: NetAddress = NetAddress {s_addr: 0,};
    loop  {
        crecp =
            cache_find_by_name(crecp, name, now,
                               (1) << 7);
        if crecp.is_null() { break ; }
        if crecp.flags & (1) << 6 != 0 {
            return crecp.addr.addr4
        }
    }
    my_syslog((3) << 3 | 4,
              "No IPv4 address found for %s" , name);
    ret.s_addr = 0;
    return ret;
}

pub fn cache_unhash_dhcp() {
    let mut cache: Crec = 0 ;
    let mut up: Crec = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < hash_size {
        cache = hash_table.offset(i);
        up = &mut hash_table.offset(i);
        while !cache.is_null() {
            if cache.flags & (1) << 4 != 0 {
                *up = cache.hash_next;
                cache.next = dhcp_spare;
                dhcp_spare = cache
            } else { up = &mut cache.hash_next }
            cache = cache.hash_next
        }
        i += 1
    };
}

pub fn cache_add_dhcp_entry(mut host_name:
                                                  &mut String,
                                              mut prot: i32,
                                              mut host_address: &mut NetAddress,
                                              mut ttd: time::Instant) {
    let mut crec: Crec = 0 ;
    let mut fail_crec: Crec = 0 ;
    let mut flags: u32 = (1) << 7;
    let mut in_hosts: i32 = 0;
    let mut addrlen: usize =
        ::std::mem::size_of::<NetAddress>();
    if prot == 10 {
        flags = (1) << 8;
        addrlen = ::std::mem::size_of::<In6Addr>()
    }
    inet_ntop(prot, host_address,
              daemon.addrbuff, 46);
    loop  {
        crec =
            cache_find_by_name(crec, host_name, 0,
                               flags |
                                   (1) << 11);
        if crec.is_null() { break ; }
        /* check all addresses associated with name */
        if crec.flags &
               ((1) << 6 |
                    (1) << 13) != 0 {
            if crec.flags & (1) << 11 != 0 {
                my_syslog((3) << 3 |
                              4,
                          "%s is a CNAME, not giving it to the DHCP lease of %s"
                              , host_name,
                          daemon.addrbuff);
            } else if memcmp(&mut crec.addr
                             host_address, addrlen) ==
                          0 {
                in_hosts = 1
            } else { fail_crec = crec }
        } else {
            if !(crec.flags & (1) << 4 == 0)
               {
                continue ;
            }
            cache_scan_free(host_name, 0 ,
                            1 ,
                            0,
                            crec.flags &
                                (flags |
                                     (1) << 11
                                     |
                                     (1) << 3),
                            0, 0);
            break ;
        }
    }
    /* if in hosts, don't need DHCP record */
    if in_hosts != 0 { return }
    /* Name in hosts, address doesn't match */
    if !fail_crec.is_null() {
        inet_ntop(prot,
                  &mut fail_crec.addr                 *const libc::c_void, daemon.namebuff,
                  1025);
        // my_syslog((3) << 3 | 4,
        //           "not giving name %s to the DHCP lease of %s because the name exists in %s with address %s"
        //               , host_name,
        //           daemon.addrbuff, record_source(fail_crec.uid),
        //           daemon.namebuff);
        return
    }
    crec =
        cache_find_by_addr(0 , host_address,
                           0, flags);
    if !crec.is_null() {
        if crec.flags & (1) << 5 != 0 {
            flags |= (1) << 2;
            cache_scan_free(0 , host_address,
                            1 ,
                            0, flags,
                            0, 0);
        }
    } else { flags |= (1) << 2 }
    crec = dhcp_spare;
    if !crec.is_null() {
        dhcp_spare = dhcp_spare.next
    } else {
        /* need new one */
        // crec =
        //     whine_malloc((::std::mem::size_of::<Crec>()).wrapping_add(::std::mem::size_of::<&mut String>()
        //                                                   ).wrapping_sub(50    libc::c_int
        //                                                                                                                   ))

    }
    if !crec.is_null() {
        /* malloc may fail */
        crec.flags =
            flags | (1) << 1 |
                (1) << 4 |
                (1) << 3;
        if ttd == 0 {
            crec.flags |= (1) << 0
        } else { crec.ttd = ttd }
        crec.addr = *host_address;
        crec.name.namep = host_name;
        crec.uid = 0;
        cache_hash(crec);
        make_non_terminals(crec);
    };
}
/* Called when we put a local or DHCP name into the cache.
   Creates empty cache entries for subnames (ie,
   for three.two.one, for two.one and one), without
   F_IPV4 or F_IPV6 or F_CNAME set. These convert
   NXDOMAIN answers to NoData ones. */
fn make_non_terminals(mut source: Crec) {
    let mut name: &mut String = cache_get_name(source);
    let mut crecp: Crec = 0 ;
    let mut tmp: Crec = 0 ;
    let mut up: Crec = 0;
    let mut type_0: i32 =
        ((1) << 6 |
             (1) << 13);
    if source.flags & (1) << 4 != 0 {
        type_0 = ((1) << 4)
    }
    /* First delete any empty entries for our new real name. Note that
     we only delete empty entries deriving from DHCP for a new DHCP-derived
     entry and vice-versa for HOSTS and CONFIG. This ensures that 
     non-terminals from DHCP go when we reload DHCP and 
     for HOSTS/CONFIG when we re-read. */
    up = hash_bucket(name);
    crecp = *up;
    while !crecp.is_null() {
        tmp = crecp.hash_next;
        if is_outdated_cname_pointer(crecp) == 0 &&
               crecp.flags & (1) << 3 != 0
               && crecp.flags & type_0 != 0 &&
               crecp.flags &
                   ((1) << 7 |
                        (1) << 8 |
                        (1) << 11 |
                        (1) << 30 |
                        (1) << 12 |
                        (1) << 14) == 0 &&
               hostname_isequal(name, cache_get_name(crecp)) != 0 {
            *up = crecp.hash_next;
            if type_0 &
                   (1) << 4 != 0 {
                crecp.next = dhcp_spare;
                dhcp_spare = crecp
            } else { free(crecp); }
            break ;
        } else { up = &mut crecp.hash_next; crecp = tmp }
    }
    loop  {
        name = strchr(name, '.' as i32);
        if name.is_null() { break ; }
        name = name.offset(1);
        /* Look for one existing, don't need another */
        crecp = *hash_bucket(name);
        while !crecp.is_null() {
            if is_outdated_cname_pointer(crecp) == 0 &&
                   crecp.flags & (1) << 3 !=
                       0 && crecp.flags & type_0 != 0 &&
                   hostname_isequal(name, cache_get_name(crecp)) != 0 {
                break ;
            }
            crecp = crecp.hash_next
        }
        if !crecp.is_null() {
            /* If the new name expires later, transfer that time to
	     empty non-terminal entry. */
            if crecp.flags & (1) << 0 == 0 {
                if source.flags & (1) << 0
                       != 0 {
                    crecp.flags |= (1) << 0
                } else if difftime(crecp.ttd, source.ttd) <
                              0  {
                    crecp.ttd = source.ttd
                }
            }
        } else {
            if source.flags & (1) << 4 != 0
                   && !dhcp_spare.is_null() {
                crecp = dhcp_spare;
                dhcp_spare = dhcp_spare.next
            } else {
                // crecp =
                //     whine_malloc((::std::mem::size_of::<Crec>()).wrapping_add(::std::mem::size_of::<&mut String>()
                //                                                           ).wrapping_sub(50                    libc::c_int             ))

            }
            if !crecp.is_null() {
                crecp.flags =
                    (source.flags |
                         (1) << 1) &
                        !((1) << 7 |
                              (1) << 8 |
                              (1) << 11 |
                              (1) << 30 |
                              (1) << 12 |
                              (1) << 14 |
                              (1) << 2);
                crecp.ttd = source.ttd;
                crecp.name.namep = name;
                cache_hash(crecp);
            }
        }
    };
}

pub fn cache_make_stat(mut t: &mut TxtRecord)
 -> i32 {
    // static mut buff: &mut String =
    //     0 ;
    // static mut bufflen: i32 = 60;
    let mut buff: String = String::new();
    let mut len: i32 = 0;
    let mut serv: Server = 0;
    let mut serv1: Server = 0;
    let mut p: &mut String = 0 ;
    if buff.is_null() &&
           {
               // buff =
               //     whine_malloc(60 );
               // buff.is_null()
               false
           } {
        return 0
    }
    p = buff;
    match t.stat {
        1 => {
            sprintf(buff.offset(1), "%d" , daemon.cachesize);
        }
        2 => {
            sprintf(buff.offset(1), "%d" , daemon.metrics[METRIC_DNS_CACHE_INSERTED                                            libc::c_int ]);
        }
        3 => {
            sprintf(buff.offset(1),
                    "%d" ,
                    daemon.metrics[METRIC_DNS_CACHE_LIVE_FREED                                            libc::c_int ]);
        }
        4 => {
            sprintf(buff.offset(1),
                    "%u" ,
                    daemon.metrics[METRIC_DNS_QUERIES_FORWARDED                                            libc::c_int ]);
        }
        5 => {
            sprintf(buff.offset(1),
                    "%u" ,
                    daemon.metrics[METRIC_DNS_LOCAL_ANSWERED                                            libc::c_int ]);
        }
        6 => {
            sprintf(buff.offset(1),
                    "%u" ,
                    daemon.metrics[METRIC_DNS_AUTH_ANSWERED                                            libc::c_int ]);
        }
        7 => {
            /* sum counts from different records for same server */
            serv = daemon.servers; /* length */
            while !serv.is_null() {
                serv.flags &= !(512);
                serv = serv.next
            }
            serv = daemon.servers;
            while !serv.is_null() {
                if serv.flags &
                       (2 | 4 |
                            512 | 1024 |
                            2048) == 0 {
                    let mut new: &mut String = 0 ;
                    let mut lenp: &mut String = 0 ;
                    let mut port: i32 = 0;
                    let mut newlen: i32 = 0;
                    let mut bytes_avail: i32 = 0;
                    let mut bytes_needed: i32 = 0;
                    let mut queries: u32 =
                        0;
                    let mut failed_queries: u32 =
                        0;
                    serv1 = serv;
                    while !serv1.is_null() {
                        if serv1.flags &
                               (2 | 4 |
                                    512 | 1024 |
                                    2048) == 0 &&
                               NetAddress_isequal(&mut serv.addr,
                                                &mut serv1.addr) != 0 {
                            serv1.flags |= 512;
                            queries = queries.wrapping_add(serv1.queries);
                            failed_queries =
                                failed_queries.wrapping_add(serv1.failed_queries)
                        }
                        serv1 = serv1.next
                    }
                    port =
                        prettyprint_addr(&mut serv.addr,
                                         daemon.addrbuff);
                    let fresh11 = p;
                    p = p.offset(1);
                    lenp = fresh11;
                    bytes_avail =
                        (bufflen -
                             p.wrapping_offset_from(buff))                      libc::c_int;
                    bytes_needed =
                        snprintf(p, bytes_avail,
                                 "%s#%d %u %u"                                *const libc::c_char,
                                 daemon.addrbuff, port, queries,
                                 failed_queries);
                    if bytes_needed >= bytes_avail {
                        /* expand buffer if necessary */
                        newlen =
                            bytes_needed + 1 + bufflen -
                                bytes_avail;
                        // new =
                        //     whine_malloc(newlen )                          &mut String;
                        if new.is_null() { return 0 }
                        memcpy(new,
                               buff,
                               bufflen);
                        free(buff);
                        p =
                            new.offset(p.wrapping_offset_from(buff)                                     i32);
                        lenp = p.offset(-(1));
                        buff = new;
                        bufflen = newlen;
                        bytes_avail =
                            (bufflen -
                                 p.wrapping_offset_from(buff))
                               ;
                        bytes_needed =
                            snprintf(p, bytes_avail,
                                     "%s#%d %u %u",
                                     daemon.addrbuff, port,
                                     queries, failed_queries)
                    }
                    *lenp = bytes_needed;
                    p = p.offset(bytes_needed)
                }
                serv = serv.next
            }
            t.txt = buff;
            t.len =
                p.wrapping_offset_from(buff);
            return 1
        }
        _ => { }
    }
    len = strlen(buff.offset(1));
    t.txt = buff;
    t.len = (len + 1) ;
    *buff = len;
    return 1;
}
/* There can be names in the cache containing control chars, don't 
   mess up logging or open security holes. */
fn sanitise(mut name: &mut String)
 -> &mut String {
    let mut r: mut Vec<u8> = 0;
    if !name.is_null() {
        r = name;
        while *r != 0 {
            if *(*__ctype_b_loc()).offsetr             libc::c_int &
                   _ISPRINT  ==
                   0 {
                return "<name unprintable>"
            }
            r = r.offset(1)
        }
    }
    return name;
}

pub fn dump_cache(mut now: time::Instant) {
    let mut serv: Server = 0;
    let mut serv1: Server = 0;
    my_syslog(6,
              "time %lu" ,
              now);
    my_syslog(6,
              "cache size %d, %d/%d cache insertions re-used unexpired cache entries."
                  ,
              daemon.cachesize,
              daemon.metrics[METRIC_DNS_CACHE_LIVE_FREED  ],
              daemon.metrics[METRIC_DNS_CACHE_INSERTED  ]);
    my_syslog(6,
              "queries forwarded %u, queries answered locally %u"            *const u8,
              daemon.metrics[METRIC_DNS_QUERIES_FORWARDED  ],
              daemon.metrics[METRIC_DNS_LOCAL_ANSWERED  ]);
    my_syslog(6,
              "queries for authoritative zones %u" ,
              daemon.metrics[METRIC_DNS_AUTH_ANSWERED  ]);
    blockdata_report();
    /* sum counts from different records for same server */
    serv = daemon.servers;
    while !serv.is_null() {
        serv.flags &= !(512);
        serv = serv.next
    }
    serv = daemon.servers;
    while !serv.is_null() {
        if serv.flags &
               (2 | 4 | 512 |
                    1024 | 2048) == 0 {
            let mut port: i32 = 0;
            let mut queries: u32 = 0;
            let mut failed_queries: u32 =
                0;
            serv1 = serv;
            while !serv1.is_null() {
                if serv1.flags &
                       (2 | 4 |
                            512 | 1024 |
                            2048) == 0 &&
                       NetAddress_isequal(&mut serv.addr, &mut serv1.addr)
                           != 0 {
                    serv1.flags |= 512;
                    queries = queries.wrapping_add(serv1.queries);
                    failed_queries =
                        failed_queries.wrapping_add(serv1.failed_queries)
                }
                serv1 = serv1.next
            }
            port =
                prettyprint_addr(&mut serv.addr,
                                 daemon.addrbuff);
            my_syslog(6,
                      "server %s#%d: queries sent %u, retried or failed %u"
                          ,
                      daemon.addrbuff, port, queries,
                      failed_queries);
        }
        serv = serv.next
    }
    if daemon.options[(6).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                     ] &
           (1) <<
               (6).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
           != 0 ||
           daemon.options[(2 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                         ] &
               (1) <<
                   (2 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8))
               != 0 {
        let mut cache: Crec = 0 ;
        let mut i: i32 = 0;
        my_syslog(6,
                  "Host                                     Address                        Flags      Expires"
                      );
        i = 0;
        while i < hash_size {
            cache = hash_table.offset(i);
            while !cache.is_null() {
                let mut t: &mut String =
                    " "                   &mut String;
                let mut a: &mut String = daemon.addrbuff;
                let mut p: &mut String = daemon.namebuff;
                let mut n: &mut String = cache_get_name(cache);
                *a = 0;
                if strlen(n) == 0 &&
                       cache.flags &
                           (1) << 2 == 0 {
                    n =
                        "<Root>"
                }
                p =
                    p.offset(sprintf(p,
                                     "%-30.30s ", sanitise(n))                           isize);
                if cache.flags & (1) << 11
                       != 0 && is_outdated_cname_pointer(cache) == 0 {
                    a = sanitise(cache_get_cname_target(cache))
                } else if cache.flags &
                              (1) << 30 != 0 &&
                              cache.flags &
                                  (1) << 5 == 0
                 {
                    let mut targetlen: i32 =
                        cache.addr.srv.targetlen;
                    let mut len: isize =
                        sprintf(a,
                                "%u %u %u "                               *const libc::c_char,
                                cache.addr.srv.priority,
                                cache.addr.srv.weight,
                                cache.addr.srv.srvport)                      isize;
                    if targetlen >
                           40 - len {
                        targetlen =
                            (40 - len)                          libc::c_int
                    }
                    blockdata_retrieve(cache.addr.srv.target,
                                       targetlen ,
                                       a.offset(len)                                    Vec<u8>);
                    *a.offset((len + targetlen)) =
                        0
                } else if cache.flags &
                              (1) << 5 == 0 ||
                              cache.flags &
                                  (1) << 3 == 0
                 {
                    a = daemon.addrbuff;
                    if cache.flags &
                           (1) << 7 != 0 {
                        inet_ntop(2,
                                  &mut cache.addr                a,
                                  46);
                    } else if cache.flags &
                                  (1) << 8 != 0
                     {
                        inet_ntop(10,
                                  &mut cache.addr                a,
                                  46);
                    }
                }
                if cache.flags & (1) << 7 !=
                       0 {
                    t =
                        "4"
                } else if cache.flags &
                              (1) << 8 != 0 {
                    t =
                        "6"
                } else if cache.flags &
                              (1) << 11 != 0 {
                    t =
                        "C"
                } else if cache.flags &
                              (1) << 30 != 0 {
                    t =
                        "V"
                }
                p =
                    p.offset(sprintf(p,
                                     "%-40.40s %s%s%s%s%s%s%s%s%s  ", a,
                                     t,
                                     if cache.flags &
                                            (1) <<
                                                3 != 0 {
                                         "F"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                2 != 0 {
                                         "R"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                0 != 0 {
                                         "I"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                4 != 0 {
                                         "D"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                5 != 0 {
                                         "N"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                10 != 0 {
                                         "X"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                6 != 0 {
                                         "H"
                                     } else {
                                         " "
                                     },
                                     if cache.flags &
                                            (1) <<
                                                15 != 0 {
                                         "V"
                                     } else {
                                         " "
                                     }));
                p =
                    p.offset(sprintf(p,
                                     "%s",
                                     if cache.flags &
                                            (1) <<
                                                0 != 0 {
                                         "\n"
                                     } else {
                                         ctime(&mut cache.ttd)                                       *const libc::c_char
                                     }));
                /* ctime includes trailing \n - eat it */
                *p.offset(-(1)) =
                    0                  libc::c_char; /* strlen("type=xxxxx") */
                my_syslog(6,
                          "%s" ,
                          daemon.namebuff); /* braces */
                cache = cache.hash_next
            } /* terminator */
            i += 1
        }
    };
}

pub fn record_source(mut index: u32)
 -> &mut String {
    let mut ah: HostsFile = 0;
    if index == 1 {
        return "config"              &mut String
    } else {
        if index == 2 {
            return "/etc/hosts"
        }
    }
    ah = daemon.addn_hosts;
    while !ah.is_null() {
        if ah.index == index { return ah.fname }
        ah = ah.next
    }
    ah = daemon.dynamic_dirs;
    while !ah.is_null() {
        if ah.index == index { return ah.fname }
        ah = ah.next
    }
    return "<unknown>"          &mut String;
}

pub fn querystr(mut desc: &mut String,
                                  mut type_0: u16)
 -> &mut String {
    let mut i: u32 = 0;
    let mut len: i32 = 10;
    let mut types: *const libc::c_char = 0;
    static mut buff: &mut String =
        0 ;
    static mut bufflen: i32 = 0;
    i = 0;
    while (i) <
              (::std::mem::size_of::<[C2rustUnnamed10; 40]>()).wrapping_div(::std::mem::size_of::<C2rustUnnamed10>()
                                                  ) {
        if TYPESTR[i ].type_0 == type_0 {
            types = TYPESTR[i ].name;
            len = strlen(types);
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if !desc.is_null() {
        len += 2;
        len =
            (len).wrapping_add(strlen(desc))

    }
    len += 1;
    if buff.is_null() || bufflen < len {
        if !buff.is_null() {
            free(buff);
        } else if len < 20 { len = 20 }
        // buff = whine_malloc(len ) ;
        bufflen = len
    }
    if !buff.is_null() {
        if !desc.is_null() {
            if !types.is_null() {
                sprintf(buff,
                        "%s[%s]" ,
                        desc, types);
            } else {
                sprintf(buff,
                        "%s[type=%d]" , desc, type_0);
            }
        } else if !types.is_null() {
            sprintf(buff, "<%s>" ,
                    types);
        } else {
            sprintf(buff, "type=%d" ,
                    type_0);
        }
    }
    return if !buff.is_null() {
               buff
           } else { ""  }
}

// pub fn log_query(daemon: &mut DnsmasqDaemon, mut flags: u32, mut name: &String, mut addr: &NetAddress, mut arg: Option<&String>) {
//     let mut source: String;
//     let mut dest: String = String::from_utf8(daemon.addrbuff).unwrap();
//     let mut verb: String = String::from("is");
//     if daemon.options[2] == false {
//         return
//     }
//     name = sanitise(name);
//     if !addr.is_null() {
//         if flags & (1) << 23 != 0 {
//             sprintf(daemon.addrbuff, arg,
//                     addr.log.keytag,
//                     addr.log.algo,
//                     addr.log.digest);
//         } else if flags & (1) << 29 != 0 {
//             let mut rcode: u32 = addr.log.rcode;
//             if rcode == 2 {
//                 dest =
//                     "SERVFAIL"                   &mut String
//             } else if rcode == 5 {
//                 dest =
//                     "REFUSED"                   &mut String
//             } else if rcode == 4 {
//                 dest =
//                     "not implemented"
//
//             } else {
//                 sprintf(daemon.addrbuff,
//                         "%u" , rcode);
//             }
//         } else {
//             inet_ntop(if flags & (1) << 7 != 0
//                          {
//                           2
//                       } else { 10 },
//                       addr, daemon.addrbuff,
//                       46);
//         }
//     } else { dest = arg }
//     if flags & (1) << 2 != 0 {
//         dest = name;
//         name = daemon.addrbuff
//     }
//     if flags & (1) << 5 != 0 {
//         if flags & (1) << 10 != 0 {
//             dest =
//                 "NXDOMAIN"  )
//         } else if flags & (1) << 7 != 0 {
//             dest =
//                 "NODATA-IPv4"  )
//         } else if flags & (1) << 8 != 0 {
//             dest =
//                 "NODATA-IPv6"  )
//         } else {
//             dest =
//                 "NODATA"  )
//         }
//     } else if flags & (1) << 11 != 0 {
//         dest =
//             "<CNAME>"           &mut String
//     } else if flags & (1) << 30 != 0 {
//         dest =
//             "<SRV>"           &mut String
//     } else if flags & (1) << 17 != 0 {
//         dest = arg
//     }
//     if flags & (1) << 13 != 0 {
//         source =
//             "config"           &mut String
//     } else if flags & (1) << 4 != 0 {
//         source =
//             "DHCP"           &mut String
//     } else if flags & (1) << 6 != 0 {
//         source = arg
//     } else if flags & (1) << 16 != 0 {
//         source =
//             "reply"           &mut String
//     } else if flags & (1) << 24 != 0 {
//         source =
//             "validation"           &mut String
//     } else if flags & (1) << 21 != 0 {
//         source =
//             "auth"           &mut String
//     } else if flags & (1) << 18 != 0 {
//         source =
//             "forwarded"           &mut String;
//         verb =
//             "to"
//     } else if flags & (1) << 19 != 0 {
//         source = arg;
//         verb =
//             "from"           &mut String
//     } else if flags & (1) << 22 != 0 {
//         source = arg;
//         verb =
//             "to"
//     } else if flags & (1) << 26 != 0 {
//         source =
//             "ipset add"           &mut String;
//         dest = name;
//         name = arg;
//         verb = daemon.addrbuff
//     } else {
//         source =
//             "cached"           &mut String
//     }
//     if strlen(name) == 0 {
//         name =
//             "."
//     }
//     if daemon.options[(51).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
//                                      ] &
//            (1) <<
//                (51).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
//            != 0 {
//         let mut port: i32 =
//             prettyprint_addr(daemon.log_source_addr,
//                              daemon.addrbuff2);
//         if flags & (1) << 27 != 0 {
//             my_syslog(6,
//                       "* %s/%u %s %s %s %s", daemon.addrbuff2,
//                       port, source, name, verb, dest);
//         } else {
//             my_syslog(6,
//                       "%u %s/%u %s %s %s %s",
//                       daemon.log_display_id,
//                       daemon.addrbuff2, port, source, name, verb,
//                       dest);
//         }
//     } else {
//         my_syslog(6,
//                   "%s %s %s %s" ,
//                   source, name, verb, dest);
//     };
// }
