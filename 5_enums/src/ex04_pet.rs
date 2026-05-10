use crate::pet::Pet;

pub fn run() {
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
