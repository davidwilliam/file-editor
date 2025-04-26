use file_editor::Editor;

#[test]
fn erase_pattern_not_found() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p = dir.path().join("erase.txt");

    Editor::create(&p)?
        .append("hello\n")
        .erase("nonexistent") // no change
        .save()?;

    assert_eq!(std::fs::read_to_string(p)?, "hello\n");
    Ok(())
}
