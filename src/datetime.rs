#[cfg(feature = "tz")]
use chrono_tz::Tz;
use chrono::{DateTime as ChronoDateTime, Datelike, TimeZone, Timelike, Utc};

use crate::duration::Duration;
use crate::format::format_datetime;
use crate::locale;

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    inner: ChronoDateTime<Utc>,
    #[cfg(feature = "tz")]
    zone: Option<Tz>,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl DateTime {
    pub fn now() -> Self {
        DateTime {
            inner: Utc::now(),
            #[cfg(feature = "tz")]
            zone: None,
        }
    }

    pub fn local() -> Self {
        let local = chrono::Local::now();
        DateTime {
            inner: local.with_timezone(&Utc),
            #[cfg(feature = "tz")]
            zone: None,
        }
    }

    pub fn from_iso(s: &str) -> Result<Self, String> {
        s.parse::<ChronoDateTime<Utc>>()
            .map(|dt| DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: None,
            })
            .map_err(|e| format!("Invalid ISO date: {}", e))
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
        
        let mut dt = self.inner;
        
        if years != 0 {
            let new_year = dt.year() + years as i32;
            dt = Utc.with_ymd_and_hms(new_year, dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second())
                .single()
                .unwrap_or(dt);
        }
        if months != 0 {
            let total_months = dt.month() as i32 + months as i32;
            let new_month = ((total_months - 1).rem_euclid(12) + 1) as u32;
            let year_offset = (total_months - 1).div_euclid(12);
            let new_year = dt.year() + year_offset;
            dt = Utc.with_ymd_and_hms(new_year, new_month, dt.day(), dt.hour(), dt.minute(), dt.second())
                .single()
                .unwrap_or(dt);
        }
        
        let total_secs = weeks * 7 * 86400 + days * 86400 + hours * 3600 + minutes * 60 + seconds;
        dt = dt + chrono::Duration::seconds(total_secs);
        dt = dt + chrono::Duration::milliseconds(millis);
        
        DateTime {
            inner: dt,
            #[cfg(feature = "tz")]
            zone: self.zone,
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
        let dt = match unit {
            "year" => Utc.with_ymd_and_hms(self.inner.year(), 1, 1, 0, 0, 0).single().unwrap(),
            "month" => Utc.with_ymd_and_hms(self.inner.year(), self.inner.month(), 1, 0, 0, 0)
                .single()
                .unwrap(),
            "day" => Utc.with_ymd_and_hms(
                self.inner.year(),
                self.inner.month(),
                self.inner.day(),
                0,
                0,
                0,
            )
            .single()
            .unwrap(),
            "hour" => Utc.with_ymd_and_hms(
                self.inner.year(),
                self.inner.month(),
                self.inner.day(),
                self.inner.hour(),
                0,
                0,
            )
            .single()
            .unwrap(),
            "minute" => Utc.with_ymd_and_hms(
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

    pub fn end_of(self, unit: &str) -> Self {
        let dt = match unit {
            "year" => Utc.with_ymd_and_hms(self.inner.year(), 12, 31, 23, 59, 59)
                .single()
                .unwrap()
                + chrono::Duration::milliseconds(999),
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
            "day" => Utc.with_ymd_and_hms(
                self.inner.year(),
                self.inner.month(),
                self.inner.day(),
                23,
                59,
                59,
            )
            .single()
            .unwrap()
                + chrono::Duration::milliseconds(999),
            "hour" => Utc.with_ymd_and_hms(
                self.inner.year(),
                self.inner.month(),
                self.inner.day(),
                self.inner.hour(),
                59,
                59,
            )
            .single()
            .unwrap()
                + chrono::Duration::milliseconds(999),
            "minute" => Utc.with_ymd_and_hms(
                self.inner.year(),
                self.inner.month(),
                self.inner.day(),
                self.inner.hour(),
                self.inner.minute(),
                59,
            )
            .single()
            .unwrap()
                + chrono::Duration::milliseconds(999),
            _ => self.inner,
        };
        DateTime {
            inner: dt,
            #[cfg(feature = "tz")]
            zone: self.zone,
        }
    }

    pub fn to_iso(&self) -> String {
        #[cfg(feature = "tz")]
        if let Some(tz) = self.zone {
            return self.inner.with_timezone(&tz).to_rfc3339();
        }
        self.inner.to_rfc3339()
    }

    pub fn to_format(&self, fmt: &str) -> String {
        #[cfg(feature = "tz")]
        if let Some(tz) = self.zone {
            let _local_dt = self.inner.with_timezone(&tz);
            return format_datetime(&self.inner, fmt);
        }
        format_datetime(&self.inner, fmt)
    }

    pub fn to_locale_string(&self, preset: &str) -> String {
        locale::to_locale_string(&self.inner, preset)
    }

    pub fn diff(&self, other: &DateTime, unit: &str) -> f64 {
        let diff_ms = (self.inner.timestamp_millis() - other.inner.timestamp_millis()) as f64;
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

    pub const DATE_SHORT: &'static str = locale::DATE_SHORT;
    pub const DATE_MED: &'static str = locale::DATE_MED;
    pub const DATE_FULL: &'static str = locale::DATE_FULL;
    pub const TIME_SIMPLE: &'static str = locale::TIME_SIMPLE;
    pub const TIME_WITH_SECONDS: &'static str = locale::TIME_WITH_SECONDS;
    pub const DATETIME_SHORT: &'static str = locale::DATETIME_SHORT;
    pub const DATETIME_MED: &'static str = locale::DATETIME_MED;
    pub const DATETIME_FULL: &'static str = locale::DATETIME_FULL;
}

#[cfg(test)]
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
