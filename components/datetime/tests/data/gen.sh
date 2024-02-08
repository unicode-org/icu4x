#!/bin/sh
cargo run -p icu_datagen -- \
--keys "datetime/gregory/datelengths@1" "datetime/gregory/datesymbols@1" "datetime/timelengths@1" "datetime/timesymbols@1" "decimal/symbols@1" "time_zone/formats@1" "time_zone/specific_short@1" \
--locales en \
--format blob2 \
--out $(dirname $0)/blob.postcard \
--overwrite