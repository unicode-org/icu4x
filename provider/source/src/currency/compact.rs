// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::BTreeMap;
use std::collections::HashSet;

use icu::experimental::dimension::provider::currency::compact::*;
use icu::plurals::PluralCategory;
use icu_provider::prelude::*;
use icu_provider::DataProvider;

impl DataProvider<ShortCurrencyCompactV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ShortCurrencyCompactV1>, DataError> {
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

        let compact_patterns = match currency_patterns
            .get(default_system)
            .and_then(|patterns| patterns.compact_short.as_ref())
            .map(|short_compact| &short_compact.standard.patterns)
        {
            Some(patterns) => patterns,
            None => {
                return Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(ShortCurrencyCompact {
                        compact_patterns: Default::default(),
                    }),
                })
            }
        };

        /// Try to parse a compact count from a string.
        fn try_parse_count_from_str(value: &str) -> Result<CompactCount, DataError> {
            let (count_str, is_alpha_next) = match value.split_once("-alt-alphaNextToNumber") {
                Some((count, _)) => (count, true),
                None => (value, false),
            };

            let count = match count_str {
                "zero" => PluralCategory::Zero,
                "one" => PluralCategory::One,
                "two" => PluralCategory::Two,
                "few" => PluralCategory::Few,
                "many" => PluralCategory::Many,
                "other" => PluralCategory::Other,
                _ => return Err(DataErrorKind::IdentifierNotFound.into_error()),
            };

            Ok(if is_alpha_next {
                CompactCount::AlphaNextToNumber(count)
            } else {
                CompactCount::Standard(count)
            })
        }

        let mut result = BTreeMap::new();

        for pattern in compact_patterns {
            let lg10 = pattern.magnitude.chars().filter(|&c| c == '0').count() as i8;

            if lg10 + 1 != pattern.magnitude.len() as i8 {
                return Err(DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("the number of zeros must be one less than the number of digits in the compact decimal count"));
            }

            let count = try_parse_count_from_str(pattern.count.as_str())?;

            result.insert((lg10, count), pattern.pattern.as_str());
        }

        // Deduplicate against `::Other`
        let compact_patterns = result
            .iter()
            .filter(|(&(lg10, count), pattern)| {
                let (CompactCount::AlphaNextToNumber(p) | CompactCount::Standard(p)) = count;
                p == PluralCategory::Other
                    || result.get(&(
                        lg10,
                        match count {
                            CompactCount::AlphaNextToNumber(_) => {
                                CompactCount::AlphaNextToNumber(PluralCategory::Other)
                            }
                            CompactCount::Standard(_) => {
                                CompactCount::Standard(PluralCategory::Other)
                            }
                        },
                    )) != Some(pattern)
            })
            .map(|(k, v)| (*k, *v))
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ShortCurrencyCompact { compact_patterns }),
        })
    }
}

impl IterableDataProviderCached<ShortCurrencyCompactV1> for SourceDataProvider {
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
    use icu::experimental::dimension::provider::currency::compact::*;
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();
    let en: DataResponse<ShortCurrencyCompactV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap();

    let en_patterns = &en.payload.get().to_owned().compact_patterns;

    assert_eq!(
        en_patterns.get(&(3, CompactCount::Standard(PluralCategory::One))),
        None
    );
    assert_eq!(
        en_patterns.get(&(3, CompactCount::AlphaNextToNumber(PluralCategory::One))),
        None
    );

    assert_eq!(
        en_patterns.get(&(3, CompactCount::Standard(PluralCategory::Other))),
        Some("¤0K")
    );
    assert_eq!(
        en_patterns.get(&(3, CompactCount::AlphaNextToNumber(PluralCategory::Other))),
        Some("¤ 0K")
    );

    let ja: DataResponse<ShortCurrencyCompactV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ja").into()),
            ..Default::default()
        })
        .unwrap();

    let ja_patterns = &ja.payload.get().to_owned().compact_patterns;

    assert_eq!(
        ja_patterns.get(&(4, CompactCount::Standard(PluralCategory::Other))),
        Some("¤0万")
    );
    assert_eq!(
        ja_patterns.get(&(4, CompactCount::AlphaNextToNumber(PluralCategory::Other))),
        Some("¤\u{a0}0万")
    );
}
