# Scrawl

Rust library that opens a user's text editor and returns the results as a string. Can be used to open and edit exisiting files, or just as a scratch space for input. Useful for having a user edit text inline with a CLI program a la `git commit -m`

Built for my new (under development) daily journaling program in Rust: [Echo](https://git.xvrqt.com/xvrqt/echo)

## Editor Struct
The Editor struct allows you to set certain options before opening the editor. It also allows you resuse these settings instead of having to build them each time you want to use an editor. Run `edit()` on the struct to open the buffer.

```rust
use scrawl::Editor;

fn main() {
    let editor = Editor::new()
                        .contents("My favorite color is: ")
                        .extension(".txt")
                        .trim(true);

    let fave_color = editor.edit().unwrap();

    /* Change the prompt, keep other settings the same */
    editor.contents("My favorite bird is: ");
    let fave_bird = editor.edit().unwrap();

    println!("About Me:\n{}\n{}", fave_color, fave_bird);
}
```

If you want to open a one off editor with settings, see the **Builder** section below.

If you want to open a one off editor without using settings, see the **Functions** section below.

### Settings

#### Editor
You can set a preferred text editor for the user. Otherwise, $VISUAL, $EDITOR or "textpad.exe" or "vi" is used as a fallback if none is set.
```rust
let output = Editor::new().editor("vim").edit()?;
```

#### File
You can set a file from which the text buffer will be seeded. This will not modify the file.
```rust
let output = Editor::new().file("my_survey.txt").edit()?;
```

#### Contents 
You can use a string to seed the text buffer.
```rust
let output = Editor::new().contents("Favorite Number: ").edit()?;
```

#### Extension 
Set the extension of the temporary file created as a buffer. Useful for hinting to text editors which syntax highlighting to use.
```rust
let output = Editor::new().extension(".rs").edit()?;
```

#### Trim 
Trim leading and trailing whitespace from the result. Enabled by default.
```rust
let output = Editor::new().trim(false).edit()?;
```

#### Edit Directly
If file is set, this will open that file for editing (instead of a temporary file) and any changes made will be reflected to that file. Disabled by default.
```rust
let output = Editor::new().file("lib.rs").edit_directly(true).edit()?;
```

## Functions
For all of these functions the user must have their $EDITOR environmental variable set (or you'll get an error telling you it is not set).

### New
Open an empty text buffer in the user's preferred editor. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the new command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/new.gif)

```rust
use scrawl;

fn main() {
    let output = match scrawl::new(path) {
        Ok(s) => s,
        Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

### With
Open an text buffer with the contents of the String slice in the user's preferred editor. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the with command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/with.gif)

```rust
use scrawl;

fn main() {
    let output = match scrawl::with("Hello World!") {
        Ok(s) => s,
        Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

### Open
Open opens a text buffer in an editor with the contents of the file specified. This does _not_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the open command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/open.gif)

```rust
use scrawl;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let output = match scrawl::open(path) {
         Ok(s) => s,
         Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

### Edit
Edit opens a text buffer in an editor with the contents of the file specified. This _does_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

![Animated example of how to use the edit command](https://xvrqt.sfo2.digitaloceanspaces.com/image-cache/edit.gif)

```rust
use scrawl;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let output = match scrawl::edit(path) {
         Ok(s) => s,
         Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

