use crate::color::Colorizer;
use crate::iterator::BarIter;
use crate::spinner::Spinner;
use crate::styles::{Styles, Themes};
use crate::type_spinner::Spinners;
use crate::{format, type_spinner};
use std::fmt::{Debug, Formatter};
use std::io;
use std::time::Instant;

macro_rules! unit_fmt {
    ($n: ident) => {{
        let kilo_bytes = 1024_f64;
        match $n {
            $n if $n as f64 >= kilo_bytes.powf(4_f64) => {
                format!("{:.*} TB", 2, $n as f64 / kilo_bytes.powf(4_f64))
            }
            $n if $n as f64 >= kilo_bytes.powf(3_f64) => {
                format!("{:.*} GB", 2, $n as f64 / kilo_bytes.powf(3_f64))
            }
            $n if $n as f64 >= kilo_bytes.powf(2_f64) => {
                format!("{:.*} MB", 2, $n as f64 / kilo_bytes.powf(2_f64))
            }
            $n if $n as f64 >= kilo_bytes => format!("{:.*} KB", 2, $n as f64 / kilo_bytes),
            _ => format!("{:.*} B", 0, $n),
        }
    }};
}

#[derive(Clone)]
pub struct Bar {
    desc: String,
    state: State,
    option: Option,
    theme: Styles,
}

#[derive(Clone)]
struct State {
    percent: f64,
    current: i64,
    current_graph_rate: isize,
}

// Output type format, indicate which format wil be used in
// the speed box.
#[derive(Debug, Clone)]
pub enum Units {
    Default,
    Bytes,
}

#[derive(Clone)]
struct Option {
    total: i64,
    unit: Units,
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
            unit: Units::Default,
            start_time: time,
            spinner: Spinner::new(type_spinner::get_spinner(Spinners::Point)),
            front_colored: "".to_string(),
            back_colored: "".to_string(),
            position: 0,
        }
    }
}

impl Debug for Bar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bar").finish()
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

    /// DefaultBytes provides a progressbar to measure byte
    /// throughput with recommended defaults.
    ///
    /// # Example:
    /// ```rust
    /// use rpb::bar::Bar;
    /// let mut bar = Bar::default_bytes(100, "downloading");
    /// ```
    pub fn default_bytes(max_bytes: i64, desc: &str) -> Bar {
        Self {
            desc: desc.to_string(),
            state: State::new(max_bytes),
            option: Option {
                total: max_bytes,
                unit: Units::Bytes,
                start_time: Instant::now(),
                spinner: Spinner::new(type_spinner::get_spinner(Spinners::Point)),
                front_colored: "".to_string(),
                back_colored: "".to_string(),
                position: 0,
            },
            theme: Default::default(),
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
                self.set_spinner(Spinners::Point);
            }
            Themes::ColoredMedium => {
                self.theme.bar_type = '█';
                self.theme.white_space = "█".to_string();
                self.theme.bar_start = '|';
                self.theme.bar_end = '|';
                self.theme.bar_width = 80;
                self.option.front_colored = "#021B79".to_string();
                self.set_spinner(Spinners::Weather);
            }
            Themes::Small => {
                self.theme.bar_type = '━';
                self.theme.white_space = " ".to_string();
                self.theme.bar_start = ' ';
                self.theme.bar_end = ' ';
                self.theme.bar_width = 80;
                self.set_spinner(Spinners::Dots3);
            }
            Themes::ColoredSmall => {
                self.theme.bar_type = '━';
                self.theme.white_space = "━".to_string();
                self.theme.bar_start = ' ';
                self.theme.bar_end = ' ';
                self.theme.bar_width = 80;
                self.option.front_colored = "#0f3443".to_string();
                self.set_spinner(Spinners::Dots4);
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

    /// Sets description of progress bar.
    pub fn set_description(&mut self, desc: &str) {
        self.desc = desc.to_string()
    }

    /// Sets spinner of progress bar.
    pub fn set_spinner(&mut self, spinner: Spinners) {
        self.option.spinner = Spinner::new(type_spinner::get_spinner(spinner))
    }

    /// Increment current value of one.
    pub fn inc(&mut self) {
        self.add(1)
    }

    /// Set units, default is `Units::Default`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rpb::bar::{Bar, Units};
    ///
    /// let n_bytes = 100;
    /// let mut bar = Bar::new(n_bytes);
    /// bar.set_unit(Units::Bytes);
    /// ```
    pub fn set_unit(&mut self, u: Units) {
        self.option.unit = u;
    }

    /// Wraps an [`io::Read`] with the progress bar
    ///
    /// # Example:
    ///
    /// ```rust, no_run
    /// use std::fs::File;
    /// use std::io;
    /// use rpb::bar::Bar;
    /// use rpb::bar::Units;
    ///
    /// fn main () -> io::Result<()> {
    ///     let src = File::open("src.txt")?;
    ///     let mut tgt = File::create("tgt.txt")?;
    ///     let mut bar = Bar::new(src.metadata()?.len() as i64);
    ///     bar.set_unit(Units::Bytes);
    ///     io::copy(&mut bar.reader(src), &mut tgt).unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub fn reader<R: io::Read>(&self, read: R) -> BarIter<R> {
        BarIter {
            it: read,
            bar: self.clone(),
        }
    }

    /// Wraps an [`io::Write`] with the progress bar
    ///
    /// # Example:
    ///
    /// ```rust, no_run
    /// use std::fs::File;
    /// use std::io;
    /// use rpb::bar::Bar;
    /// use rpb::bar::Units;
    ///
    /// fn main () -> io::Result<()> {
    ///     let mut src = File::open("src.txt")?;
    ///     let mut tgt = File::create("tgt.txt")?;
    ///     let mut bar = Bar::new(src.metadata()?.len() as i64);
    ///     bar.set_unit(Units::Bytes);
    ///     io::copy(&mut src, &mut bar.writer(tgt)).unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub fn writer<R: io::Write>(&self, write: R) -> BarIter<R> {
        BarIter {
            it: write,
            bar: self.clone(),
        }
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
        let mut units = Vec::new();
        let mut current = Vec::new();
        let mut total = Vec::new();

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

        match self.option.unit {
            Units::Default => {
                let curr = self.state.current;
                let tot = self.option.total;
                units.push(format!("{:.*}it/s", 2, it_per_s));
                current.push(format!("{:.*}", 2, curr));
                total.push(format!("{:.*}", 2, tot));
            }
            Units::Bytes => {
                let curr = self.state.current;
                let tot = self.option.total;
                units.push(format!("{}/s", unit_fmt!(it_per_s)));
                current.push(format!("{}", unit_fmt!(curr)));
                total.push(format!("{}", unit_fmt!(tot)));
            }
        };

        let it = format!("[{}-{}, {}, {}/{}]",
                         format::convert(time_elapsed),
                         format::convert(remaining_time),
                         units.into_iter().map(|x| x).collect::<String>(),
                         current.into_iter().map(|x| x).collect::<String>(),
                         total.into_iter().map(|x| x).collect::<String>(),);

        let clear = " ".repeat(10);

        format!(
            "{}{} {}  {}{}",
            background,
            self.theme.bar_end.to_string().as_str(),
            self.option
                .spinner
                .spinning_cursor(self.state.current as usize),
            it,
            clear,
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
