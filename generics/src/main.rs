use std::ops::AddAssign;

#[derive(Debug)]
struct IntPoint {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn move_point(&mut self, x: T, y: T)
    where
        T: AddAssign<T>,
    {
        self.x += x;
        self.y += y;
    }
}

impl <T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl <T: PartialEq> PartialEq for Point<T> {
    // Self is the Type, and self is the Instance (the value).
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point<String> {
    fn move_point_str(&mut self, x: &str, y: &str) {
        self.x += x;
        self.y += y;
    }
}

// Using Trait with generic 
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

// if all elements in the vector can run, 
// then the vector can run
impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

// if all elements in the vector can walk, 
// then the vector can walk
impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

fn main() {
    let int_point = IntPoint { x: 1, y: 2 };
    println!("int_point x : {}, y : {}", int_point.x, int_point.y);

    let mut int_point_generic = Point { x: 1, y: 2 };
    let mut float_point = Point { x: 1.0, y: 2.0 };
    let mut string_point = Point { x: String::from("1"), y: String::from("2") };

    int_point_generic.move_point(1, 2);
    println!("int_point_generic: {:?}", int_point_generic);
    float_point.move_point(1.0, 2.0);
    println!("float_point: {:?}", float_point);
    // Now this works because String implements AddAssign<&str>
    string_point.move_point_str("1", "2");
    println!("string_point: {:?}", string_point);

    let int_point_generic2 = Point { x: 10, y: 20 };
    int_point_generic += int_point_generic2;
    println!("int_point_generic: {:?}", int_point_generic);

    // PartialEq
    let int_point_generic3 = Point { x: 10, y: 20 };
    if int_point_generic == int_point_generic3 {
        println!("int_point_generic is equal to int_point_generic3");
    } else {
        println!("int_point_generic is not equal to int_point_generic3");
    }

    // Trait with generic
    let people = vec![
        Person { name: "John".to_string() }, 
        Person { name: "Jane".to_string() },
        Person { name: "Jim".to_string() },
        Person { name: "Jill".to_string() },
    ];
    people.run();
    people.walk();
    let elephants = vec![
        Elephant { name: "Dumbo".to_string() }, 
        Elephant { name: "Jumbo".to_string() },
        Elephant { name: "Kong".to_string() },
        Elephant { name: "King Kong".to_string() },
    ];
    elephants.walk();
    // elephants.run();
}
