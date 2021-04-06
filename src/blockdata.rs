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
use crate::defines::{BlockData, DnsmasqDaemon};

// static mut keyblock_free: BlockData =
//     0 as *const BlockData as *mut BlockData;
// static mut blockdata_count: libc::c_uint = 0;
// static mut blockdata_hwm: libc::c_uint = 0;
// static mut blockdata_alloced: libc::c_uint = 0;
fn blockdata_expand(mut n: usize) {
    let mut BlockData = Default::default();
    let mut i: usize = 0;
    let ref mut fresh6 = (*new.offset((n - 1))).next;
    *fresh6 = keyblock_free;
    keyblock_free = new;
    i = 0;
    while i < n - 1 {
        let ref mut fresh7 = (*new.offset(i)).next;
        *fresh7 = &mut *new.offset((i + 1));
        i += 1
    }
    blockdata_alloced = blockdata_alloced.wrapping_add(n)
}

/* Preallocate some blocks, proportional to cachesize, to reduce heap fragmentation. */
pub fn blockdata_init(daemon: &mut DnsmasqDaemon) {
    keyblock_free = 0;
    blockdata_alloced = 0;
    blockdata_count = 0;
    blockdata_hwm = 0;
    /* Note that daemon->cachesize is enforced to have non-zero size if OPT_DNSSEC_VALID is set */
    if daemon.options[45] {
        blockdata_expand(daemon.cachesize);
    };
}

pub fn blockdata_report() {
    // my_syslog(
    //     6,
    //     "pool memory in use %u, max %u, allocated %u" ,
    //     (blockdata_count)
    //         .wrapping_mul(::std::mem::size_of::<BlockData>()),
    //     (blockdata_hwm)
    //         .wrapping_mul(::std::mem::size_of::<BlockData>()),
    //     (blockdata_alloced)
    //         .wrapping_mul(::std::mem::size_of::<BlockData>()),
    // );
    // TODO: add in new log call
}

pub fn blockdata_alloc_real(
    mut fd: i32,
    mut data: &String,
    mut len: usize,
) -> Option<BlockData> {
    let mut block: BlockData;
    let mut ret: BlockData;
    let mut prev: BlockData;
    let mut blen: usize = 0;
    while len > 0 {
        if keyblock_free.is_null() {
            blockdata_expand(50);
        }
        if !keyblock_free.is_null() {
            block = keyblock_free;
            keyblock_free = block.next;
            blockdata_count = blockdata_count.wrapping_add(1)
        } else {
            /* failed to alloc, free partial chain */
            // blockdata_free(ret);
            return None;
        }
        if blockdata_hwm < blockdata_count {
            blockdata_hwm = blockdata_count
        }
        blen = if len > 40 { 40 } else { len };
        if !data.is_null() {
            memcpy(
                block.key.as_mut_ptr(),
                data,
                blen,
            );
            data = data.offset(blen)
        } else if read_write(
            fd,
            block.key.as_mut_ptr(),
            blen,
            1,
        ) == 0
        {
            /* failed read free partial chain */
            // blockdata_free(ret);
            return None;
        }
        len = (len).wrapping_sub(blen)  ;
        prev = block;
        prev = &mut block.next;
        block.next = 0
    }
    return Some(ret);
}

pub fn blockdata_alloc(mut data: &mut String, mut len: usize) -> BlockData {
    return blockdata_alloc_real(0, data, len);
}

pub fn blockdata_free(mut blocks: &mut BlockData) {
    let mut tmp: BlockData = 0;
    if !blocks.is_null() {
        tmp = blocks;
        while !tmp.next.is_null() {
            blockdata_count = blockdata_count.wrapping_sub(1);
            tmp = tmp.next
        }
        tmp.next = keyblock_free;
        keyblock_free = blocks;
        blockdata_count = blockdata_count.wrapping_sub(1)
    };
}
/* if data == NULL, return pointer to static block of sufficient size */

pub fn blockdata_retrieve(
    mut block: &mut BlockData,
    mut len: usize,
    mut data: &mut Vec<u8>,
) ->Vec<u8> {
    let mut blen: usize = 0;
    let mut b: BlockData;
    let mut new: Vec<u8> = Vec::new();
    let mut d: Vec<u8> = Vec::New();
    let mut buff_len: usize = 0;
    let mut buff: Vec<u8> = Vec::new();
    if data.is_null() {
        data.clone_from(&buff);
    }
    d = data.clone();
    b = block.clone();
    while len > 0 && !b.is_null() {
        blen = if len > 40 {
            40
        } else {
            len
        };
        memcpy(d, b.key.as_mut_ptr(), blen);
        d = d.offset(blen);
        len = (len).wrapping_sub(blen)  ;
        b = b.next
    }
    let out = data.clone();
    return out;
}

pub fn blockdata_write(mut block: &mut BlockData, mut len: usize, mut fd: i32) {
    while len > 0 && !block.is_null() {
        let mut blen: usize = if len > 40 {
            40
        } else {
            len
        };
        read_write(
            fd,
            block.key.as_mut_ptr(),
            blen,
            0,
        );
        len = (len).wrapping_sub(blen)  ;
        block = block.next
    }
}

pub fn blockdata_read(mut fd: i32, mut len: usize) -> BlockData {
    return blockdata_alloc_real(fd, 0 , len);
}
