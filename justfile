# shellcheck shell=bash
#
# Repo-level Justfile. Two dispatch families plus a handful of repo-wide utilities.
#
#   just lecture_N [recipe ...]      Numbered learning chapter (N = 01..14).
#   just <project>  [recipe ...]     Project crate (project_inventory_system, project_manufacturers,
#                                    project_notes, project_robot_joint_pubsub).
#
# Calling a dispatcher with no recipe lists the available recipes inside that crate.
# Run `just --list` for the full grouped index.

set shell := ['zsh', '-cu']

# ─── Repo-wide ──────────────────────────────────────────────────────────────────

# Short help. Run `just --list` for the full grouped index.
[group('repo')]
default:
    @echo 'Usage:'
    @echo '  just lecture_N  [recipe ...]   Run/list recipes in chapter N (N = 01..14).'
    @echo '  just <project>  [recipe ...]   Run/list recipes in a project crate.'
    @echo '  just pre-commit                Run fmt/clippy hooks across the repo.'
    @echo ''
    @echo 'Examples:'
    @echo '  just lecture_01                      # list chapter 1 recipes'
    @echo '  just lecture_01 run_ex01_bindings    # run a single example'
    @echo '  just project_notes                   # list project_notes recipes'
    @echo '  just project_notes run my.txt        # run with an argument'
    @echo ''
    @echo 'Full grouped menu: just --list'

# Install CLI tools used by this repo's justfiles (cargo-watch, just, clippy).
[group('repo')]
install-prerequisites:
    @bash "{{justfile_directory()}}/scripts/install-justfile-prerequisites.sh"

# Run pre-commit (rustfmt + clippy + checks) across the repo. Requires `pre-commit install` once.
[group('repo')]
pre-commit:
    @pre-commit run --all-files

# Legacy demo: `tui_tut/` has no per-project justfile of its own.
[group('repo')]
run-tui-tut:
    @cd "{{justfile_directory()}}/tui_tut" && TERM=xterm-256color cargo run

# ─── Learning chapters ──────────────────────────────────────────────────────────
# Dispatch `just lecture_N [recipe ...]` into the chapter's own justfile.

[group('lectures')]
lecture_01 *args:
    @just -f "{{justfile_directory()}}/1_variables/justfile" {{args}}

[group('lectures')]
lecture_02 *args:
    @just -f "{{justfile_directory()}}/2_ownership/justfile" {{args}}

[group('lectures')]
lecture_03 *args:
    @just -f "{{justfile_directory()}}/3_functions/justfile" {{args}}

[group('lectures')]
lecture_04 *args:
    @just -f "{{justfile_directory()}}/4_structures/justfile" {{args}}

[group('lectures')]
lecture_05 *args:
    @just -f "{{justfile_directory()}}/5_enums/justfile" {{args}}

[group('lectures')]
lecture_06 *args:
    @just -f "{{justfile_directory()}}/6_collections/justfile" {{args}}

[group('lectures')]
lecture_07 *args:
    @just -f "{{justfile_directory()}}/7_optionals/justfile" {{args}}

[group('lectures')]
lecture_08 *args:
    @just -f "{{justfile_directory()}}/8_error_handling/justfile" {{args}}

[group('lectures')]
lecture_09 *args:
    @just -f "{{justfile_directory()}}/9_lifetimes/justfile" {{args}}

[group('lectures')]
lecture_10 *args:
    @just -f "{{justfile_directory()}}/10_traits/justfile" {{args}}

[group('lectures')]
lecture_11 *args:
    @just -f "{{justfile_directory()}}/11_pointers/justfile" {{args}}

[group('lectures')]
lecture_12 *args:
    @just -f "{{justfile_directory()}}/12_generics/justfile" {{args}}

[group('lectures')]
lecture_13 *args:
    @just -f "{{justfile_directory()}}/13_pcmp/justfile" {{args}}

[group('lectures')]
lecture_14 *args:
    @just -f "{{justfile_directory()}}/14_asynchronous/justfile" {{args}}

# ─── Project crates ─────────────────────────────────────────────────────────────
# Dispatch `just <project> [recipe ...]` into the project's own justfile.

[group('projects')]
project_inventory_system *args:
    @just -f "{{justfile_directory()}}/project_inventory_system/justfile" {{args}}

[group('projects')]
project_manufacturers *args:
    @just -f "{{justfile_directory()}}/project_manufacturers/justfile" {{args}}

[group('projects')]
project_notes *args:
    @just -f "{{justfile_directory()}}/project_notes/justfile" {{args}}

[group('projects')]
project_robot_joint_pubsub *args:
    @just -f "{{justfile_directory()}}/project_robot_joint_pubsub/justfile" {{args}}
