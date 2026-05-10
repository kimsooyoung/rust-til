use crate::lifetime_fns::get_random_name;

pub fn run() {
    let random_name = get_random_name("Sooyoung Kim", "Hong Gil Dong");
    println!("random_name: {}", random_name);
}
