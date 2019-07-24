# Scrawl

Rust library that opens a user's text editor and returns the results as a string. Can be used to open and edit exisiting files, or just as a scratch space for input. Useful for having a user edit text inline with a CLI program a la `git commit -m`

Built for my new (under development) daily journaling program in Rust: [Echo](https://git.xvrqt.com/xvrqt/echo)

## Functions

### New
Open opens a text buffer in an editor with the contents of the file specified. This does _not_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

```rust
use std::path::Path;

fn main() {
    let path = Path::new("list_of_dogs_I_want_to_pet.txt");
    let output = match scrawl::open(path) {
        Ok(s) => s,
        Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

### Open
Open opens a text buffer in an editor with the contents of the file specified. This does _not_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

```rust
use std::path::Path;

fn main() {
    let path = Path::new("list_of_dogs_I_want_to_pet.txt");
    let output = match scrawl::open(path) {
         Ok(s) => s,
         Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

### Edit
Edit opens a text buffer in an editor with the contents of the file specified. This _does_ edit the contents of the file. Returns a Result<String> with the contents of the buffer.

```rust
use std::path::Path;

fn main() {
    let path = Path::new("list_of_dogs_I_want_to_pet.txt");
    let output = match scrawl::edit(path) {
         Ok(s) => s,
         Err(e) => e.to_string()
    };
    println!("{}", output);
}
```

