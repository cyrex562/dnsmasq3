
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
/* For MAXSYMLINKS */
/* the strategy is to set an inotify on the directories containing
   resolv files, for any files in the directory which are close-write 
   or moved into the directory.
   
   When either of those happen, we look to see if the file involved
   is actually a resolv-file, and if so, call poll-resolv with
   the "force" argument, to ensure it's read.

   This adds one new error condition: the directories containing
   all specified resolv-files must exist at start-up, even if the actual
   files don't. 
*/

use crate::dnsmasq_log::{die, my_syslog};
use crate::defines::{Resolvc, DnsmasqDaemon, Crec, HostsFile, DIR, stat, timespec, time::Instant};
use crate::slack::{inotify_event, IN_NONBLOCK, IN_CLOEXEC, dirent};
use crate::cache::read_hostsfile;
use crate::option::option_read_dynfile;
use std::fs::read;
use crate::dhcp_common::dhcp_update_configs;
use crate::lease::{lease_update_from_configs, lease_update_file, lease_update_dns};

static mut inotify_buffer: &mut String =
    0 ;
/* If path is a symbolic link, return the path it
   points to, made absolute if relative.
   If path doesn't exist or is not a symlink, return NULL.
   Return value is malloc'ed */
unsafe extern "C" fn my_readlink(mut path: &mut String)
 -> &mut String {
    let mut rc: isize = 0;
    let mut size: isize = 64;
    let mut buf: &mut String = 0 ;
    loop  {
        // buf = safe_malloc(size ) ;
        rc = readlink(path, buf, size );
        if rc == -(1) {
            /* Not link or doesn't exist. */
            if *__errno_location() == 22 ||
                   *__errno_location() == 2 {
                // free(buf);
                return 0
            } else {
                die("cannot access path %s: %s"                   *const libc::c_char , path,
                    5);
            }
        } else {
            if rc < size - 1 {
                let mut d: &mut String = 0 ;
                *buf.offset(rc) = 0;
                if *buf.offset(0) !=
                       '/' as i32 &&
                       { d = strrchr(path, '/' as i32); !d.is_null() } {
                    /* Add path to relative link */
                    let mut new_buf: &mut String =
                        // safe_malloc((d.wrapping_offset_from(path)                                   i32).wrapping_add(strlen(buf)).wrapping_add(2                                   ))
                        //     ;
                    *d.offset(1) =
                        0;
                    strcpy(new_buf, path);
                    strcat(new_buf, buf);
                    // free(buf);
                    buf = new_buf
                }
                return buf
            }
        }
        /* Buffer too small, increase and retry */
        size += 64;
        // free(buf);
    };
}
#[no_mangle]
pub unsafe extern "C" fn inotify_dnsmasq_init() {
    let mut res: Resolvc = 0 ;
    // inotify_buffer =
        // safe_malloc((::std::mem::size_of::<inotify_event>()).wrapping_add(255      libc::c_ulong).wrapping_add(1
        //                                                                                                                ))
        //     ;
    dnsmasq_daemon.inotifyfd =
        inotify_init1(IN_NONBLOCK | IN_CLOEXEC);
    if dnsmasq_daemon.inotifyfd == -(1) {
        die("failed to create inotify: %s",
            0 , 5);
    }
    if dnsmasq_daemon.options[(8).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (8).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           != 0 {
        return
    }
    res = dnsmasq_daemon.resolv_files;
    while !res.is_null() {
        let mut d: &mut String = 0 ;
        let mut new_path: &mut String = 0 ;
        // let mut path: &mut String =
        //     safe_malloc(strlen(res.name).wrapping_add(1   ))
        //         ;
        let mut links: i32 = 20;
        strcpy(path, res.name);
        loop 
             /* Follow symlinks until we reach a non-symlink, or a non-existent file. */
             {
            new_path = my_readlink(path); /* make path just directory */
            if new_path.is_null() {
                break ; /* pointer to filename */
            }
            let fresh6 = links;
            links = links - 1;
            if fresh6 == 0 {
                die("too many symlinks following %s"                   *const libc::c_char , res.name,
                    5);
            }
            // free(path);
            path = new_path
        }
        res.wd = -(1);
        d = strrchr(path, '/' as i32);
        if !d.is_null() {
            *d = 0;
            res.wd =
                inotify_add_watch(dnsmasq_daemon.inotifyfd, path,
                                  (0x8 | 0x80)
                                     );
            res.file = d.offset(1);
            *d = '/' ;
            if res.wd == -(1) &&
                   *__errno_location() == 2 {
                die("directory %s for resolv-file is missing, cannot poll"
                                          &mut String, res.name, 5);
            }
        }
        if res.wd == -(1) {
            die("failed to create inotify for %s: %s"               *const libc::c_char , res.name,
                5);
        }
        res = res.next
    };
}
/* initialisation for dynamic-dir. Set inotify watch for each directory, and read pre-existing files */
#[no_mangle]
pub unsafe extern "C" fn set_dynamic_inotify(mut flag: i32,
                                             mut total_size: i32,
                                             mut rhash: &mut Crec,
                                             mut revhashsz: i32) {
    let mut ah: HostsFile = 0;
    ah = dnsmasq_daemon.dynamic_dirs;
    while !ah.is_null() {
        let mut dir_stream: DIR = 0 ;
        let mut ent: dirent = 0 ent;
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
        if !(ah.flags & flag == 0) {
            if stat(ah.fname, &mut buf) == -(1) ||
                   !(buf.st_mode & 0o170000 ==
                         0o40000) {
                my_syslog(3,
                          "bad dynamic directory %s: %s" , ah.fname,
                          strerror(*__errno_location()));
            } else {
                if ah.flags & 4 == 0 {
                    ah.wd =
                        inotify_add_watch(dnsmasq_daemon.inotifyfd,
                                          ah.fname,
                                          (0x8 |
                                               0x80)                                        u32);
                    ah.flags |= 4
                }
                /* Read contents of dir _after_ calling add_watch, in the hope of avoiding
	  a race which misses files being added as we start */
                if ah.wd == -(1) ||
                       {
                           dir_stream = opendir(ah.fname);
                           dir_stream.is_null()
                       } {
                    my_syslog(3,
                              "failed to create inotify for %s: %s"                            *const u8,
                              ah.fname, strerror(*__errno_location()));
                } else {
                    loop  {
                        ent = readdir(dir_stream);
                        if ent.is_null() { break ; }
                        let mut lendir: usize = strlen(ah.fname);
                        let mut lenfile: usize =
                            strlen(ent.d_name.as_mut_ptr());
                        let mut path: &mut String =
                            0 ;
                        /* ignore emacs backups and dotfiles */
                        if lenfile == 0 ||
                               ent.d_name[lenfile.wrapping_sub(1
                                                                                 )
                                                 ] ==
                                   '~' as i32 ||
                               ent.d_name[0 ]                              == '#' as i32 &&
                                   ent.d_name[lenfile.wrapping_sub(1
                                                                                         )
                                                     ]
                                       == '#' as i32 ||
                               ent.d_name[0 ]                              == '.' as i32 {
                            continue ;
                        }
                        // path =
                        //     whine_malloc(lendir.wrapping_add(lenfile).wrapping_add(2))
                        //         ;
                        if !path.is_null() {
                            strcpy(path, ah.fname);
                            strcat(path,
                                   "/"                                  *const libc::c_char);
                            strcat(path, ent.d_name.as_mut_ptr());
                            /* ignore non-regular files */
                            if stat(path, &mut buf) != -(1) &&
                                   buf.st_mode &
                                       0o170000
                                       ==
                                       0o100000
                               {
                                if ah.flags & 8 != 0 {
                                    total_size =
                                        read_hostsfile(path, ah.index,
                                                       total_size, rhash,
                                                       revhashsz)
                                } else if ah.flags &
                                              (16 |
                                                   32) != 0 {
                                    option_read_dynfile(path, ah.flags);
                                }
                            }
                            // free(path);
                        }
                    }
                    closedir(dir_stream);
                }
            }
        }
        ah = ah.next
    };
}
#[no_mangle]
pub unsafe extern "C" fn inotify_check(mut now: time::Instant) -> i32 {
    let mut hit: i32 = 0;
    let mut ah: HostsFile = 0;
    loop  {
        let mut rc: i32 = 0;
        let mut p: &mut String = 0 ;
        let mut res: Resolvc = 0 ;
        let mut in_0: inotify_event = 0 );
        loop  {
            rc =
                read(dnsmasq_daemon.inotifyfd,
                     inotify_buffer,
                     (::std::mem::size_of::<inotify_event>()).wrapping_add(255).wrapping_add(1
                                                                                                                         ))
                   ;
            if !(rc == -(1) &&
                     *__errno_location() == 4) {
                break ;
            }
        }
        if rc <= 0 { break ; }
        p = inotify_buffer;
        while rc -
                  p.wrapping_offset_from(inotify_buffer) >=
                  ::std::mem::size_of::<inotify_event>()  {
            let mut namelen: usize = 0;
            in_0 = p );
            /* ignore emacs backups and dotfiles */
            if !(in_0.len == 0 ||
                     {
                         namelen = strlen(in_0.name.as_mut_ptr());
                         (namelen) == 0
                     } ||
                     *in_0.name.as_mut_ptr().offset(namelen.wrapping_sub(1

                                                                                                     )
                                                          )  == '~' as i32 ||
                     *in_0.name.as_mut_ptr().offset(0        )  == '#' as i32 &&
                         *in_0.name.as_mut_ptr().offset(namelen.wrapping_sub(1

                                                                                                             )
                                                              )                        == '#' as i32 ||
                     *in_0.name.as_mut_ptr().offset(0        )  == '.' as i32) {
                res = dnsmasq_daemon.resolv_files;
                while !res.is_null() {
                    if res.wd == in_0.wd &&
                           strcmp(res.file, in_0.name.as_mut_ptr()) ==
                               0 {
                        hit = 1
                    }
                    res = res.next
                }
                ah = dnsmasq_daemon.dynamic_dirs;
                while !ah.is_null() {
                    if ah.wd == in_0.wd {
                        let mut lendir: usize = strlen(ah.fname);
                        let mut path: &mut String =
                            0 ;
                        // path =
                        //     whine_malloc(lendir.wrapping_add(in_0.len       ).wrapping_add(2                           ))
                        //         ;
                        if !path.is_null() {
                            strcpy(path, ah.fname);
                            strcat(path,
                                   "/"                                  *const libc::c_char);
                            strcat(path, in_0.name.as_mut_ptr());
                            my_syslog(6,
                                      "inotify, new or changed file %s"
                                          ,
                                      path);
                            if ah.flags & 8 != 0 {
                                read_hostsfile(path, ah.index,
                                               0,
                                               0,
                                               0);
                                if !dnsmasq_daemon.dhcp.is_null() ||
                                       dnsmasq_daemon.doing_dhcp6 != 0 {
                                    /* Propagate the consequences of loading a new dhcp-host */
                                    dhcp_update_configs(dnsmasq_daemon.dhcp_conf);
                                    lease_update_from_configs();
                                    lease_update_file(now);
                                    lease_update_dns(1);
                                }
                            } else if ah.flags & 16 != 0 {
                                if option_read_dynfile(path,
                                                       16) != 0
                                   {
                                    /* Propagate the consequences of loading a new dhcp-host */
                                    dhcp_update_configs(dnsmasq_daemon.dhcp_conf);
                                    lease_update_from_configs();
                                    lease_update_file(now);
                                    lease_update_dns(1);
                                }
                            } else if ah.flags & 32 != 0 {
                                option_read_dynfile(path, 32);
                            }
                            // free(path);
                        }
                    }
                    ah = ah.next
                }
            }
            p =
                p.offset((::std::mem::size_of::<inotify_event>()).wrapping_add(in_0.len    )
                            )
        }
    }
    return hit;
}
/* INOTIFY */
