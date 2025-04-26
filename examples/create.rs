//! cargo run --example create
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("examples/sandbox/create.txt");

    Editor::create(path)?.append("hello\n").save()?;

    println!("Created {}", path.display());
    Ok(())
}
