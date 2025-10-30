# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

- Added `std` Cargo feature (enabled by default) to allow future `no_std` support and API gating.
- Implemented serde support for `DateTime` (serialize/deserialize as ISO-8601 strings) behind the `serde` feature.
- Reworked zero-deps timezone helper to a compact, case-insensitive lookup for common zones (no DST).
- Polished `from_format` literal parsing (supports escaped single-quote and clearer errors).

## 0.1.2

- Previous release notes...
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2025-10-30

### Added
- Comprehensive crate-level documentation
- Examples in all public API documentation
- Formatted tables and feature descriptions
- docs.rs metadata configuration

## [0.1.1] - 2025-10-30


# Changelog

All notable changes to this project are recorded here.

## 0.1.3 - 2025-10-30

Highlights

- Added a `std` Cargo feature (enabled by default) and scaffolding for future `no_std`/`alloc` support. This lets downstream crates opt out of the Rust standard library when they are ready.
- Implemented Serde support for `DateTime` behind the `serde` feature: DateTime now serializes and deserializes as stable ISO-8601 strings (JSON-friendly).
- Improved zero-deps timezone handling: a compact, case-insensitive builtin lookup maps ~10 common IANA names to fixed offsets (seconds east of UTC). This is a convenience (no DST) for small use-cases.
- Polished `DateTime::from_format` literal parsing: supports escaped single-quote ('' -> ') and now returns clearer errors for unterminated or mismatched literals.
- Continued cleanup: reduced warnings, added tests covering new behaviors, and documented the zero-alloc `format_into` API.

## 0.1.2

- Previous release notes...

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

[0.1.2]: https://github.com/hyoussef07/tempotime/releases/tag/v0.1.2
[0.1.1]: https://github.com/hyoussef07/tempotime/releases/tag/v0.1.1
[0.1.0]: https://github.com/hyoussef07/tempotime/releases/tag/v0.1.0