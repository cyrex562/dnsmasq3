
#[no_mangle]
pub unsafe extern "C" fn loop_send_probes() {
    let mut serv: *mut server = 0 as *mut server;
    if (*dnsmasq_daemon).options[(50 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (50 as libc::c_int as
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
    let mut current_block_7: u64;
    /* Loop through all upstream servers not for particular domains, and send a query to that server which is
      identifiable, via the uid. If we see that query back again, then the server is looping, and we should not use it. */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags &
               (4 as libc::c_int | 2 as libc::c_int | 1024 as libc::c_int |
                    2048 as libc::c_int | 8 as libc::c_int | 32 as libc::c_int
                    | 8192 as libc::c_int) == 0 {
            let mut len: ssize_t = loop_make_probe((*serv).uid);
            let mut fd: libc::c_int = 0;
            let mut rfd: *mut randfd = 0 as *mut randfd;
            if !(*serv).sfd.is_null() {
                fd = (*(*serv).sfd).fd;
                current_block_7 = 2868539653012386629;
            } else {
                rfd = allocate_rfd((*serv).addr.sa.sa_family as libc::c_int);
                if rfd.is_null() {
                    current_block_7 = 12517898123489920830;
                } else {
                    fd = (*rfd).fd;
                    current_block_7 = 2868539653012386629;
                }
            }
            match current_block_7 {
                12517898123489920830 => { }
                _ => {
                    while retry_send(sendto(fd,
                                            (*dnsmasq_daemon).packet as
                                                *const libc::c_void,
                                            len as size_t, 0 as libc::c_int,
                                            __CONST_SOCKADDR_ARG{__sockaddr__:
                                                                     &mut (*serv).addr.sa,},
                                            sa_len(&mut (*serv).addr) as
                                                socklen_t)) != 0 {
                    }
                    free_rfd(rfd);
                }
            }
        }
        serv = (*serv).next
    };
}
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
unsafe extern "C" fn loop_make_probe(mut uid: u32_0) -> ssize_t {
    let mut header: *mut dns_header =
        (*dnsmasq_daemon).packet as *mut dns_header;
    let mut p: *mut libc::c_uchar =
        header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    /* packet buffer overwritten */
    (*dnsmasq_daemon).srv_save = 0 as *mut server; /* Add terminating zero */
    (*header).id = rand16(); /* log new state */
    (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
    (*header).nscount = (*header).arcount;
    (*header).ancount = (*header).nscount;
    (*header).qdcount = __bswap_16(1 as libc::c_int as __uint16_t);
    (*header).hb3 = 0x1 as libc::c_int as u8_0;
    (*header).hb4 = 0 as libc::c_int as u8_0;
    (*header).hb3 =
        ((*header).hb3 as libc::c_int & !(0x78 as libc::c_int) |
             0 as libc::c_int) as u8_0;
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = 8 as libc::c_int as libc::c_uchar;
    sprintf(p as *mut libc::c_char,
            b"%.8x\x00" as *const u8 as *const libc::c_char, uid);
    p = p.offset(8 as libc::c_int as isize);
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 =
        strlen(b"test\x00" as *const u8 as *const libc::c_char) as
            libc::c_uchar;
    strcpy(p as *mut libc::c_char,
           b"test\x00" as *const u8 as *const libc::c_char);
    p =
        p.offset(strlen(b"test\x00" as *const u8 as
                            *const libc::c_char).wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                     as isize);
    let mut t_s: u16_0 = 16 as libc::c_int as u16_0;
    let mut t_cp: *mut libc::c_uchar = p;
    let fresh8 = t_cp;
    t_cp = t_cp.offset(1);
    *fresh8 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    *t_cp = t_s as libc::c_uchar;
    p = p.offset(2 as libc::c_int as isize);
    let mut t_s_0: u16_0 = 1 as libc::c_int as u16_0;
    let mut t_cp_0: *mut libc::c_uchar = p;
    let fresh9 = t_cp_0;
    t_cp_0 = t_cp_0.offset(1);
    *fresh9 = (t_s_0 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    *t_cp_0 = t_s_0 as libc::c_uchar;
    p = p.offset(2 as libc::c_int as isize);
    return p.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn detect_loop(mut query: *mut libc::c_char,
                                     mut type_0: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut uid: u32_0 = 0;
    let mut serv: *mut server = 0 as *mut server;
    if (*dnsmasq_daemon).options[(50 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (50 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        return 0 as libc::c_int
    }
    if type_0 != 16 as libc::c_int ||
           strlen(b"test\x00" as *const u8 as
                      *const libc::c_char).wrapping_add(9 as libc::c_int as
                                                            libc::c_ulong) !=
               strlen(query) ||
           strstr(query, b"test\x00" as *const u8 as *const libc::c_char) !=
               query.offset(9 as libc::c_int as isize) {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if *(*__ctype_b_loc()).offset(*query.offset(i as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
           {
            return 0 as libc::c_int
        }
        i += 1
    }
    uid =
        strtol(query, 0 as *mut *mut libc::c_char, 16 as libc::c_int) as
            u32_0;
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags &
               (4 as libc::c_int | 2 as libc::c_int | 1024 as libc::c_int |
                    2048 as libc::c_int | 8 as libc::c_int | 32 as libc::c_int
                    | 8192 as libc::c_int) == 0 && uid == (*serv).uid {
            (*serv).flags |= 8192 as libc::c_int;
            check_servers();
            return 1 as libc::c_int
        }
        serv = (*serv).next
    }
    return 0 as libc::c_int;
}
