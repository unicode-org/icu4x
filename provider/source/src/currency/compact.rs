// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::numbers::NumberPatternItem;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashSet;

use icu::decimal::provider::CompactPatterns;
use icu::experimental::dimension::provider::currency::compact::*;
use icu::plurals::PluralElements;
use icu_pattern::DoublePlaceholderKey;
use icu_pattern::DoublePlaceholderPattern;
use icu_pattern::PatternItemCow;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use itertools::Itertools;

impl DataProvider<ShortCurrencyCompactV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ShortCurrencyCompactV1>, DataError> {
        self.check_req::<ShortCurrencyCompactV1>(req)?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbering_system = if req.id.marker_attributes.is_empty() {
            numbers_resource
                .main
                .value
                .numbers
                .default_numbering_system
                .as_str()
        } else {
            req.id.marker_attributes.as_str()
        };

        let Some(compact_patterns) = numbers_resource
            .main
            .value
            .numbers
            .numsys_data
            .currency_patterns
            .get(numbering_system)
            .and_then(|patterns| patterns.compact_short.as_ref())
            .map(|short_compact| &short_compact.standard)
        else {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(ShortCurrencyCompactV1::INFO, req)
            );
        };

        let mut standard_patterns: BTreeMap<
            u8,
            (u8, PluralElements<Box<DoublePlaceholderPattern>>),
        > = BTreeMap::new();
        let mut alpha_next_to_patterns: BTreeMap<
            u8,
            (u8, PluralElements<Box<DoublePlaceholderPattern>>),
        > = BTreeMap::new();

        for (target, source) in [
            (&mut standard_patterns, &compact_patterns.standard),
            (
                &mut alpha_next_to_patterns,
                &compact_patterns.alpha_next_to_number,
            ),
        ] {
            for (&log10_type, pattern) in source {
                let pattern = pattern.as_ref().try_map(|pattern| {

                if pattern.negative.is_some() {
                    log::warn!(
                        "Unexpected negative pattern for {}: {}",
                        req.id.locale,
                        pattern
                    );
                }

                let number_of_0s = Some(
                    pattern.positive
                        .iter()
                        .filter(|&i| *i == NumberPatternItem::MandatoryDigit)
                        .count() as u8,
                )
                .filter(|&n| n != 0);

                let parsed = DoublePlaceholderPattern::try_from_items(
                    pattern.positive
                        .iter()
                        .map(|i| match i {
                            NumberPatternItem::MandatoryDigit => {
                                PatternItemCow::Placeholder(DoublePlaceholderKey::Place0)
                            }
                            NumberPatternItem::Currency => {
                                PatternItemCow::Placeholder(DoublePlaceholderKey::Place1)
                            }
                            i => PatternItemCow::Literal(Cow::Borrowed(i.as_str())),
                        })
                        .dedup(),
                )
                .map_err(|e| DataError::custom("pattern").with_display_context(&e))?;

                if log10_type < number_of_0s.unwrap_or_default() {
                    return Err(DataError::custom("pattern").with_display_context(&format!(
                        "Too many 0s in type 10^{}, ({}, implying nonpositive exponent c={}), pattern = {:?}",
                        log10_type,
                        number_of_0s.unwrap_or_default(),
                        log10_type as i8 - number_of_0s.unwrap_or_default() as i8 + 1,
                        pattern
                    )));
                }

                Ok((number_of_0s, parsed))
                })?;

                let other_number_of_0s = pattern.other().0.ok_or_else(|| {
                    DataError::custom("Missing placeholder in other case")
                        .with_display_context(&log10_type)
                })?;

                pattern.try_for_each(|pattern| {
                    if let Some(number_of_0s) = pattern.0 {
                        if number_of_0s != other_number_of_0s {
                            return Err(DataError::custom("Inconsistent placeholders within")
                                .with_debug_context(&log10_type)
                                .with_debug_context(&other_number_of_0s)
                                .with_debug_context(&number_of_0s));
                        }
                    }
                    Ok(())
                })?;

                let exponent = log10_type - other_number_of_0s + 1;

                target.insert(log10_type, (exponent, pattern.map(|pattern| pattern.1)));
            }
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
        let cldr = self.cldr()?;
        let mut r = self.iter_ids_for_numbers_with_locales()?;
        // TODO(#7493): This filtering might not be needed
        let mut err = None;
        r.retain(|id| {
            if err.is_some() {
                return true;
            }
            let numbers_resource = match cldr
                .numbers()
                .read_and_parse::<cldr_serde::numbers::Resource>(&id.locale, "numbers.json")
            {
                Ok(r) => r,
                Err(e) => {
                    err = Some(e);
                    return true;
                }
            };

            let numbering_system = if id.marker_attributes.is_empty() {
                numbers_resource
                    .main
                    .value
                    .numbers
                    .default_numbering_system
                    .as_str()
            } else {
                id.marker_attributes.as_str()
            };

            numbers_resource
                .main
                .value
                .numbers
                .numsys_data
                .currency_patterns
                .get(numbering_system)
                .and_then(|patterns| patterns.compact_short.as_ref())
                .is_some()
        });
        if let Some(e) = err {
            Err(e)
        } else {
            Ok(r)
        }
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
