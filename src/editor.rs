//! # Editor
//! Struct used to configure the editor before opening and using it.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

/* Standard Library */
use std::{
    env::{temp_dir, var},
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

/* Internal Modules */
use crate::error::ScrawlError;

/* Constants used by the struct to prevent naming collisions of buffer */
const PREFIX: &str = "xvrqt_scrawl";
static TEMP_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Returns a new Editor struct
/// # Example
/// ```no_run
/// # use scrawl::error::ScrawlError;
///
/// # fn main() -> Result<(), ScrawlError> {
///   let editor = scrawl::editor::new();
///   let output = editor.open()?;
///   println!("{}", output);
/// #   Ok(())
/// # }
/// ```
pub fn new() -> Editor<InitialState> {
    Editor {
        editor: get_default_editor_name(),
        unique: InitialState { extension: String::from(".txt") }
    }
}

/* Marker trait that ensures valid state transitions */
/// Marker trait used to ensure valid state transitions.
pub trait EditorState {}

/// The Editor struct allows setting up the editor before opening it. Useful for setting things like a file extension for syntax highlighting, or specifying a specific editor and more.
#[derive(Debug)]
pub struct Editor<S: EditorState> {
    /// The name of the command to use instead of $EDITOR, fallback to the user's default editor
    editor: String,
    /// Captures the state of the Editor and holds additional state information.
    unique: S,
}

/* The initial state of the Editor */
#[derive(Debug)]
/// State machine type marker. Initial state of the editor.
pub struct InitialState { extension: String }
impl EditorState for InitialState {}

impl Editor<InitialState> {
    /// Set the editor to open the buffer in. If not set, uses the user's default editor set by the $EDITOR environment variable.
    pub fn editor(mut self, editor_name: &str) -> Self {
        self.editor = editor_name.to_owned();
        self
    }

    /// Set the extension of the temporary file used as a buffer. Useful for having the text editor hilight syntax appropriately. 
    /// Open an empty buffer
    /// # Example
    /// ```no_run
    /// # use scrawl::error::ScrawlError;
    ///
    /// # fn main() -> Result<(), ScrawlError> {
    ///   let editor = scrawl::editor::new()
    ///                                 .editor("vim")
    ///                                 .extension(".rs");
    ///   let output = editor.open()?;
    ///   println!("{}", output);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn extension(mut self, ext: &str) -> Self {
        self.unique.extension = ext.to_owned();
        self
    }

    /// Open an empty buffer
    /// # Example
    /// ```no_run
    /// # use scrawl::error::ScrawlError;
    ///
    /// # fn main() -> Result<(), ScrawlError> {
    ///   let editor = scrawl::editor::new();
    ///   let output = editor.open()?;
    ///   println!("{}", output);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn open(&self) -> Result<String, ScrawlError> {
        let path = create_temp_file()?;
        open_editor(&self.editor, &path)
    }
        
    /// Use the contents of this file to seed the text buffer.
    pub fn file<F: AsRef<Path>>(self, file: F) -> Editor<FileState> {
        Editor {
            editor: self.editor,
            unique: FileState { 
                path: file.as_ref().to_owned(),
            }
        }
    }

    /// Use the contents of this string to seed the text buffer.
    pub fn contents<S: AsRef<str>>(self, contents: S) -> Editor<ContentState> {
        Editor {
            editor: self.editor,
            unique: ContentState { 
                contents: contents.as_ref().to_owned(),
                extension: self.unique.extension
            }
        }
    }

}

/* Editor that has its contents initialized by the contents of a file */
#[derive(Debug)]
/// State machine type marker. Holds the path of the file that the text buffer will be seeded with.
pub struct FileState { path: PathBuf, }
impl EditorState for FileState {}

impl Editor<FileState> {
    /// Open a buffer seeded with the contents of the file at the path provided.
    pub fn open(&self) -> Result<String, ScrawlError> {
        /* Copy the contents of this file to the temp file */
        let temp_file_path = create_temp_file()?;
        fs::copy(&self.unique.path, &temp_file_path).map_err(|_| {
            let path = self.unique.path.to_string_lossy().into();
            ScrawlError::FailedToCopyToTempFile(path)
        })?;

        open_editor(&self.editor,&temp_file_path)
    }

    /// Edit the file directly.
    pub fn edit(self) -> Editor<EditFileState> {
        Editor {
            editor: self.editor,
            unique: EditFileState { path: self.unique.path, }
        }
    }
}

/* Editor that directly edit the contents of a file */
#[derive(Debug)]
/// State machine type marker. Holds the file that will be edited.
pub struct EditFileState { path: PathBuf }
impl EditorState for EditFileState {}

impl Editor<EditFileState> {
    /// Open a file in the text editor for direct editing.
    pub fn open(&self) -> Result<(), ScrawlError> {
        Command::new(&self.editor)
                    .arg(&self.unique.path)
                    .status()
                    .map(|_| ())
                    .map_err(|_| ScrawlError::FailedToOpenEditor(self.editor.clone()))
    }
}

/* Editor that has contents initialized by a string */
#[derive(Debug)]
/// State machine type marker. Holds the contents of the string that the text buffer will be seeded with.
pub struct ContentState { 
    contents: String,
    extension: String,
}
impl EditorState for ContentState {}

impl Editor<ContentState> {
    /// Open a buffer seeded with the contents of a String in a text editor.
    pub fn open(&self) -> Result<String, ScrawlError> {
        /* Copy the contents of this file to the temp file */
        let temp_file_path = create_temp_file()?;
        fs::write(&temp_file_path, &self.unique.contents).map_err(|_| {
            ScrawlError::FailedToCopyToTempFile("[String]".into())
        })?;

        open_editor(&self.editor, &temp_file_path)
    }
}

/* Utility */

/* Opens the file specified in path in the user's preferred text editor for 
 * editing and returns the contents as a String.
 */
fn open_editor(editor: &str, path: &Path) -> Result<String, ScrawlError> {
    match Command::new(editor).arg(path).status() {
        Ok(status) if status.success() => {
            fs::read_to_string(path)
                .map_err(|_| ScrawlError::FailedToCaptureInput)
        }
        _ => Err(ScrawlError::FailedToOpenEditor(String::from(editor))),
    }
}

/* Used to seed the default editor value */
fn get_default_editor_name() -> String {
    var("VISUAL").or(var("EDITOR")).unwrap_or_else(|_| {
        /* Take a guess based on the system */
        if cfg!(windows) {
            String::from("notepad.exe")
        } else {
            String::from("vi")
        }
    })
}

/* Creates a thread safe, process safe tempfile to use as a buffer */
fn create_temp_file() -> Result<PathBuf, ScrawlError> {
    /* Generate unique path to a temporary file buffer */
    let process_id = std::process::id();
    let i = TEMP_FILE_COUNT.fetch_add(1, Ordering::SeqCst);
    let temp_file = format!("{}_{}_{}", PREFIX, process_id, i);

    /* Push the file to the OS's temp dir */
    let mut temp_dir = temp_dir();
    temp_dir.push(temp_file);

    /* Create the file */
    fs::File::create(&temp_dir).map_err(|_| ScrawlError::FailedToCreateTempfile)?;

    Ok(temp_dir)
}

