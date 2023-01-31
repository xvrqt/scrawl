//! # Scrawl Error Types
//! Error enum used by the Scrawl crate.
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

/// Error enum for the Scrawl crate
#[derive(Debug)]
pub enum ScrawlError {
    /// Could not create a new temporary file to use as a buffer for Scrawl.
    FailedToCreateTempfile(PathBuf),
    /// Could not open the editor, or the editor quit with an error.
    FailedToOpenEditor(String),
    /// Could not read the file into a valid UTF-8 String.
    FailedToCaptureInput,
    /// Could not open the file specified in the scrawl::open function.
    FailedToCopyToTempFile(String),
    EditorNotFound(std::ffi::OsString),
}

/* Display and Debug are required to satisfy the Error trait. Debug has been
   derived for ScrawlError.
*/
impl fmt::Display for ScrawlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match self {
            ScrawlError::FailedToCreateTempfile(path) => format!(
                "Could not \
                 create a temporary file to serve as a buffer for the editor. \
                 Path: \"{}\" not writeable.", path.to_string_lossy()
            ),

            ScrawlError::FailedToOpenEditor(editor) => format!(
                "Failed to \
                 open `{}` as a text editor or editor was terminated with errors.",
                editor
            ),

            ScrawlError::FailedToCaptureInput => String::from(
                "Failed to \
                 capture input. Was not a valid UTF-8 String.",
            ),

            ScrawlError::FailedToCopyToTempFile(filename) => format!(
                "Failed \
                 to copy the contents of the `{}` to the buffer for editing.",
                filename
            ),

            ScrawlError::EditorNotFound(editor) => format!(
                "Could not open the specified editor: {}", editor.to_string_lossy()
            ),
        };

        write!(f, "{}", error)
    }
}

/* Cowards way out */
impl Error for ScrawlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
