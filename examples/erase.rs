//! cargo run --example erase
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/erase.txt");
    Editor::create(p)?
        .append("remove me\nkeep me\n")
        .erase("remove me")
        .save()?;
    Ok(())
}
