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



 let mut keyblock_free: blockdata;
 unsigned blockdata_count: i32, blockdata_hwm, blockdata_alloced;

pub fn blockdata_expand(int n)
{
  struct blockdata *new = whine_malloc(n * sizeof(struct blockdata));
  
  if (new)
    {
      let mut i: i32;
      
      new[n-1].next = keyblock_free;
      keyblock_free = new;

      for (i = 0; i < n - 1; i++)
	new[i].next = &new[i+1];

      blockdata_alloced += n;
    }
}

/* Preallocate some blocks, proportional to cachesize, to reduce heap fragmentation. */
pub fn blockdata_init()
{
  keyblock_free = NULL;
  blockdata_alloced = 0;
  blockdata_count = 0;
  blockdata_hwm = 0;

  /* Note that daemon.cachesize is enforced to have non-zero size if OPT_DNSSEC_VALID is set */  
  if (daemon.opt_dnssec_valid)
    blockdata_expand(daemon.cachesize);
}

pub fn blockdata_report()
{
  my_syslog(LOG_INFO, format!("pool memory in use {}, max {}, allocated {}"), 
	    blockdata_count * sizeof(struct blockdata),  
	    blockdata_hwm * sizeof(struct blockdata),  
	    blockdata_alloced * sizeof(struct blockdata));
} 

 struct blockdata *blockdata_alloc_real(fd: i32, data: &mut String, len: usize)
{
  struct blockdata *block, *ret = NULL;
  struct blockdata **prev = &ret;
  blen: usize;

  while (len > 0)
    {
      if (!keyblock_free)
	blockdata_expand(50);
      
      if (keyblock_free)
	{
	  block = keyblock_free;
	  keyblock_free = block.next;
	  blockdata_count +=1; 
	}
      else
	{
	  /* failed to alloc, free partial chain */
	  blockdata_free(ret);
	  return NULL;
	}
       
      if (blockdata_hwm < blockdata_count)
	blockdata_hwm = blockdata_count; 
      
      blen = len > KEYBLOCK_LEN ? KEYBLOCK_LEN : len;
      if (data)
	{
	  memcpy(block.key, data, blen);
	  data += blen;
	}
      else if (!read_write(fd, block.key, blen, 1))
	{
	  /* failed read free partial chain */
	  blockdata_free(ret);
	  return NULL;
	}
      len -= blen;
      *prev = block;
      prev = &block.next;
      block.next = NULL;
    }
  
  return ret;
}

struct blockdata *blockdata_alloc(data: &mut String, len: usize)
{
  return blockdata_alloc_real(0, data, len);
}

pub fn blockdata_free(struct blockdata *blocks)
{
  let mut tmp: blockdata;
  
  if (blocks)
    {
      for (tmp = blocks; tmp.next; tmp = tmp.next)
	blockdata_count--;
      tmp.next = keyblock_free;
      keyblock_free = blocks; 
      blockdata_count--;
    }
}

/* if data == NULL, return pointer to  block of sufficient size */
blockdata_retrieve: Vec<u8>(struct blockdata *block, len: usize, data: Vec<u8>)
{
  blen: usize;
  struct  blockdata *b;
  new: Vec<u8>, *d;
  
   let mut buff_len: u32 = 0;
   let mut buff: *mut u8 = NULL;
   
  if (!data)
    {
      if (len > buff_len)
	{
	  if (!(new = whine_malloc(len)))
	    return NULL;
	  if (buff)
	    free(buff);
	  buff = new;
	}
      data = buff;
    }
  
  for (d = data, b = block; len > 0 && b;  b = b.next)
    {
      blen = len > KEYBLOCK_LEN ? KEYBLOCK_LEN : len;
      memcpy(d, b.key, blen);
      d += blen;
      len -= blen;
    }

  return data;
}


pub fn blockdata_write(struct blockdata *block, len: usize, fd: i32)
{
  for (; len > 0 && block; block = block.next)
    {
      blen: usize = len > KEYBLOCK_LEN ? KEYBLOCK_LEN : len;
      read_write(fd, block.key, blen, 0);
      len -= blen;
    }
}

struct blockdata *blockdata_read(fd: i32, len: usize)
{
  return blockdata_alloc_real(fd, NULL, len);
}

