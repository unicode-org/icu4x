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

BASEDIR=$(dirname "$(realpath "$0")")

# Build the WASM library
# TODO: This likely doesn't work if $BASEDIR has spaces
RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-arg=-zstack-size=${WASM_STACK_SIZE} -Clinker-plugin-lto -Ccodegen-units=1 -C linker=${BASEDIR}/ld.py -C linker-flavor=wasm-ld" cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
    -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release \
    --package icu_capi_cdylib \
    --features wasm_default

# Don't regen the postcard data by default; delete the file to regen
if ! test -f "icu4x_data.postcard"; then
    # Regen all data
    cargo run -p icu_datagen --features=bin,experimental -- \
        --all-locales \
        --keys-for-bin ../../../../../target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm \
        --cldr-tag 41.0.0 \
        --icuexport-tag release-72-1 \
        --format blob \
        --out ./icu4x_data.postcard
fi

# Refresh the lib folder
rm -rf lib
mkdir -p lib
cp ../../include/* lib
cp ../../../../../target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm lib/icu_capi.wasm
