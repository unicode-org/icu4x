// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::BTreeMap;
use std::collections::HashSet;

use icu::experimental::compactdecimal::provider::CompactPatterns;
use icu::experimental::dimension::provider::currency::compact::*;
use icu::plurals::PluralElements;
use icu_pattern::DoublePlaceholderPattern;
use icu_pattern::QuoteMode;
use icu_provider::prelude::*;
use icu_provider::DataProvider;

#[derive(PartialOrd, Debug, PartialEq, Ord, Eq)]
enum CurrencyPatternKind {
    Standard,
    AlphaNextToNumber,
}

#[derive(PartialEq, Clone, Debug)]
struct ParsedPattern {
    pattern: Box<DoublePlaceholderPattern>,
    number_of_0s: Option<u8>,
}

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
                        standard: Default::default(),
                        alpha_next_to_number: Default::default(),
                    }),
                })
            }
        };

        let mut parsed_patterns: BTreeMap<
            (u8, CurrencyPatternKind),
            BTreeMap<&str, ParsedPattern>,
        > = BTreeMap::new();

        for pattern in compact_patterns {
            let mut type_bytes = pattern.magnitude.bytes();

            if !(type_bytes.next() == Some(b'1') && type_bytes.all(|b| b == b'0')) {
                return Err(DataError::custom("pattern")
                    .with_display_context(&format!("Ill-formed type {}", pattern.magnitude)));
            }
            let log10_type = u8::try_from(pattern.magnitude.len() - 1).map_err(|_| {
                DataError::custom("pattern")
                    .with_display_context(&format!("Too many digits in type {}", pattern.magnitude))
            })?;

            // TODO: use negative patterns?
            let pattern_str = pattern.pattern.split(';').next().unwrap();

            let number_of_0s = pattern_str
                .split('\'')
                .enumerate()
                .filter_map(|(i, chunk)| {
                    (i % 2 == 0)
                        .then(|| chunk.chars().filter(|&c| c == '0').count() as u8)
                        .filter(|&n| n != 0)
                })
                .next();

            let pattern_str = if let Some(number_of_zeros) = number_of_0s {
                pattern_str.replace(
                    &core::iter::repeat_n('0', number_of_zeros as usize).collect::<String>(),
                    "{0}",
                )
            } else {
                pattern_str.into()
            }
            .replace("¤", "{1}");

            let parsed = DoublePlaceholderPattern::try_from_str(
                &pattern_str,
                QuoteMode::QuotingSupported.into(),
            )
            .map_err(|e| DataError::custom("pattern").with_display_context(&e))?;

            if log10_type < number_of_0s.unwrap_or_default() {
                return Err(DataError::custom("pattern").with_display_context(&format!(
                    "Too many 0s in type 10^{}, ({}, implying nonpositive exponent c={}), pattern = {}",
                    log10_type,
                    number_of_0s.unwrap_or_default(),
                    log10_type as i8 - number_of_0s.unwrap_or_default() as i8 + 1,
                    pattern.pattern
                )));
            }

            let (count, is_alpha_next) = match pattern.count.strip_suffix("-alt-alphaNextToNumber")
            {
                Some(count) => (count, CurrencyPatternKind::AlphaNextToNumber),
                None => (&*pattern.count, CurrencyPatternKind::Standard),
            };

            parsed_patterns
                .entry((log10_type, is_alpha_next))
                .or_default()
                .insert(
                    count,
                    ParsedPattern {
                        pattern: parsed,
                        number_of_0s,
                    },
                )
                .map_or_else(
                    || Ok(()),
                    |_| {
                        Err(DataError::custom("pattern").with_display_context(&format!(
                            "Plural case {count:?} is duplicated for type 10^{log10_type}"
                        )))
                    },
                )?;
        }

        let mut standard_patterns: BTreeMap<
            u8,
            (u8, PluralElements<Box<DoublePlaceholderPattern>>),
        > = BTreeMap::new();
        let mut alpha_next_to_patterns: BTreeMap<
            u8,
            (u8, PluralElements<Box<DoublePlaceholderPattern>>),
        > = BTreeMap::new();
        // Compute the exponents based on the numbers of 0s in the placeholders
        // and the type values: the exponent is 3 for type=1000, "0K", as well
        // as for type=10000, "00K", etc.
        // Remove duplicates of the count=other case in the same iteration.
        for ((log10_type, pattern_kind), mut parsed_plural_map) in parsed_patterns {
            let Some(other) = parsed_plural_map.remove(&"other") else {
                log::warn!(
                    "Missing other case for type 10^{log10_type} {} {parsed_plural_map:?}",
                    req.id.locale.language,
                );
                continue;
            };
            let parsed_plural_elements = PluralElements::new(other)
                .with_explicit_one_value(parsed_plural_map.remove(&"1"))
                .with_zero_value(parsed_plural_map.remove(&"zero"))
                .with_one_value(parsed_plural_map.remove(&"one"))
                .with_two_value(parsed_plural_map.remove(&"two"))
                .with_few_value(parsed_plural_map.remove(&"few"))
                .with_many_value(parsed_plural_map.remove(&"many"));

            let other_number_of_0s = parsed_plural_elements
                .other()
                .number_of_0s
                .unwrap_or_default();

            parsed_plural_elements
                .try_for_each(|pattern| {
                    if let Some(number_of_0s) = pattern.number_of_0s {
                        if number_of_0s != other_number_of_0s {
                            return Err(format!(
                                "Inconsistent placeholders within type 10^{}: {} 0s vs {} 0s",
                                log10_type, other_number_of_0s, number_of_0s,
                            ));
                        }
                    }
                    Ok(())
                })
                .unwrap();

            let exponent = log10_type - other_number_of_0s + 1;

            match pattern_kind {
                CurrencyPatternKind::Standard => &mut standard_patterns,
                CurrencyPatternKind::AlphaNextToNumber => &mut alpha_next_to_patterns,
            }
            .insert(
                log10_type,
                (
                    exponent,
                    parsed_plural_elements.map(|pattern| pattern.pattern),
                ),
            );
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ShortCurrencyCompact {
                standard: CompactPatterns::new(standard_patterns, None)
                    .map_err(|e| DataError::custom("pattern").with_display_context(&e))?,
                alpha_next_to_number: CompactPatterns::new(alpha_next_to_patterns, None)
                    .map_err(|e| DataError::custom("pattern").with_display_context(&e))?,
            }),
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
    use icu::plurals::provider::FourBitMetadata;

    let provider = SourceDataProvider::new_testing();
    let en: DataResponse<ShortCurrencyCompactV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap();

    let en_patterns = &en.payload.get();

    assert_eq!(
        en_patterns
            .standard
            .0
            .iter()
            .find(|t| t.sized == 3)
            .unwrap()
            .variable
            .decode(),
        PluralElements::new((
            FourBitMetadata::try_from_byte(0).unwrap(),
            &*DoublePlaceholderPattern::try_from_str("{1}{0}K", Default::default()).unwrap()
        ))
    );
    assert_eq!(
        en_patterns
            .alpha_next_to_number
            .0
            .iter()
            .find(|t| t.sized == 3)
            .unwrap()
            .variable
            .decode(),
        PluralElements::new((
            FourBitMetadata::try_from_byte(0).unwrap(),
            &*DoublePlaceholderPattern::try_from_str("{1} {0}K", Default::default()).unwrap()
        ))
    );

    let ja: DataResponse<ShortCurrencyCompactV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ja").into()),
            ..Default::default()
        })
        .unwrap();

    let ja_patterns = &ja.payload.get();

    assert_eq!(
        ja_patterns
            .standard
            .0
            .iter()
            .find(|t| t.sized == 4)
            .unwrap()
            .variable
            .decode(),
        PluralElements::new((
            FourBitMetadata::try_from_byte(0).unwrap(),
            &*DoublePlaceholderPattern::try_from_str("{1}{0}万", Default::default()).unwrap()
        ))
    );
    assert_eq!(
        ja_patterns
            .alpha_next_to_number
            .0
            .iter()
            .find(|t| t.sized == 4)
            .unwrap()
            .variable
            .decode(),
        PluralElements::new((
            FourBitMetadata::try_from_byte(0).unwrap(),
            &*DoublePlaceholderPattern::try_from_str("{1} {0}万", Default::default()).unwrap()
        ))
    );
}
