use file_editor::Editor;

#[test]
fn save_when_not_dirty_is_noop() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p = dir.path().join("noop.txt");

    Editor::create(&p)? // dirty = false
        .save()?; // hit the else-branch

    assert_eq!(std::fs::read_to_string(p)?, "");
    Ok(())
}
