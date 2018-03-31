#!/bin/sh -ve
export RUST_BACKTRACE=full

# install cargo subcommands
cargo install honggfuzz --force --verbose --version "^0.5.11"
cargo hfuzz version

cargo clean
cargo update

rm -rf hfuzz_workspace/ fuzztest/
mkdir -p hfuzz_workspace/example/input
touch hfuzz_workspace/example/input/empty # without it, the fuzzer is unreliable at finding the data_len_0 marker

cargo test --verbose -- --test-threads=1

# verify that no fuzztest directory has been left
test ! -e fuzztest

cargo clean

