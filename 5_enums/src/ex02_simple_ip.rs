use crate::simple_ip::SimpleIPAddressKind;

pub fn run() {
    let office_simple = SimpleIPAddressKind::V4(192, 168, 1, 1);
    let home_simple = SimpleIPAddressKind::V6("::1".to_string());
    println!("office_simple: {:?}", office_simple);
    println!("home_simple: {:?}", home_simple);

    match office_simple {
        SimpleIPAddressKind::V4(a, b, c, d) => println!("matched V4 octets: {:?}", (a, b, c, d)),
        SimpleIPAddressKind::V6(ref address) => println!("matched V6 string: {:?}", address),
    }

    match office_simple {
        SimpleIPAddressKind::V4(a, b, c, d) => println!("This is V4: {:?}", (a, b, c, d)),
        _ => println!("This is not V4"),
    }
}
