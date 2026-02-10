# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

ICU_CAPI := $(shell cargo metadata --format-version 1 | jq '.packages[] | select(.name == "icu_capi").manifest_path' | xargs dirname)
HEADERS := ${ICU_CAPI}/bindings/js
ALL_HEADERS := $(wildcard ${HEADERS}/*)

LLVM_COMPATIBLE_NIGHTLY ?= "nightly-2025-02-17"

$(ALL_HEADERS):

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
WASM_STACK_SIZE := 100000

BASEDIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

lib/index.mjs: $(ALL_HEADERS)
	rm -rf lib;
	cp -rL ${HEADERS} lib

diplomat.config.mjs:
	echo "export default { wasm_path: new URL('target/wasm32-unknown-unknown/release/icu_capi.wasm', import.meta.url) };" > diplomat.config.mjs

target/wasm32-unknown-unknown/release/icu_capi.wasm: FORCE
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	# Build the WASM library.
	# For an explanation of the shared flags, see ffi/npm/Makefile and ffi/capi/build.sh.
	# This Makefile adds the following flags to further reduce binary size:
	# -Clinker-plugin-lto: add symbols to object files that allows the linker to perform LTO
	# -Ccodegen-units=1: generate the code in a single process to enable more opportunities for optimization
	# -C linker=${BASEDIR}/ld.py: run a custom linker script to remove symbols
	# -C linker-flavor=wasm-ld: tells rustc to pass the arguments to ld.py as it would pass to wasm-ld
	# -Clto: run link-time optimization (when building Rust object files)
	# -Cembed-bitcode: retain llvm bitcode in the object files, required for LTO
	RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-arg=-zstack-size=${WASM_STACK_SIZE} -Clinker-plugin-lto -Ccodegen-units=1 -C linker=${BASEDIR}/ld.py -C linker-flavor=wasm-ld -Clto -Cembed-bitcode -Zwasm-c-abi=spec" \
	# -Z build-std-features=panic_immediate_abort: older version of -Cpanic=immediate-abort,
	# with the side-effect of disabling other optional std features that cause bloat. See:
	# <https://github.com/rust-lang/rust/issues/147257>
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc \
		-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
		--target wasm32-unknown-unknown \
		-p icu_capi \
		--crate-type cdylib \
		--release

build: target/wasm32-unknown-unknown/release/icu_capi.wasm lib/index.mjs diplomat.config.mjs
	ls -l target/wasm32-unknown-unknown/release/icu_capi.wasm

test: build
	node tiny.mjs

clean:
	git clean -xf *
