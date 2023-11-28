# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

ICU_CAPI := $(shell cargo metadata --format-version 1 | jq '.packages[] | select(.name == "icu_capi").manifest_path' | xargs dirname)
HEADERS := ${ICU_CAPI}/bindings/js
ALL_HEADERS := $(wildcard ${HEADERS}/*)
ICU4X_NIGHTLY_TOOLCHAIN ?= "nightly-2022-12-26"

$(ALL_HEADERS):

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
WASM_STACK_SIZE := 100000

BASEDIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

lib/index.js: $(ALL_HEADERS)
	rm -rf lib;
	cp -r ${HEADERS} lib

target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	# Build the WASM library
	CARGO_TARGET_DIR="target" \
	RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-arg=-zstack-size=${WASM_STACK_SIZE} -Clinker-plugin-lto -Ccodegen-units=1 -C linker=${BASEDIR}/ld.py -C linker-flavor=wasm-ld -Clto -Cembed-bitcode" \
	cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
	    -p icu_capi_cdylib \
		-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
		--target wasm32-unknown-unknown \
		--release

build: target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm lib/index.js
	ls -l target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm

test:
	node tiny.mjs

clean:
	git clean -xf *
