use crate::color::Colorizer;
use crate::spinner::Spinner;
use crate::styles::{Styles, Themes};
use crate::type_spinner::Spinners;
use crate::{format, type_spinner};
use std::time::Instant;

pub struct Bar {
    desc: String,
    state: State,
    option: Option,
    theme: Styles,
}

struct State {
    percent: f64,
    current: i64,
    current_graph_rate: isize,
}

#[allow(dead_code)]
struct Option {
    total: i64,
    unit: String,
    start_time: Instant,
    spinner: Spinner,
    front_colored: String,
    back_colored: String,
    position: u32,
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

impl Option {
    fn new(total: i64, time: Instant) -> Option {
        Self {
            total,
            unit: "it".to_string(),
            start_time: time,
            spinner: Spinner::new(type_spinner::get_spinner(Spinners::Earth)),
            front_colored: "".to_string(),
            back_colored: "".to_string(),
            position: 0,
        }
    }
}

/// Core implementation of console progress bar.
///
/// # Example
///
/// A simple progress bar with a max value.
///
/// ```rust
///
/// fn main() {
///     use rpb::bar::Bar;
///     let mut bar = Bar::new(100);
///
///     for _ in 0..100 {
///         bar.add(1);
///     }
/// }
/// ```
impl Bar {
    /// Create a new instance of [Bar] with a max value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rpb::bar::Bar;
    /// let mut bar = Bar::new(100);
    /// ```
    pub fn new(max: i64) -> Self {
        Self {
            desc: "".to_string(),
            state: State::new(max),
            option: Option::new(max, Instant::now()),
            theme: Styles::new(),
        }
    }

    /// Instantiate a new theme.
    ///
    ///  # Example
    ///
    /// ```rust
    /// use rpb::bar::Bar;
    /// use rpb::styles::Themes;
    /// let mut bar = Bar::new(100);
    /// bar.set_theme(Themes::Small);
    /// ```
    pub fn set_theme(&mut self, theme: Themes) {
        match theme {
            Themes::Basic => {
                self.theme.bar_type = '█';
                self.theme.white_space = " ".to_string();
                self.theme.bar_start = '|';
                self.theme.bar_end = '|';
                self.theme.bar_width = 80;
            }
            Themes::ColoredMedium => {
                self.theme.bar_type = '█';
                self.theme.white_space = "█".to_string();
                self.theme.bar_start = '|';
                self.theme.bar_end = '|';
                self.theme.bar_width = 80;
                self.option.front_colored = "#6f0000".to_string();
                //self.option.back_colored = "#000000".to_string();
            }
            Themes::Small => {
                self.theme.bar_type = '━';
                self.theme.white_space = " ".to_string();
                self.theme.bar_start = ' ';
                self.theme.bar_end = ' ';
                self.theme.bar_width = 80;
            }
            Themes::ColoredSmall => {
                self.theme.bar_type = '━';
                self.theme.white_space = "━".to_string();
                self.theme.bar_start = ' ';
                self.theme.bar_end = ' ';
                self.theme.bar_width = 80;
                self.option.front_colored = "#0f3443".to_string();
                //self.option.back_colored = "#000000".to_string();
            }
        }
    }

    /// Sets the position of the progress bar.
    /// By default, the progress bar are position 0.
    ///
    ///  # Example
    ///
    /// ```rust
    /// use rpb::bar::Bar;
    /// let mut bar = Bar::new(100);
    /// bar.set_position(1)
    /// ```
    pub fn set_position(&mut self, position: u32) {
        self.option.position = position
    }

    fn render_left_bar(&mut self) -> String {
        self.state.percent = get_percent(&self.state.current, &self.option.total);

        let desc_spacing = if self.desc == "" { "" } else { ": " };
        let mut spacing = if self.state.percent >= 10.0 {
            " "
        } else {
            "  "
        };

        if self.state.percent >= 100.0 {
            spacing = "";
        }

        format!(
            "{}{}{}{}%{}",
            self.desc, desc_spacing, spacing, self.state.percent as u64, self.theme.bar_start
        )
    }

    fn render_right_bar(&mut self) -> String {
        let mut white_space = self.theme.bar_width;
        if self.state.current >= 1 {
            white_space -= self.state.current_graph_rate as usize;
        }
        let time_elapsed = self.option.start_time.elapsed().as_secs();
        let remaining_time = time_elapsed * (self.option.total - self.state.current) as u64
            / self.state.current as u64;
        let mut it_per_s: u64 = 0;
        if time_elapsed >= 1 {
            it_per_s = (self.state.current as u64) / time_elapsed;
        }
        let background: String;
        if self.option.back_colored != "" {
            background = self
                .theme
                .white_space
                .repeat(white_space)
                .colorize(self.option.back_colored.as_str());
        } else {
            background = self.theme.white_space.repeat(white_space);
        }

        format!(
            "{}{} {} [{}-{}, {} {}/s, {}/{}]",
            background,
            self.theme.bar_end.to_string().as_str(),
            self.option
                .spinner
                .spinning_cursor(self.state.current as usize),
            format::convert(time_elapsed),
            format::convert(remaining_time),
            it_per_s,
            self.option.unit,
            self.state.current,
            self.option.total
        )
    }

    fn render_middle_bar(&mut self) -> String {
        self.state.percent = get_percent(&self.state.current, &self.option.total);
        self.state.current_graph_rate =
            (self.state.percent / 100.0 * (self.theme.bar_width as f64)).round() as isize;

        let n: usize = (self.state.current_graph_rate) as usize;
        if self.option.front_colored != "" {
            self.theme.rate = format!("{}", self.theme.bar_type)
                .repeat(n)
                .colorize(self.option.front_colored.as_str());
        } else {
            self.theme.rate = format!("{}", self.theme.bar_type).repeat(n);
        }

        format!("{}", self.theme.rate)
    }

    fn render(&mut self) -> (String, String, String) {
        let lbar = self.render_left_bar();
        let mbar = self.render_middle_bar();
        let rbar = self.render_right_bar();

        return (lbar, mbar, rbar);
    }

    fn print_bar(&self, string: String) {
        if self.option.position == 0 {
            eprint!("{}", format_args!("\r{}", string));
        } else {
            eprint!(
                "{}",
                format_args!(
                    "{}{}{}",
                    "\n".repeat(self.option.position as usize),
                    string,
                    format!("\x1B[{}A", self.option.position)
                )
            );
        }
    }

    /// Manually update the progress bar, advances the position by `num`.
    pub fn add(&mut self, num: usize) {
        assert!(self.option.total > 0, "the max must be greater than zero");
        self.state.current += num as i64;
        assert!(
            self.state.current <= self.option.total,
            "current exceeds total"
        );
        let (lbar, mbar, rbar) = self.render();
        self.print_bar(format!("{}{}{}", lbar, mbar, rbar))
    }
}

fn get_percent(current: &i64, total: &i64) -> f64 {
    100.0 * (*current as f64) / (*total as f64)
}
