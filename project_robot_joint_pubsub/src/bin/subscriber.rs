// Subscriber binary - subscribes to robot joint angles via ZMQ and visualizes with MuJoCo

use anyhow::Result;
use clap::Parser;
use project_robot_joint_pubsub::RobotState;
use std::time::Duration;
use zmq::Context;

use mujoco_rs::cpp_viewer::MjViewerCpp;
use mujoco_rs::prelude::*;

// MuJoCo model matching the example - ball with free joint
const MUJOCO_MODEL: &str = "
<mujoco>
  <worldbody>
    <light ambient=\"0.2 0.2 0.2\"/>
    <body name=\"ball\">
        <geom name=\"green_sphere\" size=\".1\" rgba=\"0 1 0 1\" solref=\"0.004 1.0\"/>
        <joint name=\"ball_joint\" type=\"free\"/>
    </body>

    <geom name=\"floor1\" type=\"plane\" size=\"10 10 1\" euler=\"15 4 0\" solref=\"0.004 1.0\"/>
    <geom name=\"floor2\" type=\"plane\" pos=\"15 -20 0\" size=\"10 10 1\" euler=\"-15 -4 0\" solref=\"0.004 1.0\"/>

  </worldbody>
</mujoco>
";

#[derive(Parser)]
#[command(name = "subscriber")]
#[command(about = "Robot joint angles subscriber with MuJoCo visualization")]
struct Cli {
    /// ZMQ connect address (default: tcp://localhost:5555)
    #[arg(short, long, default_value = "tcp://localhost:5555")]
    connect: String,
    /// Filter topic (default: robot_joints)
    #[arg(short, long, default_value = "robot_joints")]
    topic: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("👂 Starting Robot Joint Angles Subscriber with MuJoCo Visualization");
    println!("🔌 Connecting to: {}", cli.connect);
    println!("🎯 Topic filter: {}", cli.topic);

    // Load MuJoCo model and create data
    println!("📦 Loading MuJoCo model...");
    let model = MjModel::from_xml_string(MUJOCO_MODEL)
        .map_err(|e| anyhow::anyhow!("Failed to load MuJoCo model: {:?}", e))?;
    let mut data = MjData::new(&model);

    // Launch MuJoCo C++ viewer
    println!("🎬 Launching MuJoCo C++ viewer...");
    let mut viewer = MjViewerCpp::launch_passive(&model, &data, 100);

    // Get joint info for ball_joint
    let ball_info = data
        .joint("ball_joint")
        .ok_or_else(|| anyhow::anyhow!("ball_joint not found in model"))?;

    // Get timestep from model
    let timestep = model.opt().timestep;

    // Connect to ZMQ publisher
    println!("📡 Connecting to ZMQ publisher...");
    let ctx = Context::new();
    let socket = ctx.socket(zmq::SUB)?;
    socket.connect(&cli.connect)?;
    socket.set_subscribe(cli.topic.as_bytes())?;

    // Set socket to non-blocking so we can check for messages without blocking the viewer
    socket.set_rcvtimeo(10)?; // 10ms timeout

    println!("✅ Ready! Waiting for joint data and visualizing...\n");

    let mut last_received_timestamp = 0u64;

    // Main loop: check for ZMQ messages and update simulation
    while viewer.running() {
        // Try to receive a message (non-blocking)
        match socket.recv_string(zmq::DONTWAIT) {
            Ok(Ok(msg)) => {
                // Split topic and JSON data
                if let Some((topic, json_data)) = msg.split_once(' ') {
                    if topic == cli.topic {
                        match serde_json::from_str::<RobotState>(json_data) {
                            Ok(robot_state) => {
                                // Update only if we have new data
                                if robot_state.timestamp > last_received_timestamp {
                                    last_received_timestamp = robot_state.timestamp;

                                    // Find the ball_joint in received data
                                    if let Some(joint) = robot_state
                                        .joints
                                        .iter()
                                        .find(|j| j.joint_name == "ball_joint")
                                    {
                                        // Update ball position based on joint data
                                        // For a free joint, qpos[0..3] are x, y, z positions
                                        // We'll use the angle_rad to control the x position
                                        // and create a circular motion pattern
                                        let mut view = ball_info.view_mut(&mut data);
                                        let t = joint.angle_rad;

                                        // Update position (x, y, z) - create circular motion
                                        view.qpos[0] = t.sin() * 2.0; // x position
                                        view.qpos[1] = t.cos() * 2.0; // y position
                                        view.qpos[2] = 0.5 + t.sin() * 0.3; // z position (bouncing)

                                        // Update velocity if needed (qvel[0..3] are linear velocities)
                                        view.qvel[0] = joint.velocity * t.cos();
                                        view.qvel[1] = -joint.velocity * t.sin();
                                        view.qvel[2] = joint.velocity * t.cos() * 0.1;

                                        println!(
                                            "📥 [{}] Updated ball position: x={:.2}, y={:.2}, z={:.2}",
                                            robot_state.timestamp,
                                            view.qpos[0],
                                            view.qpos[1],
                                            view.qpos[2]
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to parse JSON: {}", e);
                            }
                        }
                    }
                }
            }
            Ok(Err(_)) => {
                // No message available, continue with simulation
            }
            Err(zmq::Error::EAGAIN) => {
                // Timeout - no message available, continue
            }
            Err(e) => {
                eprintln!("❌ ZMQ receive error: {}", e);
            }
        }

        // Sync and render C++ viewer (sync doesn't take parameters, render needs explicit call)
        // Order: sync -> render -> step -> sleep (as per mujoco-rs C++ viewer guidance)
        viewer.sync();
        viewer.render(true); // render on screen and update the fps timer

        // Step the simulation
        data.step();

        // Sleep to match simulation timestep
        std::thread::sleep(Duration::from_secs_f64(timestep));
    }

    println!("👋 Viewer closed. Exiting...");
    Ok(())
}
