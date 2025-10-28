use chrono::{Datelike, Timelike};

pub(crate) fn format_datetime(dt: &chrono::DateTime<chrono::Utc>, fmt: &str) -> String {
    let mut result = String::new();
    let mut chars = fmt.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'y' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'y').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    result.push_str(&format!("{:04}", dt.year()));
                } else {
                    result.push_str(&format!("{:02}", dt.year() % 100));
                }
            }
            'M' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'M').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    result.push_str(month_name(dt.month()));
                } else if count == 3 {
                    result.push_str(month_short(dt.month()));
                } else if count == 2 {
                    result.push_str(&format!("{:02}", dt.month()));
                } else {
                    result.push_str(&format!("{}", dt.month()));
                }
            }
            'd' => {
                if chars.peek() == Some(&'o') {
                    chars.next();
                    result.push_str(&ordinal(dt.day()));
                } else {
                    let count = 1 + chars.clone().take_while(|&c| c == 'd').count();
                    for _ in 1..count {
                        chars.next();
                    }
                    if count >= 2 {
                        result.push_str(&format!("{:02}", dt.day()));
                    } else {
                        result.push_str(&format!("{}", dt.day()));
                    }
                }
            }
            'E' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'E').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    result.push_str(weekday_name(dt.weekday().num_days_from_monday()));
                } else {
                    result.push_str(weekday_short(dt.weekday().num_days_from_monday()));
                }
            }
            'H' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'H').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 2 {
                    result.push_str(&format!("{:02}", dt.hour()));
                } else {
                    result.push_str(&format!("{}", dt.hour()));
                }
            }
            'h' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'h').count();
                for _ in 1..count {
                    chars.next();
                }
                let hour12 = match dt.hour() {
                    0 => 12,
                    h if h > 12 => h - 12,
                    h => h,
                };
                if count >= 2 {
                    result.push_str(&format!("{:02}", hour12));
                } else {
                    result.push_str(&format!("{}", hour12));
                }
            }
            'm' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'm').count();
                for _ in 1..count {
                    chars.next();
                }
                result.push_str(&format!("{:02}", dt.minute()));
            }
            's' => {
                let count = 1 + chars.clone().take_while(|&c| c == 's').count();
                for _ in 1..count {
                    chars.next();
                }
                result.push_str(&format!("{:02}", dt.second()));
            }
            'S' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'S').count();
                for _ in 1..count {
                    chars.next();
                }
                let ms = dt.timestamp_subsec_millis();
                result.push_str(&format!("{:03}", ms));
            }
            'a' => {
                if dt.hour() < 12 {
                    result.push_str("am");
                } else {
                    result.push_str("pm");
                }
            }
            _ => result.push(ch),
        }
    }

    result
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

fn month_short(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

fn weekday_name(day: u32) -> &'static str {
    match day {
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Saturday",
        6 => "Sunday",
        _ => "",
    }
}

fn weekday_short(day: u32) -> &'static str {
    match day {
        0 => "Mon",
        1 => "Tue",
        2 => "Wed",
        3 => "Thu",
        4 => "Fri",
        5 => "Sat",
        6 => "Sun",
        _ => "",
    }
}

fn ordinal(day: u32) -> String {
    let suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };
    format!("{}{}", day, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_tokens() {
        let dt = chrono::Utc.with_ymd_and_hms(2025, 10, 29, 14, 5, 9).unwrap();
        assert_eq!(format_datetime(&dt, "yyyy"), "2025");
        assert_eq!(format_datetime(&dt, "yy"), "25");
        assert_eq!(format_datetime(&dt, "MMMM"), "October");
        assert_eq!(format_datetime(&dt, "MMM"), "Oct");
        assert_eq!(format_datetime(&dt, "MM"), "10");
        assert_eq!(format_datetime(&dt, "dd"), "29");
        assert_eq!(format_datetime(&dt, "do"), "29th");
        assert_eq!(format_datetime(&dt, "HH"), "14");
        assert_eq!(format_datetime(&dt, "hh"), "02");
        assert_eq!(format_datetime(&dt, "mm"), "05");
        assert_eq!(format_datetime(&dt, "ss"), "09");
        assert_eq!(format_datetime(&dt, "a"), "pm");
    }

    #[test]
    fn test_ordinals() {
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(2), "2nd");
        assert_eq!(ordinal(3), "3rd");
        assert_eq!(ordinal(21), "21st");
        assert_eq!(ordinal(22), "22nd");
        assert_eq!(ordinal(23), "23rd");
        assert_eq!(ordinal(11), "11th");
    }
}
