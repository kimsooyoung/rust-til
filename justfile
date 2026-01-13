# Default recipe: show available commands
default:
    @just --list

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

# Run clippy on a specific project
# Usage: just clippy <project_name>
# Example: just clippy types
clippy project:
    @cd {{project}} && cargo clippy

# Run a specific binary from lecture_4
# Usage: just lecture4 <binary_name>
# Example: just lecture4 lecture4_2
lecture4 binary:
    @cd lecture_4 && cargo run --bin {{binary}}

# Watch a specific binary from lecture_4
# Usage: just watch-lecture4 <binary_name>
# Example: just watch-lecture4 lecture4_2
watch-lecture4 binary:
    @cd lecture_4 && cargo-watch -qc -x "run --bin {{binary}}" -x clippy

# Quick shortcuts for each project
watch-borrowing-reference:
    @just watch borrowing_reference

watch-enums:
    @just watch enums

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

watch-lecture-3:
    @just watch lecture_3

watch-ownership:
    @just watch ownership

watch-structs:
    @just watch structs

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
