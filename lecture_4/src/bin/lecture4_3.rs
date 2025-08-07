fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main(){
    let mut _my_arr = String::from("Hello Proception!!");
    let _my_arr_immut = String::from("Hello Immutable!!");

    let _slice = &_my_arr_immut[1..5];

    let _my_arr_len = first_word(&_my_arr[..]);

    _my_arr.clear();

    println!("_my_arr: {}", _my_arr);
    println!("_my_arr_immut: {}", _my_arr_immut);
    println!("_my_arr_len: {}", _my_arr_len);
    println!("_slice: {}", _slice);
}