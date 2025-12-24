fn print_second_value(tup: (i32, f32, bool)) -> f32 {
    println!("second value: {}", tup.1);
    tup.1
}

fn main() {
    let my_tuple = (1, 2.0, true);
    print_second_value(my_tuple);

    let usd_to_krw = {
        let usd = 50;
        let krw = usd * 1400;
        krw
    };
    println!("usd_to_krw: {}", usd_to_krw);
}
