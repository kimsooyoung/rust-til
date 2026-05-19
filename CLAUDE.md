# Repository orientation

This repo holds two **different** kinds of Rust code, each with its own conventions. Pick the right skill before scaffolding, editing, or reviewing anything.

## The two folder kinds

1. **Rust example folders** — numbered learning chapters at the repo root: `1_variables/`, `2_ownership/`, …, `14_asynchronous/`, and any future `N_<topic>/`. These are pedagogical: each chapter is a Cargo crate that ships many small runnable examples under `src/bin/*` plus a `justfile`. The aim is `cd N_<topic> && just` working identically everywhere.
2. **Rust project folders** — anything **not** a numbered chapter. Production-style application/library crates, robotics/EtherCAT/teleop tooling, multi-crate workspaces, etc. These follow idiomatic Cargo conventions, not the chapter many-bins layout.

## Which skill to use

Before doing non-trivial work, load the matching skill and follow it as authoritative for that folder kind:

- Working inside a numbered chapter directory (`N_<topic>/…`) → use **`rust-example`** (`.claude/skills/rust-example/SKILL.md`).
- Working anywhere else in this repo (new top-level project, project subtree, workspace member that isn't a chapter) → use **`rust-project`** (`.claude/skills/rust-project/SKILL.md`).

If a request is ambiguous (e.g. "start a new Rust thing" with no path), **ask which kind** before scaffolding — the two layouts diverge from the very first file.

## What stays in CLAUDE.md (i.e. global to the repo)

These rules apply **regardless of which skill is active** — both `rust-example` and `rust-project` enforce the same pipeline:

- **Two-agent pipeline is mandatory for every non-trivial Rust edit, in both folder kinds.** The main Claude session is the workflow orchestrator and does **NOT** write Rust or grade Rust itself.
  - Writing → `rust-code-generator` subagent (`.claude/agents/rust-code-generator.md`).
  - Review → a fresh `rust-code-evaluator` subagent (`.claude/agents/rust-code-evaluator.md`), invoked **proactively immediately after** every generator run — not only when the user asks for review.
  - Orchestrator relays evaluator findings back to the generator until the evaluator is satisfied.
- **Never let the orchestrator call `Edit`/`Write` directly** on `.rs` / `Cargo.toml` / `justfile` / build scripts in either folder kind. Delegate instead.
- **Each agent invocation is self-contained.** Quote relevant file paths, requirements, the prior agent's output, and **which folder kind** (chapter vs. project) is being worked on — never assume shared context.
- **Exempted from the dance** (orchestrator may handle directly): typo fixes, single-line non-logic edits, and `README.md` / `CLAUDE.md` / `SKILL.md` documentation-only changes.
- **Don't add new top-level dependencies** unless the work genuinely needs them.
- **Don't introduce `README.md` files for chapter folders.** Project folders, on the other hand, should have one — see `rust-project` skill.

## Quick decision flow

```
Is the path / target folder a numbered chapter (N_<topic>/)?
  ├── yes → load rust-example skill, follow its layout
  └── no  → load rust-project skill, follow its layout
            └── (ambiguous? ask the user before scaffolding)
```
