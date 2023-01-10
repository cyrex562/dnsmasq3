#[derive(Default, Debug, Clone)]
pub struct dns_header {
    pub id: u16,
    pub hb3: u8,
    pub hb4: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}
