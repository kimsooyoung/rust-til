// Option<T> — Rust's standard way to represent an optional value
// - `Some(t)` holds a `T`; `None` means "no value"
// - The compiler forces you to handle both cases (e.g. with `match`, `if let`, or combinators)
// - `unwrap` / `expect` panic on `None`; prefer `unwrap_or`, `?`, or explicit `match` in real code
// - Functions often return `Option` for "not found" / invalid input; `?` short-circuits on `None`

/// Parses a signed integer; returns `None` on parse error or if the value is not strictly positive.
fn parse_positive_i32(s: &str) -> Option<i32> {
    let n: i32 = s.parse().ok()?;
    (n > 0).then_some(n)
}

#[derive(Debug)]
struct Guest {
    name: String,
    /// Optional fields: missing data stays `None` until the user supplies it
    seat: Option<String>,
    meal: Option<String>,
}

/// Builds a line only when **both** optional fields are present (`?` propagates `None`).
fn boarding_summary(guest: &Guest) -> Option<String> {
    let seat = guest.seat.as_deref()?;
    let meal = guest.meal.as_deref()?;
    Some(format!("{} @ {} ({})", guest.name, seat, meal))
}

fn main() {
    // -------------------------------------------------------------------------
    // `match`: exhaustive handling of `Some` vs `None`
    // -------------------------------------------------------------------------
    let name: Option<&str> = Some("Option");
    match name {
        Some(name) => println!("name: {}", name),
        None => println!("name is None"),
    }

    // `expect` / `unwrap`: extract the inner value, or panic with a message (uncomment to try)
    // let user_input: Option<&str> = None;
    // let unwrapped_input = user_input.expect("No name provided");
    // println!("unwrapped_input: {}", unwrapped_input);

    // -------------------------------------------------------------------------
    // Mutating the inner value: `as_mut()` yields `Option<&mut T>` for the duration of the match
    // -------------------------------------------------------------------------
    let mut age: Option<i32> = Some(20);
    match age.as_mut() {
        Some(age) => *age += 1,
        None => println!("age is None"),
    }
    // `unwrap()` panics if `None` — safe here because we know `age` is still `Some(21)`
    println!("Your Korean age is: {}", age.unwrap());

    // -------------------------------------------------------------------------
    // Destructuring several `Option`s at once with `if let`
    // -------------------------------------------------------------------------
    let age1: Option<i32> = Some(20);
    let age2: Option<i32> = Some(30);
    let age3: Option<i32> = Some(40);
    if let (Some(age1), Some(age2), Some(age3)) = (age1, age2, age3) {
        println!("Age sum: {}", age1 + age2 + age3);
    }

    // -------------------------------------------------------------------------
    // `unwrap_or`: if `None`, use the provided default (no panic; `None` is expected here)
    // -------------------------------------------------------------------------
    let your_name: Option<&str> = None;
    let unwrapped_your_name = your_name.unwrap_or("Hong Gil Dong");
    println!("unwrapped_your_name: {}", unwrapped_your_name);

    // `unwrap_or_else`: default is computed lazily (useful when the default is expensive or needs side effects)
    let your_name: Option<&str> = None;
    let unwrapped_your_name = your_name.unwrap_or_else(|| {
        println!("No name provided");
        "Hong Gil Dong"
    });
    println!("unwrapped_your_name: {}", unwrapped_your_name);

    // `is_some` / `is_none` — quick tests (often `if let Some(x) = opt` is clearer when you need the value)
    let son_name: Option<&str> = Some("Hong Gil Dong");
    if son_name.is_some() {
        println!("son_name is some");
    } else {
        println!("son_name is none");
    }

    // `unwrap_or_default`: `None` → `T::default()` (for `i32`, default is `0`)
    let bus_ticket_price: Option<i32> = None;
    let default_price = bus_ticket_price.unwrap_or_default();
    println!("default_price: {}", default_price);

    // `map` transforms the inner `T` when `Some`; if input is `None`, result stays `None`
    let illegal_bus_ticket_price: Option<i32> = Some(1000);
    let price = illegal_bus_ticket_price
        .map(|price| price * 200)
        .unwrap_or_default();
    println!("illegal bus ticket price: {}", price);

    // -------------------------------------------------------------------------
    // Functions that return `Option` — callers decide how to handle `None`
    // -------------------------------------------------------------------------
    for s in ["42", "0", "not-a-number"] {
        println!("parse_positive_i32({:?}) = {:?}", s, parse_positive_i32(s));
    }

    // -------------------------------------------------------------------------
    // `Option` return type + optional struct fields — compose with `?` / `and_then`
    // -------------------------------------------------------------------------
    let guest_missing_meal = Guest {
        name: "Alex".into(),
        seat: Some("12A".into()),
        meal: None,
    };
    println!(
        "boarding_summary (incomplete): {:?}",
        boarding_summary(&guest_missing_meal)
    );

    let guest_ready = Guest {
        name: "Alex".into(),
        seat: Some("12A".into()),
        meal: Some("vegetarian".into()),
    };
    println!(
        "boarding_summary (complete): {:?}",
        boarding_summary(&guest_ready)
    );

    // `and_then`: use output of first `Option` only when `Some`, then call a fallible next step
    let doubled = parse_positive_i32("4").and_then(|n| (n <= 10).then_some(n * 2));
    println!("parse then clamp-ish map: {:?}", doubled);
}
