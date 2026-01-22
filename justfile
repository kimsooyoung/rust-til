# shellcheck shell=bash

# Explicit shell for recipes (helps editor tooling and linting).
# Your environment is zsh, so we run commands through zsh.
set shell := ['zsh', '-cu']

# Default recipe: show available commands
default:
    @just --list

# Dispatch into `project_robot_joint_pubsub/justfile`.
# Usage:
#   just project_robot_joint_pubsub <recipe>
# Examples:
#   just project_robot_joint_pubsub mujoco-build
#   just project_robot_joint_pubsub run-robot-subscriber
project_robot_joint_pubsub +args:
    @just -f "{{justfile_directory()}}/project_robot_joint_pubsub/justfile" {{args}}

# Run a specific project with cargo-watch (runs and checks with clippy)
# Usage: just watch <project_name>
# Example: just watch types
watch project:
    @cd {{project}} && cargo-watch -qc -x run -x clippy

# Run a specific project once (without watch mode)
# Usage: just run <project_name>
# Example: just run types
run project:
    @cd {{project}} && cargo run

# Run tui_tut with TERM set (required for terminal UI)
run-tui-tut:
    @cd tui_tut && TERM=xterm-256color cargo run

# Run inventory_system with TERM set (required for terminal UI)
run-inventory-system:
    @cd inventory_system && TERM=xterm-256color cargo run

# Run clippy on a specific project
# Usage: just clippy <project_name>
# Example: just clippy types
clippy project:
    @cd {{ project }} && cargo clippy

# Run a specific binary from lecture_4
# Usage: just lecture4 <binary_name>
# Example: just lecture4 lecture4_2
lecture4 binary:
    @cd lecture_4 && cargo run --bin "{{ binary }}"

# Watch a specific binary from lecture_4
# Usage: just watch-lecture4 <binary_name>
# Example: just watch-lecture4 lecture4_2
watch-lecture4 binary:
    @cd lecture_4 && cargo-watch -qc -x "run --bin {{ binary }}" -x clippy

# Quick shortcuts for each project
watch-borrowing-reference:
    @just watch borrowing_reference

watch-enums:
    @just watch enums

watch-error-handling:
    @just watch error_handling

watch-functions:
    @just watch functions

watch-generics:
    @just watch generics

watch-guessing-game:
    @just watch guessing_game

watch-hello-world:
    @just watch hello_world

watch-lecture-3:
    @just watch lecture_3

watch-ownership:
    @just watch ownership

watch-structs:
    @just watch structs

watch-types:
    @just watch types

watch-collections:
    @just watch collections

watch-optionals:
    @just watch optionals

watch-asynchronous:
    @just watch asynchronous

watch-pcmp:
    @just watch pcmp

watch-lifetimes:
    @just watch lifetimes

watch-traits:
    @just watch traits

watch-pointers:
    @just watch pointers

# Watch project_manufacturers with an argument
# Usage: just watch-project-manufacturers <manufacturer>
# Example: just watch-project-manufacturers BMW
watch-project-manufacturers manufacturer:
    @cd project_manufacturers && cargo-watch -qc -s "cargo run -- {{manufacturer}}" -x clippy

# Watch project_notes with a note title argument
# Usage: just watch-project-notes <note_title>
# Example: just watch-project-notes "My Notes"
# Note: Use quotes if the note title contains spaces
watch-project-notes note_title:
    @cd project_notes && cargo-watch -qc -x 'run -- {{note_title}}' -i "{{note_title}}" -i "notes.txt" -x clippy

# Run project_robot_joint_pubsub publisher with MUJOCO_DOWNLOAD_DIR set
run-robot-publisher:
    @cd project_robot_joint_pubsub && MUJOCO_DOWNLOAD_DIR="$(realpath mujoco_libs)" cargo run --bin publisher

# Run project_robot_joint_pubsub subscriber with MUJOCO_STATIC_LINK_DIR set (for C++ viewer)
# Note: MUJOCO_STATIC_LINK_DIR must be set to the mujoco build directory
# Run ./build_mujoco_cpp.sh first to build the modified MuJoCo library
run-robot-subscriber:
    @cd project_robot_joint_pubsub && if [ -z "${MUJOCO_STATIC_LINK_DIR:-}" ]; then echo "❌ Error: MUJOCO_STATIC_LINK_DIR not set. Run ./build_mujoco_cpp.sh first."; echo "   Then set: export MUJOCO_STATIC_LINK_DIR=\"/path/to/mujoco-rs/mujoco/build\""; exit 1; fi; env MUJOCO_STATIC_LINK_DIR="${MUJOCO_STATIC_LINK_DIR:-}" cargo run --bin subscriber

# Watch project_robot_joint_pubsub publisher (no MuJoCo dependency)
watch-robot-publisher:
    @cd project_robot_joint_pubsub && cargo-watch -qc -x "run --bin publisher" -x clippy

# Watch project_robot_joint_pubsub subscriber with MUJOCO_STATIC_LINK_DIR set (for C++ viewer)
watch-robot-subscriber:
    @cd project_robot_joint_pubsub && if [ -z "${MUJOCO_STATIC_LINK_DIR:-}" ]; then echo "❌ Error: MUJOCO_STATIC_LINK_DIR not set. Run ./build_mujoco_cpp.sh first."; echo "   Then set: export MUJOCO_STATIC_LINK_DIR=\"/path/to/mujoco-rs/mujoco/build\""; exit 1; fi; env MUJOCO_STATIC_LINK_DIR="${MUJOCO_STATIC_LINK_DIR:-}" cargo-watch -qc -x "run --bin subscriber" -x clippy
