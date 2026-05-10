use crate::ip::{IPAddress, IPAddressKind};

pub fn run() {
    let office = IPAddress {
        kind: IPAddressKind::V4,
        address: String::from("192.168.1.1"),
    };
    println!("office kind: {:?}", office.kind);
    println!("office address: {:?}", office.address);
    let home = IPAddress {
        kind: IPAddressKind::V6,
        address: String::from("::1"),
    };
    println!("home kind: {:?}", home.kind);
    println!("home address: {:?}", home.address);
}
