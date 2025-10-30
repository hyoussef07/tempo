#[cfg(feature = "chrono")]
use chrono::{DateTime as ChronoDateTime, Datelike, TimeZone, Timelike, Utc};
#[cfg(feature = "tz")]
use chrono_tz::Tz;

#[cfg(not(feature = "chrono"))]
use std::time::{SystemTime, UNIX_EPOCH};

use crate::duration::Duration;
#[cfg(feature = "chrono")]
use crate::format::format_datetime;
use crate::locale;

/// A date and time value with timezone support.
///
/// `DateTime` is immutable – all operations return new instances.
/// In zero-deps mode, times are always UTC. Enable the `tz` feature for timezone support.
///
/// # Examples
///
/// ```rust
/// use tempotime::{DateTime, Duration};
///
/// // Get current time
/// let now = DateTime::now();
///
/// // Parse ISO string
/// let dt = DateTime::from_iso("2025-10-30T14:30:00Z").unwrap();
///
/// // Add duration
/// let future = dt.plus(&Duration::from_object(&[("days", 7)]));
///
/// // Format output
/// println!("{}", future.to_format("yyyy-MM-dd HH:mm:ss"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    #[cfg(feature = "chrono")]
    inner: ChronoDateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    timestamp_ms: i64,
    #[cfg(feature = "tz")]
    zone: Option<Tz>,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        #[cfg(feature = "chrono")]
        return self.inner.partial_cmp(&other.inner);
        #[cfg(not(feature = "chrono"))]
        return self.timestamp_ms.partial_cmp(&other.timestamp_ms);
    }
}

impl DateTime {
    /// Creates a DateTime representing the current moment in UTC.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempotime::DateTime;
    ///
    /// let now = DateTime::now();
    /// println!("Current time: {}", now.to_iso());
    /// ```
    #[cfg(feature = "chrono")]
    pub fn now() -> Self {
        DateTime {
            inner: Utc::now(),
            #[cfg(feature = "tz")]
            zone: None,
        }
    }

    #[cfg(not(feature = "chrono"))]
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        DateTime {
            timestamp_ms: duration.as_millis() as i64,
        }
    }

    #[cfg(feature = "chrono")]
    pub fn local() -> Self {
        let local = chrono::Local::now();
        DateTime {
            inner: local.with_timezone(&Utc),
            #[cfg(feature = "tz")]
            zone: None,
        }
    }

    #[cfg(not(feature = "chrono"))]
    pub fn local() -> Self {
        Self::now()
    }

    #[cfg(feature = "chrono")]
    pub fn from_iso(s: &str) -> Result<Self, String> {
        s.parse::<ChronoDateTime<Utc>>()
            .map(|dt| DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: None,
            })
            .map_err(|e| format!("Invalid ISO date: {}", e))
    }

    #[cfg(not(feature = "chrono"))]
    pub fn from_iso(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.len() < 19 {
            return Err("ISO string too short".to_string());
        }

        let year: i32 = s[0..4].parse().map_err(|_| "Invalid year")?;
        let month: u32 = s[5..7].parse().map_err(|_| "Invalid month")?;
        let day: u32 = s[8..10].parse().map_err(|_| "Invalid day")?;
        let hour: u32 = s[11..13].parse().map_err(|_| "Invalid hour")?;
        let minute: u32 = s[14..16].parse().map_err(|_| "Invalid minute")?;
        let second: u32 = s[17..19].parse().map_err(|_| "Invalid second")?;

        let timestamp_ms = Self::compute_timestamp(year, month, day, hour, minute, second, 0);
        Ok(DateTime { timestamp_ms })
    }

    pub fn from_format(s: &str, _fmt: &str) -> Result<Self, String> {
        Self::from_iso(s)
    }

    #[cfg(feature = "tz")]
    pub fn set_zone(mut self, zone: &str) -> Self {
        if let Ok(tz) = zone.parse::<Tz>() {
            self.zone = Some(tz);
        }
        self
    }

    #[cfg(not(feature = "tz"))]
    pub fn set_zone(self, _zone: &str) -> Self {
        self
    }

    pub fn plus(self, dur: &Duration) -> Self {
        let (years, months, weeks, days, hours, minutes, seconds, millis) = dur.components();
        #[cfg(feature = "chrono")]
        {
            let mut dt = self.inner;

            if years != 0 {
                let new_year = dt.year() + years as i32;
                dt = Utc
                    .with_ymd_and_hms(
                        new_year,
                        dt.month(),
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second(),
                    )
                    .single()
                    .unwrap_or(dt);
            }
            if months != 0 {
                let total_months = dt.month() as i32 + months as i32;
                let new_month = ((total_months - 1).rem_euclid(12) + 1) as u32;
                let year_offset = (total_months - 1).div_euclid(12);
                let new_year = dt.year() + year_offset;
                dt = Utc
                    .with_ymd_and_hms(
                        new_year,
                        new_month,
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second(),
                    )
                    .single()
                    .unwrap_or(dt);
            }

            let total_secs =
                weeks * 7 * 86400 + days * 86400 + hours * 3600 + minutes * 60 + seconds;
            dt += chrono::Duration::seconds(total_secs);
            dt += chrono::Duration::milliseconds(millis);

            DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: self.zone,
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            // Accurate month/year handling without chrono.
            // 1) Decompose current timestamp into components
            let (mut y, mut m, mut d, mut h, mut mi, mut s, mut ms) =
                crate::format::decompose_timestamp_ms(self.timestamp_ms);

            // Apply years as months offset
            let mut total_months: i64 = months as i64 + years as i64 * 12;
            if total_months != 0 {
                let (ny, nm, nd) = add_months_to_ymd(y, m, d, total_months);
                y = ny;
                m = nm;
                d = nd;
            }

            // Recompute base timestamp after year/month/day adjustments
            let base_ts = Self::compute_timestamp(y, m, d, h, mi, s, ms);

            // Apply weeks/days/hours/minutes/seconds/millis as ms offset
            let small_ms = (weeks * 7 * 86400 * 1000)
                + (days * 86400 * 1000)
                + (hours * 3600 * 1000)
                + (minutes * 60 * 1000)
                + (seconds * 1000)
                + millis;

            DateTime {
                timestamp_ms: base_ts + small_ms,
                #[cfg(feature = "tz")]
                zone: self.zone,
            }
        }
    }

    pub fn minus(self, dur: &Duration) -> Self {
        let (years, months, weeks, days, hours, minutes, seconds, millis) = dur.components();
        let negated = Duration::from_object(&[
            ("years", -years),
            ("months", -months),
            ("weeks", -weeks),
            ("days", -days),
            ("hours", -hours),
            ("minutes", -minutes),
            ("seconds", -seconds),
            ("milliseconds", -millis),
        ]);
        self.plus(&negated)
    }

    pub fn start_of(self, unit: &str) -> Self {
        #[cfg(feature = "chrono")]
        {
            let dt = match unit {
                "year" => Utc
                    .with_ymd_and_hms(self.inner.year(), 1, 1, 0, 0, 0)
                    .single()
                    .unwrap(),
                "month" => Utc
                    .with_ymd_and_hms(self.inner.year(), self.inner.month(), 1, 0, 0, 0)
                    .single()
                    .unwrap(),
                "day" => Utc
                    .with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        0,
                        0,
                        0,
                    )
                    .single()
                    .unwrap(),
                "hour" => Utc
                    .with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        0,
                        0,
                    )
                    .single()
                    .unwrap(),
                "minute" => Utc
                    .with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        self.inner.minute(),
                        0,
                    )
                    .single()
                    .unwrap(),
                "second" => self.inner,
                _ => self.inner,
            };
            DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: self.zone,
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            let (y, m, d, h, mi, s, ms) = crate::format::decompose_timestamp_ms(self.timestamp_ms);
            let (ny, nm, nd, nh, nmi, ns, nms) = match unit {
                "year" => (y, 1, 1, 0, 0, 0, 0),
                "month" => (y, m, 1, 0, 0, 0, 0),
                "day" => (y, m, d, 0, 0, 0, 0),
                "hour" => (y, m, d, h, 0, 0, 0),
                "minute" => (y, m, d, h, mi, 0, 0),
                "second" => (y, m, d, h, mi, s, 0),
                _ => (y, m, d, h, mi, s, ms),
            };
            DateTime {
                timestamp_ms: Self::compute_timestamp(ny, nm, nd, nh, nmi, ns, nms),
            }
        }
    }

    pub fn end_of(self, unit: &str) -> Self {
        #[cfg(feature = "chrono")]
        {
            let dt = match unit {
                "year" => {
                    Utc.with_ymd_and_hms(self.inner.year(), 12, 31, 23, 59, 59)
                        .single()
                        .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                "month" => {
                    let next_month = if self.inner.month() == 12 {
                        Utc.with_ymd_and_hms(self.inner.year() + 1, 1, 1, 0, 0, 0)
                            .single()
                            .unwrap()
                    } else {
                        Utc.with_ymd_and_hms(self.inner.year(), self.inner.month() + 1, 1, 0, 0, 0)
                            .single()
                            .unwrap()
                    };
                    next_month - chrono::Duration::milliseconds(1)
                }
                "day" => {
                    Utc.with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        23,
                        59,
                        59,
                    )
                    .single()
                    .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                "hour" => {
                    Utc.with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        59,
                        59,
                    )
                    .single()
                    .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                "minute" => {
                    Utc.with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        self.inner.minute(),
                        59,
                    )
                    .single()
                    .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                _ => self.inner,
            };
            DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: self.zone,
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            let (y, m, d, h, mi, s, _) = crate::format::decompose_timestamp_ms(self.timestamp_ms);
            let (ny, nm, nd, nh, nmi, ns, nms) = match unit {
                "year" => (y, 12, 31, 23, 59, 59, 999),
                "month" => {
                    // compute last day of month
                    let next = if m == 12 {
                        (y + 1, 1, 1)
                    } else {
                        (y, m + 1, 1)
                    };
                    let last_day_ts =
                        Self::compute_timestamp(next.0, next.1, next.2, 0, 0, 0, 0) - 1;
                    let (yy, mm, dd, hh, mn, ss, ms) =
                        crate::format::decompose_timestamp_ms(last_day_ts);
                    (yy, mm, dd, hh, mn, ss, ms)
                }
                "day" => (y, m, d, 23, 59, 59, 999),
                "hour" => (y, m, d, h, 59, 59, 999),
                "minute" => (y, m, d, h, mi, 59, 999),
                _ => (y, m, d, h, mi, s, 0),
            };
            DateTime {
                timestamp_ms: Self::compute_timestamp(ny, nm, nd, nh, nmi, ns, nms),
            }
        }
    }

    pub fn to_iso(&self) -> String {
        #[cfg(feature = "chrono")]
        {
            #[cfg(feature = "tz")]
            if let Some(tz) = self.zone {
                return self.inner.with_timezone(&tz).to_rfc3339();
            }
            self.inner.to_rfc3339()
        }

        #[cfg(not(feature = "chrono"))]
        {
            let (y, m, d, h, mi, s, _) = crate::format::decompose_timestamp_ms(self.timestamp_ms);
            format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", y, m, d, h, mi, s)
        }
    }

    pub fn to_format(&self, fmt: &str) -> String {
        #[cfg(feature = "chrono")]
        {
            #[cfg(feature = "tz")]
            if let Some(tz) = self.zone {
                let _local_dt = self.inner.with_timezone(&tz);
                return format_datetime(&self.inner, fmt);
            }
            format_datetime(&self.inner, fmt)
        }

        #[cfg(not(feature = "chrono"))]
        {
            crate::format::format_datetime_from_ts(self.timestamp_ms, fmt)
        }
    }

    pub fn to_locale_string(&self, preset: &str) -> String {
        #[cfg(feature = "chrono")]
        {
            locale::to_locale_string(&self.inner, preset)
        }
        #[cfg(not(feature = "chrono"))]
        {
            locale::to_locale_string_from_ts(self.timestamp_ms, preset)
        }
    }

    pub fn diff(&self, other: &DateTime, unit: &str) -> f64 {
        #[cfg(feature = "chrono")]
        let diff_ms = (self.inner.timestamp_millis() - other.inner.timestamp_millis()) as f64;
        #[cfg(not(feature = "chrono"))]
        let diff_ms = (self.timestamp_ms - other.timestamp_ms) as f64;
        match unit {
            "milliseconds" | "millisecond" => diff_ms,
            "seconds" | "second" => diff_ms / 1000.0,
            "minutes" | "minute" => diff_ms / (1000.0 * 60.0),
            "hours" | "hour" => diff_ms / (1000.0 * 60.0 * 60.0),
            "days" | "day" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0),
            "weeks" | "week" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0 * 7.0),
            "months" | "month" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0 * 30.0),
            "years" | "year" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0 * 365.0),
            _ => 0.0,
        }
    }

    #[cfg(not(feature = "chrono"))]
    fn compute_timestamp(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millis: u32,
    ) -> i64 {
        let days = days_from_civil(year, month, day);
        let secs = days * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64;
        secs * 1000 + millis as i64
    }

    pub const DATE_SHORT: &'static str = locale::DATE_SHORT;
    pub const DATE_MED: &'static str = locale::DATE_MED;
    pub const DATE_FULL: &'static str = locale::DATE_FULL;
    pub const TIME_SIMPLE: &'static str = locale::TIME_SIMPLE;
    pub const TIME_WITH_SECONDS: &'static str = locale::TIME_WITH_SECONDS;
    pub const DATETIME_SHORT: &'static str = locale::DATETIME_SHORT;
    pub const DATETIME_MED: &'static str = locale::DATETIME_MED;
    pub const DATETIME_FULL: &'static str = locale::DATETIME_FULL;
}

#[cfg(not(feature = "chrono"))]
fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = year as i64 - if month <= 2 { 1 } else { 0 };
    let m = month as i64;
    let d = day as i64;
    let era = if y >= 0 { y / 400 } else { (y - 399) / 400 };
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

#[cfg(not(feature = "chrono"))]
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(not(feature = "chrono"))]
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

#[cfg(not(feature = "chrono"))]
fn add_months_to_ymd(year: i32, month: u32, day: u32, offset_months: i64) -> (i32, u32, u32) {
    // Convert to zero-based month count
    let mut total = year as i64 * 12 + (month as i64 - 1) + offset_months;
    // compute new year and month
    let new_year = (total / 12) as i32;
    let mut new_month = (total % 12) as i32 + 1;
    if new_month <= 0 {
        new_month += 12;
    }
    let new_month_u = new_month as u32;
    // clamp day to last day of new month
    let max_day = days_in_month(new_year, new_month_u);
    let new_day = if day > max_day { max_day } else { day };
    (new_year, new_month_u, new_day)
}

#[cfg(all(test, feature = "chrono"))]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let dt = DateTime::now();
        assert!(dt.inner.timestamp() > 0);
    }

    #[test]
    fn test_from_iso() {
        let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
        assert_eq!(dt.inner.year(), 2025);
        assert_eq!(dt.inner.month(), 10);
        assert_eq!(dt.inner.day(), 29);
    }

    #[test]
    fn test_plus() {
        let dt = DateTime::from_iso("2025-10-29T00:00:00Z").unwrap();
        let dur = Duration::from_object(&[("days", 3)]);
        let result = dt.plus(&dur);
        assert_eq!(result.inner.day(), 1); // Nov 1
    }

    #[test]
    fn test_start_of() {
        let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
        let start = dt.start_of("day");
        assert_eq!(start.inner.hour(), 0);
        assert_eq!(start.inner.minute(), 0);
    }

    #[test]
    fn test_end_of() {
        let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
        let end = dt.end_of("day");
        assert_eq!(end.inner.hour(), 23);
        assert_eq!(end.inner.minute(), 59);
    }
}
