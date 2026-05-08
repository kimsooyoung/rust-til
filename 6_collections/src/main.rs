// Collections and related tools (Rust std)
// - Tuple: fixed arity, each position can have a different type
// - Array [T; N]: fixed length, stack-allocated (same as C array shape, but safe)
// - Vec<T>: growable contiguous list on the heap
// - HashMap<K, V>: key → value map (hash table)
// - Iterators: lazy sequences; often end with `.collect()`, `.sum()`, or a `for` loop

use std::collections::HashMap;

fn get_tuple() -> (String, String, i32) {
    ("John".to_string(), "Doe".to_string(), 30)
}

fn main() {
    // -------------------------------------------------------------------------
    // Tuples: destructuring and ignoring fields with `_`
    // -------------------------------------------------------------------------
    let values = ("Sooyoung", "Kim", 28);
    let (_, _, age) = values;
    println!("age: {}", age);

    let (name, surname, age) = get_tuple();
    println!("name: {}, surname: {}, age: {}", name, surname, age);

    // -------------------------------------------------------------------------
    // Array [T; N] — fixed size on the stack (not Vec)
    // -------------------------------------------------------------------------
    let name_vec: [&str; 3] = ["John", "Jane", "Jim"];
    for name in name_vec.iter() {
        println!("name: {}", name);
    }

    // Indexing: `&name_vec[0]` borrows the first element (&str)
    let first_name = &name_vec[0];
    println!("first_name: {}", first_name);

    let num_ver: [i32; 3] = [1, 2, 3];
    let twice_num_ver = num_ver.map(|x| x * 2);
    println!("twice_num_ver: {:?}", twice_num_ver);

    // -------------------------------------------------------------------------
    // Vec<T> — heap-allocated, growable vector
    // -------------------------------------------------------------------------
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("vec: {:?}", vec);

    // Append elements from a slice (reallocates if capacity is tight)
    vec.extend_from_slice(&[5, 6, 7]);
    println!("vec: {:?}", vec);

    // `append` moves all elements out of the second Vec and clears it (O(1) tail pointer move)
    let mut float_vec1 = vec![1.0, 2.0, 3.0];
    let mut float_vec2 = vec![4.0, 5.0, 6.0];
    float_vec1.append(&mut float_vec2);
    println!("float_vec1: {:?}", float_vec1);
    // `float_vec2` is still a valid Vec, but its length is 0 (elements moved into float_vec1)
    println!("float_vec2: {:?}", float_vec2);

    // `contains` for `Vec<&str>` compares with element type `&str`; pass `&"…"` so the
    // argument is a `&str` reference comparable to each element
    let us_states: Vec<&str> = vec!["California", "New York", "Texas"];
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

    // -------------------------------------------------------------------------
    // HashMap<K, V> — average O(1) insert/lookup; no ordering guarantee
    // -------------------------------------------------------------------------
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

    // Indexing `map[key]` panics if the key is missing — fine for invariants you know hold
    let cali = us_states_hash["California"];
    println!("cali: {}", cali);

    // Prefer `get` (or `match`) when the key might be absent — returns `Option<&V>`
    match us_states_hash.get("California") {
        Some(value) => println!("California is in the hashmap: {}", value),
        None => println!("California is not in the hashmap"),
    }

    // Iterate by shared reference; do not mutate the map while iterating this way
    for (&key, &value) in &us_states_hash {
        println!("key: {}, value: {}", key, value);
    }

    // `entry` API: insert only if the key is not already present
    us_states_hash.entry("Florida").or_insert("FL");

    // -------------------------------------------------------------------------
    // Iterators — lazy until you consume them (.sum, .collect, `for`, etc.)
    // -------------------------------------------------------------------------
    let num_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let iter = num_vec.iter();
    let _sum1: i32 = iter.sum();
    // `sum` consumes the iterator adapter chain; `iter` cannot be reused
    // let _sum2: i32 = iter.sum();

    // `map` yields a new iterator; `collect` allocates a new Vec
    let twice_vec = num_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("twice_vec: {:?}", twice_vec);

    // `iter()` → `Iterator<Item = &i32>` (borrow elements; `num_vec` still usable afterward)
    for num in num_vec.iter() {
        println!("num: {}", num);
    }

    // `into_iter()` takes ownership of `num_vec`; after this loop, `num_vec` is moved/dropped
    for num in num_vec.into_iter().filter(|&x| x % 2 == 0) {
        println!("even num: {}", num);
    }
}
