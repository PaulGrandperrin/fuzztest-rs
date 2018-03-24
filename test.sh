#!/bin/sh -ve
export RUST_BACKTRACE=full

cargo build
cargo test
cargo doc
