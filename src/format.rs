// core::fmt::Write is referenced fully-qualified in this module; avoid an unused import.

#[cfg(feature = "chrono")]
use chrono::{Datelike, Timelike};

#[cfg(feature = "chrono")]
pub(crate) fn format_datetime(dt: &chrono::DateTime<chrono::Utc>, fmt: &str) -> String {
    let mut result = String::new();
    let _ = format_datetime_into(&mut result, dt, fmt);
    result
}

#[cfg(feature = "chrono")]
pub(crate) fn format_datetime_into<W: core::fmt::Write>(result: &mut W, dt: &chrono::DateTime<chrono::Utc>, fmt: &str) -> core::fmt::Result {
    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    let hour = dt.hour();
    let minute = dt.minute();
    let second = dt.second();
    let millis = dt.timestamp_subsec_millis();
    let mut chars = fmt.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'y' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'y').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    let _ = write!(result, "{:04}", year);
                } else {
                    let _ = write!(result, "{:02}", year % 100);
                }
            }
            'M' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'M').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    let _ = result.write_str(month_name(month));
                } else if count == 3 {
                    let _ = result.write_str(month_short(month));
                } else if count == 2 {
                    let _ = write!(result, "{:02}", month);
                } else {
                    let _ = write!(result, "{}", month);
                }
            }
            'd' => {
                if chars.peek() == Some(&'o') {
                    chars.next();
                    let _ = write_ordinal(result, day);
                } else {
                    let count = 1 + chars.clone().take_while(|&c| c == 'd').count();
                    for _ in 1..count {
                        chars.next();
                    }
                    if count >= 2 {
                        let _ = write!(result, "{:02}", day);
                    } else {
                        let _ = write!(result, "{}", day);
                    }
                }
            }
            'E' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'E').count();
                for _ in 1..count {
                    chars.next();
                }
                let wd = dt.weekday().num_days_from_monday();
                if count >= 4 {
                    let _ = result.write_str(weekday_name(wd));
                } else {
                    let _ = result.write_str(weekday_short(wd));
                }
            }
            'H' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'H').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 2 {
                    let _ = write!(result, "{:02}", hour);
                } else {
                    let _ = write!(result, "{}", hour);
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
                    let _ = write!(result, "{:02}", hour12);
                } else {
                    let _ = write!(result, "{}", hour12);
                }
            }
            'm' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'm').count();
                for _ in 1..count {
                    chars.next();
                }
                let _ = write!(result, "{:02}", minute);
            }
            's' => {
                let count = 1 + chars.clone().take_while(|&c| c == 's').count();
                for _ in 1..count {
                    chars.next();
                }
                let _ = write!(result, "{:02}", second);
            }
            'S' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'S').count();
                for _ in 1..count {
                    chars.next();
                }
                let _ = write!(result, "{:03}", millis);
            }
            'a' => {
                if hour < 12 {
                    let _ = result.write_str("am");
                } else {
                    let _ = result.write_str("pm");
                }
            }
            _ => {
                let _ = write!(result, "{}", ch);
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "chrono"))]
pub(crate) fn format_datetime_from_ts(ts_ms: i64, fmt: &str) -> String {
    let mut result = String::new();
    let _ = format_datetime_from_ts_into(&mut result, ts_ms, fmt);
    result
}

#[cfg(not(feature = "chrono"))]
pub(crate) fn format_datetime_from_ts_into<W: core::fmt::Write>(w: &mut W, ts_ms: i64, fmt: &str) -> core::fmt::Result {
    let (year, month, day, hour, minute, second, millis) = decompose_timestamp_ms(ts_ms);
    let mut chars = fmt.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'y' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'y').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    let _ = write!(w, "{:04}", year);
                } else {
                    let _ = write!(w, "{:02}", year % 100);
                }
            }
            'M' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'M').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 4 {
                    let _ = w.write_str(month_name(month));
                } else if count == 3 {
                    let _ = w.write_str(month_short(month));
                } else if count == 2 {
                    let _ = write!(w, "{:02}", month);
                } else {
                    let _ = write!(w, "{}", month);
                }
            }
            'd' => {
                if chars.peek() == Some(&'o') {
                    chars.next();
                    let _ = write_ordinal(w, day);
                } else {
                    let count = 1 + chars.clone().take_while(|&c| c == 'd').count();
                    for _ in 1..count {
                        chars.next();
                    }
                    if count >= 2 {
                        let _ = write!(w, "{:02}", day);
                    } else {
                        let _ = write!(w, "{}", day);
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
                    let _ = w.write_str(weekday_name(wd));
                } else {
                    let _ = w.write_str(weekday_short(wd));
                }
            }
            'H' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'H').count();
                for _ in 1..count {
                    chars.next();
                }
                if count >= 2 {
                    let _ = write!(w, "{:02}", hour);
                } else {
                    let _ = write!(w, "{}", hour);
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
                    let _ = write!(w, "{:02}", hour12);
                } else {
                    let _ = write!(w, "{}", hour12);
                }
            }
            'm' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'm').count();
                for _ in 1..count {
                    chars.next();
                }
                let _ = write!(w, "{:02}", minute);
            }
            's' => {
                let count = 1 + chars.clone().take_while(|&c| c == 's').count();
                for _ in 1..count {
                    chars.next();
                }
                let _ = write!(w, "{:02}", second);
            }
            'S' => {
                let count = 1 + chars.clone().take_while(|&c| c == 'S').count();
                for _ in 1..count {
                    chars.next();
                }
                let _ = write!(w, "{:03}", millis);
            }
            'a' => {
                if hour < 12 {
                    let _ = w.write_str("am");
                } else {
                    let _ = w.write_str("pm");
                }
            }
            _ => {
                let _ = write!(w, "{}", ch);
            }
        }
    }

    Ok(())
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

fn write_ordinal<W: core::fmt::Write>(w: &mut W, day: u32) -> core::fmt::Result {
    let suffix = match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };
    write!(w, "{}", day)?;
    w.write_str(suffix)
}

// `ordinal` helper removed — keep formatting helpers minimal to avoid dead code.

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
