use file_editor::Editor;

#[test]
fn insert_after_multiline_same_indent() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("multi.txt");

    Editor::create(&path)?
        .append("if cond {\n    body()\n}\n")
        .insert_after("{", "\nprintln!(\"x\");\nprintln!(\"y\");\n", true)
        .save()?;

    let expected = "\
if cond {

println!(\"x\");
println!(\"y\");
    body()
}
";
    assert_eq!(std::fs::read_to_string(path)?, expected);
    Ok(())
}
