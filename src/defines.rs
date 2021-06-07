
/* from socket.h */
/* Protocol families.  */
// #define        PF_UNSPEC        0        /* Unspecified.  */
pub const PF_UNSPEC: u16 = 0;
// #define        PF_LOCAL        1        /* Local to host (pipes and file-domain).  */
pub const PF_LOCAL: u16 = 1;
// #define        PF_UNIX                PF_LOCAL /* POSIX name for PF_LOCAL.  */
pub const PF_UNIX: u16 = 1;
// #define        PF_FILE                PF_LOCAL /* Another non-standard name for PF_LOCAL.  */
pub const PF_FILE: u16 = 1;
// #define        PF_INET                2        /* IP protocol family.  */
pub const PF_INET: u16 = 2;
// #define        PF_AX25                3        /* Amateur Radio AX.25.  */
pub const PF_AX25: u16 = 3;
// #define        PF_IPX                4        /* Novell Internet Protocol.  */
pub const PF_IPX: u16 = 4;
// #define        PF_APPLETALK        5        /* Appletalk DDP.  */
pub const PF_APPLETALK: u16 = 5;
// #define        PF_NETROM        6        /* Amateur radio NetROM.  */
pub const PF_NETROM: u16 = 6;
// #define        PF_BRIDGE        7        /* Multiprotocol bridge.  */
pub const PF_BRIDGE: u16 = 7;
// #define        PF_ATMPVC        8        /* ATM PVCs.  */
pub const PF_ATMPVC: u16 = 8;
// #define        PF_X25                9        /* Reserved for X.25 project.  */
pub const PF_X25: u16 = 9;
// #define        PF_INET6        10        /* IP version 6.  */
pub const PF_INET6: u16 = 10;
// #define        PF_ROSE                11        /* Amateur Radio X.25 PLP.  */
pub const PF_ROSE: u16 = 11;
// #define        PF_DECnet        12        /* Reserved for DECnet project.  */
pub const PF_DECnet: u16 = 12;
// #define        PF_NETBEUI        13        /* Reserved for 802.2LLC project.  */
pub const PF_NETBEUI: u16 = 13;
// #define        PF_SECURITY        14        /* Security callback pseudo AF.  */
pub const PF_SECURITY: u16 = 14;
// #define        PF_KEY                15        /* PF_KEY key management API.  */
pub const PF_KEY: u16 = 15;
// #define        PF_NETLINK        16
pub const PF_NETLINK: u16 = 16;
// #define        PF_ROUTE        PF_NETLINK /* Alias to emulate 4.4BSD.  */
pub const PF_ROUTE: u16 = 16;
// #define        PF_PACKET        17        /* Packet family.  */
pub const PF_PACKET: u16 = 17;
// #define        PF_ASH                18        /* Ash.  */
pub const PF_ASH: u16 = 18;
// #define        PF_ECONET        19        /* Acorn Econet.  */
pub const PF_ECONET: u16 = 19;
// #define        PF_ATMSVC        20        /* ATM SVCs.  */
pub const PF_ATMSVC: u16 = 20;
// #define PF_RDS                21        /* RDS sockets.  */
pub const PF_RDS: u16 = 21;
// #define        PF_SNA                22        /* Linux SNA Project */
pub const PF_SNA: u16 = 22;
// #define        PF_IRDA                23        /* IRDA sockets.  */
pub const PF_IRDA: u16 = 23;
// #define        PF_PPPOX        24        /* PPPoX sockets.  */
pub const PF_PPPOX: u16 = 24;
// #define        PF_WANPIPE        25        /* Wanpipe API sockets.  */
pub const PF_WANPIPE: u16 = 25;
// #define PF_LLC                26        /* Linux LLC.  */
pub const PF_LLC: u16 = 26;
// #define PF_CAN                29        /* Controller Area Network.  */
pub const PF_CAN: u16 = 29;
// #define PF_TIPC                30        /* TIPC sockets.  */
pub const PF_TIPC: u16 = 30;
// #define        PF_BLUETOOTH        31        /* Bluetooth sockets.  */
pub const PF_BLUETOOTH: u16 = 31;
// #define        PF_IUCV                32        /* IUCV sockets.  */
pub const PF_IUCV: u16 = 32;
// #define PF_RXRPC        33        /* RxRPC sockets.  */
pub const PF_RXRPC: u16 = 33;
// #define PF_ISDN                34        /* mISDN sockets.  */
pub const PF_ISDN: u16 = 34;
// #define PF_PHONET        35        /* Phonet sockets.  */
pub const PF_PHONET: u16 = 35;
// #define PF_IEEE802154        36        /* IEEE 802.15.4 sockets.  */
pub const PF_IEEE82154: u16 = 36;
// #define        PF_MAX                37        /* For now..  */
pub const PF_MAX: u16 = 37;

/* Address families.  */
// #define        AF_UNSPEC        PF_UNSPEC
// #define        AF_LOCAL        PF_LOCAL
// #define        AF_UNIX                PF_UNIX
// #define        AF_FILE                PF_FILE
// #define        AF_INET                PF_INET
// #define        AF_AX25                PF_AX25
// #define        AF_IPX                PF_IPX
// #define        AF_APPLETALK        PF_APPLETALK
// #define        AF_NETROM        PF_NETROM
// #define        AF_BRIDGE        PF_BRIDGE
// #define        AF_ATMPVC        PF_ATMPVC
// #define        AF_X25                PF_X25
// #define        AF_INET6        PF_INET6
// #define        AF_ROSE                PF_ROSE
// #define        AF_DECnet        PF_DECnet
// #define        AF_NETBEUI        PF_NETBEUI
// #define        AF_SECURITY        PF_SECURITY
// #define        AF_KEY                PF_KEY
// #define        AF_NETLINK        PF_NETLINK
// #define        AF_ROUTE        PF_ROUTE
// #define        AF_PACKET        PF_PACKET
// #define        AF_ASH                PF_ASH
// #define        AF_ECONET        PF_ECONET
// #define        AF_ATMSVC        PF_ATMSVC
// #define AF_RDS                PF_RDS
// #define        AF_SNA                PF_SNA
// #define        AF_IRDA                PF_IRDA
// #define        AF_PPPOX        PF_PPPOX
// #define        AF_WANPIPE        PF_WANPIPE
// #define AF_LLC                PF_LLC
// #define AF_CAN                PF_CAN
// #define AF_TIPC                PF_TIPC
// #define        AF_BLUETOOTH        PF_BLUETOOTH
// #define        AF_IUCV                PF_IUCV
// #define AF_RXRPC        PF_RXRPC
// #define AF_ISDN                PF_ISDN
// #define AF_PHONET        PF_PHONET
// #define AF_IEEE802154        PF_IEEE802154
// #define        AF_MAX                PF_MAX
