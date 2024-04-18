# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- MSRV updated to 1.68.0

### Added

- option_dotenvy! macro ([PR #99](https://github.com/allan2/dotenvy/pull/99)) by [aidenfarley](https://github.com/aidenfarley)

### Changed
- update to 2021 edition
- update MSRV to 1.74.0

- **breaking**: `dotenvy::Error` now includes IO file path info and variable name info
- **breaking**: `dotenvy::var` no longer calls `load` internally
- **breaking**: `dotenvy::Result` is now private
- **breaking**: `dotenvy::var`, `dotenvy::from_filename*` are deprecated
- `Error` is now `From<std::io::Error`

- **breaking**: dotenvy CLI uses `with_path` instead of `from_filename`
- **breaking**: dotenvy CLI defaults to *./.env*, no longer traversing parent directories.
- **breaking**: dotenvy CLI exits with code 2 instead of code 1 if the external command is omitted
- Fix doctests on windows not compiling ([PR #79](https://github.com/allan2/dotenvy/pull/79) by [vallentin](https://github.com/vallentin).
- MSRV updated to 1.68.0

## [0.15.7] - 2023-03-22

### Added

- override existing envv vars ([PR #41](https://github.com/allan2/dotenvy/pull/28) by [tshepang](https://github.com/tshepang) and [PR #47](https://github.com/allan2/dotenvy/pull/47) by [LeoniePhiline](https://github.com/LeoniePhiline))
- contribution guide ([PR #28](https://github.com/allan2/dotenvy/pull/28) by [sonro](https://github.com/sonro))

### Changed

- MSRV updated to 1.56.1
- removed `dotenv_codegen_impl` and `proc_macro_hack` dependencies ([PR #45](https://github.com/allan2/dotenvy/pull/45) by [sonro](https://github.com/sonro))
- improved examples by handling errors, rather than using `unwrap` ([PR #52](https://github.com/allan2/dotenvy/pull/52) by [LeoniePhiline](https://github.com/LeoniePhiline))
- `Iter` now public in the crate root ([PR #51](https://github.com/allan2/dotenvy/pull/51) by [LeoniePhiline](https://github.com/LeoniePhiline))

## [0.15.6] - 2022-10-17

### Added

- Support for UTF-8 BOM [(PR #28)](https://github.com/allan2/dotenvy/pull/28) by [sonro](https://github.com/sonro). Thanks [webbertakken](https://github.com/webbertakken)

### Changed

- `dirs` moved to dev dependency [(PR #24)](https://github.com/allan2/dotenvy/pull/24) by [goto-bus-stop](https://github.com/goto-bus-stop)
- Fix formatting and linting ([PR #22](https://github.com/allan2/dotenvy/pull/22) and [#23](https://github.com/allan2/dotenvy/pull/23)) by [rillian](https://github.com/rillian)

## [0.15.5] - 2022-09-19

### Added

- Minimum Supported Rust Version is now 1.58.1 [(PR #18)](https://github.com/allan2/dotenvy/pull/21) by [rillian](https://github.com/rillian)

## [0.15.4] - 2022-09-19

### Changed

- Another fix for multiline support [(PR #18)](https://github.com/allan2/dotenvy/pull/18). Thanks [BlackDex](https://github.com/BlackDex) and [LeoniePhiline](https://github.com/LeoniePhiline)

## [0.15.3] - 2022-08-29

### Changed

- Fix comments in multiline input [(PR #16)](https://github.com/allan2/dotenvy/pull/16) by [domodwyer](https://github.com/domodwyer)

## [0.15.2] - 2022-08-22

### Added

- Multiline support [(PR #3)](https://github.com/allan2/dotenvy/pull/3) by [hoijui](https://github.com/hoijui)
- `from_read` and `from_read_iter` [(PR #5)](https://github.com/allan2/dotenvy/pull/5) by [Kevin M Granger](https://github.com/KevinMGranger)

### Changed

- doc rewrite [(commit 1a45555)](https://github.com/allan2/dotenvy/commit/1a455554f5e4b4211be5490309d580d18a4cdf56) by [allan2](https://github.com/hoijui)
- doc link improvement formatting [(PR #6)](https://github.com/allan2/dotenvy/pull/6) by [Kevin M Granger](https://github.com/KevinMGranger)
- dependency bump (clap 3.1 to 3.2)

## [0.15.1] - 2022-02-28

### Added

- `dotenv` crate forked as `dotenvy`
- `dotenv_codegen` forked as `dotenvy_codgen`
- `dotenv_codegen_implementation` forked as `dotenvy_codegen_impl`
- Crate description for dotenvy_codegen
- Crate description for dotenvy_codgen_impl
- New language in README
- MIT license badge in README
- Generate helpful errors from dotenv! macro (full merge of [dotenv-rs/dotenv #58](https://github.com/dotenv-rs/dotenv/pull/57))

### Changed

- replaced deprecated `std::env_home:dir()` with `dirs:home_dir`
- Better handling of `home_dir` (merge of [dotenv-rs/dotenv #62](https://github.com/dotenv-rs/dotenv/pull/62))
- assertions dealing with `Result` (based on [dotenv-rs/dotenv #57](https://github.com/dotenv-rs/dotenv/pull/57))
- upgraded clap in `dotenvy` bin from v2 to v3.1 (covers [dotenv-rs/dotenv #76](https://github.com/dotenv-rs/dotenv/pull/76))

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

[Unreleased]: https://github.com/allan2/dotenvy/compare/v0.15.7...HEAD
[0.15.7]: https://github.com/allan2/dotenvy/releases/tag/v0.15.7
[0.15.6]: https://github.com/allan2/dotenvy/releases/tag/v0.15.6
[0.15.5]: https://github.com/allan2/dotenvy/releases/tag/v0.15.5
[0.15.4]: https://github.com/allan2/dotenvy/releases/tag/v0.15.4
[0.15.3]: https://github.com/allan2/dotenvy/releases/tag/v0.15.3
[0.15.2]: https://github.com/allan2/dotenvy/releases/tag/v0.15.2
[0.15.1]: https://github.com/allan2/dotenvy/commit/ea1572ff164c2dfabcf3c8cafd32c93186ad047f
[0.15.0]: https://github.com/allan2/dotenvy/releases/tag/v0.15.0
[0.14.1]: https://github.com/allan2/dotenvy/releases/tag/v0.14.1
[0.14.0]: https://github.com/allan2/dotenvy/releases/tag/v0.14.0
