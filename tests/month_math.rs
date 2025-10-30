#![cfg(not(feature = "chrono"))]

use tempotime::{DateTime, Duration};

#[test]
fn test_jan31_plus_one_month() {
    let dt = DateTime::from_iso("2025-01-31T00:00:00Z").unwrap();
    let result = dt.plus(&Duration::from_object(&[("months", 1)]));
    assert_eq!(result.to_format("yyyy-MM-dd"), "2025-02-28");
}

#[test]
fn test_feb28_leap_year_plus_one_day() {
    let dt = DateTime::from_iso("2024-02-28T00:00:00Z").unwrap();
    let result = dt.plus(&Duration::from_object(&[("days", 1)]));
    assert_eq!(result.to_format("yyyy-MM-dd"), "2024-02-29");
}
