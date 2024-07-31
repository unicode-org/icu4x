// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::plurals::rules::runtime::ast::Rule;
use icu::plurals::{provider::*, PluralCategory};
use icu_provider::prelude::*;
use zerovec::ZeroMap;

impl SourceDataProvider {
    fn get_rules_for(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<&cldr_serde::plurals::Rules, DataError> {
        if marker == CardinalV1Marker::INFO {
            self.cldr()?
                .core()
                .read_and_parse::<cldr_serde::plurals::Resource>("supplemental/plurals.json")?
                .supplemental
                .plurals_type_cardinal
                .as_ref()
        } else if marker == OrdinalV1Marker::INFO {
            self.cldr()?
                .core()
                .read_and_parse::<cldr_serde::plurals::Resource>("supplemental/ordinals.json")?
                .supplemental
                .plurals_type_ordinal
                .as_ref()
        } else {
            None
        }
        .ok_or(DataError::custom("Unknown marker for PluralRulesV1"))
    }

    fn get_plural_ranges(&self) -> Result<&cldr_serde::plural_ranges::PluralRanges, DataError> {
        Ok(&self
            .cldr()?
            .core()
            .read_and_parse::<cldr_serde::plural_ranges::Resource>(
                "supplemental/pluralRanges.json",
            )?
            .supplemental
            .plurals)
    }
}

macro_rules! implement {
    ($marker:ident) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(PluralRulesV1::from(
                        self.get_rules_for(<$marker>::INFO)?
                            .0
                            .get(&icu::locale::LanguageIdentifier::from((
                                req.id.locale.language,
                                req.id.locale.script,
                                req.id.locale.region,
                            )))
                            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?,
                    )),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(self
                    .get_rules_for(<$marker>::INFO)?
                    .0
                    .keys()
                    .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l)))
                    .collect())
            }
        }
    };
}

implement!(CardinalV1Marker);
implement!(OrdinalV1Marker);

impl From<&cldr_serde::plurals::LocalePluralRules> for PluralRulesV1<'static> {
    fn from(other: &cldr_serde::plurals::LocalePluralRules) -> Self {
        /// Removes samples from plural rule strings. Takes an owned [`String`] reference and
        /// returns a new [`String`] in a [`Cow::Owned`].
        fn convert(s: &str) -> Rule<'static> {
            s.parse().expect("Rule parsing failed.")
        }
        Self {
            zero: other.zero.as_deref().map(convert),
            one: other.one.as_deref().map(convert),
            two: other.two.as_deref().map(convert),
            few: other.few.as_deref().map(convert),
            many: other.many.as_deref().map(convert),
        }
    }
}

impl DataProvider<PluralRangesV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<PluralRangesV1Marker>, DataError> {
        self.check_req::<PluralRangesV1Marker>(req)?;
        if req.id.locale.is_und() {
            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(PluralRangesV1 {
                    ranges: ZeroMap::default(),
                }),
            })
        } else {
            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(PluralRangesV1::from(
                    self.get_plural_ranges()?
                        .0
                        .get(&icu::locale::LanguageIdentifier::from((
                            req.id.locale.language,
                            req.id.locale.script,
                            req.id.locale.region,
                        )))
                        .ok_or(DataErrorKind::IdentifierNotFound.into_error())?,
                )),
            })
        }
    }
}

impl IterableDataProviderCached<PluralRangesV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .get_plural_ranges()?
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l)))
            .chain([Default::default()]) // `und` is not included in the locales of plural ranges.
            .collect())
    }
}

impl From<&cldr_serde::plural_ranges::LocalePluralRanges> for PluralRangesV1<'static> {
    fn from(other: &cldr_serde::plural_ranges::LocalePluralRanges) -> Self {
        fn convert(s: &str) -> RawPluralCategory {
            PluralCategory::get_for_cldr_string(s)
                .expect("category parsing failed.")
                .into()
        }
        let mut map: BTreeMap<(RawPluralCategory, RawPluralCategory), RawPluralCategory> =
            BTreeMap::new();
        for (range, result) in &other.0 {
            let start = convert(&range.start);
            let end = convert(&range.end);
            let result = convert(result);

            // <https://unicode.org/reports/tr35/tr35-numbers.html#Plural_Ranges>
            // "If there is no value for a <start,end> pair, the default result is end."
            //
            // We can use that to save a lot of memory by not inserting the ranges that
            // have end == result.
            if end != result {
                map.insert((start, end), result);
            }
        }

        PluralRangesV1 {
            ranges: map
                .into_iter()
                .map(|((start, end), result)| {
                    (UnvalidatedPluralRange::from_range(start, end), result)
                })
                .collect(),
        }
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    // Spot-check locale 'cs' since it has some interesting entries
    let cs_rules: DataResponse<CardinalV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("cs").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    assert_eq!(None, cs_rules.payload.get().zero);
    assert_eq!(
        Some("i = 1 and v = 0".parse().expect("Failed to parse rule")),
        cs_rules.payload.get().one
    );
    assert_eq!(None, cs_rules.payload.get().two);
    assert_eq!(
        Some("i = 2..4 and v = 0".parse().expect("Failed to parse rule")),
        cs_rules.payload.get().few
    );
    assert_eq!(
        Some("v != 0".parse().expect("Failed to parse rule")),
        cs_rules.payload.get().many
    );
}

#[test]
fn test_ranges() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    // locale 'sl' seems to have a lot of interesting cases.
    let plural_ranges: DataResponse<PluralRangesV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("sl").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    assert_eq!(
        plural_ranges
            .payload
            .get()
            .ranges
            .get_copied(&UnvalidatedPluralRange::from_range(
                RawPluralCategory::Few,
                RawPluralCategory::One
            )),
        Some(RawPluralCategory::Few)
    );
    assert_eq!(
        plural_ranges
            .payload
            .get()
            .ranges
            .get_copied(&UnvalidatedPluralRange::from_range(
                RawPluralCategory::Other,
                RawPluralCategory::One
            )),
        Some(RawPluralCategory::Few)
    );
    assert!(plural_ranges
        .payload
        .get()
        .ranges
        .get_copied(&UnvalidatedPluralRange::from_range(
            RawPluralCategory::Zero,
            RawPluralCategory::One
        ))
        .is_none());
    assert!(plural_ranges
        .payload
        .get()
        .ranges
        .get_copied(&UnvalidatedPluralRange::from_range(
            RawPluralCategory::One,
            RawPluralCategory::Zero
        ))
        .is_none());

    // tests that the space optimization succeeds
    assert!(plural_ranges
        .payload
        .get()
        .ranges
        .get_copied(&UnvalidatedPluralRange::from_range(
            RawPluralCategory::One,
            RawPluralCategory::Other
        ))
        .is_none());
    assert!(plural_ranges
        .payload
        .get()
        .ranges
        .get_copied(&UnvalidatedPluralRange::from_range(
            RawPluralCategory::Few,
            RawPluralCategory::Two
        ))
        .is_none());
}
