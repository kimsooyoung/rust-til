use crate::naming::{
    print_details, print_details_with_multiple_traits, print_full_name, same_two,
    HasFullNameWithName, InitializeWithFullname,
};
use crate::person::Person;

pub fn run() {
    let me = <Person as InitializeWithFullname>::new("Sooyoung Kim");

    print_full_name(&me);
    print_details(&me);

    let other = Person {
        first_name: String::from("Alex"),
        last_name: String::from("Lee"),
        age: 30,
    };
    same_two(&me, &other);

    print_details_with_multiple_traits(&me);

    let me_with_full_name = <Person as HasFullNameWithName>::full_name(&me);
    println!("me_with_full_name: {}", me_with_full_name);
}
