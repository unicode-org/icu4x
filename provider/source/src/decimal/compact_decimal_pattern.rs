// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde::numbers::{DecimalFormat, NumberPatternItem};
use icu::decimal::provider::CompactPatterns;
use icu::plurals::PluralElements;
use icu_pattern::{
    PatternItemCow, SinglePlaceholder, SinglePlaceholderKey, SinglePlaceholderPattern,
};
use icu_provider::{DataError, DataLocale};
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::BTreeMap;

impl DecimalFormat {
    pub fn as_compact_patterns(
        &self,
        locale: DataLocale,
    ) -> Result<CompactPatterns<'static, SinglePlaceholder>, DataError> {
        let mut patterns: BTreeMap<u8, (u8, PluralElements<Box<SinglePlaceholderPattern>>)> =
            BTreeMap::new();

        // First ingest the CLDR mapping.
        for (&log10_type, pattern) in self.standard.iter() {
            let p = pattern.as_ref().try_map(|pattern| {
                if pattern.negative.is_some() {
                    log::warn!("Unexpected negative pattern for {locale}: {}", pattern);
                }

                let number_of_0s = Some(
                    pattern
                        .positive
                        .iter()
                        .filter(|&i| *i == NumberPatternItem::MandatoryDigit)
                        .count() as u8,
                )
                .filter(|&n| n != 0);

                if log10_type < number_of_0s.unwrap_or_default() {
                    return Err(DataError::custom(
                        "Too many 0s in type 10^{}, ({}, implying nonpositive exponent c={})",
                    )
                    .with_debug_context(&log10_type)
                    .with_debug_context(&number_of_0s)
                    .with_debug_context(
                        &(log10_type as i8 - number_of_0s.unwrap_or_default() as i8 + 1),
                    ));
                }

                let parsed = SinglePlaceholderPattern::try_from_items(
                    pattern
                        .positive
                        .iter()
                        .map(|i| match i {
                            NumberPatternItem::MandatoryDigit => {
                                PatternItemCow::Placeholder(SinglePlaceholderKey::Singleton)
                            }
                            NumberPatternItem::Literal(s) => PatternItemCow::Literal(s.into()),
                            // everything else is interpreted as a literal
                            i => PatternItemCow::Literal(Cow::Borrowed(i.as_str())),
                        })
                        .dedup(),
                )
                .map_err(|e| DataError::custom("pattern").with_display_context(&e))?;

                Ok((number_of_0s, parsed))
            })?;

            let other_number_of_0s = p.other().0.ok_or_else(|| {
                DataError::custom("Missing placeholder in other case")
                    .with_display_context(&log10_type)
            })?;

            p.try_for_each(|pattern| {
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

            patterns.insert(log10_type, (exponent, p.map(|pattern| pattern.1)));
        }

        CompactPatterns::new(
            patterns,
            Some(&PluralElements::new(SinglePlaceholderPattern::PASS_THROUGH)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::type_complexity)]
    fn convert_for_test(
        data: CompactPatterns<SinglePlaceholder>,
    ) -> Box<[(u8, PluralElements<(u8, Box<SinglePlaceholderPattern>)>)]> {
        data.0
            .iter()
            .map(|t| {
                (
                    t.sized,
                    t.variable.decode().map(|(a, b)| (a.get(), b.to_owned())),
                )
            })
            .collect()
    }

    #[test]
    fn test_french_compressibility() {
        // French compact-long thousands as of CLDR 42.
        // The type=1000, count=one case is incorrect (it is inconsistent with the
        // plural rules), but it is interesting because it forces a distinction
        // between 1000 and 10000 to be made in the ICU4X data.
        let cldr_42_long_french = convert_for_test(
            serde_json::from_str::<DecimalFormat>(
                r#"
                {
                    "1000-count-1": "mille",
                    "1000-count-one": "0 millier",
                    "1000-count-other": "0 mille",
                    "10000-count-one": "00 mille",
                    "10000-count-other": "00 mille",
                    "100000-count-one": "000 mille",
                    "100000-count-other": "000 mille"
                }
            "#,
            )
            .unwrap()
            .as_compact_patterns(Default::default())
            .unwrap(),
        );
        assert_eq!(
            cldr_42_long_french.as_ref(),
            [
                (
                    3,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} mille", Default::default())
                            .unwrap()
                    ))
                    .with_one_value(Some((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} millier", Default::default())
                            .unwrap()
                    )))
                    .with_explicit_one_value(Some((
                        0,
                        SinglePlaceholderPattern::try_from_str("mille", Default::default())
                            .unwrap()
                    )))
                ),
                (
                    4,
                    PluralElements::new((
                        1,
                        SinglePlaceholderPattern::try_from_str("{0} mille", Default::default())
                            .unwrap()
                    ))
                ),
            ]
        );
        // French compact-long thousands, with the anomalous « millier » removed.
        // This allows 10000 and 1000 to be collapsed.
        let compressible_long_french = convert_for_test(
            serde_json::from_str::<DecimalFormat>(
                r#"
                {
                    "1000-count-1": "mille",
                    "1000-count-one": "0 mille",
                    "1000-count-other": "0 mille",
                    "10000-count-one": "00 mille",
                    "10000-count-other": "00 mille",
                    "100000-count-one": "000 mille",
                    "100000-count-other": "000 mille"
                }
            "#,
            )
            .unwrap()
            .as_compact_patterns(Default::default())
            .unwrap(),
        );
        assert_eq!(
            compressible_long_french.as_ref(),
            [(
                3,
                PluralElements::new((
                    0,
                    SinglePlaceholderPattern::try_from_str("{0} mille", Default::default())
                        .unwrap()
                ))
                .with_explicit_one_value(Some((
                    0,
                    SinglePlaceholderPattern::try_from_str("mille", Default::default()).unwrap()
                )))
            ),]
        );
    }

    #[test]
    fn test_holes() {
        // Spanish compact-short data as of CLDR 42, up to 10¹¹.
        // Note that the abbreviation for 10⁹ is used only starting with 10¹⁰.
        let spanish = convert_for_test(
            serde_json::from_str::<DecimalFormat>(
                r#"
                {
                    "1000-count-one": "0 mil",
                    "1000-count-other": "0 mil",
                    "10000-count-one": "00 mil",
                    "10000-count-other": "00 mil",
                    "100000-count-one": "000 mil",
                    "100000-count-other": "000 mil",
                    "1000000-count-one": "0 M",
                    "1000000-count-other": "0 M",
                    "10000000-count-one": "00 M",
                    "10000000-count-other": "00 M",
                    "100000000-count-one": "000 M",
                    "100000000-count-other": "000 M",
                    "1000000000-count-one": "0000 M",
                    "1000000000-count-other": "0000 M",
                    "10000000000-count-one": "00 mil M",
                    "10000000000-count-other": "00 mil M",
                    "100000000000-count-one": "000 mil M",
                    "100000000000-count-other": "000 mil M"
                }
            "#,
            )
            .unwrap()
            .as_compact_patterns(Default::default())
            .unwrap(),
        );
        assert_eq!(
            spanish.as_ref(),
            [
                (
                    3,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} mil", Default::default())
                            .unwrap()
                    ))
                ),
                (
                    6,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} M", Default::default())
                            .unwrap()
                    ))
                ),
                (
                    10,
                    PluralElements::new((
                        1,
                        SinglePlaceholderPattern::try_from_str("{0} mil M", Default::default())
                            .unwrap()
                    ))
                ),
            ]
        );
    }

    #[test]
    fn test_inter_pattern_errors() {
        serde_json::from_str::<DecimalFormat>(
            r#"{ "1000-count-other": "0k", "1000-count-other": "0K" }"#,
        )
        .unwrap()
        .as_compact_patterns(Default::default())
        .unwrap_err();

        serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-one": "0" }"#)
            .unwrap()
            .as_compact_patterns(Default::default())
            .unwrap_err();

        serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "k" }"#)
            .unwrap()
            .as_compact_patterns(Default::default())
            .unwrap_err();

        // Given this data, it is ambiguous whether the 10 000 should be formatted as 10 thousand or 1 myriad.
        serde_json::from_str::<DecimalFormat>(
            r#"
                        {
                            "10000-count-other": "00 thousand",
                            "10000-count-one": "0 myriad"
                        }
                    "#,
        )
        .unwrap()
        .as_compact_patterns(Default::default())
        .unwrap_err();

        serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "00000 tenths" }"#)
            .unwrap()
            .as_compact_patterns(Default::default())
            .unwrap_err();

        let long_pattern = format!("thous{}nds (0)", str::repeat("a", 244));
        let overlong_pattern = format!("thous{}nds (0)", str::repeat("a", 245));

        serde_json::from_str::<DecimalFormat>(
            format!(r#"{{ "1000-count-other": "{overlong_pattern}" }}"#).as_str(),
        )
        .unwrap()
        .as_compact_patterns(Default::default())
        .unwrap();

        serde_json::from_str::<DecimalFormat>(
            format!(r#"{{ "1000-count-other": "{long_pattern}" }}"#).as_str(),
        )
        .unwrap()
        .as_compact_patterns(Default::default())
        .unwrap()
        .0
        .iter()
        .find_map(|v| (v.sized == 3).then_some(v.variable.get_default()))
        .unwrap();
    }
}
