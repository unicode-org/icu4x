# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

PINNED_CI_NIGHTLY ?= "nightly-2025-09-27"
FEATURES ?= "compiled_data,buffer_provider,default_components,experimental,logging"

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
WASM_STACK_SIZE := 100000
 
lib/index.mjs: FORCE
	rm -rf lib
	cp -r ../capi/bindings/js lib

../../target/wasm32-unknown-unknown/release/icu_capi.wasm: FORCE
	rustup toolchain install ${PINNED_CI_NIGHTLY}
	rustup component add rust-src --toolchain ${PINNED_CI_NIGHTLY}
	RUSTFLAGS="-Zunstable-options -Cpanic=immediate-abort -Copt-level=s -C link-args=-zstack-size=${WASM_STACK_SIZE} -Clto -Cembed-bitcode -Zwasm-c-abi=spec" \
	cargo +${PINNED_CI_NIGHTLY} rustc \
		-Z build-std=std,panic_abort \
		--target wasm32-unknown-unknown \
		--release \
		-p icu_capi \
		--crate-type cdylib \
		--no-default-features \
		--features ${FEATURES}

icu_capi.wasm: ../../target/wasm32-unknown-unknown/release/icu_capi.wasm
	cp ../../target/wasm32-unknown-unknown/release/icu_capi.wasm icu_capi.wasm

build: lib/index.mjs icu_capi.wasm