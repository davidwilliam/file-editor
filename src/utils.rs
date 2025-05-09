/// Return the leading whitespace of the line that contains `pos`.
///
/// ```
/// use file_editor::utils::line_indent;
/// let txt = "   indented\nnext";
/// let pos = txt.find('i').unwrap();
/// assert_eq!(line_indent(txt, pos), "   ");
/// ```
pub fn line_indent(buf: &str, pos: usize) -> String {
    let line_start = buf[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    buf[line_start..pos]
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::line_indent;

    #[test]
    fn indent_first_line() {
        let txt = "   leading spaces\nnext";
        let pos = txt.find('l').unwrap();
        assert_eq!(line_indent(txt, pos), "   ");
    }

    #[test]
    fn indent_after_newline() {
        let txt = "no indent\n\t\tindented";
        let pos = txt.find("indented").unwrap();
        assert_eq!(line_indent(txt, pos), "\t\t");
    }
}
