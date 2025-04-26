# file-editor

Clean, chain-friendly text-file editing for Rust • **edition 2024**

`file-editor` is a small, zero-dependency library that makes it painless to create,
modify, and query UTF-8 text files.  It embraces *builder semantics*, every mutating
method returns `&mut self`, so you can compose edits in a single expression.

```rust
use file_editor::Editor;

fn main() -> anyhow::Result<()> {
    Editor::create("notes.txt")?          // new file
        .append("Rust 🦀 is fun\n")
        .prepend("# My Notes\n")
        .insert_after("# My Notes", "========\n", true)
        .replace("fun", "blazingly fast")
        .save()?;                         // explicit flush
    Ok(())
}
```

---

## Features

| Verb | Method | Notes |
|------|--------|-------|
| **create / open file** | `Editor::create`, `Editor::open` |
| **rename** | `rename` |
| **prepend / append** | `prepend`, `append` |
| **insert before / after marker** | `insert_before`, `insert_after` <br>optionally preserve indentation |
| **replace marker** | `replace_marker` |
| **search** | `find_lines` → 1-based line numbers |
| **erase / replace / mask** | `erase`, `replace`, `mask` |
| **save** | explicit flush to disk |

Planned (road-map):

* **Regex** support behind `regex` feature flag  
* **Streaming** mode for gigabyte-scale files  
* **Cross-platform CLI** (`cargo install file-editor-cli`)  

## Installation

```bash
cargo add file-editor      # latest stable
```

`file-editor` requires Rust **1.85** or newer (2024 edition).

## Quick recipes

| Task | Snippet |
|------|---------|
| Create or truncate a file | `Editor::create("foo.txt")?;` |
| Add a license header to many files | see [`examples/bulk_header.rs`](examples/bulk_header.rs) |
| List first 5 lines containing `TODO` | `editor.find_lines("TODO", Some(5));` |
| Mask API keys | `editor.mask("sk_live_[A-Za-z0-9]+", "***");` |

## Contributing

1. Fork & clone the repo  
2. `cargo fmt --check && cargo clippy --all-targets`  
3. Add tests in `tests/` (use the `tempfile` crate)  
4. Submit a PR with a concise description

All contributions are released under the **MIT** license.

## 📄  License

Licensed under

* MIT license [LICENSE](LICENSE)