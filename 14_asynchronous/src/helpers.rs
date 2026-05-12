//! Shared helpers for chapter 14 — section banners for readable demo output.

/// Print a banner around `title` so each demo's output is easy to spot.
pub fn section(title: &str) {
    println!("\n=== {} ===\n", title);
}
