# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

lib/src/lib.g.dart:
	rm -rf lib/src
	cp -r ../capi/bindings/dart lib/src

../../target/release/libicu_capi_cdylib.so: FORCE
	cargo build -p icu_capi_cdylib --release

test/libicu_capi_cdylib.so: ../../target/release/libicu_capi_cdylib.so
	cp ../../target/release/libicu_capi_cdylib.so test/libicu_capi_cdylib.so

build: lib/src/lib.g.dart test/libicu_capi_cdylib.so
	
test: build
	dart test
