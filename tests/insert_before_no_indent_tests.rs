use file_editor::Editor;

#[test]
fn insert_before_without_same_indent() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p = dir.path().join("ib.txt");

    Editor::create(&p)?
        .append("marker\n")
        .insert_before("marker", "// comment\n", /*same_indent=*/ false)
        .save()?;

    let expected = "// comment\nmarker\n";
    assert_eq!(std::fs::read_to_string(p)?, expected);
    Ok(())
}
