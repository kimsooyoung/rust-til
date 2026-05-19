//! Binary entry point for the inventory TUI.
//!
//! Delegates to [`project_inventory_system::run`]; the binary is intentionally
//! kept small so the bulk of the logic stays testable from `lib.rs`.

fn main() -> anyhow::Result<()> {
    project_inventory_system::run()
}
