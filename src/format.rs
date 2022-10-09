pub fn convert(second: u64) -> String {
    let seconds = second % 60;
    let minutes = (second / 60) % 60;
    let hours = (second / 60) / 60;
    if hours == 0 {
        return format!("{:#02}:{:#02}", minutes, seconds);
    }
    format!("{:#02}:{:#02}:{:#02}", hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use crate::format::convert;

    #[test]
    fn test_convert_hours_equal_zero() {
        let hour = convert(1000);
        assert_eq!(hour, String::from("16:40"))
    }
    #[test]
    fn test_convert_hour_zero() {
        let hour = convert(0);
        assert_eq!(hour, String::from("00:00"))
    }
    #[test]
    fn test_convert_hours_not_equal_zero() {
        let hour = convert(10000);
        assert_eq!(hour, String::from("02:46:40"))
    }
}
