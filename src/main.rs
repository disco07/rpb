use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::time;

struct Bar {
    state: State,
    options: Option,
    theme: Theme
}

struct State {
    percent: f64,
    current: i64,
    current_graph_rate: isize,
}

struct Option {
    total: i64,
    start_time: time::Time,
}

struct Theme {
    rate: String,
    bar_type: String,
    bar_start: String,
    bar_end: String,
    bar_width: String,

}

fn main() {}