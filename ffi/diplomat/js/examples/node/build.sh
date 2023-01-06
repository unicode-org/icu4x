#! /usr/bin/bash

set -e

# Set toolchain variable to a default if not defined
ICU4X_NIGHTLY_TOOLCHAIN="${ICU4X_NIGHTLY_TOOLCHAIN:-nightly-2022-04-05}"

# Install Rust toolchains
rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
rustup +${ICU4X_NIGHTLY_TOOLCHAIN} component add rust-src

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
# keep in sync with .cargo/config.toml
WASM_STACK_SIZE=100000

# Build the WASM library
RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-args=-zstack-size=${WASM_STACK_SIZE}" cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
    -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release \
    --package icu_capi_cdylib \
    --features wasm_default

# Cache postcard data so as not to regen whenever blowing away `lib/`
if ! test -f "full-data-cached.postcard"; then
    # Regen all data
    cargo run  --manifest-path ../../../../../provider/datagen/Cargo.toml \
        --features=bin,experimental \
        --release -- \
        --all-locales \
        --all-keys \
        --cldr-tag 42.0.0 \
        --icuexport-tag release-72-1 \
        --format blob \
        --out ./full-data-cached.postcard
fi

# Refresh the lib folder
rm -rf lib
mkdir -p lib
cp ../../include/* lib
cp ../../../../../target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm lib/icu_capi.wasm
cp full-data-cached.postcard lib/full.postcard
