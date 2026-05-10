use std::collections::HashMap;

pub fn run() {
    let mut us_states_hash: HashMap<&str, &str> = HashMap::new();
    us_states_hash.insert("California", "CA");
    us_states_hash.insert("New York", "NY");
    us_states_hash.insert("Texas", "TX");
    println!("us_states_hash: {:?}", us_states_hash);

    if us_states_hash.contains_key("California") {
        println!("California is in the hashmap");
    } else {
        println!("California is not in the hashmap");
    }

    let cali = us_states_hash["California"];
    println!("cali: {}", cali);

    match us_states_hash.get("California") {
        Some(value) => println!("California is in the hashmap: {}", value),
        None => println!("California is not in the hashmap"),
    }

    for (&key, &value) in &us_states_hash {
        println!("key: {}, value: {}", key, value);
    }

    us_states_hash.entry("Florida").or_insert("FL");
}
