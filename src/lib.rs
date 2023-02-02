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
use std::path::Path;

/* Internal Modules */
pub mod editor;

/* Convenience functions */
/// New opens an empty text buffer in an editor and returns a Readable struct on success.
///
/// # Example
/// ```no_run()
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
    editor::new().open(editor::Contents::Empty)
}

/// With opens a text buffer with the provided contents in an editor. Returns a Readble struct on success.
///
/// # Example
/// ```no_run
/// # use std::error::Error;
/// # use std::io::Read;
/// # fn main() -> Result<(), Box<dyn Error>> {
///     /* Opens the user's editor, buffer pre-filled with custom content */
///     let input = scrawl::with(&"What is your favorite color")?;
///     println!("{}", input.to_string()?);
/// #   Ok(())
/// # }
/// ```
pub fn with<U: AsRef<[u8]>>(input: &U) -> Result<editor::Reader, Box<dyn Error>> {
    editor::new().open(editor::Contents::FromString(input))
}

/// FromFile opens a text buffer with the content of the provided file in an editor. Returns a Readble struct on success.
///
/// # Example
/// ```no_run
/// # use std::error::Error;
/// # use std::io::Read;
/// # use std::path::Path;
/// # fn main() -> Result<(), Box<dyn Error>> {
///     /* Opens the user's editor, buffer pre-filled with custom content */
///     let path = Path::new("foo.txt"); 
///     let input = scrawl::from_file(path)?;
///     println!("{}", input.to_string()?);
/// #   Ok(())
/// # }
/// ```
pub fn from_file<P: AsRef<Path>>(path: &P) -> Result<editor::Reader, Box<dyn Error>> {
    editor::new().open(editor::Contents::FromFile(path))
}


/// EditFile opens a text buffer with the content of the provided file, allowing direct editing in an editor. Returns a Readble struct on success.
///
/// # Example
/// ```no_run
/// # use std::error::Error;
/// # use std::io::Read;
/// # use std::path::Path;
/// # fn main() -> Result<(), Box<dyn Error>> {
///     /* Opens the user's editor, buffer pre-filled with custom content */
///     let path = Path::new("bar.rs"); 
///     let input = scrawl::edit_file(path)?;
///     println!("{}", input.to_string()?);
/// #   Ok(())
/// # }
/// ```
pub fn edit_file<P: AsRef<Path>>(path: &P) -> Result<editor::Reader, Box<dyn Error>> {
    editor::new().edit(path)
}

