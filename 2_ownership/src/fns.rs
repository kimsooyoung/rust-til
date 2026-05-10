//! Small functions used by the ownership demos: moves, borrows, and returning owned data.

/// Takes ownership of `s`, prints its length, then returns the same `String` so the caller owns it again.
pub fn get_str_len_with_ownership(s: String) -> String {
    let str_len = s.len();
    println!("string length of {} is {}", s, str_len);
    s
}

/// Immutable borrow: only reads `s`; the caller keeps the owned `String`.
pub fn get_str_len_wo_ownership(s: &str) -> usize {
    s.len()
}

/// Clears the buffer in place; caller still owns the `String`.
pub fn clear_string(s: &mut String) {
    s.clear();
}

pub fn print_str_len(s: &str) {
    println!("print_str_len: {} (len {})", s, s.len());
}

/// Returns a fresh heap `String` owned by the caller.
pub fn get_name_owned() -> String {
    "Sooyoung".to_string()
}

/// String literals live for `'static`; returning `&'static str` is always valid.
pub fn get_name_static() -> &'static str {
    "Sooyoung"
}
