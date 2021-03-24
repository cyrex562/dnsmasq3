
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
static mut outpacket_counter: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn end_opt6(mut container: libc::c_int) {
    let mut p: *mut libc::c_void =
        (*dnsmasq_daemon).outpacket.iov_base.offset(container as
                                                        isize).offset(2 as
                                                                          libc::c_int
                                                                          as
                                                                          isize);
    let mut len: u16_0 =
        outpacket_counter.wrapping_sub(container as
                                           libc::c_ulong).wrapping_sub(4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
            as u16_0;
    let mut t_s: u16_0 = len;
    let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
    let fresh6 = t_cp;
    t_cp = t_cp.offset(1);
    *fresh6 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    *t_cp = t_s as libc::c_uchar;
    p = p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn reset_counter() {
    /* Clear out buffer when starting from beginning */
    if !(*dnsmasq_daemon).outpacket.iov_base.is_null() {
        memset((*dnsmasq_daemon).outpacket.iov_base, 0 as libc::c_int,
               (*dnsmasq_daemon).outpacket.iov_len);
    }
    save_counter(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn save_counter(mut newval: libc::c_int)
 -> libc::c_int {
    let mut ret: libc::c_int = outpacket_counter as libc::c_int;
    if newval != -(1 as libc::c_int) { outpacket_counter = newval as size_t }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn expand(mut headroom: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if expand_buf(&mut (*dnsmasq_daemon).outpacket,
                  outpacket_counter.wrapping_add(headroom)) != 0 {
        ret =
            (*dnsmasq_daemon).outpacket.iov_base.offset(outpacket_counter as
                                                            isize);
        outpacket_counter =
            (outpacket_counter as libc::c_ulong).wrapping_add(headroom) as
                size_t as size_t;
        return ret
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn new_opt6(mut opt: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = outpacket_counter as libc::c_int;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(4 as libc::c_int as size_t);
    if !p.is_null() {
        let mut t_s: u16_0 = opt as u16_0;
        let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh7 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh7 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_s_0: u16_0 = 0 as libc::c_int as u16_0;
        let mut t_cp_0: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh8 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh8 = (t_s_0 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_0 = t_s_0 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6(mut data: *mut libc::c_void,
                                  mut len: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(len);
    if !p.is_null() && !data.is_null() { memcpy(p, data, len); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_long(mut val: libc::c_uint) {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(4 as libc::c_int as size_t);
    if !p.is_null() {
        let mut t_l: u32_0 = val;
        let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh9 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh9 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
        let fresh10 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh10 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
        let fresh11 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh11 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_l as libc::c_uchar;
        p = p.offset(4 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_short(mut val: libc::c_uint) {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = expand(2 as libc::c_int as size_t);
    if !p.is_null() {
        let mut t_s: u16_0 = val as u16_0;
        let mut t_cp: *mut libc::c_uchar = p as *mut libc::c_uchar;
        let fresh12 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh12 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_char(mut val: libc::c_uint) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = expand(1 as libc::c_int as size_t) as *mut libc::c_uchar;
    if !p.is_null() { *p = val as libc::c_uchar };
}
#[no_mangle]
pub unsafe extern "C" fn put_opt6_string(mut s: *mut libc::c_char) {
    put_opt6(s as *mut libc::c_void, strlen(s));
}
