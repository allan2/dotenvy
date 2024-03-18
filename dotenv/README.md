# dotenvy

[![Crates.io](https://img.shields.io/crates/v/dotenvy.svg)](https://crates.io/crates/dotenvy)
[![msrv
1.68.0](https://img.shields.io/badge/msrv-1.68.0-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.68.0)
[![ci](https://github.com/allan2/dotenvy/actions/workflows/ci.yml/badge.svg)](https://github.com/allan2/dotenvy/actions/workflows/ci.yml)
[![docs](https://img.shields.io/docsrs/dotenvy?logo=docs.rs)](https://docs.rs/dotenvy/)

A well-maintained fork of the [dotenv](https://github.com/dotenv-rs/dotenv) crate.

This crate is the suggested alternative for `dotenv` in security advisory [RUSTSEC-2021-0141](https://rustsec.org/advisories/RUSTSEC-2021-0141.html).

This library loads environment variables from a _.env_ file. This is convenient for dev environments.

## Components

1. [`dotenvy`](https://crates.io/crates/dotenvy) crate - A well-maintained fork of the `dotenv` crate.
2. [`dotenvy_macro`](https://crates.io/crates/dotenvy_macro) crate - A macro for compile time dotenv inspection. This is a fork of `dotenv_codegen`.
3. `dotenvy` CLI tool for running a command using the environment from a _.env_ file (currently Unix only)

## Usage

### Loading at runtime

```rs
use dotenvy::dotenv;
use std::env;

fn main() {
    // load environment variables from .env file
    load().expect(".env file not found");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}
```

### Loading at compile time

The `dotenv!` macro provided by `dotenvy_macro` crate can be used.

## Minimum supported Rust version

Currently: **1.68.0**

We aim to support the latest 8 rustc versions - approximately 1 year. Increasing
MSRV is _not_ considered a semver-breaking change.

## Why does this fork exist?

The original dotenv crate has not been updated since June 26, 2020. Attempts to reach the authors and present maintainer were not successful ([dotenv-rs/dotenv #74](https://github.com/dotenv-rs/dotenv/issues/74)).

This fork intends to serve as the development home for the dotenv implementation in Rust.

## What are the differences from the original?

This repo fixes:
- the primary function is `dotenvy::load` rather than `dotenv::dotenv`
- home directory works correctly (no longer using the deprecated `std::env::home_dir`)
- more helpful errors for `dotenv!` ([dotenv-rs/dotenv #57](https://github.com/dotenv-rs/dotenv/pull/57))

It also adds:

- multiline support for environment variable values
- `io::Read` support via [`from_read`](https://docs.rs/dotenvy/latest/dotenvy/fn.from_read.html) and [`from_read_iter`](https://docs.rs/dotenvy/latest/dotenvy/fn.from_read_iter.html)
- improved docs

For a full list of changes, refer to the [changelog](./CHANGELOG.md).

## Contributing

Thank you very much for considering to contribute to this project! See
[CONTRIBUTING.md](./CONTRIBUTING.md) for details.

**Note**: Before you take the time to open a pull request, please open an issue first.

## The legend

Legend has it that the Lost Maintainer will return, merging changes from `dotenvy` into `dotenv` with such thrust that all `Cargo.toml`s will lose one keystroke. Only then shall the Rust dotenv crateverse be united in true harmony.

Until then, this repo dutifully carries on the dotenv torch. It is actively maintained.
