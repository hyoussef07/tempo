#[cfg(feature = "chrono")]
use chrono::{Datelike, Timelike};

#[cfg(feature = "chrono")]
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

#[cfg(not(feature = "chrono"))]
pub(crate) fn format_datetime_from_ts(ts_ms: i64, fmt: &str) -> String {
    let (year, month, day, hour, minute, second, millis) = decompose_timestamp_ms(ts_ms);
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
                    result.push_str(&format!("{:04}", year));
                } else {
                    result.push_str(&format!("{:02}", year % 100));
                }
            }
            'M' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'M').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    result.push_str(month_name(month));
                } else if count == 3 {
                    result.push_str(month_short(month));
                } else if count == 2 {
                    result.push_str(&format!("{:02}", month));
                } else {
                    result.push_str(&format!("{}", month));
                }
            }
            'd' => {
                if chars.peek() == Some(&'o') {
                    chars.next();
                    result.push_str(&ordinal(day));
                } else {
                    let count = 1 + chars.clone().take_while(|&c| c == 'd').count();
                    for _ in 1..count {
                        chars.next();
                    }
                    if count >= 2 {
                        result.push_str(&format!("{:02}", day));
                    } else {
                        result.push_str(&format!("{}", day));
                    }
                }
            }
            'E' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'E').count();
                for _ in 1..count {
                    chars.next();
                }
                let wd = weekday_from_ymd(year, month, day);
                if count >= 4 {
                    result.push_str(weekday_name(wd));
                } else {
                    result.push_str(weekday_short(wd));
                }
            }
            'H' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'H').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 2 {
                    result.push_str(&format!("{:02}", hour));
                } else {
                    result.push_str(&format!("{}", hour));
                }
            }
            'h' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'h').count();
                for _ in 1..count {
                    chars.next();
                }
                let hour12 = match hour {
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
                result.push_str(&format!("{:02}", minute));
            }
            's' => {
                let count = 1 + chars.clone().take_while(|&c| c == 's').count();
                for _ in 1..count {
                    chars.next();
                }
                result.push_str(&format!("{:02}", second));
            }
            'S' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'S').count();
                for _ in 1..count {
                    chars.next();
                }
                result.push_str(&format!("{:03}", millis));
            }
            'a' => {
                if hour < 12 {
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

#[cfg(not(feature = "chrono"))]
pub(crate) fn decompose_timestamp_ms(ts_ms: i64) -> (i32, u32, u32, u32, u32, u32, u32) {
    let ms_per_day = 86_400_000i64;
    let days = ts_ms.div_euclid(ms_per_day);
    let mut rem = ts_ms.rem_euclid(ms_per_day);
    let hour = (rem / 3_600_000) as u32;
    rem %= 3_600_000;
    let minute = (rem / 60_000) as u32;
    rem %= 60_000;
    let second = (rem / 1000) as u32;
    let millis = (rem % 1000) as u32;

    let (year, month, day) = civil_from_days(days);
    (year, month, day, hour, minute, second, millis)
}

#[cfg(not(feature = "chrono"))]
fn civil_from_days(mut z: i64) -> (i32, u32, u32) {
    z = z + 719468;
    let era = if z >= 0 {
        z / 146097
    } else {
        (z - 146096) / 146097
    };
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = (y + (m <= 2) as i64) as i32;
    let month = m as u32;
    (year, month, d as u32)
}

#[cfg(not(feature = "chrono"))]
fn weekday_from_ymd(y: i32, m: u32, d: u32) -> u32 {
    let y = y as i32;
    let m = m as i32;
    let d = d as i32;
    let (y, m) = if m < 3 { (y - 1, m + 12) } else { (y, m) };
    let k = y % 100;
    let j = y / 100;
    let h = (d + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    let dow = match h {
        0 => 5,
        1 => 6,
        2 => 0,
        3 => 1,
        4 => 2,
        5 => 3,
        6 => 4,
        _ => 0,
    };
    dow
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

#[cfg(all(test, feature = "chrono"))]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_tokens() {
        let dt = chrono::Utc
            .with_ymd_and_hms(2025, 10, 29, 14, 5, 9)
            .unwrap();
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
