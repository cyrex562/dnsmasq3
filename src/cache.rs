
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
static mut cache_head: *mut crec = 0 as *const crec as *mut crec;
static mut cache_tail: *mut crec = 0 as *const crec as *mut crec;
static mut hash_table: *mut *mut crec =
    0 as *const *mut crec as *mut *mut crec;
static mut dhcp_spare: *mut crec = 0 as *const crec as *mut crec;
static mut new_chain: *mut crec = 0 as *const crec as *mut crec;
static mut insert_error: libc::c_int = 0;
static mut big_free: *mut bigname = 0 as *const bigname as *mut bigname;
static mut bignames_left: libc::c_int = 0;
static mut hash_size: libc::c_int = 0;
static mut typestr: [C2RustUnnamed_10; 40] =
    [{
         let mut init =
             C2RustUnnamed_10{type_0: 1 as libc::c_int as libc::c_uint,
                              name:
                                  b"A\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 2 as libc::c_int as libc::c_uint,
                              name:
                                  b"NS\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 5 as libc::c_int as libc::c_uint,
                              name:
                                  b"CNAME\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 6 as libc::c_int as libc::c_uint,
                              name:
                                  b"SOA\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 10 as libc::c_int as libc::c_uint,
                              name:
                                  b"NULL\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 11 as libc::c_int as libc::c_uint,
                              name:
                                  b"WKS\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 12 as libc::c_int as libc::c_uint,
                              name:
                                  b"PTR\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 13 as libc::c_int as libc::c_uint,
                              name:
                                  b"HINFO\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 15 as libc::c_int as libc::c_uint,
                              name:
                                  b"MX\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 16 as libc::c_int as libc::c_uint,
                              name:
                                  b"TXT\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 22 as libc::c_int as libc::c_uint,
                              name:
                                  b"NSAP\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 23 as libc::c_int as libc::c_uint,
                              name:
                                  b"NSAP_PTR\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 24 as libc::c_int as libc::c_uint,
                              name:
                                  b"SIG\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 25 as libc::c_int as libc::c_uint,
                              name:
                                  b"KEY\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 28 as libc::c_int as libc::c_uint,
                              name:
                                  b"AAAA\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 29 as libc::c_int as libc::c_uint,
                              name:
                                  b"LOC\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 33 as libc::c_int as libc::c_uint,
                              name:
                                  b"SRV\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 35 as libc::c_int as libc::c_uint,
                              name:
                                  b"NAPTR\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 36 as libc::c_int as libc::c_uint,
                              name:
                                  b"KX\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 37 as libc::c_int as libc::c_uint,
                              name:
                                  b"CERT\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 38 as libc::c_int as libc::c_uint,
                              name:
                                  b"A6\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 39 as libc::c_int as libc::c_uint,
                              name:
                                  b"DNAME\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 41 as libc::c_int as libc::c_uint,
                              name:
                                  b"OPT\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 43 as libc::c_int as libc::c_uint,
                              name:
                                  b"DS\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 46 as libc::c_int as libc::c_uint,
                              name:
                                  b"RRSIG\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 47 as libc::c_int as libc::c_uint,
                              name:
                                  b"NSEC\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 48 as libc::c_int as libc::c_uint,
                              name:
                                  b"DNSKEY\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 50 as libc::c_int as libc::c_uint,
                              name:
                                  b"NSEC3\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 51 as libc::c_int as libc::c_uint,
                              name:
                                  b"NSEC3PARAM\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 52 as libc::c_int as libc::c_uint,
                              name:
                                  b"TLSA\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 53 as libc::c_int as libc::c_uint,
                              name:
                                  b"SMIMEA\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 55 as libc::c_int as libc::c_uint,
                              name:
                                  b"HIP\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 249 as libc::c_int as libc::c_uint,
                              name:
                                  b"TKEY\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 250 as libc::c_int as libc::c_uint,
                              name:
                                  b"TSIG\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 251 as libc::c_int as libc::c_uint,
                              name:
                                  b"IXFR\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 252 as libc::c_int as libc::c_uint,
                              name:
                                  b"AXFR\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 253 as libc::c_int as libc::c_uint,
                              name:
                                  b"MAILB\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 254 as libc::c_int as libc::c_uint,
                              name:
                                  b"MAILA\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 255 as libc::c_int as libc::c_uint,
                              name:
                                  b"ANY\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_10{type_0: 257 as libc::c_int as libc::c_uint,
                              name:
                                  b"CAA\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn next_uid(mut crecp: *mut crec) {
    static mut uid: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*crecp).uid == 0 as libc::c_int as libc::c_uint {
        uid = uid.wrapping_add(1);
        /* uid == 0 used to indicate CNAME to interface name. */
        if uid == 0 as libc::c_int as libc::c_uint {
            uid = uid.wrapping_add(1)
        }
        (*crecp).uid = uid
    };
}
#[no_mangle]
pub unsafe extern "C" fn cache_init() {
    let mut crecp: *mut crec = 0 as *mut crec;
    let mut i: libc::c_int = 0;
    bignames_left = (*dnsmasq_daemon).cachesize / 10 as libc::c_int;
    if (*dnsmasq_daemon).cachesize > 0 as libc::c_int {
        crecp =
            safe_malloc(((*dnsmasq_daemon).cachesize as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<crec>()
                                                             as
                                                             libc::c_ulong))
                as *mut crec;
        i = 0 as libc::c_int;
        while i < (*dnsmasq_daemon).cachesize {
            cache_link(crecp);
            (*crecp).flags = 0 as libc::c_int as libc::c_uint;
            (*crecp).uid = 0 as libc::c_int as libc::c_uint;
            i += 1;
            crecp = crecp.offset(1)
        }
    }
    /* create initial hash table*/
    rehash((*dnsmasq_daemon).cachesize);
}
/* In most cases, we create the hash table once here by calling this with (hash_table == NULL)
   but if the hosts file(s) are big (some people have 50000 ad-block entries), the table
   will be much too small, so the hosts reading code calls rehash every 1000 addresses, to
   expand the table. */
unsafe extern "C" fn rehash(mut size: libc::c_int) {
    let mut new: *mut *mut crec = 0 as *mut *mut crec;
    let mut old: *mut *mut crec = 0 as *mut *mut crec;
    let mut p: *mut crec = 0 as *mut crec;
    let mut tmp: *mut crec = 0 as *mut crec;
    let mut i: libc::c_int = 0;
    let mut new_size: libc::c_int = 0;
    let mut old_size: libc::c_int = 0;
    /* hash_size is a power of two. */
    new_size = 64 as libc::c_int;
    while new_size < size / 10 as libc::c_int {
        new_size = new_size << 1 as libc::c_int
    }
    /* must succeed in getting first instance, failure later is non-fatal */
    if hash_table.is_null() {
        new =
            safe_malloc((new_size as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut crec>()
                                                             as
                                                             libc::c_ulong))
                as *mut *mut crec
    } else if new_size <= hash_size ||
                  {
                      new =
                          whine_malloc((new_size as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut crec>()
                                                                            as
                                                                            libc::c_ulong))
                              as
                              *mut *mut crec; /* Barker code - minimum self-correlation in cyclic shift */
                      new.is_null()
                  } {
        return
    }
    i = 0 as libc::c_int;
    while i < new_size {
        let ref mut fresh6 = *new.offset(i as isize);
        *fresh6 = 0 as *mut crec;
        i += 1
    }
    old = hash_table;
    old_size = hash_size;
    hash_table = new;
    hash_size = new_size;
    if !old.is_null() {
        i = 0 as libc::c_int;
        while i < old_size {
            p = *old.offset(i as isize);
            while !p.is_null() {
                tmp = (*p).hash_next;
                cache_hash(p);
                p = tmp
            }
            i += 1
        }
        free(old as *mut libc::c_void);
    };
}
unsafe extern "C" fn hash_bucket(mut name: *mut libc::c_char)
 -> *mut *mut crec {
    let mut c: libc::c_uint = 0;
    let mut val: libc::c_uint = 0o17465 as libc::c_int as libc::c_uint;
    let mut mix_tab: *const libc::c_uchar =
        typestr.as_ptr() as *const libc::c_uchar;
    loop  {
        let fresh7 = name;
        name = name.offset(1);
        c = *fresh7 as libc::c_uchar as libc::c_uint;
        if !(c != 0) { break ; }
        /* don't use tolower and friends here - they may be messed up by LOCALE */
        if c >= 'A' as i32 as libc::c_uint && c <= 'Z' as i32 as libc::c_uint
           {
            c = c.wrapping_add(('a' as i32 - 'A' as i32) as libc::c_uint)
        }
        val =
            (val << 7 as libc::c_int |
                 val >>
                     32 as libc::c_int -
                         7 as
                             libc::c_int).wrapping_add(*mix_tab.offset((val.wrapping_add(c)
                                                                            &
                                                                            0x3f
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                                           as
                                                                           isize)
                                                           as libc::c_uint ^
                                                           c)
    }
    /* hash_size is a power of two */
    return hash_table.offset(((val ^ val >> 16 as libc::c_int) &
                                  (hash_size - 1 as libc::c_int) as
                                      libc::c_uint) as isize);
}
unsafe extern "C" fn cache_hash(mut crecp: *mut crec) {
    /* maintain an invariant that all entries with F_REVERSE set
     are at the start of the hash-chain  and all non-reverse
     immortal entries are at the end of the hash-chain.
     This allows reverse searches and garbage collection to be optimised */
    let mut up: *mut *mut crec =
        hash_bucket(cache_get_name(crecp)); /* invalidate CNAMES pointing to this. */
    if (*crecp).flags & (1 as libc::c_uint) << 2 as libc::c_int == 0 {
        while !(*up).is_null() &&
                  (**up).flags & (1 as libc::c_uint) << 2 as libc::c_int != 0
              {
            up = &mut (**up).hash_next
        }
        if (*crecp).flags & (1 as libc::c_uint) << 0 as libc::c_int != 0 {
            while !(*up).is_null() &&
                      (**up).flags & (1 as libc::c_uint) << 0 as libc::c_int
                          == 0 {
                up = &mut (**up).hash_next
            }
        }
    }
    (*crecp).hash_next = *up;
    *up = crecp;
}
unsafe extern "C" fn cache_blockdata_free(mut crecp: *mut crec) {
    if (*crecp).flags & (1 as libc::c_uint) << 5 as libc::c_int == 0 {
        if (*crecp).flags & (1 as libc::c_uint) << 30 as libc::c_int != 0 {
            blockdata_free((*crecp).addr.srv.target);
        }
    };
}
unsafe extern "C" fn cache_free(mut crecp: *mut crec) {
    (*crecp).flags &= !((1 as libc::c_uint) << 3 as libc::c_int);
    (*crecp).flags &= !((1 as libc::c_uint) << 2 as libc::c_int);
    (*crecp).uid = 0 as libc::c_int as libc::c_uint;
    if !cache_tail.is_null() {
        (*cache_tail).next = crecp
    } else { cache_head = crecp }
    (*crecp).prev = cache_tail;
    (*crecp).next = 0 as *mut crec;
    cache_tail = crecp;
    /* retrieve big name for further use. */
    if (*crecp).flags & (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        (*(*crecp).name.bname).next = big_free;
        big_free = (*crecp).name.bname;
        (*crecp).flags &= !((1 as libc::c_uint) << 9 as libc::c_int)
    }
    cache_blockdata_free(crecp);
}
/* insert a new cache entry at the head of the list (youngest entry) */
unsafe extern "C" fn cache_link(mut crecp: *mut crec) {
    if !cache_head.is_null() {
        /* check needed for init code */
        (*cache_head).prev = crecp
    }
    (*crecp).next = cache_head;
    (*crecp).prev = 0 as *mut crec;
    cache_head = crecp;
    if cache_tail.is_null() { cache_tail = crecp };
}
/* remove an arbitrary cache entry for promotion */
unsafe extern "C" fn cache_unlink(mut crecp: *mut crec) {
    if !(*crecp).prev.is_null() {
        (*(*crecp).prev).next = (*crecp).next
    } else { cache_head = (*crecp).next }
    if !(*crecp).next.is_null() {
        (*(*crecp).next).prev = (*crecp).prev
    } else { cache_tail = (*crecp).prev };
}
#[no_mangle]
pub unsafe extern "C" fn cache_get_name(mut crecp: *mut crec)
 -> *mut libc::c_char {
    if (*crecp).flags & (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        return (*(*crecp).name.bname).name.as_mut_ptr()
    } else {
        if (*crecp).flags & (1 as libc::c_uint) << 1 as libc::c_int != 0 {
            return (*crecp).name.namep
        }
    }
    return (*crecp).name.sname.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn cache_get_cname_target(mut crecp: *mut crec)
 -> *mut libc::c_char {
    if (*crecp).addr.cname.is_name_ptr != 0 {
        return (*crecp).addr.cname.target.name
    } else { return cache_get_name((*crecp).addr.cname.target.cache) };
}
#[no_mangle]
pub unsafe extern "C" fn cache_enumerate(mut init: libc::c_int) -> *mut crec {
    static mut bucket: libc::c_int = 0;
    static mut cache: *mut crec = 0 as *const crec as *mut crec;
    if init != 0 {
        bucket = 0 as libc::c_int;
        cache = 0 as *mut crec
    } else if !cache.is_null() && !(*cache).hash_next.is_null() {
        cache = (*cache).hash_next
    } else {
        cache = 0 as *mut crec;
        while bucket < hash_size {
            let fresh8 = bucket;
            bucket = bucket + 1;
            cache = *hash_table.offset(fresh8 as isize);
            if !cache.is_null() { break ; }
        }
    }
    return cache;
}
unsafe extern "C" fn is_outdated_cname_pointer(mut crecp: *mut crec)
 -> libc::c_int {
    if (*crecp).flags & (1 as libc::c_uint) << 11 as libc::c_int == 0 ||
           (*crecp).addr.cname.is_name_ptr != 0 {
        return 0 as libc::c_int
    }
    /* NB. record may be reused as DS or DNSKEY, where uid is 
     overloaded for something completely different */
    if !(*crecp).addr.cname.target.cache.is_null() &&
           (*(*crecp).addr.cname.target.cache).flags &
               ((1 as libc::c_uint) << 12 as libc::c_int |
                    (1 as libc::c_uint) << 14 as libc::c_int) == 0 &&
           (*crecp).addr.cname.uid == (*(*crecp).addr.cname.target.cache).uid
       {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_expired(mut now: time_t, mut crecp: *mut crec)
 -> libc::c_int {
    if (*crecp).flags & (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    if difftime(now, (*crecp).ttd) < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn cache_scan_free(mut name: *mut libc::c_char,
                                     mut addr: *mut all_addr,
                                     mut class: libc::c_ushort,
                                     mut now: time_t, mut flags: libc::c_uint,
                                     mut target_crec: *mut *mut crec,
                                     mut target_uid: *mut libc::c_uint)
 -> *mut crec {
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
    let mut crecp: *mut crec = 0 as *mut crec;
    let mut up: *mut *mut crec = 0 as *mut *mut crec;
    if flags & (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        let mut current_block_18: u64;
        up = hash_bucket(name);
        crecp = *up;
        while !crecp.is_null() {
            if (*crecp).flags & (1 as libc::c_uint) << 3 as libc::c_int != 0
                   && hostname_isequal(cache_get_name(crecp), name) != 0 {
                /* Don't delete DNSSEC in favour of a CNAME, they can co-exist */
                if flags & (*crecp).flags &
                       ((1 as libc::c_uint) << 7 as libc::c_int |
                            (1 as libc::c_uint) << 8 as libc::c_int |
                            (1 as libc::c_uint) << 30 as libc::c_int) != 0 ||
                       ((*crecp).flags | flags) &
                           (1 as libc::c_uint) << 11 as libc::c_int != 0 &&
                           (*crecp).flags &
                               ((1 as libc::c_uint) << 12 as libc::c_int |
                                    (1 as libc::c_uint) << 14 as libc::c_int)
                               == 0 {
                    if (*crecp).flags &
                           ((1 as libc::c_uint) << 6 as libc::c_int |
                                (1 as libc::c_uint) << 4 as libc::c_int |
                                (1 as libc::c_uint) << 13 as libc::c_int) != 0
                       {
                        return crecp
                    }
                    *up = (*crecp).hash_next;
                    /* If this record is for the name we're inserting and is the target
		     of a CNAME record. Make the new record for the same name, in the same
		     crec, with the same uid to avoid breaking the existing CNAME. */
                    if (*crecp).uid != 0 as libc::c_int as libc::c_uint {
                        if !target_crec.is_null() { *target_crec = crecp }
                        if !target_uid.is_null() {
                            *target_uid = (*crecp).uid
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
                        *up = (*crecp).hash_next;
                        if (*crecp).flags &
                               ((1 as libc::c_uint) << 6 as libc::c_int |
                                    (1 as libc::c_uint) << 4 as libc::c_int |
                                    (1 as libc::c_uint) << 13 as libc::c_int)
                               == 0 {
                            cache_unlink(crecp);
                            cache_free(crecp);
                        }
                    } else { up = &mut (*crecp).hash_next }
                }
                _ => { }
            }
            crecp = (*crecp).hash_next
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut addrlen: libc::c_int =
            if flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                16 as libc::c_int
            } else { 4 as libc::c_int };
        i = 0 as libc::c_int;
        while i < hash_size {
            crecp = *hash_table.offset(i as isize);
            up = &mut *hash_table.offset(i as isize) as *mut *mut crec;
            while !crecp.is_null() &&
                      ((*crecp).flags &
                           (1 as libc::c_uint) << 2 as libc::c_int != 0 ||
                           (*crecp).flags &
                               (1 as libc::c_uint) << 0 as libc::c_int == 0) {
                if is_expired(now, crecp) != 0 {
                    *up = (*crecp).hash_next;
                    if (*crecp).flags &
                           ((1 as libc::c_uint) << 6 as libc::c_int |
                                (1 as libc::c_uint) << 4 as libc::c_int |
                                (1 as libc::c_uint) << 13 as libc::c_int) == 0
                       {
                        cache_unlink(crecp);
                        cache_free(crecp);
                    }
                } else if (*crecp).flags &
                              ((1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 13 as libc::c_int)
                              == 0 &&
                              flags & (*crecp).flags &
                                  (1 as libc::c_uint) << 2 as libc::c_int != 0
                              &&
                              flags & (*crecp).flags &
                                  ((1 as libc::c_uint) << 7 as libc::c_int |
                                       (1 as libc::c_uint) <<
                                           8 as libc::c_int) != 0 &&
                              memcmp(&mut (*crecp).addr as *mut all_addr as
                                         *const libc::c_void,
                                     addr as *const libc::c_void,
                                     addrlen as libc::c_ulong) ==
                                  0 as libc::c_int {
                    *up = (*crecp).hash_next;
                    cache_unlink(crecp);
                    cache_free(crecp);
                } else { up = &mut (*crecp).hash_next }
                crecp = (*crecp).hash_next
            }
            i += 1
        }
    }
    return 0 as *mut crec;
}
/* Note: The normal calling sequence is
   cache_start_insert
   cache_insert * n
   cache_end_insert

   but an abort can cause the cache_end_insert to be missed 
   in which can the next cache_start_insert cleans things up. */
#[no_mangle]
pub unsafe extern "C" fn cache_start_insert() {
    /* Free any entries which didn't get committed during the last
     insert due to error.
  */
    while !new_chain.is_null() {
        let mut tmp: *mut crec = (*new_chain).next;
        cache_free(new_chain);
        new_chain = tmp
    }
    new_chain = 0 as *mut crec;
    insert_error = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cache_insert(mut name: *mut libc::c_char,
                                      mut addr: *mut all_addr,
                                      mut class: libc::c_ushort,
                                      mut now: time_t, mut ttl: libc::c_ulong,
                                      mut flags: libc::c_uint) -> *mut crec {
    /* Don't log DNSSEC records here, done elsewhere */
    log_query(flags | (1 as libc::c_uint) << 16 as libc::c_int, name, addr,
              0 as *mut libc::c_char);
    if (*dnsmasq_daemon).max_cache_ttl != 0 as libc::c_int as libc::c_ulong &&
           (*dnsmasq_daemon).max_cache_ttl < ttl {
        ttl = (*dnsmasq_daemon).max_cache_ttl
    }
    if (*dnsmasq_daemon).min_cache_ttl != 0 as libc::c_int as libc::c_ulong &&
           (*dnsmasq_daemon).min_cache_ttl > ttl {
        ttl = (*dnsmasq_daemon).min_cache_ttl
    }
    return really_insert(name, addr, class, now, ttl, flags);
}
unsafe extern "C" fn really_insert(mut name: *mut libc::c_char,
                                   mut addr: *mut all_addr,
                                   mut class: libc::c_ushort, mut now: time_t,
                                   mut ttl: libc::c_ulong,
                                   mut flags: libc::c_uint) -> *mut crec {
    let mut new: *mut crec = 0 as *mut crec;
    let mut target_crec: *mut crec = 0 as *mut crec;
    let mut big_name: *mut bigname = 0 as *mut bigname;
    let mut freed_all: libc::c_int =
        (flags & (1 as libc::c_uint) << 2 as libc::c_int) as libc::c_int;
    let mut free_avail: libc::c_int = 0 as libc::c_int;
    let mut target_uid: libc::c_uint = 0;
    /* if previous insertion failed give up now. */
    if insert_error != 0 { return 0 as *mut crec }
    /* we don't cache zero-TTL records. */
    if ttl == 0 as libc::c_int as libc::c_ulong {
        insert_error = 1 as libc::c_int;
        return 0 as *mut crec
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
               ((1 as libc::c_uint) << 7 as libc::c_int |
                    (1 as libc::c_uint) << 8 as libc::c_int) != 0 &&
               flags & (1 as libc::c_uint) << 3 as libc::c_int != 0 &&
               !addr.is_null() {
            if flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 &&
                   (*new).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0
                   && (*new).addr.addr4.s_addr == (*addr).addr4.s_addr {
                return new
            } else {
                if flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 &&
                       (*new).flags & (1 as libc::c_uint) << 8 as libc::c_int
                           != 0 &&
                       ({
                            let mut __a: *const in6_addr =
                                &mut (*new).addr.addr6 as *mut in6_addr as
                                    *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*addr).addr6 as *mut in6_addr as
                                    *const in6_addr;
                            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] &&
                                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[1 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[2 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[3 as
                                                                    libc::c_int
                                                                    as usize])
                                as libc::c_int
                        }) != 0 {
                    return new
                }
            }
        }
        insert_error = 1 as libc::c_int;
        return 0 as *mut crec
    }
    /* Now get a cache entry from the end of the LRU list */
    if target_crec.is_null() {
        loop  {
            new = cache_tail;
            if new.is_null() {
                /* no entries left - cache is too small, bail */
                insert_error = 1 as libc::c_int;
                return 0 as *mut crec
            }
            /* Free entry at end of LRU list, use it. */
            if (*new).flags &
                   ((1 as libc::c_uint) << 3 as libc::c_int |
                        (1 as libc::c_uint) << 2 as libc::c_int) == 0 {
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
                static mut warned: libc::c_int = 0 as libc::c_int;
                if warned == 0 {
                    my_syslog(3 as libc::c_int,
                              b"Internal error in cache.\x00" as *const u8 as
                                  *const libc::c_char);
                    warned = 1 as libc::c_int
                }
                insert_error = 1 as libc::c_int;
                return 0 as *mut crec
            }
            if freed_all != 0 {
                /* For DNSSEC records, uid holds class. */
                free_avail = 1 as libc::c_int; /* Must be free space now. */
                cache_scan_free(cache_get_name(new), &mut (*new).addr,
                                (*new).uid as libc::c_ushort, now,
                                (*new).flags, 0 as *mut *mut crec,
                                0 as *mut libc::c_uint);
                (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_LIVE_FREED as
                                              libc::c_int as usize] =
                    (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_LIVE_FREED as
                                                  libc::c_int as
                                                  usize].wrapping_add(1)
            } else {
                cache_scan_free(0 as *mut libc::c_char, 0 as *mut all_addr,
                                class, now, 0 as libc::c_int as libc::c_uint,
                                0 as *mut *mut crec, 0 as *mut libc::c_uint);
                freed_all = 1 as libc::c_int
            }
        }
    }
    /* Check if we need to and can allocate extra memory for a long name.
     If that fails, give up now, always succeed for DNSSEC records. */
    if !name.is_null() &&
           strlen(name) >
               (50 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        if !big_free.is_null() {
            big_name = big_free;
            big_free = (*big_free).next
        } else if bignames_left == 0 as libc::c_int &&
                      flags &
                          ((1 as libc::c_uint) << 14 as libc::c_int |
                               (1 as libc::c_uint) << 12 as libc::c_int) == 0
                      ||
                      {
                          big_name =
                              whine_malloc(::std::mem::size_of::<bigname>() as
                                               libc::c_ulong) as *mut bigname;
                          big_name.is_null()
                      } {
            insert_error = 1 as libc::c_int;
            return 0 as *mut crec
        } else { if bignames_left != 0 as libc::c_int { bignames_left -= 1 } }
    }
    /* If we freed a cache entry for our name which was a CNAME target, use that.
     and preserve the uid, so that existing CNAMES are not broken. */
    if !target_crec.is_null() { new = target_crec; (*new).uid = target_uid }
    /* Got the rest: finally grab entry. */
    cache_unlink(new);
    (*new).flags = flags;
    if !big_name.is_null() {
        (*new).name.bname = big_name;
        (*new).flags |= (1 as libc::c_uint) << 9 as libc::c_int
    }
    if !name.is_null() {
        strcpy(cache_get_name(new), name);
    } else { *cache_get_name(new) = 0 as libc::c_int as libc::c_char }
    if !addr.is_null() { (*new).addr = *addr }
    (*new).ttd = now + ttl as time_t;
    (*new).next = new_chain;
    new_chain = new;
    return new;
}
/* after end of insertion, commit the new entries */
#[no_mangle]
pub unsafe extern "C" fn cache_end_insert() {
    if insert_error != 0 { return }
    while !new_chain.is_null() {
        let mut tmp: *mut crec = (*new_chain).next;
        /* drop CNAMEs which didn't find a target. */
        if is_outdated_cname_pointer(new_chain) != 0 {
            cache_free(new_chain);
        } else {
            cache_hash(new_chain);
            cache_link(new_chain);
            (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_INSERTED as libc::c_int
                                          as usize] =
                (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_INSERTED as
                                              libc::c_int as
                                              usize].wrapping_add(1);
            /* If we're a child process, send this cache entry up the pipe to the master.
	     The marshalling process is rather nasty. */
            if (*dnsmasq_daemon).pipe_to_parent != -(1 as libc::c_int) {
                let mut name: *mut libc::c_char = cache_get_name(new_chain);
                let mut m: ssize_t = strlen(name) as ssize_t;
                let mut flags: libc::c_uint = (*new_chain).flags;
                read_write((*dnsmasq_daemon).pipe_to_parent,
                           &mut m as *mut ssize_t as *mut libc::c_uchar,
                           ::std::mem::size_of::<ssize_t>() as libc::c_ulong
                               as libc::c_int, 0 as libc::c_int);
                read_write((*dnsmasq_daemon).pipe_to_parent,
                           name as *mut libc::c_uchar, m as libc::c_int,
                           0 as libc::c_int);
                read_write((*dnsmasq_daemon).pipe_to_parent,
                           &mut (*new_chain).ttd as *mut time_t as
                               *mut libc::c_uchar,
                           ::std::mem::size_of::<time_t>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int);
                read_write((*dnsmasq_daemon).pipe_to_parent,
                           &mut flags as *mut libc::c_uint as
                               *mut libc::c_uchar,
                           ::std::mem::size_of::<libc::c_uint>() as
                               libc::c_ulong as libc::c_int,
                           0 as libc::c_int);
                if flags &
                       ((1 as libc::c_uint) << 7 as libc::c_int |
                            (1 as libc::c_uint) << 8 as libc::c_int |
                            (1 as libc::c_uint) << 12 as libc::c_int |
                            (1 as libc::c_uint) << 14 as libc::c_int |
                            (1 as libc::c_uint) << 30 as libc::c_int) != 0 {
                    read_write((*dnsmasq_daemon).pipe_to_parent,
                               &mut (*new_chain).addr as *mut all_addr as
                                   *mut libc::c_uchar,
                               ::std::mem::size_of::<all_addr>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int);
                }
                if flags & (1 as libc::c_uint) << 30 as libc::c_int != 0 {
                    /* A negative SRV entry is possible and has no data, obviously. */
                    if flags & (1 as libc::c_uint) << 5 as libc::c_int == 0 {
                        blockdata_write((*new_chain).addr.srv.target,
                                        (*new_chain).addr.srv.targetlen as
                                            size_t,
                                        (*dnsmasq_daemon).pipe_to_parent);
                    }
                }
            }
        }
        new_chain = tmp
    }
    /* signal end of cache insert in master process */
    if (*dnsmasq_daemon).pipe_to_parent != -(1 as libc::c_int) {
        let mut m_0: ssize_t = -(1 as libc::c_int) as ssize_t;
        read_write((*dnsmasq_daemon).pipe_to_parent,
                   &mut m_0 as *mut ssize_t as *mut libc::c_uchar,
                   ::std::mem::size_of::<ssize_t>() as libc::c_ulong as
                       libc::c_int, 0 as libc::c_int);
    }
    new_chain = 0 as *mut crec;
}
/* A marshalled cache entry arrives on fd, read, unmarshall and insert into cache of master process. */
#[no_mangle]
pub unsafe extern "C" fn cache_recv_insert(mut now: time_t,
                                           mut fd: libc::c_int)
 -> libc::c_int {
    let mut m: ssize_t = 0;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut ttl: libc::c_ulong = 0;
    let mut ttd: time_t = 0;
    let mut flags: libc::c_uint = 0;
    let mut crecp: *mut crec = 0 as *mut crec;
    cache_start_insert();
    loop  {
        if read_write(fd, &mut m as *mut ssize_t as *mut libc::c_uchar,
                      ::std::mem::size_of::<ssize_t>() as libc::c_ulong as
                          libc::c_int, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        if m == -(1 as libc::c_int) as libc::c_long {
            cache_end_insert();
            return 1 as libc::c_int
        }
        if read_write(fd, (*dnsmasq_daemon).namebuff as *mut libc::c_uchar,
                      m as libc::c_int, 1 as libc::c_int) == 0 ||
               read_write(fd, &mut ttd as *mut time_t as *mut libc::c_uchar,
                          ::std::mem::size_of::<time_t>() as libc::c_ulong as
                              libc::c_int, 1 as libc::c_int) == 0 ||
               read_write(fd,
                          &mut flags as *mut libc::c_uint as
                              *mut libc::c_uchar,
                          ::std::mem::size_of::<libc::c_uint>() as
                              libc::c_ulong as libc::c_int, 1 as libc::c_int)
                   == 0 {
            return 0 as libc::c_int
        }
        *(*dnsmasq_daemon).namebuff.offset(m as isize) =
            0 as libc::c_int as libc::c_char;
        ttl = difftime(ttd, now) as libc::c_ulong;
        if flags &
               ((1 as libc::c_uint) << 7 as libc::c_int |
                    (1 as libc::c_uint) << 8 as libc::c_int |
                    (1 as libc::c_uint) << 12 as libc::c_int |
                    (1 as libc::c_uint) << 14 as libc::c_int |
                    (1 as libc::c_uint) << 30 as libc::c_int) != 0 {
            let mut class: libc::c_ushort =
                1 as libc::c_int as libc::c_ushort;
            if read_write(fd,
                          &mut addr as *mut all_addr as *mut libc::c_uchar,
                          ::std::mem::size_of::<all_addr>() as libc::c_ulong
                              as libc::c_int, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            if flags & (1 as libc::c_uint) << 30 as libc::c_int != 0 &&
                   flags & (1 as libc::c_uint) << 5 as libc::c_int == 0 &&
                   {
                       addr.srv.target =
                           blockdata_read(fd, addr.srv.targetlen as size_t);
                       addr.srv.target.is_null()
                   } {
                return 0 as libc::c_int
            }
            crecp =
                really_insert((*dnsmasq_daemon).namebuff, &mut addr, class,
                              now, ttl, flags)
        } else if flags & (1 as libc::c_uint) << 11 as libc::c_int != 0 {
            let mut newc: *mut crec =
                really_insert((*dnsmasq_daemon).namebuff, 0 as *mut all_addr,
                              1 as libc::c_int as libc::c_ushort, now, ttl,
                              flags);
            /* This relies on the fact that the target of a CNAME immediately precedes
	     it because of the order of extraction in extract_addresses, and
	     the order reversal on the new_chain. */
            if !newc.is_null() {
                (*newc).addr.cname.is_name_ptr = 0 as libc::c_int;
                if crecp.is_null() {
                    (*newc).addr.cname.target.cache = 0 as *mut crec
                } else {
                    next_uid(crecp);
                    (*newc).addr.cname.target.cache = crecp;
                    (*newc).addr.cname.uid = (*crecp).uid
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cache_find_non_terminal(mut name: *mut libc::c_char,
                                                 mut now: time_t)
 -> libc::c_int {
    let mut crecp: *mut crec = 0 as *mut crec;
    crecp = *hash_bucket(name);
    while !crecp.is_null() {
        if is_outdated_cname_pointer(crecp) == 0 &&
               is_expired(now, crecp) == 0 &&
               (*crecp).flags & (1 as libc::c_uint) << 3 as libc::c_int != 0
               &&
               (*crecp).flags & (1 as libc::c_uint) << 10 as libc::c_int == 0
               && hostname_isequal(name, cache_get_name(crecp)) != 0 {
            return 1 as libc::c_int
        }
        crecp = (*crecp).hash_next
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cache_find_by_name(mut crecp: *mut crec,
                                            mut name: *mut libc::c_char,
                                            mut now: time_t,
                                            mut prot: libc::c_uint)
 -> *mut crec {
    let mut ans: *mut crec = 0 as *mut crec;
    let mut no_rr: libc::c_int =
        (prot & (1 as libc::c_uint) << 25 as libc::c_int) as libc::c_int;
    prot &= !((1 as libc::c_uint) << 25 as libc::c_int);
    if !crecp.is_null() {
        /* iterating */
        ans = (*crecp).next
    } else {
        /* first search, look for relevant entries and push to top of list
	 also free anything which has expired */
        let mut next: *mut crec = 0 as *mut crec;
        let mut up: *mut *mut crec = 0 as *mut *mut crec;
        let mut insert: *mut *mut crec = 0 as *mut *mut crec;
        let mut chainp: *mut *mut crec = &mut ans;
        let mut ins_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        up = hash_bucket(name);
        crecp = *up;
        while !crecp.is_null() {
            next = (*crecp).hash_next;
            if is_expired(now, crecp) == 0 &&
                   is_outdated_cname_pointer(crecp) == 0 {
                if (*crecp).flags & (1 as libc::c_uint) << 3 as libc::c_int !=
                       0 && (*crecp).flags & prot != 0 &&
                       hostname_isequal(cache_get_name(crecp), name) != 0 {
                    if (*crecp).flags &
                           ((1 as libc::c_uint) << 6 as libc::c_int |
                                (1 as libc::c_uint) << 4 as libc::c_int |
                                (1 as libc::c_uint) << 13 as libc::c_int) != 0
                       {
                        *chainp = crecp;
                        chainp = &mut (*crecp).next
                    } else { cache_unlink(crecp); cache_link(crecp); }
                    /* Move all but the first entry up the hash chain
		     this implements round-robin. 
		     Make sure that re-ordering doesn't break the hash-chain
		     order invariants. 
		  */
                    if !insert.is_null() &&
                           (*crecp).flags &
                               ((1 as libc::c_uint) << 2 as libc::c_int |
                                    (1 as libc::c_uint) << 0 as libc::c_int)
                               == ins_flags {
                        *up = (*crecp).hash_next;
                        (*crecp).hash_next = *insert;
                        *insert = crecp;
                        insert = &mut (*crecp).hash_next
                    } else {
                        if insert.is_null() && no_rr == 0 {
                            insert = up;
                            ins_flags =
                                (*crecp).flags &
                                    ((1 as libc::c_uint) << 2 as libc::c_int |
                                         (1 as libc::c_uint) <<
                                             0 as libc::c_int)
                        }
                        up = &mut (*crecp).hash_next
                    }
                } else {
                    /* case : not expired, incorrect entry. */
                    up = &mut (*crecp).hash_next
                }
            } else {
                /* expired entry, free it */
                *up = (*crecp).hash_next;
                if (*crecp).flags &
                       ((1 as libc::c_uint) << 6 as libc::c_int |
                            (1 as libc::c_uint) << 4 as libc::c_int |
                            (1 as libc::c_uint) << 13 as libc::c_int) == 0 {
                    cache_unlink(crecp);
                    cache_free(crecp);
                }
            }
            crecp = next
        }
        *chainp = cache_head
    }
    if !ans.is_null() &&
           (*ans).flags & (1 as libc::c_uint) << 3 as libc::c_int != 0 &&
           (*ans).flags & prot != 0 &&
           hostname_isequal(cache_get_name(ans), name) != 0 {
        return ans
    }
    return 0 as *mut crec;
}
#[no_mangle]
pub unsafe extern "C" fn cache_find_by_addr(mut crecp: *mut crec,
                                            mut addr: *mut all_addr,
                                            mut now: time_t,
                                            mut prot: libc::c_uint)
 -> *mut crec {
    let mut ans: *mut crec = 0 as *mut crec;
    let mut addrlen: libc::c_int =
        if prot == (1 as libc::c_uint) << 8 as libc::c_int {
            16 as libc::c_int
        } else { 4 as libc::c_int };
    if !crecp.is_null() {
        /* iterating */
        ans = (*crecp).next
    } else {
        /* first search, look for relevant entries and push to top of list
	 also free anything which has expired. All the reverse entries are at the
	 start of the hash chain, so we can give up when we find the first 
	 non-REVERSE one.  */
        let mut i: libc::c_int = 0;
        let mut up: *mut *mut crec = 0 as *mut *mut crec;
        let mut chainp: *mut *mut crec = &mut ans;
        i = 0 as libc::c_int;
        while i < hash_size {
            crecp = *hash_table.offset(i as isize);
            up = &mut *hash_table.offset(i as isize) as *mut *mut crec;
            while !crecp.is_null() &&
                      (*crecp).flags & (1 as libc::c_uint) << 2 as libc::c_int
                          != 0 {
                if is_expired(now, crecp) == 0 {
                    if (*crecp).flags & prot != 0 &&
                           memcmp(&mut (*crecp).addr as *mut all_addr as
                                      *const libc::c_void,
                                  addr as *const libc::c_void,
                                  addrlen as libc::c_ulong) ==
                               0 as libc::c_int {
                        if (*crecp).flags &
                               ((1 as libc::c_uint) << 6 as libc::c_int |
                                    (1 as libc::c_uint) << 4 as libc::c_int |
                                    (1 as libc::c_uint) << 13 as libc::c_int)
                               != 0 {
                            *chainp = crecp;
                            chainp = &mut (*crecp).next
                        } else { cache_unlink(crecp); cache_link(crecp); }
                    }
                    up = &mut (*crecp).hash_next
                } else {
                    *up = (*crecp).hash_next;
                    if (*crecp).flags &
                           ((1 as libc::c_uint) << 6 as libc::c_int |
                                (1 as libc::c_uint) << 4 as libc::c_int |
                                (1 as libc::c_uint) << 13 as libc::c_int) == 0
                       {
                        cache_unlink(crecp);
                        cache_free(crecp);
                    }
                }
                crecp = (*crecp).hash_next
            }
            i += 1
        }
        *chainp = cache_head
    }
    if !ans.is_null() &&
           (*ans).flags & (1 as libc::c_uint) << 2 as libc::c_int != 0 &&
           (*ans).flags & prot != 0 &&
           memcmp(&mut (*ans).addr as *mut all_addr as *const libc::c_void,
                  addr as *const libc::c_void, addrlen as libc::c_ulong) ==
               0 as libc::c_int {
        return ans
    }
    return 0 as *mut crec;
}
unsafe extern "C" fn add_hosts_entry(mut cache: *mut crec,
                                     mut addr: *mut all_addr,
                                     mut addrlen: libc::c_int,
                                     mut index: libc::c_uint,
                                     mut rhash: *mut *mut crec,
                                     mut hashsz: libc::c_int) {
    let mut lookup: *mut crec =
        cache_find_by_name(0 as *mut crec, cache_get_name(cache),
                           0 as libc::c_int as time_t,
                           (*cache).flags &
                               ((1 as libc::c_uint) << 7 as libc::c_int |
                                    (1 as libc::c_uint) << 8 as libc::c_int));
    let mut i: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    /* Remove duplicates in hosts files. */
    if !lookup.is_null() &&
           (*lookup).flags & (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
           memcmp(&mut (*lookup).addr as *mut all_addr as *const libc::c_void,
                  addr as *const libc::c_void, addrlen as libc::c_ulong) ==
               0 as libc::c_int {
        free(cache as *mut libc::c_void);
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
        j = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int;
        while i < addrlen {
            j =
                j.wrapping_mul(2 as libc::c_int as
                                   libc::c_uint).wrapping_add(*(addr as
                                                                    *mut libc::c_uchar).offset(i
                                                                                                   as
                                                                                                   isize)
                                                                  as
                                                                  libc::c_uint).wrapping_rem(hashsz
                                                                                                 as
                                                                                                 libc::c_uint);
            i += 1
        }
        lookup = *rhash.offset(j as isize);
        while !lookup.is_null() {
            if (*lookup).flags & (*cache).flags &
                   ((1 as libc::c_uint) << 7 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) != 0 &&
                   memcmp(&mut (*lookup).addr as *mut all_addr as
                              *const libc::c_void,
                          addr as *const libc::c_void,
                          addrlen as libc::c_ulong) == 0 as libc::c_int {
                (*cache).flags &= !((1 as libc::c_uint) << 2 as libc::c_int);
                break ;
            } else { lookup = (*lookup).next }
        }
        /* maintain address hash chain, insert new unique address */
        if lookup.is_null() {
            (*cache).next = *rhash.offset(j as isize);
            let ref mut fresh9 = *rhash.offset(j as isize);
            *fresh9 = cache
        }
    } else {
        /* incremental read, lookup in cache */
        lookup =
            cache_find_by_addr(0 as *mut crec, addr,
                               0 as libc::c_int as time_t,
                               (*cache).flags &
                                   ((1 as libc::c_uint) << 7 as libc::c_int |
                                        (1 as libc::c_uint) <<
                                            8 as libc::c_int));
        if !lookup.is_null() &&
               (*lookup).flags & (1 as libc::c_uint) << 6 as libc::c_int != 0
           {
            (*cache).flags &= !((1 as libc::c_uint) << 2 as libc::c_int)
        }
    }
    (*cache).uid = index;
    memcpy(&mut (*cache).addr as *mut all_addr as *mut libc::c_void,
           addr as *const libc::c_void, addrlen as libc::c_ulong);
    cache_hash(cache);
    make_non_terminals(cache);
}
unsafe extern "C" fn eatspace(mut f: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut nl: libc::c_int = 0 as libc::c_int;
    loop  {
        c = getc(f);
        if c == '#' as i32 {
            while c != '\n' as i32 && c != -(1 as libc::c_int) { c = getc(f) }
        }
        if c == -(1 as libc::c_int) { return 1 as libc::c_int }
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            ungetc(c, f);
            return nl
        }
        if c == '\n' as i32 { nl += 1 }
    };
}
unsafe extern "C" fn gettok(mut f: *mut FILE, mut token: *mut libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    loop  {
        c = getc(f);
        if c == -(1 as libc::c_int) {
            return if count == 0 as libc::c_int {
                       -(1 as libc::c_int)
                   } else { 1 as libc::c_int }
        }
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
               || c == '#' as i32 {
            ungetc(c, f);
            return eatspace(f)
        }
        if count < 1025 as libc::c_int - 1 as libc::c_int {
            let fresh10 = count;
            count = count + 1;
            *token.offset(fresh10 as isize) = c as libc::c_char;
            *token.offset(count as isize) = 0 as libc::c_int as libc::c_char
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn read_hostsfile(mut filename: *mut libc::c_char,
                                        mut index: libc::c_uint,
                                        mut cache_size: libc::c_int,
                                        mut rhash: *mut *mut crec,
                                        mut hashsz: libc::c_int)
 -> libc::c_int {
    let mut f: *mut FILE =
        fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    let mut token: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    let mut domain_suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr_count: libc::c_int = 0 as libc::c_int;
    let mut name_count: libc::c_int = cache_size;
    let mut lineno: libc::c_int = 1 as libc::c_int;
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut atnl: libc::c_int = 0;
    let mut addrlen: libc::c_int = 0 as libc::c_int;
    if f.is_null() {
        my_syslog(3 as libc::c_int,
                  b"failed to load names from %s: %s\x00" as *const u8 as
                      *const libc::c_char, filename,
                  strerror(*__errno_location()));
        return cache_size
    }
    lineno += eatspace(f);
    loop  {
        atnl = gettok(f, token);
        if !(atnl != -(1 as libc::c_int)) { break ; }
        if inet_pton(2 as libc::c_int, token,
                     &mut addr as *mut all_addr as *mut libc::c_void) >
               0 as libc::c_int {
            flags =
                (1 as libc::c_uint) << 6 as libc::c_int |
                    (1 as libc::c_uint) << 0 as libc::c_int |
                    (1 as libc::c_uint) << 3 as libc::c_int |
                    (1 as libc::c_uint) << 2 as libc::c_int |
                    (1 as libc::c_uint) << 7 as libc::c_int;
            addrlen = 4 as libc::c_int;
            domain_suffix = get_domain(addr.addr4)
        } else if inet_pton(10 as libc::c_int, token,
                            &mut addr as *mut all_addr as *mut libc::c_void) >
                      0 as libc::c_int {
            flags =
                (1 as libc::c_uint) << 6 as libc::c_int |
                    (1 as libc::c_uint) << 0 as libc::c_int |
                    (1 as libc::c_uint) << 3 as libc::c_int |
                    (1 as libc::c_uint) << 2 as libc::c_int |
                    (1 as libc::c_uint) << 8 as libc::c_int;
            addrlen = 16 as libc::c_int;
            domain_suffix = get_domain6(&mut addr.addr6)
        } else {
            my_syslog(3 as libc::c_int,
                      b"bad address at %s line %d\x00" as *const u8 as
                          *const libc::c_char, filename, lineno);
            while atnl == 0 as libc::c_int { atnl = gettok(f, token) }
            lineno += atnl;
            continue ;
        }
        addr_count += 1;
        /* rehash every 1000 names. */
        if !rhash.is_null() && name_count - cache_size > 1000 as libc::c_int {
            rehash(name_count);
            cache_size = name_count
        }
        while atnl == 0 as libc::c_int {
            let mut cache: *mut crec = 0 as *mut crec;
            let mut fqdn: libc::c_int = 0;
            let mut nomem: libc::c_int = 0;
            let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
            atnl = gettok(f, token);
            if atnl == -(1 as libc::c_int) { break ; }
            fqdn = !strchr(token, '.' as i32).is_null() as libc::c_int;
            canon = canonicalise(token, &mut nomem);
            if !canon.is_null() {
                /* If set, add a version of the name with a default domain appended */
                if (*dnsmasq_daemon).options[(9 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (9 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 && !domain_suffix.is_null() && fqdn == 0 &&
                       {
                           cache =
                               whine_malloc((::std::mem::size_of::<crec>() as
                                                 libc::c_ulong).wrapping_sub(50
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(strlen(canon)).wrapping_add(2
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong).wrapping_add(strlen(domain_suffix)))
                                   as *mut crec;
                           !cache.is_null()
                       } {
                    strcpy((*cache).name.sname.as_mut_ptr(), canon);
                    strcat((*cache).name.sname.as_mut_ptr(),
                           b".\x00" as *const u8 as *const libc::c_char);
                    strcat((*cache).name.sname.as_mut_ptr(), domain_suffix);
                    (*cache).flags = flags;
                    (*cache).ttd = (*dnsmasq_daemon).local_ttl as time_t;
                    add_hosts_entry(cache, &mut addr, addrlen, index, rhash,
                                    hashsz);
                    name_count += 1
                }
                cache =
                    whine_malloc((::std::mem::size_of::<crec>() as
                                      libc::c_ulong).wrapping_sub(50 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_add(strlen(canon)).wrapping_add(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_ulong))
                        as *mut crec;
                if !cache.is_null() {
                    strcpy((*cache).name.sname.as_mut_ptr(), canon);
                    (*cache).flags = flags;
                    (*cache).ttd = (*dnsmasq_daemon).local_ttl as time_t;
                    add_hosts_entry(cache, &mut addr, addrlen, index, rhash,
                                    hashsz);
                    name_count += 1
                }
                free(canon as *mut libc::c_void);
            } else if nomem == 0 {
                my_syslog(3 as libc::c_int,
                          b"bad name at %s line %d\x00" as *const u8 as
                              *const libc::c_char, filename, lineno);
            }
        }
        lineno += atnl
    }
    fclose(f);
    if !rhash.is_null() { rehash(name_count); }
    my_syslog(6 as libc::c_int,
              b"read %s - %d addresses\x00" as *const u8 as
                  *const libc::c_char, filename, addr_count);
    return name_count;
}
#[no_mangle]
pub unsafe extern "C" fn cache_reload() {
    let mut cache: *mut crec = 0 as *mut crec;
    let mut up: *mut *mut crec = 0 as *mut *mut crec;
    let mut tmp: *mut crec = 0 as *mut crec;
    let mut revhashsz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut total_size: libc::c_int = (*dnsmasq_daemon).cachesize;
    let mut ah: *mut hostsfile = 0 as *mut hostsfile;
    let mut hr: *mut host_record = 0 as *mut host_record;
    let mut nl: *mut name_list = 0 as *mut name_list;
    let mut a: *mut cname = 0 as *mut cname;
    let mut lrec: crec =
        crec{next: 0 as *mut crec,
             prev: 0 as *mut crec,
             hash_next: 0 as *mut crec,
             addr: all_addr{addr4: in_addr{s_addr: 0,},},
             ttd: 0,
             uid: 0,
             flags: 0,
             name: C2RustUnnamed_8{sname: [0; 50],},};
    let mut mx: *mut mx_srv_record = 0 as *mut mx_srv_record;
    let mut txt: *mut txt_record = 0 as *mut txt_record;
    let mut intr: *mut interface_name = 0 as *mut interface_name;
    let mut ptr: *mut ptr_record = 0 as *mut ptr_record;
    let mut naptr: *mut naptr = 0 as *mut naptr;
    (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_INSERTED as libc::c_int as
                                  usize] = 0 as libc::c_int as u32_0;
    (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_LIVE_FREED as libc::c_int as
                                  usize] = 0 as libc::c_int as u32_0;
    i = 0 as libc::c_int;
    while i < hash_size {
        cache = *hash_table.offset(i as isize);
        up = &mut *hash_table.offset(i as isize) as *mut *mut crec;
        while !cache.is_null() {
            cache_blockdata_free(cache);
            tmp = (*cache).hash_next;
            if (*cache).flags &
                   ((1 as libc::c_uint) << 6 as libc::c_int |
                        (1 as libc::c_uint) << 13 as libc::c_int) != 0 {
                *up = (*cache).hash_next;
                free(cache as *mut libc::c_void);
            } else if (*cache).flags & (1 as libc::c_uint) << 4 as libc::c_int
                          == 0 {
                *up = (*cache).hash_next;
                if (*cache).flags & (1 as libc::c_uint) << 9 as libc::c_int !=
                       0 {
                    (*(*cache).name.bname).next = big_free;
                    big_free = (*cache).name.bname
                }
                (*cache).flags = 0 as libc::c_int as libc::c_uint
            } else { up = &mut (*cache).hash_next }
            cache = tmp
        }
        i += 1
    }
    /* Add locally-configured CNAMEs to the cache */
    a = (*dnsmasq_daemon).cnames;
    while !a.is_null() {
        if *(*a).alias.offset(1 as libc::c_int as isize) as libc::c_int !=
               '*' as i32 &&
               {
                   cache =
                       whine_malloc((::std::mem::size_of::<crec>() as
                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut libc::c_char>()
                                                                         as
                                                                         libc::c_ulong).wrapping_sub(50
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                           as *mut crec;
                   !cache.is_null()
               } {
            (*cache).flags =
                (1 as libc::c_uint) << 3 as libc::c_int |
                    (1 as libc::c_uint) << 1 as libc::c_int |
                    (1 as libc::c_uint) << 11 as libc::c_int |
                    (1 as libc::c_uint) << 0 as libc::c_int |
                    (1 as libc::c_uint) << 13 as libc::c_int;
            (*cache).ttd = (*a).ttl as time_t;
            (*cache).name.namep = (*a).alias;
            (*cache).addr.cname.target.name = (*a).target;
            (*cache).addr.cname.is_name_ptr = 1 as libc::c_int;
            (*cache).uid = 0 as libc::c_int as libc::c_uint;
            cache_hash(cache);
            make_non_terminals(cache);
        }
        a = (*a).next
    }
    /* borrow the packet buffer for a temporary by-address hash */
    memset((*dnsmasq_daemon).packet as *mut libc::c_void, 0 as libc::c_int,
           (*dnsmasq_daemon).packet_buff_sz as libc::c_ulong);
    revhashsz =
        ((*dnsmasq_daemon).packet_buff_sz as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut crec>() as
                                             libc::c_ulong) as libc::c_int;
    /* we overwrote the buffer... */
    (*dnsmasq_daemon).srv_save = 0 as *mut server;
    /* Do host_records in config. */
    hr = (*dnsmasq_daemon).host_records;
    while !hr.is_null() {
        nl = (*hr).names;
        while !nl.is_null() {
            if (*hr).flags & 2 as libc::c_int != 0 &&
                   {
                       cache =
                           whine_malloc((::std::mem::size_of::<crec>() as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut libc::c_char>()
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(50
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                               as *mut crec;
                       !cache.is_null()
                   } {
                (*cache).name.namep = (*nl).name;
                (*cache).ttd = (*hr).ttl as time_t;
                (*cache).flags =
                    (1 as libc::c_uint) << 6 as libc::c_int |
                        (1 as libc::c_uint) << 0 as libc::c_int |
                        (1 as libc::c_uint) << 3 as libc::c_int |
                        (1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 7 as libc::c_int |
                        (1 as libc::c_uint) << 1 as libc::c_int |
                        (1 as libc::c_uint) << 13 as libc::c_int;
                add_hosts_entry(cache,
                                &mut (*hr).addr as *mut in_addr as
                                    *mut all_addr, 4 as libc::c_int,
                                1 as libc::c_int as libc::c_uint,
                                (*dnsmasq_daemon).packet as *mut *mut crec,
                                revhashsz);
            }
            if (*hr).flags & 1 as libc::c_int != 0 &&
                   {
                       cache =
                           whine_malloc((::std::mem::size_of::<crec>() as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut libc::c_char>()
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(50
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                               as *mut crec;
                       !cache.is_null()
                   } {
                (*cache).name.namep = (*nl).name;
                (*cache).ttd = (*hr).ttl as time_t;
                (*cache).flags =
                    (1 as libc::c_uint) << 6 as libc::c_int |
                        (1 as libc::c_uint) << 0 as libc::c_int |
                        (1 as libc::c_uint) << 3 as libc::c_int |
                        (1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int |
                        (1 as libc::c_uint) << 1 as libc::c_int |
                        (1 as libc::c_uint) << 13 as libc::c_int;
                add_hosts_entry(cache,
                                &mut (*hr).addr6 as *mut in6_addr as
                                    *mut all_addr, 16 as libc::c_int,
                                1 as libc::c_int as libc::c_uint,
                                (*dnsmasq_daemon).packet as *mut *mut crec,
                                revhashsz);
            }
            nl = (*nl).next
        }
        hr = (*hr).next
    }
    if (*dnsmasq_daemon).options[(4 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (4 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 && (*dnsmasq_daemon).addn_hosts.is_null() {
        if (*dnsmasq_daemon).cachesize > 0 as libc::c_int {
            my_syslog(6 as libc::c_int,
                      b"cleared cache\x00" as *const u8 as
                          *const libc::c_char);
        }
    } else {
        if (*dnsmasq_daemon).options[(4 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (4 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            total_size =
                read_hostsfile(b"/etc/hosts\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               2 as libc::c_int as libc::c_uint, total_size,
                               (*dnsmasq_daemon).packet as *mut *mut crec,
                               revhashsz)
        }
        (*dnsmasq_daemon).addn_hosts =
            expand_filelist((*dnsmasq_daemon).addn_hosts);
        ah = (*dnsmasq_daemon).addn_hosts;
        while !ah.is_null() {
            if (*ah).flags & 2 as libc::c_int == 0 {
                total_size =
                    read_hostsfile((*ah).fname, (*ah).index, total_size,
                                   (*dnsmasq_daemon).packet as *mut *mut crec,
                                   revhashsz)
            }
            ah = (*ah).next
        }
    }
    /* Make non-terminal records for all locally-define RRs */
    lrec.flags =
        (1 as libc::c_uint) << 3 as libc::c_int |
            (1 as libc::c_uint) << 13 as libc::c_int |
            (1 as libc::c_uint) << 1 as libc::c_int |
            (1 as libc::c_uint) << 0 as libc::c_int;
    txt = (*dnsmasq_daemon).txt;
    while !txt.is_null() {
        lrec.name.namep = (*txt).name;
        make_non_terminals(&mut lrec);
        txt = (*txt).next
    }
    naptr = (*dnsmasq_daemon).naptr;
    while !naptr.is_null() {
        lrec.name.namep = (*naptr).name;
        make_non_terminals(&mut lrec);
        naptr = (*naptr).next
    }
    mx = (*dnsmasq_daemon).mxnames;
    while !mx.is_null() {
        lrec.name.namep = (*mx).name;
        make_non_terminals(&mut lrec);
        mx = (*mx).next
    }
    intr = (*dnsmasq_daemon).int_names;
    while !intr.is_null() {
        lrec.name.namep = (*intr).name;
        make_non_terminals(&mut lrec);
        intr = (*intr).next
    }
    ptr = (*dnsmasq_daemon).ptr;
    while !ptr.is_null() {
        lrec.name.namep = (*ptr).name;
        make_non_terminals(&mut lrec);
        ptr = (*ptr).next
    }
    set_dynamic_inotify(8 as libc::c_int, total_size,
                        (*dnsmasq_daemon).packet as *mut *mut crec,
                        revhashsz);
}
#[no_mangle]
pub unsafe extern "C" fn a_record_from_hosts(mut name: *mut libc::c_char,
                                             mut now: time_t) -> in_addr {
    let mut crecp: *mut crec = 0 as *mut crec;
    let mut ret: in_addr = in_addr{s_addr: 0,};
    loop  {
        crecp =
            cache_find_by_name(crecp, name, now,
                               (1 as libc::c_uint) << 7 as libc::c_int);
        if crecp.is_null() { break ; }
        if (*crecp).flags & (1 as libc::c_uint) << 6 as libc::c_int != 0 {
            return (*crecp).addr.addr4
        }
    }
    my_syslog((3 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
              b"No IPv4 address found for %s\x00" as *const u8 as
                  *const libc::c_char, name);
    ret.s_addr = 0 as libc::c_int as in_addr_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cache_unhash_dhcp() {
    let mut cache: *mut crec = 0 as *mut crec;
    let mut up: *mut *mut crec = 0 as *mut *mut crec;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < hash_size {
        cache = *hash_table.offset(i as isize);
        up = &mut *hash_table.offset(i as isize) as *mut *mut crec;
        while !cache.is_null() {
            if (*cache).flags & (1 as libc::c_uint) << 4 as libc::c_int != 0 {
                *up = (*cache).hash_next;
                (*cache).next = dhcp_spare;
                dhcp_spare = cache
            } else { up = &mut (*cache).hash_next }
            cache = (*cache).hash_next
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn cache_add_dhcp_entry(mut host_name:
                                                  *mut libc::c_char,
                                              mut prot: libc::c_int,
                                              mut host_address: *mut all_addr,
                                              mut ttd: time_t) {
    let mut crec: *mut crec = 0 as *mut crec;
    let mut fail_crec: *mut crec = 0 as *mut crec;
    let mut flags: libc::c_uint = (1 as libc::c_uint) << 7 as libc::c_int;
    let mut in_hosts: libc::c_int = 0 as libc::c_int;
    let mut addrlen: size_t =
        ::std::mem::size_of::<in_addr>() as libc::c_ulong;
    if prot == 10 as libc::c_int {
        flags = (1 as libc::c_uint) << 8 as libc::c_int;
        addrlen = ::std::mem::size_of::<in6_addr>() as libc::c_ulong
    }
    inet_ntop(prot, host_address as *const libc::c_void,
              (*dnsmasq_daemon).addrbuff, 46 as libc::c_int as socklen_t);
    loop  {
        crec =
            cache_find_by_name(crec, host_name, 0 as libc::c_int as time_t,
                               flags |
                                   (1 as libc::c_uint) << 11 as libc::c_int);
        if crec.is_null() { break ; }
        /* check all addresses associated with name */
        if (*crec).flags &
               ((1 as libc::c_uint) << 6 as libc::c_int |
                    (1 as libc::c_uint) << 13 as libc::c_int) != 0 {
            if (*crec).flags & (1 as libc::c_uint) << 11 as libc::c_int != 0 {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"%s is a CNAME, not giving it to the DHCP lease of %s\x00"
                              as *const u8 as *const libc::c_char, host_name,
                          (*dnsmasq_daemon).addrbuff);
            } else if memcmp(&mut (*crec).addr as *mut all_addr as
                                 *const libc::c_void,
                             host_address as *const libc::c_void, addrlen) ==
                          0 as libc::c_int {
                in_hosts = 1 as libc::c_int
            } else { fail_crec = crec }
        } else {
            if !((*crec).flags & (1 as libc::c_uint) << 4 as libc::c_int == 0)
               {
                continue ;
            }
            cache_scan_free(host_name, 0 as *mut all_addr,
                            1 as libc::c_int as libc::c_ushort,
                            0 as libc::c_int as time_t,
                            (*crec).flags &
                                (flags |
                                     (1 as libc::c_uint) << 11 as libc::c_int
                                     |
                                     (1 as libc::c_uint) << 3 as libc::c_int),
                            0 as *mut *mut crec, 0 as *mut libc::c_uint);
            break ;
        }
    }
    /* if in hosts, don't need DHCP record */
    if in_hosts != 0 { return }
    /* Name in hosts, address doesn't match */
    if !fail_crec.is_null() {
        inet_ntop(prot,
                  &mut (*fail_crec).addr as *mut all_addr as
                      *const libc::c_void, (*dnsmasq_daemon).namebuff,
                  1025 as libc::c_int as socklen_t);
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
                  b"not giving name %s to the DHCP lease of %s because the name exists in %s with address %s\x00"
                      as *const u8 as *const libc::c_char, host_name,
                  (*dnsmasq_daemon).addrbuff, record_source((*fail_crec).uid),
                  (*dnsmasq_daemon).namebuff);
        return
    }
    crec =
        cache_find_by_addr(0 as *mut crec, host_address,
                           0 as libc::c_int as time_t, flags);
    if !crec.is_null() {
        if (*crec).flags & (1 as libc::c_uint) << 5 as libc::c_int != 0 {
            flags |= (1 as libc::c_uint) << 2 as libc::c_int;
            cache_scan_free(0 as *mut libc::c_char, host_address,
                            1 as libc::c_int as libc::c_ushort,
                            0 as libc::c_int as time_t, flags,
                            0 as *mut *mut crec, 0 as *mut libc::c_uint);
        }
    } else { flags |= (1 as libc::c_uint) << 2 as libc::c_int }
    crec = dhcp_spare;
    if !crec.is_null() {
        dhcp_spare = (*dhcp_spare).next
    } else {
        /* need new one */
        crec =
            whine_malloc((::std::mem::size_of::<crec>() as
                              libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut libc::c_char>()
                                                              as
                                                              libc::c_ulong).wrapping_sub(50
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong))
                as *mut crec
    }
    if !crec.is_null() {
        /* malloc may fail */
        (*crec).flags =
            flags | (1 as libc::c_uint) << 1 as libc::c_int |
                (1 as libc::c_uint) << 4 as libc::c_int |
                (1 as libc::c_uint) << 3 as libc::c_int;
        if ttd == 0 as libc::c_int as libc::c_long {
            (*crec).flags |= (1 as libc::c_uint) << 0 as libc::c_int
        } else { (*crec).ttd = ttd }
        (*crec).addr = *host_address;
        (*crec).name.namep = host_name;
        (*crec).uid = 0 as libc::c_int as libc::c_uint;
        cache_hash(crec);
        make_non_terminals(crec);
    };
}
/* Called when we put a local or DHCP name into the cache.
   Creates empty cache entries for subnames (ie,
   for three.two.one, for two.one and one), without
   F_IPV4 or F_IPV6 or F_CNAME set. These convert
   NXDOMAIN answers to NoData ones. */
unsafe extern "C" fn make_non_terminals(mut source: *mut crec) {
    let mut name: *mut libc::c_char = cache_get_name(source);
    let mut crecp: *mut crec = 0 as *mut crec;
    let mut tmp: *mut crec = 0 as *mut crec;
    let mut up: *mut *mut crec = 0 as *mut *mut crec;
    let mut type_0: libc::c_int =
        ((1 as libc::c_uint) << 6 as libc::c_int |
             (1 as libc::c_uint) << 13 as libc::c_int) as libc::c_int;
    if (*source).flags & (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        type_0 = ((1 as libc::c_uint) << 4 as libc::c_int) as libc::c_int
    }
    /* First delete any empty entries for our new real name. Note that
     we only delete empty entries deriving from DHCP for a new DHCP-derived
     entry and vice-versa for HOSTS and CONFIG. This ensures that 
     non-terminals from DHCP go when we reload DHCP and 
     for HOSTS/CONFIG when we re-read. */
    up = hash_bucket(name);
    crecp = *up;
    while !crecp.is_null() {
        tmp = (*crecp).hash_next;
        if is_outdated_cname_pointer(crecp) == 0 &&
               (*crecp).flags & (1 as libc::c_uint) << 3 as libc::c_int != 0
               && (*crecp).flags & type_0 as libc::c_uint != 0 &&
               (*crecp).flags &
                   ((1 as libc::c_uint) << 7 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int |
                        (1 as libc::c_uint) << 11 as libc::c_int |
                        (1 as libc::c_uint) << 30 as libc::c_int |
                        (1 as libc::c_uint) << 12 as libc::c_int |
                        (1 as libc::c_uint) << 14 as libc::c_int) == 0 &&
               hostname_isequal(name, cache_get_name(crecp)) != 0 {
            *up = (*crecp).hash_next;
            if type_0 as libc::c_uint &
                   (1 as libc::c_uint) << 4 as libc::c_int != 0 {
                (*crecp).next = dhcp_spare;
                dhcp_spare = crecp
            } else { free(crecp as *mut libc::c_void); }
            break ;
        } else { up = &mut (*crecp).hash_next; crecp = tmp }
    }
    loop  {
        name = strchr(name, '.' as i32);
        if name.is_null() { break ; }
        name = name.offset(1);
        /* Look for one existing, don't need another */
        crecp = *hash_bucket(name);
        while !crecp.is_null() {
            if is_outdated_cname_pointer(crecp) == 0 &&
                   (*crecp).flags & (1 as libc::c_uint) << 3 as libc::c_int !=
                       0 && (*crecp).flags & type_0 as libc::c_uint != 0 &&
                   hostname_isequal(name, cache_get_name(crecp)) != 0 {
                break ;
            }
            crecp = (*crecp).hash_next
        }
        if !crecp.is_null() {
            /* If the new name expires later, transfer that time to
	     empty non-terminal entry. */
            if (*crecp).flags & (1 as libc::c_uint) << 0 as libc::c_int == 0 {
                if (*source).flags & (1 as libc::c_uint) << 0 as libc::c_int
                       != 0 {
                    (*crecp).flags |= (1 as libc::c_uint) << 0 as libc::c_int
                } else if difftime((*crecp).ttd, (*source).ttd) <
                              0 as libc::c_int as libc::c_double {
                    (*crecp).ttd = (*source).ttd
                }
            }
        } else {
            if (*source).flags & (1 as libc::c_uint) << 4 as libc::c_int != 0
                   && !dhcp_spare.is_null() {
                crecp = dhcp_spare;
                dhcp_spare = (*dhcp_spare).next
            } else {
                crecp =
                    whine_malloc((::std::mem::size_of::<crec>() as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut libc::c_char>()
                                                                      as
                                                                      libc::c_ulong).wrapping_sub(50
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                        as *mut crec
            }
            if !crecp.is_null() {
                (*crecp).flags =
                    ((*source).flags |
                         (1 as libc::c_uint) << 1 as libc::c_int) &
                        !((1 as libc::c_uint) << 7 as libc::c_int |
                              (1 as libc::c_uint) << 8 as libc::c_int |
                              (1 as libc::c_uint) << 11 as libc::c_int |
                              (1 as libc::c_uint) << 30 as libc::c_int |
                              (1 as libc::c_uint) << 12 as libc::c_int |
                              (1 as libc::c_uint) << 14 as libc::c_int |
                              (1 as libc::c_uint) << 2 as libc::c_int);
                (*crecp).ttd = (*source).ttd;
                (*crecp).name.namep = name;
                cache_hash(crecp);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cache_make_stat(mut t: *mut txt_record)
 -> libc::c_int {
    static mut buff: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    static mut bufflen: libc::c_int = 60 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut serv: *mut server = 0 as *mut server;
    let mut serv1: *mut server = 0 as *mut server;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if buff.is_null() &&
           {
               buff =
                   whine_malloc(60 as libc::c_int as size_t) as
                       *mut libc::c_char;
               buff.is_null()
           } {
        return 0 as libc::c_int
    }
    p = buff;
    match (*t).stat {
        1 => {
            sprintf(buff.offset(1 as libc::c_int as isize),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).cachesize);
        }
        2 => {
            sprintf(buff.offset(1 as libc::c_int as isize),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_INSERTED as
                                                  libc::c_int as usize]);
        }
        3 => {
            sprintf(buff.offset(1 as libc::c_int as isize),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_LIVE_FREED as
                                                  libc::c_int as usize]);
        }
        4 => {
            sprintf(buff.offset(1 as libc::c_int as isize),
                    b"%u\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).metrics[METRIC_DNS_QUERIES_FORWARDED as
                                                  libc::c_int as usize]);
        }
        5 => {
            sprintf(buff.offset(1 as libc::c_int as isize),
                    b"%u\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).metrics[METRIC_DNS_LOCAL_ANSWERED as
                                                  libc::c_int as usize]);
        }
        6 => {
            sprintf(buff.offset(1 as libc::c_int as isize),
                    b"%u\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).metrics[METRIC_DNS_AUTH_ANSWERED as
                                                  libc::c_int as usize]);
        }
        7 => {
            /* sum counts from different records for same server */
            serv = (*dnsmasq_daemon).servers; /* length */
            while !serv.is_null() {
                (*serv).flags &= !(512 as libc::c_int);
                serv = (*serv).next
            }
            serv = (*dnsmasq_daemon).servers;
            while !serv.is_null() {
                if (*serv).flags &
                       (2 as libc::c_int | 4 as libc::c_int |
                            512 as libc::c_int | 1024 as libc::c_int |
                            2048 as libc::c_int) == 0 {
                    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut lenp: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut port: libc::c_int = 0;
                    let mut newlen: libc::c_int = 0;
                    let mut bytes_avail: libc::c_int = 0;
                    let mut bytes_needed: libc::c_int = 0;
                    let mut queries: libc::c_uint =
                        0 as libc::c_int as libc::c_uint;
                    let mut failed_queries: libc::c_uint =
                        0 as libc::c_int as libc::c_uint;
                    serv1 = serv;
                    while !serv1.is_null() {
                        if (*serv1).flags &
                               (2 as libc::c_int | 4 as libc::c_int |
                                    512 as libc::c_int | 1024 as libc::c_int |
                                    2048 as libc::c_int) == 0 &&
                               sockaddr_isequal(&mut (*serv).addr,
                                                &mut (*serv1).addr) != 0 {
                            (*serv1).flags |= 512 as libc::c_int;
                            queries = queries.wrapping_add((*serv1).queries);
                            failed_queries =
                                failed_queries.wrapping_add((*serv1).failed_queries)
                        }
                        serv1 = (*serv1).next
                    }
                    port =
                        prettyprint_addr(&mut (*serv).addr,
                                         (*dnsmasq_daemon).addrbuff);
                    let fresh11 = p;
                    p = p.offset(1);
                    lenp = fresh11;
                    bytes_avail =
                        (bufflen as libc::c_long -
                             p.wrapping_offset_from(buff) as libc::c_long) as
                            libc::c_int;
                    bytes_needed =
                        snprintf(p, bytes_avail as libc::c_ulong,
                                 b"%s#%d %u %u\x00" as *const u8 as
                                     *const libc::c_char,
                                 (*dnsmasq_daemon).addrbuff, port, queries,
                                 failed_queries);
                    if bytes_needed >= bytes_avail {
                        /* expand buffer if necessary */
                        newlen =
                            bytes_needed + 1 as libc::c_int + bufflen -
                                bytes_avail;
                        new =
                            whine_malloc(newlen as size_t) as
                                *mut libc::c_char;
                        if new.is_null() { return 0 as libc::c_int }
                        memcpy(new as *mut libc::c_void,
                               buff as *const libc::c_void,
                               bufflen as libc::c_ulong);
                        free(buff as *mut libc::c_void);
                        p =
                            new.offset(p.wrapping_offset_from(buff) as
                                           libc::c_long as isize);
                        lenp = p.offset(-(1 as libc::c_int as isize));
                        buff = new;
                        bufflen = newlen;
                        bytes_avail =
                            (bufflen as libc::c_long -
                                 p.wrapping_offset_from(buff) as libc::c_long)
                                as libc::c_int;
                        bytes_needed =
                            snprintf(p, bytes_avail as libc::c_ulong,
                                     b"%s#%d %u %u\x00" as *const u8 as
                                         *const libc::c_char,
                                     (*dnsmasq_daemon).addrbuff, port,
                                     queries, failed_queries)
                    }
                    *lenp = bytes_needed as libc::c_char;
                    p = p.offset(bytes_needed as isize)
                }
                serv = (*serv).next
            }
            (*t).txt = buff as *mut libc::c_uchar;
            (*t).len =
                p.wrapping_offset_from(buff) as libc::c_long as
                    libc::c_ushort;
            return 1 as libc::c_int
        }
        _ => { }
    }
    len = strlen(buff.offset(1 as libc::c_int as isize)) as libc::c_int;
    (*t).txt = buff as *mut libc::c_uchar;
    (*t).len = (len + 1 as libc::c_int) as libc::c_ushort;
    *buff = len as libc::c_char;
    return 1 as libc::c_int;
}
/* There can be names in the cache containing control chars, don't 
   mess up logging or open security holes. */
unsafe extern "C" fn sanitise(mut name: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut r: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !name.is_null() {
        r = name as *mut libc::c_uchar;
        while *r != 0 {
            if *(*__ctype_b_loc()).offset(*r as libc::c_int as isize) as
                   libc::c_int &
                   _ISprint as libc::c_int as libc::c_ushort as libc::c_int ==
                   0 {
                return b"<name unprintable>\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char
            }
            r = r.offset(1)
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn dump_cache(mut now: time_t) {
    let mut serv: *mut server = 0 as *mut server;
    let mut serv1: *mut server = 0 as *mut server;
    my_syslog(6 as libc::c_int,
              b"time %lu\x00" as *const u8 as *const libc::c_char,
              now as libc::c_ulong);
    my_syslog(6 as libc::c_int,
              b"cache size %d, %d/%d cache insertions re-used unexpired cache entries.\x00"
                  as *const u8 as *const libc::c_char,
              (*dnsmasq_daemon).cachesize,
              (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_LIVE_FREED as
                                            libc::c_int as usize],
              (*dnsmasq_daemon).metrics[METRIC_DNS_CACHE_INSERTED as
                                            libc::c_int as usize]);
    my_syslog(6 as libc::c_int,
              b"queries forwarded %u, queries answered locally %u\x00" as
                  *const u8 as *const libc::c_char,
              (*dnsmasq_daemon).metrics[METRIC_DNS_QUERIES_FORWARDED as
                                            libc::c_int as usize],
              (*dnsmasq_daemon).metrics[METRIC_DNS_LOCAL_ANSWERED as
                                            libc::c_int as usize]);
    my_syslog(6 as libc::c_int,
              b"queries for authoritative zones %u\x00" as *const u8 as
                  *const libc::c_char,
              (*dnsmasq_daemon).metrics[METRIC_DNS_AUTH_ANSWERED as
                                            libc::c_int as usize]);
    blockdata_report();
    /* sum counts from different records for same server */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        (*serv).flags &= !(512 as libc::c_int);
        serv = (*serv).next
    }
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags &
               (2 as libc::c_int | 4 as libc::c_int | 512 as libc::c_int |
                    1024 as libc::c_int | 2048 as libc::c_int) == 0 {
            let mut port: libc::c_int = 0;
            let mut queries: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut failed_queries: libc::c_uint =
                0 as libc::c_int as libc::c_uint;
            serv1 = serv;
            while !serv1.is_null() {
                if (*serv1).flags &
                       (2 as libc::c_int | 4 as libc::c_int |
                            512 as libc::c_int | 1024 as libc::c_int |
                            2048 as libc::c_int) == 0 &&
                       sockaddr_isequal(&mut (*serv).addr, &mut (*serv1).addr)
                           != 0 {
                    (*serv1).flags |= 512 as libc::c_int;
                    queries = queries.wrapping_add((*serv1).queries);
                    failed_queries =
                        failed_queries.wrapping_add((*serv1).failed_queries)
                }
                serv1 = (*serv1).next
            }
            port =
                prettyprint_addr(&mut (*serv).addr,
                                 (*dnsmasq_daemon).addrbuff);
            my_syslog(6 as libc::c_int,
                      b"server %s#%d: queries sent %u, retried or failed %u\x00"
                          as *const u8 as *const libc::c_char,
                      (*dnsmasq_daemon).addrbuff, port, queries,
                      failed_queries);
        }
        serv = (*serv).next
    }
    if (*dnsmasq_daemon).options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           (*dnsmasq_daemon).options[(2 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (2 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        let mut cache: *mut crec = 0 as *mut crec;
        let mut i: libc::c_int = 0;
        my_syslog(6 as libc::c_int,
                  b"Host                                     Address                        Flags      Expires\x00"
                      as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < hash_size {
            cache = *hash_table.offset(i as isize);
            while !cache.is_null() {
                let mut t: *mut libc::c_char =
                    b" \x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char;
                let mut a: *mut libc::c_char = (*dnsmasq_daemon).addrbuff;
                let mut p: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
                let mut n: *mut libc::c_char = cache_get_name(cache);
                *a = 0 as libc::c_int as libc::c_char;
                if strlen(n) == 0 as libc::c_int as libc::c_ulong &&
                       (*cache).flags &
                           (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                    n =
                        b"<Root>\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                }
                p =
                    p.offset(sprintf(p,
                                     b"%-30.30s \x00" as *const u8 as
                                         *const libc::c_char, sanitise(n)) as
                                 isize);
                if (*cache).flags & (1 as libc::c_uint) << 11 as libc::c_int
                       != 0 && is_outdated_cname_pointer(cache) == 0 {
                    a = sanitise(cache_get_cname_target(cache))
                } else if (*cache).flags &
                              (1 as libc::c_uint) << 30 as libc::c_int != 0 &&
                              (*cache).flags &
                                  (1 as libc::c_uint) << 5 as libc::c_int == 0
                 {
                    let mut targetlen: libc::c_int =
                        (*cache).addr.srv.targetlen as libc::c_int;
                    let mut len: ssize_t =
                        sprintf(a,
                                b"%u %u %u \x00" as *const u8 as
                                    *const libc::c_char,
                                (*cache).addr.srv.priority as libc::c_int,
                                (*cache).addr.srv.weight as libc::c_int,
                                (*cache).addr.srv.srvport as libc::c_int) as
                            ssize_t;
                    if targetlen as libc::c_long >
                           40 as libc::c_int as libc::c_long - len {
                        targetlen =
                            (40 as libc::c_int as libc::c_long - len) as
                                libc::c_int
                    }
                    blockdata_retrieve((*cache).addr.srv.target,
                                       targetlen as size_t,
                                       a.offset(len as isize) as
                                           *mut libc::c_void);
                    *a.offset((len + targetlen as libc::c_long) as isize) =
                        0 as libc::c_int as libc::c_char
                } else if (*cache).flags &
                              (1 as libc::c_uint) << 5 as libc::c_int == 0 ||
                              (*cache).flags &
                                  (1 as libc::c_uint) << 3 as libc::c_int == 0
                 {
                    a = (*dnsmasq_daemon).addrbuff;
                    if (*cache).flags &
                           (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                        inet_ntop(2 as libc::c_int,
                                  &mut (*cache).addr as *mut all_addr as
                                      *const libc::c_void, a,
                                  46 as libc::c_int as socklen_t);
                    } else if (*cache).flags &
                                  (1 as libc::c_uint) << 8 as libc::c_int != 0
                     {
                        inet_ntop(10 as libc::c_int,
                                  &mut (*cache).addr as *mut all_addr as
                                      *const libc::c_void, a,
                                  46 as libc::c_int as socklen_t);
                    }
                }
                if (*cache).flags & (1 as libc::c_uint) << 7 as libc::c_int !=
                       0 {
                    t =
                        b"4\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                } else if (*cache).flags &
                              (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                    t =
                        b"6\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                } else if (*cache).flags &
                              (1 as libc::c_uint) << 11 as libc::c_int != 0 {
                    t =
                        b"C\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                } else if (*cache).flags &
                              (1 as libc::c_uint) << 30 as libc::c_int != 0 {
                    t =
                        b"V\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                }
                p =
                    p.offset(sprintf(p,
                                     b"%-40.40s %s%s%s%s%s%s%s%s%s  \x00" as
                                         *const u8 as *const libc::c_char, a,
                                     t,
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                3 as libc::c_int != 0 {
                                         b"F\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                2 as libc::c_int != 0 {
                                         b"R\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                0 as libc::c_int != 0 {
                                         b"I\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                4 as libc::c_int != 0 {
                                         b"D\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                5 as libc::c_int != 0 {
                                         b"N\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                10 as libc::c_int != 0 {
                                         b"X\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                6 as libc::c_int != 0 {
                                         b"H\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     },
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                15 as libc::c_int != 0 {
                                         b"V\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b" \x00" as *const u8 as
                                             *const libc::c_char
                                     }) as isize);
                p =
                    p.offset(sprintf(p,
                                     b"%s\x00" as *const u8 as
                                         *const libc::c_char,
                                     if (*cache).flags &
                                            (1 as libc::c_uint) <<
                                                0 as libc::c_int != 0 {
                                         b"\n\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         ctime(&mut (*cache).ttd) as
                                             *const libc::c_char
                                     }) as isize);
                /* ctime includes trailing \n - eat it */
                *p.offset(-(1 as libc::c_int as isize)) =
                    0 as libc::c_int as
                        libc::c_char; /* strlen("type=xxxxx") */
                my_syslog(6 as libc::c_int,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).namebuff); /* braces */
                cache = (*cache).hash_next
            } /* terminator */
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn record_source(mut index: libc::c_uint)
 -> *mut libc::c_char {
    let mut ah: *mut hostsfile = 0 as *mut hostsfile;
    if index == 1 as libc::c_int as libc::c_uint {
        return b"config\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    } else {
        if index == 2 as libc::c_int as libc::c_uint {
            return b"/etc/hosts\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    }
    ah = (*dnsmasq_daemon).addn_hosts;
    while !ah.is_null() {
        if (*ah).index == index { return (*ah).fname }
        ah = (*ah).next
    }
    ah = (*dnsmasq_daemon).dynamic_dirs;
    while !ah.is_null() {
        if (*ah).index == index { return (*ah).fname }
        ah = (*ah).next
    }
    return b"<unknown>\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn querystr(mut desc: *mut libc::c_char,
                                  mut type_0: libc::c_ushort)
 -> *mut libc::c_char {
    let mut i: libc::c_uint = 0;
    let mut len: libc::c_int = 10 as libc::c_int;
    let mut types: *const libc::c_char = 0 as *const libc::c_char;
    static mut buff: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    static mut bufflen: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[C2RustUnnamed_10; 40]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_10>()
                                                   as libc::c_ulong) {
        if typestr[i as usize].type_0 == type_0 as libc::c_uint {
            types = typestr[i as usize].name;
            len = strlen(types) as libc::c_int;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if !desc.is_null() {
        len += 2 as libc::c_int;
        len =
            (len as libc::c_ulong).wrapping_add(strlen(desc)) as libc::c_int
                as libc::c_int
    }
    len += 1;
    if buff.is_null() || bufflen < len {
        if !buff.is_null() {
            free(buff as *mut libc::c_void);
        } else if len < 20 as libc::c_int { len = 20 as libc::c_int }
        buff = whine_malloc(len as size_t) as *mut libc::c_char;
        bufflen = len
    }
    if !buff.is_null() {
        if !desc.is_null() {
            if !types.is_null() {
                sprintf(buff,
                        b"%s[%s]\x00" as *const u8 as *const libc::c_char,
                        desc, types);
            } else {
                sprintf(buff,
                        b"%s[type=%d]\x00" as *const u8 as
                            *const libc::c_char, desc, type_0 as libc::c_int);
            }
        } else if !types.is_null() {
            sprintf(buff, b"<%s>\x00" as *const u8 as *const libc::c_char,
                    types);
        } else {
            sprintf(buff, b"type=%d\x00" as *const u8 as *const libc::c_char,
                    type_0 as libc::c_int);
        }
    }
    return if !buff.is_null() {
               buff as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char } as
               *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn log_query(mut flags: libc::c_uint,
                                   mut name: *mut libc::c_char,
                                   mut addr: *mut all_addr,
                                   mut arg: *mut libc::c_char) {
    let mut source: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dest: *mut libc::c_char = (*dnsmasq_daemon).addrbuff;
    let mut verb: *mut libc::c_char =
        b"is\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if (*dnsmasq_daemon).options[(2 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (2 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        return
    }
    name = sanitise(name);
    if !addr.is_null() {
        if flags & (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            sprintf((*dnsmasq_daemon).addrbuff, arg,
                    (*addr).log.keytag as libc::c_int,
                    (*addr).log.algo as libc::c_int,
                    (*addr).log.digest as libc::c_int);
        } else if flags & (1 as libc::c_uint) << 29 as libc::c_int != 0 {
            let mut rcode: libc::c_uint = (*addr).log.rcode as libc::c_uint;
            if rcode == 2 as libc::c_int as libc::c_uint {
                dest =
                    b"SERVFAIL\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            } else if rcode == 5 as libc::c_int as libc::c_uint {
                dest =
                    b"REFUSED\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            } else if rcode == 4 as libc::c_int as libc::c_uint {
                dest =
                    b"not implemented\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
            } else {
                sprintf((*dnsmasq_daemon).addrbuff,
                        b"%u\x00" as *const u8 as *const libc::c_char, rcode);
            }
        } else {
            inet_ntop(if flags & (1 as libc::c_uint) << 7 as libc::c_int != 0
                         {
                          2 as libc::c_int
                      } else { 10 as libc::c_int },
                      addr as *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                      46 as libc::c_int as socklen_t);
        }
    } else { dest = arg }
    if flags & (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        dest = name;
        name = (*dnsmasq_daemon).addrbuff
    }
    if flags & (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        if flags & (1 as libc::c_uint) << 10 as libc::c_int != 0 {
            dest =
                b"NXDOMAIN\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
            dest =
                b"NODATA-IPv4\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            dest =
                b"NODATA-IPv6\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else {
            dest =
                b"NODATA\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
    } else if flags & (1 as libc::c_uint) << 11 as libc::c_int != 0 {
        dest =
            b"<CNAME>\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        dest =
            b"<SRV>\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 17 as libc::c_int != 0 {
        dest = arg
    }
    if flags & (1 as libc::c_uint) << 13 as libc::c_int != 0 {
        source =
            b"config\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        source =
            b"DHCP\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        source = arg
    } else if flags & (1 as libc::c_uint) << 16 as libc::c_int != 0 {
        source =
            b"reply\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 24 as libc::c_int != 0 {
        source =
            b"validation\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 21 as libc::c_int != 0 {
        source =
            b"auth\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 18 as libc::c_int != 0 {
        source =
            b"forwarded\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        verb =
            b"to\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 19 as libc::c_int != 0 {
        source = arg;
        verb =
            b"from\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 22 as libc::c_int != 0 {
        source = arg;
        verb =
            b"to\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if flags & (1 as libc::c_uint) << 26 as libc::c_int != 0 {
        source =
            b"ipset add\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        dest = name;
        name = arg;
        verb = (*dnsmasq_daemon).addrbuff
    } else {
        source =
            b"cached\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    if strlen(name) == 0 as libc::c_int as libc::c_ulong {
        name =
            b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if (*dnsmasq_daemon).options[(51 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (51 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut port: libc::c_int =
            prettyprint_addr((*dnsmasq_daemon).log_source_addr,
                             (*dnsmasq_daemon).addrbuff2);
        if flags & (1 as libc::c_uint) << 27 as libc::c_int != 0 {
            my_syslog(6 as libc::c_int,
                      b"* %s/%u %s %s %s %s\x00" as *const u8 as
                          *const libc::c_char, (*dnsmasq_daemon).addrbuff2,
                      port, source, name, verb, dest);
        } else {
            my_syslog(6 as libc::c_int,
                      b"%u %s/%u %s %s %s %s\x00" as *const u8 as
                          *const libc::c_char,
                      (*dnsmasq_daemon).log_display_id,
                      (*dnsmasq_daemon).addrbuff2, port, source, name, verb,
                      dest);
        }
    } else {
        my_syslog(6 as libc::c_int,
                  b"%s %s %s %s\x00" as *const u8 as *const libc::c_char,
                  source, name, verb, dest);
    };
}
