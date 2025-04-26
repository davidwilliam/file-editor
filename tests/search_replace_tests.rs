use file_editor::Editor;

#[test]
fn find_erase_replace_mask() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("data.txt");

    let text = "\
token=abc123
password=topsecret
token=def456
";
    std::fs::write(&path, text)?;

    // open + query
    let mut ed = Editor::open(&path)?;
    assert_eq!(ed.find_lines("token", None), vec![1, 3]);
    assert_eq!(ed.find_lines("token", Some(1)), vec![1]);

    // erase + replace + mask
    ed.erase("password=")
        .replace("abc123", "xyz999")
        .mask(r#"def456"#, "***")
        .save()?;

    let out = std::fs::read_to_string(path)?;
    assert_eq!(
        out,
        "\
token=xyz999
topsecret
token=***
"
    );
    Ok(())
}
