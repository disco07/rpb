pub enum Themes {
    Basic,
    Small,
    ColoredSmall,
    ColoredMedium,
}

pub struct Styles {
    pub rate: String,
    pub white_space: String,
    pub bar_type: char,
    pub bar_start: char,
    pub bar_end: char,
    pub bar_width: usize,
}

impl Styles {
    pub fn new() -> Styles {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            rate: "".to_string(),
            white_space: " ".to_string(),
            bar_type: '█',
            bar_start: '|',
            bar_end: '|',
            bar_width: 60,
        }
    }
}
