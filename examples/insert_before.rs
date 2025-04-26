//! cargo run --example insert_before
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/insert_before.txt");
    Editor::create(p)?
        .append("Line A\nLine C\n")
        .insert_before("Line C", "Line B\n", true)
        .save()?;
    Ok(())
}
