#!/usr/bin/env sh
set -e

MSRV="1.68.0"

echo "fmt check"
cargo fmt --all --check

echo "clippy"
cargo clippy -q --all-features --all-targets --workspace -- -D warnings

echo "build docs"
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc -q --no-deps --all-features --document-private-items

echo "build tests"
cargo test -q --no-run

echo "running tests..."
cargo test -q > /dev/null

echo "build tests on msrv"
RUSTUP_TOOLCHAIN="$MSRV" cargo test -q --no-run 

echo "running tests on msrv..."
RUSTUP_TOOLCHAIN="$MSRV" cargo test -q > /dev/null

echo ">>>>>> ALL OK <<<<<<<<"
