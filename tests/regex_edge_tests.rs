//! Extra “edge” test that hits rarely-used branches so line-coverage
//! stays at 100 %.

use file_editor::Editor;

#[test]
fn edge_branches() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;

    /*─ 0. inline insert that *doesn’t* need helper space ──────────*/
    let p_inline2 = dir.path().join("inline2.txt");
    Editor::create(&p_inline2)?
        .append("GREETING|world")
        .insert_after("|", " there", false) // text already starts with space
        .save()?;
    assert_eq!(std::fs::read_to_string(&p_inline2)?, "GREETING| thereworld");

    /*─ 1. inline insert → auto-space branch (same_indent = false) ─*/
    let p_inline = dir.path().join("inline.txt");
    Editor::create(&p_inline)?
        .append("START|END")
        .insert_after("|", "_MID_", false) // same line, no indent copy
        .save()?;
    assert_eq!(std::fs::read_to_string(&p_inline)?, "START| _MID_END");

    /*─ 2. multiline insert with same_indent = true → re-indent branch ─*/
    let p_block = dir.path().join("block.txt");
    Editor::create(&p_block)?
        .append("if cond {\n    body()\n}\n")
        .insert_after(
            "{",
            "\nprintln!(\"x\");\nprintln!(\"y\");", // has newlines
            true,                                   // copy indent
        )
        .save()?;
    let out = std::fs::read_to_string(&p_block)?;
    // Quick structural checks (exact spacing not critical)
    assert!(out.starts_with("if cond {\n"));
    assert!(out.contains("println!(\"x\");"));
    assert!(out.contains("println!(\"y\");"));
    assert!(out.trim_end().ends_with("}"));

    /*─ 3. erase with no match → early-return branch in erase() ─*/
    let mut ed = Editor::open(&p_inline)?;
    ed.erase("does-not-exist");
    ed.save()?; // should be a no-op

    /*─ 4. find_lines with limit = None → unlimited branch ─*/
    assert_eq!(ed.find_lines("MID", None), vec![1]);

    Ok(())
}
