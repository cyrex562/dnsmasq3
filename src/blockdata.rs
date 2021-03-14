use std::usize;

use crate::dnsmasq_h::{Daemon, OPT_DNSSEC_VALID, blockdata, option_bool};

/* dnsmasq is Copyright (c) 2000-2018 Simon Kelley

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

// //#include "dnsmasq.h"

// //#ifdef HAVE_DNSSEC

// static struct blockdata *keyblock_free;
static keyblock_free: Vec<blockdata> = Vec::new();

// static unsigned int blockdata_count, blockdata_hwm, blockdata_alloced;
static blockdata_count: u32 = 0;
static blockdata_hwm: u32 = 0;
static blockdata_alloced: u32 = 0;

pub fn blockdata_expand(n: u32)
{
  // struct blockdata *new = whine_malloc(n * sizeof(struct blockdata));
  let new: Vec<blockdata> = Vec::new();

  let mut i: i32;
     // TODO: refactor 
  // new[n-1].next = keyblock_free;
  // keyblock_free = new;

  // // for (i = 0; i < n - 1; i++) {
  // for i in 0..n-1 {
  //   new[i].next = &new[i+1];
  // }
  blockdata_alloced += n;
}

/* Preallocate some blocks, proportional to cachesize, to reduce heap fragmentation. */
pub fn blockdata_init(daemon: &Daemon)
{
  keyblock_free.clear();
  blockdata_alloced = 0;
  blockdata_count = 0;
  blockdata_hwm = 0;

  /* Note that daemon->cachesize is enforced to have non-zero size if OPT_DNSSEC_VALID is set */  
  if option_bool(OPT_DNSSEC_VALID, daemon) > 0 {
    blockdata_expand(daemon.cachesize);
  }
}

pub fn blockdata_report()
{
  // if (option_bool(OPT_DNSSEC_VALID))
  //   // my_syslog(LOG_INFO, _("DNSSEC memory in use %u, max %u, allocated %u"), 
	//   //     blockdata_count * sizeof(struct blockdata),  
	//   //     blockdata_hwm * sizeof(struct blockdata),  
	//   //     blockdata_alloced * sizeof(struct blockdata));
  //   let i;
} 

pub fn blockdata_alloc(data: &mut String) -> blockdata
{
  // struct blockdata *block, *ret = nullptr;
  let mut ret: blockdata;
  let mut block: blockdata;

  // struct blockdata **prev = &ret;
  let mut prev: blockdata = ret;
  let mut blen: usize;

  while (len > 0) {
      if (!keyblock_free) {
	      blockdata_expand(50);
      }
      
      if (keyblock_free)
	    {
	  block = keyblock_free;
	  keyblock_free = block.next;
	  blockdata_count += 1; 
	}
      else
	{
	  /* failed to alloc, free partial chain */
	  blockdata_free(ret);
	  return nullptr;
	}
       
      if (blockdata_hwm < blockdata_count) {
        blockdata_hwm = blockdata_count;
      }
	 
      
      // blen = len > KEYBLOCK_LEN ? KEYBLOCK_LEN : len;
      if len > KEYBLOCK_LEN {
        blen = KEYBLOCK_LEN
      } else {
        blen = len
      }
      
      // memcpy(block->key, data, blen);
      // TODO: set blen elements of block.key to data
      
      data += blen;
      len -= blen;
      *prev = block;
      prev = &block.next;
      block.next = None;
    }
  
  return ret;
}


pub fn blockdata_free(blockdata: &mut blockdata)
{
  // struct blockdata *tmp;
  let mut tmp: blockdata;

  if (blocks)
  {
    //for (tmp = blocks; tmp->next; tmp = tmp->next) 
	  //    blockdata_count--;
    for block in blocks {
      blockdata_count -= 1;
    }
    
    tmp.next = keyblock_free;
    keyblock_free = blocks; 
    blockdata_count-=1;
  }
}

/* if data == NULL, return pointer to static block of sufficient size */
// pub fn blockdata_retrieve(struct blockdata *block, size_t len, void *data)
pub fn blockdata_retrieve(block: &mut Vec<blockdata>, len: usize, data: Vec<u8>)
{
  let mut blen: usize;
  let mut b: blockdata;
  let mut new: Vec<u8>;
  let mut d: Vec<u8>;
  let mut buff_len: usize = 0;
  let mut buff: Vec<u8>;
  if (!data)
  {
      if (len > buff_len)
	    {
	        if (!(new = whine_malloc(len)))
          {
            return nullptr;
          }
	        
	  if (buff) {
      free(buff);
    }
	    
	  buff = new;
	}
      data = buff;
    }
  
  // for (d = data, b = block; len > 0 && b;  b = b->next)
    // {
    //   blen = len > KEYBLOCK_LEN ? KEYBLOCK_LEN : len;
    //   memcpy(d, b->key, blen);
    //   d += blen;
    //   len -= blen;
    // }
    // TODO: re-implement

  return data;
}
 
//#endif
