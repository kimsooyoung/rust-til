# 🦀 Rust TIL - Today I Learned

> A collection of Rust learning projects and examples by Sooyoung Kim

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## 🎯 Overview

This repository contains hands-on Rust projects covering fundamental concepts, from basic syntax to advanced topics like ownership, borrowing, and error handling. Each project is a self-contained Cargo workspace designed for learning and experimentation.

---

## 🚀 Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Install Development Tools

#### 1. Install Clippy (Rust Linter)

Clippy is Rust's official linter that helps catch common mistakes and improve your code quality.

```bash
rustup component add clippy
```

**Usage:**
```bash
cargo clippy
```

#### 2. Install cargo-watch (Auto-rebuild Tool)

`cargo-watch` automatically rebuilds your project when files change, making development faster.

```bash
cargo install cargo-watch
```

#### 3. Install just (Command Runner)

`just` is a command runner (like `make`) that simplifies running common tasks. Instead of typing long commands like `cargo-watch -qc -x run -x clippy`, you can use simple commands like `just watch types`.

**Why use just?**
- ✅ **Shorter commands** - Replace long cargo-watch commands with simple shortcuts
- ✅ **Consistent workflow** - Same commands work across all projects
- ✅ **Easy to remember** - Intuitive command names
- ✅ **Self-documenting** - Run `just` to see all available commands

**Installation:**

```bash
# On macOS
brew install just

# On Linux (using cargo)
cargo install just

# On Windows (using cargo)
cargo install just

# Or download from https://github.com/casey/just
```

**Verify installation:**
```bash
just --version
```

**What is a justfile?**
The `justfile` in this repository contains all the command recipes. It's similar to a Makefile but simpler and more readable. You can view all available commands by running `just` or `just --list`.

---

## 💻 Usage

### Using justfile Commands (Recommended)

The easiest way to run projects is using the `just` commands. The `justfile` in this repository provides shortcuts for common tasks.

**Basic Usage:**

```bash
# Run a specific project with watch mode (runs code and clippy on changes)
just watch <project_name>

# Examples:
just watch types
just watch ownership
just watch functions
```

**What happens when you run `just watch types`?**
- Changes directory to the `types` project
- Runs `cargo-watch -qc -x run -x clippy`
- Automatically rebuilds and runs when files change
- Checks code with clippy on each change

#### Quick Shortcuts

You can use project-specific shortcuts for faster access (no need to type the project name):

```bash
just watch-types
just watch-ownership
just watch-functions
just watch-generics
just watch-borrowing-reference
just watch-enums
just watch-error-handling
just watch-guessing-game
just watch-hello-world
just watch-lecture-3
just watch-structs
just watch-collections
just watch-optionals
just watch-asynchronous
just watch-pcmp
just watch-lifetimes
just watch-traits
just watch-pointers
just watch-project-manufacturers
```

**Tip:** Run `just` or `just --list` to see all available commands with descriptions.

#### Other Useful Commands

```bash
# Run once without watch mode
just run <project_name>

# Run specific projects with TERM set (for TUI applications)
just run-tui-tut
just run-inventory-system

# Run clippy only
just clippy <project_name>

# List all available commands
just
# or
just --list
```

#### Lecture 4 Binaries

Lecture 4 has multiple binaries. Run them with:

```bash
# Run a specific binary
just lecture4 lecture4_2
just lecture4 lecture4_3

# Watch a specific binary
just watch-lecture4 lecture4_2
```

#### Project with Arguments

Some projects accept command-line arguments. For example, `project_manufacturers` accepts a manufacturer name:

```bash
# Watch project_manufacturers with an argument
just watch-project-manufacturers BMW
just watch-project-manufacturers Toyota
```

### Running Projects Manually

If you prefer to run projects manually, navigate to any project directory and run:

```bash
cd types
cargo run
```

Or use cargo-watch directly:

```bash
cd types
cargo-watch -qc -x run -x clippy
```

### Creating a New Project

To create a new Rust project without initializing a Git repository:

```bash
cargo new my_project --vcs none
```

### Building All Projects

```bash
# Build all projects
for dir in */; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Building $dir..."
        (cd "$dir" && cargo build)
    fi
done
```

---

## ⚙️ VSCode Settings

This workspace includes `.vscode/settings.json` with Rust formatting configuration. The settings automatically format Rust files on save using rust-analyzer.

### Features

- ✅ **Auto-format on save** - Rust files are automatically formatted when you save
- ✅ **rust-analyzer integration** - Uses the official Rust language server

### If it's not working

**Check rust-analyzer is installed:**
- Open Extensions (`Ctrl+Shift+X`)
- Search for "rust-analyzer"
- Install if missing

**Reload VSCode:**
- Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
- Type "Reload Window" and select it

**Check the settings are loaded:**
- Press `Ctrl+Shift+P`
- Type "Preferences: Open Workspace Settings (JSON)"
- You should see your settings there

---

## 📝 Notes

- Each project is independent and can be run separately
- All projects use the latest stable Rust features
- Code includes comments explaining key concepts
- Projects are designed for learning, not production use

---

**Happy Rusting! 🦀**
