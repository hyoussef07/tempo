use tempotime::DateTime;

#[test]
fn roundtrip_full_month_and_ordinal() {
    let s = "October 31st, 2025 at 03:05 pm";
    let pattern = "MMMM do, yyyy 'at' hh:mm a";
    let dt = DateTime::from_format(s, pattern).expect("parse");
    assert_eq!(dt.to_format("yyyy-MM-dd HH:mm:ss"), "2025-10-31 15:05:00");
}

#[test]
fn roundtrip_short_month_and_24h() {
    let s = "Oct 05 2025 07:09:03";
    let pattern = "MMM dd yyyy HH:mm:ss";
    let dt = DateTime::from_format(s, pattern).expect("parse");
    assert_eq!(dt.to_format("yyyy-MM-dd HH:mm:ss"), "2025-10-05 07:09:03");
}

#[test]
fn roundtrip_numeric_month_and_millis() {
    let s = "2025-12-31 23:59:59.123";
    let pattern = "yyyy-MM-dd HH:mm:ss.SSS";
    let dt = DateTime::from_format(s, pattern).expect("parse");
    assert_eq!(dt.to_format("yyyy-MM-dd HH:mm:ss.SSS"), "2025-12-31 23:59:59.123");
}

#[test]
fn roundtrip_two_digit_year() {
    let s = "25-01-01 00:00";
    let pattern = "yy-MM-dd HH:mm";
    let dt = DateTime::from_format(s, pattern).expect("parse");
    // two-digit year assumed to be 2000+
    assert_eq!(dt.to_format("yyyy-MM-dd HH:mm"), "2025-01-01 00:00");
}

 