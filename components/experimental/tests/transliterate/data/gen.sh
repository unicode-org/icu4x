#!/bin/sh
cargo run -p icu4x-datagen --features experimental_components -- \
--markers TransliteratorRulesV1Marker \
--locales full \
--deduplication none \
--no-internal-fallback \
--cldr-root $(dirname $0)/../../../../../provider/source/tests/data/cldr \
--format baked \
--out $(dirname $0)/baked \
--pretty \
--overwrite