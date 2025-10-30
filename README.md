# tempotime

A Rust port of Luxon.js with immutable, chainable DateTime operations and IANA timezone support.

Zero external dependencies by default – only `std::time` for UTC operations. Optionally enable `chrono` for full timezone support.

```rust
use tempotime::{dt, Duration};

dt()
    .plus(&Duration::from_object(&[("days", 3)]))
    .start_of("day")
    .to_format("MMMM do, yyyy")
```

## Features

- **Zero dependencies by default** – UTC-only DateTime using `std::time`
- Immutable operations that return new `DateTime` instances
- Optional IANA timezone support via `chrono-tz`
- Object-based duration syntax
- Round to start/end of time units
- Luxon-compatible formatting tokens
- Built-in locale presets
- Small footprint (< 100KB binary in zero-deps mode)

## Installation

### Zero-Deps Mode (UTC only)

```toml
[dependencies]
tempotime = "0.1"
```

### With Timezone Support

```toml
[dependencies]
tempotime = { version = "0.1", features = ["chrono", "tz"] }
```

## Zero-Deps Mode

By default, `tempotime` uses only `std::time::SystemTime` for UTC timestamps, resulting in:

- **Tiny binary** – ~80KB vs ~2MB with chrono
- **Fast compilation** – No external dependencies
- **Full API** – All methods work identically

### Limitations

- UTC only (`.set_zone()` is a no-op)
- Month/year arithmetic uses approximations (30 days/month, 365 days/year)
- `.local()` returns UTC

### When to Use

- Microservices that only need UTC timestamps
- CLI tools with strict binary size requirements
- Projects that want minimal dependencies

### Feature Comparison

| Feature | Zero-Deps | `chrono` | `tz` |
|---------|-----------|----------|------|
| Binary Size | ~80KB | ~2MB | ~2MB |
| Dependencies | 0 | 1 | 2 |
| `.now()` | ✅ | ✅ | ✅ |
| `.from_iso()` | ✅ | ✅ | ✅ |
| `.to_format()` | ✅ | ✅ | ✅ |
| `.plus()/.minus()` | ✅ | ✅ | ✅ |
| Month/year math | ~30d/365d | Accurate | Accurate |
| `.set_zone()` | No-op | No-op | ✅ |
| Timezones | UTC only | UTC only | IANA |

## Usage

### Basic Operations

```rust
use tempotime::{dt, DateTime, Duration};

let now = dt();

let future = now
    .plus(&Duration::from_object(&[("weeks", 2), ("days", 3)]))
    .start_of("day");

println!("{}", future.to_format("yyyy-MM-dd"));
```

### Timezones

```rust
// Enable with --features tz
let ny_time = dt()
    .set_zone("America/New_York")
    .to_format("h:mm a");
```

### Formatting

```rust
let now = dt();

now.to_format("EEEE, MMMM do yyyy");

now.to_locale_string(DateTime::DATE_FULL);
now.to_locale_string(DateTime::TIME_SIMPLE);
now.to_locale_string(DateTime::DATETIME_SHORT);
```

### Format Tokens

| Token | Output | Description |
|-------|--------|-------------|
| `yyyy` | 2025 | 4-digit year |
| `yy` | 25 | 2-digit year |
| `MMMM` | October | Full month name |
| `MMM` | Oct | Short month name |
| `MM` | 10 | 2-digit month |
| `M` | 10 | Month (no padding) |
| `dd` | 29 | 2-digit day |
| `d` | 29 | Day (no padding) |
| `do` | 29th | Day with ordinal |
| `EEEE` | Wednesday | Full weekday |
| `EEE` | Wed | Short weekday |
| `HH` | 14 | 24-hour (padded) |
| `H` | 14 | 24-hour |
| `hh` | 02 | 12-hour (padded) |
| `h` | 2 | 12-hour |
| `mm` | 05 | Minutes (padded) |
| `ss` | 09 | Seconds (padded) |
| `SSS` | 123 | Milliseconds |
| `a` | pm | AM/PM |

### Durations

```rust
use tempotime::Duration;

let dur = Duration::from_object(&[
    ("weeks", 2),
    ("days", 3),
    ("hours", 4)
]);

dur.as_unit("days");
dur.as_unit("hours");
```

### Intervals

```rust
use tempotime::{dt, Duration, Interval};

let start = dt();
let end = start.clone().plus(&Duration::from_object(&[("days", 30)]));
let interval = Interval::from_date_times(start, end);

let check = dt().plus(&Duration::from_object(&[("days", 15)]));
interval.contains(&check);
interval.length("days").as_unit("days");
```

### Differences

```rust
let now = dt();
let past = DateTime::from_iso("2020-01-01T00:00:00Z").unwrap();

now.diff(&past, "days");
now.diff(&past, "years");
```

## Why tempotime?

### vs. `chrono`

```rust
// chrono (verbose)
let dt = Utc::now()
    .checked_add_signed(Duration::days(3))
    .unwrap()
    .format("%Y-%m-%d")
    .to_string();

// tempotime (clean)
let dt = dt()
    .plus(&Duration::from_object(&[("days", 3)]))
    .to_format("yyyy-MM-dd");
```

### vs. `time`

`tempotime` provides:
- Immutable by default
- Luxon-style formatting
- Object-based durations
- Chainable API

## API Reference

### `DateTime`

```rust
DateTime::now() -> Self
DateTime::local() -> Self
DateTime::from_iso(s: &str) -> Result<Self, String>
DateTime::from_format(s: &str, fmt: &str) -> Result<Self, String>

dt.set_zone(zone: &str) -> Self
dt.plus(dur: &Duration) -> Self
dt.minus(dur: &Duration) -> Self
dt.start_of(unit: &str) -> Self  // "year", "month", "day", "hour", "minute"
dt.end_of(unit: &str) -> Self

dt.to_iso() -> String
dt.to_format(fmt: &str) -> String
dt.to_locale_string(preset: &str) -> String

dt.diff(other: &DateTime, unit: &str) -> f64
```

### `Duration`

```rust
Duration::from_object(obj: &[(&str, i64)]) -> Self
dur.to_object() -> HashMap<String, i64>
dur.as_unit(unit: &str) -> i64
```

### `Interval`

```rust
Interval::from_date_times(start: DateTime, end: DateTime) -> Self
interval.contains(dt: &DateTime) -> bool
interval.length(unit: &str) -> Duration
interval.start() -> &DateTime
interval.end() -> &DateTime
```

### Convenience

```rust
dt() -> DateTime  // Alias for DateTime::now()
```

## Features

All features are optional:

```toml
[features]
default = []
chrono = ["dep:chrono"]
tz = ["chrono", "chrono-tz"]
serde = ["dep:serde", "chrono?/serde"]
```

Enable features as needed:

```toml
# Zero-deps (default)
tempotime = "0.1"

# With chrono (accurate month/year math, still UTC-only)
tempotime = { version = "0.1", features = ["chrono"] }

# With timezones
tempotime = { version = "0.1", features = ["tz"] }

# With serialization
tempotime = { version = "0.1", features = ["serde"] }
```

## Examples

```bash
cargo run --example demo
cargo run --example timezone --features tz
```

## Testing

```bash
cargo test
cargo test --features tz
cargo bench
```

## Contributing

Contributions are welcome. This is a community-driven port of Luxon.js to Rust.

1. Fork the repo
2. Create a feature branch
3. Add tests for new features
4. Submit a PR

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Inspiration

This project is a Rust port of [Luxon.js](https://moment.github.io/luxon/), the modern successor to Moment.js.

## Links

- [Luxon.js Documentation](https://moment.github.io/luxon/)
- [GitHub Repository](https://github.com/hyoussef07/tempo)
