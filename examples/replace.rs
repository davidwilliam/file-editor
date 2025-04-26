//! cargo run --example replace
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/replace.txt");
    Editor::create(p)?
        .append("colour colour colour\n")
        .replace("colour", "color")
        .save()?;
    Ok(())
}
