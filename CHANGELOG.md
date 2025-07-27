# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions workflows for CI/CD
  - Comprehensive CI pipeline with testing on multiple platforms
  - Security audit workflow
  - Automated release workflow
- CONTRIBUTING.md with contribution guidelines
- Integration with datalogic-rs custom operator feature
- Optimized variable generation using preserve_structure mode

### Changed
- Moved `fake_operator_handler` from engine.rs to operators/fake.rs for better code organization
- Updated module exports for cleaner API
- Improved documentation with clearer architecture explanation
- Enhanced README with badges and updated installation instructions

### Fixed
- Fixed all clippy warnings for better code quality
- Improved error handling consistency

## [0.1.0] - 2024-01-15

### Added
- Initial release of datafake-rs
- JSONLogic-based configuration system
- 50+ fake data generation methods
- Variable system for reusable values
- Batch generation support
- Comprehensive test suite
- Full documentation with examples
- Apache 2.0 license

### Features
- Numeric types (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64)
- Personal data (names, emails, phone numbers)
- Address data (streets, cities, countries, coordinates)
- Financial data (BIC, credit cards, currencies)
- Internet data (IPs, MACs, usernames, passwords)
- Company data (names, industries, professions)
- Content generation (words, sentences, paragraphs)
- Unique identifiers (UUIDs)
- Barcodes (ISBN10, ISBN13)
- File system paths

[Unreleased]: https://github.com/GoPlasmatic/datafake-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/GoPlasmatic/datafake-rs/releases/tag/v0.1.0