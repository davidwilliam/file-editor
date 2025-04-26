//! **file-editor** − clean, chain-friendly text-file editing.
//!
//! ```
//! use file_editor::Editor;
//! # fn main() -> std::io::Result<()> {
//! Editor::create("demo.txt")?                 // new file
//!     .append("world\n")
//!     .prepend("hello ")
//!     .insert_after("hello", " rustaceans!\n", false)
//!     .save()?;
//! # Ok(()) }
//! ```
//!
//! See `Editor` for the full API.

mod editor;
pub mod utils;

pub use editor::Editor;
