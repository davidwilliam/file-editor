#![cfg(feature = "regex")]

use file_editor::Editor;
use regex::Regex;

#[test]
fn regex_replace_mask() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("data.txt");

    std::fs::write(&path, "token=abc123\ntoken=def456\n")?;

    let re = Regex::new(r"token=\w+").unwrap();

    Editor::open(&path)?
        .replace(&re, "TOKEN") // use &Regex
        .save()?;

    assert_eq!(std::fs::read_to_string(path)?, "TOKEN\nTOKEN\n");
    Ok(())
}
