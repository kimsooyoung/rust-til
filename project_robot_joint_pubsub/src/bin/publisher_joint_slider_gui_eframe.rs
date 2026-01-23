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

/// Hand pose presets for the ProHand MJCF joint naming scheme.
///
/// ## Joint naming assumptions
/// This uses **suffix matching** (e.g. `"i1_MCP"`) so it works for both `"L/i1_MCP"` and `"R/i1_MCP"`.
/// The default model (`pro-models/example/scenes/left_hand_scene.xml`) uses:
/// - Index: `i0_CMC_abd`, `i1_MCP`, `i2_PIP`, `i3_DIP`
/// - Middle: `m0_CMC_abd`, `m1_MCP`, `m2_PIP`, `m3_DIP`
/// - Ring: `r0_CMC_abd`, `r1_MCP`, `r2_PIP`, `r3_DIP`
/// - Pinky: `p0_CMC_abd`, `p1_MCP`, `p2_PIP`, `p3_DIP`
/// - Thumb: `t0_TM_abd`, `t1_TM`, `t2_CMC`, `t3_DIP`
///
/// ## Environments
/// - **Dev**: these presets are for quickly driving the MuJoCo visualization via `subscriber.rs`.
/// - **Test/CI**: prefer the headless `publisher.rs` or unit tests.
/// - **Prod**: not intended.
#[derive(Debug, Copy, Clone)]
enum HandPreset {
    Fist,
    OpenHand,
    Scissor,
    IndexFinger,
    MiddleFinger,
    RingFinger,
    PinkyFinger,
}

impl HandPreset {
    fn label(self) -> &'static str {
        match self {
            HandPreset::Fist => "Fist",
            HandPreset::OpenHand => "Open Hand",
            HandPreset::Scissor => "Scissor",
            HandPreset::IndexFinger => "Index Finger",
            HandPreset::MiddleFinger => "Middle Finger",
            HandPreset::RingFinger => "Ring Finger",
            HandPreset::PinkyFinger => "Pinky Finger",
        }
    }

    /// Apply a pose by setting `JointControl.value_rad` values and syncing `last_sent_value_rad`.
    ///
    /// Syncing `last_sent_value_rad` keeps the published velocity near zero for preset jumps, which
    /// is usually what you want for visualization-driven presets.
    fn apply(self, joints: &mut [JointControl]) {
        // Defaults: open posture, neutral abduction.
        set_joint_neutral(joints, "i0_CMC_abd");
        set_joint_neutral(joints, "m0_CMC_abd");
        set_joint_neutral(joints, "r0_CMC_abd");
        set_joint_neutral(joints, "p0_CMC_abd");
        set_joint_neutral(joints, "t0_TM_abd");

        match self {
            HandPreset::OpenHand => {
                set_finger_open(joints, 'i');
                set_finger_open(joints, 'm');
                set_finger_open(joints, 'r');
                set_finger_open(joints, 'p');
                set_thumb_open(joints);
            }
            HandPreset::Fist => {
                set_finger_curled(joints, 'i');
                set_finger_curled(joints, 'm');
                set_finger_curled(joints, 'r');
                set_finger_curled(joints, 'p');
                set_thumb_curled(joints);
            }
            HandPreset::Scissor => {
                // Index + middle extended; ring + pinky curled.
                set_finger_open(joints, 'i');
                set_finger_open(joints, 'm');
                set_finger_curled(joints, 'r');
                set_finger_curled(joints, 'p');
                set_thumb_open(joints);
            }
            HandPreset::IndexFinger => {
                set_finger_open(joints, 'i');
                set_finger_curled(joints, 'm');
                set_finger_curled(joints, 'r');
                set_finger_curled(joints, 'p');
                set_thumb_open(joints);
            }
            HandPreset::MiddleFinger => {
                set_finger_curled(joints, 'i');
                set_finger_open(joints, 'm');
                set_finger_curled(joints, 'r');
                set_finger_curled(joints, 'p');
                set_thumb_open(joints);
            }
            HandPreset::RingFinger => {
                set_finger_curled(joints, 'i');
                set_finger_curled(joints, 'm');
                set_finger_open(joints, 'r');
                set_finger_curled(joints, 'p');
                set_thumb_open(joints);
            }
            HandPreset::PinkyFinger => {
                set_finger_curled(joints, 'i');
                set_finger_curled(joints, 'm');
                set_finger_curled(joints, 'r');
                set_finger_open(joints, 'p');
                set_thumb_open(joints);
            }
        }
    }
}

/// Returns true if `full` ends with the exact joint token boundary (e.g. `"/i1_MCP"` or `"i1_MCP"`).
fn joint_name_matches_suffix(full: &str, token: &str) -> bool {
    full == token || full.ends_with(&format!("/{token}")) || full.ends_with(token)
}

fn clamp_to_range(value: f64, min: f64, max: f64) -> f64 {
    value.clamp(min, max)
}

/// Set a joint to a target value (radians), clamped to its MuJoCo range.
fn set_joint_value(joints: &mut [JointControl], token: &str, target_rad: f64) {
    for j in joints {
        if joint_name_matches_suffix(&j.name, token) {
            let v = clamp_to_range(target_rad, j.min_rad, j.max_rad);
            j.value_rad = v;
            j.last_sent_value_rad = v;
        }
    }
}

/// Set a joint to a value at a fraction of its range \([0, 1]\), where 1 means "near max".
fn set_joint_fraction_of_range(joints: &mut [JointControl], token: &str, fraction: f64) {
    let f = fraction.clamp(0.0, 1.0);
    for j in joints {
        if joint_name_matches_suffix(&j.name, token) {
            let v = j.min_rad + f * (j.max_rad - j.min_rad);
            let v = clamp_to_range(v, j.min_rad, j.max_rad);
            j.value_rad = v;
            j.last_sent_value_rad = v;
        }
    }
}

/// Neutral posture for most joints is 0.0 rad (if within range); otherwise clamp.
fn set_joint_neutral(joints: &mut [JointControl], token: &str) {
    set_joint_value(joints, token, 0.0);
}

/// Open finger posture: set MCP/PIP/DIP to neutral.
fn set_finger_open(joints: &mut [JointControl], finger: char) {
    set_joint_neutral(joints, &format!("{finger}1_MCP"));
    set_joint_neutral(joints, &format!("{finger}2_PIP"));
    set_joint_neutral(joints, &format!("{finger}3_DIP"));
}

/// Curled finger posture: drive MCP/PIP/DIP close to their maximum.
fn set_finger_curled(joints: &mut [JointControl], finger: char) {
    // 0.95 stays slightly away from the hard stop, which tends to look nicer and avoids clamping artifacts.
    set_joint_fraction_of_range(joints, &format!("{finger}1_MCP"), 0.95);
    set_joint_fraction_of_range(joints, &format!("{finger}2_PIP"), 0.95);
    set_joint_fraction_of_range(joints, &format!("{finger}3_DIP"), 0.95);
}

fn set_thumb_open(joints: &mut [JointControl]) {
    // Thumb joints in the default ProHand MJCF.
    set_joint_neutral(joints, "t1_TM");
    set_joint_neutral(joints, "t2_CMC");
    set_joint_neutral(joints, "t3_DIP");
}

fn set_thumb_curled(joints: &mut [JointControl]) {
    set_joint_fraction_of_range(joints, "t1_TM", 0.95);
    set_joint_fraction_of_range(joints, "t2_CMC", 0.95);
    set_joint_fraction_of_range(joints, "t3_DIP", 0.95);
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

            ui.group(|ui| {
                ui.label("Presets (click to set sliders and publish immediately):");
                ui.horizontal_wrapped(|ui| {
                    let presets = [
                        HandPreset::Fist,
                        HandPreset::OpenHand,
                        HandPreset::Scissor,
                        HandPreset::IndexFinger,
                        HandPreset::MiddleFinger,
                        HandPreset::RingFinger,
                        HandPreset::PinkyFinger,
                    ];
                    for p in presets {
                        if ui.button(p.label()).clicked() {
                            p.apply(&mut self.joints);
                            // Force a publish regardless of cadence so the subscriber updates instantly.
                            self.last_publish = Instant::now() - self.publish_interval;
                            self.publish_if_due();
                        }
                    }
                });
            });

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
                        j.last_sent_value_rad = 0.0;
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
