use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::utils::line_indent;

/// Primary handle to a UTF-8 text file kept in memory until [`save`] is called.
#[derive(Debug, Clone)]
pub struct Editor {
    path: PathBuf,
    buf: String,
    dirty: bool,
}

impl Editor {
    /*──────────────── Constructors ────────────────*/

    /// Creates or truncates a file at `path` and returns an [`Editor`] over it.
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        fs::write(&path, "")?;
        Self::open(path)
    }

    /// Opens an existing text file into memory.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let p = path.as_ref().to_owned();
        let buf = fs::read_to_string(&p)?;
        Ok(Self {
            path: p,
            buf,
            dirty: false,
        })
    }

    /*──────────────── Meta ops ────────────────────*/

    /// Renames the underlying file on disk and updates the internal path.
    pub fn rename<P: AsRef<Path>>(&mut self, new_name: P) -> io::Result<&mut Self> {
        fs::rename(&self.path, &new_name)?;
        self.path = new_name.as_ref().to_owned();
        Ok(self)
    }

    /// Persists the in-memory buffer back to disk if it has been modified.
    pub fn save(&mut self) -> io::Result<&mut Self> {
        if self.dirty {
            fs::write(&self.path, &self.buf)?;
            self.dirty = false;
        }
        Ok(self)
    }

    /*──────────────── Content ops ─────────────────*/

    pub fn prepend(&mut self, text: &str) -> &mut Self {
        self.buf.insert_str(0, text);
        self.dirty = true;
        self
    }

    pub fn append(&mut self, text: &str) -> &mut Self {
        self.buf.push_str(text);
        self.dirty = true;
        self
    }

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

    pub fn insert_after(&mut self, marker: &str, text: &str, same_indent: bool) -> &mut Self {
        if let Some(pos) = self.buf.find(marker) {
            let after_marker = pos + marker.len();

            // New-line aware insert point
            let insert_pos = if self.buf[after_marker..].starts_with('\n') {
                after_marker + 1
            } else {
                after_marker
            };

            // Prepare insertion string
            let mut insertion = text.to_owned();

            // Inline helper space
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

    pub fn find_lines(&self, pattern: &str, limit: Option<usize>) -> Vec<usize> {
        self.buf
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(pattern))
            .map(|(i, _)| i + 1)
            .take(limit.unwrap_or(usize::MAX))
            .collect()
    }

    pub fn erase(&mut self, pattern: &str) -> &mut Self {
        self.buf = self.buf.replace(pattern, "");
        self.dirty = true;
        self
    }

    pub fn replace(&mut self, pattern: &str, replacement: &str) -> &mut Self {
        self.buf = self.buf.replace(pattern, replacement);
        self.dirty = true;
        self
    }

    pub fn mask(&mut self, pattern: &str, mask: &str) -> &mut Self {
        self.replace(pattern, mask)
    }
}
