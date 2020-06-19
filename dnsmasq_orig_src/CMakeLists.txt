cmake_minimum_required(VERSION 3.14)
project(dnsmasq3)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_STANDARD 11)

add_subdirectory(fmt)

add_subdirectory(spdlog)

include_directories(src spdlog/include fmt/include)

add_executable(dnsmasq3
        src/arp.cpp
        src/auth.cpp
        src/blockdata.cpp
        src/bpf.cpp
        src/cache.cpp
        src/config.h
        src/conntrack.cpp
        src/crypto.cpp
        src/dbus.cpp
        src/dhcp-common.cpp
        src/dhcp-protocol.h
        src/dhcp.cpp
        src/dhcp6-protocol.h
        src/dhcp6.cpp
        src/dns-protocol.h
        src/dnsmasq.cpp
        src/dnsmasq.h
        src/dnssec.cpp
        src/domain.cpp
        src/dump.cpp
        src/edns0.cpp
        src/forward.cpp
        src/helper.cpp
        src/inotify.cpp
        src/ip6addr.h
        src/ipset.cpp
        src/lease.cpp
        src/log.cpp
        src/loop.cpp
        src/metrics.cpp
        src/metrics.h
        src/netlink.cpp
        src/network.cpp
        src/option.cpp
        src/outpacket.cpp
        src/poll.cpp
        src/radv-protocol.h
        src/radv.cpp
        src/rfc1035.cpp
        src/rfc2131.cpp
        src/rfc3315.cpp
        src/rrfilter.cpp
        src/slaac.cpp
        src/tables.cpp
        src/tftp.cpp
        src/ubus.cpp
        src/util.cpp src/dnsmasq_sys.h)

target_include_directories(dnsmasq3 PUBLIC fmt/include)
target_include_directories(dnsmasq3 PUBLIC spdlog/include)
target_link_libraries(dnsmasq3 fmt)
target_link_libraries(dnsmasq3 spdlog)

#
# END OF FILE
#