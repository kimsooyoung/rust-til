# Robot Joint Angles Pub/Sub with Tokio and ZMQ

A robotics project demonstrating asynchronous publish/subscribe messaging for robot joint angles using Tokio and ZeroMQ (ZMQ).

## Features

- **Publisher**: Simulates a robot arm sending joint angles periodically
- **Subscriber**: Receives and displays joint angle data in real-time
- **6-DOF Robot Arm Simulation**: Simulates realistic joint angles for a 6-degree-of-freedom robot arm
- **Async/Await**: Uses Tokio for asynchronous operations
- **ZMQ Pub/Sub**: Uses ZeroMQ PUB/SUB pattern for messaging
- **JSON Serialization**: Joint data is serialized as JSON for easy integration

## Data Structure

Each message contains:
- **Robot ID**: Identifier for the robot
- **Timestamp**: Message sequence number
- **Joints**: Array of joint data including:
  - Joint name (e.g., "shoulder_pan", "elbow")
  - Angle in radians
  - Velocity in rad/s
  - Torque in N⋅m

## Setup

### MuJoCo Library with C++ Viewer Support

This project uses MuJoCo physics simulation library with **C++ viewer support**. The C++ viewer provides the full-featured MuJoCo Simulate UI with advanced capabilities.

**Important:** Unlike the auto-download feature, the C++ viewer requires building a modified MuJoCo library from source. This is a one-time setup process.

**Prerequisites:**
- CMake (3.10 or higher)
- C++ compiler (GCC or Clang)
- Git
- Build tools (make, ninja, etc.)

**Build Steps:**

1. **Run the build script:**
   ```bash
   cd project_robot_joint_pubsub
   ./build_mujoco_cpp.sh
   ```

   This script will:
   - Clone the mujoco-rs repository (if not already present) to `../mujoco-rs`
   - Initialize the modified MuJoCo submodule
   - Build MuJoCo statically with C++ viewer support (`libsimulate`)
   - Output the library directory path

2. **Set the environment variable:**
   ```bash
   # The script will output the library directory, set it like this:
   export MUJOCO_STATIC_LINK_DIR="/path/to/mujoco-rs/mujoco/build"
   
   # Or add it to your shell profile (~/.bashrc, ~/.zshrc, etc.) for persistence:
   echo 'export MUJOCO_STATIC_LINK_DIR="/path/to/mujoco-rs/mujoco/build"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Build and run:**
   ```bash
   # Using justfile (checks for MUJOCO_STATIC_LINK_DIR)
   just run-robot-subscriber
   
   # Or manually:
   cargo build --bin subscriber
   cargo run --bin subscriber
   ```

**Note:** 
- The C++ viewer requires static linking, so `auto-download-mujoco` feature is not used
- The build script clones mujoco-rs to `../mujoco-rs` by default (can be changed via `MUJOCO_RS_REPO_DIR` env var)
- Building MuJoCo can take several minutes depending on your system
- The mujoco-rs repository and build artifacts are excluded from git (via `.gitignore`)

## Usage

### Build the project

```bash
# Using justfile (automatically handles MUJOCO_DOWNLOAD_DIR)
just run-robot-publisher
just run-robot-subscriber

# Or manually:
MUJOCO_DOWNLOAD_DIR="$(realpath mujoco_libs)" cargo build --release
```

### Run the Publisher

In one terminal, start the publisher:

```bash
# Using justfile (recommended)
just run-robot-publisher

# Or with cargo directly
MUJOCO_DOWNLOAD_DIR="$(realpath mujoco_libs)" cargo run --bin publisher
```

Or with custom options:

```bash
# Custom bind address and interval
MUJOCO_DOWNLOAD_DIR="$(realpath mujoco_libs)" cargo run --bin publisher -- --bind tcp://*:5556 --interval 50
```

### Run the Subscriber

In another terminal, start the subscriber:

```bash
# Using justfile (recommended)
just run-robot-subscriber

# Or with cargo directly (requires LD_LIBRARY_PATH for subscriber)
MUJOCO_DOWNLOAD_DIR="$(realpath mujoco_libs)" \
LD_LIBRARY_PATH="$(realpath mujoco_libs/mujoco-3.3.7/lib):$LD_LIBRARY_PATH" \
cargo run --bin subscriber
```

Or with custom options:

```bash
# Custom connect address and topic
MUJOCO_DOWNLOAD_DIR="$(realpath mujoco_libs)" \
LD_LIBRARY_PATH="$(realpath mujoco_libs/mujoco-3.3.7/lib):$LD_LIBRARY_PATH" \
cargo run --bin subscriber -- --connect tcp://localhost:5556 --topic robot_joints
```

## Example Output

**Publisher:**
```
🤖 Starting Robot Joint Angles Publisher
📡 Binding to: tcp://*:5555
⏱️  Publishing interval: 100ms
⏳ Waiting for subscribers to connect...
🚀 Publishing joint angles...

📤 [1] Published state for robot_arm_001 with 6 joints
📤 [2] Published state for robot_arm_001 with 6 joints
...
```

**Subscriber:**
```
👂 Starting Robot Joint Angles Subscriber
🔌 Connecting to: tcp://localhost:5555
🎯 Topic filter: robot_joints
✅ Connected! Waiting for messages...

📥 Received robot state:
   Robot ID: robot_arm_001
   Timestamp: 1
   Joints (6):
     - shoulder_pan: 0.000 rad (0.0°), vel: 0.100 rad/s, torque: 0.00 N⋅m
     - shoulder_lift: 0.479 rad (27.4°), vel: 0.088 rad/s, torque: 4.76 N⋅m
     - elbow: 0.841 rad (48.2°), vel: 0.054 rad/s, torque: -4.95 N⋅m
     ...
```

## Architecture

- **Tokio**: Provides async runtime for concurrent operations
- **ZMQ PUB/SUB**: Publisher sends messages to all subscribers
- **JSON**: Human-readable and easy to integrate with other systems
- **Topic Filtering**: Subscribers can filter by topic prefix

## Use Cases

- Real-time robot state monitoring
- Multi-client robot data distribution
- Testing and simulation of robot communication
- Learning async Rust and ZMQ patterns
- Integration with robot control systems

## Dependencies

- `tokio`: Async runtime
- `zmq`: ZeroMQ bindings
- `serde` / `serde_json`: JSON serialization
- `clap`: Command-line argument parsing
- `anyhow`: Error handling
- `mujoco-rs`: MuJoCo physics simulation library (with auto-download feature)
- `zlib-rs`: Compression library (required by mujoco-rs)

## Credits

This project uses **[mujoco-rs](https://github.com/davidhozic/mujoco-rs)** - an open-source Rust wrapper for the MuJoCo physics engine.

- **Repository**: [davidhozic/mujoco-rs](https://github.com/davidhozic/mujoco-rs)
- **Documentation**: [mujoco-rs.readthedocs.io](https://mujoco-rs.readthedocs.io/)
- **MuJoCo Version**: 3.3.7

MuJoCo-rs provides safe Rust bindings and high-level wrappers around MuJoCo's C API, including features like:
- Safe wrappers with automatic allocation and cleanup
- Lifetime guarantees
- Rust-native viewer
- Offscreen rendering capabilities
- Automatic MuJoCo library download (via `auto-download-mujoco` feature)

We thank the mujoco-rs maintainers and contributors for their excellent work!

