use file_editor::Editor;

#[test]
fn insert_after_inline_multiline_same_indent() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("final.txt");

    // marker inside a single line, no surrounding whitespace
    Editor::create(&path)?
        .append("fooBARbaz") // <- marker “BAR” inline
        .insert_after("BAR", "X\ny", /*same_indent=*/ true)
        .save()?;

    // one space auto-inserted before 'X', newline re-indented with zero spaces
    assert_eq!(std::fs::read_to_string(path)?, "fooBAR X\nybaz");
    Ok(())
}
