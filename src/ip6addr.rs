/* dnsmasq is Copyright (c) 2000-2018 Simon Kelley

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
use std::convert::TryInto;


// #define IN6_IS_ADDR_ULA(a) \
//         ((((__const uint32_t *) (a))[0] & htonl (0xff000000))                 \
//          == htonl (0xfd000000))
fn IN6_IS_ADDR_ULA(a: &[u8]) -> bool {
        let f: u32 = 0xff000000;
        let c = f.swap_bytes();
        let d: u32 = 0xfd000000;
        let e = d.swap_bytes();
        let g: [u8;4] = a[0..4].try_into().unwrap();
        let b = u32::from_ne_bytes(g);
        b & c == e     
}

// #define IN6_IS_ADDR_ULA_ZERO(a) \
//         (((__const uint32_t *) (a))[0] == htonl (0xfd000000)                        \
//          && ((__const uint32_t *) (a))[1] == 0                                \
//          && ((__const uint32_t *) (a))[2] == 0                                \
//          && ((__const uint32_t *) (a))[3] == 0)
fn IN6_IS_ADDR_ULA_ZERO(a: &[u8]) -> bool {
        let b = u32::from_ne_bytes(a[0..4].try_into().unwrap());
        let c = u32::from_ne_bytes(a[4..8].try_into().unwrap());
        let d = u32::from_ne_bytes(a[8..12].try_into().unwrap());
        let e = u32::from_ne_bytes(a[12..].try_into().unwrap());
        let g = 0xfd000000u32.swap_bytes();
        if b != g {
                return false;
        }
        if c != 0 {
                return false;
        } 
        if d != 0 {
                return false;
        } 
        if e != 0 {
                return false;
        }
        true
}

// #define IN6_IS_ADDR_LINK_LOCAL_ZERO(a) \
//         (((__const uint32_t *) (a))[0] == htonl (0xfe800000)                  \
//          && ((__const uint32_t *) (a))[1] == 0                                \
//          && ((__const uint32_t *) (a))[2] == 0                                \
//          && ((__const uint32_t *) (a))[3] == 0)
fn IN6_IS_ADDR_LINK_LOCAL_ZERO(a: &[u8]) -> bool {
        let b = u32::from_ne_bytes(a[0..4].try_into().unwrap());
        let c = u32::from_ne_bytes(a[4..8].try_into().unwrap());
        let d = u32::from_ne_bytes(a[8..12].try_into().unwrap());
        let e = u32::from_ne_bytes(a[12..].try_into().unwrap());
        let g = 0xfe800000u32.swap_bytes();
        b == g && c == 0 && d == 0 && e == 0
}
