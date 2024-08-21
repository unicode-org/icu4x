// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::HashSet;

use icu::experimental::dimension::provider::currency_patterns::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use zerovec::ZeroMap;

impl DataProvider<CurrencyPatternsDataV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyPatternsDataV1Marker>, DataError> {
        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let default_system = numbers_resource
            .main
            .value
            .numbers
            .default_numbering_system
            .as_str();

        let currency_patterns = &numbers_resource
            .main
            .value
            .numbers
            .numsys_data
            .currency_patterns;

        // `default_patterns` is the patterns that came from the default numbering system
        let patterns = &currency_patterns
            .get(default_system)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyPatternsDataV1 {
                // TODO(#5334):
                //      Before graduating the currency crate,
                //      Check that the .json data files are completed and no need to fallback chain up to the root.
                unit_patterns: ZeroMap::from_iter(
                    [
                        (PatternCount::Zero, patterns.pattern_zero.as_deref()),
                        (PatternCount::One, patterns.pattern_one.as_deref()),
                        (PatternCount::Two, patterns.pattern_two.as_deref()),
                        (PatternCount::Few, patterns.pattern_few.as_deref()),
                        (PatternCount::Many, patterns.pattern_many.as_deref()),
                        (PatternCount::Other, patterns.pattern_other.as_deref()),
                    ]
                    .into_iter()
                    .filter_map(|(count, pattern)| match (count, pattern) {
                        (PatternCount::Other, pattern) => Some((count, pattern?)),
                        // NOTE:
                        //      According to [Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-numbers.html),
                        //      if a specific count is not available, the `other` pattern should be used.
                        //      Therefore, if the pattern is equal to the `other` pattern, we should not include it in the map.
                        (_, pattern) if pattern == patterns.pattern_other.as_deref() => None,
                        _ => Some((count, pattern?)),
                    }),
                ),
            }),
        })
    }
}

impl IterableDataProviderCached<CurrencyPatternsDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();
    let en: DataPayload<CurrencyPatternsDataV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;
    let patterns_en = en.get().to_owned().unit_patterns;
    assert_eq!(patterns_en.get(&PatternCount::Zero), None);
    assert_eq!(patterns_en.get(&PatternCount::One), None);
    assert_eq!(patterns_en.get(&PatternCount::Other), Some("{0} {1}"));

    let ar: DataPayload<CurrencyPatternsDataV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("ar-EG").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let patterns_ar = ar.get().to_owned().unit_patterns;
    assert_eq!(patterns_ar.get(&PatternCount::Zero), None);
    assert_eq!(patterns_ar.get(&PatternCount::One), None);
    assert_eq!(patterns_ar.get(&PatternCount::Other), Some("{0} {1}"));

    let jp: DataPayload<CurrencyPatternsDataV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("ja").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let patterns_jp = jp.get().to_owned().unit_patterns;
    assert_eq!(patterns_jp.get(&PatternCount::Zero), None);
    assert_eq!(patterns_jp.get(&PatternCount::One), None);
    assert_eq!(patterns_jp.get(&PatternCount::Other), Some("{0}{1}"));
}
