//! cargo run --example mask
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/mask.txt");
    Editor::create(p)?
        .append("token=abc123\n")
        .mask("abc123", "***")
        .save()?;
    Ok(())
}
