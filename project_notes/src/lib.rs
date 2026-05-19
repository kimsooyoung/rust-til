//! Append a wrapped local-time marker to a text file.
//!
//! The single public function, [`append_timestamp`], opens the target file
//! in append mode (creating it if missing) and writes `<--YYYY-MM-DD HH:MM:SS-->`
//! followed by a blank line. Useful as a tiny "now" marker in a running notes
//! file.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Local;

/// Append a timestamp marker to `filename`, creating the file if necessary.
pub fn append_timestamp(filename: &Path) -> Result<()> {
    let now_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .with_context(|| format!("opening {} for append", filename.display()))?;
    file.write_all(b"<--")
        .and_then(|()| file.write_all(now_string.as_bytes()))
        .and_then(|()| file.write_all(b"-->"))
        .and_then(|()| file.write_all(b"\n\n"))
        .with_context(|| format!("writing timestamp to {}", filename.display()))?;
    Ok(())
}
