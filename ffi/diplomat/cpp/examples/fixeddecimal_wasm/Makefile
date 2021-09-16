# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
.PHONY: build test clean serve build-host test-host

ALL_HEADERS := $(wildcard ../../include/*.hpp) $(wildcard ../../../capi/include/*.h)
ALL_RUST := $(wildcard ../../../capi//src/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):

../../../../target/debug/libicu_capi.a: $(ALL_RUST)
	cargo build -p icu_capi

a.out: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) test.cpp
	g++ -std=c++17 test.cpp ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g

../../../../target/wasm32-unknown-emscripten/release/libicu_capi.a: $(ALL_RUST)
	RUSTFLAGS="-Cpanic=abort" cargo +nightly-2021-08-20 build --release -p icu_capi --target wasm32-unknown-emscripten -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort

web-version.html: ../../../../target/wasm32-unknown-emscripten/release/libicu_capi.a $(ALL_HEADERS) test.cpp
	emcc -std=c++17 test.cpp ../../../../target/wasm32-unknown-emscripten/release/libicu_capi.a -ldl -lpthread -lm -g  -o web-version.html --bind --emrun -sENVIRONMENT=web -sWASM=1 -sEXPORT_ES6=1 -sMODULARIZE=1

node-version.js: ../../../../target/wasm32-unknown-emscripten/release/libicu_capi.a $(ALL_HEADERS) test.cpp
	emcc -std=c++17 test.cpp ../../../../target/wasm32-unknown-emscripten/release/libicu_capi.a -ldl -lpthread -lm -g  -o node-version.js --bind -sWASM=1 -sENVIRONMENT=node -sWASM_ASYNC_COMPILATION=0 -DNOMAIN

build: web-version.html node-version.js

test: node-version.js
	exec node ./node-test.js

serve: web-version.html
	emrun web-version.html

# These make it possible to ensure that the C++ code is up to date with the bindings
# without needing to set up emsdk. This way `make test-ffi` works without emsdk.
build-host: a.out

test-host: build-host
	./a.out

clean:
	rm -f web-version.html
	rm -f web-version.wasm
	rm -f web-version.js
	rm -f node-version.js
	rm -f node-version.wasm
	rm -f ../../../../target/wasm32-unknown-emscripten/release/libicu_capi.a
	rm -f ../../../../target/debug/libicu_capi.a
	rm -f a.out
