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

# 60 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
# keep in sync with .cargo/config.toml
WASM_STACK_SIZE=100000

# Build the WASM library
RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-args=-zstack-size=${WASM_STACK_SIZE}" cargo +nightly-2022-04-05 build \
    -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release \
    --package icu_capi_cdylib \
    --features wasm_default \

cp ../../../../../target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm lib/icu_capi.wasm

# Cache postcard data so as not to regen whenever blowing away `lib/`
if test -f "full-data-cached.postcard"; then
    cp full-data-cached.postcard lib/full.postcard
    exit 0
else
    # Regen all data
    cargo run -p icu_datagen --features=bin,experimental -- --all-locales --all-keys --cldr-tag latest --icuexport-tag latest --format blob --out lib/full.postcard
    cp lib/full.postcard full-data-cached.postcard
fi
