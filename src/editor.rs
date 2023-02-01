//! # Editor
//! Struct used to configure the editor before opening and using it.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

/* Standard Library */
use std::{
    fs,
    env,
    io::BufReader,
    ops::Drop,
    error::Error,
    process::Command,
    ffi::{OsStr, OsString},
    path::{PathBuf, Path},
    time::{SystemTime, UNIX_EPOCH},
    sync::atomic::{AtomicUsize, Ordering},
};

/* Trait to keep things DRY */
trait ScrawlState {}

/* Constants */
const SCRAWL_TEMP_DIR: &str = "xvrqt_scrawl";
const DEFAULT_EXT: &str = ".txt";
static TEMP_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

/* The struct used to construct an Editor */
#[derive(Debug)]
/// This is the struct that allows the caller to customize which editor is called, what it is seeded with, and more.
pub struct Editor<S: EditorState> { extension: String, state: S }

/* Function that returns the default Editor state */
/// Creates a new Editor struct, ready for customizing or opening.
pub fn new() -> Editor<DefaultState> {
    Editor {
        extension: String::from(DEFAULT_EXT),
        state: DefaultState {}
    }
}

/* Trait that is used to keep track which state the Editor is in */
/// Used to keep track of which state the Editor struct is in while it's constructed.
pub trait EditorState {}

/* These function are available to all states of the Editor. Utility functions */
impl<S: EditorState> Editor<S> {

    /// Creates a temporary file to use a buffer for the user's editor.
    fn create_buffer_file(&mut self) -> Result<PathBuf, Box<dyn Error>> {
        /* Check create a Scawl directory in the user's tmp/ directory */
        let mut temp_dir = env::temp_dir();
        temp_dir.push(SCRAWL_TEMP_DIR);
        /* Create it if it doesn't already exist */
        match fs::metadata(&temp_dir) {
            Err(_) => {  fs::create_dir(&temp_dir)? },
            _ => (),
        };

        /* Generate unique path to a temporary file */
        let i = TEMP_FILE_COUNT.fetch_add(1, Ordering::SeqCst);
        let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or_else(|_| 0, |v| v.as_secs());
        let ext = &self.extension;
        let process_id = std::process::id();
        /* e.g. 1674864208_123_17.txt */
        let temp_file = format!("{}_{}_{}{}", ts, process_id, i, ext);

        /* Create the file path & file */
        temp_dir.push(&temp_file);
        fs::File::create(&temp_dir)?;

        /* Return the path */
        let path = PathBuf::from(temp_dir);
        Ok(path)
    }

    /// Returns the name of the editor to use if user specified, or a list of editors to try if Default is selected.
    fn get_editor_programs(&self) -> Vec<OsString> {
        let mut programs = Vec::with_capacity(3);
        /* Check the usual ENV variables for programs */
        if let Ok(p) = env::var("VISUAL") { programs.push(OsString::from(p)) };
        if let Ok(p) = env::var("EDITOR") { programs.push(OsString::from(p)) };
        
        /* Add some common programs */
        if cfg!(windows) { programs.push("notepad.exe".into()); }
        else {
            let p: Vec<OsString> = vec!["vim".into(), "neovim".into(),
                "nvim".into(), "nano".into(), "emacs".into(), "mcedit".into(),
                "tilde".into(), "micro".into(), "helix".into(), "ne".into(),
                "vi".into()];
            programs.extend_from_slice(&p);
        }
        programs
    }
}

/* The default EditorState, in builder mode */
#[derive(Debug, Clone, Copy)]
/// Holds the data and implementation for the initial state of the Editor struct.
pub struct DefaultState {}
impl EditorState for DefaultState {}

impl Editor<DefaultState> {
    /// Specify which extension should be used on the temporary file (often used by text editors to infer syntax highlighting).
    pub fn ext<S: AsRef<str>>(&mut self, ext: S) -> &mut Self {
        self.extension = ext.as_ref().into();
        self
    }
    
    /* Returns a different struct, consumes the Editor instead of returning a
       reference; enforces a certain builder grammar.
    */
    /// Specify which editor should be opened instead of the user's default.
    pub fn editor<S: AsRef<OsStr>>(self, editor: S) -> Editor<SpecificEditorState> {
        Editor {
            extension: self.extension,
            state: SpecificEditorState {
                editor: OsString::from(editor.as_ref()),
                args: None,
            }
        }
    }

    /// Opens the user's editor.
    pub fn open(mut self) -> Result<Reader, Box<dyn Error>> {
        /* Create a temporary file to use as a buffer */
        let path = self.create_buffer_file()?;

        self.get_editor_programs().iter().find(|e| {
            Command::new(e).arg(&path).status().is_ok()
        }).ok_or("Could not find a text editing program")?;

        Ok(Reader { path })
    }
}


/// A variant of the Editor struct with a specific command and arguments for the text editor instead of the user's defaults. This struct is created when an editor is specified.
#[derive(Debug)]
pub struct SpecificEditorState { 
    args: Option<Vec<OsString>>,
    editor: OsString, 
}
impl EditorState for SpecificEditorState {}

impl Editor<SpecificEditorState> {
    /// Add arguments that you want to be used when the command is run. The first argument is always the file being used as the buffer. Requires that a specific editor has been set.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.state.args.get_or_insert(vec![]).push(OsString::from(arg.as_ref()));
        self
    }

    /// Specify which extension should be used on the temporary file (often used by text editors to infer syntax highlighting).
    pub fn ext<S: AsRef<str>>(&mut self, ext: S) -> &mut Self {
        self.extension = ext.as_ref().into();
        self
    }

    /// Opens the user's editor.
    pub fn open(&mut self) -> Result<Reader, Box<dyn Error>> {
        /* Create a temporary file to use as a buffer */
        let path = self.create_buffer_file()?;

        /* Open the editor, store a handle to the child process */
        Command::new(&self.state.editor)
            .arg(&path)
            .args(self.state.args.as_ref().unwrap_or(&vec![]))
            .status()?;

        Ok(Reader { path })
    }
}

/// After the user closes their editor, it transforms into a Reader object where the input can be retrieved.
#[derive(Debug)]
pub struct Reader {
    path: PathBuf,
}

impl Reader {
    /// Read to a vector of bytes.
    pub fn read(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fs::read(&self.path)?)
    }

    /// Returns the buffer as a String.
    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(&self.path)?)
    }

    /// Returns the buffer as a BufReader.
    pub fn to_bufreader(&self) -> Result<BufReader<fs::File>, Box<dyn Error>> {
        Ok(BufReader::new(fs::File::open(&self.path)?))
    }

    /// Saves the contents to a file at the specified path.
    pub fn to_file<P: AsRef<Path>>(&self, path: &P) -> Result<u64, Box<dyn Error>> {
        Ok(fs::copy(&self.path, path.as_ref())?)
    }
}

/* Delete our temporary file to clean up */
impl Drop for Reader {
    fn drop(&mut self) -> () {
        let _ = fs::remove_file(&self.path);
    }
}

