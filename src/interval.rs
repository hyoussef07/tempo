use crate::{DateTime, Duration};

/// A range of time between two DateTimes.
///
/// # Examples
///
/// ```rust
/// use tempotime::{dt, Duration, Interval};
///
/// let start = dt();
/// let end = start.clone().plus(&Duration::from_object(&[("days", 30)]));
/// let interval = Interval::from_date_times(start, end);
///
/// let check = dt().plus(&Duration::from_object(&[("days", 15)]));
/// assert!(interval.contains(&check));
/// ```
#[derive(Debug, Clone)]
pub struct Interval {
    start: DateTime,
    end: DateTime,
}

impl Interval {
    pub fn from_date_times(start: DateTime, end: DateTime) -> Self {
        Interval { start, end }
    }

    pub fn contains(&self, dt: &DateTime) -> bool {
        dt >= &self.start && dt <= &self.end
    }

    pub fn length(&self, unit: &str) -> Duration {
        let diff = self.start.diff(&self.end, unit).abs() as i64;
        Duration::from_object(&[(unit, diff)])
    }

    pub fn start(&self) -> &DateTime {
        &self.start
    }

    pub fn end(&self) -> &DateTime {
        &self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_contains() {
        let start = DateTime::from_iso("2025-10-01T00:00:00Z").unwrap();
        let end = DateTime::from_iso("2025-10-31T23:59:59Z").unwrap();
        let interval = Interval::from_date_times(start, end);

        let mid = DateTime::from_iso("2025-10-15T12:00:00Z").unwrap();
        assert!(interval.contains(&mid));

        let before = DateTime::from_iso("2025-09-30T00:00:00Z").unwrap();
        assert!(!interval.contains(&before));
    }

    #[test]
    fn test_interval_length() {
        let start = DateTime::from_iso("2025-10-01T00:00:00Z").unwrap();
        let end = DateTime::from_iso("2025-10-08T00:00:00Z").unwrap();
        let interval = Interval::from_date_times(start, end);

        let dur = interval.length("days");
        assert_eq!(dur.as_unit("days"), 7);
    }
}
