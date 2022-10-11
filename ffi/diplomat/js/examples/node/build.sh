#! /usr/bin/bash

set -e

if test -d "lib"; then
    exit 0
fi

mkdir lib

# Copy JS "header" files
cp ../../include/* lib

# Install Rust toolchains
rustup toolchain install nightly-2022-04-05
rustup +nightly-2022-04-05 component add rust-src

# Build the WASM library
cargo +nightly-2022-04-05 build \
    -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --profile=release-opt-size \
    --package icu_capi_cdylib \
    --features wasm_default \
    --target-dir target

if ! command -v wasm-opt &> /dev/null
then
    npm install -g wasm-opt
fi

# Optimize the WASM library
wasm-opt -Os target/wasm32-unknown-unknown/release-opt-size/icu_capi_cdylib.wasm -o lib/icu_capi.wasm

rm -r target
