# project_notes

A tiny CLI that appends a wrapped local-time marker to a text file. Handy as
a "now" stamp inside a long-running notes file.

## What it does

Given one positional argument, the target filename, it appends a single
record of the form:

```
<--YYYY-MM-DD HH:MM:SS-->

```

If the file does not exist, it is created. Existing contents are preserved
(append mode).

## Build & run

This crate ships a per-project `justfile`. From the repo root:

```bash
just project_notes run notes.txt    # append a timestamp marker to ./notes.txt
just project_notes build            # cargo build
just project_notes clippy           # cargo clippy -D warnings
just project_notes fmt-check        # rustfmt in check mode
```

Or directly inside the crate:

```bash
cd project_notes
just                  # list available recipes
just run notes.txt
just watch notes.txt  # cargo-watch wrapper around `run`
```

Filenames are taken verbatim from argv, so they are resolved relative to
the current working directory. Quote filenames that contain spaces.

## Expected output

```
$ just run notes.txt
filename: notes.txt
$ cat notes.txt
<--2026-05-18 23:25:01-->

```
