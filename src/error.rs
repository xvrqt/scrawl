use std::fmt;

#[derive(Debug)]
pub enum ScrawlError {
    EditorNotFound,
    FailedToCreateTempfile,
    FailedToOpenEditor(String),
    FailedToReadIntoString,
    FailedToCopyToTempFile(String),
    Other(String)
}

/* Display and Debug are required to satisfy the Error trait. Debug has been
   derived for ScrawlError.
*/
impl fmt::Display for ScrawlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match self {
            ScrawlError::EditorNotFound => String::from("Could not determine
            the user's preferred text editor. Make sure the $EDITOR environment
            variable is set."),

            ScrawlError::FailedToCreateTempfile => String::from("Could not
            create a temporary file to serve as a buffer."),

            ScrawlError::FailedToOpenEditor(editor)  => format!("Failed to open
            `{}` as a text editor or editor was terminated with errors.",
            editor),

            ScrawlError::FailedToReadIntoString => String::from("Failed to
            parse file into valid UTF-8 String."),

            ScrawlError::FailedToCopyToTempFile(filename) => format!("Failed to
            copy the contents of the `{}` to the temporary file for editing.",
            filename),

            ScrawlError::Other(string) => String::from(string)
        };

        write!(f, "{}", error)
    }
}

/* Implement From for String types to allow us to coerce thrid party errors into
   ScrawlError
*/
impl From<String> for ScrawlError {
    fn from(error: String) -> Self {
        ScrawlError::Other(error)
    }
}

impl From<&str> for ScrawlError {
    fn from(error: &str) -> Self {
        ScrawlError::Other(error.into())
    }
}

