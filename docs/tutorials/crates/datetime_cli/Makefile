# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export CARGO_TARGET_DIR ?= target

all: data/ccp.blob

../bin/icu4x-datagen:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo install icu_datagen --root ..

bin/datetime_cli:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo build --release
	mkdir -p bin
	cp -p "$(CARGO_TARGET_DIR)"/release/datetime_cli bin/datetime_cli

data/%.blob: ../bin/icu4x-datagen bin/datetime_cli
	../bin/icu4x-datagen \
		--format blob \
		--keys-for-bin bin/datetime_cli \
		--locales $(notdir $(basename $@)) \
		--cldr-tag latest \
		--overwrite \
		--out $@

.PHONY: clean
clean:
	git clean -xf *
