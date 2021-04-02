
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
use crate::defines::{size_t, DnsmasqDaemon};

static mut outpacket_counter: usize = 0;

pub fn end_opt6(mut container: i32) {
    let mut p:Vec<u8> =
        daemon.outpacket.iov_base.offset(container     isize).offset(2                       libc::c_int
                                                          );
    let mut len: u16 =
        outpacket_counter.wrapping_sub(container                              ).wrapping_sub(4                        libc::c_int
                                                                                           )
           ;
    let mut t_s: u16 = len;
    let mut t_cp: mut Vec<u8> = p;
    let fresh6 = t_cp;
    t_cp = t_cp.offset(1);
    *fresh6 = (t_s >> 8);
    *t_cp = t_s;
    p = p.offset(2);
}

pub fn reset_counter(daemon: &mut DnsmasqDaemon) {
    /* Clear out buffer when starting from beginning */
    if !daemon.outpacket.iov_base.is_null() {
    }
    save_counter(0);
}

pub fn save_counter(mut newval: i32)
 -> i32 {
    let mut ret: i32 = outpacket_counter;
    if newval != -(1) { outpacket_counter = newval  }
    return ret;
}

pub fn expand(mut headroom: usize) ->Vec<u8> {
    let mut ret:Vec<u8> = 0;
    if expand_buf(&mut daemon.outpacket,
                  outpacket_counter.wrapping_add(headroom)) != 0 {
        ret =
            daemon.outpacket.iov_base.offset(outpacket_counter         isize);
        outpacket_counter =
            (outpacket_counter).wrapping_add(headroom)          size_t ;
        return ret
    }
    return 0;
}

pub fn new_opt6(mut opt: i32) -> i32 {
    let mut ret: i32 = outpacket_counter;
    let mut p:Vec<u8> = 0;
    p = expand(4 );
    if !p.is_null() {
        let mut t_s: u16 = opt;
        let mut t_cp: mut Vec<u8> = p;
        let fresh7 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh7 = (t_s >> 8);
        *t_cp = t_s;
        p = p.offset(2);
        let mut t_s_0: u16 = 0;
        let mut t_cp_0: mut Vec<u8> = p;
        let fresh8 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh8 = (t_s_0 >> 8);
        *t_cp_0 = t_s_0;
        p = p.offset(2)
    }
    return ret;
}

pub fn put_opt6(mut data:Vec<u8>,
                                  mut len: usize) ->Vec<u8> {
    let mut p:Vec<u8> = 0;
    p = expand(len);
    if !p.is_null() && !data.is_null() { memcpy(p, data, len); }
    return p;
}

pub fn put_opt6_long(mut val: u32) {
    let mut p:Vec<u8> = 0;
    p = expand(4 );
    if !p.is_null() {
        let mut t_l: u32 = val;
        let mut t_cp: mut Vec<u8> = p;
        let fresh9 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh9 = (t_l >> 24);
        let fresh10 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh10 = (t_l >> 16);
        let fresh11 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh11 = (t_l >> 8);
        *t_cp = t_l;
        p = p.offset(4)
    };
}

pub fn put_opt6_short(mut val: u32) {
    let mut p:Vec<u8> = 0;
    p = expand(2 );
    if !p.is_null() {
        let mut t_s: u16 = val;
        let mut t_cp: mut Vec<u8> = p;
        let fresh12 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh12 = (t_s >> 8);
        *t_cp = t_s;
        p = p.offset(2)
    };
}

pub fn put_opt6_char(mut val: u32) {
    let mut p: mut Vec<u8> = 0;
    p = expand(1 );
    if !p.is_null() { *p = val };
}

pub fn put_opt6_string(mut s: &mut String) {
    put_opt6(s, strlen(s));
}
