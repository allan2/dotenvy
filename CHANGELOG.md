# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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



[Unreleased]: https://github.com/dotenv-rs/dotenv/compare/v0.15.0...HEAD
[0.15.0]: https://github.com/dotenv-rs/dotenv/compare/v0.14.1...v0.15.0
[0.14.1]: https://github.com/dotenv-rs/dotenv/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/dotenv-rs/dotenv/compare/v0.13.0...v0.14.0