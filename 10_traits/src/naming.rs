//! Naming-related traits, blanket implementations, and small helpers.
//! (`Person` and its `impl Trait for Person` blocks live in `person.rs`.)

/// Associated function on a trait (not a method: no `self`) — here used like a named constructor.
pub trait InitializeWithFullname {
    fn new(full_name: &str) -> Self;
}

pub trait HasFullName {
    fn full_name(&self) -> String;
}

/// Trait bound as sugar: `&impl HasFullName` means “any reference to a type that implements `HasFullName`”.
pub fn print_full_name(person: &impl HasFullName) {
    println!("Full name: {}", person.full_name());
}

/// Generic form of the same idea: `T` must implement `HasFullName`.
pub fn print_details<T: HasFullName>(person: &T) {
    println!("Full name: {}", person.full_name());
}

/// One type parameter `T` for both arguments: `a` and `b` must be the same concrete type.
/// If you wrote `fn loose(a: &impl HasFullName, b: &impl HasFullName)`, the compiler would treat
/// that as *two* independent anonymous generics, so `a` could be `Person` and `b` some other
/// `HasFullName` type. Use a named `T` when you need them to match.
pub fn same_two<T: HasFullName>(a: &T, b: &T) {
    println!("same_two: {} / {}", a.full_name(), b.full_name());
}

pub trait CanDrive {
    fn can_drive(&self) -> bool;
}

/// Multiple bounds: `T` must implement both traits (`+` combines bounds).
pub fn print_details_with_multiple_traits<T>(person: &T)
where
    T: HasFullName + CanDrive,
{
    println!("Full name: {}", person.full_name());
    println!("Can drive: {}", person.can_drive());
}

/// Building blocks for a *supertrait* pattern: types that implement `HasName` expose name parts.
pub trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

/// Supertrait: anything that implements `HasFullNameWithName` must also implement `HasName`.
/// The default impl below can then call `first_name` / `last_name` on `Self`.
pub trait HasFullNameWithName
where
    Self: HasName,
{
    fn full_name(&self) -> String;
}

/// Blanket impl: every `T: HasName` gets `HasFullNameWithName` for free (DRY default behavior).
impl<T> HasFullNameWithName for T
where
    T: HasName,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}
