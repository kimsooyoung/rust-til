---
name: rust-example
description: Conventions for the Rust **learning-chapter** folders in this repo (`1_variables`, `2_ownership`, …, `14_asynchronous`, …). Use this skill whenever the working path is one of the numbered chapter directories or when adding/editing chapter examples. The goal is that `cd N_<topic> && just` works identically across every chapter.
---

# Rust learning-chapter conventions

These chapters are pedagogical: each chapter directory is a self-contained Cargo crate that ships many small, runnable examples plus a `justfile` for one-command execution. Follow the standardized layout below unless a documented exception applies.

## ⚠️ MANDATORY: Two-agent pipeline for every Rust edit

**No Rust code in chapter folders is ever written or reviewed by the main Claude session directly.** Every non-trivial Rust edit MUST go through this pipeline:

1. **Workflow orchestrator** — the main Claude session. Plans the task, decides what to delegate, sequences generation and evaluation, relays evaluator findings back to the generator for revisions, and reports the final result to the user. **Does NOT write Rust code. Does NOT review Rust code.**
2. **`rust-code-generator`** (project subagent at `.claude/agents/rust-code-generator.md`) — the **only** agent permitted to write or edit Rust source files in chapter folders: chapter modules, bin launchers, `Cargo.toml`, `justfile`, shared `<domain>.rs` files, etc. Must follow the standardized chapter layout described below. Does **NOT** self-grade its own output.
3. **`rust-code-evaluator`** (project subagent at `.claude/agents/rust-code-evaluator.md`) — **invoked immediately after every `rust-code-generator` run, proactively, even when the user did not ask for review.** Independently checks layout compliance (lib/bins split, `exNN_` naming, `default-run`, justfile targets), idiomatic Rust quality, doc-comment presence, and `cargo build` / `cargo clippy` results. Runs with no shared state from the generator so the review is independent.

How to apply:

- For any non-trivial change: delegate writing to `rust-code-generator` via the Agent tool → delegate review to a fresh `rust-code-evaluator` → either accept the result or send the evaluator's specific findings back to `rust-code-generator` for another pass. Iterate until the evaluator is satisfied.
- Each agent invocation receives a **self-contained brief** — never assume the next agent saw the previous agent's context. Quote relevant file paths, requirements, and (for the evaluator) what the generator just produced.
- **Never collapse two roles into one agent.** Never have a single agent do all three. Never skip the evaluator step.
- **Scope**: applies to new chapters, new topic examples, refactors of existing chapters, and any meaningful Rust edit. **Exempted** (orchestrator may handle directly): typo fixes, single-line non-logic edits, documentation-only changes to `README.md` / `CLAUDE.md` / `SKILL.md`.

If you are the main Claude session and you find yourself about to call `Edit` or `Write` on a `.rs` / `Cargo.toml` / `justfile` inside a chapter folder — **stop and delegate to `rust-code-generator` instead.**

## Chapter directory layout

```
N_<topic>/
├── Cargo.toml
├── Cargo.lock
├── justfile
└── src/
    ├── lib.rs                  # `pub mod` declarations only + chapter doc-comment
    ├── exNN_<topic>.rs         # topic module exposing `pub fn run()`
    ├── exNN_<topic>.rs
    ├── …
    ├── <domain>.rs             # shared types/helpers used by the topic modules
    └── bin/
        ├── exNN_<topic>.rs     # 3-line launcher: `fn main() { <crate>::exNN_<topic>::run(); }`
        ├── exNN_<topic>.rs
        └── …
```

Rules:

- `N` is the chapter number (no zero-padding). `NN` inside `exNN_` is **zero-padded to 2 digits** (`ex01`, `ex02`, … `ex10`).
- Topic file name and its bin launcher file name **must match exactly** (`src/ex03_shapes.rs` ↔ `src/bin/ex03_shapes.rs`).
- Each `src/exNN_*.rs` exposes a single `pub fn run()` that contains the entire demo for that topic.
- Each `src/bin/exNN_*.rs` is a thin launcher — three lines, no logic:
  ```rust
  fn main() {
      <crate_name>::exNN_<topic>::run();
  }
  ```
- Shared structs/enums/functions used by multiple topics live in their own module file at `src/<domain>.rs` (e.g. `bank_account.rs`, `person.rs`, `helpers.rs`) and are re-exported via `lib.rs`.

## `Cargo.toml`

```toml
[package]
name = "<crate_name>"
version = "0.1.0"
edition = "2021"          # or "2024" for newer chapters — match neighboring chapters
default-run = "ex01_<topic>"

[lib]
path = "src/lib.rs"

[dependencies]
```

- `default-run` must point to `ex01_…` so `cargo run` with no args runs the first example.
- `[lib]` block is required so topic modules can be reached from `src/bin/*` as `<crate_name>::exNN_…`.

## `src/lib.rs`

```rust
//! Chapter N — <topic>: short one-liner describing the chapter.
//! Runnable slices live in `src/bin/*`; shared modules here.

pub mod ex01_<topic>;
pub mod ex02_<topic>;
// …

pub mod <domain>;     // shared types
```

## Topic module doc-comment

Every `src/exNN_*.rs` starts with:

```rust
//! Topic N — <one-line description of what this example demonstrates>.
//!
//! Run via: `cargo run --bin exNN_<topic>`
```

## `justfile`

```just
# shellcheck shell=bash
# Chapter N — <topic>. Topics in `src/bin/`; shared code in `src/lib.rs` + modules.
# Run: `just run_exNN_<topic>`, … or `cargo run --bin exNN_<topic>`

set shell := ['zsh', '-cu']

default:
    @just --list

run_ex01_<topic>:
    @cargo run --bin ex01_<topic>

run_ex02_<topic>:
    @cargo run --bin ex02_<topic>

# … one target per example …

build-all:
    @cargo build --bins

watch:
    @cargo-watch -qc -x 'run --bin ex01_<topic>' -x clippy

clippy:
    @cargo clippy
```

## When to deviate from the standard layout

The standard layout above (lib + multiple bins) is the default. Only deviate when the chapter's content demands it, matching one of these existing precedents:

- **Single-example chapter** (like `12_generics`): one `src/main.rs`, no `lib.rs`, no `src/bin/`. Justfile has a single `run:` target.
- **Multi-crate chapter** (like `13_pcmp`): root binary in `src/main.rs` plus a sibling library crate (e.g. `robot_core/`) wired via `[dependencies] robot_core = { path = "robot_core" }`.
- **No shared code needed** (like `11_pointers`): skip `lib.rs` and put each self-contained example directly in `src/bin/exNN_*.rs`.

Default to the standard lib + bins layout unless one of the above clearly applies.

## Other conventions

- Output uses a `section(title: &str)` helper that prints `\n=== {title} ===\n` for readability. Either define it in `src/helpers.rs` or reuse the chapter's existing helper.
- Don't introduce new top-level dependencies unless the topic genuinely needs them — most chapters have empty `[dependencies]`.
- Don't add a `README.md` per chapter; the chapter doc-comment in `lib.rs` and the per-topic doc-comments are the documentation.
- Demos should be **observably teaching** — the runtime output (not just comments) must surface the concept being taught. If the output doesn't make the concept visible, redesign the timing/printing until it does.
