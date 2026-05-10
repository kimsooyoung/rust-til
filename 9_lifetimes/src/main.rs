// Lifetimes (`'a`, `'static`, …)
// - Every reference `&T` has a lifetime: how long that borrow is valid.
// - The compiler often infers lifetimes (elision); you write them when the compiler cannot prove safety.
// - Returning `&str` from a function needs a known lifetime: string literals are `&'static str`;
//   a bare `-> &str` without context is ambiguous (compiler error) unless you use `'static` or tie
//   the return to input parameters (see `get_random_name`, `get_name_ref`).

/// Returns an owned `String` — no lifetime on the return type (heap-allocated, caller owns it).
fn get_full_name_string() -> String {
    "Sooyoung Kim String".to_string()
}

// This does NOT compile as written:
//   fn get_full_name_ref() -> &str { "Sooyoung Kim" }
// The compiler cannot infer *which* lifetime the returned `&str` has. String literals are valid
// for `'static`, so we spell that out explicitly:
fn get_full_name_ref() -> &'static str {
    "Sooyoung Kim (string literal)"
}

/// Both arguments share lifetime `'a`; the returned slice cannot outlive *either* borrow.
/// Here both are string literals (`'static`), so the result is effectively `'static` too.
fn get_random_name<'a>(name1: &'a str, _name2: &'a str) -> &'a str {
    name1
}

#[derive(Debug)]
struct Person<'a> {
    /// Borrows must live at least as long as `'a` (the `Person` cannot outlive these strings).
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person<'a> {
    /// Subslice of `first_name` — same lifetime `'a` as the borrowed fields.
    fn first_char_of_first_name(&self) -> &'a str {
        &self.first_name[..1]
    }
}

/// Lifetime elision: one input reference → output gets the same inferred lifetime as `name`.
fn get_name_ref(name: &str) -> &str {
    name
}

fn main() {
    let full_name = get_full_name_string();
    println!("full_name: {}", full_name);

    let full_name = get_full_name_ref();
    println!("full_name: {}", full_name);

    let random_name = get_random_name("Sooyoung Kim", "Hong Gil Dong");
    println!("random_name: {}", random_name);

    let person = Person {
        first_name: "Sooyoung",
        last_name: "Kim",
    };
    println!(
        "person: first_name: {}, last_name: {}",
        person.first_name, person.last_name
    );
    let first_char = person.first_char_of_first_name();
    println!("first_char: {}", first_char);

    let name_ref = get_name_ref("Maci Marshall");
    println!("name_ref: {}", name_ref);
}
