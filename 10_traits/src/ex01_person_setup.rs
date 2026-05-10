use crate::naming::InitializeWithFullname;
use crate::person::Person;

pub fn run() {
    let me = Person {
        first_name: String::from("Sooyoung"),
        last_name: String::from("Kim"),
        age: 28,
    };
    println!("me: {:?}", me);

    let me = <Person as InitializeWithFullname>::new("Sooyoung Kim");
    println!("me: {}", me);
}
