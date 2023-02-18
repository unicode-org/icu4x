# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export CARGO_TARGET_DIR ?= target

all: buffer_data.postcard

FORCE:

bin/icu4x-datagen:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo install icu_datagen --root .

# Note: the second `echo` statement below can be replaced by `cargo build --release`.
# This Makefile does not run it because ICU4X CI requires custom `cargo build` configs.
bin/tutorial_buffer: FORCE
	echo "Target dir: $(CARGO_TARGET_DIR)"
	echo "If the following command fails, run cargo build --release first."
	cp -p "$(CARGO_TARGET_DIR)"/release/tutorial_buffer bin/tutorial_buffer

buffer_data.postcard: bin/icu4x-datagen bin/tutorial_buffer
	./bin/icu4x-datagen \
		--format blob \
		--keys-for-bin bin/tutorial_buffer \
		--locales my en-ZA \
		--cldr-tag latest \
		--overwrite \
		--out buffer_data.postcard
