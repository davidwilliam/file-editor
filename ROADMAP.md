# file-editor Roadmap

This document tracks the major milestones and planned work for the `file-editor` crate.

## v0.1.0 — Initial release

**Completed** (2025-04-27)

- **Basic file lifecycle**
  - `Editor::create` (truncates or new file)  
  - `Editor::open`  
  - `Editor::rename`  
  - `Editor::save` (writes only when dirty)  
- **Content operations**
  - `prepend` / `append`  
  - `insert_before` / `insert_after` (with optional same-line vs newline behavior)  
  - `replace_marker` (first occurrence)  
- **Search & transform**
  - `find_lines` → returns 1-based line numbers  
  - `erase`, `replace`, `mask` (all occurrences, literal substrings)  
- **Test suite** covering all operations  
- **Lint & formatting** via Clippy & `rustfmt`  
- **CI** with tests, Clippy, and coverage gating  

## v0.2.0 — Opt-in Regex support

**Completed** (2025-04-29)

- **Feature flag** `regex`  
- **Pattern abstraction**  
  - new `pub enum Pattern<'a>`  
  - `From<&str>` (always) + `From<&Regex>` (when `regex` feature enabled)  
- **Enhanced APIs**  
  - `find_lines<P: Into<Pattern>>`  
  - `erase<P: Into<Pattern>>`  
  - `replace<P: Into<Pattern>>`  
  - `mask<P: Into<Pattern>>`  
- **Zero-dependency by default**  
  - no extra crates until you enable `regex`  
- **Documentation** updated with usage examples and feature-flag instructions  
- **Examples** in `examples/` folder expanded  

## Upcoming for v0.3.0+

*(not yet scheduled — feedback welcome!)*

- **Streaming mode** for very large files (line-by-line or chunked edits)  
- **Regex replacer enhancements**  
  - support for `RegexSet` or multi-pattern?  
- **`cli` companion**  
  - small binary to apply edits via command line  
- **`regex` feature improvements**  
  - add `default_mask` for `mask()`  
- **Windows path & encoding edge cases**  
- **Performance benchmarks & tuning**  

### How to contribute to the roadmap

1. Open an issue or discussion with your feature request.  
2. Tag it with `enhancement` and drop a brief “why it matters.”  
3. Once design is agreed, pick a milestone and submit a PR.  

Your feedback and PRs are always welcome!  
