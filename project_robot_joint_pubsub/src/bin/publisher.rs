// Publisher binary - publishes robot joint angles via ZMQ

use anyhow::Result;
use clap::Parser;
use project_robot_joint_pubsub::{JointAngles, RobotState};
use std::time::Duration;
use tokio::time::sleep;
use zmq::Context;

#[derive(Parser)]
#[command(name = "publisher")]
#[command(about = "Robot joint angles publisher using Tokio and ZMQ")]
struct Cli {
    /// ZMQ bind address (default: tcp://*:5555)
    #[arg(short, long, default_value = "tcp://*:5555")]
    bind: String,
    /// Publishing interval in milliseconds (default: 100)
    #[arg(short, long, default_value_t = 100)]
    interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("🤖 Starting Robot Joint Angles Publisher");
    println!("📡 Binding to: {}", cli.bind);
    println!("⏱️  Publishing interval: {}ms", cli.interval);

    let ctx = Context::new();
    let socket = ctx.socket(zmq::PUB)?;
    socket.bind(&cli.bind)?;

    // Give subscribers time to connect
    println!("⏳ Waiting for subscribers to connect...");
    sleep(Duration::from_millis(500)).await;

    let mut timestamp = 0u64;
    let joint_name = "ball_joint";

    println!("🚀 Publishing joint angles for {}...\n", joint_name);

    loop {
        timestamp += 1;

        // Simulate ball position using sinusoidal motion
        // For a free joint, we'll simulate x, y, z position changes
        let t = timestamp as f64 * 0.01;
        let angle_rad = t.sin() * 2.0; // Simulated angle/position component
        let velocity = t.cos() * 0.1; // Velocity component
        let torque = (t * 2.0).sin() * 0.5; // Torque component

        let joint = JointAngles {
            timestamp,
            joint_name: joint_name.to_string(),
            angle_rad,
            velocity,
            torque,
        };

        let robot_state = RobotState {
            timestamp,
            robot_id: "ball_robot".to_string(),
            joints: vec![joint],
        };

        let topic = "robot_joints";
        let json_data = serde_json::to_string(&robot_state)?;
        let message = format!("{} {}", topic, json_data);

        socket.send(&message, 0)?;

        println!(
            "📤 [{}] Published {}: angle={:.3} rad, vel={:.3} rad/s, torque={:.2} N⋅m",
            timestamp,
            joint_name,
            robot_state.joints[0].angle_rad,
            robot_state.joints[0].velocity,
            robot_state.joints[0].torque
        );

        sleep(Duration::from_millis(cli.interval)).await;
    }
}
