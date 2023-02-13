# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export CARGO_TARGET_DIR ?= target

all: buffer_data.postcard

FORCE:

bin/icu4x-datagen:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo install icu_datagen --root .

bin/demo_buffer: FORCE
	echo "Target dir: $(CARGO_TARGET_DIR)"
	echo "If the following command fails, run cargo build --release first."
	cp -p "$(CARGO_TARGET_DIR)"/release/demo_buffer bin/demo_buffer

buffer_data.postcard: bin/icu4x-datagen bin/demo_buffer
	./bin/icu4x-datagen \
		--format blob \
		--keys-for-bin bin/demo_buffer \
		--locales my en-ZA \
		--cldr-tag latest \
		--overwrite \
		--out buffer_data.postcard
