// Chapter examples: generics on structs, trait implementations, and blanket impls.
use std::ops::AddAssign;

// A concrete struct: both coordinates are always `i32`.
#[derive(Debug)]
struct IntPoint {
    x: i32,
    y: i32,
}

// A generic struct: `x` and `y` share the same type parameter `T`.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Inherent impl: methods that belong to `Point<T>` for any `T`.
// `AddAssign<T>` is required so `self.x += x` type-checks (same for `y`).
impl<T> Point<T> {
    fn move_point(&mut self, x: T, y: T)
    where
        T: AddAssign<T>,
    {
        self.x += x;
        self.y += y;
    }
}

// Implement the standard `+=` operator for points via `AddAssign`.
// After this, `point += other_point` works when `T` supports `+=`.
impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Implement equality so you can use `==` and `!=` on two `Point<T>` values.
// `Self` is the type (`Point<T>`); `self` / `other` are the values being compared.
impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Specialization for `Point<String>`: `String` uses `+=` with `&str`, not with `String`,
// so we offer a dedicated helper instead of forcing awkward generic bounds.
impl Point<String> {
    fn move_point_str(&mut self, x: &str, y: &str) {
        self.x += x;
        self.y += y;
    }
}

// --- Custom traits (our own interfaces) ---

trait CanRun {
    fn run(&self);
}

trait CanWalk {
    fn walk(&self);
}

struct Person {
    name: String,
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}

impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

struct Elephant {
    name: String,
}

impl CanWalk for Elephant {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

// Blanket impl: if every element of a `Vec<T>` can run, treat the whole vector as runnable.
// This only compiles for `T` types that already implement `CanRun`.
impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

// Same idea for walking: `Vec<T>` implements `CanWalk` whenever `T: CanWalk`.
impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

fn main() {
    // Non-generic point
    let int_point = IntPoint { x: 1, y: 2 };
    println!("int_point x : {}, y : {}", int_point.x, int_point.y);

    // Same struct, three different `T`s: `i32`, `f64`, `String`
    let mut int_point_generic = Point { x: 1, y: 2 };
    let mut float_point = Point { x: 1.0, y: 2.0 };
    let mut string_point = Point {
        x: String::from("1"),
        y: String::from("2"),
    };

    int_point_generic.move_point(1, 2);
    println!("int_point_generic: {:?}", int_point_generic);
    float_point.move_point(1.0, 2.0);
    println!("float_point: {:?}", float_point);
    string_point.move_point_str("1", "2");
    println!("string_point: {:?}", string_point);

    // Uses our `AddAssign for Point<T>` impl
    let int_point_generic2 = Point { x: 10, y: 20 };
    int_point_generic += int_point_generic2;
    println!("int_point_generic: {:?}", int_point_generic);

    // Uses our `PartialEq for Point<T>` impl
    let int_point_generic3 = Point { x: 10, y: 20 };
    if int_point_generic == int_point_generic3 {
        println!("int_point_generic is equal to int_point_generic3");
    } else {
        println!("int_point_generic is not equal to int_point_generic3");
    }

    // `Vec<Person>` gets `CanRun` / `CanWalk` from the blanket impls above.
    let people = vec![
        Person {
            name: "John".to_string(),
        },
        Person {
            name: "Jane".to_string(),
        },
        Person {
            name: "Jim".to_string(),
        },
        Person {
            name: "Jill".to_string(),
        },
    ];
    people.run();
    people.walk();

    let elephants = vec![
        Elephant {
            name: "Dumbo".to_string(),
        },
        Elephant {
            name: "Jumbo".to_string(),
        },
        Elephant {
            name: "Kong".to_string(),
        },
        Elephant {
            name: "King Kong".to_string(),
        },
    ];
    elephants.walk();
    // `elephants.run()` does not compile: `Elephant` has no `impl CanRun for Elephant`.
}
