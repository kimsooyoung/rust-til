fn str_len(s: &String) -> usize {
    s.len()
}

fn str_len_mut(s: &mut String){
    s.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("dangerous string");

    // &s
    s
}

fn main(){
    let _my_str = String::from("my string!");
    let mut _my_str_mut = String::from("my mutable string!");
    
    let _str_len = str_len(&_my_str);
    str_len_mut(&mut _my_str_mut);
    str_len_mut(&mut _my_str_mut);
    
    println!("_str_len: {}", _str_len);
    println!("_my_str_mut: {}", _my_str_mut);

    dangle();
}