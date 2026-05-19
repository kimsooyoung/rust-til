# project_inventory_system

A small Cursive-based terminal UI for managing a product inventory. Products
are persisted to `inventory.json` in the current working directory.

## What it does

- Add a product (type, quantity, price-per-unit). Sales tax is computed at 10%.
- List all products currently stored.
- Delete a product by its 1-based ID.
- Quit.

State is read from `inventory.json` on startup and rewritten on every
mutation, so a single file is the source of truth.

## Build & run

This crate ships a per-project `justfile`. From the repo root:

```bash
just project_inventory_system run        # run the TUI
just project_inventory_system build      # cargo build
just project_inventory_system clippy     # cargo clippy -D warnings
just project_inventory_system fmt-check  # rustfmt in check mode
```

Or directly inside the crate:

```bash
cd project_inventory_system
just            # list available recipes
just run        # cursive needs TERM=xterm-256color; the recipe sets it
```

Cursive requires a real terminal with `TERM=xterm-256color`; the `run`
recipe sets this for you.

## Expected output

On launch you see a dialog titled "Inventory Management" with three text
fields (Product Type, Quantity, Price per Unit) and four buttons (Save,
Show All, Delete by ID, Quit). All actions show feedback in modal dialogs.
