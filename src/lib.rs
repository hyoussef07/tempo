//! # Tempotime
//!
//! **A Luxon.js-inspired datetime library for Rust with zero dependencies by default.**
//!
//! Tempotime provides a clean, immutable, and chainable API for working with dates and times in Rust.
//! Inspired by the elegant design of Luxon.js (the modern successor to Moment.js), it brings that
//! same developer-friendly experience to Rust while offering unique advantages like zero-dependency
//! operation for UTC-only use cases.
//!
//! ## 🌟 Key Features
//!
//! - **🚀 Zero dependencies by default** – UTC-only DateTime using only `std::time`
//! - **🔒 Immutable operations** – All methods return new instances, preventing bugs
//! - **⛓️ Chainable API** – Fluent interface for complex date operations
//! - **🌍 Optional timezone support** – Enable `tz` feature for full IANA timezone database
//! - **📝 Luxon-compatible formatting** – Familiar, intuitive token-based formatting
//! - **📦 Small footprint** – ~175KB binary in zero-deps mode vs ~2MB with full features
//! - **🔄 Serde support** – Optional JSON serialization with ISO-8601 strings
//! - **⚡ Fast compilation** – Minimal dependencies mean quick build times
//!
//! ## 🚀 Quick Start
//!
//! ```rust
//! use tempotime::{dt, Duration};
//!
//! // Get current time
//! let now = dt();
//!
//! // Chain operations fluently
//! let future = now
//!     .plus(&Duration::from_object(&[("weeks", 2), ("days", 3)]))
//!     .start_of("day");
//!
//! // Format output beautifully
//! println!("{}", future.to_format("MMMM do, yyyy 'at' h:mm a"));
//! // Output: "November 16th, 2025 at 12:00 am"
//! ```
//!
//! ## 📦 Installation
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! # Zero-deps mode (UTC only, smallest binary)
//! [dependencies]
//! tempotime = "0.1"
//!
//! # With timezone support (IANA timezone database)
//! [dependencies]
//! tempotime = { version = "0.1", features = ["tz"] }
//!
//! # With JSON serialization
//! [dependencies]
//! tempotime = { version = "0.1", features = ["serde"] }
//!
//! # All features enabled
//! [dependencies]
//! tempotime = { version = "0.1", features = ["tz", "serde"] }
//! ```
//!
//! ## 🎯 Feature Flags
//!
//! | Feature | Description | Binary Impact | Use When |
//! |---------|-------------|---------------|----------|
//! | `std` (default) | Standard library support | Base | Always enabled |
//! | `chrono` | Accurate month/year math | +~2MB | Need precise date arithmetic |
//! | `tz` | IANA timezone database | +~2MB | Need timezone conversions |
//! | `serde` | JSON serialization | +~100KB | Need to serialize/deserialize |
//!
//! ## 📚 Examples
//!
//! ### Basic DateTime Operations
//!
//! ```rust
//! use tempotime::{dt, DateTime, Duration};
//!
//! // Create DateTime instances
//! let now = DateTime::now();
//! let iso = DateTime::from_iso("2025-10-30T14:30:00Z").unwrap();
//! let formatted = DateTime::from_format("Oct 30, 2025", "MMM dd, yyyy").unwrap();
//!
//! // Add or subtract durations
//! let tomorrow = now.clone().plus(&Duration::from_object(&[("days", 1)]));
//! let last_week = now.clone().minus(&Duration::from_object(&[("weeks", 1)]));
//!
//! // Round to start/end of units
//! let start_of_day = now.clone().start_of("day");     // 00:00:00
//! let end_of_month = now.end_of("month");     // Last millisecond of month
//! ```
//!
//! ### Chainable Operations
//!
//! ```rust
//! use tempotime::{dt, Duration};
//!
//! // Complex operations in a single chain
//! let result = dt()
//!     .plus(&Duration::from_object(&[("days", 3), ("hours", 2)]))
//!     .start_of("day")
//!     .to_format("yyyy-MM-dd HH:mm:ss");
//! 
//! println!("3 days from now (start of day): {}", result);
//! ```
//!
//! ### Formatting Dates
//!
//! ```rust
//! use tempotime::dt;
//!
//! let now = dt();
//!
//! // Custom formatting with tokens
//! println!("{}", now.to_format("yyyy-MM-dd"));              // "2025-10-30"
//! println!("{}", now.to_format("MMMM do, yyyy"));           // "October 30th, 2025"
//! println!("{}", now.to_format("EEEE 'at' h:mm a"));        // "Wednesday at 2:30 pm"
//! println!("{}", now.to_format("yyyy-MM-dd'T'HH:mm:ss"));   // "2025-10-30T14:30:00"
//!
//! // Locale presets
//! use tempotime::DateTime;
//! println!("{}", now.to_locale_string(DateTime::DATE_SHORT));     // "10/30/2025"
//! println!("{}", now.to_locale_string(DateTime::DATE_FULL));      // "October 30, 2025"
//! println!("{}", now.to_locale_string(DateTime::TIME_SIMPLE));    // "2:30 pm"
//! println!("{}", now.to_locale_string(DateTime::DATETIME_MED));   // "Oct 30, 2025, 2:30 pm"
//! ```
//!
//! ### Working with Timezones
//!
//! ```rust
//! # #[cfg(feature = "tz")]
//! # {
//! use tempotime::dt;
//!
//! // Convert to different timezones
//! let utc = dt();
//! let ny = utc.clone().set_zone("America/New_York");
//! let tokyo = utc.clone().set_zone("Asia/Tokyo");
//!
//! println!("UTC:      {}", utc.to_format("h:mm a"));
//! println!("New York: {}", ny.to_format("h:mm a"));
//! println!("Tokyo:    {}", tokyo.to_format("h:mm a"));
//! # }
//! ```
//!
//! ### Durations
//!
//! ```rust
//! use tempotime::Duration;
//!
//! // Create durations with multiple units
//! let dur = Duration::from_object(&[
//!     ("weeks", 2),
//!     ("days", 3),
//!     ("hours", 4),
//! ]);
//!
//! // Convert to different units
//! println!("Total hours: {}", dur.as_unit("hours"));
//! println!("Total days:  {}", dur.as_unit("days"));
//!
//! // Export as object
//! let obj = dur.to_object();
//! ```
//!
//! ### Intervals
//!
//! ```rust
//! use tempotime::{dt, Duration, Interval};
//!
//! // Define a time interval
//! let start = dt();
//! let end = start.clone().plus(&Duration::from_object(&[("days", 30)]));
//! let interval = Interval::from_date_times(start, end);
//!
//! // Check if a datetime falls within the interval
//! let check = dt().plus(&Duration::from_object(&[("days", 15)]));
//! assert!(interval.contains(&check));
//!
//! // Get interval length
//! println!("Interval length: {} days", interval.length("days").as_unit("days"));
//! ```
//!
//! ### Time Differences
//!
//! ```rust
//! use tempotime::DateTime;
//!
//! let now = DateTime::now();
//! let past = DateTime::from_iso("2025-01-01T00:00:00Z").unwrap();
//!
//! // Calculate differences in various units
//! println!("Days ago: {:.0}", now.diff(&past, "days"));
//! println!("Hours ago: {:.1}", now.diff(&past, "hours"));
//! println!("Years ago: {:.2}", now.diff(&past, "years"));
//! ```
//!
//! ## 🎨 Format Tokens
//!
//! Tempotime supports Luxon.js-style formatting tokens:
//!
//! | Token | Output | Description |
//! |-------|--------|-------------|
//! | `yyyy` | 2025 | 4-digit year |
//! | `yy` | 25 | 2-digit year |
//! | `MMMM` | October | Full month name |
//! | `MMM` | Oct | Short month name |
//! | `MM` | 10 | 2-digit month |
//! | `M` | 10 | Month (no padding) |
//! | `dd` | 30 | 2-digit day |
//! | `d` | 30 | Day (no padding) |
//! | `do` | 30th | Day with ordinal suffix |
//! | `EEEE` | Wednesday | Full weekday name |
//! | `EEE` | Wed | Short weekday name |
//! | `HH` | 14 | 24-hour (padded) |
//! | `H` | 14 | 24-hour (no padding) |
//! | `hh` | 02 | 12-hour (padded) |
//! | `h` | 2 | 12-hour (no padding) |
//! | `mm` | 30 | Minutes (padded) |
//! | `ss` | 00 | Seconds (padded) |
//! | `SSS` | 123 | Milliseconds |
//! | `a` | pm | AM/PM lowercase |
//!
//! Escape literal text with single quotes: `'at'` → "at"
//!
//! ## ⚙️ Zero-Deps Mode
//!
//! By default, `tempotime` uses only `std::time::SystemTime` for UTC timestamps,
//! resulting in zero external dependencies.
//!
//! ### Advantages
//!
//! - ✅ Zero external dependencies
//! - ✅ Fast compilation (~2-3 seconds vs ~30 seconds with full features)
//! - ✅ Tiny binary footprint (~175KB vs ~2MB)
//! - ✅ Perfect for microservices, CLI tools, and embedded systems
//! - ✅ Full API compatibility (same methods work identically)
//!
//! ### Limitations
//!
//! - ⚠️ UTC only (`.set_zone()` is a no-op without `tz` feature)
//! - ⚠️ Month/year arithmetic uses approximations (30 days/month, 365 days/year)
//! - ⚠️ No DST (Daylight Saving Time) handling
//! - ⚠️ `.local()` returns UTC time
//!
//! ### When to Use
//!
//! Use zero-deps mode when:
//! - You only need UTC timestamps
//! - Binary size is critical
//! - Compilation speed matters
//! - You want minimal dependencies
//!
//! Enable `chrono` or `tz` features when:
//! - You need accurate month/year arithmetic
//! - Timezone conversions are required
//! - DST handling is important
//!
//! ## 🆚 Comparison with Other Libraries
//!
//! ### vs. `chrono`
//!
//! ```rust
//! // chrono (verbose, mutable)
//! # #[cfg(feature = "chrono")]
//! # {
//! use chrono::{Utc, Duration};
//! let dt = Utc::now()
//!     .checked_add_signed(Duration::days(3))
//!     .unwrap()
//!     .format("%Y-%m-%d")
//!     .to_string();
//! # }
//!
//! // tempotime (clean, immutable)
//! use tempotime::{dt, Duration};
//! let dt = dt()
//!     .plus(&Duration::from_object(&[("days", 3)]))
//!     .to_format("yyyy-MM-dd");
//! ```
//!
//! ### vs. `time`
//!
//! Tempotime provides:
//! - Immutable-by-default design
//! - Luxon-style formatting tokens
//! - Object-based duration syntax
//! - More intuitive chainable API
//! - Optional zero-dependency mode
//!
//! ## 🤝 Contributing
//!
//! Contributions are welcome! This is a community-driven port of Luxon.js to Rust.
//!
//! ## 📄 License
//!
//! Licensed under either of:
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/hyoussef07/tempotime/blob/main/LICENSE-APACHE))
//! - MIT license ([LICENSE-MIT](https://github.com/hyoussef07/tempotime/blob/main/LICENSE-MIT))
//!
//! at your option.
//!
//! ## 🙏 Inspiration
//!
//! This project is inspired by [Luxon.js](https://moment.github.io/luxon/), the modern
//! successor to Moment.js, bringing its elegant API design to the Rust ecosystem.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod datetime;
mod duration;
mod format;
mod interval;
mod locale;

pub use datetime::DateTime;
pub use duration::Duration;
pub use interval::Interval;

/// Convenience function to get the current DateTime.
///
/// Alias for [`DateTime::now()`].
///
/// # Examples
///
/// ```rust
/// use tempotime::dt;
///
/// let now = dt();
/// println!("Current time: {}", now.to_iso());
/// ```
pub fn dt() -> DateTime {
    DateTime::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dt_convenience() {
        let now = dt();
        assert!(now.to_iso().len() > 0);
    }

    #[test]
    fn test_chainable() {
        let dur = Duration::from_object(&[("days", 1)]);
        let result = dt().plus(&dur).start_of("day");
        assert!(result.to_iso().contains("T00:00:00"));
    }
}
