//! file_editor – clean, chain-friendly text-file editing
//!
//! ```
//! use file_editor::Editor;
//! # fn main() -> std::io::Result<()> {
//! Editor::create("demo.txt")?
//!     .append("world\n")
//!     .prepend("hello ")
//!     .insert_after("hello", " rustaceans!\n", false)
//!     .save()?;
//! # Ok(()) }
//! ```

mod editor;
mod utils;

pub use editor::Editor;
