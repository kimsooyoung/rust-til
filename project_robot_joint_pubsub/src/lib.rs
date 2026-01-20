// Shared data structures for publisher and subscriber

use serde::{Deserialize, Serialize};

/// Represents joint angle data for a robot joint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointAngles {
    pub timestamp: u64,
    pub joint_name: String,
    pub angle_rad: f64,
    pub velocity: f64,
    pub torque: f64,
}

/// Represents the complete state of a robot with multiple joints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotState {
    pub timestamp: u64,
    pub robot_id: String,
    pub joints: Vec<JointAngles>,
}
