# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-30

### Added
- Zero-dependency mode (default): Use `std::time::SystemTime` for UTC operations without external dependencies
- Optional `chrono` feature for accurate date arithmetic (month/year operations)
- Optional `tz` feature for IANA timezone support via chrono-tz
- Optional `serde` feature for serialization support
- Luxon.js-inspired API with chainable, immutable DateTime operations
- `DateTime` type with methods: `now()`, `from_iso()`, `plus()`, `minus()`, `start_of()`, `end_of()`, `diff()`
- `Duration` type with object-based API: `from_object()`, `to_object()`, `as_unit()`, `components()`
- `Interval` type for working with time ranges
- Format tokens support: `yyyy`, `MMMM`, `dd`, `do` (ordinals), `EEEE`, `HH`, `hh`, `mm`, `ss`, `SSS`, `a`
- Locale presets: `DATE_SHORT`, `DATE_MED`, `DATE_FULL`, `TIME_SIMPLE`, `DATETIME_SHORT`, etc.
- Comprehensive test suite with 63+ tests
- Examples demonstrating zero-deps and full-featured usage
- Binary size: ~175KB (zero-deps) vs ~2MB (with timezone data)

### Features
- **Zero-Dependency Mode**: Default configuration with no external dependencies
- **Dual Backend**: Seamlessly switch between SystemTime and chrono implementations
- **UTC Focus**: Zero-deps mode provides UTC operations only
- **Feature Flags**: Granular control over dependencies and functionality

### Documentation
- Comprehensive README with feature comparison table
- API documentation for all public types and methods
- Migration guide for zero-deps mode
- Examples for common use cases

[0.1.0]: https://github.com/hyoussef07/tempotime/releases/tag/v0.1.0