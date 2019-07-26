//! # Scrawl 
//! A library for opening a file for editing in a text editor and capturing the result as a String
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

/* Standard Library */
use std::{
    fs,
    env::{temp_dir, var},
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    process::Command
};

/* Internal Modules */
pub mod error;
use error::ScrawlError as ScrawlError;

/// The Editor struct allows setting up the editor before opening it. Useful for setting things like a file extension for syntax highlighting, or specifying a specific editor and more.
#[derive(Debug)]
pub struct Editor {
    /// The name of the command to use instead of $EDITOR
    bin: Option<String>,
    /// The name of the command to use if $EDITOR is not set
    fallback: Option<String>,
    /// Use the contents of specified file to seed the buffer.
    file: Option<PathBuf>,
    /// Use the contents of this String slice to seed the buffer.
    contents: Option<String>,
    /// The extension to set on the file used a temporary buffer. Useful for having the correct syntax highlighting when the editor is opened.
    extension: Option<String>,

    /// Trim the white space off the resulting string. True by default.
    trim: bool,
}

impl Editor {
    /// Returns a new Editor struct with Trim Newlines and Require Save enabled. 
    pub fn new() -> Self {
        Editor {
            bin: None,
            fallback: None,

            file: None,
            contents: None,
            extension: None,

            trim: true,
        }
    }

    /// Sets the name of the editor to open the text buffer. If this editor is not found it will not fallback on the user's default and return an error instead.
    /// # Example
    /// ```no_run
    /// fn main() {
    ///     let output = match scrawl::Editor.new()
    ///                                .executable("vim")
    ///                                .edit() 
    ///     {
    ///          Ok(s) => s,
    ///          Err(e) => e.to_string()
    ///     };
    ///     println!("{}", output);
    /// }
    /// ```
    pub fn executable(&mut self, command: &str) -> &mut Editor {
        self.bin = Some(command.to_string());        
        self
    }

    /// Sets the name of the editor to be used to open the text buffer if the user has not set a default text editor. Scrawl will attempt to discern the default text editor using $VISUAL and $EDITOR environmental variables in that order. If they are not present, it will set it "textpad.exe" for Windows and "vi" otherwise. Setting fallback will cause it to use the specified instead of the previous two.
    ///
    pub fn fallback(&mut self, command: &str) -> &mut Editor {
        self.fallback = Some(command.to_string());        
        self
    }

    /// Fills the text buffer with the contents of the file specified. This does _not_ edit the contents of the file. 
    pub fn file(&mut self, file: &Path) -> &mut Editor {
        self.file = Some(file.to_owned());        
        self
    }

    /// Fills the text buffer with the contents of the specified string. If both file and contents are set contents will take priority. 
    pub fn contents(&mut self, contents: &str) -> &mut Editor {
        self.contents = Some(contents.to_owned());        
        self
    }

    /// Sets whether or not to trim the resulting String of newlines
    pub fn trim(&mut self, trim: bool) -> &mut Editor {
        self.trim = trim;
        self
    }

}

/// Creates a new Editor struct. Used to indicate you're not saving the struct for resuse.
/// # Example
/// ```no_run
/// fn main() {
///     let output = match scrawl::builder()
///                                .executable("vim") {
///          Ok(s) => s,
///          Err(e) => e.to_string()
///    };
///    println!("{}", output);
/// }
/// ```
pub fn builder() -> Editor {
    Editor::new()
}

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

/// New opens an text buffer with the contents of the provided String in an editor. Returns a Result<String> with the edited contents.
///
/// # Example
/// ```no_run
/// fn main() {
///     let output = match scrawl::with("Hello World!") {
///          Ok(s) => s,
///          Err(e) => e.to_string()
///    };
///    println!("{}", output);
/// }
/// ```
pub fn with(content: &str) -> Result<String, ScrawlError> {
    let temp_file = create_temp_file()?;

    fs::write(&temp_file, content).map_err(|_| {
        ScrawlError::FailedToCreateTempfile
    })?;

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

