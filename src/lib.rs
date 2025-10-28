pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod datetime;
mod duration;
mod format;
mod interval;
mod locale;

pub use datetime::DateTime;
pub use duration::Duration;
pub use interval::Interval;

pub fn dt() -> DateTime {
    DateTime::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dt_convenience() {
        let now = dt();
        assert!(now.to_iso().len() > 0);
    }

    #[test]
    fn test_chainable() {
        let dur = Duration::from_object(&[("days", 1)]);
        let result = dt().plus(&dur).start_of("day");
        assert!(result.to_iso().contains("T00:00:00"));
    }
}
