#!/bin/sh -ve
export RUST_BACKTRACE=full

cargo clean
cargo update

# try to generate doc
cargo doc

# run unit tests
cargo test

cargo clean

cd example
./test.sh
cd ..
