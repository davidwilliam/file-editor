//! cargo run --example replace_marker
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/replace_marker.txt");
    Editor::create(p)?
        .append("    PLACEHOLDER\n")
        .replace_marker("PLACEHOLDER", "VALUE", true)
        .save()?;
    Ok(())
}
