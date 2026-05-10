#[derive(Debug)]
pub enum SimpleIPAddressKind {
    V4(i32, i32, i32, i32),
    V6(String),
}
