//! Small helper used by every topic to print a titled section in the terminal.

/// Prints a clear heading so output from different topics is easy to scan.
pub fn section(title: &str) {
    println!("\n=== {} ===\n", title);
}
