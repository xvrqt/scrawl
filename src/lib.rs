//! # Scrawl
//! A library for opening a file for editing in a text editor and capturing the result as a String
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

/* Standard Library */
use std::path::Path;

/* Internal Modules */
pub mod error;
use error::ScrawlError as ScrawlError;

pub mod editor;
use editor::Editor as Editor;

/* Convenience functions */
/// New opens an empty text buffer in an editor and returns a Result<String> with the contents.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # fn main() -> Result<(), ScrawlError> {
///     let output = scrawl::new()?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn new() -> Result<String, ScrawlError> {
    Editor::new().edit()
}

/// With opens a text buffer with the contents of the provided String in an editor. Returns a Result<String> with the edited contents.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # fn main() -> Result<(), ScrawlError> {
///     let output = scrawl::with("Hello World!")?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn with(content: &str) -> Result<String, ScrawlError> {
    Editor::new().contents(content).edit()
}

/// Open opens a text buffer in an editor with the contents of the file specified. This does **not** edit the contents of the file. Returns a Result<String> with the contents of the buffer.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # use std::path::Path;
///
/// # fn main() -> Result<(), ScrawlError> {
///     let path = Path::new("hello.txt");
///     let output = scrawl::open(path)?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn open(p: &Path) -> Result<String, ScrawlError> {
    Editor::new().file(p).edit()
}

/// Edit opens a text buffer in an editor with the contents of the file specified. This **does** edit the contents of the file. Returns a Result<String> with the contents of the buffer.
///
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
/// # use std::path::Path;
///
/// # fn main() -> Result<(), ScrawlError> { 
///     let path = Path::new("hello.txt");
///     let output = scrawl::edit(path)?;
///     println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn edit(p: &Path) -> Result<String, ScrawlError> {
    Editor::new().file(p).edit_directly(true).edit()
}

