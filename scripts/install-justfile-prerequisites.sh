#!/usr/bin/env bash
# Install external tools referenced by repo Justfiles (root + chapter + project_robot).
#
# Usage (from repo root):
#   just install-prerequisites
#   ./scripts/install-justfile-prerequisites.sh
#
# shellcheck shell=bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

info() { printf '%s\n' "$*"; }
warn() { printf '%s\n' "WARN: $*" >&2; }
die() { printf '%s\n' "ERROR: $*" >&2; exit 1; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Missing '$1'. Install Rust from https://rustup.rs/ (includes cargo), then re-run this script."
}

info "==> Checking shell / path tools used by Justfiles"
need_cmd cargo
command -v rustup >/dev/null 2>&1 || warn "rustup not found; skipping 'rustup component add'. Install https://rustup.rs/ for best results."
command -v zsh >/dev/null 2>&1 || warn "zsh not found; root Justfile uses set shell := ['zsh', '-cu']. Install zsh or adjust justfile."
command -v bash >/dev/null 2>&1 || warn "bash not found; project_robot_joint_pubsub recipes invoke bash."
command -v env >/dev/null 2>&1 || die "env(1) missing (unusual on macOS/Linux)."

if command -v realpath >/dev/null 2>&1; then
  info "    realpath: ok ($(command -v realpath))"
else
  warn "realpath not in PATH. Recipes like run-robot-publisher use realpath. On macOS, upgrade the OS or install GNU coreutils (brew install coreutils) and use grealpath / adjust PATH."
fi

info "==> Rust toolchain components (for cargo clippy in watch recipes)"
if command -v rustup >/dev/null 2>&1; then
  rustup component add clippy
else
  warn "Skipping 'rustup component add clippy' (no rustup)."
fi

info "==> Cargo-installed CLI tools used by Justfiles"
# cargo-watch: watch / watch-* / chapter justfiles watch
# just: default recipe and nested just -f dispatch (often also installed via brew)
for crate in cargo-watch just; do
  if command -v "$crate" >/dev/null 2>&1; then
    info "    $crate: already on PATH ($(command -v "$crate"))"
  else
    info "    Installing $crate via cargo install ..."
    cargo install "$crate" --locked
  fi
done

info "==> Done."
info "    You should be able to run: just watch 1_variables"
info "    (Ensure ~/.cargo/bin is on your PATH if commands are not found.)"
