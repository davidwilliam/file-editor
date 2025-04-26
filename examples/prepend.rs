//! cargo run --example prepend
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/prepend.txt");
    Editor::create(p)?
        .append("world\n")
        .prepend("hello ")
        .save()?;
    Ok(())
}
