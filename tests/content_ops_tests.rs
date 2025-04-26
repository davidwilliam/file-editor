use file_editor::Editor;

#[test]
fn prepend_append_insert_variants() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("doc.md");

    Editor::create(&path)?
        .append("Line 2\n")
        .append("Line 3\n")
        .prepend("Line 1\n")
        .insert_before("Line 3", "Before-3\n", true)
        .insert_after("Line 1", "After-1\n", true)
        .replace_marker("Line 2", "Two", false)
        .save()?;

    let expected = "\
Line 1
After-1
Two
Before-3
Line 3
";
    assert_eq!(std::fs::read_to_string(path)?, expected);
    Ok(())
}
