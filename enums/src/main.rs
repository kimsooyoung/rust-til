#[derive(Debug)]
enum IPAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

#[derive(Debug)]
enum SimpleIPAddressKind {
    V4(i32, i32, i32, i32),
    V6(String),
}

fn main() {

    let office = IPAddress {
        kind: IPAddressKind::V4,
        address: String::from("192.168.1.1"),
    };
    let home = IPAddress {
        kind: IPAddressKind::V6,
        address: String::from("::1"),
    };
    println!("office: {:?}", office);
    println!("home: {:?}", home);


    let office_simple = SimpleIPAddressKind::V4(192, 168, 1, 1);
    let home_simple = SimpleIPAddressKind::V6("::1".to_string());
    println!("office_simple: {:?}", office_simple);
    println!("home_simple: {:?}", home_simple);
}
