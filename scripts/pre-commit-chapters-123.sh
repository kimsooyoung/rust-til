#!/usr/bin/env bash
# Run rustfmt (check) and clippy only for learning chapters 1–3.
# shellcheck shell=bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

for dir in 1_variables 2_ownership 3_functions; do
  manifest="${dir}/Cargo.toml"
  if [[ ! -f "$manifest" ]]; then
    echo "Missing $manifest" >&2
    exit 1
  fi
  echo ">>> fmt --check: $manifest"
  cargo fmt --manifest-path "$manifest" --check
  echo ">>> clippy: $manifest"
  cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings
done
