use crate::guest::{boarding_summary, parse_positive_i32, Guest};

pub fn run() {
    for s in ["42", "0", "not-a-number"] {
        println!("parse_positive_i32({:?}) = {:?}", s, parse_positive_i32(s));
    }

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

    let doubled = parse_positive_i32("4").and_then(|n| (n <= 10).then_some(n * 2));
    println!("parse then clamp-ish map: {:?}", doubled);
}
