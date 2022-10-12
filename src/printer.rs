use std::fmt;
use std::fmt::Arguments;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Output {
    Stderr,
    Stdout,
}

/// Write to stdout followed by a flush.
pub fn write_to_stdout(args: Arguments<'_>) {
    let mut stdout = std::io::stdout();
    stdout.write_fmt(args).unwrap();
    stdout.flush().unwrap();
}

/// Write to stderr followed by a flush.
pub fn write_to_stderr(args: Arguments<'_>) {
    let mut stderr = std::io::stderr();
    stderr.write_fmt(args).unwrap();
    stderr.flush().unwrap();
}
