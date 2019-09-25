//
// Created by JoshMadden on 9/24/2019.
//

#ifndef DNSMASQ3_DNSMASQ_SYS_H
#define DNSMASQ3_DNSMASQ_SYS_H

#include <cstdint>

#ifdef _WIN32

constexpr size_t IFNAMSIZ = 16;

struct iovec {
    void* iov_base; // starting address
    size_t iov_len; // number of bytes to transfer
};

struct ifmap
{
    unsigned long mem_start;
    unsigned long mem_end;
    unsigned long base_addr;
    unsigned long irq;
    unsigned long dma;
    unsigned long port;
};

struct ifreq
{
    char ifr_name[IFNAMSIZ];
    struct sockaddr ifr_addr;
    struct sockaddr ifr_dstaddr;
    struct sockaddr ifr_broadaddr;
    struct sockaddr ifr_netmask;
    struct sockaddr ifr_hwaddr;
    short ifr_flags;
    int ifr_ifindex;
    int ifr_metric;
    int ifr_mtu;
    struct ifmap ifr_map;
    char ifr_slave[IFNAMSIZ];
    char ifr_newname[IFNAMSIZ];
    char* ifr_data;
};


using uid_t = unsigned int;
using gid_t = unsigned int;
using in_addr_t = uint32_t;
using pid_t = int;


#endif


#if defined _WIN64
using ssize_t = int64_t;
#elif define _WIN32
using ssize_t = long;
#endif


#endif //DNSMASQ3_DNSMASQ_SYS_H
