# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cross-compilation support for ARM architecture
- Mark tax payments as done/pending

### Changed

- SQLite backend enabled by default

## [0.1.1] - 2024-04-23

#### Fixed

- Windows support

## [0.1.0] - 2024-04-23

#### Added

- Initialize application configuration and storage
- Load tax rates and report parameters from configuration
- Group income reports by quarters
- Filter incomes and reports on quarters and years
- Generate reports in taxer format

[unreleased]: https://github.com/dimasmith/monotax/compare/0.1.1...HEAD
[0.1.1]: https://github.com/dimasmith/monotax/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/dimasmith/monotax/releases/tag/0.1.0