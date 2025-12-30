use std::collections::HashMap;

fn get_tuple() -> (String, String, i32) {
    ("John".to_string(), "Doe".to_string(), 30)
}

fn main() {
    // tuple
    let values = ("Sooyoung", "Kim", 28);
    let (_, _, age) = values;
    println!("age: {}", age);

    let (name, surname, age) = get_tuple();
    println!("name: {}, surname: {}, age: {}", name, surname, age);

    // Fixed size vector
    let name_vec: [&str; 3] = ["John", "Jane", "Jim"];
    for name in name_vec.iter() {
        println!("name: {}", name);
    }

    // get vector element as reference
    let first_name = &name_vec[0];
    println!("first_name: {}", first_name);

    let num_ver: [i32; 3] = [1, 2, 3];
    let twice_num_ver = num_ver.map(|x| x * 2);
    println!("twice_num_ver: {:?}", twice_num_ver);

    // vector create with shortcut
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("vec: {:?}", vec);

    // extend vector with another vector
    vec.extend_from_slice(&[5, 6, 7]);
    println!("vec: {:?}", vec);

    // merge two vectors
    let mut float_vec1 = vec![1.0, 2.0, 3.0];
    let mut float_vec2 = vec![4.0, 5.0, 6.0];
    float_vec1.append(&mut float_vec2);
    println!("float_vec1: {:?}", float_vec1);
    // this will shows empty vector
    // because append moves the ownership of the vector to the first vector
    println!("float_vec2: {:?}", float_vec2);

    // contains, is_empty
    let us_states: Vec<&str> = vec!["California", "New York", "Texas"];
    // contains() expects &&str when array contains &str, so we need to pass &"British Columbia"
    if us_states.contains(&"British Columbia") {
        println!("British Columbia is in the list");
    } else {
        println!("British Columbia is not in the list");
    }

    if us_states.is_empty() {
        println!("us_states is empty");
    } else {
        println!("us_states is not empty");
    }

    // =========================================================================
    // Hashmap
    // =========================================================================
    let mut us_states_hash: HashMap<&str, &str> = HashMap::new();
    us_states_hash.insert("California", "CA");
    us_states_hash.insert("New York", "NY");
    us_states_hash.insert("Texas", "TX");
    println!("us_states_hash: {:?}", us_states_hash);

    // contains_key, access by name, access by get()
    if us_states_hash.contains_key("California") {
        println!("California is in the hashmap");
    } else {
        println!("California is not in the hashmap");
    }

    // this is not good
    let cali = us_states_hash["California"];
    println!("cali: {}", cali);

    // should use like this
    match us_states_hash.get("California") {
        Some(value) => println!("California is in the hashmap: {}", value),
        None => println!("California is not in the hashmap"),
    }

    // traverse key and value by reference
    for (&key, &value) in &us_states_hash {
        println!("key: {}, value: {}", key, value);
    }

    // insert if key is absent
    us_states_hash.entry("Florida").or_insert("FL");

    // =========================================================================
    // Iterator
    // =========================================================================

    // iterator is lazy
    let num_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let iter = num_vec.iter();
    let _sum1: i32 = iter.sum();
    // this will not work because iter is already
    // consumed
    // let _sum2: i32 = iter.sum();

    let twice_vec = num_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("twice_vec: {:?}", twice_vec);

    // iter() creates an iterator over immutable references
    for num in num_vec.iter() {
        println!("num: {}", num);
    }

    // into_iter() creates an iterator that takes ownership of the vector
    for num in num_vec.into_iter().filter(|&x| x % 2 == 0) {
        println!("even num: {}", num);
    }
}
