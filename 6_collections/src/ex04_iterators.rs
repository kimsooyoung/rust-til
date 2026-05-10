pub fn run() {
    let num_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let iter = num_vec.iter();
    let _sum1: i32 = iter.sum();

    let twice_vec = num_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("twice_vec: {:?}", twice_vec);

    for num in num_vec.iter() {
        println!("num: {}", num);
    }

    for num in num_vec.into_iter().filter(|&x| x % 2 == 0) {
        println!("even num: {}", num);
    }
}
