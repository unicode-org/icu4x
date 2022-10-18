#! /usr/bin/bash

set -e

ICU4X_NIGHTLY_TOOLCHAIN="${ICU4X_NIGHTLY_TOOLCHAIN:-nightly-2022-04-05}"

if test -d "lib"; then
    exit 0
fi

mkdir lib

# Copy JS "header" files
cp ../../include/* lib

# Install Rust toolchains
rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
rustup +${ICU4X_NIGHTLY_TOOLCHAIN} component add rust-src

# 60 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
# keep in sync with .cargo/config.toml
WASM_STACK_SIZE=100000

# Build the WASM library
RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-args=-zstack-size=${WASM_STACK_SIZE}" cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
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
    cargo run -p icu_datagen --features=bin,experimental -- --all-locales --all-keys --cldr-tag latest --icuexport-tag icu4x/2022-08-17/71.x --format blob --out lib/full.postcard
    cp lib/full.postcard full-data-cached.postcard
fi
