//! Shared data model for the robot joint pub/sub binaries.
//!
//! [`JointAngles`] and [`RobotState`] are the wire types serialized as JSON
//! over a ZMQ PUB/SUB socket by `bin/publisher.rs` (or the GUI publisher) and
//! decoded by `bin/subscriber.rs`. Keeping them in this library crate ensures
//! both ends share a single definition.

use serde::{Deserialize, Serialize};

/// Joint angle data sampled from a single robot joint at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointAngles {
    /// Monotonic publisher-side sequence number (not wall-clock).
    pub timestamp: u64,
    /// Human-readable joint identifier (e.g. `"shoulder_pan"`).
    pub joint_name: String,
    /// Joint angle in radians.
    pub angle_rad: f64,
    /// Joint angular velocity in rad/s.
    pub velocity: f64,
    /// Joint torque in N·m.
    pub torque: f64,
}

/// Snapshot of every joint on a single robot at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotState {
    /// Monotonic publisher-side sequence number (not wall-clock).
    pub timestamp: u64,
    /// Stable identifier for the publishing robot.
    pub robot_id: String,
    /// One [`JointAngles`] entry per joint, in publisher-defined order.
    pub joints: Vec<JointAngles>,
}
