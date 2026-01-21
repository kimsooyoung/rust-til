//! GUI publisher binary: publishes robot joint angles from user-controlled sliders (egui).
//!
//! ## What this does
//! - Loads an MJCF model from disk (supports `<include file="..."/>`).
//! - Enumerates MuJoCo joint names and (when available) joint limits.
//! - Presents an egui UI with one slider per joint.
//! - Publishes `RobotState` messages over ZMQ `PUB` at a fixed rate.
//!
//! ## Message format (wire protocol)
//! The publisher sends a single ZMQ string message in the form:
//! `"{topic} {json}"`, where `{json}` is a serialized `RobotState`.
//! This matches the format used by `subscriber.rs`.
//!
//! ## Environments
//! - **Dev**: run this GUI publisher and the MuJoCo subscriber locally.
//! - **Test/CI**: keep using `publisher.rs` (headless random publisher) or `cargo test`.
//! - **Prod**: typically not used; this is a developer control tool.

use anyhow::Result;
use clap::Parser;
use eframe::egui;
use mujoco_rs::prelude::*;
use project_robot_joint_pubsub::{JointAngles, RobotState};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use zmq::Context;

/// Conservative fallback range (radians) for joints without limits in the model.
const DEFAULT_UNLIMITED_RANGE_RAD: std::ops::RangeInclusive<f64> = -1.5..=1.5;

#[derive(Parser, Debug, Clone)]
#[command(name = "publisher_joint_slider_gui_eframe")]
#[command(about = "GUI publisher: egui sliders -> ZMQ RobotState (for MuJoCo subscriber)")]
struct Cli {
    /// ZMQ bind address (default: tcp://*:5555)
    #[arg(long, default_value = "tcp://*:5555")]
    bind: String,

    /// Publish topic prefix (default: robot_joints)
    #[arg(long, default_value = "robot_joints")]
    topic: String,

    /// Publishing rate in Hz (default: 50)
    #[arg(long, default_value_t = 50)]
    publish_hz: u64,

    /// MJCF model path (supports `<include/>`), relative to `project_robot_joint_pubsub/`
    #[arg(long, default_value = "pro-models/example/scenes/left_hand_scene.xml")]
    model: PathBuf,

    /// Robot identifier included in the published message (default: pro_hand)
    #[arg(long, default_value = "pro_hand")]
    robot_id: String,

    /// Optional joint name prefixes to include (repeatable or comma-separated).
    /// If omitted, all joints in the MJCF are shown.
    ///
    /// Examples:
    /// - `--filter-prefix finger_`
    /// - `--filter-prefix finger_,thumb_`
    #[arg(long, value_delimiter = ',', num_args = 0..)]
    filter_prefix: Vec<String>,
}

#[derive(Debug, Clone)]
struct JointControl {
    name: String,
    value_rad: f64,
    min_rad: f64,
    max_rad: f64,
    last_sent_value_rad: f64,
}

struct GuiPublisherApp {
    cli: Cli,
    socket: zmq::Socket,
    publish_interval: Duration,
    last_publish: Instant,
    seq: u64,
    joints: Vec<JointControl>,
}

impl GuiPublisherApp {
    fn new(cli: Cli) -> Result<Self> {
        let model_path = std::fs::canonicalize(&cli.model).map_err(|e| {
            anyhow::anyhow!(
                "Failed to resolve model path '{}': {e}",
                cli.model.display()
            )
        })?;
        let model = MjModel::from_xml(&model_path).map_err(|e| {
            anyhow::anyhow!("Failed to load MJCF '{}': {:?}", model_path.display(), e)
        })?;

        let publish_hz = cli.publish_hz.max(1);
        let publish_interval = Duration::from_secs_f64(1.0 / publish_hz as f64);

        let ctx = Context::new();
        let socket = ctx.socket(zmq::PUB)?;
        socket.bind(&cli.bind)?;

        let mut joints = enumerate_joint_controls(&model, &cli.filter_prefix);
        // Keep ordering stable and user-friendly.
        joints.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self {
            cli,
            socket,
            publish_interval,
            last_publish: Instant::now(),
            seq: 0,
            joints,
        })
    }

    fn publish_if_due(&mut self) {
        if self.last_publish.elapsed() < self.publish_interval {
            return;
        }

        let dt = self.last_publish.elapsed().as_secs_f64().max(1e-9);
        self.last_publish = Instant::now();
        self.seq += 1;

        let joints: Vec<JointAngles> = self
            .joints
            .iter_mut()
            .map(|j| {
                let vel = (j.value_rad - j.last_sent_value_rad) / dt;
                j.last_sent_value_rad = j.value_rad;
                JointAngles {
                    timestamp: self.seq,
                    joint_name: j.name.clone(),
                    angle_rad: j.value_rad,
                    velocity: vel,
                    torque: 0.0,
                }
            })
            .collect();

        let robot_state = RobotState {
            timestamp: self.seq,
            robot_id: self.cli.robot_id.clone(),
            joints,
        };

        let Ok(json) = serde_json::to_string(&robot_state) else {
            return;
        };
        let msg = format!("{} {}", self.cli.topic, json);
        let _ = self.socket.send(&msg, 0);
    }
}

impl eframe::App for GuiPublisherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.publish_if_due();

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Bind: {}", self.cli.bind));
                ui.separator();
                ui.label(format!("Topic: {}", self.cli.topic));
                ui.separator();
                ui.label(format!("Hz: {}", self.cli.publish_hz.max(1)));
                ui.separator();
                ui.label(format!("Robot: {}", self.cli.robot_id));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Joint sliders");
            ui.label("Move sliders to publish joint angles (radians).");

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for j in &mut self.joints {
                        // Avoid borrowing `j` immutably while also borrowing `j.value_rad` mutably.
                        let range = j.min_rad..=j.max_rad;
                        ui.add(egui::Slider::new(&mut j.value_rad, range).text(&j.name));
                    }
                });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Publish once now").clicked() {
                    // Force a publish regardless of cadence.
                    self.last_publish = Instant::now() - self.publish_interval;
                    self.publish_if_due();
                }
                if ui.button("Zero all joints").clicked() {
                    for j in &mut self.joints {
                        j.value_rad = 0.0;
                    }
                }
            });
        });

        // This is a real-time control surface; keep repainting.
        ctx.request_repaint();
    }
}

fn enumerate_joint_controls(model: &MjModel, filter_prefix: &[String]) -> Vec<JointControl> {
    let njnt = model.ffi().njnt.max(0) as usize;

    let limited = model.jnt_limited();
    let range = model.jnt_range();

    let mut out = Vec::with_capacity(njnt);

    for id in 0..njnt {
        let Some(name) = model.id_to_name(MjtObj::mjOBJ_JOINT, id as i32) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }

        if !filter_prefix.is_empty()
            && !filter_prefix
                .iter()
                .any(|p| !p.is_empty() && name.starts_with(p))
        {
            continue;
        }

        let (min_rad, max_rad) = if limited.get(id).copied().unwrap_or(false) {
            let r = range.get(id).copied().unwrap_or([
                *DEFAULT_UNLIMITED_RANGE_RAD.start(),
                *DEFAULT_UNLIMITED_RANGE_RAD.end(),
            ]);
            let (a, b) = (r[0] as f64, r[1] as f64);
            if a.is_finite() && b.is_finite() && a < b {
                (a, b)
            } else {
                (
                    *DEFAULT_UNLIMITED_RANGE_RAD.start(),
                    *DEFAULT_UNLIMITED_RANGE_RAD.end(),
                )
            }
        } else {
            (
                *DEFAULT_UNLIMITED_RANGE_RAD.start(),
                *DEFAULT_UNLIMITED_RANGE_RAD.end(),
            )
        };

        out.push(JointControl {
            name: name.to_string(),
            value_rad: 0.0,
            min_rad,
            max_rad,
            last_sent_value_rad: 0.0,
        });
    }

    out
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ProHand Joint Publisher (egui)",
        native_options,
        Box::new(move |_cc| Ok(Box::new(GuiPublisherApp::new(cli)?))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to start egui app: {e}"))?;

    Ok(())
}
