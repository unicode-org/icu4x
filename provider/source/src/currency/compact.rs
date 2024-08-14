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

        let mut result = ZeroMap2d::new();

        let compact_patterns = match currency_patterns
            .get(default_system)
            .and_then(|patterns| patterns.compact_short.as_ref())
            .map(|short_compact| &short_compact.standard.patterns)
        {
            Some(patterns) => patterns,
            None => {
                return Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(CurrencyCompactV1 {
                        compact_patterns: result,
                    }),
                })
            }
        };

        for pattern in compact_patterns {
            let lg10 = pattern
                .compact_decimal_type
                .chars()
                .filter(|&c| c == '0')
                .count() as i8;

            if lg10 + 1 != pattern.compact_decimal_type.len() as i8 {
                return Err(DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("the number of zeros must be one less than the number of digits in the compact decimal count"));
            }

            let count = CompactCount::try_from(pattern.compact_decimal_count.as_str())
                .map_err(|_| DataErrorKind::IdentifierNotFound.into_error())?;

            result.insert(&lg10, &count, pattern.pattern.as_str());
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyCompactV1 {
                compact_patterns: result,
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

#[test]
fn test_basic() {
    use icu::experimental::dimension::provider::currency_compact::*;
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();
    let en: DataResponse<CurrencyCompactV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap();

    let en_patterns = &en.payload.get().to_owned().compact_patterns;

    assert_eq!(en_patterns.get_2d(&3, &CompactCount::One), Some("¤0K"));
    assert_eq!(en_patterns.get_2d(&3, &CompactCount::OneAlt), Some("¤ 0K"));

    let ja: DataResponse<CurrencyCompactV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ja").into()),
            ..Default::default()
        })
        .unwrap();

    let ja_patterns = &ja.payload.get().to_owned().compact_patterns;

    assert_eq!(ja_patterns.get_2d(&4, &CompactCount::Other), Some("¤0万"));
    assert_eq!(
        ja_patterns.get_2d(&4, &CompactCount::OtherAlt),
        Some("¤\u{a0}0万")
    );
}
