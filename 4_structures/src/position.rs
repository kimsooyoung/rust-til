//! Tuple struct used by `ex02_tuple_struct.rs` — three `i32` components with methods.

#[derive(Debug)]
pub struct Position(pub i32, pub i32, pub i32);

impl Position {
    pub fn twice(&self) -> Position {
        Position(self.0 * 2, self.1 * 2, self.2 * 2)
    }

    pub fn make_twice(&mut self) {
        self.0 *= 2;
        self.1 *= 2;
        self.2 *= 2;
    }

    pub fn describe(&self) {
        println!("Position is at ({}, {}, {})", self.0, self.1, self.2);
    }

    pub fn zero() -> Position {
        Position(0, 0, 0)
    }
}
