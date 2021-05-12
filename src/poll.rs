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

// struct pollfd *pollfds = NULL;
// nfds_t nfds, arrsize = 0;

/* Binary search. Returns either the pollfd with fd, or
   if the fd doesn't match, or return equals nfds, the entry
   to the left of which a new record should be inserted. */
use crate::dnsmasq_h::{PollFd, nfds_t};

pub fn fd_search(fd: i32) -> nfds_t
{
// nfds_t left, right, mid;
    let mut left: nfds_t;
    let mut right: nfds_t;
    let mut mid: nfds_t;

    right = nfds;
    if right == 0 { return 0; }

    left = 0;

    loop
    {
        if right == left + 1 {
            return if pollfds[left].fd >= fd { left } else { right };
        }

        mid = (left + right)/2;

        if pollfds[mid].fd > fd { right = mid; }
        else { left = mid; }
    }
}

pub fn poll_reset() {
    nfds = 0;
}

pub fn do_poll(timeout: i32) -> i32 {
    return poll(pollfds, nfds, timeout);
}

pub fn poll_check(fd: i32, event: i16) -> i32 {
    let i = fd_search(fd);

    if i < nfds && pollfds[i].fd == fd { return pollfds[i].revents & event; }

    return 0;
}

pub fn poll_listen(fd: i32, event: i16)
{
    let i = fd_search(fd);

    if i < nfds && pollfds[i].fd == fd { pollfds[i].events |= event; }
    else
    {
        if arrsize != nfds {
            // TODO:
            // memmove(&pollfds[i + 1], &pollfds[i], (nfds - i) * sizeof(struct pollfd));
        }
        else
        {
            /* Array too small, extend. */
            let mut new = PollFd::new();

            // arrsize = (arrsize == 0) ? 64 : arrsize * 2;
            if arrsize == 0 {
                arrsize = 64
            } else {
                arrsize = arrsize * 2;
            }

            // TODO:
            // if !(new = whine_malloc(arrsize * sizeof(struct pollfd))))
            // { return;}

            if pollfds
            {
                // TODO
                // memcpy(new, pollfds, i * sizeof(struct pollfd));
                // memcpy(&new[i+1], &pollfds[i], (nfds - i) * sizeof(struct pollfd));
                // free(pollfds);
            }

            pollfds = new;
        }

        pollfds[i].fd = fd;
        pollfds[i].events = event;
        nfds +=1;
    }
}
