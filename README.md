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

### Advanced

- `std` feature: the crate now exposes a `std` Cargo feature (enabled by default) so downstream crates can opt out of `std` in the future; full `no_std` + `alloc` support will be progressively added and gated behind this feature.
- `serde` feature: when enabled, `DateTime` serializes/deserializes as ISO-8601 strings (useful for JSON interchange).

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
### When to Use

- Microservices that only need UTC timestamps

### Feature Comparison

| Feature | Zero-Deps | `chrono` | `tz` |
|---------|-----------|----------|------|
| Binary Size | ~80KB | ~2MB | ~2MB |
| Dependencies | 0 | 1 | 2 |
| `.now()` | ✅ | ✅ | ✅ |
| `.from_iso()` | ✅ | ✅ | ✅ |
Features

Default and optional features are provided to let you pick the implementation and dependencies you want. The crate ships with the `std` feature enabled by default to preserve the usual std behaviour; you can opt-out when preparing a `no_std` build.

Enable features from your Cargo.toml:

```toml
[dependencies.tempotime]
version = "0.1"
# default includes "std"; add optional features as needed
features = ["tz", "serde"]
```

- std (enabled by default): enable `std`-based helpers such as `now()` and `SystemTime` integration; also keeps the normal std `String`/IO conveniences. For `no_std` builds, remove this from the default features and add `alloc` support as needed.
- chrono (optional): use the `chrono` crate for a richer DateTime backend; falls back to a compact timestamp-based implementation when disabled.
- tz (optional): enable `chrono_tz` timezone support (requires `chrono`). When disabled the crate provides a tiny builtin static zone lookup (fixed offsets only, no DST).
- serde (optional): enable `serde` Serialize / Deserialize implementations for `DateTime` (ISO-8601 string representation).
let now = dt();

let future = now
    .plus(&Duration::from_object(&[("weeks", 2), ("days", 3)]))
```

### Timezones
let ny_time = dt()
    .set_zone("America/New_York")
    .to_format("h:mm a");

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

Zero-deps / small-std builds

When the `chrono` feature is not enabled the crate uses a compact timestamp-backed implementation that has no external runtime dependency. It provides accurate Gregorian calendar arithmetic (leap years, month/day clamping), the `from_format` parser and zero-allocation `format_into` helpers.

Timezone behavior in zero-deps mode: `set_zone()` now applies a fixed offset (seconds east of UTC) for a small set of common zones (case-insensitive lookup) as a convenience. This builtin mapping is intentionally minimal and does not implement DST or historical offset rules — for full timezone and DST behavior enable the `tz` feature.
### Durations

```rust
use tempotime::Duration;

let dur = Duration::from_object(&[

Examples

Basic usage (zero-deps):

```rust
use tempotime::DateTime;

let dt = DateTime::from_iso("2024-02-29T12:00:00Z").unwrap();
let mut buf = String::new();
dt.format_into(&mut buf, "yyyy-MM-dd'T'HH:mm:ss'Z'").unwrap();
println!("{}", buf);
```

If you need full timezone/DST handling enable the `chrono` + `tz` features in Cargo.toml:

```toml
[dependencies.tempotime]
version = "0.1"
features = ["chrono", "tz"]
```

```rust
use tempotime::DateTime;

let dt = DateTime::now();
let dt_ny = dt.clone().set_zone("America/New_York");
let mut b = String::new();
dt.format_into(&mut b, "yyyy-MM-dd HH:mm").unwrap();
let mut b2 = String::new();
dt_ny.format_into(&mut b2, "yyyy-MM-dd HH:mm").unwrap();
println!("local: {} | ny: {}", b, b2);
```
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
- [GitHub Repository](https://github.com/hyoussef07/tempotime)
