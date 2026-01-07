use intutils::addition::add;
use intutils::subtraction::sub;


fn main() {
    let add_result = add(1, 2);
    println!("Add result: {}", add_result);

    let sub_result = sub(1, 2);
    println!("Sub result: {}", sub_result);
}
