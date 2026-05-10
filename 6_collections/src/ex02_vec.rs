pub fn run() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("vec: {:?}", vec);

    vec.extend_from_slice(&[5, 6, 7]);
    println!("vec: {:?}", vec);

    let mut float_vec1 = vec![1.0, 2.0, 3.0];
    let mut float_vec2 = vec![4.0, 5.0, 6.0];
    float_vec1.append(&mut float_vec2);
    println!("float_vec1: {:?}", float_vec1);
    println!("float_vec2: {:?}", float_vec2);

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
}
