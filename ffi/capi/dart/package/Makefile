# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

FORCE:

../../../../target/debug/libicu_capi_cdylib.so: FORCE
	cargo build -p icu_capi_cdylib

test/libicu_capi_cdylib.so: ../../../../target/debug/libicu_capi_cdylib.so
	cp ../../../../target/debug/libicu_capi_cdylib.so test/libicu_capi_cdylib.so

test: test/libicu_capi_cdylib.so
	dart test test/test.dart
