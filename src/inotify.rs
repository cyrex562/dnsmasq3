
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
static mut inotify_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* If path is a symbolic link, return the path it
   points to, made absolute if relative.
   If path doesn't exist or is not a symlink, return NULL.
   Return value is malloc'ed */
unsafe extern "C" fn my_readlink(mut path: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut rc: ssize_t = 0;
    let mut size: ssize_t = 64 as libc::c_int as ssize_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        buf = safe_malloc(size as size_t) as *mut libc::c_char;
        rc = readlink(path, buf, size as size_t);
        if rc == -(1 as libc::c_int) as libc::c_long {
            /* Not link or doesn't exist. */
            if *__errno_location() == 22 as libc::c_int ||
                   *__errno_location() == 2 as libc::c_int {
                free(buf as *mut libc::c_void);
                return 0 as *mut libc::c_char
            } else {
                die(b"cannot access path %s: %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, path,
                    5 as libc::c_int);
            }
        } else {
            if rc < size - 1 as libc::c_int as libc::c_long {
                let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
                *buf.offset(rc as isize) = 0 as libc::c_int as libc::c_char;
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int !=
                       '/' as i32 &&
                       { d = strrchr(path, '/' as i32); !d.is_null() } {
                    /* Add path to relative link */
                    let mut new_buf: *mut libc::c_char =
                        safe_malloc((d.wrapping_offset_from(path) as
                                         libc::c_long as
                                         libc::c_ulong).wrapping_add(strlen(buf)).wrapping_add(2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                            as *mut libc::c_char;
                    *d.offset(1 as libc::c_int as isize) =
                        0 as libc::c_int as libc::c_char;
                    strcpy(new_buf, path);
                    strcat(new_buf, buf);
                    free(buf as *mut libc::c_void);
                    buf = new_buf
                }
                return buf
            }
        }
        /* Buffer too small, increase and retry */
        size += 64 as libc::c_int as libc::c_long;
        free(buf as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn inotify_dnsmasq_init() {
    let mut res: *mut resolvc = 0 as *mut resolvc;
    inotify_buffer =
        safe_malloc((::std::mem::size_of::<inotify_event>() as
                         libc::c_ulong).wrapping_add(255 as libc::c_int as
                                                         libc::c_ulong).wrapping_add(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
            as *mut libc::c_char;
    (*dnsmasq_daemon).inotifyfd =
        inotify_init1(IN_NONBLOCK as libc::c_int | IN_CLOEXEC as libc::c_int);
    if (*dnsmasq_daemon).inotifyfd == -(1 as libc::c_int) {
        die(b"failed to create inotify: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 5 as libc::c_int);
    }
    if (*dnsmasq_daemon).options[(8 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (8 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        return
    }
    res = (*dnsmasq_daemon).resolv_files;
    while !res.is_null() {
        let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut new_path: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut path: *mut libc::c_char =
            safe_malloc(strlen((*res).name).wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong))
                as *mut libc::c_char;
        let mut links: libc::c_int = 20 as libc::c_int;
        strcpy(path, (*res).name);
        loop 
             /* Follow symlinks until we reach a non-symlink, or a non-existent file. */
             {
            new_path = my_readlink(path); /* make path just directory */
            if new_path.is_null() {
                break ; /* pointer to filename */
            }
            let fresh6 = links;
            links = links - 1;
            if fresh6 == 0 as libc::c_int {
                die(b"too many symlinks following %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, (*res).name,
                    5 as libc::c_int);
            }
            free(path as *mut libc::c_void);
            path = new_path
        }
        (*res).wd = -(1 as libc::c_int);
        d = strrchr(path, '/' as i32);
        if !d.is_null() {
            *d = 0 as libc::c_int as libc::c_char;
            (*res).wd =
                inotify_add_watch((*dnsmasq_daemon).inotifyfd, path,
                                  (0x8 as libc::c_int | 0x80 as libc::c_int)
                                      as uint32_t);
            (*res).file = d.offset(1 as libc::c_int as isize);
            *d = '/' as i32 as libc::c_char;
            if (*res).wd == -(1 as libc::c_int) &&
                   *__errno_location() == 2 as libc::c_int {
                die(b"directory %s for resolv-file is missing, cannot poll\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char, (*res).name, 5 as libc::c_int);
            }
        }
        if (*res).wd == -(1 as libc::c_int) {
            die(b"failed to create inotify for %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, (*res).name,
                5 as libc::c_int);
        }
        res = (*res).next
    };
}
/* initialisation for dynamic-dir. Set inotify watch for each directory, and read pre-existing files */
#[no_mangle]
pub unsafe extern "C" fn set_dynamic_inotify(mut flag: libc::c_int,
                                             mut total_size: libc::c_int,
                                             mut rhash: *mut *mut crec,
                                             mut revhashsz: libc::c_int) {
    let mut ah: *mut hostsfile = 0 as *mut hostsfile;
    ah = (*dnsmasq_daemon).dynamic_dirs;
    while !ah.is_null() {
        let mut dir_stream: *mut DIR = 0 as *mut DIR;
        let mut ent: *mut dirent = 0 as *mut dirent;
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
        if !((*ah).flags & flag == 0) {
            if stat((*ah).fname, &mut buf) == -(1 as libc::c_int) ||
                   !(buf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                         0o40000 as libc::c_int as libc::c_uint) {
                my_syslog(3 as libc::c_int,
                          b"bad dynamic directory %s: %s\x00" as *const u8 as
                              *const libc::c_char, (*ah).fname,
                          strerror(*__errno_location()));
            } else {
                if (*ah).flags & 4 as libc::c_int == 0 {
                    (*ah).wd =
                        inotify_add_watch((*dnsmasq_daemon).inotifyfd,
                                          (*ah).fname,
                                          (0x8 as libc::c_int |
                                               0x80 as libc::c_int) as
                                              uint32_t);
                    (*ah).flags |= 4 as libc::c_int
                }
                /* Read contents of dir _after_ calling add_watch, in the hope of avoiding
	  a race which misses files being added as we start */
                if (*ah).wd == -(1 as libc::c_int) ||
                       {
                           dir_stream = opendir((*ah).fname);
                           dir_stream.is_null()
                       } {
                    my_syslog(3 as libc::c_int,
                              b"failed to create inotify for %s: %s\x00" as
                                  *const u8 as *const libc::c_char,
                              (*ah).fname, strerror(*__errno_location()));
                } else {
                    loop  {
                        ent = readdir(dir_stream);
                        if ent.is_null() { break ; }
                        let mut lendir: size_t = strlen((*ah).fname);
                        let mut lenfile: size_t =
                            strlen((*ent).d_name.as_mut_ptr());
                        let mut path: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        /* ignore emacs backups and dotfiles */
                        if lenfile == 0 as libc::c_int as libc::c_ulong ||
                               (*ent).d_name[lenfile.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                                                 as usize] as libc::c_int ==
                                   '~' as i32 ||
                               (*ent).d_name[0 as libc::c_int as usize] as
                                   libc::c_int == '#' as i32 &&
                                   (*ent).d_name[lenfile.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                                     as usize] as libc::c_int
                                       == '#' as i32 ||
                               (*ent).d_name[0 as libc::c_int as usize] as
                                   libc::c_int == '.' as i32 {
                            continue ;
                        }
                        path =
                            whine_malloc(lendir.wrapping_add(lenfile).wrapping_add(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
                                as *mut libc::c_char;
                        if !path.is_null() {
                            strcpy(path, (*ah).fname);
                            strcat(path,
                                   b"/\x00" as *const u8 as
                                       *const libc::c_char);
                            strcat(path, (*ent).d_name.as_mut_ptr());
                            /* ignore non-regular files */
                            if stat(path, &mut buf) != -(1 as libc::c_int) &&
                                   buf.st_mode &
                                       0o170000 as libc::c_int as libc::c_uint
                                       ==
                                       0o100000 as libc::c_int as libc::c_uint
                               {
                                if (*ah).flags & 8 as libc::c_int != 0 {
                                    total_size =
                                        read_hostsfile(path, (*ah).index,
                                                       total_size, rhash,
                                                       revhashsz)
                                } else if (*ah).flags &
                                              (16 as libc::c_int |
                                                   32 as libc::c_int) != 0 {
                                    option_read_dynfile(path, (*ah).flags);
                                }
                            }
                            free(path as *mut libc::c_void);
                        }
                    }
                    closedir(dir_stream);
                }
            }
        }
        ah = (*ah).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn inotify_check(mut now: time_t) -> libc::c_int {
    let mut hit: libc::c_int = 0 as libc::c_int;
    let mut ah: *mut hostsfile = 0 as *mut hostsfile;
    loop  {
        let mut rc: libc::c_int = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut res: *mut resolvc = 0 as *mut resolvc;
        let mut in_0: *mut inotify_event = 0 as *mut inotify_event;
        loop  {
            rc =
                read((*dnsmasq_daemon).inotifyfd,
                     inotify_buffer as *mut libc::c_void,
                     (::std::mem::size_of::<inotify_event>() as
                          libc::c_ulong).wrapping_add(255 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                    as libc::c_int;
            if !(rc == -(1 as libc::c_int) &&
                     *__errno_location() == 4 as libc::c_int) {
                break ;
            }
        }
        if rc <= 0 as libc::c_int { break ; }
        p = inotify_buffer;
        while rc as libc::c_long -
                  p.wrapping_offset_from(inotify_buffer) as libc::c_long >=
                  ::std::mem::size_of::<inotify_event>() as libc::c_ulong as
                      libc::c_int as libc::c_long {
            let mut namelen: size_t = 0;
            in_0 = p as *mut inotify_event;
            /* ignore emacs backups and dotfiles */
            if !((*in_0).len == 0 as libc::c_int as libc::c_uint ||
                     {
                         namelen = strlen((*in_0).name.as_mut_ptr());
                         (namelen) == 0 as libc::c_int as libc::c_ulong
                     } ||
                     *(*in_0).name.as_mut_ptr().offset(namelen.wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                                           as isize) as
                         libc::c_int == '~' as i32 ||
                     *(*in_0).name.as_mut_ptr().offset(0 as libc::c_int as
                                                           isize) as
                         libc::c_int == '#' as i32 &&
                         *(*in_0).name.as_mut_ptr().offset(namelen.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                               as isize) as
                             libc::c_int == '#' as i32 ||
                     *(*in_0).name.as_mut_ptr().offset(0 as libc::c_int as
                                                           isize) as
                         libc::c_int == '.' as i32) {
                res = (*dnsmasq_daemon).resolv_files;
                while !res.is_null() {
                    if (*res).wd == (*in_0).wd &&
                           strcmp((*res).file, (*in_0).name.as_mut_ptr()) ==
                               0 as libc::c_int {
                        hit = 1 as libc::c_int
                    }
                    res = (*res).next
                }
                ah = (*dnsmasq_daemon).dynamic_dirs;
                while !ah.is_null() {
                    if (*ah).wd == (*in_0).wd {
                        let mut lendir: size_t = strlen((*ah).fname);
                        let mut path: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        path =
                            whine_malloc(lendir.wrapping_add((*in_0).len as
                                                                 libc::c_ulong).wrapping_add(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                                as *mut libc::c_char;
                        if !path.is_null() {
                            strcpy(path, (*ah).fname);
                            strcat(path,
                                   b"/\x00" as *const u8 as
                                       *const libc::c_char);
                            strcat(path, (*in_0).name.as_mut_ptr());
                            my_syslog(6 as libc::c_int,
                                      b"inotify, new or changed file %s\x00"
                                          as *const u8 as *const libc::c_char,
                                      path);
                            if (*ah).flags & 8 as libc::c_int != 0 {
                                read_hostsfile(path, (*ah).index,
                                               0 as libc::c_int,
                                               0 as *mut *mut crec,
                                               0 as libc::c_int);
                                if !(*dnsmasq_daemon).dhcp.is_null() ||
                                       (*dnsmasq_daemon).doing_dhcp6 != 0 {
                                    /* Propagate the consequences of loading a new dhcp-host */
                                    dhcp_update_configs((*dnsmasq_daemon).dhcp_conf);
                                    lease_update_from_configs();
                                    lease_update_file(now);
                                    lease_update_dns(1 as libc::c_int);
                                }
                            } else if (*ah).flags & 16 as libc::c_int != 0 {
                                if option_read_dynfile(path,
                                                       16 as libc::c_int) != 0
                                   {
                                    /* Propagate the consequences of loading a new dhcp-host */
                                    dhcp_update_configs((*dnsmasq_daemon).dhcp_conf);
                                    lease_update_from_configs();
                                    lease_update_file(now);
                                    lease_update_dns(1 as libc::c_int);
                                }
                            } else if (*ah).flags & 32 as libc::c_int != 0 {
                                option_read_dynfile(path, 32 as libc::c_int);
                            }
                            free(path as *mut libc::c_void);
                        }
                    }
                    ah = (*ah).next
                }
            }
            p =
                p.offset((::std::mem::size_of::<inotify_event>() as
                              libc::c_ulong).wrapping_add((*in_0).len as
                                                              libc::c_ulong)
                             as isize)
        }
    }
    return hit;
}
/* INOTIFY */
