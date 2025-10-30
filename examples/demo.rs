use tempo::{dt, DateTime, Duration};

fn main() {
    #[cfg(feature = "tz")]
    {
        let ny = dt()
            .set_zone("America/New_York")
            .minus(&Duration::from_object(&[("weeks", 1)]))
            .end_of("day");

        println!("With timezone:");
        println!("  ISO: {}", ny.to_iso());
        println!("  Format: {}", ny.to_format("MMMM do, yyyy"));
        println!("  Locale: {}", ny.to_locale_string(DateTime::DATE_FULL));
        println!();
    }

    let result = dt()
        .plus(&Duration::from_object(&[("days", 3), ("hours", 2)]))
        .start_of("day")
        .to_format("yyyy-MM-dd HH:mm:ss");

    println!("Chainable demo:");
    println!("  3 days + 2 hours from now, start of day: {}", result);
    println!();

    let dur = Duration::from_object(&[("weeks", 2), ("days", 3)]);
    println!("Duration demo:");
    println!("  2 weeks + 3 days = {} days", dur.as_unit("days"));
    println!("  Object: {:?}", dur.to_object());
    println!();

    let now = dt();
    println!("Format tokens:");
    println!("  yyyy-MM-dd: {}", now.to_format("yyyy-MM-dd"));
    println!("  MMMM do, yyyy: {}", now.to_format("MMMM do, yyyy"));
    println!("  EEEE at h:mm a: {}", now.to_format("EEEE 'at' h:mm a"));
    println!();

    println!("Locale presets:");
    println!(
        "  DATE_SHORT: {}",
        now.to_locale_string(DateTime::DATE_SHORT)
    );
    println!("  DATE_MED: {}", now.to_locale_string(DateTime::DATE_MED));
    println!("  DATE_FULL: {}", now.to_locale_string(DateTime::DATE_FULL));
    println!(
        "  TIME_SIMPLE: {}",
        now.to_locale_string(DateTime::TIME_SIMPLE)
    );
    println!(
        "  DATETIME_SHORT: {}",
        now.to_locale_string(DateTime::DATETIME_SHORT)
    );
    println!();

    println!("Start/end of units:");
    println!(
        "  Start of month: {}",
        now.clone()
            .start_of("month")
            .to_format("yyyy-MM-dd HH:mm:ss")
    );
    println!(
        "  End of day: {}",
        now.clone()
            .end_of("day")
            .to_format("yyyy-MM-dd HH:mm:ss.SSS")
    );
}
