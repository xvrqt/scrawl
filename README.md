# Scrawl

Rust library that opens a user's text editor and returns the results as a string. Can be used to open and edit exisiting files, or just as a scratch space for input. Useful for having a user edit text inline with a CLI program a la `git commit -m`

## Quick Start
```rust
use scrawl;

fn main() {
    // Open an empty buffer with the user's preferred text editor
    let output = scrawl::new()?;
    println!("User Input: {}", output.to_string());

    // Open a buffer with contents in the text editor
    let output = scrawl::with(&"Favorite color: ")?;
    println!("{}", output.to_string());

    // Open a buffer with text from a file in the text editor
    let output = scrawl::from_file(&"survey.txt")?;
    println!("{}", output.to_string());

    // Open a file for direct editing in the text editor
    scrawl::edit_file(&"README.md")?;
}
```

## Editor Struct
The Editor struct allows you to set certain options before opening the editor. Such as: which editor to open, which arguments to pass to that editor, which file extension to use when editing (provides editors with syntax highlight hints).

```rust
use scrawl::{editor, Contents};

fn main() -> Result<(), error::ScrawlError> {
    let output = editor::new()
                        .editor("vim")
                        .args("--clean")
                        .ext(".rs)
                        .open(Contents::FromFile(&"foo.txt"))?;
}
```

