
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
/* Wrapper for poll(). Allocates and extends array of struct pollfds,
   keeps them in fd order so that we can set and test conditions on
   fd using a simple but efficient binary chop. */
/* poll_reset()
   poll_listen(fd, event)
   .
   .
   poll_listen(fd, event);

   hits = do_poll(timeout);

   if (poll_check(fd, event)
    .
    .

   if (poll_check(fd, event)
    .
    .

    event is OR of POLLIN, POLLOUT, POLLERR, etc
*/
static mut pollfds: *mut pollfd = 0 as *const pollfd as *mut pollfd;
static mut nfds: nfds_t = 0;
static mut arrsize: nfds_t = 0 as libc::c_int as nfds_t;
/* Binary search. Returns either the pollfd with fd, or
   if the fd doesn't match, or return equals nfds, the entry
   to the left of which a new record should be inserted. */
unsafe extern "C" fn fd_search(mut fd: libc::c_int) -> nfds_t {
    let mut left: nfds_t = 0;
    let mut right: nfds_t = 0;
    let mut mid: nfds_t = 0;
    right = nfds;
    if right == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as nfds_t
    }
    left = 0 as libc::c_int as nfds_t;
    loop  {
        if right == left.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            return if (*pollfds.offset(left as isize)).fd >= fd {
                       left
                   } else { right }
        }
        mid =
            left.wrapping_add(right).wrapping_div(2 as libc::c_int as
                                                      libc::c_ulong);
        if (*pollfds.offset(mid as isize)).fd > fd {
            right = mid
        } else { left = mid }
    };
}
#[no_mangle]
pub unsafe extern "C" fn poll_reset() { nfds = 0 as libc::c_int as nfds_t; }
#[no_mangle]
pub unsafe extern "C" fn do_poll(mut timeout: libc::c_int) -> libc::c_int {
    return poll(pollfds, nfds, timeout);
}
#[no_mangle]
pub unsafe extern "C" fn poll_check(mut fd: libc::c_int,
                                    mut event: libc::c_short) -> libc::c_int {
    let mut i: nfds_t = fd_search(fd);
    if i < nfds && (*pollfds.offset(i as isize)).fd == fd {
        return (*pollfds.offset(i as isize)).revents as libc::c_int &
                   event as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn poll_listen(mut fd: libc::c_int,
                                     mut event: libc::c_short) {
    let mut i: nfds_t = fd_search(fd);
    if i < nfds && (*pollfds.offset(i as isize)).fd == fd {
        let ref mut fresh6 = (*pollfds.offset(i as isize)).events;
        *fresh6 =
            (*fresh6 as libc::c_int | event as libc::c_int) as libc::c_short
    } else {
        if arrsize != nfds {
            memmove(&mut *pollfds.offset(i.wrapping_add(1 as libc::c_int as
                                                            libc::c_ulong) as
                                             isize) as *mut pollfd as
                        *mut libc::c_void,
                    &mut *pollfds.offset(i as isize) as *mut pollfd as
                        *const libc::c_void,
                    nfds.wrapping_sub(i).wrapping_mul(::std::mem::size_of::<pollfd>()
                                                          as libc::c_ulong));
        } else {
            /* Array too small, extend. */
            let mut new: *mut pollfd = 0 as *mut pollfd;
            arrsize =
                if arrsize == 0 as libc::c_int as libc::c_ulong {
                    64 as libc::c_int as libc::c_ulong
                } else {
                    arrsize.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                };
            new =
                whine_malloc(arrsize.wrapping_mul(::std::mem::size_of::<pollfd>()
                                                      as libc::c_ulong)) as
                    *mut pollfd;
            if new.is_null() { return }
            if !pollfds.is_null() {
                memcpy(new as *mut libc::c_void,
                       pollfds as *const libc::c_void,
                       i.wrapping_mul(::std::mem::size_of::<pollfd>() as
                                          libc::c_ulong));
                memcpy(&mut *new.offset(i.wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong) as
                                            isize) as *mut pollfd as
                           *mut libc::c_void,
                       &mut *pollfds.offset(i as isize) as *mut pollfd as
                           *const libc::c_void,
                       nfds.wrapping_sub(i).wrapping_mul(::std::mem::size_of::<pollfd>()
                                                             as
                                                             libc::c_ulong));
                free(pollfds as *mut libc::c_void);
            }
            pollfds = new
        }
        (*pollfds.offset(i as isize)).fd = fd;
        (*pollfds.offset(i as isize)).events = event;
        nfds = nfds.wrapping_add(1)
    };
}
