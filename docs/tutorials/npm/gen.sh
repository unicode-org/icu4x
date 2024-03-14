#!/bin/bash

locales=("ja" "th" "zh" "bn" "und" "de" "en")

for locale in "${locales[@]}"; do
  icu4x-datagen --keys \
    "datetime/symbols/hebrew/years@1" \
    "segmenter/dictionary/w_auto@1" \
    "datetime/symbols/weekdays@1" \
    "datetime/symbols/dangi/months@1" \
    "time_zone/iana_to_bcp47@1" \
    "datetime/skeletons@1" \
    "time_zone/bcp47_to_iana@1" \
    "datetime/patterns/chinese/date@1" \
    "datetime/roc/datesymbols@1" \
    "datetime/patterns/coptic/date@1" \
    "datetime/symbols/persian/months@1" \
    "datetime/hebrew/datelengths@1" \
    "decimal/symbols@1" \
    "datetime/symbols/islamic/years@1" \
    "datetime/symbols/indian/months@1" \
    "locid_transform/script_dir@1" \
    "fallback/supplement/co@1" \
    "datetime/symbols/buddhist/months@1" \
    "datetime/patterns/ethiopic/date@1" \
    "plurals/ranges@1" \
    "segmenter/grapheme@1" \
    "time_zone/metazone_period@1" \
    "datetime/persian/datelengths@1" \
    "datetime/ethiopic/datesymbols@1" \
    "time_zone/generic_short@1" \
    "datetime/symbols/japanese/months@1" \
    "time_zone/generic_long@1" \
    "normalizer/nfkd@1" \
    "datetime/patterns/persian/date@1" \
    "datetime/symbols/ethiopic/months@1" \
    "datetime/patterns/buddhist/date@1" \
    "locid_transform/likelysubtags_l@1" \
    "datetime/symbols/persian/years@1" \
    "datetime/patterns/hebrew/date@1" \
    "datetime/symbols/buddhist/years@1" \
    "datetime/chinese/datelengths@1" \
    "datetime/symbols/japanext/months@1" \
    "plurals/ordinal@1" \
    "datetime/symbols/coptic/years@1" \
    "datetime/patterns/islamic/date@1" \
    "datetime/patterns/time@1" \
    "datetime/symbols/dangi/years@1" \
    "time_zone/formats@1" \
    "datetime/patterns/dangi/date@1" \
    "datetime/japanese/datelengths@1" \
    "datetime/ethiopic/datelengths@1" \
    "plurals/cardinal@1" \
    "datetime/indian/datelengths@1" \
    "segmenter/lstm/wl_auto@1" \
    "datetime/symbols/chinese/years@1" \
    "fallback/parents@1" \
    "datetime/gregory/datelengths@1" \
    "datetime/patterns/gregory/date@1" \
    "datetime/symbols/japanext/years@1" \
    "datetime/gregory/datesymbols@1" \
    "datetime/week_data@1" \
    "time_zone/exemplar_cities@1" \
    "datetime/coptic/datelengths@1" \
    "segmenter/dictionary/wl_ext@1" \
    "list/or@1" \
    "datetime/dangi/datelengths@1" \
    "datetime/persian/datesymbols@1" \
    "datetime/buddhist/datelengths@1" \
    "normalizer/nfd@1" \
    "datetime/patterns/roc/date@1" \
    "time_zone/specific_short@1" \
    "datetime/japanese/datesymbols@1" \
    "segmenter/line@1" \
    "datetime/symbols/hebrew/months@1" \
    "datetime/buddhist/datesymbols@1" \
    "datetime/symbols/ethiopic/years@1" \
    "datetime/coptic/datesymbols@1" \
    "normalizer/nfkdex@1" \
    "time_zone/specific_long@1" \
    "datetime/roc/datelengths@1" \
    "calendar/japanext@1" \
    "datetime/japanext/datesymbols@1" \
    "normalizer/decomp@1" \
    "datetime/islamic/datelengths@1" \
    "list/unit@1" \
    "datetime/timelengths@1" \
    "segmenter/word@1" \
    "datetime/dangi/datesymbols@1" \
    "datetime/islamic/datesymbols@1" \
    "datetime/symbols/gregory/months@1" \
    "datetime/timesymbols@1" \
    "datetime/symbols/japanese/years@1" \
    "datetime/patterns/japanese/date@1" \
    "datetime/indian/datesymbols@1" \
    "datetime/patterns/japanext/date@1" \
    "datetime/patterns/datetime@1" \
    "segmenter/sentence@1" \
    "datetime/symbols/chinese/months@1" \
    "locid_transform/aliases@1" \
    "datetime/symbols/roc/years@1" \
    "locid_transform/likelysubtags_sr@1" \
    "datetime/symbols/indian/years@1" \
    "datetime/chinese/datesymbols@1" \
    "normalizer/uts46d@1" \
    "locid_transform/likelysubtags@1" \
    "locid_transform/likelysubtags_ext@1" \
    "datetime/symbols/gregory/years@1" \
    "datetime/japanext/datelengths@1" \
    "datetime/symbols/coptic/months@1" \
    "datetime/hebrew/datesymbols@1" \
    "list/and@1" \
    "normalizer/comp@1" \
    "fallback/likelysubtags@1" \
    "datetime/patterns/indian/date@1" \
    "calendar/japanese@1" \
    "datetime/symbols/islamic/months@1" \
    "datetime/symbols/roc/months@1" \
    "datetime/symbols/dayperiods@1" \
    "normalizer/nfdex@1" \
    --fallback preresolved --locales $locale --format blob2 --out dist/$locale.postcard --overwrite
done

ts_content="const locales: string[] = ["

for locale in "${locales[@]}"; do
    ts_content+="\"$locale\", "
done

ts_content=${ts_content%, }
ts_content+="];"

ts_content+="\nexport default locales;"

echo "$ts_content" > dist/locales.ts

echo "locales.ts file has been generated."