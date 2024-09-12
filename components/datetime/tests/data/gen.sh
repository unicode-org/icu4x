#!/bin/sh
cargo run -p icu4x-datagen -- \
--markers \
GregorianDateLengthsV1Marker \
GregorianDateSymbolsV1Marker \
TimeLengthsV1Marker \
TimeSymbolsV1Marker \
DecimalSymbolsV1Marker \
TimeZoneFormatsV1Marker \
MetazoneSpecificNamesShortV1Marker \
MetazonePeriodV1Marker \
--locales en \
--format blob2 \
--out $(dirname $0)/blob.postcard \
--overwrite