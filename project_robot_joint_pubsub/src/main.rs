use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio::time::sleep;
use zmq::Context;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct JointAngles {
    timestamp: u64,
    joint_name: String,
    angle_rad: f64,
    velocity: f64,
    torque: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RobotState {
    timestamp: u64,
    robot_id: String,
    joints: Vec<JointAngles>,
}

#[derive(Parser)]
#[command(name = "robot_joint_pubsub")]
#[command(about = "Robot joint angles publisher/subscriber using Tokio and ZMQ")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as publisher (sends joint angles)
    Publisher {
        /// ZMQ bind address (default: tcp://*:5555)
        #[arg(short, long, default_value = "tcp://*:5555")]
        bind: String,
        /// Publishing interval in milliseconds (default: 100)
        #[arg(short, long, default_value_t = 100)]
        interval: u64,
    },
    /// Run as subscriber (receives joint angles)
    Subscriber {
        /// ZMQ connect address (default: tcp://localhost:5555)
        #[arg(short, long, default_value = "tcp://localhost:5555")]
        connect: String,
        /// Filter topic (default: robot_joints)
        #[arg(short, long, default_value = "robot_joints")]
        topic: String,
    },
}

async fn run_publisher(bind_addr: String, interval_ms: u64) -> Result<()> {
    println!("🤖 Starting Robot Joint Angles Publisher");
    println!("📡 Binding to: {}", bind_addr);
    println!("⏱️  Publishing interval: {}ms", interval_ms);

    let ctx = Context::new();
    let socket = ctx.socket(zmq::PUB)?;
    socket.bind(&bind_addr)?;

    // Give subscribers time to connect
    println!("⏳ Waiting for subscribers to connect...");
    sleep(Duration::from_millis(500)).await;

    let mut timestamp = 0u64;
    let joint_names = vec![
        "shoulder_pan",
        "shoulder_lift",
        "elbow",
        "wrist_1",
        "wrist_2",
        "wrist_3",
    ];

    println!("🚀 Publishing joint angles...\n");

    loop {
        timestamp += 1;

        // Simulate realistic joint angles for a 6-DOF robot arm
        let joints: Vec<JointAngles> = joint_names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                // Simulate sinusoidal motion for each joint
                let base_angle = (timestamp as f64 * 0.01 + i as f64) * 0.5;
                JointAngles {
                    timestamp,
                    joint_name: name.to_string(),
                    angle_rad: base_angle.sin() * 1.5,
                    velocity: base_angle.cos() * 0.1,
                    torque: (base_angle * 2.0).sin() * 5.0,
                }
            })
            .collect();

        let robot_state = RobotState {
            timestamp,
            robot_id: "robot_arm_001".to_string(),
            joints,
        };

        let topic = "robot_joints";
        let json_data = serde_json::to_string(&robot_state)?;
        let message = format!("{} {}", topic, json_data);

        socket.send(&message, 0)?;

        println!(
            "📤 [{}] Published state for {} with {} joints",
            timestamp,
            robot_state.robot_id,
            robot_state.joints.len()
        );

        sleep(Duration::from_millis(interval_ms)).await;
    }
}

async fn run_subscriber(connect_addr: String, topic_filter: String) -> Result<()> {
    println!("👂 Starting Robot Joint Angles Subscriber");
    println!("🔌 Connecting to: {}", connect_addr);
    println!("🎯 Topic filter: {}", topic_filter);

    let ctx = Context::new();
    let socket = ctx.socket(zmq::SUB)?;
    socket.connect(&connect_addr)?;
    socket.set_subscribe(topic_filter.as_bytes())?;

    println!("✅ Connected! Waiting for messages...\n");

    loop {
        let message = socket.recv_string(0)?;
        if let Ok(msg) = message {
            // Split topic and JSON data
            if let Some((topic, json_data)) = msg.split_once(' ') {
                if topic == topic_filter {
                    match serde_json::from_str::<RobotState>(json_data) {
                        Ok(robot_state) => {
                            println!("📥 Received robot state:");
                            println!("   Robot ID: {}", robot_state.robot_id);
                            println!("   Timestamp: {}", robot_state.timestamp);
                            println!("   Joints ({}):", robot_state.joints.len());
                            for joint in &robot_state.joints {
                                println!(
                                    "     - {}: {:.3} rad ({:.1}°), vel: {:.3} rad/s, torque: {:.2} N⋅m",
                                    joint.joint_name,
                                    joint.angle_rad,
                                    joint.angle_rad.to_degrees(),
                                    joint.velocity,
                                    joint.torque
                                );
                            }
                            println!();
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to parse JSON: {}", e);
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Publisher { bind, interval } => {
            run_publisher(bind, interval).await?;
        }
        Commands::Subscriber { connect, topic } => {
            run_subscriber(connect, topic).await?;
        }
    }

    Ok(())
}
