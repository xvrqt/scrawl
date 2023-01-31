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
static TEMP_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Used to customize the editor before opening it, and to handle closing the program and collecting the output at the end.
#[derive(Debug, Clone, Copy)]
pub struct Editor {}

impl Editor {
    /// Creates a new Editor struct, ready for customizing or opening.
    pub fn new() -> Editor {
        Editor {}
    }

    /// Specify which editor should be opened instead of the user's default.
    pub fn editor<S: AsRef<OsStr>>(self, editor: S) -> SpecificEditor {
        SpecificEditor {
            editor: OsString::from(editor.as_ref()),
            args: None,
        }
    }

    /// Opens the user's editor.
    pub fn open(self) -> Result<Reader, Box<dyn Error>> {
        /* Create a temporary file to use as a buffer */
        let path = Editor::create_buffer_file()?;

        /* Open the editor, store a handle to the child process */
        let editor = OsString::from("vim");
        Command::new(editor).arg(&path).status()?;
        Ok(Reader { path })
    }

    /// Creates a temporary file to use a buffer for the user's editor.
    fn create_buffer_file() -> Result<PathBuf, Box<dyn Error>> {
        /* Check create a Scawl directory in the user's tmp/ directory */
        let mut temp_dir = env::temp_dir();
        temp_dir.push(SCRAWL_TEMP_DIR);
        /* Create it if it doesn't already exist */
        match fs::metadata(&temp_dir) {
            Err(_) => {  fs::create_dir(&temp_dir)? },
            _ => (),
        };

        /* Generate unique path to a temporary file */
        let process_id = std::process::id();
        let i = TEMP_FILE_COUNT.fetch_add(1, Ordering::SeqCst);
        let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or_else(|_| 0, |v| v.as_secs());

        /* e.g. 1674864208_123_17.txt */
        let temp_file = format!("{}_{}_{}.txt", ts, process_id, i);
        /* Create the file path & file */
        temp_dir.push(&temp_file);
        fs::File::create(&temp_dir)?;

        /* Return the path */
        let path = PathBuf::from(temp_dir);
        Ok(path)
    }
}

/// A variant of the Editor struct with a specific command and arguments for the text editor instead of the user's defaults. This struct is created when an editor is specified.
#[derive(Debug)]
pub struct SpecificEditor { editor: OsString, args: Option<Vec<OsString>> }

impl SpecificEditor {
    /// Add arguments that you want to be used when the command is run. The first argument is always the file being used as the buffer. Requires that a specific editor has been set.
    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> Self {
        self.args.get_or_insert(vec![]).push(OsString::from(arg.as_ref()));
        self
    }

    /// Opens the user's editor.
    pub fn open(self) -> Result<Reader, Box<dyn Error>> {
        /* Create a temporary file to use as a buffer */
        let path = Editor::create_buffer_file()?;

        /* Open the editor, store a handle to the child process */
        Command::new(self.editor)
            .arg(&path)
            .args(self.args.unwrap_or_default())
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

