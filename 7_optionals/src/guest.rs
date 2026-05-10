#[derive(Debug)]
pub struct Guest {
    pub name: String,
    pub seat: Option<String>,
    pub meal: Option<String>,
}

pub fn boarding_summary(guest: &Guest) -> Option<String> {
    let seat = guest.seat.as_deref()?;
    let meal = guest.meal.as_deref()?;
    Some(format!("{} @ {} ({})", guest.name, seat, meal))
}

pub fn parse_positive_i32(s: &str) -> Option<i32> {
    let n: i32 = s.parse().ok()?;
    (n > 0).then_some(n)
}
