
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
use crate::defines::{stat, timespec, DnsmasqDaemon, size_t, NetAddress, IpHdr, NetAddress, C2RustUnnamed, In6Addr, __bswap_16, IPPROTO_UDP};
use std::fs::File;
use crate::util::read_write;
use std::io::{Error, Seek};
use crate::slack::{pcap_hdr_s, pcaprec_hdr_s, ip6_hdr, ip6_hdrctl, udphdr, timeval};
use crate::dnsmasq_log::my_syslog;

// static mut packet_count: u32 = 0;
/* actual length of packet */
#[no_mangle]
pub fn dump_init(daemon: &mut DnsmasqDaemon) ->Result<(), &'static str> {
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
    packet_count = 0;
    if daemon.dump_file.exists() == false {
        /* doesn't exist, create and add header */
        header.magic_number =
            0xa1b2c3d4; /* slop for IP/UDP headers */
        header.version_major = 2; /* DLT_RAW http://www.tcpdump.org/linktypes.html */
        header.version_minor = 4;
        header.thiszone = 0;
        header.sigfigs = 0;
        header.snaplen = daemon.edns_pktsz + 200;
        header.network = 101;
        let mut file = File::create(&daemon.dump_file)?;
        // todo: set 0400 perms on file
        // let perms: Permissions = Permissions();
        // file.set_permissions()
        daemon.dumpfd = file;
        read_write(daemon.dumpfd,
                   &mut header
                      ,
                   ::std::mem::size_of::<pcap_hdr_s>()
                      , 0)?;
    } else {
        daemon.dumpfd = File::open(&daemon.dump_file)?;
        read_write(daemon.dumpfd,
                   &mut header
                      ,
                   ::std::mem::size_of::<pcap_hdr_s>()
                      , 1)?;
    }

    if header.magic_number != 0xa1b2c3d4 {
        return Err("invalid header");
    } else {
        /* count existing records */
        while read_write(daemon.dumpfd,
                         &mut pcap_header ,
                         ::std::mem::size_of::<pcaprec_hdr_s>(),
                         1) != 0 {
            daemon.dumpfd.seek(pcap_header.incl_len, 1)?;
            packet_count = packet_count.wrapping_add(1)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dump_packet(mut mask: i32,
                                     mut packet:Vec<u8>,
                                     mut len: usize,
                                     mut src: NetAddress,
                                     mut dst: NetAddress) {
    let mut ip: IpHdr =
        IpHdr {ip_hl_ip_v: [0; 1],
           ip_tos: 0,
           ip_len: 0,
           ip_id: 0,
           ip_off: 0,
           ip_ttl: 0,
           ip_p: 0,
           ip_sum: 0,
           ip_src: NetAddress {s_addr: 0,},
           ip_dst: NetAddress {s_addr: 0,},};
    let mut ip6: Ip6Hdr = Default::default();
    let mut family: i32 = 0;
    let mut udp: UdpHdr = Default::default();
    let mut pcap_header: pcaprec_hdr_s =
        pcaprec_hdr_s{ts_sec: 0, ts_usec: 0, incl_len: 0, orig_len: 0,};
    let mut time: ti
    meval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut i: u32 = 0;
    let mut sum: u32 = 0;
    let mut iphdr:Vec<u8> = 0;
    let mut ipsz: usize = 0;
    let mut rc: i32 = 0;
    if daemon.dumpfd == -(1) ||
           mask & daemon.dump_mask == 0 {
        return
    }
    /* So wireshark can Id the packet. */
    udp.uh_dport = __bswap_16(53);
    udp.uh_sport = udp.uh_dport;
    if !src.is_null() {
        family = src.sa.sa_family
    } else { family = dst.sa.sa_family }
    if family == 10 {
        iphdr = &mut ip6 ;
        ipsz = ::std::mem::size_of::<ip6_hdr>();
        ip6 = Default::default();
        ip6.ip6_ctlun.ip6_un2_vfc =
            ((6) << 4) as u8;
        ip6.ip6_ctlun.ip6_un1.ip6_un1_plen =
            __bswap_16((::std::mem::size_of::<udphdr>() ).wrapping_add(len));
        ip6.ip6_ctlun.ip6_un1.ip6_un1_nxt =
            IPPROTO_UDP as u8;
        ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = 64 as u8;
        if !src.is_null() {
            memcpy(&mut ip6.ip6_src,
                   &mut src.in6.sin6_addr ,
                   16);
            udp.uh_sport = src.in6.sin6_port
        }
        if !dst.is_null() {
            memcpy(&mut ip6.ip6_dst,
                   &mut dst.in6.sin6_addr ,
                   16);
            udp.uh_dport = dst.in6.sin6_port
        }
        /* start UDP checksum */
        sum = 0;
        i = 0;
        while i < 16 {
            sum =
                (sum               libc::c_uint).wrapping_add((ip6.ip6_src.__in6_u.__u6_addr8[i
                                                                                                                    usize]
                                                     +
                                                     ((ip6.ip6_src.__in6_u.__u6_addr8[i.wrapping_add(1                                 libc::c_int                                 libc::c_uint)   usize]
                                                          ) <<
                                                          8))
                                                   )
                   ;
            sum =
                (sum               libc::c_uint).wrapping_add((ip6.ip6_dst.__in6_u.__u6_addr8[i
                                                                                                                    usize]
                                                     +
                                                     ((ip6.ip6_dst.__in6_u.__u6_addr8[i.wrapping_add(1                                 libc::c_int                                 libc::c_uint)   usize]
                                                          ) <<
                                                          8))
                                                   )
                   ;
            i =
                (i               libc::c_uint).wrapping_add(2 libc::c_uint)              u32
        }
    } else {
        iphdr = &mut ip;
        ipsz = ::std::mem::size_of::<IpHdr>();
        ip.set_ip_v(4);
        ip.set_ip_hl((::std::mem::size_of::<IpHdr>()).wrapping_div(4));
        ip.ip_len =
            __bswap_16((::std::mem::size_of::<IpHdr>() ).wrapping_add(::std::mem::size_of::<udphdr>()
                                                             ).wrapping_add(len)
                          );
        ip.ip_ttl = 64 as u8;
        ip.ip_p = IPPROTO_UDP as u8;
        if !src.is_null() {
            ip.ip_src = src.in_0.sin_addr;
            udp.uh_sport = src.in_0.sin_port
        }
        if !dst.is_null() {
            ip.ip_dst = dst.in_0.sin_addr;
            udp.uh_dport = dst.in_0.sin_port
        }
        ip.ip_sum = 0 ;
        sum = 0;
        i = 0;
        while (i) <
                  (::std::mem::size_of::<IpHdr>() ).wrapping_div(2    libc::c_ulong) {
            sum =
                (sum               libc::c_uint).wrapping_add(*(&mut ip    &mut u16) .offset(i                          isize)
                                                   )
                   ;
            i = i.wrapping_add(1)
        }
        while sum >> 16 != 0 {
            sum =
                (sum &
                     0xffff).wrapping_add(sum >> 16)
        }
        ip.ip_sum =
            if sum == 0xffff {
                sum
            } else { !sum } ;
        /* start UDP checksum */
        sum =
            ip.ip_src.s_addr &
                0xffff              libc::c_uint; /* for checksum, in case length is odd. */
        sum =
            (sum           libc::c_uint).wrapping_add(ip.ip_src.s_addr >>
                                                16 &
                                                0xffff libc::c_uint)          u32;
        sum =
            (sum           libc::c_uint).wrapping_add(ip.ip_dst.s_addr &
                                                0xffff libc::c_uint)          u32;
        sum =
            (sum           libc::c_uint).wrapping_add(ip.ip_dst.s_addr >>
                                                16 &
                                                0xffff libc::c_uint)          u32
    }
    if len & 1 != 0 {
        *(packet).offset(len) =
            0
    }
    udp.uh_sum = 0;
    udp.uh_ulen =
        __bswap_16((::std::mem::size_of::<udphdr>() )).wrapping_add(len));
    sum =
        (sum       libc::c_uint).wrapping_add(__bswap_16(IPPROTO_UDP
                                                      )                                      libc::c_uint);
    sum =
        (sum       libc::c_uint).wrapping_add(__bswap_16((::std::mem::size_of::<udphdr>()
                                                     ).wrapping_add(len)
                                                      )                                      libc::c_uint);
    i = 0;
    while (i) <
              (::std::mem::size_of::<udphdr>()).wrapping_div(2libc::c_ulong) {
        sum =
            (sum           libc::c_uint).wrapping_add(*(&mut udp                                             &mut u16) .offset(i                      isize)
                                               )          u32;
        i = i.wrapping_add(1)
    }
    i = 0;
    while (i) <
              len.wrapping_add(1).wrapping_div(2                libc::c_int
                                                                           )
          {
        sum =
            (sum           libc::c_uint).wrapping_add(*(packet                                             &mut u16) .offset(i                      isize)
                                               )          u32;
        i = i.wrapping_add(1)
    }
    while sum >> 16 != 0 {
        sum =
            (sum &
                 0xffff               libc::c_uint).wrapping_add(sum >> 16)
    }
    udp.uh_sum =
        if sum == 0xffff { sum } else { !sum }
           ;
    rc = gettimeofday(&mut time, 0);
    pcap_header.ts_sec = time.tv_sec;
    pcap_header.ts_usec = time.tv_usec;
    pcap_header.orig_len =
        ipsz.wrapping_add(::std::mem::size_of::<udphdr>()).wrapping_add(len);
    pcap_header.incl_len = pcap_header.orig_len;
    if rc == -(1) ||
           read_write(daemon.dumpfd,
                      &mut pcap_header,
                      ::std::mem::size_of::<pcaprec_hdr_s>()
                         , 0) == 0 ||
           read_write(daemon.dumpfd, iphdr,
                      ipsz, 0) == 0 ||
           read_write(daemon.dumpfd,
                      &mut udp,
                      ::std::mem::size_of::<udphdr>(), 0) == 0 ||
           read_write(daemon.dumpfd, packet,
                      len, 0) == 0 {
        my_syslog(3,
                  "failed to write packet dump");
    } else {
        packet_count = packet_count.wrapping_add(1);
        my_syslog(6,
                  "dumping UDP packet %u mask 0x%04x", packet_count, mask);
    };
}
