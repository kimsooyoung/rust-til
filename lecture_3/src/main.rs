extern crate rand;

use rand::Rng;

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {

    const BIG_NUM: u32 = 1_000_000;
    println!("Big Num : {}", BIG_NUM);

    let _x = 12;
    let _x = "Hello";
    let _x = 3.14;
    println!("Fianl value for x: {}", _x);

    let _spaces = "   ";
    let _spaces = _spaces.len();
    println!("Fianl value spaces : {}", _spaces);

    let _guess: u32 = "42".parse().expect("Enter number plz!");

    let _fx = 2.0; // f64
    let _fy: f32 = 3.0; // f32

    let _f: bool = false;

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    println!("_heart_eyed_cat {}", _heart_eyed_cat);

    let tup: (i32, f32, bool) = (41728, 0.5, true);    
    println!("tup.0 {}", tup.0);
    println!("tup.1 {}", tup.1);
    println!("tup.2 {}", tup.2);

    let _my_list = [1, 2, 3, 4, 5];
    let _first = _my_list[0];
    let _second = _my_list[1];

    let _output_five = five();
    let _output_plus_one = plus_one(3);

    let _rand_num = rand::thread_rng().gen_range(1, 100);
    let _rand_num2 = if _rand_num > 50 {
        100
    } else {
        0
    };

    if _rand_num > 50 {
        println!("Greater than 50");
    } else if _rand_num < 50 {
        println!("Smaller than 50");
    }

    let my_arr = [1,2,3,4,5];

    for item in my_arr.iter(){
        println!("Current Item is {}", item);
    }
}
