#![allow(clippy::result_unit_err)]

pub fn get_user_name() -> Result<&'static str, ()> {
    Ok("Sooyoung")
}

pub fn get_user_name_err() -> Result<&'static str, ()> {
    Err(())
}

pub fn get_first_name() -> Result<String, ()> {
    Ok("Sooyoung".to_string())
}

pub fn get_last_name() -> Result<String, ()> {
    Err(())
}

pub fn get_full_name() -> Result<String, ()> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

pub fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

pub fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}
