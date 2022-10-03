pub fn convert(second: u64) -> String {
    let seconds = second % 60;
    let minutes = (second / 60) % 60;
    let hours = (second / 60) / 60;
    if hours == 0 {
        return format!("{:#02}:{:#02}", minutes, seconds);
    }
    format!("{:#02}:{:#02}:{:#02}", hours, minutes, seconds)
}