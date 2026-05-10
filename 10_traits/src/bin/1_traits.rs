// Binary entry: uses the `traits` library crate (`src/lib.rs`).

use traits::naming::{
    print_details, print_details_with_multiple_traits, print_full_name, same_two,
    HasFullNameWithName, InitializeWithFullname,
};
use traits::person::Person;

fn main() {
    let me = Person {
        first_name: String::from("Sooyoung"),
        last_name: String::from("Kim"),
        age: 28,
    };
    println!("me: {:?}", me);

    // UFCS: `<Type as Trait>::method(...)` picks which trait’s `new` to call (useful when names collide).
    let me = <Person as InitializeWithFullname>::new("Sooyoung Kim");
    println!("me: {}", me);

    print_full_name(&me);
    print_details(&me);

    let other = Person {
        first_name: String::from("Alex"),
        last_name: String::from("Lee"),
        age: 30,
    };
    same_two(&me, &other);

    print_details_with_multiple_traits(&me);

    // `Person` also has `HasFullName::full_name` — same method name on another trait — so use UFCS
    // to call the version from `HasFullNameWithName` (built from `HasName` accessors).
    let me_with_full_name = <Person as HasFullNameWithName>::full_name(&me);
    println!("me_with_full_name: {}", me_with_full_name);
}
