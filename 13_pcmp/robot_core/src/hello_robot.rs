//! Beginner-friendly “map” of how the `pcmp` binary and `robot_core` fit together.
//!
//! This module is **not** new robot logic — it only prints a short story and runs
//! one tiny example so you can see the same building blocks `pcmp/src/main.rs` uses.

use crate::ai::{Autopilot, ConservativeAi};
use crate::motor::Motor;
use crate::sensor::DistanceSensor;

/// Print how pieces connect, then run **one** autopilot tick (same idea as `pcmp`).
///
/// Call this from `pcmp` (or your own binary) when you want a gentle tour before
/// the full demo.
pub fn walkthrough_for_beginners() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║ hello_robot — how `pcmp` talks to `robot_core`             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("1) Crate wiring (no Rust code here — only Cargo metadata)");
    println!("   • Folder `pcmp/` is the **binary** crate.");
    println!("   • Its `Cargo.toml` has:  robot_core = {{ path = \"robot_core\" }}");
    println!("   • That means: “build the library in `pcmp/robot_core/` next to this file.”");
    println!();
    println!("2) Import paths in `pcmp/src/main.rs`");
    println!("   • `use robot_core::motor::Motor;`  ← `robot_core` is the LIBRARY name");
    println!("     (from `robot_core/Cargo.toml` → [package] name = \"robot_core\").");
    println!("   • After `motor::` you pick a module inside `robot_core/src/motor.rs`.");
    println!();
    println!("3) One control loop in plain words");
    println!("   • Read a sensor  →  ask AI for power  →  command a motor.");
    println!("   • In code, `Autopilot::tick` does those three steps and returns `Result`.");
    println!();
    println!("─── minimal live example (one tick) ───");

    let mut motor = Motor::new(42);
    let sensor = DistanceSensor::new("tutorial-lidar", 1.5);
    let pilot = Autopilot::new(ConservativeAi);

    match pilot.tick(&mut motor, &sensor) {
        Ok(reading) => {
            println!(
                "   sensor reading: {} = {:?}",
                reading.label, reading.value
            );
            println!(
                "   motor id {} is now at power {}",
                motor.id,
                motor.current_power()
            );
        }
        Err(e) => println!("   tick failed: {e}"),
    }

    println!();
    println!("(End of hello_robot — the `pcmp` program below repeats the same ideas");
    println!(" with more motors, sensors, and an error-handling example.)");
    println!();
}
