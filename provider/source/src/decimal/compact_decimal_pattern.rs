// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde::numbers::DecimalFormat;
use icu::experimental::compactdecimal::provider::CompactPatterns;
use icu::plurals::PluralElements;
use icu_pattern::{QuoteMode, SinglePlaceholder, SinglePlaceholderPattern};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// A [`ParsedPattern`] represents a compact decimal pattern, which consists of
/// literal text with an optional placeholder.  The literal text is unescaped,
/// and the information about the number of 0s in the placeholder is stored
/// separately.
#[derive(PartialEq, Clone)]
struct ParsedPattern {
    pattern: Box<SinglePlaceholderPattern>,
    number_of_0s: Option<u8>,
}

impl TryFrom<&DecimalFormat> for CompactPatterns<'static, SinglePlaceholder> {
    type Error = Cow<'static, str>;

    fn try_from(other: &DecimalFormat) -> Result<Self, Self::Error> {
        let mut parsed_patterns: BTreeMap<u8, BTreeMap<&str, ParsedPattern>> = BTreeMap::new();
        // First ingest the CLDR mapping.
        for pattern in other.patterns.iter() {
            let mut type_bytes = pattern.magnitude.bytes();

            if !(type_bytes.next() == Some(b'1') && type_bytes.all(|b| b == b'0')) {
                return Err(format!("Ill-formed type {}", pattern.magnitude).into());
            }
            let log10_type = u8::try_from(pattern.magnitude.len() - 1)
                .map_err(|_| format!("Too many digits in type {}", pattern.magnitude))?;

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

            if log10_type < number_of_0s.unwrap_or_default() {
                return Err(format!(
                    "Too many 0s in type 10^{}, ({}, implying nonpositive exponent c={})",
                    log10_type,
                    number_of_0s.unwrap_or_default(),
                    log10_type as i8 - number_of_0s.unwrap_or_default() as i8 + 1
                )
                .into());
            }

            let pattern_str = if let Some(number_of_zeros) = number_of_0s {
                pattern_str.replace(
                    &core::iter::repeat_n('0', number_of_zeros as usize).collect::<String>(),
                    "{0}",
                )
            } else {
                pattern_str.into()
            };

            let parsed = SinglePlaceholderPattern::try_from_str(
                &pattern_str,
                QuoteMode::QuotingSupported.into(),
            )
            .map_err(|e| e.to_string())?;

            parsed_patterns
                .entry(log10_type)
                .or_default()
                .insert(
                    &pattern.count,
                    ParsedPattern {
                        pattern: parsed,
                        number_of_0s,
                    },
                )
                .map_or_else(
                    // TODO(egg): This should be try_insert.
                    || Ok(()),
                    |_| {
                        Err(format!(
                            "Plural case {:?} is duplicated for type 10^{log10_type}",
                            pattern.count
                        ))
                    },
                )?;
        }

        let mut patterns: BTreeMap<u8, (u8, PluralElements<Box<SinglePlaceholderPattern>>)> =
            BTreeMap::new();
        // Compute the exponents based on the numbers of 0s in the placeholders
        // and the type values: the exponent is 3 for type=1000, "0K", as well
        // as for type=10000, "00K", etc.
        // Remove duplicates of the count=other case in the same iteration.
        for (log10_type, mut parsed_plural_map) in parsed_patterns {
            let parsed_plural_elements = PluralElements::new(
                parsed_plural_map
                    .remove(&"other")
                    .ok_or_else(|| format!("Missing other case for type 10^{log10_type}"))?,
            )
            .with_explicit_one_value(parsed_plural_map.remove(&"1"))
            .with_zero_value(parsed_plural_map.remove(&"zero"))
            .with_one_value(parsed_plural_map.remove(&"one"))
            .with_two_value(parsed_plural_map.remove(&"two"))
            .with_few_value(parsed_plural_map.remove(&"few"))
            .with_many_value(parsed_plural_map.remove(&"many"));

            let other_number_of_0s =
                parsed_plural_elements.other().number_of_0s.ok_or_else(|| {
                    format!("Missing placeholder in other case of type 10^{log10_type}")
                })?;

            parsed_plural_elements.try_for_each(|pattern| {
                if let Some(number_of_0s) = pattern.number_of_0s {
                    if number_of_0s != other_number_of_0s {
                        return Err(format!(
                            "Inconsistent placeholders within type 10^{}: {} 0s vs {} 0s",
                            log10_type, other_number_of_0s, number_of_0s,
                        ));
                    }
                }
                Ok(())
            })?;

            let exponent = log10_type - other_number_of_0s + 1;

            patterns.insert(
                log10_type,
                (
                    exponent,
                    parsed_plural_elements.map(|pattern| pattern.pattern),
                ),
            );
        }

        Ok(Self::new(
            patterns,
            Some(&PluralElements::new(SinglePlaceholderPattern::PASS_THROUGH)),
        )?)
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
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(
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
                .unwrap(),
            )
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
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(
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
                .unwrap(),
            )
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
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(
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
                .unwrap(),
            )
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
        assert_eq!(
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(
                    r#"{ "1000-count-other": "0k", "1000-count-other": "0K" }"#,
                )
                .unwrap(),
            )
            .err()
            .unwrap(),
            "Plural case \"other\" is duplicated for type 10^3"
        );

        assert_eq!(
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-one": "0" }"#).unwrap()
            )
            .err()
            .unwrap(),
            "Missing other case for type 10^3"
        );

        assert_eq!(
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "k" }"#).unwrap()
            )
            .err()
            .unwrap(),
            "Missing placeholder in other case of type 10^3"
        );

        // Given this data, it is ambiguous whether the 10 000 should be formatted as 10 thousand or 1 myriad.
        assert_eq!(
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(
                    r#"
                        {
                            "10000-count-other": "00 thousand",
                            "10000-count-one": "0 myriad"
                        }
                    "#,
                )
                .unwrap(),
            )
            .err()
            .unwrap(),
            "Inconsistent placeholders within type 10^4: 2 0s vs 1 0s"
        );

        assert_eq!(
            CompactPatterns::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "00000 tenths" }"#)
                    .unwrap()
            )
            .err()
            .unwrap(),
            "Too many 0s in type 10^3, (5, implying nonpositive exponent c=-1)"
        );

        let long_pattern = format!("thous{}nds (0)", str::repeat("a", 244));
        let overlong_pattern = format!("thous{}nds (0)", str::repeat("a", 245));

        CompactPatterns::try_from(
            &serde_json::from_str::<DecimalFormat>(
                format!(r#"{{ "1000-count-other": "{overlong_pattern}" }}"#).as_str(),
            )
            .unwrap(),
        )
        .unwrap();

        CompactPatterns::try_from(
            &serde_json::from_str::<DecimalFormat>(
                format!(r#"{{ "1000-count-other": "{long_pattern}" }}"#).as_str(),
            )
            .unwrap(),
        )
        .unwrap()
        .0
        .iter()
        .find_map(|v| (v.sized == 3).then_some(v.variable.get_default()))
        .unwrap();
    }
}
