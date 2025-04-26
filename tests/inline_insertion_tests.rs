// tests/inline_insertion_tests.rs
use file_editor::Editor;

/// cover: `insert_after` on the *same line* (`insert_pos == after_marker`)
#[test]
fn insert_after_same_line_no_space_needed() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("inline.txt");

    Editor::create(&path)?
        .append("hello:world") // no newline
        .insert_after("hello", "_", false)
        .save()?;

    assert_eq!(std::fs::read_to_string(path)?, "hello _:world");
    Ok(())
}
