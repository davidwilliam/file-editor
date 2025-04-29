use file_editor::Editor;

/// Exercise `insert_before` with `same_indent = false` (no indent copy).
#[test]
fn insert_before_no_indent_branch() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("plain.txt");

    Editor::create(&path)?
        .append("alpha beta gamma") // single line
        .insert_before("beta", "X_", false) // same_indent = false → no extra spaces
        .save()?;

    assert_eq!(std::fs::read_to_string(path)?, "alpha X_beta gamma");
    Ok(())
}
