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
# Example: just watch 1_variables
watch project:
    @cd {{project}} && cargo-watch -qc -x run -x clippy

# Run a specific project once (without watch mode)
# Usage: just run <project_name>
# Example: just run 1_variables
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
# Example: just clippy 1_variables
clippy project:
    @cd {{ project }} && cargo clippy

# Quick shortcuts for each project
watch-enums:
    @just watch 5_enums

watch-error-handling:
    @just watch 8_error_handling

watch-functions:
    @just watch 3_functions

watch-generics:
    @just watch 12_generics

# Chapter 1 — variables tutorial (`1_variables/`)
# Usage: `just run-chapter-1` | `just watch-chapter-1` | `just clippy-chapter-1`
# From inside the crate: `cd 1_variables && just run_ex01_bindings` (or `cargo run`)
run-chapter-1:
    @cd 1_variables && cargo run

watch-chapter-1:
    @just watch 1_variables

clippy-chapter-1:
    @cd 1_variables && cargo clippy

# Chapter 2 — ownership tutorial (`2_ownership/`)
# Usage: `just run-chapter-2` | `just watch-chapter-2` | `just clippy-chapter-2`
# From inside the crate: `cd 2_ownership && just run_ex01_strings` (or `cargo run`)
run-chapter-2:
    @cd 2_ownership && cargo run

watch-chapter-2:
    @just watch 2_ownership

clippy-chapter-2:
    @cd 2_ownership && cargo clippy

# Chapter 3 — functions tutorial (`3_functions/`)
# Usage: `just run-chapter-3` | `just watch-chapter-3` | `just clippy-chapter-3`
# From inside the crate: `cd 3_functions && just run_ex01_tuple_block` (or `cargo run`)
run-chapter-3:
    @cd 3_functions && cargo run

watch-chapter-3:
    @just watch 3_functions

clippy-chapter-3:
    @cd 3_functions && cargo clippy

# Chapter 4 — structs tutorial (`4_structures/`)
# Usage: `just run-chapter-4` | `just watch-chapter-4` | `just clippy-chapter-4`
# From inside the crate: `cd 4_structures && just run_ex01_bank_account` (or `cargo run`)
run-chapter-4:
    @cd 4_structures && cargo run

watch-chapter-4:
    @just watch 4_structures

clippy-chapter-4:
    @cd 4_structures && cargo clippy

# Chapter 5 — enums tutorial (`5_enums/`)
# Usage: `just run-chapter-5` | `just watch-chapter-5` | `just clippy-chapter-5`
# From inside the crate: `cd 5_enums && just run_ex01_ip_struct` (or `cargo run`)
run-chapter-5:
    @cd 5_enums && cargo run

watch-chapter-5:
    @just watch 5_enums

clippy-chapter-5:
    @cd 5_enums && cargo clippy

# Chapter 6 — collections tutorial (`6_collections/`)
run-chapter-6:
    @cd 6_collections && cargo run

watch-chapter-6:
    @just watch 6_collections

clippy-chapter-6:
    @cd 6_collections && cargo clippy

# Chapter 7 — optionals tutorial (`7_optionals/`)
run-chapter-7:
    @cd 7_optionals && cargo run

watch-chapter-7:
    @just watch 7_optionals

clippy-chapter-7:
    @cd 7_optionals && cargo clippy

# Chapter 8 — error handling tutorial (`8_error_handling/`)
run-chapter-8:
    @cd 8_error_handling && cargo run

watch-chapter-8:
    @just watch 8_error_handling

clippy-chapter-8:
    @cd 8_error_handling && cargo clippy

# Chapter 9 — lifetimes tutorial (`9_lifetimes/`)
run-chapter-9:
    @cd 9_lifetimes && cargo run

watch-chapter-9:
    @just watch 9_lifetimes

clippy-chapter-9:
    @cd 9_lifetimes && cargo clippy

# Chapter 10 — traits tutorial (`10_traits/`)
run-chapter-10:
    @cd 10_traits && cargo run

watch-chapter-10:
    @just watch 10_traits

clippy-chapter-10:
    @cd 10_traits && cargo clippy

# Chapter 11 — pointers tutorial (`11_pointers/`)
run-chapter-11:
    @cd 11_pointers && cargo run

watch-chapter-11:
    @just watch 11_pointers

clippy-chapter-11:
    @cd 11_pointers && cargo clippy

# Chapter 12 — generics tutorial (`12_generics/`)
run-chapter-12:
    @cd 12_generics && cargo run

watch-chapter-12:
    @just watch 12_generics

clippy-chapter-12:
    @cd 12_generics && cargo clippy

# Chapter 13 — pcmp tutorial (`13_pcmp/`)
run-chapter-13:
    @cd 13_pcmp && cargo run

watch-chapter-13:
    @just watch 13_pcmp

clippy-chapter-13:
    @cd 13_pcmp && cargo clippy

# Chapter 14 — asynchronous tutorial (`14_asynchronous/`)
run-chapter-14:
    @cd 14_asynchronous && cargo run

watch-chapter-14:
    @just watch 14_asynchronous

clippy-chapter-14:
    @cd 14_asynchronous && cargo clippy

watch-structs:
    @just watch 4_structures

watch-collections:
    @just watch 6_collections

watch-optionals:
    @just watch 7_optionals

watch-asynchronous:
    @just watch 14_asynchronous

watch-pcmp:
    @just watch 13_pcmp

watch-lifetimes:
    @just watch 9_lifetimes

watch-traits:
    @just watch 10_traits

watch-pointers:
    @just watch 11_pointers

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
