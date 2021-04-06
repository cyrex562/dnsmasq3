
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
use crate::util::whine_malloc;

// static mut pollfds: pollfd = 0 ;
// static mut nfds: nfds_t = 0;
// static mut arrsize: nfds_t = 0 as nfds_t;
/* Binary search. Returns either the pollfd with fd, or
   if the fd doesn't match, or return equals nfds, the entry
   to the left of which a new record should be inserted. */
 fn fd_search(mut fd: i32) -> nfds_t {
    let mut left: nfds_t = 0;
    let mut right: nfds_t = 0;
    let mut mid: nfds_t = 0;
    right = nfds;
    if right == 0 {
        return 0 as nfds_t
    }
    left = 0 as nfds_t;
    loop  {
        if right == left.wrapping_add(1) {
            return if (*pollfds.offset(left)).fd >= fd {
                       left
                   } else { right }
        }
        mid = left.wrapping_add(right).wrapping_div(2 );
        if (*pollfds.offset(mid)).fd > fd {
            right = mid
        } else { left = mid }
    };
}

pub  fn poll_reset() { nfds = 0 as nfds_t; }

pub  fn do_poll(mut timeout: i32) -> i32 {
    return poll(pollfds, nfds, timeout);
}

pub  fn poll_check(mut fd: i32,
                                    mut event: libc::c_short) -> i32 {
    let mut i: nfds_t = fd_search(fd);
    if i < nfds && (*pollfds.offset(i)).fd == fd {
        return (*pollfds.offset(i)).revents &
                   event
    }
    return 0;
}

pub  fn poll_listen(mut fd: i32,
                                     mut event: libc::c_short) {
    let mut i: nfds_t = fd_search(fd);
    if i < nfds && (*pollfds.offset(i)).fd == fd {
        let ref mut fresh6 = (*pollfds.offset(i)).events;
        *fresh6 =
            (*fresh6 | event)
    } else {
        if arrsize != nfds {
            // memmove(&mut *pollfds.offset(i.wrapping_add(1)),
            //         &mut *pollfds.offset(i),
            //         nfds.wrapping_sub(i).wrapping_mul(::std::mem::size_of::<pollfd>()
            //                                              ));
        } else {
            /* Array too small, extend. */
            let mut new: pollfd = 0 ;
            arrsize =
                if arrsize == 0 {
                    64
                } else {
                    arrsize.wrapping_mul(2)
                };
            new =
                whine_malloc(arrsize.wrapping_mul(::std::mem::size_of::<pollfd>()
                                                     ))              pollfd;
            if new.is_null() { return }
            if !pollfds.is_null() {
                memcpy(new,
                       pollfds,
                       i.wrapping_mul(::std::mem::size_of::<pollfd>() ));
                memcpy(&mut *new.offset(i.wrapping_add(1 ))                     Vec<u8>,
                       &mut *pollfds.offset(i)
                       nfds.wrapping_sub(i).wrapping_mul(::std::mem::size_of::<pollfd>()
                                                               ));
                free(pollfds);
            }
            pollfds = new
        }
        (*pollfds.offset(i)).fd = fd;
        (*pollfds.offset(i)).events = event;
        nfds = nfds.wrapping_add(1)
    };
}
