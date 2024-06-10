#!/bin/sh
cargo run -p icu_datagen --features experimental_components -- \
--markers "transliterator/rules@1" "normalizer/comp@1" "normalizer/decomp@1" "normalizer/nfd@1" "normalizer/nfdex@1" "normalizer/nfkd@1" "normalizer/nfkdex@1" "normalizer/uts46d@1" \
--locales full \
--runtime-fallback-location external \
--cldr-root $(dirname $0)/../../../../../provider/datagen/tests/data/cldr \
--format mod \
--out $(dirname $0)/baked \
--pretty \
--use-separate-crates \
--overwrite