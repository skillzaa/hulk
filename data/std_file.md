# std::File

### Definition 
---
>A reference to an open file on the filesystem.



An instance of a File can be read and/or written depending on what options it was opened with. Files also implement Seek to alter the logical cursor that the file contains internally.
---

A simple example from rust docs, little improved.
**we do not need the ./ in the begining of the path** 

```rust
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::create("./foo.txt")?;
    Ok(())
}
```  
If a file with the same name is already present there then that file will be over written. **use with caution**
---
If we give it a path in which some folders are non existing then it will just give an error and quit.

```rust
    let f = File::create("./folder/foo.txt")?;
```
---
When the documentation says "path does not exist" it can mean any missing folder or missing file.

One of the basic modules in rust std library is **std::io**. The main meat of io module are **Write** and **Read** traits.

Because they are traits, Read and Write are implemented by a number of other types, and you can implement them for your types too.

Read and Write are so important, implementors of the two traits have a nickname: readers and writers. So you’ll sometimes see ‘a reader’ instead of ‘a type that implements the Read trait’. Much easier!

Beyond that, there are two important traits that are provided: Seek and BufRead. Both of these build on top of a reader to control how the reading happens. Seek lets you control where the next byte is coming from

## std::io::BufReader
Adds buffering to any reader

BufReader<R> can improve the speed of programs that make small and repeated read calls to the same file or network socket. It does not help when reading very large amounts at once, or reading just one or a few times. It also provides no advantage when reading from a source that is already in memory, like a Vec<u8>.

Example

```rust
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f:File = File::open("foo.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);
    Ok(())
}
```

Note these:
- b"..."	Byte string literal; constructs a [u8] instead of a string.
- b'...'	ASCII byte literal
---
[Rust Operators](https://doc.rust-lang.org/book/appendix-02-operators.html)

## Reading a File Line By Line

```rust
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```
