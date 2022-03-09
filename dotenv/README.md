# dotenvy

[![crates.io](https://img.shields.io/crates/v/dotenvy.svg)](https://crates.io/crates/dotenvy)
[![Released API docs](https://docs.rs/dotenvy/badge.svg)](https://docs.rs/dotenvy)

A well-maintained fork of the [dotenv](https://github.com/dotenv-rs/dotenv) crate.

This library loads environment variables from a _.env_ file. This is convenient for dev environments.

## Components

1. [`dotenvy`](https://crates.io/crates/dotenvy) crate - A well-maintained fork of the `dotenv` crate.
2. [`dotenvy_codegen`](https://crates.io/crates/dotenvy_codegen) crate - A macro for compile time dotenv inspection.
3. [`dotenvy_codgen_impl`](https://crates.io/crates/dotenvy_codegen_impl) crate - Internal implementation for dotenvy_codegen.
4. `dotenvy` CLI tool for running a command using the environment from a _.env_ file (currently Unix only)

## Usage

### Loading at runtime

```rs
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}
```

### Loading at compile time

The `dotenv!` macro provided by [`dotenvy_codegen`](https://crates.io/crates/dotenvy_codegen) can be used.

## Why does this fork exist?

The original dotenv crate has not been updated since June 26, 2020. Attempts to reach the authors and present maintainer were not successful ([dotenv-rs/dotenv #74](https://github.com/dotenv-rs/dotenv/issues/74)).

This fork is intended to serve as the development home for the dotenv implementation in Rust.

## What are the differences from the original?

This repo fixes:

- home directory works correctly (no longer using the deprecated `std::env::home_dir`)
- more helpful errors for `dotenv!` ([dotenv-rs/dotenv #57](https://github.com/dotenv-rs/dotenv/pull/57))

For a full list of changes, read the [changelog](./CHANGELOG.md).

## The legend

Legend has it that the Lost Maintainer will return, merging changes from `dotenvy` into `dotenv` with such thrust that all `Cargo.toml`s will lose one keystroke. Only then shall the Rust dotenv crateverse be united in true harmony.

Until then, this repo dutifully carries on the dotenv torch. It is actively maintained. Contributions and PRs are very welcome!
