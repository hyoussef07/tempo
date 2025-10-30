use tempotime::DateTime;

#[test]
fn escaped_single_quote_roundtrip() {
    let input = "2025-10-30 05 o'clock am";
    let fmt = "yyyy-MM-dd hh 'o''clock' a"; // pattern contains escaped single-quote
    let dt = DateTime::from_format(input, fmt).expect("parse should succeed");
    assert_eq!(dt.to_format("yyyy-MM-dd"), "2025-10-30");
    assert_eq!(dt.to_format("hh"), "05");
}

#[test]
fn unterminated_literal_is_error() {
    // Use a format with no separating literal so the unterminated quote is reached immediately
    let res = DateTime::from_format("2025-10-30", "yyyy-MM-dd'unterminated");
    assert!(res.is_err());
    let e = res.err().unwrap();
    // print the error to help debugging and assert it mentions unterminated literal
    eprintln!("from_format error: {}", e);
    assert!(e.to_lowercase().contains("unterminated") || e.to_lowercase().contains("untertermin"));
}
