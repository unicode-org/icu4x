# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

ICU4C_SOURCE ?= $(HOME)/icu/icu4c/source
ICU4C_BUILD ?= $(ICU4C_SOURCE)
CXX := clang++

.PHONY: all clean
all: list_to_ucptrie list_to_ucptrie.wasm

icu4c_obj/wasm/%.o: $(ICU4C_SOURCE)/%.cpp
	mkdir -p icu4c_obj/wasm/common
	$(CXX) --target=wasm32-unknown-wasi \
		-I/usr/include/wasm32-wasi \
		-DTRUE=1 \
		-DU_DISABLE_RENAMING=1 \
		-c \
		-flto \
		-I$(ICU4C_SOURCE)/common \
		$< \
		-o $@

icu4c_obj_wasm_files = \
	icu4c_obj/wasm/common/cmemory.o \
	icu4c_obj/wasm/common/errorcode.o \
	icu4c_obj/wasm/common/ucptrie.o \
	icu4c_obj/wasm/common/umutablecptrie.o \
	icu4c_obj/wasm/common/uobject.o \
	icu4c_obj/wasm/common/utypes.o

ucptrie_wrap.wasm: ucptrie_wrap.cpp $(icu4c_obj_wasm_files)
	$(CXX) --target=wasm32-unknown-wasi \
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
		$(icu4c_obj_wasm_files) \
		ucptrie_wrap.cpp \
		-o ucptrie_wrap.wasm

clean:
	rm -rf icu4c_obj
	rm -f ucptrie_wrap.wasm
