
pub fn get_spinner(spinner: Spinner) -> Vec<&str>{
    Spinners::Dots => SpinnerFrames {frames: vec![
        "⠋",
        "⠙",
        "⠹",
        "⠸",
        "⠼",
        "⠴",
        "⠦",
        "⠧",
        "⠇",
        "⠏"
    ], interval: 80},
    Spinners::Dots2 => SpinnerFrames {frames: vec![
        "⠋",
        "⠙",
        "⠚",
        "⠞",
        "⠖",
        "⠦",
        "⠴",
        "⠲",
        "⠳",
        "⠓"
    ], interval: 80},
    Spinners::Dots3 => SpinnerFrames {frames: vec![
        "⢹",
        "⢺",
        "⢼",
        "⣸",
        "⣇",
        "⡧",
        "⡗",
        "⡏"
    ], interval: 80},
    Spinners::Dots4 => SpinnerFrames {frames: vec![
        "⢄",
        "⢂",
        "⢁",
        "⡁",
        "⡈",
        "⡐",
        "⡠"
    ], interval: 80},
    Spinners::Line => SpinnerFrames {frames: vec![
        "-",
        "\\",
        "|",
        "/"
    ], interval: 130},
    Spinners::GrowVertical => SpinnerFrames {frames: vec![
        "▁",
        "▃",
        "▄",
        "▅",
        "▆",
        "▇",
        "▆",
        "▅",
        "▄",
        "▃",
        "▁",
    ], interval: 120},
    Spinners::Bounce => SpinnerFrames {frames: vec![
        "⠁",
        "⠂",
        "⠄",
        "⠂"
    ], interval: 120},
    Spinners::Triangle => SpinnerFrames {frames: vec![
        "◢",
        "◣",
        "◤",
        "◥"
    ], interval: 50},
    Spinners::CircleHalves => SpinnerFrames {frames: vec![
        "◐",
        "◓",
        "◑",
        "◒"
    ], interval: 50},
    Spinners::Arrow => SpinnerFrames {frames: vec![
        "←",
        "↖",
        "↑",
        "↗",
        "→",
        "↘",
        "↓",
        "↙"
    ], interval: 100},
    Spinners::Clock => SpinnerFrames {frames: vec![
        "🕛",
        "🕐",
        "🕑",
        "🕒",
        "🕓",
        "🕔",
        "🕕",
        "🕖",
        "🕗",
        "🕘",
        "🕙",
        "🕚"
    ], interval: 100},
    Spinners::Earth => SpinnerFrames {frames: vec![
        "🌍",
        "🌎",
        "🌏"
    ], interval: 180},
    Spinners::Moon => SpinnerFrames {frames: vec![
        "🌑",
        "🌒",
        "🌓",
        "🌔",
        "🌕",
        "🌖",
        "🌗",
        "🌘"
    ], interval: 80},
    Spinners::Men => SpinnerFrames {frames: vec![
        "🚶",
        "🏃"
    ], interval: 140},
    Spinners::Weather => SpinnerFrames {frames: vec![
        "☀️",
        "☀️",
        "☀️",
        "🌤",
        "⛅️",
        "🌥",
        "☁️",
        "🌧",
        "🌨",
        "🌧",
        "🌨",
        "🌧",
        "🌨",
        "⛈",
        "🌨",
        "🌧",
        "🌨",
        "☁️",
        "🌥",
        "⛅️",
        "🌤",
        "☀️",
        "☀️"
    ], interval: 100},
    Spinners::Point => SpinnerFrames {frames: vec![
        "∙∙∙",
        "●∙∙",
        "∙●∙",
        "∙∙●",
        "∙∙∙"
    ], interval: 125}
}