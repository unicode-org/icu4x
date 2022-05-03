# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

ICU4C_SRC:=$(HOME)/icu/icu4c/source

run_icu_breakiterator: run_icu_breakiterator.cpp
	clang++-12 -g -o run_icu_breakiterator -lpthread -ldl -I$(ICU4C_SRC)/common run_icu_breakiterator.cpp $(ICU4C_SRC)/lib/libicuuc.a $(ICU4C_SRC)/lib/libicudata.a
