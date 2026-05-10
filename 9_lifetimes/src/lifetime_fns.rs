#[derive(Debug)]
pub struct Person<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}

impl<'a> Person<'a> {
    pub fn first_char_of_first_name(&self) -> &'a str {
        &self.first_name[..1]
    }
}

pub fn get_full_name_string() -> String {
    "Sooyoung Kim String".to_string()
}

pub fn get_full_name_ref() -> &'static str {
    "Sooyoung Kim (string literal)"
}

pub fn get_random_name<'a>(name1: &'a str, _name2: &'a str) -> &'a str {
    name1
}

pub fn get_name_ref(name: &str) -> &str {
    name
}
