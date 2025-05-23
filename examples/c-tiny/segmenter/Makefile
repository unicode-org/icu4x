# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

# PLEASE KEEP THIS FILE IN SYNC WITH ../decimal/Makefile
# Differences between this file and that one:
# - CAPI_COMPONENT is different
# - This file does not build datagen, build baked_data, or set ICU4X_DATA_DIR

# The component built by this makefile.
CAPI_COMPONENT := segmenter

.DEFAULT_GOAL := test
.PHONY: build test

ICU_CAPI := $(shell cargo metadata --manifest-path Cargo.toml --format-version 1 | jq '.packages[] | select(.name == "icu_capi").manifest_path' | xargs dirname)
HEADERS := ${ICU_CAPI}/bindings/c
ALL_HEADERS := $(wildcard ${HEADERS}/*)

ALL_RUST_SRC = $(wildcard ../../../components/**/*.rs) $(wildcard ../../../provider/**/*.rs) $(wildcard ../../../ffi/capi/**/*.rs)

$(ALL_HEADERS):

GCC := gcc
CLANG := clang-19
LLD := lld-19
LLVM_COMPATIBLE_NIGHTLY = "nightly-2025-02-17"


target-normal/release/libicu_capi.a:
	cargo rustc --release --target-dir target-normal -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/${CAPI_COMPONENT},icu_capi/std


target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a:
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	RUSTFLAGS="-Cpanic=abort -Copt-level=s" \
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/${CAPI_COMPONENT},icu_capi/looping_panic_handler,icu_capi/libc_alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort 

target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a:
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	RUSTFLAGS="-Clto -Cembed-bitcode -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort -Copt-level=s" \
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/${CAPI_COMPONENT},icu_capi/looping_panic_handler,icu_capi/libc_alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort-lto 


target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a:
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	RUSTFLAGS="-Clto -Cembed-bitcode -Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort -Copt-level=s" \
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/${CAPI_COMPONENT},icu_capi/looping_panic_handler,icu_capi/libc_alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort-linker-plugin-lto 

# Naive target: basic release mode, full std
1-release.elf: target-normal/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(GCC) test.c -I${HEADERS} target-normal/release/libicu_capi.a -ldl -lm -o 1-release.elf

# gcc with maximum link-time code stripping (gc-sections and strip-all)
2-release-gcc-stripped.elf: target-normal/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(GCC) -fdata-sections -ffunction-sections test.c -I${HEADERS} target-normal/release/libicu_capi.a -ldl -lm -o 2-release-gcc-stripped.elf -Wl,--gc-sections -Wl,--strip-all

# clang; rust with release, panic-abort (std panic-immediate-abort)
3-panic-abort-clang.elf: target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) test.c -I${HEADERS} target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a -o 3-panic-abort-clang.elf


# clang with LTO, rust with release, LTO, panic-abort (std panic-immediate-abort)
4-panic-abort-lto-clang.elf: target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -fdata-sections -ffunction-sections test.c -I${HEADERS} target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -o 4-panic-abort-lto-clang.elf


# clang with gc-sections, stripping; rust with release, LTO, panic-abort (std panic-immediate-abort)
5-panic-abort-clang-stripped.elf: target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) test.c -fdata-sections -ffunction-sections -I${HEADERS} target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a -o 5-panic-abort-clang-stripped.elf  -Wl,--gc-sections -Wl,--strip-all


# clang with LTO, gc-sections, stripping, rust with release, LTO, panic-abort (std panic-immediate-abort)
6-panic-abort-lto-clang-stripped.elf: target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -fdata-sections -ffunction-sections test.c -I${HEADERS} target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -o 6-panic-abort-lto-clang-stripped.elf -Wl,--gc-sections -Wl,--strip-all


# clang with lld, ThinLTO, gc-sections, stripping, rust with release, LTO, panic-abort (std panic-immediate-abort), with cross-compiler LTO
7-panic-abort-linker-plugin-lto-clang-stripped.elf: target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -o 7-panic-abort-linker-plugin-lto-clang-stripped.elf target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all



build: 1-release.elf 2-release-gcc-stripped.elf 3-panic-abort-clang.elf 4-panic-abort-lto-clang.elf 5-panic-abort-clang-stripped.elf  6-panic-abort-lto-clang-stripped.elf 7-panic-abort-linker-plugin-lto-clang-stripped.elf 
	ls -l *.elf

test: build
	./1-release.elf bn
	./2-release-gcc-stripped.elf bn
	./3-panic-abort-clang.elf bn
	./4-panic-abort-lto-clang.elf bn
	./5-panic-abort-clang-stripped.elf bn
	./6-panic-abort-lto-clang-stripped.elf bn
	./7-panic-abort-linker-plugin-lto-clang-stripped.elf bn

clean:
	git clean -xf *