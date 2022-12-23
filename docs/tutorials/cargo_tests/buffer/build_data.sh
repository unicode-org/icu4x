# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
#!/usr/bin/env bash

if ! command -v icu4x-datagen
then
    echo "Installing icu4x-datagen..."
    cargo install --features bin icu_datagen
fi

icu4x-datagen \
    --format blob \
    --keys-for-bin target/debug/buffer \
    --locales my en-ZA \
    --cldr-tag 42.0.0 \
    --overwrite \
    --out buffer_data.postcard
