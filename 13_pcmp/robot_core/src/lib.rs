//! Robot control primitives for learning Rust features together.
//!
//! # How Rust finds these files
//!
//! Each `pub mod foo;` below tells the compiler to load `src/foo.rs` next to this
//! `lib.rs` (crate root for the library). Submodules of `foo` would live in `src/foo/`
//! if you later split further.
//!
//! # How this crate is organized
//!
//! - [`error`] — shared [`RobotError`] type used across modules.
//! - [`motor`] — structs and a trait for wheel / drive motors.
//! - [`sensor`] — generic [`SensorReading<T>`] and a [`Readable`] trait.
//! - [`ai`] — traits and a generic controller that ties sensor hints to motor power.
//! - [`hello_robot`] — narrated “how does `pcmp` connect?” tour for beginners.
//!
//! # How `pcmp` imports this crate
//!
//! The binary crate adds `robot_core` in **its** `Cargo.toml` under `[dependencies]`.
//! Rust code then imports with `use robot_core::motor::Motor;` — the path always
//! matches the **`name`** field here, not necessarily the folder name on disk.

pub mod ai;
pub mod error;
pub mod hello_robot;
pub mod motor;
pub mod sensor;
