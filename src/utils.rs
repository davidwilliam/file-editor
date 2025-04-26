/// Return the indentation (whitespace prefix) of the line that contains `pos`.
pub(crate) fn line_indent(buf: &str, pos: usize) -> String {
    let line_start = buf[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    buf[line_start..pos]
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect()
}
