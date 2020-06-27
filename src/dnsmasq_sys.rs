//
// Created by JoshMadden on 9/24/2019.
//
use std::ffi::{c_void};

// #ifndef DNSMASQ3_DNSMASQ_SYS_H
// #define DNSMASQ3_DNSMASQ_SYS_H

// #include <cstdint>

// #ifdef _WIN32
struct iovec {
    iov_base: *c_void,// starting address
    iov_len: usize, // number of bytes to transfer
}

// typedef unsigned int uid_t;
type uid_t = u32;
// typedef unsigned int gid_t;
type gid_t = u32;
// typedef uint32_t in_addr_t;
type in_addr_t = u32;

// #endif

// #endif //DNSMASQ3_DNSMASQ_SYS_H
