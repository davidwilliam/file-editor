use file_editor::Editor;

/// Same-line insert _with_ preceding whitespace ➜ no extra space inserted.
/// Covers the `false` branch of the inline space check.
#[test]
fn insert_after_same_line_already_spaced() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p = dir.path().join("nospace.txt");

    Editor::create(&p)?
        .append("hello _world")
        .insert_after("hello", "_", false)
        .save()?;

    assert_eq!(std::fs::read_to_string(p)?, "hello_ _world");
    Ok(())
}
