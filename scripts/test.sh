#!/usr/bin/env sh
set -e
set -x
export RUST_BACKTRACE=full

echo 'Attempt ''cargo check'' before publishing'
cargo check

echo 'Attempt ''cargo check --all-features'' before publishing'
cargo check --all-features

echo 'Attempt ''cargo test'' before publishing'
cargo test --workspace

echo 'Attempt ''cargo test --all-features'' before publishing'
cargo test --workspace --all-features
