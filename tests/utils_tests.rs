use file_editor::utils::line_indent;

#[test]
fn indent_first_line() {
    let txt = "   abc\nnext";
    let pos = txt.find('a').unwrap();
    assert_eq!(line_indent(txt, pos), "   ");
}

#[test]
fn indent_after_newline() {
    let txt = "no indent\n\t\txyz";
    let pos = txt.find('x').unwrap();
    assert_eq!(line_indent(txt, pos), "\t\t");
}
