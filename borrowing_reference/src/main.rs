fn main() {
    let _x: i32 = 5;
    let _ref_x: &i32 = &_x;

    println!("x = {}", _x);
    println!("ref_x = {}", _ref_x);

    // This does not work because _ref_x is a reference to _x
    // *_ref_x += 1;
    // println!("x = {}", _x);
    // println!("ref_x = {}", _ref_x);

    let mut _y: i32 = 5;
    let _ref_y: &mut i32 = &mut _y;

    *_ref_y += 1;

    // This does not work because _ref_y is a mutable reference to _y
    // println!("y = {}", _y);

    // This works because _ref_y is a mutable reference to _y
    println!("ref_y = {}", _ref_y);
}
