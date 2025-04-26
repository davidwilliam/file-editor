//! cargo run --example append
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/append.txt");
    Editor::create(p)?
        .append("first\n")
        .append("second\n")
        .save()?;
    Ok(())
}
