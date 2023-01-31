//! # Scrawl
//! A library for opening a file for editing in a text editor and capturing the result as a String
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

/* Standard Library */
use std::error::Error;

/* Internal Modules */
mod editor;
pub use editor::Editor as Editor;

/* Convenience functions */
/// New opens an empty text buffer in an editor and returns a Result<String> with the contents.
///
/// # Example
/// ```
/// # use std::error::Error;
/// # use std::io::Read;
/// # fn main() -> Result<(), Box<dyn Error>> {
///     /* Opens the user's editor */
///     let input = scrawl::new()?;
///     println!("{}", input.to_string()?);
/// #   Ok(())
/// # }
/// ```
pub fn new() -> Result<editor::Reader, Box<dyn Error>> {
    editor::Editor::new().open()
}

