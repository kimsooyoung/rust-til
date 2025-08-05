extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let _rand_num = rand::thread_rng().gen_range(1, 100);

    println!("Number is {}", _rand_num);

    loop {
        println!("Please input your number : ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Question: not let mut guess? not &guess.trim()?
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You typed {}", guess);

        match guess.cmp(&_rand_num){
            Ordering::Less =>    println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal =>   {
                println!("You win!");
                break;
            }
        }
    }
}
