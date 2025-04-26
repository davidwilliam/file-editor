use file_editor::Editor;

#[test]
fn create_open_rename_save() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let p1 = dir.path().join("foo.txt");
    let p2 = dir.path().join("bar.txt");

    // create + save
    Editor::create(&p1)?
        .append("hello") // mark dirty
        .save()?; // flush

    assert_eq!(std::fs::read_to_string(&p1)?, "hello");

    // open + rename
    let mut ed = Editor::open(&p1)?;
    ed.rename(&p2)?.append(" world").save()?;

    assert!(!p1.exists());
    assert_eq!(std::fs::read_to_string(&p2)?, "hello world");
    Ok(())
}
