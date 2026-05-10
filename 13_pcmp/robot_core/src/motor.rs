//! Motor modeling: a concrete `struct`, an `Actuator` trait, and validation.
//!
//! This module only knows about [`crate::error::RobotError`] for failures — it does
//! not depend on sensors or AI, which keeps the dependency direction clean.

use crate::error::RobotError;

/// Allowed power band for the demo robot (percent-like, symmetric).
pub const MIN_POWER: i16 = -100;
pub const MAX_POWER: i16 = 100;

/// One drive motor with a stable identifier.
#[derive(Debug, Clone)]
pub struct Motor {
    /// Human / log friendly id (e.g., left wheel = 0).
    pub id: u8,
    current_power: i16,
}

impl Motor {
    /// Start a motor at rest (`0` power).
    pub fn new(id: u8) -> Self {
        Self {
            id,
            current_power: 0,
        }
    }

    /// Last commanded power (useful for logging / closed-loop checks).
    pub fn current_power(&self) -> i16 {
        self.current_power
    }
}

/// Anything that can accept a signed power command (motors, mocked test doubles, …).
pub trait Actuator {
    /// Apply signed power in the range [`MIN_POWER`], [`MAX_POWER`].
    fn set_power(&mut self, power: i16) -> Result<(), RobotError>;
}

impl Actuator for Motor {
    fn set_power(&mut self, power: i16) -> Result<(), RobotError> {
        if !(MIN_POWER..=MAX_POWER).contains(&power) {
            return Err(RobotError::MotorFault {
                motor_id: self.id,
                message: format!("power {power} out of range {MIN_POWER}..={MAX_POWER}"),
            });
        }
        self.current_power = power;
        Ok(())
    }
}
