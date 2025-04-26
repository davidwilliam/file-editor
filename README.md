# file-editor &nbsp;📄✏️

Clean, chain-friendly text-file editing for Rust • **edition 2024**

`file-editor` is a tiny, zero-dependency library that makes it painless to create,
modify, and query UTF-8 text files. Every mutating method returns **`&mut self`**,
so edits compose naturally:

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

## Features

| Verb | Method | Notes |
|------|--------|-------|
| Create / open | `Editor::create`, `Editor::open` |
| Rename file   | `rename` |
| Prepend / append | `prepend`, `append` |
| Insert before / after | `insert_before`, `insert_after` <br>`same_indent` flag preserves indentation |
| Replace marker | `replace_marker` |
| Search | `find_lines` → 1-based line numbers |
| Erase / replace / mask | `erase`, `replace`, `mask` |
| Save | `save` (explicit) |

### Road-map

* Regex search / replace (opt-in `regex` feature)  
* Streaming mode for multi-GB files  
* Companion CLI (`cargo install file-editor-cli`)

## Installation

```bash
cargo add file-editor      # latest published version
```

Requires Rust **1.85** or newer (edition 2024).

## 🛠️ Getting started

| Task | Command |
|------|---------|
| **Build** | `cargo build` |
| **Run tests** | `cargo test` |
| **Format** | `cargo fmt --all` |
| **Lint (deny warnings)** | `cargo clippy --all-targets -- -D warnings` |
| **Coverage (HTML report)** |<br>`cargo llvm-cov --workspace --open`<br>*(install once: `cargo install cargo-llvm-cov`)* |

CI runs the same steps on every PR (see `.github/workflows/ci.yml`).

## Quick recipes

| Task | Snippet |
|------|---------|
| Truncate or create a file | `Editor::create("foo.txt")?;` |
| Add a license header to many files | see [`examples/bulk_header.rs`](examples/bulk_header.rs) |
| List first 5 lines containing “TODO” | `editor.find_lines("TODO", Some(5));` |
| Mask API keys | `editor.mask("sk_live_[A-Za-z0-9]+", "***");` |

Full API docs live on **docs.rs**.

---

## Contributing

1. Fork & clone  
2. `cargo fmt --check && cargo clippy --all-targets -- -D warnings`  
3. Add or update tests under `tests/` (use `tempfile`)  
4. Open a PR with a concise description

All contributions are released under the **MIT** license.

---

## License

`file-editor` is licensed under the MIT License.  
See [LICENSE](LICENSE) for the full text.