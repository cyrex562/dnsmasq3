/* https://wiki.wireshark.org/Development/LibpcapFileFormat */
#[derive(Default, Debug, Clone)]
pub struct pcap_hdr_s {
    pub magic_number: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub thiszone: u32,
    pub sigfigs: u32,
    pub snaplen: u32,
    pub network: u32,
}