// Enums (enumerations)
// - A type that can be *one of* several variants at runtime
// - Each variant can carry no data (unit-like), tuple data, or named fields (struct-like)
// - `match` is the usual way to branch on which variant you have (exhaustive: every variant covered)

// C-style enum: variants are just names (often used with a separate struct for payload, as below)
#[derive(Debug)]
enum IPAddressKind {
    V4,
    V6,
}

// Struct + enum: kind and address are not tied together in the type system (easy to mismatch)
#[derive(Debug)]
struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

// Enum variants can carry data directly (no extra struct needed)
#[derive(Debug)]
enum SimpleIPAddressKind {
    V4(i32, i32, i32, i32),
    V6(String),
}

// Struct-like variants: each variant has its own named fields
#[derive(Debug)]
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
    // -------------------------------------------------------------------------
    // Enum + struct: two pieces of state you keep in sync by hand
    // -------------------------------------------------------------------------
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

    // -------------------------------------------------------------------------
    // Data-carrying enum: variant includes its payload
    // -------------------------------------------------------------------------
    let office_simple = SimpleIPAddressKind::V4(192, 168, 1, 1);
    let home_simple = SimpleIPAddressKind::V6("::1".to_string());
    println!("office_simple: {:?}", office_simple);
    println!("home_simple: {:?}", home_simple);

    // `ref address` borrows the inner `String` for this arm only (does not move out of the enum)
    match office_simple {
        SimpleIPAddressKind::V4(a, b, c, d) => println!("matched V4 octets: {:?}", (a, b, c, d)),
        SimpleIPAddressKind::V6(ref address) => println!("matched V6 string: {:?}", address),
    }

    // Catch-all `_`: matches every variant not listed in earlier arms (here, only non-V4).
    // You can match `office_simple` again only if the earlier `match` did not move non-Copy
    // data out (here: V4 copies `i32`; V6 uses `ref`, so the `String` stays inside the enum).
    match office_simple {
        SimpleIPAddressKind::V4(a, b, c, d) => println!("This is V4: {:?}", (a, b, c, d)),
        _ => println!("This is not V4"),
    }

    // -------------------------------------------------------------------------
    // Struct-like enum variants + `match` on owned value
    // -------------------------------------------------------------------------
    let arbitrary_shape = Shapes::Circle {
        radius: 10.0,
        center: (0.0, 0.0),
    };
    let _rect = Shapes::Rectangle {
        width: 10.0,
        height: 20.0,
    };

    // Call `area` before `match` consumes `arbitrary_shape` (clearer than calling `.area()`
    // inside an arm while the value is being deconstructed)
    let shape_area = arbitrary_shape.area();
    match arbitrary_shape {
        Shapes::Circle { radius, center } => {
            println!(
                "Circle: radius: {}, center: {:?}, area: {}",
                radius, center, shape_area
            );
        }
        Shapes::Rectangle { width, height } => {
            println!(
                "Rectangle: width: {}, height: {}, area: {}",
                width, height, shape_area
            );
        }
    }

    // -------------------------------------------------------------------------
    // Moving fields out of an enum with `match` (same field name, different variants)
    // -------------------------------------------------------------------------
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
