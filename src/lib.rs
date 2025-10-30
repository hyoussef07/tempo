//! # Tempotime
//!
//! A Luxon.js-inspired datetime library for Rust with zero dependencies by default.
//!
//! ## Features
//!
//! - **Zero dependencies by default** – UTC-only DateTime using `std::time`
//! - **Immutable operations** – All methods return new instances
//! - **Chainable API** – Fluent interface for complex operations
//! - **Optional timezone support** – Enable `tz` feature for IANA timezones
//! - **Luxon-compatible formatting** – Familiar token-based formatting
//! - **Small footprint** – ~175KB binary in zero-deps mode
//!
//! ## Quick Start
//!
//! ```rust
//! use tempotime::{dt, Duration};
//!
//! // Get current time
//! let now = dt();
//!
//! // Add durations
//! let future = now.plus(&Duration::from_object(&[
//!     ("weeks", 2),
//!     ("days", 3),
//! ]));
//!
//! // Format output
//! println!("{}", future.to_format("MMMM do, yyyy 'at' h:mm a"));
//! // Output: November 16th, 2025 at 2:30 pm
//! ```
//!
//! ## Feature Flags
//!
//! - `chrono` – Enable accurate month/year arithmetic using chrono
//! - `tz` – Enable IANA timezone support via chrono-tz
//! - `serde` – Enable serialization support
//!
//! ```toml
//! # Zero-deps (default)
//! [dependencies]
//! tempotime = "0.1"
//!
//! # With timezones
//! [dependencies]
//! tempotime = { version = "0.1", features = ["tz"] }
//! ```
//!
//! ## Examples
//!
//! ### Chainable Operations
//!
//! ```rust
//! use tempotime::{dt, Duration};
//!
//! let result = dt()
//!     .plus(&Duration::from_object(&[("days", 3)]))
//!     .start_of("day")
//!     .to_format("yyyy-MM-dd HH:mm:ss");
//! ```
//!
//! ### Parsing ISO Strings
//!
//! ```rust
//! use tempotime::DateTime;
//!
//! let dt = DateTime::from_iso("2025-10-30T14:30:00Z").unwrap();
//! assert_eq!(dt.to_format("yyyy-MM-dd"), "2025-10-30");
//! ```
//!
//! ### Working with Intervals
//!
//! ```rust
//! use tempotime::{dt, Duration, Interval};
//!
//! let start = dt();
//! let end = start.clone().plus(&Duration::from_object(&[("days", 30)]));
//! let interval = Interval::from_date_times(start, end);
//!
//! let check = dt().plus(&Duration::from_object(&[("days", 15)]));
//! assert!(interval.contains(&check));
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
//! let days_diff = now.diff(&past, "days");
//! let hours_diff = now.diff(&past, "hours");
//! ```
//!
//! ## Format Tokens
//!
//! | Token | Output | Description |
//! |-------|--------|-------------|
//! | `yyyy` | 2025 | 4-digit year |
//! | `MMMM` | October | Full month name |
//! | `dd` | 30 | 2-digit day |
//! | `do` | 30th | Day with ordinal |
//! | `EEEE` | Wednesday | Full weekday |
//! | `HH` | 14 | 24-hour (padded) |
//! | `hh` | 02 | 12-hour (padded) |
//! | `mm` | 30 | Minutes |
//! | `ss` | 00 | Seconds |
//! | `a` | pm | AM/PM |
//!
//! ## Zero-Deps Mode
//!
//! By default, `tempotime` uses only `std::time::SystemTime` for UTC timestamps.
//!
//! **Limitations:**
//! - UTC only (no timezones)
//! - Month/year arithmetic uses approximations
//! - No DST handling
//!
//! **Advantages:**
//! - Zero external dependencies
//! - Fast compilation
//! - Tiny binary (~175KB)
//!
//! Enable the `chrono` feature for accurate date arithmetic.

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
