# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

FEATURES ?= "compiled_data,default_components,logging"


ifeq ($(OS),Windows_NT)
  DYLIB_EXT = "dll"
  DYLIB_PREFIX = ""
else
	UNAME := $(shell uname)
	ifeq ($(UNAME), Darwin)
  	DYLIB_EXT = "dylib"
  	DYLIB_PREFIX = "lib"
  else
  	DYLIB_EXT = "so"
  	DYLIB_PREFIX = "lib"
  endif
endif

../../target/release/$(DYLIB_PREFIX)icu_capi.$(DYLIB_EXT): FORCE
	cargo rustc \
		--release \
		-p icu_capi \
		--crate-type cdylib \
		--no-default-features \
		--features ${FEATURES}

src/main/native/$(DYLIB_PREFIX)icu4x.$(DYLIB_EXT): ../../target/release/$(DYLIB_PREFIX)icu_capi.$(DYLIB_EXT)
	cp ../../target/release/$(DYLIB_PREFIX)icu_capi.$(DYLIB_EXT) src/main/native/$(DYLIB_PREFIX)icu4x.$(DYLIB_EXT)

build: src/main/native/$(DYLIB_PREFIX)icu4x.$(DYLIB_EXT)
