// Debug Traits

use std::fmt;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// Define a trait for initializing
trait InitializeWithFullname {
    fn new(full_name: &str) -> Self;
}

// Implement the trait for the Person struct
impl InitializeWithFullname for Person {
    fn new(full_name: &str) -> Self {
        let parts = full_name.split_whitespace().collect::<Vec<&str>>();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 0,
        }
    }
}

// Implement the Display trait for the Person struct
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({} years old)",
            self.first_name, self.last_name, self.age
        )
    }
}

// trait as parameter
trait HasFullName {
    fn full_name(&self) -> String;
}

impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn print_full_name(person: &impl HasFullName) {
    println!("Full name: {}", person.full_name());
}

// We can also use Trait Bound syntax to specify the type of the parameter
fn print_details<T: HasFullName>(person: &T) {
    println!("Full name: {}", person.full_name());
}

// Conformance to multiple traits
trait CanDrive {
    fn can_drive(&self) -> bool;
}

impl CanDrive for Person {
    fn can_drive(&self) -> bool {
        self.age >= 18
    }
}

fn print_details_with_multiple_traits<T>(person: &T)
where
    T: HasFullName + CanDrive,
{
    println!("Full name: {}", person.full_name());
    println!("Can drive: {}", person.can_drive());
}

// Trait in Trait
trait HasName {
    fn fist_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

impl HasName for Person {
    fn fist_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
}

trait HasFullNameWithName
where
    Self: HasName,
{
    fn full_name(&self) -> String;
}

impl<T> HasFullNameWithName for T
where
    T: HasName,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.fist_name(), self.last_name())
    }
}

fn main() {
    let me = Person {
        first_name: String::from("Sooyoung"),
        last_name: String::from("Kim"),
        age: 28,
    };
    println!("me: {:?}", me);

    // Use fully-qualified path syntax to specify which type implements the trait
    let me = <Person as InitializeWithFullname>::new("Sooyoung Kim");
    println!("me: {}", me);

    print_full_name(&me);
    print_details(&me);
    print_details_with_multiple_traits(&me);

    let me_with_full_name = <Person as HasFullNameWithName>::full_name(&me);
    println!("me_with_full_name: {}", me_with_full_name);
}
