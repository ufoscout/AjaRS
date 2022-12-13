#!/usr/bin/env bash

## exit if something fails
set -e

echo 'Attempt ''cargo check'' before publishing'
cargo check

echo 'Attempt ''cargo check --all-features'' before publishing'
cargo check --all-features

echo 'Attempt ''cargo test'' before publishing'
cargo test --workspace

echo 'Attempt ''cargo test --all-features'' before publishing'
cargo test --workspace --all-features
