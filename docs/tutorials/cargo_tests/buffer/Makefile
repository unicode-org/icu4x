# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export CARGO_TARGET_DIR ?= target

all: buffer_data.postcard

bin/icu4x-datagen:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo install icu_datagen --root .

bin/demo_buffer: Cargo.toml src/*.rs
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo build --release
	cp "$(CARGO_TARGET_DIR)"/release/demo_buffer bin/demo_buffer

buffer_data.postcard: bin/icu4x-datagen bin/demo_buffer
	./bin/icu4x-datagen \
		--format blob \
		--keys-for-bin bin/demo_buffer \
		--locales my en-ZA \
		--cldr-tag 42.0.0 \
		--overwrite \
		--out buffer_data.postcard
