use tempotime::DateTime;

#[test]
fn parse_human_readable_with_ordinal_and_am_pm() {
    let input = "October 31st, 2025 at 3 pm";
    let pattern = "MMMM do, yyyy 'at' h a";
    let dt = DateTime::from_format(input, pattern).expect("parse");
    // Expect 2025-10-31 15:00:00
    let out = dt.to_format("yyyy-MM-dd HH:mm:ss");
    assert_eq!(out, "2025-10-31 15:00:00");
}
