use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter, Display)]
pub enum Spinners {
    Dots,
    Dots2,
    Dots3,
    Dots4,
    Line,
    GrowVertical,
    Bounce,
    Triangle,
    CircleHalves,
    Clock,
    Earth,
    Moon,
    Men,
    Weather,
    Point,
}

fn spinner_to_int(spinner: Spinners) -> usize {
    use Spinners::*;
    match spinner {
        Dots => 0,
        Dots2 => 1,
        Dots3 => 2,
        Dots4 => 3,
        Line => 4,
        GrowVertical => 5,
        Bounce => 6,
        Triangle => 7,
        CircleHalves => 8,
        Clock => 9,
        Earth => 10,
        Moon => 11,
        Men => 12,
        Weather => 13,
        Point => 14,
    }
}

pub fn get_spinner(spinner: Spinners) -> Vec<&'static str> {
    let arr_spinner = [
        vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", ""],
        vec!["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"],
        vec!["⢹", "⢺", "⢼", "⣸", "⣇", "⡧", "⡗", "⡏"],
        vec!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
        vec!["-", "\\", "|", "/"],
        vec!["▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃", "▁"],
        vec!["⠁", "⠂", "⠄", "⠂"],
        vec!["◢", "◣", "◤", "◥"],
        vec!["◐", "◓", "◑", "◒"],
        vec![
            "🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚",
        ],
        vec!["🌍", "🌎", "🌏"],
        vec!["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
        vec!["🚶", "🏃"],
        vec![
            "☀️", "☀️", "☀️", "🌤", "⛅️", "🌥", "☁️", "🌧", "🌨", "🌧", "🌨", "🌧", "🌨", "⛈", "🌨", "🌧", "🌨",
            "☁️", "🌥", "⛅️", "🌤", "☀️", "☀️",
        ],
        vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
    ];
    // arr_spinner.get(spinner as i32).unwrap().to_vec()
    arr_spinner.get(spinner_to_int(spinner)).unwrap().to_vec()
}
