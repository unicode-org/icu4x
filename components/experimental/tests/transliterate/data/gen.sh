#!/bin/sh
cargo run -p icu4x-datagen --features experimental_components -- \
--markers "transliterator/rules@1" \
--locales full \
--runtime-fallback-location external \
--cldr-root $(dirname $0)/../../../../../provider/datagen/tests/data/cldr \
--format mod \
--out $(dirname $0)/baked \
--pretty \
--overwrite