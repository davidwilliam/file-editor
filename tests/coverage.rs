//! Hit every rare branch in `insert_after`, `insert_before`, `erase`, and `find_lines`.

use file_editor::Editor;

#[test]
fn coverage_edge_cases() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;

    // 1) inline insert — NEED auto-space (same_indent = false, no leading whitespace)
    let p1 = dir.path().join("e1.txt");
    Editor::create(&p1)?
        .append("FOO|BAR")
        .insert_after("|", "_X_", false)
        .save()?;
    assert_eq!(std::fs::read_to_string(&p1)?, "FOO| _X_BAR");

    // 2) inline insert — SKIP auto-space (same_indent = false, text already starts with a space)
    let p2 = dir.path().join("e2.txt");
    Editor::create(&p2)?
        .append("FOO|BAR")
        .insert_after("|", " Z", false)
        .save()?;
    assert_eq!(std::fs::read_to_string(&p2)?, "FOO| ZBAR");

    // 3) insert_before WITHOUT indent copy (same_indent = false)
    let p3 = dir.path().join("e3.txt");
    Editor::create(&p3)?
        .append("marker\n")
        .insert_before("marker", "// note\n", false)
        .save()?;
    assert_eq!(std::fs::read_to_string(&p3)?, "// note\nmarker\n");

    // 4) multiline insert with same_indent = true (re-indent path)
    let p4 = dir.path().join("e4.txt");
    Editor::create(&p4)?
        .append("if foo {\n    bar();\n}\n")
        .insert_after(
            "{",
            "\nbaz();\nquux();", // contains newlines
            true,                // copy indent of marker line
        )
        .save()?;
    let out4 = std::fs::read_to_string(&p4)?;
    // We don't assert exact spaces—just that the three calls appear in order
    let i_baz = out4.find("baz();").unwrap();
    let i_quux = out4.find("quux();").unwrap();
    let i_bar = out4.find("bar();").unwrap();
    assert!(
        i_baz < i_quux && i_quux < i_bar,
        "baz(), quux(), bar() must appear in that order"
    );

    // 5) erase when nothing matches → early-return branch
    let mut ed = Editor::open(&p1)?;
    ed.erase("NOTHING"); // no match
    ed.save()?; // still no-op

    // 6) find_lines(limit=None) → unlimited branch
    assert_eq!(ed.find_lines("_X_", None), vec![1]);

    Ok(())
}

#[test]
fn cover_last_branches() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;

    // 1) same_indent=true but no newline → skip re-indent
    let p1 = dir.path().join("c1.txt");
    Editor::create(&p1)?
        .append("A|B")
        .insert_after("|", "X", true) // no '\n' in "X"
        .save()?;
    // auto-space STILL happens (no re-indent), so we expect "A| XB"
    assert_eq!(std::fs::read_to_string(&p1)?, "A| XB");

    // 2) skip auto-space because buffer has whitespace next
    let p2 = dir.path().join("c2.txt");
    Editor::create(&p2)?
        .append("A| WORLD")
        .insert_after("|", "_X_", false) // insertion does not start with space
        .save()?;
    // buffer had a space, so we expect no extra space: "A|_X_ WORLD"
    assert_eq!(std::fs::read_to_string(&p2)?, "A|_X_ WORLD");

    // 3) insert_before with same_indent=true but no newline in text → skip indent copy
    //    (covers the “else” arm inside insert_before’s same_indent logic)
    let p3 = dir.path().join("c3.txt");
    Editor::create(&p3)?
        .append("foo\n")
        .insert_before("foo", "//!", true) // same_indent=true but no '\n' in "///"
        .save()?;
    // should just prefix exactly, not copy indent
    assert_eq!(std::fs::read_to_string(&p3)?, "//!foo\n");

    // 4) replace_marker when marker missing → early return
    let p4 = dir.path().join("c4.txt");
    Editor::create(&p4)?
        .append("nothing here\n")
        .replace_marker("xyz", "ZZZ", true) // no-op
        .save()?;
    assert_eq!(std::fs::read_to_string(&p4)?, "nothing here\n");

    Ok(())
}
