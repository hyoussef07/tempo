# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2025-10-30

### Added
- Comprehensive crate-level documentation with examples and feature comparisons
- Enhanced API documentation for all public types and methods
- Professional README with badges, feature tables, and detailed usage examples
- Examples demonstrating zero-deps mode, timezone support, and advanced usage

### Changed
- Improved documentation structure and clarity throughout the crate
- Updated README with better visual formatting and organization

## [0.1.2] - 2025-10-30

### Added
- **`std` feature flag**: Enabled by default to allow future `no_std` + `alloc` support
  - Provides foundation for optional standard library usage
  - Enables progressive addition of no_std compatibility
  - No breaking changes for existing users (enabled by default)

- **Serde support**: Optional serialization/deserialization behind `serde` feature
  - DateTime serializes/deserializes as stable ISO-8601 strings
  - JSON-friendly format for web APIs and data interchange
  - Compatible with both zero-deps and chrono backends

- **Improved zero-deps timezone handling**: Compact builtin timezone lookup
  - Case-insensitive matching for ~10 common IANA timezone names
  - Fixed offset support (no DST) for simple use cases
  - Zero heap allocations for timezone name lookups
  - Covers: UTC, Europe/London, America/New_York, America/Los_Angeles, Europe/Paris, Asia/Tokyo, Asia/Shanghai, Australia/Sydney, Asia/Kolkata, America/Sao_Paulo

- **Enhanced `from_format` literal parsing**
  - Support for escaped single quotes (`''` → `'`)
  - Clearer error messages for unterminated or mismatched literals
  - Better handling of literal text in format strings

- **Zero-allocation formatting API**: New `format_into` method
  - Write formatted output directly to any `fmt::Write` impl
  - Avoids heap allocation beyond the writer's buffer
  - Useful for performance-critical paths and embedded systems

### Fixed
- Reduced compiler warnings throughout the codebase
- Improved error messages for parsing failures
- Fixed edge cases in format string parsing

### Documentation
- Added comprehensive crate-level documentation
- Documented all public API surfaces with examples
- Created detailed feature comparison tables
- Added migration guide for zero-deps mode

## [0.1.1] - 2025-10-30

### Added
- Locale presets for common date/time formats
  - `DATE_SHORT`: "10/30/2025"
  - `DATE_MED`: "Oct 30, 2025"
  - `DATE_FULL`: "October 30, 2025"
  - `TIME_SIMPLE`: "2:30 pm"
  - `TIME_WITH_SECONDS`: "2:30:00 pm"
  - `DATETIME_SHORT`: "10/30/2025, 2:30 pm"
  - `DATETIME_MED`: "Oct 30, 2025, 2:30 pm"
  - `DATETIME_FULL`: "October 30, 2025, 2:30 pm"

- Comprehensive test suite with 63+ test cases
- Examples demonstrating both zero-deps and full-featured usage
- Benchmarks for performance testing

### Performance
- Zero-deps binary size: ~175KB (measured)
- With timezone data: ~2MB (measured)
- Fast compilation in zero-deps mode (~2-3 seconds)

### Documentation
- Comprehensive README with feature comparison table
- API documentation for all public types and methods
- Migration guide for users transitioning from zero-deps to full features
- Examples for common use cases

## [0.1.0] - 2025-10-29

### Added
- **Initial release** of Tempotime - A Luxon.js-inspired datetime library for Rust

- **Zero-dependency mode (default)**
  - UTC-only DateTime operations using only `std::time::SystemTime`
  - No external crate dependencies
  - Tiny binary footprint (~175KB)
  - Fast compilation times

- **Optional chrono backend** (`chrono` feature)
  - Accurate month and year arithmetic
  - Proper calendar calculations
  - Still UTC-only without `tz` feature

- **Optional IANA timezone support** (`tz` feature)
  - Full timezone database via chrono-tz
  - DST (Daylight Saving Time) handling
  - 600+ IANA timezone identifiers

- **Core DateTime API**
  - `DateTime::now()` - Current UTC time
  - `DateTime::local()` - Current local time (as UTC)
  - `DateTime::from_iso()` - Parse ISO 8601 strings
  - `DateTime::from_format()` - Parse custom format strings
  - `.plus()` / `.minus()` - Add/subtract durations
  - `.start_of()` / `.end_of()` - Round to time units
  - `.set_zone()` - Convert timezones (with `tz` feature)
  - `.diff()` - Calculate differences between DateTimes
  - `.to_iso()` - Export as ISO 8601
  - `.to_format()` - Custom formatting with tokens
  - `.to_locale_string()` - Locale-based formatting

- **Duration API**
  - `Duration::from_object()` - Create from key-value pairs
  - `.to_object()` - Export as HashMap
  - `.as_unit()` - Convert to specific unit
  - Support for: years, months, weeks, days, hours, minutes, seconds, milliseconds

- **Interval API**
  - `Interval::from_date_times()` - Create time range
  - `.contains()` - Check if DateTime falls within range
  - `.length()` - Calculate interval duration
  - `.start()` / `.end()` - Access interval boundaries

- **Luxon-style formatting tokens**
  - Year: `yyyy`, `yy`
  - Month: `MMMM`, `MMM`, `MM`, `M`
  - Day: `dd`, `d`, `do` (with ordinal)
  - Weekday: `EEEE`, `EEE`
  - Hour: `HH`, `H` (24-hour), `hh`, `h` (12-hour)
  - Minute: `mm`, `m`
  - Second: `ss`, `s`
  - Millisecond: `SSS`
  - AM/PM: `a`
  - Literal text: `'text'`

- **Immutable design**
  - All DateTime operations return new instances
  - Safe to share across threads
  - Prevents common date manipulation bugs

- **Chainable API**
  - Fluent interface for complex operations
  - Readable, expressive code
  - Inspired by Luxon.js patterns

### Documentation
- Complete API documentation
- README with quick start guide
- Multiple examples demonstrating features
- Feature comparison table

### Performance
- Zero external dependencies in default configuration
- Fast compilation (~2-3 seconds in zero-deps mode)
- Small binary size (~175KB without timezone data)
- Efficient UTC operations using standard library only

---

## Release Notes Legend

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes
- **Performance**: Performance improvements
- **Documentation**: Documentation changes

---

[Unreleased]: https://github.com/hyoussef07/tempotime/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/hyoussef07/tempotime/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/hyoussef07/tempotime/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/hyoussef07/tempotime/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/hyoussef07/tempotime/releases/tag/v0.1.0
