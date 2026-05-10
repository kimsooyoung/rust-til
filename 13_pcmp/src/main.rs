//! `pcmp` — small **binary** that drives a pretend robot using the `robot_core` library.
//!
//! # Where dependencies are configured
//!
//! Open `Cargo.toml` next to this `src/` folder (the **crate root** for `pcmp`).
//! Under `[dependencies]` you will see:
//!
//! ```toml
//! robot_core = { path = "robot_core" }
//! ```
//!
//! That tells Cargo: “compile the library in `./robot_core` and make it available
//! as the Rust crate `robot_core`.” The import name always matches `[package].name`
//! inside `robot_core/Cargo.toml`, **not** the binary’s package name (`pcmp`).

use robot_core::ai::{Autopilot, ConservativeAi};
use robot_core::error::RobotError;
use robot_core::hello_robot;
use robot_core::motor::{Actuator, Motor};
// `DistanceSensor` implements `Readable` inside the library; callers of `tick`
// usually only need the concrete sensor type here.
use robot_core::sensor::DistanceSensor;

/// Run one control tick and print what happened, or explain the error.
fn demo_tick(
    label: &str,
    motor: &mut Motor,
    sensor: &DistanceSensor,
    pilot: &Autopilot<ConservativeAi>,
) {
    println!("--- {label} ---");
    match pilot.tick(motor, sensor) {
        Ok(reading) => {
            println!(
                "sensor {} → value {:?} | motor {} now at {}",
                reading.label,
                reading.value,
                motor.id,
                motor.current_power()
            );
        }
        Err(e) => println!("tick failed: {e}"),
    }
}

fn main() {
    // Same library the rest of `main` uses — see `robot_core/src/hello_robot.rs`.
    hello_robot::walkthrough_for_beginners();

    println!("pcmp robot demo (library code lives in ./robot_core/src/*.rs)\n");

    // Actuators: two independent motors (left / right ids are arbitrary labels).
    let mut left = Motor::new(0);
    let mut right = Motor::new(1);

    // Sensors: pretend distances to an obstacle (meters).
    let far = DistanceSensor::new("front-lidar", 3.0);
    let close = DistanceSensor::new("front-lidar", 0.5);

    // AI: generic `Autopilot<E>` where `E` is our conservative policy.
    let pilot = Autopilot::new(ConservativeAi);

    demo_tick("open space", &mut left, &far, &pilot);
    demo_tick("near obstacle", &mut right, &close, &pilot);

    // Error handling demo: illegal motor command still returns a structured error.
    println!("\n--- invalid manual power ---");
    match left.set_power(9000) {
        Ok(()) => println!("unexpected success"),
        Err(RobotError::MotorFault { motor_id, message }) => {
            println!("caught motor fault on id={motor_id}: {message}");
        }
        Err(other) => println!("other error: {other}"),
    }
}
