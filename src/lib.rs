//! # Scrawl 
//! A library for opening a file for editing in a text editor and capturing the result as a String
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

use std::{
    fs,
    env::{temp_dir, var},
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    process::Command
};

pub mod error;
use error::ScrawlError as ScrawlError;

/// New opens an empty text buffer in an editor and returns a Result<String> with the contents.
///
/// # Example
/// ```no_run
/// fn main() {
///     let output = match scrawl::new() {
///          Ok(s) => s,
///          Err(e) => e.to_string()
///    };
///    println!("{}", output);
/// }
/// ```
pub fn new() -> Result<String, ScrawlError> {
    let temp_file = create_temp_file()?;
    open_editor(&temp_file).and_then(|output| {
        let _ = fs::remove_file(temp_file);
        Ok(output)
    })
}

/// Open opens a text buffer in an editor with the contents of the file specified. This does _not_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// 
/// fn main() {
///     let path = Path::new("hello.txt");
///     let output = match scrawl::open(path) {
///          Ok(s) => s,
///          Err(e) => e.to_string()
///    };
///    println!("{}", output);
/// }
/// ```
pub fn open(p: &Path) -> Result<String, ScrawlError> {
    let temp_file = create_temp_file()?;

    /* Copy the contents of the file to the temp file */
    fs::copy(p, &temp_file).map_err(|_| {
        let p = p.to_str().unwrap_or("<unknown>");
        ScrawlError::FailedToCopyToTempFile(String::from(p))
    })?;
    
    open_editor(&temp_file).and_then(|output| {
        let _ = fs::remove_file(temp_file);
        Ok(output)
    })
}

/// Edit opens a text buffer in an editor with the contents of the file specified. This _does_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// 
/// fn main() {
///     let path = Path::new("hello.txt");
///     let output = match scrawl::edit(path) {
///          Ok(s) => s,
///          Err(e) => e.to_string()
///    };
///    println!("{}", output);
/// }
/// ```
pub fn edit(p: &Path) -> Result<String, ScrawlError> {
    open_editor(p)
}

/* Attempts to determine which text editor to open the text buffer with. */
fn get_editor_name() -> Result<String, ScrawlError> {
    match var("EDITOR") {
        Ok(s) => Ok(s),
        _ => Err(ScrawlError::EditorNotFound)
    }
}

/* Creates the temporary file */
const PREFIX: &str = "xvrqt_scrawl";
static TEMP_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);
fn create_temp_file() -> Result<PathBuf, ScrawlError> {
    /* Generate unique path to a temporary file buffer */
    let i = TEMP_FILE_COUNT.fetch_add(1, Ordering::SeqCst);
    let process_id = std::process::id();
    let temp_file = format!("{}_{}_{}", PREFIX, process_id, i);

    let mut temp_dir = temp_dir();
    temp_dir.push(temp_file);

    match fs::File::create(&temp_dir) {
        Err(_) => Err(ScrawlError::FailedToCreateTempfile),
        _ => Ok(temp_dir)
    }
}

/* Opens the file in the user's preferred text editor, and returns the contents
   as a String 
*/
fn open_editor(path: &Path) -> Result<String, ScrawlError> {
    let editor_name = get_editor_name()?;
    match Command::new(&editor_name)
        .arg(&path)
        .status() { 
            Ok(status) if status.success() => {
                fs::read_to_string(path).map_err(|_| {
                    ScrawlError::FailedToReadIntoString
                })
            },
            _ => Err(ScrawlError::FailedToOpenEditor(editor_name))
    }
}

