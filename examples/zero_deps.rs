use tempotime::{dt, Duration};

fn main() {
    println!("=== Tempotime Zero-Deps Demo ===\n");

    let now = dt();
    println!("Now (UTC): {}", now.to_iso());
    println!("Formatted: {}", now.to_format("MMMM do, yyyy - h:mm a"));

    let tomorrow = now.clone().plus(&Duration::from_object(&[("days", 1)]));
    println!("\nTomorrow: {}", tomorrow.to_format("yyyy-MM-dd"));

    let start = now.clone().start_of("day");
    println!("Start of day: {}", start.to_iso());

    let end = now.clone().end_of("day");
    println!("End of day: {}", end.to_iso());

    let past = dt().minus(&Duration::from_object(&[("days", 7)]));
    let diff = now.diff(&past, "days");
    println!("\nDays since last week: {:.1}", diff);

    println!("\n✅ All operations work with zero external dependencies!");
    println!("Binary size: ~80KB (vs ~2MB with chrono)");
}
