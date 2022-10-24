use std::io::{Stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use crossbeam::channel::{Receiver, Sender, unbounded};
use crate::bar::Bar;

pub struct MultiBar{
    state: Mutex<State>,
    n_bars: AtomicUsize,
    chan: (Sender<WriteMsg>, Receiver<WriteMsg>),
}

struct State {
    lines: Vec<String>,
    n_lines: usize,
    handle: T,
}

impl MultiBar {
    pub fn new() {
        
    }
}

impl MultiBar {
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
    pub fn on(handle: T) -> MultiBar {
        Self {
            state: Mutex::new(State {
                lines: vec![],
                n_lines: 0,
                handle
            }),
            n_bars: Default::default(),
            chan: unbounded()
        }
    }

    pub fn new_bar(&self, total: u64) -> Bar {
        let mut state = self.state.lock().unwrap();

        state.lines.push(String::new());
        state.n_lines += 1;

        self.n_bars.fetch_add(1, Ordering::SeqCst);

        let mut b = Bar::new(
            total,
        );

        b.option.is_multibar = true;
        b.add(0);
        b
    }
}