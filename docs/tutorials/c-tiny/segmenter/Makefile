# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

# PLEASE KEEP THIS FILE IN SYNC WITH ../fixeddecimal_tiny/Makefile

.DEFAULT_GOAL := test
.PHONY: build test
FORCE:

ICU_CAPI := $(shell cargo metadata --manifest-path Cargo.toml --format-version 1 | jq '.packages[] | select(.name == "icu_capi").manifest_path' | xargs dirname)
HEADERS := ${ICU_CAPI}/bindings/c
ALL_HEADERS := $(wildcard ${HEADERS}/*)
ICU4X_NIGHTLY_TOOLCHAIN ?= "nightly-2022-12-26"

$(ALL_HEADERS):

GCC := gcc
CLANG := clang-15
LLD := lld-15

target/debug/libicu_capi.a: FORCE
	cargo rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/icu_segmenter,icu_capi/std

target/x86_64-unknown-linux-gnu/debug/libicu_capi.a: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" \
	cargo +${ICU4X_NIGHTLY_TOOLCHAIN} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/icu_segmenter,icu_capi/panic-handler,icu_capi/libc-alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu

target/x86_64-unknown-linux-gnu/release/libicu_capi.a: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	RUSTFLAGS="-Clto -Cembed-bitcode -Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort -Copt-level=s" \
	cargo +${ICU4X_NIGHTLY_TOOLCHAIN} rustc -p icu_capi --crate-type staticlib --no-default-features --features icu_capi/compiled_data,icu_capi/icu_segmenter,icu_capi/panic-handler,icu_capi/libc-alloc \
	-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release

# Naive target: no optimizations, full std
optim0.elf: target/debug/libicu_capi.a $(ALL_HEADERS) test.c
	$(GCC) test.c -I${HEADERS} target/debug/libicu_capi.a -ldl -lm -g -o optim0.elf

# optim.elf: gcc with maximum link-time code stripping (gc-sections and strip-all)
optim1.elf: target/debug/libicu_capi.a $(ALL_HEADERS) test.c
	$(GCC) -fdata-sections -ffunction-sections test.c -I${HEADERS} target/debug/libicu_capi.a -ldl -lm -g -o optim1.elf -Wl,--gc-sections -Wl,--strip-all

# optim2.elf: clang single-step with gc-sections
optim2.elf: target/x86_64-unknown-linux-gnu/debug/libicu_capi.a $(ALL_HEADERS) test.c
	$(CLANG) -flto -fdata-sections -ffunction-sections test.c -I${HEADERS} target/x86_64-unknown-linux-gnu/debug/libicu_capi.a -g -o optim2.elf -Wl,--gc-sections

optim3.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -g -o optim3.o

# optim3.elf: clang two-step with lld, debug mode
optim3.elf: optim3.o target/x86_64-unknown-linux-gnu/debug/libicu_capi.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim3.elf optim3.o target/x86_64-unknown-linux-gnu/debug/libicu_capi.a -Wl,--gc-sections

optim4.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -g -o optim4.o

# optim4.elf: clang two-step with lld, release mode with debug symbols
optim4.elf: optim4.o target/x86_64-unknown-linux-gnu/release/libicu_capi.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim4.elf optim4.o target/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections

optim5.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -I${HEADERS} -o optim5.o

# optim5.elf: clang two-step with lld, release mode stripped of debug symbols
optim5.elf: optim5.o target/x86_64-unknown-linux-gnu/release/libicu_capi.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim5.elf optim5.o target/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all

build: optim0.elf optim1.elf optim2.elf optim3.elf optim4.elf optim5.elf
	ls -l optim*.elf

test: build
	./optim0.elf
	./optim1.elf
	./optim2.elf
	./optim2.elf
	./optim4.elf
	./optim5.elf

clean:
	git clean -xf *
