#!/bin/bash

pushd $(dirname "$0")/../../../../docs/tutorials/rust/buffer
cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
popd
cp $(dirname "$0")/../../../../docs/tutorials/rust/buffer/target/wasm32-unknown-unknown/release/tutorial_buffer.wasm $(dirname "$0")
    