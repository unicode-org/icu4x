#! /usr/bin/bash
# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

set -e

# Set toolchain variable to a default if not defined
ICU4X_NIGHTLY_TOOLCHAIN="${ICU4X_NIGHTLY_TOOLCHAIN:-nightly-2022-04-05}"

# Install Rust toolchains
rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
rustup +${ICU4X_NIGHTLY_TOOLCHAIN} component add rust-src

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
# keep in sync with .cargo/config.toml
WASM_STACK_SIZE=1000000

BASEDIR=$(dirname "$(realpath "$0")")

# Build the WASM library
# TODO: This likely doesn't work if $BASEDIR has spaces
RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-arg=-zstack-size=${WASM_STACK_SIZE} -Clinker-plugin-lto -Ccodegen-units=1 -C linker=${BASEDIR}/ld.py -C linker-flavor=wasm-ld" \
# Special linker flags shouldn't overwrite the top-level target directory
CARGO_TARGET_DIR="target" \
cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
    -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release \
    --package icu_capi_cdylib \
    --features buffer_provider,logging \

cp target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm icu_capi.wasm

# Don't regen the postcard data by default; delete the file to regen
if ! test -f "icu4x_data.postcard"; then
    # Regen all data
    cargo run -p icu_datagen -- \
        --keys-for-bin icu_capi.wasm \
        --locales full \
        --format blob \
        --out ./icu4x_data.postcard
fi

# Refresh the lib folder
rm -rf lib
mkdir -p lib
cp ../../include/* lib
