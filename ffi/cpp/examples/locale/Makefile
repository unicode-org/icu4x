# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard ../../include/*.hpp) $(wildcard ../../../capi/include/*.h)
ALL_RUST := $(wildcard ../../../capi//src/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):


../../../../target/debug/libicu_capi.a: $(ALL_RUST)
	cargo build -p icu_capi

a.out: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) test.cpp
	g++ -std=c++17 test.cpp ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g

build: a.out

test: build
	./a.out