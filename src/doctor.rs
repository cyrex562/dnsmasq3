/* dns doctor param */
#[derive(Default, Debug, Clone)]
pub struct doctor {
    // struct in_addr in, end, out, mask;
    pub in_address: in_addr,
    pub end_address: in_addr,
    pub out_address: in_addr,
    pub mask_address: in_addr,
    // struct doctor *next;
}
