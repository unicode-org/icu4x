# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test
FORCE:

ICU_CAPI := $(shell cargo metadata --format-version 1 | jq '.packages[] | select(.name == "icu_capi").manifest_path' | xargs dirname)
HEADERS := ${ICU_CAPI}/bindings/cpp
ALL_HEADERS := $(wildcard ${HEADERS}/*)

CXX?=g++

TEST_FILES := $(wildcard *.cpp)
OUT_FILES = $(subst .cpp,,$(TEST_FILES))

$(ALL_HEADERS):

../../../../target/debug/libicu_capi_staticlib.a: FORCE
	cargo build -p icu_capi_staticlib --features provider_fs,icu_provider/deserialize_json 

%: ../../../../target/debug/libicu_capi_staticlib.a $(ALL_HEADERS) %.cpp
	$(CXX) -std=c++17 $@.cpp ../../../../target/debug/libicu_capi_staticlib.a -I${HEADERS} -lm -g -o$@ $<
	./$@ $<

test: $(OUT_FILES)

clean:
	rm $(OUT_FILES)
