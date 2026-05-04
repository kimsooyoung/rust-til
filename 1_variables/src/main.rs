//! Chapter 1 — Rust variables: bindings, mutability, types, shadowing, constants, tuples.
//!
//! Run: `cd 1_variables && cargo run`

fn section(title: &str) {
    println!("\n=== {} ===\n", title);
}

fn main() {
    section("Immutable vs mutable bindings");
    let answer = 42; // default: immutable
    println!("answer (immutable): {answer}");
    // answer = 0; // error: cannot assign twice to immutable variable

    let mut score = 0;
    println!("score start: {score}");
    score += 10;
    println!("after += 10: {score}");
    score = 100; // allowed: binding is `mut`
    println!("score after updates: {score}");

    section("Changing the value (same type only)");
    let mut count: u32 = 1;
    println!("count start: {count}");
    count = 2;
    count += 5;
    println!("count: {count}");
    // Rust does not let you change the *type* of an existing binding:
    // count = -1;        // error: expected u32, found i32 (or similar)
    // count = 3.14;      // error: expected u32, found floating-point
    // To use a new type, shadow with a new `let` (see shadowing) or use a new variable.

    section("Integer types (signed / unsigned, widths)");
    let a: i8 = -128;
    let b: i16 = 1000;
    let c: i32 = -1; // very common default inference target
    let d: i64 = 1_000_000;
    let e: i128 = i128::MAX;
    let f: isize = 7; // pointer-sized, good for indexing / sizes on this platform

    let u: u8 = 255;
    let v: u16 = 65535;
    let w: u32 = 4_294_967_295;
    let x: u64 = 1 << 20;
    let y: u128 = u128::MAX;
    let z: usize = 1024;

    println!(
        "i8 {a}, i16 {b}, i32 {c}, i64 {d}, i128 {e}, isize {f}\n\
         u8 {u}, u16 {v}, u32 {w}, u64 {x}, u128 {y}, usize {z}"
    );

    section("Floating-point types");
    let pi32: f32 = std::f32::consts::PI;
    let pi64: f64 = std::f64::consts::PI;
    println!("f32 π ≈ {pi32:.7}, f64 π ≈ {pi64:.15}");

    section("Operators mixing numeric types (explicit casts)");
    let n: i32 = 10;
    let m: f64 = 3.0;
    // Rust will not silently mix i32 and f64:
    // let bad = n + m; // error
    let sum = f64::from(n) + m;
    let diff = n - (m as i32); // truncates toward zero
    let bits: u8 = 0b1010;
    let shifted = bits << 1;
    let masked = shifted & 0b1111;
    println!("f64::from(i32) + f64: {sum}, i32 - f64 as i32: {diff}, bit play: {masked}");

    section("Variable shadowing (new binding, can change type)");
    let spaces = "   ";
    println!("shadow 1 (str): len = {}", spaces.len());
    let spaces = spaces.len(); // new `let`: name reused, type can change
    println!("shadow 2 (usize): {spaces}");

    let raw = "100";
    let raw: i32 = raw.parse().expect("digits");
    println!("parsed shadowed `raw` as i32: {raw}");

    section("Constants (compile-time, UPPER_SNAKE_CASE, type required)");
    const MAX_POINTS: u32 = 100_000;
    const DEFAULT_TAX_RATE_PERCENT: f64 = 8.25;
    println!("MAX_POINTS = {MAX_POINTS}, DEFAULT_TAX_RATE_PERCENT = {DEFAULT_TAX_RATE_PERCENT}");

    // `const` cannot use runtime values; this would not compile:
    // const NOW: u128 = std::time::SystemTime::now(); // error

    section("Tuples — access, iterate, edit");
    let pair = ("Rust", 2026);
    println!("tuple Debug: {pair:?}");
    println!("by index: language = {}, year = {}", pair.0, pair.1);
    let (lang, year) = pair; // destructuring
    println!("destructured: {lang} {year}");

    // Edit elements: the tuple binding must be `mut`
    let mut rgb = (255u8, 128u8, 0u8);
    rgb.1 = 200;
    println!("mut tuple after edit: {:?}", rgb);

    // Iterate *homogeneous* tuples as fixed-size sequences via array (same types):
    let triple = (1i32, 2, 3);
    let arr = [triple.0, triple.1, triple.2];
    print!("iterate (via array from homogeneous tuple): ");
    for v in arr {
        print!("{v} ");
    }
    println!();

    // Iterate *over* tuples stored in a collection (common pattern):
    let rows = vec![("Alice", 10u32), ("Bob", 20), ("Carol", 30)];
    println!("iterate Vec of tuples:");
    for (name, value) in &rows {
        println!("  {name}: {value}");
    }

    // Heterogeneous tuples like (i32, &str, bool) have no built-in `.iter()`;
    // access fields by index or destructure instead.

    println!("\nDone. Try uncommenting the `error` lines above to see the compiler messages.");
}
