//! cargo run --example open_rename
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let src = Path::new("examples/sandbox/original.txt");
    let dst = Path::new("examples/sandbox/renamed.txt");

    Editor::create(src)?.append("content\n").save()?;
    let mut ed = Editor::open(src)?;
    ed.rename(dst)?.append("renamed\n").save()?;

    println!("Renamed to {}", dst.display());
    Ok(())
}
