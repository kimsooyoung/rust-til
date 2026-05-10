#[derive(Debug)]
pub enum IPAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
pub struct IPAddress {
    pub kind: IPAddressKind,
    pub address: String,
}
