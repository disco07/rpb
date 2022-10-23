use std::io::Write;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use crossbeam::channel::{Receiver, Sender};

pub struct MultiBar<T: Write> {
    state: Mutex<State<T>>,
    nbars: AtomicUsize,
    chan: (Sender<WriteMsg>, Receiver<WriteMsg>),
}

struct State<T: Write> {
    lines: Vec<String>,
    nlines: usize,
    handle: T,
}