use file_editor::Editor;

/// touches prepend/append dirty flags *without* any further edits,
/// and covers insert_after on same line with same_indent = true
#[test]
fn edge_coverage() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("edge.txt");

    // prepend & append make the file dirty, save() writes once
    Editor::create(&path)?
        .prepend("START|") // dirty = true  (now covered)
        .append("|END") // dirty = true  (now covered)
        // same-line insertion, same_indent = true but no '\n'
        .insert_after("START", "MIDDLE", /*same_indent=*/ true)
        .save()?; // flush

    assert_eq!(std::fs::read_to_string(path)?, "START MIDDLE||END");
    Ok(())
}
