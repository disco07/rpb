[![Rust](https://github.com/disco07/rpb/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/disco07/rpb/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/rpb.svg)](https://crates.io/crates/rpb)
[![Documentation](https://docs.rs/rpb/badge.svg)](https://docs.rs/rpb/)
# rpb
A simple progress bar for Rust 🦀 projects. I created that because there was a long processes in some project and I didn't know what was the progression.

## Installation
Add from command line.
```
cargo add rpb@0.1.5
```
Or add this to your Cargo.toml file.
```
[dependencies]
rpb = "0.1.5"

# Or add from github main branch.
rpb = { git = "https://github.com/disco07/rpb.git", branch = "main" }

```

## Usage
### Basic usage
```rust
fn main() {
    use rpb::bar::Bar;
    use std::thread::sleep;
    use std::time;
    
    let mut bar = Bar::new(200);

    for _i in 0..200 {
        bar.add(1);
        sleep(time::Duration::from_millis(50))
    }
}
```
![Basic bar](images/basic.gif)
[examples/custom.rs](examples/custom.rs)
![Custom bar](images/custom.gif)

### I/O operations
The `rpb` implements an `io writer` and `io reader` so it can automatically detect the number of bytes written to a stream.
```rust
use rpb::bar::Bar;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let source = File::open("data.sql")?;
    let mut target = File::create("src.sql")?;
    let bar = Bar::default_bytes(source.metadata()?.len(), "downloading");
    io::copy(&mut bar.reader(source), &mut target).unwrap();
    Ok(())
}
```
![Custom bar](images/download.gif)

## Contributing 🤝
Contributions, issues, and feature requests are welcome!

Feel free to check the issues page.

## 📝 License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
