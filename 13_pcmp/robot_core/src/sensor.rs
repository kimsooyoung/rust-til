//! Generic sensor readings and a trait for anything that can `read()`.
//!
//! [`SensorReading<T>`] is **generic** over the measured value (`f32`, `i32`, …).
//! The [`Readable`] trait uses an **associated type** `Output` so each sensor picks
//! its own value type while still sharing one trait name.

use crate::error::RobotError;
use std::fmt::Debug;

/// One measurement from some sensor (label + typed payload).
#[derive(Debug, Clone)]
pub struct SensorReading<T> {
    pub label: String,
    pub value: T,
}

impl<T> SensorReading<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Minimal sensor interface: synchronous read that may fail.
pub trait Readable {
    /// Concrete measurement type for this sensor (e.g., `f32` centimeters).
    type Output: Clone + Debug;

    fn read(&self) -> Result<SensorReading<Self::Output>, RobotError>;
}

/// Simple forward-facing distance sensor (meters) for demos.
#[derive(Debug, Clone)]
pub struct DistanceSensor {
    id: String,
    /// Pretend this value came from hardware; kept fixed for deterministic samples.
    obstacle_meters: f32,
}

impl DistanceSensor {
    pub fn new(id: impl Into<String>, obstacle_meters: f32) -> Self {
        Self {
            id: id.into(),
            obstacle_meters,
        }
    }
}

impl Readable for DistanceSensor {
    type Output = f32;

    fn read(&self) -> Result<SensorReading<Self::Output>, RobotError> {
        if !self.obstacle_meters.is_finite() || self.obstacle_meters < 0.0 {
            return Err(RobotError::SensorReadFailed {
                sensor_id: self.id.clone(),
                message: "invalid distance".into(),
            });
        }
        Ok(SensorReading::new(format!("{} (m)", self.id), self.obstacle_meters))
    }
}
