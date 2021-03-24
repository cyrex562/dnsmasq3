
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
static mut packet_count: u32_0 = 0;
/* actual length of packet */
#[no_mangle]
pub unsafe extern "C" fn dump_init() {
    let mut buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut header: pcap_hdr_s =
        pcap_hdr_s{magic_number: 0,
                   version_major: 0,
                   version_minor: 0,
                   thiszone: 0,
                   sigfigs: 0,
                   snaplen: 0,
                   network: 0,};
    let mut pcap_header: pcaprec_hdr_s =
        pcaprec_hdr_s{ts_sec: 0, ts_usec: 0, incl_len: 0, orig_len: 0,};
    packet_count = 0 as libc::c_int as u32_0;
    if stat((*dnsmasq_daemon).dump_file, &mut buf) == -(1 as libc::c_int) {
        /* doesn't exist, create and add header */
        header.magic_number =
            0xa1b2c3d4 as libc::c_uint; /* slop for IP/UDP headers */
        header.version_major =
            2 as libc::c_int as
                u16_0; /* DLT_RAW http://www.tcpdump.org/linktypes.html */
        header.version_minor = 4 as libc::c_int as u16_0;
        header.thiszone = 0 as libc::c_int as u32_0;
        header.sigfigs = 0 as libc::c_int as u32_0;
        header.snaplen =
            ((*dnsmasq_daemon).edns_pktsz as libc::c_int + 200 as libc::c_int)
                as u32_0;
        header.network = 101 as libc::c_int as u32_0;
        if *__errno_location() != 2 as libc::c_int ||
               {
                   (*dnsmasq_daemon).dumpfd =
                       creat((*dnsmasq_daemon).dump_file,
                             (0o400 as libc::c_int | 0o200 as libc::c_int) as
                                 mode_t);
                   ((*dnsmasq_daemon).dumpfd) == -(1 as libc::c_int)
               } ||
               read_write((*dnsmasq_daemon).dumpfd,
                          &mut header as *mut pcap_hdr_s as *mut libc::c_void
                              as *mut libc::c_uchar,
                          ::std::mem::size_of::<pcap_hdr_s>() as libc::c_ulong
                              as libc::c_int, 0 as libc::c_int) == 0 {
            die(b"cannot create %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).dump_file, 3 as libc::c_int);
        }
    } else {
        (*dnsmasq_daemon).dumpfd =
            open((*dnsmasq_daemon).dump_file,
                 0o2000 as libc::c_int | 0o2 as libc::c_int);
        if (*dnsmasq_daemon).dumpfd == -(1 as libc::c_int) ||
               read_write((*dnsmasq_daemon).dumpfd,
                          &mut header as *mut pcap_hdr_s as *mut libc::c_void
                              as *mut libc::c_uchar,
                          ::std::mem::size_of::<pcap_hdr_s>() as libc::c_ulong
                              as libc::c_int, 1 as libc::c_int) == 0 {
            die(b"cannot access %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).dump_file, 3 as libc::c_int);
        } else {
            if header.magic_number != 0xa1b2c3d4 as libc::c_uint {
                die(b"bad header in %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    (*dnsmasq_daemon).dump_file, 3 as libc::c_int);
            } else {
                /* count existing records */
                while read_write((*dnsmasq_daemon).dumpfd,
                                 &mut pcap_header as *mut pcaprec_hdr_s as
                                     *mut libc::c_void as *mut libc::c_uchar,
                                 ::std::mem::size_of::<pcaprec_hdr_s>() as
                                     libc::c_ulong as libc::c_int,
                                 1 as libc::c_int) != 0 {
                    lseek((*dnsmasq_daemon).dumpfd,
                          pcap_header.incl_len as __off64_t,
                          1 as libc::c_int);
                    packet_count = packet_count.wrapping_add(1)
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump_packet(mut mask: libc::c_int,
                                     mut packet: *mut libc::c_void,
                                     mut len: size_t,
                                     mut src: *mut mysockaddr,
                                     mut dst: *mut mysockaddr) {
    let mut ip: ip =
        ip{ip_hl_ip_v: [0; 1],
           ip_tos: 0,
           ip_len: 0,
           ip_id: 0,
           ip_off: 0,
           ip_ttl: 0,
           ip_p: 0,
           ip_sum: 0,
           ip_src: in_addr{s_addr: 0,},
           ip_dst: in_addr{s_addr: 0,},};
    let mut ip6: ip6_hdr =
        ip6_hdr{ip6_ctlun:
                    C2RustUnnamed_1{ip6_un1:
                                        ip6_hdrctl{ip6_un1_flow: 0,
                                                   ip6_un1_plen: 0,
                                                   ip6_un1_nxt: 0,
                                                   ip6_un1_hlim: 0,},},
                ip6_src:
                    in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                ip6_dst:
                    in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},};
    let mut family: libc::c_int = 0;
    let mut udp: udphdr =
        udphdr{uh_sport: 0, uh_dport: 0, uh_ulen: 0, uh_sum: 0,};
    let mut pcap_header: pcaprec_hdr_s =
        pcaprec_hdr_s{ts_sec: 0, ts_usec: 0, incl_len: 0, orig_len: 0,};
    let mut time: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut i: u32_0 = 0;
    let mut sum: u32_0 = 0;
    let mut iphdr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ipsz: size_t = 0;
    let mut rc: libc::c_int = 0;
    if (*dnsmasq_daemon).dumpfd == -(1 as libc::c_int) ||
           mask & (*dnsmasq_daemon).dump_mask == 0 {
        return
    }
    /* So wireshark can Id the packet. */
    udp.uh_dport = __bswap_16(53 as libc::c_int as __uint16_t);
    udp.uh_sport = udp.uh_dport;
    if !src.is_null() {
        family = (*src).sa.sa_family as libc::c_int
    } else { family = (*dst).sa.sa_family as libc::c_int }
    if family == 10 as libc::c_int {
        iphdr = &mut ip6 as *mut ip6_hdr as *mut libc::c_void;
        ipsz = ::std::mem::size_of::<ip6_hdr>() as libc::c_ulong;
        memset(&mut ip6 as *mut ip6_hdr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<ip6_hdr>() as libc::c_ulong);
        ip6.ip6_ctlun.ip6_un2_vfc =
            ((6 as libc::c_int) << 4 as libc::c_int) as uint8_t;
        ip6.ip6_ctlun.ip6_un1.ip6_un1_plen =
            __bswap_16((::std::mem::size_of::<udphdr>() as
                            libc::c_ulong).wrapping_add(len) as __uint16_t);
        ip6.ip6_ctlun.ip6_un1.ip6_un1_nxt =
            IPPROTO_UDP as libc::c_int as uint8_t;
        ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = 64 as libc::c_int as uint8_t;
        if !src.is_null() {
            memcpy(&mut ip6.ip6_src as *mut in6_addr as *mut libc::c_void,
                   &mut (*src).in6.sin6_addr as *mut in6_addr as
                       *const libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
            udp.uh_sport = (*src).in6.sin6_port
        }
        if !dst.is_null() {
            memcpy(&mut ip6.ip6_dst as *mut in6_addr as *mut libc::c_void,
                   &mut (*dst).in6.sin6_addr as *mut in6_addr as
                       *const libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
            udp.uh_dport = (*dst).in6.sin6_port
        }
        /* start UDP checksum */
        sum = 0 as libc::c_int as u32_0;
        i = 0 as libc::c_int as u32_0;
        while i < 16 as libc::c_int as libc::c_uint {
            sum =
                (sum as
                     libc::c_uint).wrapping_add((ip6.ip6_src.__in6_u.__u6_addr8[i
                                                                                    as
                                                                                    usize]
                                                     as libc::c_int +
                                                     ((ip6.ip6_src.__in6_u.__u6_addr8[i.wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                                          as
                                                                                          usize]
                                                           as libc::c_int) <<
                                                          8 as libc::c_int))
                                                    as libc::c_uint) as u32_0
                    as u32_0;
            sum =
                (sum as
                     libc::c_uint).wrapping_add((ip6.ip6_dst.__in6_u.__u6_addr8[i
                                                                                    as
                                                                                    usize]
                                                     as libc::c_int +
                                                     ((ip6.ip6_dst.__in6_u.__u6_addr8[i.wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                                          as
                                                                                          usize]
                                                           as libc::c_int) <<
                                                          8 as libc::c_int))
                                                    as libc::c_uint) as u32_0
                    as u32_0;
            i =
                (i as
                     libc::c_uint).wrapping_add(2 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
    } else {
        iphdr = &mut ip as *mut ip as *mut libc::c_void;
        ipsz = ::std::mem::size_of::<ip>() as libc::c_ulong;
        memset(&mut ip as *mut ip as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<ip>() as libc::c_ulong);
        ip.set_ip_v(4 as libc::c_int as libc::c_uint);
        ip.set_ip_hl((::std::mem::size_of::<ip>() as
                          libc::c_ulong).wrapping_div(4 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_uint);
        ip.ip_len =
            __bswap_16((::std::mem::size_of::<ip>() as
                            libc::c_ulong).wrapping_add(::std::mem::size_of::<udphdr>()
                                                            as
                                                            libc::c_ulong).wrapping_add(len)
                           as __uint16_t);
        ip.ip_ttl = 64 as libc::c_int as uint8_t;
        ip.ip_p = IPPROTO_UDP as libc::c_int as uint8_t;
        if !src.is_null() {
            ip.ip_src = (*src).in_0.sin_addr;
            udp.uh_sport = (*src).in_0.sin_port
        }
        if !dst.is_null() {
            ip.ip_dst = (*dst).in_0.sin_addr;
            udp.uh_dport = (*dst).in_0.sin_port
        }
        ip.ip_sum = 0 as libc::c_int as libc::c_ushort;
        sum = 0 as libc::c_int as u32_0;
        i = 0 as libc::c_int as u32_0;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<ip>() as
                       libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                       libc::c_ulong) {
            sum =
                (sum as
                     libc::c_uint).wrapping_add(*(&mut ip as *mut ip as
                                                      *mut u16_0).offset(i as
                                                                             isize)
                                                    as libc::c_uint) as u32_0
                    as u32_0;
            i = i.wrapping_add(1)
        }
        while sum >> 16 as libc::c_int != 0 {
            sum =
                (sum &
                     0xffff as libc::c_int as
                         libc::c_uint).wrapping_add(sum >> 16 as libc::c_int)
        }
        ip.ip_sum =
            if sum == 0xffff as libc::c_int as libc::c_uint {
                sum
            } else { !sum } as libc::c_ushort;
        /* start UDP checksum */
        sum =
            ip.ip_src.s_addr &
                0xffff as libc::c_int as
                    libc::c_uint; /* for checksum, in case length is odd. */
        sum =
            (sum as
                 libc::c_uint).wrapping_add(ip.ip_src.s_addr >>
                                                16 as libc::c_int &
                                                0xffff as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                u32_0;
        sum =
            (sum as
                 libc::c_uint).wrapping_add(ip.ip_dst.s_addr &
                                                0xffff as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                u32_0;
        sum =
            (sum as
                 libc::c_uint).wrapping_add(ip.ip_dst.s_addr >>
                                                16 as libc::c_int &
                                                0xffff as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                u32_0
    }
    if len & 1 as libc::c_int as libc::c_ulong != 0 {
        *(packet as *mut libc::c_uchar).offset(len as isize) =
            0 as libc::c_int as libc::c_uchar
    }
    udp.uh_sum = 0 as libc::c_int as u16_0;
    udp.uh_ulen =
        __bswap_16((::std::mem::size_of::<udphdr>() as
                        libc::c_ulong).wrapping_add(len) as __uint16_t);
    sum =
        (sum as
             libc::c_uint).wrapping_add(__bswap_16(IPPROTO_UDP as libc::c_int
                                                       as __uint16_t) as
                                            libc::c_uint) as u32_0 as u32_0;
    sum =
        (sum as
             libc::c_uint).wrapping_add(__bswap_16((::std::mem::size_of::<udphdr>()
                                                        as
                                                        libc::c_ulong).wrapping_add(len)
                                                       as __uint16_t) as
                                            libc::c_uint) as u32_0 as u32_0;
    i = 0 as libc::c_int as u32_0;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<udphdr>() as
                   libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                   libc::c_ulong) {
        sum =
            (sum as
                 libc::c_uint).wrapping_add(*(&mut udp as *mut udphdr as
                                                  *mut u16_0).offset(i as
                                                                         isize)
                                                as libc::c_uint) as u32_0 as
                u32_0;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as u32_0;
    while (i as libc::c_ulong) <
              len.wrapping_add(1 as libc::c_int as
                                   libc::c_ulong).wrapping_div(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong)
          {
        sum =
            (sum as
                 libc::c_uint).wrapping_add(*(packet as
                                                  *mut u16_0).offset(i as
                                                                         isize)
                                                as libc::c_uint) as u32_0 as
                u32_0;
        i = i.wrapping_add(1)
    }
    while sum >> 16 as libc::c_int != 0 {
        sum =
            (sum &
                 0xffff as libc::c_int as
                     libc::c_uint).wrapping_add(sum >> 16 as libc::c_int)
    }
    udp.uh_sum =
        if sum == 0xffff as libc::c_int as libc::c_uint { sum } else { !sum }
            as u16_0;
    rc = gettimeofday(&mut time, 0 as *mut libc::c_void);
    pcap_header.ts_sec = time.tv_sec as u32_0;
    pcap_header.ts_usec = time.tv_usec as u32_0;
    pcap_header.orig_len =
        ipsz.wrapping_add(::std::mem::size_of::<udphdr>() as
                              libc::c_ulong).wrapping_add(len) as u32_0;
    pcap_header.incl_len = pcap_header.orig_len;
    if rc == -(1 as libc::c_int) ||
           read_write((*dnsmasq_daemon).dumpfd,
                      &mut pcap_header as *mut pcaprec_hdr_s as
                          *mut libc::c_void as *mut libc::c_uchar,
                      ::std::mem::size_of::<pcaprec_hdr_s>() as libc::c_ulong
                          as libc::c_int, 0 as libc::c_int) == 0 ||
           read_write((*dnsmasq_daemon).dumpfd, iphdr as *mut libc::c_uchar,
                      ipsz as libc::c_int, 0 as libc::c_int) == 0 ||
           read_write((*dnsmasq_daemon).dumpfd,
                      &mut udp as *mut udphdr as *mut libc::c_void as
                          *mut libc::c_uchar,
                      ::std::mem::size_of::<udphdr>() as libc::c_ulong as
                          libc::c_int, 0 as libc::c_int) == 0 ||
           read_write((*dnsmasq_daemon).dumpfd, packet as *mut libc::c_uchar,
                      len as libc::c_int, 0 as libc::c_int) == 0 {
        my_syslog(3 as libc::c_int,
                  b"failed to write packet dump\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        packet_count = packet_count.wrapping_add(1);
        my_syslog(6 as libc::c_int,
                  b"dumping UDP packet %u mask 0x%04x\x00" as *const u8 as
                      *const libc::c_char, packet_count, mask);
    };
}
