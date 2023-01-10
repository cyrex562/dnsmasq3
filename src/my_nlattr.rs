#[derive(Default, Debug, Clone)]
pub struct my_nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}