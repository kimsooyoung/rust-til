use crate::lifetime_fns::{get_name_ref, Person};

pub fn run() {
    let person = Person {
        first_name: "Sooyoung",
        last_name: "Kim",
    };
    println!(
        "person: first_name: {}, last_name: {}",
        person.first_name, person.last_name
    );
    let first_char = person.first_char_of_first_name();
    println!("first_char: {}", first_char);

    let name_ref = get_name_ref("Maci Marshall");
    println!("name_ref: {}", name_ref);
}
