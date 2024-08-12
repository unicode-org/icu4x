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
