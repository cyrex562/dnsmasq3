use crate::defines::{Server, DnsmasqDaemon, RandFd, ConstNetAddressArg, DnsHeader, _ISXDIGIT};
use crate::forward::{allocate_rfd, free_rfd};
use crate::util::{retry_send, sa_len, rand16};
use crate::network::check_servers;


pub fn loop_send_probes() {
    let mut serv: Server = 0;
    if dnsmasq_daemon.options[(50).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (50).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           == 0 {
        return
    }
    let mut current_block_7: u64;
    /* Loop through all upstream servers not for particular domains, and send a query to that server which is
      identifiable, via the uid. If we see that query back again, then the server is looping, and we should not use it. */
    serv = dnsmasq_daemon.servers;
    while !serv.is_null() {
        if serv.flags &
               (4 | 2 | 1024 |
                    2048 | 8 | 32
                    | 8192) == 0 {
            let mut len: isize = loop_make_probe(serv.uid);
            let mut fd: i32 = 0;
            let mut rfd: RandFd = 0 ;
            if !serv.sfd.is_null() {
                fd = (*serv.sfd).fd;
                current_block_7 = 2868539653012386629;
            } else {
                rfd = allocate_rfd(serv.addr.sa.sa_family);
                if rfd.is_null() {
                    current_block_7 = 12517898123489920830;
                } else {
                    fd = rfd.fd;
                    current_block_7 = 2868539653012386629;
                }
            }
            match current_block_7 {
                12517898123489920830 => { }
                _ => {
                    while retry_send(sendto(fd,
                                            dnsmasq_daemon.packet ,
                                            len , 0,
                                            ConstNetAddressArg::new(),
                                            sa_len(&mut serv.addr))) != 0 {
                    }
                    free_rfd(rfd);
                }
            }
        }
        serv = serv.next
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
fn loop_make_probe(mut uid: u32) -> isize {
    let mut header: DnsHeader =
        dnsmasq_daemon.packet ;
    let mut p: DnsHeader = header.offset(1);
    /* packet buffer overwritten */
    dnsmasq_daemon.srv_save = 0; /* Add terminating zero */
    header.id = rand16(); /* log new state */
    header.arcount = __bswap_16(0);
    header.nscount = header.arcount;
    header.ancount = header.nscount;
    header.qdcount = __bswap_16(1);
    header.hb3 = 0x1 as u8;
    header.hb4 = 0 as u8;
    header.hb3 =
        (header.hb3 & !(0x78) |
             0) as u8;
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = 8;
    sprintf(p ,
            "%.8x" , uid);
    p = p.offset(8);
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = "test".to_string().len();
    p = "test";    
    p = p.offset(strlen("test" ).wrapping_add(1));
    let mut t_s: u16 = 16;
    let mut t_cp = p;
    let fresh8 = t_cp;
    t_cp = t_cp.offset(1);
    *fresh8 = (t_s >> 8);
    *t_cp = t_s;
    p = p.offset(2);
    let mut t_s_0: u16 = 1;
    let mut t_cp_0 = p;
    let fresh9 = t_cp_0;
    t_cp_0 = t_cp_0.offset(1);
    *fresh9 = (t_s_0 >> 8);
    *t_cp_0 = t_s_0;
    p = p.offset(2);
    return p.wrapping_offset_from(header) ;
}

pub fn detect_loop(mut query: &mut String,
                                     mut type_0: i32) -> i32 {
    let mut i: i32 = 0;
    let mut uid: u32 = 0;
    let mut serv: Server = 0;
    if dnsmasq_daemon.options[(50).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (50).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           == 0 {
        return 0
    }
    if type_0 != 16 ||
           strlen("test").wrapping_add(9  ) !=
               strlen(query) ||
           strstr(query, "test" ) !=
               query.offset(9) {
        return 0
    }
    i = 0;
    while i < 8 {
        if *(*__ctype_b_loc()).offset(*query.offset(i)
                                         ) &
               _ISXDIGIT  == 0
           {
            return 0
        }
        i += 1
    }
    uid = u32::from_le_bytes(query.as_bytes());
    serv = dnsmasq_daemon.servers;
    while !serv.is_null() {
        if serv.flags &
               (4 | 2 | 1024 |
                    2048 | 8 | 32
                    | 8192) == 0 && uid == serv.uid {
            serv.flags |= 8192;
            check_servers();
            return 1
        }
        serv = serv.next
    }
    return 0;
}
