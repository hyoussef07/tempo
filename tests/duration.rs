use tempo::Duration;

#[test]
fn test_from_object_single() {
    let dur = Duration::from_object(&[("days", 5)]);
    assert_eq!(dur.as_unit("days"), 5);
}

#[test]
fn test_from_object_multiple() {
    let dur = Duration::from_object(&[("weeks", 2), ("days", 3)]);
    assert_eq!(dur.as_unit("days"), 17); // 2*7 + 3
}

#[test]
fn test_from_object_singular() {
    let dur = Duration::from_object(&[("day", 1), ("hour", 2)]);
    assert_eq!(dur.as_unit("hours"), 26); // 24 + 2
}

#[test]
fn test_to_object() {
    let dur = Duration::from_object(&[("weeks", 1), ("days", 2), ("hours", 3)]);
    let obj = dur.to_object();
    assert_eq!(obj.get("weeks"), Some(&1));
    assert_eq!(obj.get("days"), Some(&2));
    assert_eq!(obj.get("hours"), Some(&3));
}

#[test]
fn test_to_object_zero_values() {
    let dur = Duration::from_object(&[("days", 5)]);
    let obj = dur.to_object();
    assert!(obj.get("weeks").is_none());
    assert!(obj.get("hours").is_none());
}

#[test]
fn test_as_unit_conversion() {
    let dur = Duration::from_object(&[("hours", 2)]);
    assert_eq!(dur.as_unit("minutes"), 120);
    assert_eq!(dur.as_unit("seconds"), 7200);
}

#[test]
fn test_as_unit_days_to_weeks() {
    let dur = Duration::from_object(&[("days", 14)]);
    assert_eq!(dur.as_unit("weeks"), 2);
}

#[test]
fn test_as_unit_complex() {
    let dur = Duration::from_object(&[("days", 1), ("hours", 12)]);
    assert_eq!(dur.as_unit("hours"), 36);
}

#[test]
fn test_negative_duration() {
    let dur = Duration::from_object(&[("days", -3)]);
    assert_eq!(dur.as_unit("days"), -3);
}

#[test]
fn test_all_units() {
    let dur = Duration::from_object(&[
        ("years", 1),
        ("months", 2),
        ("weeks", 3),
        ("days", 4),
        ("hours", 5),
        ("minutes", 6),
        ("seconds", 7),
        ("milliseconds", 8),
    ]);
    let obj = dur.to_object();
    assert_eq!(obj.len(), 8);
}

#[test]
fn test_milliseconds_precision() {
    let dur = Duration::from_object(&[("milliseconds", 1500)]);
    assert_eq!(dur.as_unit("seconds"), 1);
    assert_eq!(dur.as_unit("milliseconds"), 1500);
}

#[test]
fn test_large_values() {
    let dur = Duration::from_object(&[("years", 100)]);
    let days = dur.as_unit("days");
    assert!(days > 36000); // ~~~365 * 100
}
