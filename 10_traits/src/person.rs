//! The `Person` type and all `impl … for Person` blocks.

use std::fmt;

use crate::naming::{
    CanDrive, HasFullName, HasName, InitializeWithFullname,
};

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

impl InitializeWithFullname for Person {
    fn new(full_name: &str) -> Self {
        // Demo: assumes at least two whitespace-separated parts (otherwise `parts[1]` panics).
        let parts = full_name.split_whitespace().collect::<Vec<&str>>();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 0,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({} years old)",
            self.first_name, self.last_name, self.age
        )
    }
}

impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl CanDrive for Person {
    fn can_drive(&self) -> bool {
        self.age >= 18
    }
}

impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
}
