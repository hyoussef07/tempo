# ⏰ Tempotime

[![Crates.io](https://img.shields.io/crates/v/tempotime.svg)](https://crates.io/crates/tempotime)
[![Documentation](https://docs.rs/tempotime/badge.svg)](https://docs.rs/tempotime)
[![License](https://img.shields.io/crates/l/tempotime.svg)](https://github.com/hyoussef07/tempotime#license)
[![Build Status](https://img.shields.io/github/actions/workflow/status/hyoussef07/tempotime/ci.yml?branch=main)](https://github.com/hyoussef07/tempotime/actions)

**A Luxon.js-inspired datetime library for Rust with zero dependencies by default.**

Tempotime brings the elegant, immutable, and chainable API of [Luxon.js](https://moment.github.io/luxon/) to Rust, while offering unique advantages like optional zero-dependency operation for UTC-only use cases.

```rust
use tempotime::{dt, Duration};

let result = dt()
    .plus(&Duration::from_object(&[("weeks", 2), ("days", 3)]))
    .start_of("day")
    .to_format("MMMM do, yyyy 'at' h:mm a");

println!("{}", result);
// Output: "November 16th, 2025 at 12:00 am"
```

---

## ✨ Features

### 🚀 **Zero Dependencies by Default**
Use only `std::time` for UTC operations – no external crates required. Perfect for microservices, CLI tools, and fast compilation.

### 🔒 **Immutable by Design**
All operations return new instances, preventing common date manipulation bugs.

### ⛓️ **Fluent & Chainable**
Write clean, readable date manipulation code with method chaining.

### 🌍 **Optional Timezone Support**
Enable IANA timezone database when you need it with the `tz` feature.

### 📝 **Luxon-Compatible Formatting**
Familiar, intuitive token-based formatting inspired by Luxon.js.

### 📦 **Tiny Binary Footprint**
- **Zero-deps mode**: ~175 KB
- **With timezone support**: ~2 MB

---

## 📥 Installation

Add to your `Cargo.toml`:

```toml
# Zero-deps mode (UTC only, minimal binary size)
[dependencies]
tempotime = "0.1"
```

### Feature Flags

```toml
# Accurate month/year arithmetic
tempotime = { version = "0.1", features = ["chrono"] }

# Full IANA timezone support
tempotime = { version = "0.1", features = ["tz"] }

# JSON serialization
tempotime = { version = "0.1", features = ["serde"] }

# All features
tempotime = { version = "0.1", features = ["tz", "serde"] }
```

---

## 🚀 Quick Start

```rust
use tempotime::{dt, DateTime, Duration};

// Get current time
let now = dt();

// Parse from ISO 8601
let date = DateTime::from_iso("2025-10-30T14:30:00Z").unwrap();

// Add/subtract durations
let tomorrow = now.plus(&Duration::from_object(&[("days", 1)]));
let last_week = now.minus(&Duration::from_object(&[("weeks", 1)]));

// Format output
println!("{}", tomorrow.to_format("yyyy-MM-dd HH:mm:ss"));
```

### Chainable Operations

```rust
use tempotime::{dt, Duration};

let result = dt()
    .plus(&Duration::from_object(&[("days", 3), ("hours", 2)]))
    .start_of("day")
    .to_format("EEEE, MMMM do");

println!("{}", result);  // "Saturday, November 2nd"
```

---

## 📖 Core API

### DateTime

```rust
// Creation
DateTime::now()                                        // Current UTC time
DateTime::from_iso("2025-10-30T14:30:00Z")           // Parse ISO 8601
DateTime::from_format("Oct 30, 2025", "MMM dd, yyyy") // Parse custom format

// Manipulation
dt.plus(&Duration::from_object(&[("days", 7)]))      // Add duration
dt.minus(&Duration::from_object(&[("hours", 3)]))    // Subtract duration
dt.start_of("day")                                    // Round down
dt.end_of("month")                                    // Round up
dt.set_zone("America/New_York")                       // Convert timezone

// Formatting
dt.to_iso()                                           // ISO 8601 string
dt.to_format("yyyy-MM-dd")                           // Custom format
dt.to_locale_string(DateTime::DATE_FULL)             // Locale preset

// Comparison
dt.diff(&other, "days")                               // Difference in days
dt > other_dt                                         // Compare dates
```

### Duration

```rust
let dur = Duration::from_object(&[
    ("weeks", 2),
    ("days", 3),
    ("hours", 4),
]);

dur.as_unit("days")       // Convert to days
dur.to_object()           // Export as HashMap
```

### Interval

```rust
use tempotime::{dt, Duration, Interval};

let start = dt();
let end = start.clone().plus(&Duration::from_object(&[("days", 30)]));
let interval = Interval::from_date_times(start, end);

interval.contains(&dt())                              // Check if in range
interval.length("days").as_unit("days")               // Get length
```

---

## 🎨 Format Tokens

| Token | Output | Description |
|-------|--------|-------------|
| `yyyy` | 2025 | 4-digit year |
| `yy` | 25 | 2-digit year |
| `MMMM` | October | Full month name |
| `MMM` | Oct | Short month name |
| `MM` | 10 | 2-digit month |
| `dd` | 30 | 2-digit day |
| `do` | 30th | Day with ordinal |
| `EEEE` | Wednesday | Full weekday |
| `EEE` | Wed | Short weekday |
| `HH` | 14 | 24-hour (padded) |
| `hh` | 02 | 12-hour (padded) |
| `mm` | 30 | Minutes |
| `ss` | 05 | Seconds |
| `SSS` | 123 | Milliseconds |
| `a` | pm | AM/PM |
| `'text'` | text | Literal text |

### Examples

```rust
let dt = dt();

dt.to_format("yyyy-MM-dd");                    // "2025-10-30"
dt.to_format("MMMM do, yyyy");                 // "October 30th, 2025"
dt.to_format("EEEE 'at' h:mm a");              // "Wednesday at 2:30 pm"
```

---

## 🎯 Zero-Deps Mode

By default, Tempotime uses only `std::time::SystemTime` for UTC timestamps.

### ✅ Advantages

- Zero external dependencies
- Fast compilation (~2-3 seconds)
- Tiny binary (~175 KB)
- Full API compatibility

### ⚠️ Limitations

- UTC only
- Approximate month/year math
- No DST support
- `.local()` returns UTC

### When to Upgrade

Enable features when you need:
- ✓ Accurate month/year arithmetic (`chrono`)
- ✓ Timezone conversions (`tz`)
- ✓ DST handling (`tz`)

---

## 🆚 Comparison with Other Libraries

### vs. `chrono`

**chrono**:
```rust
use chrono::{Utc, Duration};

let dt = Utc::now()
    .checked_add_signed(Duration::days(3))
    .unwrap()
    .format("%Y-%m-%d")
    .to_string();
```

**tempotime**:
```rust
use tempotime::{dt, Duration};

let dt = dt()
    .plus(&Duration::from_object(&[("days", 3)]))
    .to_format("yyyy-MM-dd");
```

### vs. `time`

Tempotime provides:
- ✓ Immutable-by-default design
- ✓ Luxon-style formatting
- ✓ Object-based durations
- ✓ More chainable API
- ✓ Optional zero-dependency mode

---

## 📊 Feature Comparison

| Feature | Zero-Deps | `chrono` | `tz` |
|---------|-----------|----------|------|
| Binary Size | ~175 KB | ~2 MB | ~2 MB |
| Dependencies | 0 | 1 | 2 |
| Compilation | ~2-3s | ~15-20s | ~25-30s |
| UTC Operations | ✅ | ✅ | ✅ |
| Month/Year Math | ~30d/365d | Accurate | Accurate |
| Timezones | UTC only | UTC only | IANA (600+) |
| DST Support | ❌ | ❌ | ✅ |

---

## 📚 Examples

Run the included examples:

```bash
# Basic demo
cargo run --example demo

# Timezone example
cargo run --example timezone --features tz

# Zero-deps demonstration  
cargo run --example zero_deps
```

---

## 🧪 Testing

```bash
# Run tests (zero-deps mode)
cargo test

# Run tests with all features
cargo test --all-features

# Run benchmarks
cargo bench
```

---

## 🤝 Contributing

Contributions are welcome! This is a community-driven port of Luxon.js to Rust.

1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Submit a pull request

---

## 📜 License

Licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))
- **MIT license** ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## 🙏 Acknowledgments

This project is inspired by **[Luxon.js](https://moment.github.io/luxon/)**, the modern successor to Moment.js.

---

## 🔗 Links

- **📦 Crates.io**: https://crates.io/crates/tempotime
- **📖 Documentation**: https://docs.rs/tempotime
- **🐙 GitHub**: https://github.com/hyoussef07/tempotime
- **🌟 Luxon.js**: https://moment.github.io/luxon/

---

<div align="center">

**⭐ If you find Tempotime useful, please consider giving it a star on GitHub! ⭐**

Made with ❤️ for the Rust community

</div>