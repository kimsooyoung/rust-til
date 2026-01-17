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

## Usage

### Build the project

```bash
cargo build --release
```

### Run the Publisher

In one terminal, start the publisher:

```bash
cargo run --release -- publisher
```

Or with custom options:

```bash
# Custom bind address and interval
cargo run --release -- publisher --bind tcp://*:5556 --interval 50
```

### Run the Subscriber

In another terminal, start the subscriber:

```bash
cargo run --release -- subscriber
```

Or with custom options:

```bash
# Custom connect address and topic
cargo run --release -- subscriber --connect tcp://localhost:5556 --topic robot_joints
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

