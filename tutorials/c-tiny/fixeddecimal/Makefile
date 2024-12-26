# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

# PLEASE KEEP THIS FILE IN SYNC WITH ../segmenter_tiny/Makefile

.DEFAULT_GOAL := test
.PHONY: build test

ICU_CAPI := $(shell cargo metadata --manifest-path Cargo.toml --format-version 1 | jq '.packages[] | select(.name == "icu_capi").manifest_path' | xargs dirname)
HEADERS := ${ICU_CAPI}/bindings/c
ALL_HEADERS := $(wildcard ${HEADERS}/*)

ALL_RUST_SRC = $(wildcard ../../../components/**/*.rs) $(wildcard ../../../provider/**/*.rs) $(wildcard ../../../ffi/capi/**/*.rs)

$(ALL_HEADERS):

GCC := gcc
CLANG := clang-17
LLD := lld-17
LLVM_COMPATIBLE_NIGHTLY = "nightly-2024-01-01"


target/release/icu4x-datagen: $(ALL_RUST_SRC)
	cargo build --manifest-path ../../../Cargo.toml -p icu4x-datagen --target-dir target --release

baked_data/mod.rs: target/release/icu4x-datagen
	target/release/icu4x-datagen --locales en bn --markers all --deduplication none --format baked --out baked_data --overwrite

target/debug/libicu_capi.a: baked_data/mod.rs
	cargo rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/decimal,icu_capi/std


target-normal/release/libicu_capi.a: baked_data/mod.rs
	cargo rustc --release --target-dir target-normal -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/decimal,icu_capi/std


target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a: baked_data/mod.rs
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	RUSTFLAGS="-Cpanic=abort -Copt-level=s" ICU4X_DATA_DIR=$(shell pwd)/baked_data \
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/decimal,icu_capi/looping_panic_handler,icu_capi/libc_alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort 

target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a: baked_data/mod.rs
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	RUSTFLAGS="-Clto -Cembed-bitcode -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort -Copt-level=s" ICU4X_DATA_DIR=$(shell pwd)/baked_data \
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/decimal,icu_capi/looping_panic_handler,icu_capi/libc_alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort-lto 


target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a: baked_data/mod.rs
	rustup toolchain install ${LLVM_COMPATIBLE_NIGHTLY}
	rustup component add rust-src --toolchain ${LLVM_COMPATIBLE_NIGHTLY}
	RUSTFLAGS="-Clto -Cembed-bitcode -Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort -Copt-level=s" ICU4X_DATA_DIR=$(shell pwd)/baked_data \
	cargo +${LLVM_COMPATIBLE_NIGHTLY} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/decimal,icu_capi/looping_panic_handler,icu_capi/libc_alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release  --target-dir target-panic-abort-linker-plugin-lto 

# Naive target: basic release mode, full std
release.elf: target-normal/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(GCC) test.c -I${HEADERS} target-normal/release/libicu_capi.a -ldl -lm -g -o release.elf

# gcc with maximum link-time code stripping (gc-sections and strip-all)
release-gcc-stripped.elf: target-normal/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(GCC) -fdata-sections -ffunction-sections test.c -I${HEADERS} target-normal/release/libicu_capi.a -ldl -lm -g -o release-gcc-stripped.elf -Wl,--gc-sections -Wl,--strip-all

# clang single-step; rust with release, panic-abort (std panic-immediate-abort)
panic-abort-clang.elf: target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) test.c -I${HEADERS} target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a -g -o panic-abort-clang.elf



# clang single-step with gc-sections, stripping; rust with release, LTO, panic-abort (std panic-immediate-abort)
panic-abort-clang-stripped.elf: target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) test.c -fdata-sections -ffunction-sections -I${HEADERS} target-panic-abort/x86_64-unknown-linux-gnu/release/libicu_capi.a -g -o panic-abort-clang-stripped.elf  -Wl,--gc-sections -Wl,--strip-all


# clang single-step with LTO, rust with release, LTO, panic-abort (std panic-immediate-abort)
panic-abort-lto-clang.elf: target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -flto -fuse-ld=$(LLD) -fdata-sections -ffunction-sections test.c -I${HEADERS} target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -g -o panic-abort-lto-clang.elf

# clang single-step with LTO, gc-sections, stripping, rust with release, LTO, panic-abort (std panic-immediate-abort)
panic-abort-lto-clang-stripped.elf: target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -flto -fuse-ld=$(LLD) -fdata-sections -ffunction-sections test.c -I${HEADERS} target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -g -o panic-abort-lto-clang-stripped.elf -Wl,--gc-sections -Wl,--strip-all

panic-abort-lto-clang-twostep.o: target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fuse-ld=$(LLD) -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -g -o panic-abort-lto-clang-twostep.o

# clang two-step with lld, rust with release, panic-abort (std panic-immediate-abort)
panic-abort-lto-clang-twostep.elf: panic-abort-lto-clang-twostep.o
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o panic-abort-lto-clang-twostep.elf panic-abort-lto-clang-twostep.o target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a


panic-abort-lto-clang-twostep-stripped.o: target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -g -o panic-abort-lto-clang-twostep-stripped.o

# clang two-step with lld, ThinLTO, gc-sections, stripping, rust with release, LTO, panic-abort (std panic-immediate-abort)
panic-abort-lto-clang-twostep-stripped.elf: panic-abort-lto-clang-twostep-stripped.o
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o panic-abort-lto-clang-twostep-stripped.elf panic-abort-lto-clang-twostep.o target-panic-abort-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all


panic-abort-linker-plugin-lto-clang-twostep.o: target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -g -o panic-abort-linker-plugin-lto-clang-twostep.o

# clang two-step with lld, ThinLTO, rust with release, LTO, panic-abort (std panic-immediate-abort), with cross-compiler LTO
panic-abort-linker-plugin-lto-clang-twostep.elf: panic-abort-linker-plugin-lto-clang-twostep.o
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o panic-abort-linker-plugin-lto-clang-twostep.elf panic-abort-linker-plugin-lto-clang-twostep.o target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all

# clang one-step with lld, ThinLTO, gc-sections, stripping, rust with release, LTO, panic-abort (std panic-immediate-abort), with cross-compiler LTO
panic-abort-linker-plugin-lto-clang-stripped.elf:
	$(CLANG) -flto=thin -fuse-ld=$(LLD) --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -o panic-abort-linker-plugin-lto-clang-stripped.elf target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all

panic-abort-linker-plugin-lto-clang-twostep-stripped.o: target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -g -o panic-abort-linker-plugin-lto-clang-twostep-stripped.o

# clang two-step with lld, ThinLTO, gc-sections, stripping, rust with release, LTO, panic-abort (std panic-immediate-abort), with cross-compiler LTO
panic-abort-linker-plugin-lto-clang-twostep-stripped.elf: panic-abort-linker-plugin-lto-clang-twostep-stripped.o
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o panic-abort-linker-plugin-lto-clang-twostep-stripped.elf panic-abort-linker-plugin-lto-clang-twostep-stripped.o target-panic-abort-linker-plugin-lto/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all

build: release.elf release-gcc-stripped.elf panic-abort-clang.elf panic-abort-clang-stripped.elf panic-abort-lto-clang.elf panic-abort-lto-clang-stripped.elf panic-abort-lto-clang-twostep.elf panic-abort-lto-clang-twostep-stripped.elf panic-abort-linker-plugin-lto-clang-stripped.elf panic-abort-linker-plugin-lto-clang-twostep.elf panic-abort-linker-plugin-lto-clang-twostep-stripped.elf
	ls -l *.elf

test: build
	./release.elf bn
	./release-gcc-stripped.elf bn
	./panic-abort-clang.elf bn
	./panic-abort-clang-stripped.elf bn
	./panic-abort-lto-clang.elf bn
	./panic-abort-lto-clang-stripped.elf bn
	./panic-abort-lto-clang-twostep.elf bn
	./panic-abort-lto-clang-twostep-stripped.elf bn
	./panic-abort-linker-plugin-lto-clang-twostep.elf bn
	./panic-abort-linker-plugin-lto-clang-stripped.elf bn
	./panic-abort-linker-plugin-lto-clang-twostep-stripped.elf bn

clean:
	git clean -xf *
