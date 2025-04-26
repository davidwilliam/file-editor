//! cargo run --example insert_after
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/insert_after.txt");
    Editor::create(p)?
        .append("alpha\nomega\n")
        .insert_after("alpha", "beta\ngamma\n", true)
        .save()?;
    Ok(())
}
