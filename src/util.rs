use core::time;
use std::net;

use crate::dns_protocol::{MAXDNAME, MAXLABEL};

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

/* The SURF random number generator was taken from djbdns-1.05, by
Daniel J Bernstein, which is public domain. */

/* SURF random number generator */

//  u32 seed[32];
//  u32 in[12];
//  u32 out[8];
// let mut outleft: i32 = 0;

pub fn rand_init() {
    // do any random number generation seeding/initialization
    todo!()
}

pub fn rand16() -> u16 {
    todo!()
}

pub fn rand32() -> u32 {
    todo!()
}

pub fn rand64() -> u64 {
    todo!()
}

/* returns 2 if names is OK but contains one or more underscores */
pub fn check_name(_in: &mut String) {
    /* remove trailing .
    also fail empty string and label > 63 chars */
    // dotgap: usize = 0, l = strlen(_in);
    let mut dotgap: usize = 0;
    let l = _in.len();
    let mut c: u8;
    let mut nowhite: i32 = 0;
    let mut hasuscore: i32 = 0;

    if (l == 0 || l > MAXDNAME) {
        return 0;
    }

    if (_in[l - 1] == '.') {
        _in[l - 1] = 0;
        nowhite = 1;
    }

    // for (; (c = *_in); _in++)
    for c in _in {
        if (c == '.') {
            dotgap = 0;
        } else if (dotgap + 1 > MAXLABEL) {
            dotgap += 1;
            return 0;
        } else if (isascii(c) && iscntrl(c)) {
            /* iscntrl only gives expected results for ascii */
            return 0;
        } else if (!isascii(c)) {
            return 0;
        } else if (c != ' ') {
            nowhite = 1;
            if (c == '_') {
                hasuscore = 1;
            }
        }
    }

    if (!nowhite) {
        return 0;
    }

    // return hasuscore ? 2 : 1;
    if hasuscore {
        return 2;
    } else {
        return 1;
    }
}

/* Hostnames have a more limited valid charset than domain names
so check for legal char a-z A-Z 0-9 - _
Note that this may receive a FQDN, so only check the first label
for the tighter criteria. */
pub fn legal_hostname(name: &mut String) -> i32 {
    let mut c: u8;
    let mut first: i32;

    if (!check_name(name)) {
        return 0;
    }

    //   for (first = 1; (c = *name); name++, first = 0)
    first = 1;
    for c in name
    /* check for legal char a-z A-Z 0-9 - _ . */
    {
        if ((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')) {
            continue;
        }

        if (!first && (c == '-' || c == '_')) {
            continue;
        }

        /* end of hostname part */
        if (c == '.') {
            return 1;
        }

        return 0;
    }

    return 1;
}

pub fn canonicalise(_in: &mut String, nomem: &mut i32) -> Option<String> {
    //   char *ret = NULL;
    let mut ret: String;
    let mut rc: i32;

    if (nomem) {
        *nomem = 0;
    }

    if !(rc = check_name(_in)) {
        return None;
    }

    /* older libidn2 strips underscores, so don't do IDN processing
    if the name has an underscore (check_name() returned 2) */
    if (rc != 2) {
        rc = idn2_to_ascii_lz(_in, &ret, IDN2_NONTRANSITIONAL);
        if (rc == IDN2_DISALLOWED) {
            rc = idn2_to_ascii_lz(_in, &ret, IDN2_TRANSITIONAL);
            rc = idna_to_ascii_lz(_in, &ret, 0);
        }
        if (rc != IDNA_SUCCESS) {
            if (ret) {
                free(ret);
            }

            if (nomem && (rc == IDNA_MALLOC_ERROR || rc == IDNA_DLOPEN_ERROR)) {
                my_syslog(LOG_ERR, format!("failed to allocate memory"));
                *nomem = 1;
            }

            return None;
        }

        return Some(ret);
    }

    if (ret = whine_malloc(strlen(_in) + 1)) {
        strcpy(ret, _in);
    } else if (nomem) {
        *nomem = 1;
    }

    return Some(ret);
}

pub fn do_rfc1035_name(p: &mut String, sval: &mut String, limit: &mut String) -> Option<String> {
    // needs work refactoring
    todo!();
    let mut j: i32;
    let mut p_index: usize = 0;
    let mut s_index: usize = 0;

    while (sval && *sval) {
        //unsigned char *cp = p +=1;
        // let mut cp: &mut Vec<u8> = p.offset(1);

        if (limit && p > limit) {
            return None;
        }

        // for (j = 0; *sval && (*sval != '.'); sval++, j++)
        let mut j = 0;
        for s in sval {
            if (limit && p + 1 > limit) {
                return None;
            }

            if (daemon.opt_dnssec_valid && s == NAME_ESCAPE) {
                s_index += 1;
                p[p_index] = (sval[s_index]) - 1;
            } else {
                p[p_index] = s;
                p += 1;
            }

            if s == '.' {
                break;
            }

            j += 1;
        }

        *cp = j;

        if (*sval) {
            sval += 1;
        }
    }

    return p;
}

/* for use during startup */
// safe_malloc: Vec<u8>(size: usize)
// {
//   ret: Vec<u8> = calloc(1, size);

//   if (!ret)
//     die(format!("could not get memory"), NULL, EC_NOMEM);

//   return ret;
// }

/* Ensure limited size string is always terminated.
 * Can be replaced by ()strlcpy() on some platforms */
// void safe_strncpy(dest: &mut String, const src: &mut String, size: usize)
// {
//   if (size != 0)
//     {
//       dest[size-1] = '\0';
//       strncpy(dest, src, size-1);
//     }
// }

pub fn safe_pipe(fd: &mut i32, read_noblock: i32) {
    if (pipe(fd) == -1 || !fix_fd(fd[1]) || (read_noblock && !fix_fd(fd[0]))) {
        panic!(format!("cannot create pipe"));
    }
}

// whine_malloc: Vec<u8>(size: usize)
// {
//   ret: Vec<u8> = calloc(1, size);

//   if (!ret)
//     my_syslog(LOG_ERR, format!("failed to allocate {} bytes"),  size);

//   return ret;
// }

// sockaddr_isequal: i32(s1: &mut net::IpAddr, s2: &mut net::IpAddr)
// {
//   if (s1.sa.sa_family == s2.sa.sa_family)
//     {
//       if (s1.sa.sa_family == AF_INET &&
// 	  s1._in.sin_port == s2._in.sin_port &&
// 	  s1._in.sin_addr.s_addr == s2._in.sin_addr.s_addr)
// 	return 1;

//       if (s1.sa.sa_family == AF_INET6 &&
// 	  s1.in6.sin6_port == s2.in6.sin6_port &&
// 	  s1.in6.sin6_scope_id == s2.in6.sin6_scope_id &&
// 	  IN6_ARE_ADDR_EQUAL(&s1.in6.sin6_addr, &s2.in6.sin6_addr))
// 	return 1;
//     }
//   return 0;
// }

// pub fn sa_len(addr: &mut net::IpAddr) -> i32
// {
//  HAVE_SOCKADDR_SA_LEN
//   return addr.sa.sa_len;

//   if (addr.sa.sa_family == AF_INET6)
//     return sizeof(addr.in6);
//   else
//     return sizeof(addr.in);

// }

/* don't use strcasecmp and friends here - they may be messed up by LOCALE */
// hostname_isequal: i32(const a: &mut String, const char *b)
// {
//   unsigned c1: i32, c2;

//   do {
//     c1 =  *a +=1;
//     c2 =  *b +=1;

//     if (c1 >= 'A' && c1 <= 'Z')
//       c1 += 'a' - 'A';
//     if (c2 >= 'A' && c2 <= 'Z')
//       c2 += 'a' - 'A';

//     if (c1 != c2)
//       return 0;
//   } while (c1);

//   return 1;
// }
pub fn hostname_isequal(a: &String, b: &String) -> bool {
    a == b
}

/* is b equal to or a subdomain of a return 2 for equal, 1 for subdomain */
pub fn hostname_issubdomain(a: &mut String, b: &mut String) -> i32 {
    todo!()
}

// dnsmasq_time: time::Instant()
// {

//   struct tms dummy;
//    long tps = 0;

//   if (tps == 0)
//     tps = sysconf(_SC_CLK_TCK);

//   return (time_t)(times(&dummy)/tps);

//   return time(NULL);

// }

pub fn netmask_length(mask: &net::IpAddr) -> u32 {
    let mut zero_count: i32 = 0;

    while (0x0 == (mask.s_addr & 0x1) && zero_count < 32) {
        mask.s_addr >>= 1;
        zero_count += 1;
    }

    return 32 - zero_count;
}

pub fn is_same_net(a: &net::IpAddr, b: &net::IpAddr, mask: &net::IpAddr) -> bool {
    //   return (a.s_addr & mask.s_addr) == (b.s_addr & mask.s_addr);
    todo!()
}

// is_same_net6: i32(a: &mut net::IpAddr, b: &mut net::IpAddr, prefixlen: i32)
// {
//   pfbytes: i32 = prefixlen >> 3;
//   pfbits: i32 = prefixlen & 7;

//   if (memcmp(&a.s6_addr, &b.s6_addr, pfbytes) != 0)
//     return 0;

//   if (pfbits == 0 ||
//       (a.s6_addr[pfbytes] >> (8 - pfbits) == b.s6_addr[pfbytes] >> (8 - pfbits)))
//     return 1;

//   return 0;
// }

/* return least significant 64 bits if IPv6 address */
pub fn addr6part(addr: &net::IpAddr) -> u64 {
    todo!()
}

pub fn setaddr6part(addr: &mut net::IpAddr, host: u64) {
    //   let mut i: i32;

    //   for (i = 15; i >= 8; i--)
    //     {
    //       addr.s6_addr[i] = host;
    //       host = host >> 8;
    //     }
    todo!()
}

/* returns port number from address */
// prettyprint_addr: i32(addr: &mut net::IpAddr, buf: &mut String)
// {
//   let mut port: i32 = 0;

//   if (addr.sa.sa_family == AF_INET)
//     {
//       inet_ntop(AF_INET, &addr.in.sin_addr, buf, ADDRSTRLEN);
//       port = ntohs(addr.in.sin_port);
//     }
//   else if (addr.sa.sa_family == AF_INET6)
//     {
//       char name[IF_NAMESIZE];
//       inet_ntop(AF_INET6, &addr.in6.sin6_addr, buf, ADDRSTRLEN);
//       if (addr.in6.sin6_scope_id != 0 &&
// 	  if_indextoname(addr.in6.sin6_scope_id, name) &&
// 	  strlen(buf) + strlen(name) + 2 <= ADDRSTRLEN)
// 	{
// 	  strcat(buf, "%");
// 	  strcat(buf, name);
// 	}
//       port = ntohs(addr.in6.sin6_port);
//     }

//   return port;
// }

// void prettyprint_time(buf: &mut String, unsigned t: i32)
// {
//   if (t == 0xffffffff)
//     sprintf(buf, format!("infinite"));
//   else
//     {
//       unsigned x: i32, p = 0;
//        if ((x = t/86400))
// 	p += sprintf(&buf[p], "{}", x);
//        if ((x = (t/3600)%24))
// 	p += sprintf(&buf[p], "{}h", x);
//       if ((x = (t/60)%60))
// 	p += sprintf(&buf[p], "{}m", x);
//       if ((x = t%60))
// 	p += sprintf(&buf[p], "{}", x);
//     }
// }

/* in may equal out, when maxlen may be -1 (No max len).
Return -1 for extraneous no-hex chars found. */
// parse_hex: i32(in: &mut String, out: &mut Vec<u8>, maxlen: i32,
// 	      wildcard_mask: &mut u32, mac_type: &i32)
// {
//   done: i32 = 0, mask = 0, i = 0;
//   char *r;

//   if (mac_type)
//     *mac_type = 0;

//   while (!done && (maxlen == -1 || i < maxlen))
//     {
//       for (r = in; *r != 0 && *r != ':' && *r != '-' && *r != ' '; r++)
// 	if (*r != '*' && !isxdigit(*r))
// 	  return -1;

//       if (*r == 0)
// 	done = 1;

//       if (r != in )
// 	{
// 	  if (*r == '-' && i == 0 && mac_type)
// 	   {
// 	      *r = 0;
// 	      *mac_type = strtol(in, NULL, 16);
// 	      mac_type = NULL;
// 	   }
// 	  else
// 	    {
// 	      *r = 0;
// 	      if (strcmp(in, "*") == 0)
// 		{
// 		  mask = (mask << 1) | 1;
// 		  i +=1;
// 		}
// 	      else
// 		{
// 		  j: i32, bytes = (1 + (r - in))/2;
// 		  for (j = 0; j < bytes; j++)
// 		    {
// 		      char sav = sav;
// 		      if (j < bytes - 1)
// 			{
// 			  sav = in[(j+1)*2];
// 			  in[(j+1)*2] = 0;
// 			}
// 		      /* checks above allow mix of hexdigit and *, which
// 			 is illegal. */
// 		      if (strchr(&in[j*2], '*'))
// 			return -1;
// 		      out[i] = strtol(&in[j*2], NULL, 16);
// 		      mask = mask << 1;
// 		      if (++i == maxlen)
// 			break;
// 		      if (j < bytes - 1)
// 			in[(j+1)*2] = sav;
// 		    }
// 		}
// 	    }
// 	}
//       in = r+1;
//     }

//   if (wildcard_mask)
//     *wildcard_mask = mask;

//   return i;
// }

/* return 0 for no match, or (no matched octets) + 1 */
// memcmp_masked: i32(a: &mut Vec<u8>, b: &mut Vec<u8>, len: i32, unsigned mask: i32)
// {
//   i: i32, count;
//   for (count = 1, i = len - 1; i >= 0; i--, mask = mask >> 1)
//     if (!(mask & 1))
//       {
// 	if (a[i] == b[i])
// 	  count +=1;
// 	else
// 	  return 0;
//       }
//   return count;
// }

/* _note_ may copy buffer */
// expand_buf: i32(struct iovec *iov, size: usize)
// {
//   new: Vec<u8>;

//   if (size <= iov.iov_len)
//     return 1;

//   if (!(new = whine_malloc(size)))
//     {
//       errno = ENOMEM;
//       return 0;
//     }

//   if (iov.iov_base)
//     {
//       memcpy(new, iov.iov_base, iov.iov_len);
//       free(iov.iov_base);
//     }

//   iov.iov_base = new;
//   iov.iov_len = size;

//   return 1;
// }

pub fn print_mac(mac: &[u8], len: usize) -> String {
    todo!()
}

/* rc is return from sendto and friends.
Return 1 if we should retry.
Set errno to zero if we succeeded. */
pub fn retry_send(rc: isize) -> i32 {
    let mut retries: i32 = 0;
    //   struct timespec waiter;
    let mut waiter: time::Duration;

    if (rc != -1) {
        retries = 0;
        errno = 0;
        return 0;
    }

    /* Linux kernels can return EAGAIN in perpetuity when calling
    sendmsg() and the relevant interface has gone. Here we loop
    retrying in EAGAIN for 1 second max, to avoid this hanging
    dnsmasq. */

    if (errno == EAGAIN || errno == EWOULDBLOCK) {
        waiter.tv_sec = 0;
        waiter.tv_nsec = 10000;
        nanosleep(&waiter, NULL);
        retries += 1;
        if (retries < 1000) {
            return 1;
        }
    }

    retries = 0;

    if (errno == EINTR) {
        return 1;
    }

    return 0;
}

pub fn read_write(fd: i32, packet: &mut Vec<u8>, size: i32, rw: i32) -> i32 {
    //   sn: usize, done;
    let mut n: isize;
    let mut done: isize;

    //   for (done = 0; done < size; done += n)
    while (done < size) {
        while {
            if (rw) {
                n = read(fd, &packet[done], (size - done));
            } else {
                n = write(fd, &packet[done], (size - done));
            }

            if (n == 0) {
                return 0;
            }
            retry_send(n) || errno == ENOMEM || errno == ENOBUFS
        } {}

        if (errno != 0) {
            return 0;
        }
        done += n
    }

    return 1;
}

/* close all fds except STDIN, STDOUT and STDERR, spare1, spare2 and spare3 */
pub fn close_fds(max_fd: i32, spare1: i32, spare2: i32, spare3: i32) {
    /* On Linux, use the /proc/ filesystem to find which files
    are actually open, rather than iterate over the whole space,
    for efficiency reasons. If this fails we drop back to the dumb code. */

    //   DIR *d;
    let d: DIR;

    if (d = opendir("/proc/self/fd")) {
        let mut de: dirent;

        while (de = readdir(d)) {
            let fd: i32;
            char * e = NULL;

            errno = 0;
            fd = strtol(de.d_name, &e, 10);

            if (errno != 0
                || !e
                || *e
                || fd == dirfd(d)
                || fd == STDOUT_FILENO
                || fd == STDERR_FILENO
                || fd == STDIN_FILENO
                || fd == spare1
                || fd == spare2
                || fd == spare3)
            {
                continue;
            }

            close(fd);
        }

        closedir(d);
        return;
    }

    /* fallback, dumb code. */
    //   for (max_fd--; max_fd >= 0; max_fd--)
    while (max_fd >= 0) {
        if (max_fd != STDOUT_FILENO
            && max_fd != STDERR_FILENO
            && max_fd != STDIN_FILENO
            && max_fd != spare1
            && max_fd != spare2
            && max_fd != spare3)
        {
            close(max_fd);
        }
        max_fd -= 1
    }
}

// /* Basically match a string value against a wildcard pattern.  */
// pub fn wildcard_match(wildcard: &mut String, match: &mut String) -> i32
// {
//   while (*wildcard && *match)
//     {
//       if (*wildcard == '*')
//         return 1;

//       if (*wildcard != *match)
//         return 0;

//       ++wildcard;
//       ++match;
//     }

//   return *wildcard == *match;
// }

/* The same but comparing a maximum of NUM characters, like strncmp.  */
// wildcard_matchn: i32(const char* wildcard, const char* match, num: i32)
// {
//   while (*wildcard && *match && num)
//     {
//       if (*wildcard == '*')
//         return 1;

//       if (*wildcard != *match)
//         return 0;

//       ++wildcard;
//       ++match;
//       --num;
//     }

//   return (!num) || (*wildcard == *match);
// }

pub fn kernel_version() -> i32 {
    //   struct utsname utsname;
    //   let mut version: i32;
    //   char *split;

    //   if (uname(&utsname) < 0)
    //     die(format!("failed to find kernel version: {}"), NULL, EC_MISC);

    //   split = strtok(utsname.release, ".");
    //   version = (split ? atoi(split) : 0);
    //   split = strtok(NULL, ".");
    //   version = version * 256 + (split ? atoi(split) : 0);
    //   split = strtok(NULL, ".");
    //   return version * 256 + (split ? atoi(split) : 0);
    todo!()
}

pub fn find_subnet() {
    unimplemented!()
}

pub fn GETSHORT(dest: &mut u32, buff: &[u8]) {
    unimplemented!()
}

pub fn log_query(
    daemon: &mut DnsmasqDaemon,
    mut flags: u32,
    mut name: &String,
    mut addr: &NetAddress,
    mut arg: Option<&String>,
) {
    unimplemented!()
}
