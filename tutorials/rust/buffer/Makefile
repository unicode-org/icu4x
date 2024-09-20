# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export CARGO_TARGET_DIR ?= target
export MODE ?= release

CLANG := clang-16
PINNED_CI_NIGHTLY = "nightly-2024-07-23"

all: buffer_data.postcard bin/tutorial_buffer.wasm

../bin/icu4x-datagen:
	echo "Target dir: $(CARGO_TARGET_DIR)"
ifeq ($(MODE), "release")
	cargo install --path ../../../provider/icu4x-datagen --root ..
else
	cargo install --path ../../../provider/icu4x-datagen --debug --root ..
endif

bin/tutorial_buffer:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	cargo build --release
	mkdir -p bin
	cp -p "$(CARGO_TARGET_DIR)"/release/tutorial_buffer bin/tutorial_buffer
	ls -l bin/

bin/tutorial_buffer.wasm:
	echo "Target dir: $(CARGO_TARGET_DIR)"
	rustup toolchain install ${PINNED_CI_NIGHTLY}
	rustup target add wasm32-wasip1 --toolchain ${PINNED_CI_NIGHTLY}
	cargo +${PINNED_CI_NIGHTLY} build --target wasm32-wasip1 --release
	mkdir -p bin
	cp -p "$(CARGO_TARGET_DIR)"/wasm32-wasip1/release/tutorial_buffer.wasm bin/
	ls -l bin/

buffer_data.postcard: ../bin/icu4x-datagen bin/tutorial_buffer
	../bin/icu4x-datagen \
		--format blob2 \
		--markers-for-bin bin/tutorial_buffer \
		--locales my en-ZA \
		--cldr-tag latest \
		--overwrite \
		--out buffer_data.postcard

clean:
	git clean -xf *
