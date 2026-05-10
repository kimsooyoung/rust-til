use crate::fns::get_tuple;

pub fn run() {
    let values = ("Sooyoung", "Kim", 28);
    let (_, _, age) = values;
    println!("age: {}", age);

    let (name, surname, age) = get_tuple();
    println!("name: {}, surname: {}, age: {}", name, surname, age);

    let name_vec: [&str; 3] = ["John", "Jane", "Jim"];
    for name in name_vec.iter() {
        println!("name: {}", name);
    }

    let first_name = &name_vec[0];
    println!("first_name: {}", first_name);

    let num_ver: [i32; 3] = [1, 2, 3];
    let twice_num_ver = num_ver.map(|x| x * 2);
    println!("twice_num_ver: {:?}", twice_num_ver);
}
