# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export CARGO_TARGET_DIR ?= target
export MODE ?= release

all: clean baked_data/mod.rs

../bin/icu4x-datagen:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	ifeq ($(MODE), "release")
		cargo install --path ../../../provider/icu4x-datagen --root ..
	else
		cargo install --path ../../../provider/icu4x-datagen --debug --root ..
	endif

baked_data/mod.rs: ../bin/icu4x-datagen
	../bin/icu4x-datagen \
		--format baked \
		--markers all \
		--locales ru \
		--overwrite \
		--out baked_data

.PHONY: clean
clean:
	git clean -xf *
