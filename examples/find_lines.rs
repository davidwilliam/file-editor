//! cargo run --example find_lines
use file_editor::Editor;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let p = Path::new("examples/sandbox/find.txt");
    Editor::create(p)?
        .append("foo\nbar\nfoo\nbaz\n") // two “foo” matches
        .save()?;

    let ed = Editor::open(p)?;
    println!("foo at lines {:?}", ed.find_lines("foo", None));
    Ok(())
}
