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

enum Shapes {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}

impl Shapes {
    fn area(&self) -> f64 {
        match self {
            Shapes::Circle { radius, .. } => radius * radius * std::f64::consts::PI,
            Shapes::Rectangle { width, height } => width * height,
        }
    }
}

enum Pet {
    Cat { name: String },
    Dog { name: String },
}
fn main() {
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

    let office_simple = SimpleIPAddressKind::V4(192, 168, 1, 1);
    let home_simple = SimpleIPAddressKind::V6("::1".to_string());
    println!("office_simple: {:?}", office_simple);
    println!("home_simple: {:?}", home_simple);

    match office_simple {
        SimpleIPAddressKind::V4(a, b, c, d) => println!("office_simple: {:?}", (a, b, c, d)),
        SimpleIPAddressKind::V6(ref address) => println!("office_simple: {:?}", address),
    }

    // catch-all-case
    match office_simple {
        SimpleIPAddressKind::V4(a, b, c, d) => println!("This is V4: {:?}", (a, b, c, d)),
        _ => println!("This is not V4"),
    }

    let arbitrary_shape = Shapes::Circle {
        radius: 10.0,
        center: (0.0, 0.0),
    };
    let _rect = Shapes::Rectangle {
        width: 10.0,
        height: 20.0,
    };

    match arbitrary_shape {
        Shapes::Circle { radius, center } => {
            println!(
                "Circle: radius: {}, center: {:?}, area: {}",
                radius,
                center,
                arbitrary_shape.area()
            )
        }
        Shapes::Rectangle { width, height } => {
            println!(
                "Rectangle: width: {}, height: {:?}, area: {}",
                width,
                height,
                arbitrary_shape.area()
            )
        }
    }

    let my_cat = Pet::Cat {
        name: String::from("Whiskers"),
    };
    let my_dog = Pet::Dog {
        name: String::from("Buddy"),
    };
    let my_cat_name = match my_cat {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };
    let my_dog_name = match my_dog {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };
    println!("my_cat_name: {}", my_cat_name);
    println!("my_dog_name: {}", my_dog_name);
}
