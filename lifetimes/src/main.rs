// get_full_name_string will works
// But get_full_name_ref will not work
// because &str is a borrowed value

fn get_full_name_string() -> String {
    "Sooyoung Kim String".to_string()
}

// fn get_full_name_ref() -> &str {
//     "Sooyoung Kim"
// }
fn get_full_name_ref() -> &'static str {
    "Sooyoung Kim $str"
}

fn get_random_name<'a>(name1: &'a str, _name2: &'a str) -> &'a str {
    name1
}

#[derive(Debug)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person<'a> {
    fn first_char_of_first_name(&self) -> &'a str {
        &self.first_name[..1]
    }
}

// when input and output have same reference
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
