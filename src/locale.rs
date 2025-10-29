use crate::format::format_datetime;

pub const DATE_SHORT: &str = "M/d/yyyy";
pub const DATE_MED: &str = "MMM d, yyyy";
pub const DATE_FULL: &str = "MMMM d, yyyy";
pub const TIME_SIMPLE: &str = "h:mm a";
pub const TIME_WITH_SECONDS: &str = "h:mm:ss a";
pub const DATETIME_SHORT: &str = "M/d/yyyy, h:mm a";
pub const DATETIME_MED: &str = "MMM d, yyyy, h:mm a";
pub const DATETIME_FULL: &str = "MMMM d, yyyy, h:mm a";

pub(crate) fn to_locale_string(dt: &chrono::DateTime<chrono::Utc>, preset: &str) -> String {
    let format = match preset {
        "DATE_SHORT" => DATE_SHORT,
        "DATE_MED" => DATE_MED,
        "DATE_FULL" => DATE_FULL,
        "TIME_SIMPLE" => TIME_SIMPLE,
        "TIME_WITH_SECONDS" => TIME_WITH_SECONDS,
        "DATETIME_SHORT" => DATETIME_SHORT,
        "DATETIME_MED" => DATETIME_MED,
        "DATETIME_FULL" => DATETIME_FULL,
        _ => preset,
    };
    format_datetime(dt, format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_presets() {
        let dt = chrono::Utc.with_ymd_and_hms(2025, 10, 29, 14, 30, 0).unwrap();
        assert_eq!(to_locale_string(&dt, "DATE_SHORT"), "10/29/2025");
        assert_eq!(to_locale_string(&dt, "DATE_MED"), "Oct 29, 2025");
        assert_eq!(to_locale_string(&dt, "DATE_FULL"), "October 29, 2025");
        assert_eq!(to_locale_string(&dt, "TIME_SIMPLE"), "2:30 pm");
    }
}
