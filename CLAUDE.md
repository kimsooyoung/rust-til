# Repository conventions for new chapters

This repo is a sequence of Rust learning chapters (`1_variables`, `2_ownership`, …). When adding a new chapter, follow the standardized layout below. The goal is that `cd N_<topic> && just` works identically across every chapter.

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

## Agent separation (always required)

This project ships two purpose-built subagents in `.claude/agents/`. Every non-trivial change in this repo must be routed through both of them, with the main Claude session acting as the workflow orchestrator. Never collapse two roles into one agent, and never have a single agent do all three.

1. **Workflow orchestrator** — the main Claude session. Plans the task, decides what to delegate, sequences the generation and evaluation phases, relays evaluator findings back to the generator for revisions, and reports the final result to the user. Does **not** write Rust code itself and does **not** perform code evaluation itself.
2. **`rust-code-generator`** (project subagent) — the only agent that writes or edits Rust source files in this repo (chapter modules, bin launchers, `Cargo.toml`, `justfile`, shared `<domain>.rs` files, etc.). Must follow the standardized chapter layout described above. Does **not** self-grade its own output.
3. **`rust-code-evaluator`** (project subagent) — invoked immediately after `rust-code-generator` produces or modifies code. Independently reviews layout compliance (lib/bins split, `exNN_` naming, `default-run`, justfile targets), idiomatic Rust quality, doc-comment presence, and `cargo build` / `cargo clippy` results. Runs with no shared state from the generator so the review is independent.

How to apply:

- For any non-trivial change, the workflow orchestrator delegates writing to `rust-code-generator` via the Agent tool, then delegates review to a fresh `rust-code-evaluator` invocation, then either accepts the result or sends the evaluator's specific findings back to `rust-code-generator` for another pass.
- Each agent invocation receives a self-contained brief — never assume the next agent saw the previous agent's context. Quote the relevant file paths, requirements, and (for the evaluator) what the generator just produced.
- Invoke `rust-code-evaluator` **proactively** after every `rust-code-generator` run, not only when the user asks for review.
- Scope: applies to new chapters, new topic examples, refactors of existing chapters, and any meaningful Rust edit. Exempted: typo fixes, single-line non-logic edits, documentation-only changes to `README.md` / `CLAUDE.md` — the orchestrator may handle these directly.

## Other conventions

- Output uses a `section(title: &str)` helper that prints `\n=== {title} ===\n` for readability. Either define it in `src/helpers.rs` or reuse the chapter's existing helper.
- Don't introduce new top-level dependencies unless the topic genuinely needs them — most chapters have empty `[dependencies]`.
- Don't add a `README.md` per chapter; the chapter doc-comment in `lib.rs` and the per-topic doc-comments are the documentation.
