use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Bar {
    desc: String,
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
            desc: "".to_string(),
            state: State::new(max),
            option: Option::new(max, Instant::now()),
            theme: Theme::new('█', '[', ']', 50),
        }
    }

    fn render_left_bar(&mut self) -> String {
        self.state.percent = get_percent(&self.state.current, &self.option.total);

        let desc_spacing = if self.desc == "" { "" } else { ": " };
        let mut spacing = if self.state.percent >= 10.0 { " " } else { "  " };

        if self.state.percent >= 1.0 {
            spacing = "";
        }

        return format!("{}{}{}{}%", self.desc, desc_spacing, spacing, self.state.percent as u64);
    }

    fn render_right_bar(&mut self) -> String {
        return format!(
            " {}/{}",
            self.state.current, self.option.total
        );
    }

    fn render_middle_bar(&mut self) -> String {
        self.state.percent = get_percent(&self.state.current, &self.option.total);
        self.state.current_graph_rate = (self.state.percent / 100.0 * (self.theme.bar_width as f64)) as isize;

        let n: usize = (self.state.current_graph_rate) as usize;
        self.theme.rate = format!("{}", self.theme.bar_type).repeat(n);

        return format!("[{}]", self.theme.rate);
    }

    fn render(&mut self) -> (String, String, String) {
        let lbar = self.render_left_bar();
        let rbar = self.render_right_bar();

        let mbar = self.render_middle_bar();

        return (lbar, mbar, rbar);
    }

    fn print_bar(&mut self, string: String) {
        let mut stdout = std::io::stdout();
        stdout.write_fmt(format_args!("{}", string)).unwrap();
        stdout.flush().unwrap();
    }

    fn add(&mut self, num: isize) {
        assert!(self.option.total > 0, "the max must be greater than zero");
        self.state.current += num as i64;
        assert!(self.state.current <= self.option.total, "current exceeds total");
        let (lbar, mbar, rbar) = self.render();
        self.print_bar(format!("\r{}{}{}", lbar, mbar, rbar))
    }
}

fn get_percent(current: &i64, total: &i64) -> f64 {
    100.0 * (*current as f64) / (*total as f64)
}

fn main() {
    let mut bar = Bar::new(10);

    for i in 0..10 {
        bar.add(1);
        sleep(Duration::from_millis(1000))
    }
}