use std::thread::sleep;
use std::time::{Duration, Instant};

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
    start_time: Instant,
}

struct Theme {
    rate: String,
    bar_type: char,
    bar_start: char,
    bar_end: char,
    bar_width: usize,
}

impl State {
    fn new(max: i64) -> State {
        Self {
            current: 0,
            percent: get_percent(&0, &max),
            current_graph_rate: 0,
        }
    }
}

impl Theme {
    fn new(bar_type: char, bar_start: char, bar_end: char, bar_width: usize) -> Theme {
        Self {
            rate: "".to_string(),
            bar_type,
            bar_start,
            bar_end,
            bar_width,
        }
    }
}

impl Option {
    fn new(total: i64, time: Instant) -> Option {
        Self {
            total,
            start_time: time,
        }
    }
}

impl Bar {
    fn new(max: i64) -> Self {
        Self {
            state: State::new(max),
            option: Option::new(max, Instant::now()),
            theme: Theme::new('█', '[', ']', 50),
        }
    }

    fn render(&mut self) {
        let last = self.state.percent;
        self.state.percent = get_percent(&self.state.current, &self.option.total);
        self.state.current_graph_rate = (self.state.percent / 100.0 * (self.theme.bar_width as f64)) as isize;
        if self.state.percent != last {
            let n: usize = (self.state.current_graph_rate) as usize;
            self.theme.rate = format!("{}", self.theme.bar_type).repeat(n);
        }
        print!("\r|{}|", self.theme.rate);
    }

    fn add(&mut self, num: isize) {
        assert!(self.option.total > 0, "the max must be greater than zero");
        self.state.current += num as i64;
        assert!(self.state.current <= self.option.total, "current exceeds total");
        self.render()
    }
}

fn get_percent(current: &i64, total: &i64) -> f64 {
    100.0 * (*current as f64) / (*total as f64)
}

fn main() {
    let mut bar = Bar::new(10);

    for i in 0..10 {
        bar.add(1);
        sleep(Duration::from_millis(100))
    }
}