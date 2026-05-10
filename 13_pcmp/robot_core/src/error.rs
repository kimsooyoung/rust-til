//! Shared error type for motors, sensors, and planners.
//!
//! Centralizing errors keeps each feature module small while still letting
//! `main` (or other crates) match on a single `RobotError` enum.

use std::fmt;

/// Anything that can go wrong while driving or sensing.
#[derive(Debug, Clone)]
pub enum RobotError {
    /// Motor rejected a command (out of range, fault, etc.).
    MotorFault {
        motor_id: u8,
        message: String,
    },
    /// A sensor could not produce a reading.
    SensorReadFailed {
        sensor_id: String,
        message: String,
    },
    /// High-level planner / AI refused to act.
    PlanningFailed(String),
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotError::MotorFault { motor_id, message } => {
                write!(f, "motor {motor_id}: {message}")
            }
            RobotError::SensorReadFailed { sensor_id, message } => {
                write!(f, "sensor {sensor_id}: {message}")
            }
            RobotError::PlanningFailed(msg) => write!(f, "planner: {msg}"),
        }
    }
}

impl std::error::Error for RobotError {}
