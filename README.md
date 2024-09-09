# dotenvy

[![Crates.io](https://img.shields.io/crates/v/dotenvy.svg)](https://crates.io/crates/dotenvy)
[![msrv
1.74.0](https://img.shields.io/badge/msrv-1.74.0-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.74.0)
[![ci](https://github.com/allan2/dotenvy/actions/workflows/ci.yml/badge.svg)](https://github.com/allan2/dotenvy/actions/workflows/ci.yml)
[![docs](https://img.shields.io/docsrs/dotenvy?logo=docs.rs)](https://docs.rs/dotenvy/)

A well-maintained fork of the [dotenv](https://github.com/dotenv-rs/dotenv) crate.

This crate is the suggested alternative for `dotenv` in security advisory [RUSTSEC-2021-0141](https://rustsec.org/advisories/RUSTSEC-2021-0141.html).

This library loads environment variables from an env file.

## Components

1. [`dotenvy`](https://crates.io/crates/dotenvy) crate - A well-maintained fork of the `dotenv` crate.
2. [`dotenvy_macro`](https://crates.io/crates/dotenvy_macro) crate - A macro for compile-time _.env_ inspection. This is a fork of `dotenv_codegen`.
3. [`dotenvy-macros`](https://crates.io/crates/dotenvy-macros) crate - A runtime macro library containg the `load` attribute macro.
4. `dotenvy` binary that loads an env file before executing a specified command.

## Environment file

An env file consists of keypairs like so:

_.env_

```sh
HOST=foo
PORT=3000
```

They are commonly named _.env_, _.env.dev_, _.env.prod_, etc., but any name can be used.

Variables can span multiple lines and also be substituted. For an explanation on substituion rules, please refer to
the [_.env-substitution_](.env-substitution) example file.

## Usage

This crate contains two APIs, a non-environment-modifying API and an environment-modifying API.

Modifying calls [`std::env::set_var`](`https://doc.rust-lang.org/std/env/fn.set_var.html`) internally,
which is marked unsafe in the Rust 2024 edition. For this reason, we recommend using the non-modifying API unless
necesary.

### Configuration

```rs
// from a file
let loader1 = EnvLoader::with_path("./.env").sequence(EnvSequence::InputThenEnv);
let loader2 = EnvLoader::new();  // same as loader1

// from a reader
let s = "HOST=foo\nPORT=3000";
let str_loader = EnvLoader::with_reader(Cursor::new(s));

// will load from the env file, override exiting values in the program environment
let overriding_loader = EnvLoader::new().sequence(EnvSequence::EnvThenInput);
```

Load constuction is infallible. I/O is derred until `load` or `load_and_modify` is called.
This is to allow support configurations such as [dev/prod](examples/dev-prod/src/main.rs) and
[optional loading](examples/optional/src/main.rs).

### Non-modifying API

```rs
use dotenvy::{EnvLoader};
use std::{error, env};

fn main() -> Result<(), Box<dyn error::Error>> {
    let env_map = EnvLoader::new().load()?;
    println!("HOST={}", env_map.var("HOST")?);
    Ok(())
}
```

### Modifying API

Sometimes, you have to modify the environment. You might [spawn a child process](examples/modify/src/main.rs) or rely
on `std::env::var` to read variables.

```rs
use dotenvy::{EnvLoader};
use std::{error, env};

fn main() -> Result<(), Box<dyn error::Error>> {
    let loader = EnvLoader::new();
    let env_map = unsafe { loader.load_and_modify() }?;
    println!("HOST={}", env_map.var("HOST")?);
    println!("HOST={}", std::env::var("HOST")?);
    Ok(())
}
```

If using async, you must modify the environment before starting the async runtime.

The `load` attribute macro can be used to do this. To use it, enable the `macros` feature for the `dotenvy` crate.

```rs
#[dotenvy::load]  // will call `load_and_modify` before the async runtime is started
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("HOST={}", env::var("HOST")?);
    Ok(())
}
```
`dotenvy::load` must be placed before `tokio::main`.

A non-macro example is [here](examples/modify-tokio/src/main.rs).

### Loading at compile time

The `dotenv!` macro provided by `dotenvy_macro` crate can be used.

## Minimum supported Rust version

We aim to support the latest 8 rustc versions - approximately 1 year. Increasing
MSRV is _not_ considered a semver-breaking change.

## Why does this fork exist?

The original dotenv crate has not been updated since June 26, 2020. Attempts to reach the authors and present maintainer were not successful ([dotenv-rs/dotenv #74](https://github.com/dotenv-rs/dotenv/issues/74)).

This fork intends to serve as the development home for the dotenv implementation in Rust.

## What are the differences from the original?

This repo adds:

- non-modifying API
- configurable `EnvLoader`
- optional loading, ergonomic dev/prod handling
- reader support, such as reading from any reader that is `io::Read`
- more informative `Error` type, containing the file path and variable name
- `load` attribute macro
- multiline support
- more examples and docs

For a full list of changes, refer to the [changelog](./CHANGELOG.md).

## Contributing

Thank you very much for considering to contribute to this project! See
[CONTRIBUTING.md](./CONTRIBUTING.md) for details.

**Note**: Before you take the time to open a pull request, please open an issue first.

## The legend

Legend has it that the Lost Maintainer will return, merging changes from `dotenvy` into `dotenv` with such thrust that all `Cargo.toml`s will lose one keystroke. Only then shall the Rust dotenv crateverse be united in true harmony.

Until then, this repo dutifully carries on the dotenv torch. It is actively maintained.
