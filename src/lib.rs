//! # file-editor
//! **Fluent, chain-friendly text-file editing for Rust (edition 2024)**  
//!
//! ---
//! ## Quick taste
//! ```no_run
//! use file_editor::Editor;
//!
//! fn main() -> std::io::Result<()> {
//!     Editor::create("demo.txt")?            // create or truncate
//!         .append("world\n")
//!         .prepend("hello ")
//!         .insert_after("hello", " Rustaceans!\n", false)
//!         .save()?;                         // flush to disk
//!     Ok(())
//! }
//! ```
//!
//! ---
//! ## Opt-in **regex** power  
//! The default build is **zero-dependency**.  
//! Enable the feature below to pass a compiled `regex::Regex` anywhere a pattern is accepted.
//!
//! ```toml
//! file-editor = { version = "0.2", features = ["regex"] }
//! ```
//!
//! ```ignore
//! use file_editor::Editor;
//! use regex::Regex;
//!
//! fn main() -> std::io::Result<()> {
//!     let re = Regex::new(r"token=\w+")?;      // redact secrets
//!     Editor::open("config.env")?
//!         .mask(&re, "***")
//!         .save()?;
//!     Ok(())
//! }
//! ```
//!
//! ---
//! **See [`Editor`] for the complete API and method-by-method examples.**

mod editor;
mod pattern;
pub mod utils;

pub use editor::Editor;
