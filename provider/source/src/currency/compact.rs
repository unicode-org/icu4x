// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::HashSet;

use icu::experimental::dimension::provider::currency_compact::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use zerovec::ZeroMap;
use zerovec::ZeroMap2d;

impl DataProvider<CurrencyCompactV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyCompactV1Marker>, DataError> {
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
        let _compact_patterns = &currency_patterns
            .get(default_system)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?
            .compact_short;


        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyCompactV1 {
                compact_patterns: ZeroMap2d::new(),
            }),
        })
    }
}

impl IterableDataProviderCached<CurrencyCompactV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}
