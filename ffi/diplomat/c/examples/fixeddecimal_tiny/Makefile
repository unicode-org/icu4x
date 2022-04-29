# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard ../../include/*.h)
ALL_RUST := $(wildcard ../../../src/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):

GCC := gcc
CLANG := clang-13
LLD := lld-13


../../../../../target/debug/libicu_capi_staticlib_test.a: $(ALL_RUST)
	cargo build -p icu_capi_staticlib --features provider_test
	mv ../../../../../target/debug/libicu_capi_staticlib.a ../../../../../target/debug/libicu_capi_staticlib_test.a

../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_test.a: $(ALL_RUST)
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" cargo +nightly-2022-01-31 panic-abort-build -p icu_capi_staticlib --target x86_64-unknown-linux-gnu --no-default-features --features x86tiny --features provider_test
	mv ../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib.a ../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_test.a

../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib_test_small.a: $(ALL_RUST)
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" cargo +nightly-2022-01-31 panic-abort-build -p icu_capi_staticlib --target x86_64-unknown-linux-gnu --no-default-features --features x86tiny --features smaller_test --release
	mv ../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib.a ../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib_test_small.a

# Naive target: no optimizations, full std
optim0.elf: ../../../../../target/debug/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.c
	$(GCC) test.c ../../../../../target/debug/libicu_capi_staticlib_test.a -ldl -lpthread -lm -g -o optim0.elf

# optim.elf: gcc with maximum link-time code stripping (gc-sections and strip-all)
optim1.elf: ../../../../../target/debug/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.c
	$(GCC) -fdata-sections -ffunction-sections test.c ../../../../../target/debug/libicu_capi_staticlib_test.a -ldl -lpthread -lm -g -o optim1.elf -Wl,--gc-sections -Wl,--strip-all

# optim2.elf: clang single-step with gc-sections
optim2.elf: ../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.c
	$(CLANG) -flto -fdata-sections -ffunction-sections test.c ../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_test.a -g -o optim2.elf -Wl,--gc-sections

optim3.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -g -o optim3.o

# optim3.elf: clang two-step with lld, debug mode
optim3.elf: optim3.o ../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_test.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim3.elf optim3.o ../../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_test.a -Wl,--gc-sections

optim4.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -g -o optim4.o

# optim4.elf: clang two-step with lld, release mode with debug symbols
optim4.elf: optim4.o ../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib_test_small.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim4.elf optim4.o ../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib_test_small.a -Wl,--gc-sections

optim5.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -o optim5.o

# optim5.elf: clang two-step with lld, release mode stripped of debug symbols
optim5.elf: optim5.o ../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib_test_small.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim5.elf optim5.o ../../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi_staticlib_test_small.a -Wl,--gc-sections -Wl,--strip-all

build: optim0.elf optim1.elf optim2.elf optim3.elf optim4.elf optim5.elf

# note: optim2.elf and optim3.elf crash when run with error "Illegal instruction" (investigate?)
test: build
	./optim0.elf
	./optim1.elf
	./optim4.elf
	./optim5.elf
