# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

ICU4C_SOURCE ?= $(HOME)/icu/icu4c/source
ICU4C_BUILD ?= $(ICU4C_SOURCE)
CXX := clang++
WASICXX := wasic++

.PHONY: all clean
all: list_to_ucptrie list_to_ucptrie.wasm

icu4c_obj/wasm/%.o: $(ICU4C_SOURCE)/%.cpp
	mkdir -p icu4c_obj/wasm/common
	mkdir -p icu4c_obj/wasm/tools/toolutil
	$(WASICXX) -DTRUE=1 -c -flto -I$(ICU4C_SOURCE)/common -I$(ICU4C_SOURCE)/tools/toolutil $< -o $@

icu4c_obj_wasm_files = \
icu4c_obj/wasm/common/cmemory.o \
icu4c_obj/wasm/common/errorcode.o \
icu4c_obj/wasm/common/ucptrie.o \
icu4c_obj/wasm/common/umutablecptrie.o \
icu4c_obj/wasm/common/uobject.o \
icu4c_obj/wasm/common/utypes.o \
icu4c_obj/wasm/tools/toolutil/writesrc.o

list_to_ucptrie: list_to_ucptrie.cpp $(icu4c_obj_native_files)
	$(CXX) -o list_to_ucptrie -ldl -I$(ICU4C_SOURCE)/common -I$(ICU4C_SOURCE)/tools/toolutil list_to_ucptrie.cpp $(ICU4C_BUILD)/lib/libicutu.a $(ICU4C_BUILD)/lib/libicuuc.a

list_to_ucptrie.wasm: list_to_ucptrie.cpp $(icu4c_obj_wasm_files)
	$(WASICXX) -DTRUE=1 -flto -Wl,--gc-sections -Wl,--strip-all -I$(ICU4C_SOURCE)/common -I$(ICU4C_SOURCE)/tools/toolutil $(icu4c_obj_wasm_files) list_to_ucptrie.cpp -o list_to_ucptrie.wasm

clean:
	rm -rf icu4c_obj
	rm -f list_to_ucptrie
	rm -f list_to_ucptrie.wasm
