
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
static mut keyblock_free: *mut blockdata =
    0 as *const blockdata as *mut blockdata;
static mut blockdata_count: libc::c_uint = 0;
static mut blockdata_hwm: libc::c_uint = 0;
static mut blockdata_alloced: libc::c_uint = 0;
unsafe extern "C" fn blockdata_expand(mut n: libc::c_int) {
    let mut new: *mut blockdata =
        whine_malloc((n as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<blockdata>()
                                                          as libc::c_ulong))
            as *mut blockdata;
    if !new.is_null() {
        let mut i: libc::c_int = 0;
        let ref mut fresh6 =
            (*new.offset((n - 1 as libc::c_int) as isize)).next;
        *fresh6 = keyblock_free;
        keyblock_free = new;
        i = 0 as libc::c_int;
        while i < n - 1 as libc::c_int {
            let ref mut fresh7 = (*new.offset(i as isize)).next;
            *fresh7 =
                &mut *new.offset((i + 1 as libc::c_int) as isize) as
                    *mut blockdata;
            i += 1
        }
        blockdata_alloced = blockdata_alloced.wrapping_add(n as libc::c_uint)
    };
}
/* Preallocate some blocks, proportional to cachesize, to reduce heap fragmentation. */
#[no_mangle]
pub unsafe extern "C" fn blockdata_init() {
    keyblock_free = 0 as *mut blockdata;
    blockdata_alloced = 0 as libc::c_int as libc::c_uint;
    blockdata_count = 0 as libc::c_int as libc::c_uint;
    blockdata_hwm = 0 as libc::c_int as libc::c_uint;
    /* Note that daemon->cachesize is enforced to have non-zero size if OPT_DNSSEC_VALID is set */
    if (*dnsmasq_daemon).options[(45 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (45 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        blockdata_expand((*dnsmasq_daemon).cachesize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn blockdata_report() {
    my_syslog(6 as libc::c_int,
              b"pool memory in use %u, max %u, allocated %u\x00" as *const u8
                  as *const libc::c_char,
              (blockdata_count as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<blockdata>()
                                                   as libc::c_ulong),
              (blockdata_hwm as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<blockdata>()
                                                   as libc::c_ulong),
              (blockdata_alloced as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<blockdata>()
                                                   as libc::c_ulong));
}
unsafe extern "C" fn blockdata_alloc_real(mut fd: libc::c_int,
                                          mut data: *mut libc::c_char,
                                          mut len: size_t) -> *mut blockdata {
    let mut block: *mut blockdata = 0 as *mut blockdata;
    let mut ret: *mut blockdata = 0 as *mut blockdata;
    let mut prev: *mut *mut blockdata = &mut ret;
    let mut blen: size_t = 0;
    while len > 0 as libc::c_int as libc::c_ulong {
        if keyblock_free.is_null() { blockdata_expand(50 as libc::c_int); }
        if !keyblock_free.is_null() {
            block = keyblock_free;
            keyblock_free = (*block).next;
            blockdata_count = blockdata_count.wrapping_add(1)
        } else {
            /* failed to alloc, free partial chain */
            blockdata_free(ret);
            return 0 as *mut blockdata
        }
        if blockdata_hwm < blockdata_count { blockdata_hwm = blockdata_count }
        blen =
            if len > 40 as libc::c_int as libc::c_ulong {
                40 as libc::c_int as libc::c_ulong
            } else { len };
        if !data.is_null() {
            memcpy((*block).key.as_mut_ptr() as *mut libc::c_void,
                   data as *const libc::c_void, blen);
            data = data.offset(blen as isize)
        } else if read_write(fd, (*block).key.as_mut_ptr(),
                             blen as libc::c_int, 1 as libc::c_int) == 0 {
            /* failed read free partial chain */
            blockdata_free(ret);
            return 0 as *mut blockdata
        }
        len = (len as libc::c_ulong).wrapping_sub(blen) as size_t as size_t;
        *prev = block;
        prev = &mut (*block).next;
        (*block).next = 0 as *mut blockdata
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn blockdata_alloc(mut data: *mut libc::c_char,
                                         mut len: size_t) -> *mut blockdata {
    return blockdata_alloc_real(0 as libc::c_int, data, len);
}
#[no_mangle]
pub unsafe extern "C" fn blockdata_free(mut blocks: *mut blockdata) {
    let mut tmp: *mut blockdata = 0 as *mut blockdata;
    if !blocks.is_null() {
        tmp = blocks;
        while !(*tmp).next.is_null() {
            blockdata_count = blockdata_count.wrapping_sub(1);
            tmp = (*tmp).next
        }
        (*tmp).next = keyblock_free;
        keyblock_free = blocks;
        blockdata_count = blockdata_count.wrapping_sub(1)
    };
}
/* if data == NULL, return pointer to static block of sufficient size */
#[no_mangle]
pub unsafe extern "C" fn blockdata_retrieve(mut block: *mut blockdata,
                                            mut len: size_t,
                                            mut data: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut blen: size_t = 0;
    let mut b: *mut blockdata = 0 as *mut blockdata;
    let mut new: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut d: *mut libc::c_void = 0 as *mut libc::c_void;
    static mut buff_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    static mut buff: *mut libc::c_uchar =
        0 as *const libc::c_uchar as *mut libc::c_uchar;
    if data.is_null() {
        if len > buff_len as libc::c_ulong {
            new = whine_malloc(len);
            if new.is_null() { return 0 as *mut libc::c_void }
            if !buff.is_null() { free(buff as *mut libc::c_void); }
            buff = new as *mut libc::c_uchar
        }
        data = buff as *mut libc::c_void
    }
    d = data;
    b = block;
    while len > 0 as libc::c_int as libc::c_ulong && !b.is_null() {
        blen =
            if len > 40 as libc::c_int as libc::c_ulong {
                40 as libc::c_int as libc::c_ulong
            } else { len };
        memcpy(d, (*b).key.as_mut_ptr() as *const libc::c_void, blen);
        d = d.offset(blen as isize);
        len = (len as libc::c_ulong).wrapping_sub(blen) as size_t as size_t;
        b = (*b).next
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn blockdata_write(mut block: *mut blockdata,
                                         mut len: size_t,
                                         mut fd: libc::c_int) {
    while len > 0 as libc::c_int as libc::c_ulong && !block.is_null() {
        let mut blen: size_t =
            if len > 40 as libc::c_int as libc::c_ulong {
                40 as libc::c_int as libc::c_ulong
            } else { len };
        read_write(fd, (*block).key.as_mut_ptr(), blen as libc::c_int,
                   0 as libc::c_int);
        len = (len as libc::c_ulong).wrapping_sub(blen) as size_t as size_t;
        block = (*block).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn blockdata_read(mut fd: libc::c_int, mut len: size_t)
 -> *mut blockdata {
    return blockdata_alloc_real(fd, 0 as *mut libc::c_char, len);
}
