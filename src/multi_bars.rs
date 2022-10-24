use crate::bar::Bar;
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::io::{Stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub struct MultiBar<T: Write> {
    state: Mutex<State<T>>,
    n_bars: AtomicUsize,
    channel: (Sender<WriteMsg>, Receiver<WriteMsg>),
}

struct State<T: Write> {
    lines: Vec<String>,
    n_lines: usize,
    handle: T,
}

// WriteMsg is the message format used to communicate
// between MultiBar and its bars
struct WriteMsg {
    done: bool,
    level: usize,
    string: String,
}

impl MultiBar<Stdout> {
    pub fn new() -> MultiBar<Stdout> {
        MultiBar::on(std::io::Stdout)
    }
}

impl<T> MultiBar<T> where T: Write {
    /// Create a new MultiBar with an arbitrary writer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::io::stderr;
    /// use rpb::multi_bars::MultiBar;
    ///
    /// let mut mb = MultiBar::on(stderr());
    /// ```
    pub fn on(handle: T) -> MultiBar<T> {
        Self {
            state: Mutex::new(State {
                lines: vec![],
                n_lines: 0,
                handle,
            }),
            n_bars: Default::default(),
            channel: unbounded(),
        }
    }

    pub fn new_bar(&self, total: u64) -> Bar {
        let mut state = self.state.lock().unwrap();

        state.lines.push(String::new());
        state.n_lines += 1;

        self.n_bars.fetch_add(1, Ordering::SeqCst);

        let mut b = Bar::new(total);

        b.option.is_multibar = true;
        b.add(0);
        b
    }
}
