use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::pattern::Pattern;
use crate::utils::line_indent;

/// Handle to a UTF-8 text file kept in memory until [`save`](Editor::save) is called.
///
/// All mutating methods return `&mut self`, enabling a fluent builder style.
///
/// ```no_run
/// # use file_editor::Editor;
/// # fn run() -> std::io::Result<()> {
/// Editor::open("Cargo.toml")?
///     .insert_before("[dependencies]", "regex = \"1\"\n", false)
///     .save()?;
/// # Ok(()) }
/// ```
#[derive(Debug, Clone)]
pub struct Editor {
    path: PathBuf,
    buf: String,
    dirty: bool,
}

impl Editor {
    /// **Create** or truncate a file and return an editor over it.
    ///
    /// Equivalent to `fs::write(path, "")` followed by [`open`](Editor::open).
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        fs::write(&path, "")?;
        Self::open(path)
    }

    /// **Open** an existing UTF-8 file into an in-memory buffer.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let p = path.as_ref().to_owned();
        let buf = fs::read_to_string(&p)?;
        Ok(Self {
            path: p,
            buf,
            dirty: false,
        })
    }

    /// Rename the underlying file on disk **and** update the internal path.
    pub fn rename<P: AsRef<Path>>(&mut self, new_name: P) -> io::Result<&mut Self> {
        fs::rename(&self.path, &new_name)?;
        self.path = new_name.as_ref().to_owned();
        Ok(self)
    }

    /// Write the in-memory buffer back to disk **iff** it was modified.
    ///
    /// Returns `Ok(self)` even when there was nothing to do.
    pub fn save(&mut self) -> io::Result<&mut Self> {
        if self.dirty {
            fs::write(&self.path, &self.buf)?;
            self.dirty = false;
        }
        Ok(self)
    }

    /// Insert `text` **at the beginning** of the buffer.
    pub fn prepend(&mut self, text: &str) -> &mut Self {
        self.buf.insert_str(0, text);
        self.dirty = true;
        self
    }

    /// Append `text` **to the end** of the buffer.
    pub fn append(&mut self, text: &str) -> &mut Self {
        self.buf.push_str(text);
        self.dirty = true;
        self
    }

    /// Insert `text` **before** the first occurrence of `marker`.
    ///
    /// * If `same_indent` is `true`, the current indentation of the line
    ///   containing `marker` is copied and prepended to `text`.
    pub fn insert_before(&mut self, marker: &str, text: &str, same_indent: bool) -> &mut Self {
        if let Some(pos) = self.buf.find(marker) {
            let insertion = if same_indent {
                format!("{}{}", line_indent(&self.buf, pos), text)
            } else {
                text.to_owned()
            };
            self.buf.insert_str(pos, &insertion);
            self.dirty = true;
        }
        self
    }

    /// Insert `text` **after** the first occurrence of `marker`.
    ///
    /// * If `marker` ends a line, the insertion starts on the next line.  
    /// * Otherwise the insertion is in-line; a space is auto-inserted when needed.  
    /// * When `same_indent` is `true`, every *subsequent* line in `text`
    ///   is indented to match the marker line.
    pub fn insert_after(&mut self, marker: &str, text: &str, same_indent: bool) -> &mut Self {
        if let Some(pos) = self.buf.find(marker) {
            let after_marker = pos + marker.len();
            let insert_pos = if self.buf[after_marker..].starts_with('\n') {
                after_marker + 1 // insert on next line
            } else {
                after_marker // insert in-line
            };

            let mut insertion = text.to_owned();

            // Auto-space for inline insertions like `foo|bar` → `foo X bar`
            if insert_pos == after_marker
                && !insertion.starts_with(char::is_whitespace)
                && !self.buf[insert_pos..].starts_with(char::is_whitespace)
            {
                insertion.insert(0, ' ');
            }

            // Re-indent multiline insertions
            if same_indent && insertion.contains('\n') {
                let indent = line_indent(&self.buf, pos);
                insertion = insertion
                    .split('\n')
                    .enumerate()
                    .map(|(i, line)| {
                        if i == 0 {
                            line.to_owned()
                        } else {
                            format!("{indent}{line}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }

            self.buf.insert_str(insert_pos, &insertion);
            self.dirty = true;
        }
        self
    }

    /// Replace the first occurrence of `marker` with `text`.
    ///
    /// When `same_indent` is `true`, the replacement receives the indentation
    /// that preceded the marker.
    pub fn replace_marker(&mut self, marker: &str, text: &str, same_indent: bool) -> &mut Self {
        if let Some(pos) = self.buf.find(marker) {
            let indent = if same_indent {
                line_indent(&self.buf, pos)
            } else {
                String::new()
            };
            self.buf = self.buf.replacen(marker, &(indent + text), 1);
            self.dirty = true;
        }
        self
    }

    /// Return 1-based line numbers where `pattern` occurs.
    ///
    /// Pass `limit = Some(n)` to cap the results.
    pub fn find_lines<'a, P>(&self, pattern: P, limit: Option<usize>) -> Vec<usize>
    where
        P: Into<Pattern<'a>>,
    {
        let pat = pattern.into();
        self.buf
            .lines()
            .enumerate()
            .filter(|(_, line)| pat.is_match(line))
            .map(|(i, _)| i + 1)
            .take(limit.unwrap_or(usize::MAX))
            .collect()
    }

    /// Erase _all_ occurrences of `pattern`.
    pub fn erase<'a, P>(&mut self, pattern: P) -> &mut Self
    where
        P: Into<Pattern<'a>>,
    {
        let pat = pattern.into();
        self.buf = pat.replace_all(&self.buf, "");
        self.dirty = true;
        self
    }

    /// Replace _all_ occurrences of `pattern` with `replacement`.
    pub fn replace<'a, P>(&mut self, pattern: P, replacement: &str) -> &mut Self
    where
        P: Into<Pattern<'a>>,
    {
        let pat = pattern.into();
        self.buf = pat.replace_all(&self.buf, replacement);
        self.dirty = true;
        self
    }

    /// Mask _all_ occurrences of `pattern` with `mask`.
    pub fn mask<'a, P>(&mut self, pattern: P, mask: &str) -> &mut Self
    where
        P: Into<Pattern<'a>>,
    {
        self.replace(pattern, mask)
    }
}
