# file-editor

[![CI](https://github.com/davidwilliam/file-editor/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/davidwilliam/file-editor/actions/workflows/ci.yml)  
[![codecov](https://codecov.io/gh/davidwilliam/file-editor/branch/master/graph/badge.svg)](https://app.codecov.io/gh/davidwilliam/file-editor)  
[![crates.io](https://img.shields.io/crates/v/file-editor.svg)](https://crates.io/crates/file-editor)  
[![docs.rs](https://img.shields.io/docsrs/file-editor)](https://docs.rs/file-editor)

Clean, chain-friendly **text-file editing for Rust** • _edition 2024_ • **v0.2.0**

`file-editor` is a zero-dependency library that makes it painless to create,
modify, and query UTF-8 text files.  
All mutating methods return **`&mut self`**, so edits compose naturally:

```rust
use file_editor::Editor;

fn main() -> std::io::Result<()> {
    Editor::create("notes.txt")?               // new (or truncate existing)
        .append("Rust 🦀 is fun\n")
        .prepend("# My Notes\n")
        .insert_after("# My Notes", "========\n", true)
        .replace("fun", "blazingly fast")
        .save()?;                              // explicit flush to disk
    Ok(())
}
```

---

## Opt-in **regex** power

By default `file-editor` has no dependencies.  
Activate the **`regex`** feature to let every pattern-taking method accept a
compiled [`regex::Regex`]:

```toml
file-editor = { version = "0.2.0", features = ["regex"] }
```

```rust,no_run
use file_editor::Editor;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"token=\w+").unwrap();
    Editor::open("config.env")?
        .mask(&re, "***")      // redact every `token=…`
        .save()?;
    Ok(())
}
```

---

## Feature table

| Verb                       | Method(s)                          | Notes                                           |
|----------------------------|------------------------------------|-------------------------------------------------|
| **Create / open**          | `Editor::create`, `Editor::open`  | `create` truncates an existing file             |
| **Rename**                 | `rename`                          | Renames on disk & updates the internal path     |
| **Prepend / append**       | `prepend`, `append`               |                                                 |
| **Insert before / after**  | `insert_before`, `insert_after`   | `same_indent` flag preserves indentation        |
| **Replace marker**         | `replace_marker`                  | Optional `same_indent`                         |
| **Search pattern**         | `find_lines`                      | Returns **1-based** line numbers                |
| **Erase / replace / mask** | `erase`, `replace`, `mask`        | Operate on *all* occurrences in the buffer      |
| **Save**                   | `save`                            | Writes only when the buffer is dirty            |

_Planned &gt; streaming mode, companion CLI…_

---

## Installation

```bash
# stable crate (no regex):
cargo add file-editor

# with regex support:
cargo add file-editor --features regex
```

Requires **Rust 1.85** or newer (edition 2024).

---

## Workflow

| Task                         | Command                                                        |
|------------------------------|----------------------------------------------------------------|
| **Build**                    | `cargo build`                                                  |
| **Run tests & doctests**     | `cargo test`                                                   |
| **Format**                   | `cargo fmt --all`                                              |
| **Lint (deny warnings)**     | `cargo clippy --all-targets -- -D warnings`                    |
| **Coverage (HTML + terminal)** | `cargo llvm-cov --workspace --open` <br/>*(install once: `cargo install cargo-llvm-cov`)* |
| **Generate docs**            | `cargo doc --no-deps --open`                                   |

_CI runs the same commands on every PR (see `.github/workflows/ci.yml`)._

---

## Runnable examples

Everything under **`examples/`** writes into `examples/sandbox/` (ignored by Git):

| Example                    | Demonstrates                                      |
|----------------------------|---------------------------------------------------|
| `basic.rs`                 | End-to-end mini workflow                          |
| `create.rs`                | `create` + `append`                               |
| `open_rename.rs`           | `open` + `rename`                                 |
| `prepend.rs`, `append.rs`  | Header / footer insertion                         |
| `insert_before.rs`         | Block insertion above a marker                    |
| `insert_after.rs`          | Block insertion below a marker                    |
| `replace_marker.rs`        | In-place marker replacement                       |
| `find_lines.rs`            | Pattern search                                    |
| `erase.rs`                 | Deleting fragments                                |
| `replace.rs`, `mask.rs`    | Global replacements / masking                     |

```bash
cargo run --example insert_after
```

---

## Contributing

1. Fork & clone  
2. `cargo fmt --check && cargo clippy --all-targets -- -D warnings`  
3. Add or update tests in `tests/` (use the `tempfile` crate)  
4. Open a PR describing **what** changed and **why**  

_All contributions are released under the **MIT** license._

---

## License

Copyright © 2025   
Licensed under the **MIT** License. See [LICENSE](LICENSE) for details.  