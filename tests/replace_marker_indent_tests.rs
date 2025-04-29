use file_editor::Editor;

#[test]
fn replace_marker_same_indent() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p = dir.path().join("rm.txt");

    Editor::create(&p)?
        .append("    KEY\n") // 4-space indent
        .replace_marker("KEY", "VALUE", true)
        .save()?;

    // original 4 spaces + replicated 4 spaces = 8
    assert_eq!(std::fs::read_to_string(p)?, "        VALUE\n");
    Ok(())
}
