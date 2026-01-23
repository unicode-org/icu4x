# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

FEATURES ?= "compiled_data,buffer_provider,default_components,experimental,logging"

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
WASM_STACK_SIZE := 100000

icu_capi.wasm: FORCE
	RUSTFLAGS="-Copt-level=s -Clink-args=-zstack-size=${WASM_STACK_SIZE} -Zwasm-c-abi=spec" \
	TARGET="wasm32-unknown-unknown" \
	TYPE="dynamic" \
	OUT="$(realpath .)/icu_capi.wasm" \
	FEATURES=${FEATURES} \
	bash ../capi/build.sh

build: icu_capi.wasm