# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.15.1] - 2022-02-28

### Added

- `dotenv` crate forked as `dotenvy`
- `dotenv_codegen` forked as `dotenvy_codgen`
- `dotenv_codegen_implementation` forked as `dotenvy_codegen_impl`
- Crate description for dotenvy_codegen
- Crate description for dotenvy_codgen_impl
- New language in README
- MIT license badge in README
- Generate helpful errors from dotenv! macro (full merge of [dotenv-rs/dotenv #58](https://github.com/dotenv-rs/dotenv/pull/57/files#))

### Changed

- replaced deprecated `std::env_home:dir()` with `dirs:home_dir`
- Better handling of `home_dir` (merge of [dotenv-rs/dotenv #62](https://github.com/dotenv-rs/dotenv/pull/62/files#))
- assertions dealing with `Result` (based on [dotenv-rs/dotenv #57](https://github.com/dotenv-rs/dotenv/pull/57/files#))
- upgraded clap in `dotenvy` bin from v2 to v3.1 (covers [dotenv-rs/dotenv #76](https://github.com/dotenv-rs/dotenv/pull/76/files))

### Removed

- example folder. The simple example has been moved to the README.
- `extern`
- unnecessary `use` statements in doc examples

## [0.15.0] - 2019-10-21

### Changed

- Undeprecate `iter` methods
- Library no longer has any dependencies

### Added

- Support for variables with a `.` in their name
- Support `\n` in double-quoted lines
- Support for variable substitution

## [0.14.1] - 2019-05-14

### Changed

- Deprecate `iter` methods.

## [0.14.0] - 2019-05-07

### Changed

- Switched repo to use cargo workspaces.
- Renamed dotenv_codegen_impl to dotenv_codegen_implementation since we no longer own the original crate.
- Update code to 2018 edition

[unreleased]: https://github.com/dotenv-rs/dotenv/compare/v0.15.0...HEAD
[0.15.0]: https://github.com/dotenv-rs/dotenv/compare/v0.14.1...v0.15.0
[0.14.1]: https://github.com/dotenv-rs/dotenv/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/dotenv-rs/dotenv/compare/v0.13.0...v0.14.0
