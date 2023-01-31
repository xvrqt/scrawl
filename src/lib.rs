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
pub mod editor;

/* Convenience functions */
/// New opens an empty text buffer in an editor and returns a Result<String> with the contents.
///
/// # Example
/// ```
/// # use scrawl::error::ScrawlError;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
///     /* Opens the user's editor */
///     let editor = scrawl::new()?;
///     /* Waits for the user to end the program and returns a Read-able object */
///     let output = editor.collect()?; 
///     for line in output { println!(line); }
/// #   Ok(())
/// # }
/// ```
pub fn new() -> Result<Editor, Box<dyn Error>> {
    editor::new().open()
}

