use tempo::{DateTime, Duration};

#[test]
fn test_datetime_now() {
    let dt = DateTime::now();
    let iso = dt.to_iso();
    assert!(iso.contains("T"));
    assert!(iso.contains("Z") || iso.contains("+") || iso.contains("-"));
}

#[test]
fn test_datetime_local() {
    let dt = DateTime::local();
    assert!(dt.to_iso().len() > 0);
}

#[test]
fn test_from_iso_valid() {
    let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
    assert_eq!(dt.to_format("yyyy-MM-dd"), "2025-10-29");
}

#[test]
fn test_from_iso_invalid() {
    let result = DateTime::from_iso("not a date");
    assert!(result.is_err());
}

#[test]
fn test_plus_days() {
    let dt = DateTime::from_iso("2025-10-29T12:00:00Z").unwrap();
    let dur = Duration::from_object(&[("days", 3)]);
    let result = dt.plus(&dur);
    assert_eq!(result.to_format("yyyy-MM-dd"), "2025-11-01");
}

#[test]
fn test_plus_months() {
    let dt = DateTime::from_iso("2025-01-15T12:00:00Z").unwrap();
    let dur = Duration::from_object(&[("months", 2)]);
    let result = dt.plus(&dur);
    assert_eq!(result.to_format("yyyy-MM"), "2025-03");
}

#[test]
fn test_plus_years() {
    let dt = DateTime::from_iso("2025-06-15T12:00:00Z").unwrap();
    let dur = Duration::from_object(&[("years", 1)]);
    let result = dt.plus(&dur);
    assert_eq!(result.to_format("yyyy"), "2026");
}

#[test]
fn test_minus() {
    let dt = DateTime::from_iso("2025-10-29T12:00:00Z").unwrap();
    let dur = Duration::from_object(&[("weeks", 1)]);
    let result = dt.minus(&dur);
    assert_eq!(result.to_format("yyyy-MM-dd"), "2025-10-22");
}

#[test]
fn test_start_of_year() {
    let dt = DateTime::from_iso("2025-10-29T14:30:45Z").unwrap();
    let result = dt.start_of("year");
    assert_eq!(
        result.to_format("yyyy-MM-dd HH:mm:ss"),
        "2025-01-01 00:00:00"
    );
}

#[test]
fn test_start_of_month() {
    let dt = DateTime::from_iso("2025-10-29T14:30:45Z").unwrap();
    let result = dt.start_of("month");
    assert_eq!(
        result.to_format("yyyy-MM-dd HH:mm:ss"),
        "2025-10-01 00:00:00"
    );
}

#[test]
fn test_start_of_day() {
    let dt = DateTime::from_iso("2025-10-29T14:30:45Z").unwrap();
    let result = dt.start_of("day");
    assert_eq!(
        result.to_format("yyyy-MM-dd HH:mm:ss"),
        "2025-10-29 00:00:00"
    );
}

#[test]
fn test_end_of_day() {
    let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
    let result = dt.end_of("day");
    assert_eq!(
        result.to_format("yyyy-MM-dd HH:mm:ss"),
        "2025-10-29 23:59:59"
    );
}

#[test]
fn test_end_of_month() {
    let dt = DateTime::from_iso("2025-10-15T12:00:00Z").unwrap();
    let result = dt.end_of("month");
    assert_eq!(result.to_format("yyyy-MM-dd"), "2025-10-31");
}

#[test]
fn test_end_of_year() {
    let dt = DateTime::from_iso("2025-06-15T12:00:00Z").unwrap();
    let result = dt.end_of("year");
    assert_eq!(result.to_format("yyyy-MM-dd"), "2025-12-31");
}

#[test]
fn test_to_format_basic() {
    let dt = DateTime::from_iso("2025-10-29T14:05:09Z").unwrap();
    assert_eq!(dt.to_format("yyyy"), "2025");
    assert_eq!(dt.to_format("MM"), "10");
    assert_eq!(dt.to_format("dd"), "29");
}

#[test]
fn test_to_format_full() {
    let dt = DateTime::from_iso("2025-10-29T14:05:09Z").unwrap();
    assert_eq!(dt.to_format("yyyy-MM-dd"), "2025-10-29");
}

#[test]
fn test_to_format_ordinal() {
    let dt = DateTime::from_iso("2025-10-01T12:00:00Z").unwrap();
    assert!(dt.to_format("do").contains("st"));

    let dt2 = DateTime::from_iso("2025-10-22T12:00:00Z").unwrap();
    assert!(dt2.to_format("do").contains("nd"));

    let dt3 = DateTime::from_iso("2025-10-23T12:00:00Z").unwrap();
    assert!(dt3.to_format("do").contains("rd"));
}

#[test]
fn test_to_locale_string() {
    let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
    let result = dt.to_locale_string(DateTime::DATE_SHORT);
    assert!(result.contains("10"));
    assert!(result.contains("29"));
    assert!(result.contains("2025"));
}

#[test]
fn test_diff() {
    let dt1 = DateTime::from_iso("2025-10-29T12:00:00Z").unwrap();
    let dt2 = DateTime::from_iso("2025-10-22T12:00:00Z").unwrap();
    let diff = dt1.diff(&dt2, "days");
    assert!((diff - 7.0).abs() < 0.01);
}

#[test]
fn test_immutability() {
    let dt = DateTime::from_iso("2025-10-29T12:00:00Z").unwrap();
    let dur = Duration::from_object(&[("days", 1)]);
    let _modified = dt.clone().plus(&dur);
    assert_eq!(dt.to_format("dd"), "29");
}

#[cfg(feature = "tz")]
#[test]
fn test_set_zone() {
    let dt = DateTime::now().set_zone("America/New_York");
    let iso = dt.to_iso();
    assert!(iso.contains("-") || iso.contains("+"));
}

#[test]
#[cfg(feature = "chrono")]
fn test_chainable_operations() {
    let result = DateTime::from_iso("2025-01-01T00:00:00Z")
        .unwrap()
        .plus(&Duration::from_object(&[("months", 2)]))
        .plus(&Duration::from_object(&[("days", 15)]))
        .start_of("day")
        .to_format("yyyy-MM-dd");

    assert_eq!(result, "2025-03-16");
}
