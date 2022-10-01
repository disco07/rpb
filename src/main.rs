use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::{io, time};
use std::fmt::Error;
use std::time::Instant;

struct Bar {
    state: State,
    option: Option,
    theme: Theme,
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
    bar_type: char,
    bar_start: char,
    bar_end: char,
    bar_width: isize,
}

impl State {
    fn new(max: i64) -> State {
        Self {
            current: 0,
            percent: get_percent(0, max),
            current_graph_rate: 0
        }
    }
}

impl Theme {
    fn new(bar_type: char, bar_start: char, bar_end: char, bar_width: isize,) -> Theme {
        Self {
            rate: "".to_string(),
            bar_type,
            bar_start,
            bar_end,
            bar_width
        }
    }
}

impl Option {
    fn new(total: i64, time: time) -> Option {
        Self {
            total,
            start_time: time
        }
    }
}

impl Bar {
    fn new(max: i64) -> Self {
        Self {
            state: State::new(max),
            option: Option::new(max, Instant::now()),
            theme: Theme::new('█', '[', ']', 50)
        }
    }

    fn add(&self, num: isize) -> io::Error {
        if &self.option.total == 0 {
            Error::from("the max must be greater than zero")
        }
    }
}

fn get_percent(current: i64, total: i64) -> f64 {
    100 * (current as f64)/(total as f64)
}

fn main() {}