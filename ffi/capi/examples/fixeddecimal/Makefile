# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard ../../include/*.h)
ALL_RUST := $(wildcard ../../src/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):


../../../../target/debug/libicu_capi.a: $(ALL_RUST)
	cargo build

../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a: $(ALL_RUST)
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-flto -Cpanic=abort" cargo +nightly-2021-02-28 panic-abort-build --target x86_64-unknown-linux-gnu --no-default-features --features x86tiny

../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi.a: $(ALL_RUST)
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-flto -Cpanic=abort" cargo +nightly-2021-02-28 panic-abort-build --target x86_64-unknown-linux-gnu --no-default-features --features x86tiny --features smaller_static --release

# Naive target: no optimizations, full std
a.out: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) test.c
	gcc test.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g

# optim.elf: gcc with maximum link-time code stripping (gc-sections and strip-all)
optim.elf: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) test.c
	gcc -fdata-sections -ffunction-sections test.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g -o optim.elf -Wl,--gc-sections -Wl,--strip-all

# optim2.elf: clang single-step with gc-sections
optim2.elf: ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a $(ALL_HEADERS) test.c
	clang -flto -fdata-sections -ffunction-sections test.c ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a -g -o optim2.elf -Wl,--gc-sections

optim3.o: $(ALL_HEADERS)
	clang -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -g -o optim3.o

# optim3.elf: clang two-step with lld, debug mode
optim3.elf: optim3.o ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a
	clang -flto=thin -fuse-ld=lld -L . -o optim3.elf optim3.o ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a -Wl,--gc-sections

optim4.o: $(ALL_HEADERS)
	clang -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -g -o optim4.o

# optim4.elf: clang two-step with lld, release mode with debug symbols
optim4.elf: optim4.o ../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi.a
	clang -flto=thin -fuse-ld=lld -L . -o optim4.elf optim4.o ../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections

optim5.o: $(ALL_HEADERS)
	clang -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -o optim5.o

# optim5.elf: clang two-step with lld, release mode stripped of debug symbols
optim5.elf: optim5.o ../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi.a
	clang -flto=thin -fuse-ld=lld -L . -o optim5.elf optim5.o ../../../../target/x86_64-unknown-linux-gnu/release/libicu_capi.a -Wl,--gc-sections -Wl,--strip-all

build: a.out

test: build
	./a.out

build-optim: optim.elf optim2.elf optim3.elf optim4.elf optim5.elf

# note: optim2.elf and optim3.elf crash when run with error "Illegal instruction" (investigate?)
test-optim: build-optim
	./optim.elf
	./optim4.elf
	./optim5.elf
