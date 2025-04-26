//! Basic usage demo for the `file-editor` crate.
//!
//! Run with:
//! ```bash
//! cargo run --example basic
//! ```

use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out = Path::new("examples/sandbox/notes.txt");

    Editor::create(out)?
        .append("Rust 🦀 is fun\n")
        .prepend("# My Notes\n")
        .insert_after("# My Notes", "========\n", true)
        .replace("fun", "blazingly fast")
        .save()?;

    println!("wrote {}", out.display());
    println!("run cat {} to see the result.", out.display());
    Ok(())
}
