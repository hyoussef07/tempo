use tempotime::{dt, DateTime, Duration, Interval};

fn main() {
    println!("=== Tempo Timezone Example ===\n");

    let utc_now = dt();
    println!("Current UTC time:");
    println!("  {}", utc_now.to_iso());
    println!("  {}", utc_now.to_format("EEEE, MMMM do yyyy, h:mm:ss a"));
    println!();

    #[cfg(feature = "tz")]
    {
        let zones = vec![
            "America/New_York",
            "Europe/London",
            "Asia/Tokyo",
            "Australia/Sydney",
        ];

        println!("Same moment in different timezones:");
        for zone in zones {
            let tz_time = dt().set_zone(zone);
            println!("  {}: {}", zone, tz_time.to_iso());
        }
        println!();

        let meeting = dt()
            .set_zone("Europe/Paris")
            .plus(&Duration::from_object(&[("weeks", 2)]))
            .start_of("day")
            .plus(&Duration::from_object(&[("hours", 15)]));

        println!("Meeting in 2 weeks at 3 PM Paris time:");
        println!(
            "  Paris: {}",
            meeting.to_format("EEEE, MMMM do 'at' h:mm a")
        );
        println!();
    }

    #[cfg(not(feature = "tz"))]
    {
        println!("⚠️  Timezone features disabled. Enable with --features tz");
        println!();
    }

    let start = dt();
    let end = start.clone().plus(&Duration::from_object(&[("days", 30)]));
    let interval = Interval::from_date_times(start.clone(), end);

    println!("Interval example (next 30 days):");
    println!("  Start: {}", start.to_format("MMM d, yyyy"));
    println!("  End: {}", interval.end().to_format("MMM d, yyyy"));
    println!("  Length: {} days", interval.length("days").as_unit("days"));

    let check_date = dt().plus(&Duration::from_object(&[("days", 15)]));
    println!("  Contains +15 days? {}", interval.contains(&check_date));
    let future = dt().plus(&Duration::from_object(&[("days", 100)]));
    println!("  Contains +100 days? {}", interval.contains(&future));
    println!();

    let past = DateTime::from_iso("2020-01-01T00:00:00Z").unwrap();
    let diff_days = utc_now.diff(&past, "days");
    let diff_years = utc_now.diff(&past, "years");
    println!("Time since 2020-01-01:");
    println!("  {:.0} days", diff_days);
    println!("  {:.2} years", diff_years);
}
