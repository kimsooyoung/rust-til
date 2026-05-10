//! Lightweight “AI” layer: traits + a generic wrapper that composes a policy.
//!
//! This module demonstrates:
//! - A small [`DecisionEngine`] trait (policy).
//! - A generic [`Autopilot<E>`] that works with **any** engine type `E` that implements
//!   the trait (monomorphized at compile time — no `dyn` needed for this demo).

use crate::error::RobotError;
use crate::motor::{Actuator, MAX_POWER, MIN_POWER};
use crate::sensor::{Readable, SensorReading};

/// Turns sensor context into a motor command suggestion.
pub trait DecisionEngine {
    /// Given a forward distance in **meters**, suggest motor power in [`MIN_POWER`], [`MAX_POWER`].
    fn suggest_power(&self, obstacle_m: f32) -> Result<i16, RobotError>;
}

/// Keeps the robot cautious: slow down when something is close.
#[derive(Debug, Default, Clone, Copy)]
pub struct ConservativeAi;

impl DecisionEngine for ConservativeAi {
    fn suggest_power(&self, obstacle_m: f32) -> Result<i16, RobotError> {
        if obstacle_m < 0.0 || !obstacle_m.is_finite() {
            return Err(RobotError::PlanningFailed(
                "distance must be a finite, non-negative meter value".into(),
            ));
        }
        // Farther than 2 m → full forward; closer → scale down; very close → reverse slightly.
        let power = if obstacle_m > 2.0 {
            MAX_POWER
        } else if obstacle_m > 0.25 {
            ((obstacle_m / 2.0) * f32::from(MAX_POWER)) as i16
        } else {
            -25
        };
        Ok(power.clamp(MIN_POWER, MAX_POWER))
    }
}

/// Generic autopilot: holds any `DecisionEngine` implementation.
#[derive(Debug, Clone)]
pub struct Autopilot<E: DecisionEngine> {
    engine: E,
}

impl<E: DecisionEngine> Autopilot<E> {
    pub fn new(engine: E) -> Self {
        Self { engine }
    }

    /// Read a sensor, ask the engine for power, then command an actuator.
    pub fn tick<A, S>(&self, actuator: &mut A, sensor: &S) -> Result<SensorReading<S::Output>, RobotError>
    where
        A: Actuator,
        S: Readable,
        S::Output: Into<f32>,
    {
        let reading = sensor.read()?;
        let distance_m: f32 = reading.value.clone().into();
        let power = self.engine.suggest_power(distance_m)?;
        actuator.set_power(power)?;
        Ok(reading)
    }
}
