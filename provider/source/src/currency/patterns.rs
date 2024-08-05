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

        let patterns = &numbers_resource
            .main
            .value
            .numbers
            .numsys_data
            .currency_patterns
            .get(default_system)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyPatternsDataV1 {
                unit_patterns: ZeroMap::from_iter(
                    [
                        (UnitPattern::Zero, patterns.pattern_zero.as_deref()),
                        (UnitPattern::One, patterns.pattern_one.as_deref()),
                        (UnitPattern::Two, patterns.pattern_two.as_deref()),
                        (UnitPattern::Few, patterns.pattern_few.as_deref()),
                        (UnitPattern::Many, patterns.pattern_many.as_deref()),
                        (UnitPattern::Other, patterns.pattern_other.as_deref()),
                    ]
                    .into_iter()
                    .filter_map(|(k, v)| match v {
                        Some(v) => Some((k, v)),
                        None => None,
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
