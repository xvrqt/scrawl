# Scrawl

Rust library that opens a user's text editor and returns the results as a string. Can be used to open and edit exisiting files, or just as a scratch space for input. Useful for having a user edit text inline with a CLI program a la `git commit -m`

Built for my new (under development) daily journaling program in Rust: [Echo](https://git.xvrqt.com/xvrqt/echo)

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

