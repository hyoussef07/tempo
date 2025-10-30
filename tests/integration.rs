use tempotime::{dt, DateTime, Duration, Interval};

#[test]
fn test_full_luxon_example() {
    let result = dt()
        .plus(&Duration::from_object(&[("days", 3)]))
        .start_of("day");

    assert!(result.to_iso().contains("T00:00:00"));
}

#[test]
fn test_interval_basic() {
    let start = DateTime::from_iso("2025-10-01T00:00:00Z").unwrap();
    let end = DateTime::from_iso("2025-10-31T23:59:59Z").unwrap();
    let interval = Interval::from_date_times(start, end);

    let mid = DateTime::from_iso("2025-10-15T12:00:00Z").unwrap();
    assert!(interval.contains(&mid));
}

#[test]
fn test_interval_boundaries() {
    let start = DateTime::from_iso("2025-10-01T00:00:00Z").unwrap();
    let end = DateTime::from_iso("2025-10-31T23:59:59Z").unwrap();
    let interval = Interval::from_date_times(start.clone(), end.clone());

    assert!(interval.contains(&start));
    assert!(interval.contains(&end));
}

#[test]
fn test_interval_outside() {
    let start = DateTime::from_iso("2025-10-01T00:00:00Z").unwrap();
    let end = DateTime::from_iso("2025-10-31T23:59:59Z").unwrap();
    let interval = Interval::from_date_times(start, end);

    let before = DateTime::from_iso("2025-09-30T23:59:59Z").unwrap();
    let after = DateTime::from_iso("2025-11-01T00:00:00Z").unwrap();

    assert!(!interval.contains(&before));
    assert!(!interval.contains(&after));
}

#[test]
fn test_interval_length() {
    let start = DateTime::from_iso("2025-10-01T00:00:00Z").unwrap();
    let end = DateTime::from_iso("2025-10-08T00:00:00Z").unwrap();
    let interval = Interval::from_date_times(start, end);

    let dur = interval.length("days");
    assert_eq!(dur.as_unit("days"), 7);
}

#[test]
fn test_complex_chaining() {
    let result = DateTime::from_iso("2025-01-15T08:30:00Z")
        .unwrap()
        .plus(&Duration::from_object(&[("months", 3)]))
        .minus(&Duration::from_object(&[("weeks", 2)]))
        .start_of("month")
        .plus(&Duration::from_object(&[("days", 10)]))
        .end_of("day");

    assert!(result.to_format("yyyy-MM-dd").starts_with("2025-04-11"));
}

#[test]
fn test_format_and_parse_roundtrip() {
    let original = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
    let iso = original.to_iso();
    let parsed = DateTime::from_iso(&iso).unwrap();

    let diff = original.diff(&parsed, "seconds").abs();
    assert!(diff < 1.0);
}

#[test]
fn test_dt_convenience_function() {
    let dt1 = dt();
    let dt2 = DateTime::now();

    let diff = dt1.diff(&dt2, "seconds").abs();
    assert!(diff < 1.0);
}

#[test]
fn test_multiple_format_tokens() {
    let dt = DateTime::from_iso("2025-10-29T14:05:09Z").unwrap();
    let formatted = dt.to_format("EEEE, MMMM do yyyy 'at' h:mm a");

    assert!(formatted.contains("2025"));
    assert!(formatted.contains("October") || formatted.contains("Oct"));
    assert!(formatted.contains("29") || formatted.contains("th"));
}

#[test]
fn test_locale_presets_accessible() {
    let dt = dt();

    let _ = dt.to_locale_string(DateTime::DATE_SHORT);
    let _ = dt.to_locale_string(DateTime::DATE_MED);
    let _ = dt.to_locale_string(DateTime::DATE_FULL);
    let _ = dt.to_locale_string(DateTime::TIME_SIMPLE);
    let _ = dt.to_locale_string(DateTime::DATETIME_SHORT);
}

#[test]
fn test_diff_various_units() {
    let dt1 = DateTime::from_iso("2025-10-29T12:00:00Z").unwrap();
    let dt2 = DateTime::from_iso("2025-10-22T12:00:00Z").unwrap();

    assert!((dt1.diff(&dt2, "days") - 7.0).abs() < 0.01);
    assert!((dt1.diff(&dt2, "weeks") - 1.0).abs() < 0.01);
    assert!((dt1.diff(&dt2, "hours") - 168.0).abs() < 0.01);
}

#[test]
fn test_end_of_february_leap_year() {
    let dt = DateTime::from_iso("2024-02-15T12:00:00Z").unwrap();
    let end = dt.end_of("month");
    assert_eq!(end.to_format("dd"), "29");
}

#[test]
fn test_end_of_february_non_leap_year() {
    let dt = DateTime::from_iso("2025-02-15T12:00:00Z").unwrap();
    let end = dt.end_of("month");
    assert_eq!(end.to_format("dd"), "28");
}

#[test]
#[cfg(feature = "chrono")]
fn test_month_overflow() {
    let dt = DateTime::from_iso("2025-01-31T12:00:00Z").unwrap();
    let plus_one = dt.plus(&Duration::from_object(&[("months", 1)]));

    assert!(plus_one.to_format("MM") == "02" || plus_one.to_format("MM") == "01");
}
