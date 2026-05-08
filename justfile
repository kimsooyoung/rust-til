# shellcheck shell=bash

# Explicit shell for recipes (helps editor tooling and linting).
# Your environment is zsh, so we run commands through zsh.
set shell := ['zsh', '-cu']

# Default recipe: show available commands
default:
    @just --list

# Install CLI tools used by this repo's Justfiles (cargo-watch, just; rustup clippy; path checks).
# Usage: just install-prerequisites
install-prerequisites:
    @bash "{{justfile_directory()}}/scripts/install-justfile-prerequisites.sh"

# Run pre-commit on the whole repo (Rust fmt/clippy only for `1_variables`, `2_ownership`, `3_functions`).
# Install once: `pip install pre-commit` && `pre-commit install`
# Usage: just pre-commit
pre-commit:
    @pre-commit run --all-files

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
    @just watch 5_enums

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

# Chapter 1 — variables tutorial (`1_variables/`)
# Usage: `just run-chapter-1` | `just watch-chapter-1` | `just clippy-chapter-1`
# From inside the crate: `cd 1_variables && just run`
run-chapter-1:
    @cd 1_variables && cargo run

watch-chapter-1:
    @just watch 1_variables

clippy-chapter-1:
    @cd 1_variables && cargo clippy

# Chapter 2 — ownership tutorial (`2_ownership/`)
# Usage: `just run-chapter-2` | `just watch-chapter-2` | `just clippy-chapter-2`
# From inside the crate: `cd 2_ownership && just run`
run-chapter-2:
    @cd 2_ownership && cargo run

watch-chapter-2:
    @just watch 2_ownership

clippy-chapter-2:
    @cd 2_ownership && cargo clippy

# Chapter 3 — functions tutorial (`3_functions/`)
# Usage: `just run-chapter-3` | `just watch-chapter-3` | `just clippy-chapter-3`
# From inside the crate: `cd 3_functions && just run`
run-chapter-3:
    @cd 3_functions && cargo run

watch-chapter-3:
    @just watch 3_functions

clippy-chapter-3:
    @cd 3_functions && cargo clippy

# Chapter 4 — structs tutorial (`4_structures/`)
# Usage: `just run-chapter-4` | `just watch-chapter-4` | `just clippy-chapter-4`
# From inside the crate: `cd 4_structures && just run`
run-chapter-4:
    @cd 4_structures && cargo run

watch-chapter-4:
    @just watch 4_structures

clippy-chapter-4:
    @cd 4_structures && cargo clippy

# Chapter 5 — enums tutorial (`5_enums/`)
# Usage: `just run-chapter-5` | `just watch-chapter-5` | `just clippy-chapter-5`
# From inside the crate: `cd 5_enums && just run`
run-chapter-5:
    @cd 5_enums && cargo run

watch-chapter-5:
    @just watch 5_enums

clippy-chapter-5:
    @cd 5_enums && cargo clippy

watch-lecture-3:
    @just watch lecture_3

watch-ownership:
    @just watch ownership

watch-structs:
    @just watch 4_structures

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
