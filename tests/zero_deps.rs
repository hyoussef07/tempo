#![cfg(not(feature = "tz"))]

use tempotime::{DateTime, Duration};

#[test]
fn iso_roundtrip() {
    let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
    let iso = dt.to_iso();
    assert!(iso.starts_with("2025-10-29T14:30:00"));
    assert!(iso.ends_with("Z") || iso.contains("+00:00"));
    let dt2 = DateTime::from_iso(&dt.to_iso()).unwrap();
    assert_eq!(
        dt2.to_format("yyyy-MM-dd HH:mm:ss"),
        dt.to_format("yyyy-MM-dd HH:mm:ss")
    );
}

#[test]
fn basic_format_and_plus() {
    let dt = DateTime::from_iso("2025-10-29T00:00:00Z").unwrap();
    let s = dt.to_format("yyyy-MM-dd");
    assert_eq!(s, "2025-10-29");

    let dur = Duration::from_object(&[("days", 3)]);
    let dt2 = dt.plus(&dur);
    assert_eq!(dt2.to_format("yyyy-MM-dd"), "2025-11-01");
}
