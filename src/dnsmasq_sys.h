//
// Created by JoshMadden on 9/24/2019.
//

#ifndef DNSMASQ3_DNSMASQ_SYS_H
#define DNSMASQ3_DNSMASQ_SYS_H

#include <cstdint>

struct iovec {
    void* iov_base; // starting address
    size_t iov_len; // number of bytes to transfer
};

typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef uint32_t in_addr_t;


#endif //DNSMASQ3_DNSMASQ_SYS_H
