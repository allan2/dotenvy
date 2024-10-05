#!/usr/bin/env sh
set -e

MSRV="1.74.0"

echo "MSRV set to $MSRV"

echo "cargo fmt"
cargo fmt --all --check

echo "clippy"
cargo clippy -q --all-features --all-targets --workspace -- -D warnings

echo "Building docs"
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc -q --no-deps --all-features --document-private-items

echo "Building tests"
cargo test -q --no-run

echo "Running tests..."
cargo test -q > /dev/null

echo "Building tests on MSRV"
RUSTUP_TOOLCHAIN="$MSRV" cargo test -q --no-run 

echo "Running tests on MSRV..."
RUSTUP_TOOLCHAIN="$MSRV" cargo test -q > /dev/null

echo ">>>>>> ALL OK <<<<<<<<"
