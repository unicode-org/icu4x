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

a.out: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) test.c
	gcc test.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g

optim.elf: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) test.c
	gcc -fdata-sections -ffunction-sections test.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g -o optim.elf -Wl,--gc-sections -Wl,--strip-all

optim2.elf: ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a $(ALL_HEADERS) test.c
	clang -flto -fdata-sections -ffunction-sections test.c ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a -g -o optim2.elf -Wl,--gc-sections

../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a: $(ALL_RUST)
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-flto -Cpanic=abort" cargo +nightly-2021-02-28 panic-abort-build --target x86_64-unknown-linux-gnu --no-default-features --features x86tiny

optim3.o: $(ALL_HEADERS)
	clang -c -flto=thin --target=x86_64-unknown-linux-gnu test.c -g -o optim3.o

optim3.elf: optim3.o ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a
	clang -flto=thin -fuse-ld=lld -L . -o optim3.elf optim3.o ../../../../target/x86_64-unknown-linux-gnu/debug/libicu_capi.a

build: a.out optim.elf optim2.elf

test: build
	./a.out
