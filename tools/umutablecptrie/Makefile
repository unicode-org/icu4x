# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

ICU4C_SRC:=/home/sffc/projects/icu_Laptop1/icu4c/source

list_to_ucptrie: list_to_ucptrie.cpp
	clang++-12 -o list_to_ucptrie -ldl -I$(ICU4C_SRC)/common -I$(ICU4C_SRC)/tools/toolutil list_to_ucptrie.cpp $(ICU4C_SRC)/lib/libicutu.a $(ICU4C_SRC)/lib/libicuuc.a
