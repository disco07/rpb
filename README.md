[![Rust](https://github.com/disco07/rpb/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/disco07/rpb/actions/workflows/rust.yml)
# rpb
A simple progress bar for Rust projects. I created that because there was a long processes in some project and I didn't know what was the progression.

## Installation
Add from command line.
```
cargo add rpb@0.1.1
```
Or add this to your Cargo.toml file.
```
[dependencies]
rpb = "0.1.1"

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
    
    let mut bar = Bar::new(100);

    for _i in 0..100 {
        bar.add(1);
        sleep(time::Duration::from_millis(100))
    }
}
```
![Basic bar](examples/basic/progressbar.gif)

## Contributing 🤝
Contributions, issues, and feature requests are welcome!

Feel free to check the issues page.

## 📝 License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
