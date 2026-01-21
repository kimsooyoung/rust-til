//! Subscriber binary: receives robot joint angles via ZMQ and visualizes them in MuJoCo.
//!
//! Behind the scenes (high level):
//! - Loads an MJCF model from disk (supports `<include .../>`).
//! - Starts MuJoCo's C++ viewer (via `mujoco-rs` `cpp-viewer` feature).
//! - Receives `RobotState` messages and applies joint positions/velocities into `MjData`.
//! - Each loop: sync viewer state → render UI → run `mj_forward` (no time integration).

use anyhow::Result;
use clap::Parser;
use project_robot_joint_pubsub::RobotState;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use zmq::Context;

use mujoco_rs::cpp_viewer::MjViewerCpp;
use mujoco_rs::prelude::*;

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
    /// MJCF model path (supports `<include/>`), relative to `project_robot_joint_pubsub/`
    #[arg(long, default_value = "pro-models/example/scenes/left_hand_scene.xml")]
    model: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("👂 Starting Robot Joint Angles Subscriber with MuJoCo Visualization");
    println!("🔌 Connecting to: {}", cli.connect);
    println!("🎯 Topic filter: {}", cli.topic);
    println!("📄 Model: {}", cli.model.display());

    // Load MuJoCo model and create data
    println!("📦 Loading MuJoCo model...");
    // Use `from_xml(path)` (file-based) so MuJoCo can resolve `<include file="..."/>`
    // relative to the MJCF file location.
    let model_path = std::fs::canonicalize(&cli.model).map_err(|e| {
        anyhow::anyhow!(
            "Failed to resolve model path '{}': {e}",
            cli.model.display()
        )
    })?;
    let model = MjModel::from_xml(&model_path)
        .map_err(|e| anyhow::anyhow!("Failed to load MJCF '{}': {:?}", model_path.display(), e))?;
    let mut data = MjData::new(&model);

    // Cache joint handles once (avoids repeated `mj_name2id` calls on every message).
    // This is important for hand models with many joints at higher publish rates.
    let mut joint_cache: HashMap<String, MjJointDataInfo> = HashMap::new();
    let njnt = model.ffi().njnt.max(0) as usize;
    for id in 0..njnt {
        let Some(name) = model.id_to_name(MjtObj::mjOBJ_JOINT, id as i32) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        if let Some(info) = data.joint(name) {
            joint_cache.insert(name.to_string(), info);
        }
    }

    // Launch MuJoCo C++ viewer
    println!("🎬 Launching MuJoCo C++ viewer...");
    let mut viewer = MjViewerCpp::launch_passive(&model, &data, 100);

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

                                    // Apply joint updates by name.
                                    //
                                    // Notes:
                                    // - Many hand joints are hinge joints (1 DoF): `qpos[0]` is the angle, `qvel[0]` is angular velocity.
                                    // - For more complex joints (e.g., `free` or `ball`), this simplistic mapping won't be sufficient.
                                    //   We intentionally "best-effort" update only the first DoF if present.
                                    let mut _updated = 0usize;
                                    for joint in &robot_state.joints {
                                        let Some(joint_info) = joint_cache.get(&joint.joint_name)
                                        else {
                                            continue;
                                        };

                                        let mut view = joint_info.view_mut(&mut data);
                                        if let Some(qpos0) = view.qpos.get_mut(0) {
                                            *qpos0 = joint.angle_rad;
                                        }
                                        if let Some(qvel0) = view.qvel.get_mut(0) {
                                            *qvel0 = joint.velocity;
                                        }
                                        _updated += 1;
                                    }

                                    // Intentionally no per-message logging here:
                                    // printing at high frequency significantly slows down the render loop,
                                    // and this subscriber is intended for real-time visualization.
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
        // Order: sync -> render -> forward -> sleep
        viewer.sync();
        viewer.render(true); // render on screen and update the fps timer

        // For pose visualization driven by external joint angles, we do *not* integrate time.
        // `forward()` updates all derived quantities (kinematics/dynamics) from the current state.
        data.forward();

        // Sleep to match simulation timestep
        std::thread::sleep(Duration::from_secs_f64(timestep));
    }

    println!("👋 Viewer closed. Exiting...");
    Ok(())
}
