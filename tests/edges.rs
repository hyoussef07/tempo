use tempotime::DateTime;

#[test]
fn leap_day_2024() {
    // Feb 29, 2024 should parse and exist
    let dt = DateTime::from_format("2024-02-29 00:00:00", "yyyy-MM-dd HH:mm:ss").expect("parse");
    assert_eq!(dt.to_format("yyyy-MM-dd"), "2024-02-29");
}

#[test]
fn jan31_plus_one_month() {
    let dt = DateTime::from_format("2025-01-31 00:00:00", "yyyy-MM-dd HH:mm:ss").unwrap();
    let plus = dt.plus(&tempotime::Duration::from_object(&[("months", 1)]) );
    // should clamp to Feb 28 (2025 is not leap)
    assert_eq!(plus.to_format("yyyy-MM-dd"), "2025-02-28");
}

#[test]
fn invalid_iso_returns_err() {
    let r = DateTime::from_iso("not-a-date");
    assert!(r.is_err());
}

#[test]
fn iso_round_trip() {
    let dt = DateTime::now();
    let s = dt.to_iso();
    let parsed = DateTime::from_iso(&s).expect("roundtrip");
    assert_eq!(parsed.to_iso(), s);
}
