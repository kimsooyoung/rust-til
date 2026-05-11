#!/usr/bin/env bash
# Run rustfmt (check), clippy, and cargo check across every chapter crate.
# shellcheck shell=bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

CHAPTERS=(
  1_variables
  2_ownership
  3_functions
  4_structures
  5_enums
  6_collections
  7_optionals
  8_error_handling
  9_lifetimes
  10_traits
  11_pointers
  12_generics
  13_pcmp
  14_asynchronous
)

fail=0
for dir in "${CHAPTERS[@]}"; do
  manifest="${dir}/Cargo.toml"
  if [[ ! -f "$manifest" ]]; then
    echo "!!! Missing $manifest — skipping" >&2
    continue
  fi

  echo ">>> fmt --check: $manifest"
  cargo fmt --manifest-path "$manifest" --check || fail=1

  echo ">>> clippy: $manifest"
  cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings || fail=1

  echo ">>> check: $manifest"
  cargo check --manifest-path "$manifest" --all-targets || fail=1
done

exit "$fail"
