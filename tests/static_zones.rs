use tempotime::DateTime;

#[test]
#[cfg(not(feature = "tz"))]
fn apply_new_york_offset() {
    // 2025-10-30T12:00:00Z => 12:00 UTC
    let dt = DateTime::from_iso("2025-10-30T12:00:00Z").unwrap();
    let ny = dt.set_zone("America/New_York");
    // New York is UTC-5 => local should be 07:00
    assert_eq!(ny.to_format("HH"), "07");
}

#[test]
#[cfg(not(feature = "tz"))]
fn apply_tokyo_offset() {
    let dt = DateTime::from_iso("2025-10-30T00:00:00Z").unwrap();
    let t = dt.set_zone("Asia/Tokyo");
    // Tokyo is UTC+9 => local should be 09:00
    assert_eq!(t.to_format("HH"), "09");
}
