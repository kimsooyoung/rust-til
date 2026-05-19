---
name: rust-project
description: Conventions for **real Rust projects** living in this repo (anything that is not a numbered learning chapter — application crates, library crates, robotics/EtherCAT/teleop tooling, multi-crate workspaces, etc.). Use this skill whenever the working path is outside the `N_<topic>/` learning folders, or when the user asks to start/extend a non-chapter Rust project. Favors idiomatic Cargo project structure over the pedagogical many-bins layout used by chapters.
---

# Rust project conventions

These conventions apply to **production-style** Rust code in this repo: anything meant to ship, run on hardware, or be reused as a dependency — not the teaching slices in numbered chapter folders.

## When to use this skill vs. `rust-example`

- Working inside `N_<topic>/` (e.g. `14_asynchronous/`) → use **`rust-example`**.
- Working anywhere else (new top-level folder, a `projects/` subtree, a robotics/teleop/EtherCAT crate, a Cargo workspace member that isn't a chapter) → use **this skill**.

If in doubt, ask the user which kind of folder they're starting before scaffolding.

## ⚠️ MANDATORY: Two-agent pipeline for every Rust edit

**No Rust code in project folders is ever written or reviewed by the main Claude session directly.** Every non-trivial Rust edit MUST go through this pipeline — the same one that applies to chapter folders:

1. **Workflow orchestrator** — the main Claude session. Plans the task, decides what to delegate, sequences generation and evaluation, relays evaluator findings back to the generator for revisions, and reports the final result to the user. **Does NOT write Rust code. Does NOT review Rust code.**
2. **`rust-code-generator`** (project subagent at `.claude/agents/rust-code-generator.md`) — the **only** agent permitted to write or edit Rust source files in project folders: `src/*.rs`, `Cargo.toml`, workspace manifests, build scripts, integration tests, etc. Must follow the project conventions described below. Does **NOT** self-grade its own output.
3. **`rust-code-evaluator`** (project subagent at `.claude/agents/rust-code-evaluator.md`) — **invoked immediately after every `rust-code-generator` run, proactively, even when the user did not ask for review.** Independently checks idiomatic Rust quality, project-appropriate error handling (app vs. library), `tracing` over raw `println!`, public-API doc-comment coverage, `cargo fmt --check` / `cargo clippy --all-targets --all-features -- -D warnings` / `cargo test` results, and the layout/structural choices described in this skill. Runs with no shared state from the generator so the review is independent.

How to apply:

- For any non-trivial change: delegate writing to `rust-code-generator` via the Agent tool → delegate review to a fresh `rust-code-evaluator` → either accept the result or send the evaluator's specific findings back to `rust-code-generator` for another pass. Iterate until the evaluator is satisfied.
- Each agent invocation receives a **self-contained brief** — never assume the next agent saw the previous agent's context. Quote relevant file paths, requirements, and (for the evaluator) what the generator just produced. The brief should also tell the generator that this is a **project folder** (not a chapter), so it does not apply the `exNN_`/`bin/` pedagogical layout.
- **Never collapse two roles into one agent.** Never have a single agent do all three. Never skip the evaluator step.
- **Scope**: applies to new project crates, new modules, refactors, hardware-driver code, and any meaningful Rust edit. **Exempted** (orchestrator may handle directly): typo fixes, single-line non-logic edits, documentation-only changes to `README.md` / `CLAUDE.md` / `SKILL.md`.

If you are the main Claude session and you find yourself about to call `Edit` or `Write` on a `.rs` / `Cargo.toml` / build script inside a project folder — **stop and delegate to `rust-code-generator` instead.**

## Default crate layout

Stick to the layout `cargo new` produces unless the project genuinely needs more. Add structure as it becomes necessary, not preemptively.

**Binary crate (single application):**

```
<project>/
├── Cargo.toml
├── Cargo.lock
├── README.md           # what the project is, how to run it
├── src/
│   ├── main.rs         # thin: arg parsing → call into lib
│   └── lib.rs          # all real logic lives here, so it's testable
└── tests/              # integration tests once they exist
```

**Library crate:** drop `src/main.rs`; keep `src/lib.rs` as the public API surface.

**Multi-crate workspace** (when a single project grows multiple cooperating crates — e.g. a binary + a reusable library + a hardware-abstraction crate):

```
<project>/
├── Cargo.toml          # [workspace] members = ["app", "core", "hal"]
├── app/                # binary
├── core/               # domain library
└── hal/                # hardware abstraction
```

Use a workspace only when at least one crate is genuinely reused or has independent CI/test concerns. Don't pre-split.

## `Cargo.toml` baseline

```toml
[package]
name = "<project>"
version = "0.1.0"
edition = "2024"           # prefer 2024; drop to 2021 only if a dependency forces it
rust-version = "1.80"      # set when you start pinning toolchain expectations
license = "MIT OR Apache-2.0"
description = "<one-line project description>"

[dependencies]

[dev-dependencies]
```

Notes:

- Pin `rust-version` once the project is shared or CI-built — it makes toolchain breakage visible.
- Group dependencies, dev-dependencies, and build-dependencies clearly; don't scatter feature flags.
- Prefer `workspace.dependencies` to deduplicate versions when you do introduce a workspace.

## Module organization

- Start with a flat `src/lib.rs`. Split into submodules only when a single file exceeds ~300 lines or mixes unrelated concerns.
- Public API stays in `lib.rs` (re-exports + top-level types). Internals live in `mod` files: `src/<area>.rs` or `src/<area>/mod.rs` + `src/<area>/<sub>.rs`.
- Keep `main.rs` boring: parse args, build config, call a single `lib::run(...)` entry point. This keeps the binary thin and the logic testable from integration tests.
- Don't expose anything you wouldn't be willing to support — `pub(crate)` and module-private are the defaults; `pub` is a deliberate choice.

## Errors, logging, async

- **Errors**: applications use `anyhow::Result<T>` for top-level glue; libraries define a typed error with `thiserror`. Don't `unwrap()` outside tests, examples, or genuinely-infallible paths — document why if you do.
- **Logging**: use `tracing` (not raw `println!`) for anything beyond throwaway examples. Set up `tracing-subscriber` in `main.rs` once.
- **Async runtime**: prefer `tokio` with explicit feature selection (`features = ["macros", "rt-multi-thread", ...]`). For control-loop / hardware code, document why a given runtime flavor (current-thread vs. multi-thread) was chosen — it matters for real-time behavior.
- **Blocking work inside async**: route through `tokio::task::spawn_blocking` or a dedicated thread; never call sync blocking code directly inside an `async fn` that runs on a shared runtime.

## Quality gates

Every project should run cleanly under:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `cargo build --release` (when the project is meant to deploy)

Wire these into CI as soon as the project leaves "scratch" status. Locally, the repo-level pre-commit hook already runs fmt + clippy + check across all chapter crates; mirror that expectation for project crates.

## Documentation

- **Crate-level**: top of `lib.rs` (or `main.rs` for binaries) gets a `//!` doc-comment explaining what the crate does and how to use it.
- **Public items**: every `pub` item gets a `///` doc-comment with at least a one-line summary and, when useful, a runnable `# Examples` block (these are tested by `cargo test`).
- **README.md**: required for project crates (unlike chapter folders). Include: what it is, how to build/run, key environment variables, and a pointer to the entry point.

## Testing

- Unit tests live next to the code in `#[cfg(test)] mod tests { … }` blocks.
- Integration tests go under `tests/<feature>.rs` and exercise the public API only.
- For hardware-adjacent code (EtherCAT, motor drivers, teleop), keep a clear split between (a) pure logic that runs in CI, and (b) hardware-in-the-loop tests that are explicitly gated behind a feature flag or runner.

## Things to avoid

- Copy-pasting the chapter `exNN_` layout into a project crate — it's pedagogy, not architecture.
- Adding dependencies "just in case" — every dep is a future audit and compile-time cost.
- Hiding logic in `main.rs` where it can't be tested.
- Silent `unwrap()` / `expect()` in production paths.
- Shipping without `fmt` + `clippy -D warnings` clean.
