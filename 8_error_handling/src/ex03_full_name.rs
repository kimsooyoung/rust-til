use crate::fns::get_full_name;

pub fn run() {
    let full_name = get_full_name();
    match full_name {
        Ok(x) => println!("Full name: {}", x),
        Err(_) => println!("Error occurred"),
    }

    let full_name = get_full_name();
    let full_name_len = full_name.map(|x| x.len()).unwrap_or_default();
    println!("Full name length: {}", full_name_len);

    let full_name = get_full_name();
    let full_name_err = full_name.map_err(|_| "Error during map");
    match full_name_err {
        Ok(x) => println!("Full name: {}", x),
        Err(e) => println!("Error: {}", e),
    }
}
