# Scrawl

Rust library that opens a user's text editor and returns the results as a string. Can be used to open and edit exisiting files, or just as a scratch space for input. Useful for having a user edit text inline with a CLI program a la `git commit -m`

![Animated example of how to use the with command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/with.gif)

Built for my new (under development) daily journaling program in Rust: [Echo](https://git.xvrqt.com/xvrqt/echo)

## Quick Start
```rust
use scrawl;

fn main() {
    // Open an empty buffer with the user's preferred text editor
    let output = scrawl::new()?;
    println!("User Input: {}", output);

    // Open a buffer with contents in the text editor
    let output = scrawl::with("Favorite color: ")?;
    println!("{}", output);

    // Open a buffer with text from a file in the text editor
    let output = scrawl::open("survey.txt")?;
    println!("{}", output);

    // Open a file for direct editing in the text editor
    let output = scrawl::edit("README.md")?;
    println!("{}", output);
}
```

## Editor Struct
The Editor struct allows you to set certain options before opening the editor. It also allows you resuse these settings instead of having to build them each time you want to use an editor. Run `edit()` on the struct to open the buffer.

```rust
use scrawl::editor::Editor;

fn main() {
    let editor = Editor::new()
                        .contents("My favorite color is: ")
                        .extension(".txt")
                        .trim(true);

    let fave_color = editor.open().unwrap();

    /* Change the prompt, keep other settings the same */
    editor.contents("My favorite bird is: ");
    let fave_bird = editor.open().unwrap();

    println!("About Me:\n{}\n{}", fave_color, fave_bird);
}
```

If you want to open a one off editor without using settings, see the **Functions** section below.

### Settings

#### Editor
You can set a preferred text editor for the user. Otherwise, $VISUAL, $EDITOR or "textpad.exe"/"vi" is used as a fallback if none is set.
```rust
let output = Editor::new().editor("vim").open()?;
```

#### File
You can set a file from which the text buffer will be seeded. If the file has an extension, this will also set the extension of the temporary buffer. This will _**not**_ modify the file.
```rust
let output = Editor::new().file("my_survey.txt").open()?;
```

#### Contents 
You can use a string to seed the text buffer.
```rust
let output = Editor::new().contents("Favorite Number: ").open()?;
```

#### Extension 
Set the extension of the temporary file created as a buffer. Useful for hinting to text editors which syntax highlighting to use.
```rust
let output = Editor::new().extension(".rs").open()?;
```

#### Trim 
Trim leading and trailing whitespace from the result. Enabled by default.
```rust
let output = Editor::new().trim(false).open()?;
```

#### Edit Directly
If **file** is set, this will open that file for editing (instead of a temporary file) and any changes made will be reflected to that file. Disabled by default.
```rust
let output = Editor::new().file("lib.rs").edit_directly(true).open()?;
```

## Functions
These functions are provided for convenience. Useful for prototyping, or if you don't want to build and maintain a struct just to open an editor.

### New
Open an empty text buffer in the user's preferred editor. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the new command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/new.gif)

```rust
use scrawl;

fn main() {
    let output = scrawl::new(path).unwrap();
    println!("{}", output);
}
```

### With
Open an text buffer with the contents of the String slice in the user's preferred editor. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the with command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/with.gif)

```rust
use scrawl;

fn main() {
    let output = scrawl::with("Hello World!").unwrap();
    println!("{}", output);
}
```

### Open
Open opens a text buffer in an editor with the contents of the file specified. This does _**not**_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the open command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/open.gif)

```rust
use scrawl;

fn main() {
    let output = scrawl::open("hello.txt").unwrap();
    println!("{}", output);
}
```

### Edit
Edit opens a text buffer in an editor with the contents of the file specified. This _**does**_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the edit command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/edit.gif)

```rust
use scrawl;

fn main() {
    let output = scrawl::edit("README.md").unwrap();
    println!("{}", output);
}
```

