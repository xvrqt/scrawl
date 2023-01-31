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
    borrow::Cow,
    time::{SystemTime, UNIX_EPOCH},
    sync::Once,
    path::{Path, PathBuf},
    process::{Command, Child},
    sync::atomic::{AtomicUsize, Ordering},
    ffi::{OsString, OsStr},
};

/* Internal Modules */
use crate::error::ScrawlError;

/* Used for one time setup */
static SETUP: Once = Once::new();

/* Constants used by the struct to prevent naming collisions of buffer */
const TEMP_DIR: &str = "xvrqt_scrawl";
static TEMP_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

/* Default Values */
static DEFAULT_EXTENSION: &str = ".txt";

/* Program list */
static WINDOWS_PROGRAM_01: &OsStr = OsStr::new("notepad.exe");

static LINUX_PROGRAM_01: &OsStr = OsStr::new("vim");
static LINUX_PROGRAM_02: &OsStr = OsStr::new("neovim");
static LINUX_PROGRAM_03: &OsStr = OsStr::new("nvim");
static LINUX_PROGRAM_04: &OsStr = OsStr::new("nano");
static LINUX_PROGRAM_05: &OsStr = OsStr::new("emacs");
static LINUX_PROGRAM_06: &OsStr = OsStr::new("mcedit");
static LINUX_PROGRAM_07: &OsStr = OsStr::new("tilde");
static LINUX_PROGRAM_08: &OsStr = OsStr::new("micro");
static LINUX_PROGRAM_09: &OsStr = OsStr::new("helix");
static LINUX_PROGRAM_10: &OsStr = OsStr::new("ne");
static LINUX_PROGRAM_11: &OsStr = OsStr::new("vi");


/// The Editor struct allows setting up the editor before opening it. Useful for setting things like a file extension for syntax highlighting, or specifying a specific editor and more.
#[derive(Debug)]
pub struct Editor<'a, S: EditorState> {
    /// The name of the command to use instead of $EDITOR, fallback to the user's default editor.
    // Might make more sense to go back to the ENUM version because then you can try multiple times if the editor isn't specified.
    editor: EditorProgram<'a>,
    /*
    /// What to seed the buffer with (String, or contents of another file).
    buffer: BufferContent,
    */
    /// Captures the state of the Editor and holds additional state information.
    state: S,
}

impl<'a, S: EditorState> Editor<'a, S> {

    /// Returns the name of the editor to use if user specified, or a list of editors to try if Default is selected.
    fn get_editor_programs(&self) -> Vec<Cow<'a, OsStr>> {
        match self.editor {
            /* The specified branch is not intended to be used */
            EditorProgram::UserSpecified(s) => vec![s],
            EditorProgram::Default => {
                let mut programs: Vec<Cow<'a, OsStr>> = Vec::with_capacity(3);
                /* Check the last program that worked */
                // TBD

                /* Check the usual ENV variables for programs */
                if let Ok(p) = var("VISUAL") { programs.push(OsStr::new(&p).into()) };
                if let Ok(p) = var("EDITOR") { programs.push(OsStr::new(&p).into()) };
                
                /* Add some common programs */
                if cfg!(windows) { programs.push(WINDOWS_PROGRAM_01.into()); }
                else {
                    programs.push(LINUX_PROGRAM_01.into());
                    programs.push(LINUX_PROGRAM_02.into());
                    programs.push(LINUX_PROGRAM_03.into());
                    programs.push(LINUX_PROGRAM_04.into());
                    programs.push(LINUX_PROGRAM_05.into());
                    programs.push(LINUX_PROGRAM_06.into());
                    programs.push(LINUX_PROGRAM_07.into());
                    programs.push(LINUX_PROGRAM_08.into());
                    programs.push(LINUX_PROGRAM_09.into());
                    programs.push(LINUX_PROGRAM_10.into());
                    programs.push(LINUX_PROGRAM_11.into());
                }
                programs
            }
        }
    }
}

#[derive(Debug)]
enum EditorProgram<'a> {
    Default,
    UserSpecified(Cow<'a, OsStr>),
}

impl<'a> EditorProgram<'a> {
    /* Returns a list of possible editor programs if the not user specified */
}

/* Strategy for seeding the buffer when the editor is opened */
/*
#[derive(Debug)]
enum BufferContent {
    Empty{extension: String},
    FromString{content: String, extension: String},
    FromFile{path: PathBuf},
}
*/

/* This struct uses a state machine pattern. The struct is generic over types
   that implement "EditorState" which allows the methods to change as the 
   struct is used, and prevents users from calling methods which are no longer
   relevant.

   Builder -> File|Content -> Editing -> Done
*/

/* Marker trait that ensures valid state transitions */
/// Marker trait used to ensure valid state transitions.
pub trait EditorState {}

/* The initial state of the Editor, allows customization to be made */
#[derive(Debug)]
/// State machine type marker. Initial state of the editor.
pub struct BuilderMode<'a> { extension: Cow<'a, str> }
impl<'a> EditorState for BuilderMode<'a> {}

/// Returns a new Editor struct with the default options.
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
pub fn new<'a>() -> Editor<'a, BuilderMode<'a>> {
    Editor {
        editor: EditorProgram::Default,
//      buffer: BufferContent::Empty{extension: String::from(".txt")},
        state: BuilderMode {
            extension: DEFAULT_EXTENSION.into(),
        },
    }
}

/* In BuilderMode the user can set the extension, the buffer, and the editor */
impl<'a> Editor<'a, BuilderMode<'a>> {
    /// Set the editor to open the buffer in. If not set, uses the user's default editor set by the $EDITOR environment variable, falls back to $VISUAL, and then takes some educated guesses before giving up and throwing an error that it cannot find a text editor.
    pub fn editor<S: AsRef<str>>(mut self, editor: S) -> Self {
        self.editor = EditorProgram::UserSpecified(OsStr::new(editor.as_ref()).into());
        self
    }

    /// Args

    /// Set the extension of the temporary file used as a buffer. Useful for having the text editor highlight syntax appropriately.
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
    pub fn extension<S: AsRef<str>>(mut self, ext: S) -> Self {
        self.state.extension = ext.as_ref().into();
        self
    }
/*
    /// Use the contents of this string to seed the text buffer.
    pub fn contents<S: Into<String>>(mut self, content: S) -> Self {
        self.buffer = match self.buffer {
           BufferContent::Empty{extension: ext} => BufferContent::FromString{content: content.into(), extension: ext.into()},
           BufferContent::FromString{content: _, extension: ext} => BufferContent::FromString{content: content.into(), extension: ext.into()},
           BufferContent::FromFile{path: _} => BufferContent::FromString{content: content.into(), extension: EXT.into()},

        };
        self
    }
*/

    /*
    /// Use the contents of this file to seed the text buffer.
    pub fn file<F: Into<PathBuf>>(self, path: F) -> Self {
       self.buffer = BufferContent::FromFile{path: path.into()};
       self
    }
    */

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
    pub fn open(self) -> Result<Editor<'a, EditingMode<'a>>, ScrawlError> {
        /* Create the new Editor object using the buffer */
        let editor = Editor {
            editor: self.editor,
            state: EditingMode {
                extension: self.state.extension,
                buffer: None,
                process: None,

            }
        };
        editor.state.buffer = Some(editor.createBufferFile()?);
        editor.state.process = Some(editor.spawn()?);

        Ok(editor)
    }
}

#[derive(Debug)]
/// State machine type marker; when the editor is open.
pub struct EditingMode<'a> {
    extension: Cow<'a, str>,
    buffer: Option<std::fs::File>,
    process: Option<Child>,
}
impl<'a> EditorState for EditingMode<'a> {}

impl<'a> Editor<'a, EditingMode<'a>> {
    /* Closes the editor without saving */
    fn kill(&mut self) -> () {
        if let Some(c) = self.state.process {
            c.kill();
        }
    }


    /* Opens the user's editor, stores a handle to it in the struct */ 
    fn spawn(&self) -> Result<Child, ScrawlError> {
        match self.editor {
            EditorProgram::UserSpecified(e) => {
                Command::new(e).spawn().map_err(|_| ScrawlError::EditorNotFound(e.into()))
            },
            EditorProgram::Default => {
                let editors = self.get_editor_programs();
                for editor in editors {
                    match Command::new(editor).spawn() {
                        Ok(child) => return Ok(child),
                        Err(_) => (),
                    };
                }
                Err(ScrawlError::EditorNotFound("gay".into()))
            }
        }
    }

    /// Creates a File object based on a temporary file for the Editor to open
    fn createBufferFile (&self) -> Result<std::fs::File, ScrawlError> {
        /* Check create a Scawl directory in the user's tmp/ directory */
        let mut temp_dir = temp_dir();
        temp_dir.push(TEMP_DIR);
        /* Create it if it doesn't already exist */
        fs::metadata(&temp_dir).map_err(|_| fs::create_dir(&temp_dir));
        
        /* Generate unique path to a temporary file buffer, this naming is 
           necessary in case two programs are using Scrawl at the same time, or if
           files aren't cleaned up there is no collisions resulting in leaked info.
        */
        let process_id = std::process::id();
        let i = TEMP_FILE_COUNT.fetch_add(1, Ordering::SeqCst);
        let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(s) => s.as_secs(),
            _ => 0,
        };

        /* e.g. 1674864208_123_17.txt */
        let temp_file = format!("{}_{}_{}{}", ts, process_id, i, self.state.extension);

        /* Create the file */
        temp_dir.push(temp_file);
        fs::File::create(&temp_dir)
            .map_err(|_| ScrawlError::FailedToCreateTempfile(temp_dir))
    }
}


/* Editor that has contents initialized by a string */
/*
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
        fs::write(&temp_file_path, &self.unique.contents)
            .map_err(|_| ScrawlError::FailedToCopyToTempFile("[String]".into()))?;

        open_editor(&self.editor, &temp_file_path)
    }
}
*/

/* Utility */

/* Opens the file specified in path in the user's preferred text editor for
 * editing and returns the contents as a String.
 */
fn open_editor(editor: &str, path: &Path) -> Result<String, ScrawlError> {
    match Command::new(editor).arg(path).status() {
        Ok(status) if status.success() => {
            fs::read_to_string(path).map_err(|_| ScrawlError::FailedToCaptureInput)
        }
        _ => Err(ScrawlError::FailedToOpenEditor(String::from(editor))),
    }
}



