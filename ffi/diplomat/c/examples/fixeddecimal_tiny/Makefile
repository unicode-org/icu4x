# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test

GCC := gcc
CLANG := clang-14
LLD := lld-14
TARGET := x86_64-unknown-linux-gnu

../../../../../target/debug/libicu_capi_staticlib.a:
	cargo build -p icu_capi_staticlib --no-default-features --features buffer_provider

../../../../../target/$(TARGET)/debug/libicu_capi_staticlib.a:
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" \
	cargo +nightly-2022-04-05 --target=$(TARGET) \
	panic-abort-build -p icu_capi_staticlib --no-default-features --features x86tiny,buffer_provider

../../../../../target/$(TARGET)/release-opt-size/libicu_capi_staticlib.a:
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" \
	cargo +nightly-2022-04-05 --target=$(TARGET) \
	panic-abort-build -p icu_capi_staticlib --no-default-features --features x86tiny,buffer_provider --profile=release-opt-size

decimal-bn-en.postcard:
	cargo run -p icu_datagen --features bin -- --locales en bn --keys "decimal/symbols@1" --cldr-root ../../../../../provider/testdata/data/cldr/ --format blob --out decimal-bn-en.postcard

decimal_bn_en.h: decimal-bn-en.postcard
	xxd -i -C decimal-bn-en.postcard > decimal_bn_en.h

# Naive target: no optimizations, full std
optim0.elf: ../../../../../target/debug/libicu_capi_staticlib.a test.c decimal_bn_en.h
	$(GCC) --target=$(TARGET) test.c ../../../../../target/debug/libicu_capi_staticlib.a \
	-ldl -lpthread -lm -g \
	-o optim0.elf

# optim.elf: gcc with maximum link-time code stripping (gc-sections and strip-all)
optim1.elf: ../../../../../target/debug/libicu_capi_staticlib.a test.c decimal_bn_en.h
	$(GCC) --target=$(TARGET) test.c ../../../../../target/debug/libicu_capi_staticlib.a \
	-ldl -lpthread -lm -g -fdata-sections -ffunction-sections $(GC_SECTIONS) $(STRIP_ALL) \
	-o optim1.elf

# optim2.elf: clang single-step with gc-sections
optim2.elf: ../../../../../target/$(TARGET)/debug/libicu_capi_staticlib.a test.c decimal_bn_en.h
	$(CLANG) --target=$(TARGET) test.c ../../../../../target/$(TARGET)/debug/libicu_capi_staticlib.a \
	-fdata-sections -ffunction-sections -flto -g $(GC_SECTIONS) \
	-o optim2.elf

optim34.o: test.c decimal_bn_en.h
	$(CLANG) --target=$(TARGET) test.c \
	-c -fdata-sections -ffunction-sections -flto=thin -g \
	-o optim34.o

# optim3.elf: clang two-step with lld, debug mode
optim3.elf: optim34.o ../../../../../target/$(TARGET)/debug/libicu_capi_staticlib.a
	$(CLANG) --target=$(TARGET) optim34.o ../../../../../target/$(TARGET)/debug/libicu_capi_staticlib.a \
	-flto=thin -fuse-ld=$(LLD) $(GC_SECTIONS) -L . \
	-o optim3.elf

# optim4.elf: clang two-step with lld, release mode with debug symbols
optim4.elf: optim34.o ../../../../../target/$(TARGET)/release-opt-size/libicu_capi_staticlib.a
	$(CLANG) --target=$(TARGET) optim34.o ../../../../../target/$(TARGET)/release-opt-size/libicu_capi_staticlib.a \
	-flto=thin -fuse-ld=$(LLD) $(GC_SECTIONS) -L . \
	-o optim4.elf

optim5.o: test.c decimal_bn_en.h
	$(CLANG) --target=$(TARGET) test.c \
	-c -flto=thin -fdata-sections -ffunction-sections \
	-o optim5.o

# optim5.elf: clang two-step with lld, release mode stripped of debug symbols
optim5.elf: optim5.o ../../../../../target/$(TARGET)/release-opt-size/libicu_capi_staticlib.a
	$(CLANG) --target=$(TARGET) optim5.o ../../../../../target/$(TARGET)/release-opt-size/libicu_capi_staticlib.a \
	-flto=thin -fuse-ld=$(LLD) $(GC_SECTIONS) -L . $(STRIP_ALL) \
	-o optim5.elf 

build: optim0.elf optim1.elf optim2.elf optim3.elf optim4.elf optim5.elf

test: build
	./optim0.elf
	./optim1.elf
	./optim2.elf
	./optim3.elf
	./optim4.elf
	./optim5.elf
