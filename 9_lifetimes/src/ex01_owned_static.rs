use crate::lifetime_fns::{get_full_name_ref, get_full_name_string};

pub fn run() {
    let full_name = get_full_name_string();
    println!("full_name: {}", full_name);

    let full_name = get_full_name_ref();
    println!("full_name: {}", full_name);
}
