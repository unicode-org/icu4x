# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

ICU4C_SOURCE ?= $(HOME)/icu/icu4c/source
CXX := clang++
WASM2WAT := wasm2wat

.PHONY: all clean
all: ucptrie_wrap.wasm

# The objects are built with the following flags:
# --target=wasm32-unknown-wasi to build with WASI for system symbols
# -DTRUE=1 to fix error use of undeclared identifier 'TRUE'
# -DU_DISABLE_RENAMING so the same WASM module works across ICU versions
# --compile to skip the linking step
# -flto to include LTO (link-time optimization) metadata

wasm_obj/icu4c/%.o: $(ICU4C_SOURCE)/%.cpp
	mkdir -p wasm_obj/icu4c/common
	$(CXX) --target=wasm32-unknown-wasi \
		-DTRUE=1 \
		-DU_DISABLE_RENAMING=1 \
		--compile \
		-flto \
		-I$(ICU4C_SOURCE)/common \
		$< \
		-o $@

wasm_obj/ucptrie_wrap.o: ucptrie_wrap.cpp
	mkdir -p wasm_obj
	$(CXX) --target=wasm32-unknown-wasi \
		-I/usr/include/wasm32-wasi \
		--compile \
		-flto \
		-I$(ICU4C_SOURCE)/common \
		$< \
		-o $@

wasm_obj_files = \
	wasm_obj/icu4c/common/cmemory.o \
	wasm_obj/icu4c/common/errorcode.o \
	wasm_obj/icu4c/common/ucptrie.o \
	wasm_obj/icu4c/common/umutablecptrie.o \
	wasm_obj/icu4c/common/uobject.o \
	wasm_obj/icu4c/common/utypes.o \
	wasm_obj/ucptrie_wrap.o

# Compile as a Reactor Module so we don't need a main function.
# This also means we don't need WASI imports at runtime.
# <https://dylibso.com/blog/wasi-command-reactor/>
ucptrie_wrap.wasm: $(wasm_obj_files)
	$(CXX) --target=wasm32-unknown-wasi \
		-mexec-model=reactor \
		-DTRUE=1 \
		-flto \
		-Wl,--gc-sections \
		-Wl,--strip-all \
		-Wl,--export=umutablecptrie_open \
		-Wl,--export=umutablecptrie_set \
		-Wl,--export=umutablecptrie_buildImmutable \
		-Wl,--export=ucptrie_close \
		-Wl,--export=umutablecptrie_close \
		-Wl,--export=create_uerrorcode \
		-Wl,--export=read_uerrorcode \
		-Wl,--export=read_ucptrie_highStart \
		-Wl,--export=read_ucptrie_shifted12HighStart \
		-Wl,--export=read_ucptrie_index3NullOffset \
		-Wl,--export=read_ucptrie_dataNullOffset \
		-Wl,--export=read_ucptrie_nullValue \
		-Wl,--export=get_index_ptr \
		-Wl,--export=get_index_length \
		-Wl,--export=get_data_ptr \
		-Wl,--export=get_data_length \
		-I$(ICU4C_SOURCE)/common \
		-I$(ICU4C_SOURCE)/tools/toolutil \
		$(wasm_obj_files) \
		-g \
		-o ucptrie_wrap.wasm

%.wat: %.wasm
	$(WASM2WAT) $< -o $@

clean:
	rm -rf wasm_obj
	rm -f ucptrie_wrap.wasm
