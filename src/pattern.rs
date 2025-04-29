//! Pattern abstraction: literal `&str` or (with `regex`) compiled `Regex`.
#[cfg(feature = "regex")]
use regex::Regex;

/// Pattern: either a substring literal or, if you enable the `regex` feature, a `Regex`.
pub enum Pattern<'a> {
    /// Literal substring match.
    Literal(&'a str),
    /// Regex match (opt-in via `features = ["regex"]`).
    #[cfg(feature = "regex")]
    Re(&'a Regex),
}

impl Pattern<'_> {
    /// Returns `true` if the pattern matches `hay`.
    pub fn is_match(&self, hay: &str) -> bool {
        match *self {
            Pattern::Literal(s) => hay.contains(s),
            #[cfg(feature = "regex")]
            Pattern::Re(re) => re.is_match(hay),
        }
    }

    /// Replaces all matches in `buf` with `repl`, returning a new `String`.
    pub fn replace_all(&self, buf: &str, repl: &str) -> String {
        match *self {
            Pattern::Literal(s) => buf.replace(s, repl),
            #[cfg(feature = "regex")]
            Pattern::Re(re) => re.replace_all(buf, repl).into(),
        }
    }
}

/// Convert a `&str` into a `Pattern::Literal`.
impl<'a> From<&'a str> for Pattern<'a> {
    fn from(s: &'a str) -> Self {
        Pattern::Literal(s)
    }
}

#[cfg(feature = "regex")]
impl<'a> From<&'a Regex> for Pattern<'a> {
    fn from(re: &'a Regex) -> Self {
        Pattern::Re(re)
    }
}
