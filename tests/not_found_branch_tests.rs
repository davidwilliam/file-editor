use file_editor::Editor;

#[test]
fn operations_when_marker_missing_are_noops() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p = dir.path().join("none.txt");

    // baseline content
    Editor::create(&p)?.append("data\n").save()?;

    // reopen and try edits with a marker that is absent
    let original = std::fs::read_to_string(&p)?;

    let mut ed = Editor::open(&p)?;
    ed.insert_before("ZZZ", "X", false)
        .insert_after("ZZZ", "Y", true)
        .replace_marker("ZZZ", "Q", false)
        .save()?;

    assert_eq!(std::fs::read_to_string(p)?, original);
    Ok(())
}
