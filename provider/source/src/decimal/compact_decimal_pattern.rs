// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde::numbers::DecimalFormat;
use icu::experimental::compactdecimal::provider::CompactDecimalPatternData;
use icu::plurals::provider::{FourBitMetadata, PluralElementsPackedULE};
use icu::plurals::PluralElements;
use icu_pattern::{PatternItemCow, SinglePlaceholderKey, SinglePlaceholderPattern};
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use zerovec::ule::encode_varule_to_box;
use zerovec::ule::vartuple::{VarTuple, VarTupleULE};
use zerovec::vecs::VarZeroVecOwned;

/// A [`ParsedPattern`] represents a compact decimal pattern, which consists of
/// literal text with an optional placeholder.  The literal text is unescaped,
/// and the information about the number of 0s in the placeholder is stored
/// separately.
#[derive(PartialEq, Clone)]
struct ParsedPattern {
    pattern: Box<SinglePlaceholderPattern>,
    number_of_0s: Option<u8>,
}

/// Parses a compact decimal pattern string, performing any validation that can
/// be done without the context of the associated type and count.
fn parse(pattern: &str) -> Result<ParsedPattern, Cow<'static, str>> {
    let cldr_overrides: HashMap<String, String> = [
        // Unescaped - in yrl (Nheengatu).
        ("0 millón-ita", "0 millón'-'ita"),
        ("0 billón-ita", "0 billón'-'ita"),
        ("0 tirillón-ita", "0 tirillón'-'ita"),
        ("0 miliãu-ita", "0 miliãu'-'ita"),
        ("0 biliãu-ita", "0 biliãu'-'ita"),
        ("0 tiriliãu-ita", "0 tiriliãu'-'ita"),
        // All compact decimal patterns for sw (Swahili) are split by sign;
        // the sign ends up where it would be as part of the significand, so
        // this special handling is unneeded.  Depending on the region subtag,
        // the space may be breaking or nonbreaking.
        ("elfu 0;elfu -0", "elfu 0"),
        ("milioni 0;milioni -0", "milioni 0"),
        ("bilioni 0;bilioni -0", "bilioni 0"),
        ("trilioni 0;trilioni -0", "trilioni 0"),
        ("elfu\u{A0}0;elfu\u{A0}-0", "elfu\u{A0}0"),
        ("milioni\u{A0}0;milioni\u{A0}-0", "milioni\u{A0}0"),
        ("bilioni\u{A0}0;bilioni\u{A0}-0", "bilioni\u{A0}0"),
        ("trilioni\u{A0}0;trilioni\u{A0}-0", "trilioni\u{A0}0"),
        ("0M;-0M", "0M"),
        ("0B;-0B", "0B"),
        ("0T;-0T", "0B"),
        // Unescaped E in hu (Hungarian).
        ("0\u{A0}E", "0\u{A0}'E'"),
    ]
    .iter()
    .flat_map(|(key, value)| {
        [
            (key.to_string(), value.to_string()),
            (key.replace('0', "00"), value.replace('0', "00")),
            (key.replace('0', "000"), value.replace('0', "000")),
        ]
    })
    .collect();
    let pattern = cldr_overrides
        .get(pattern)
        .map(|s| s.as_str())
        .unwrap_or(pattern);

    let mut number_of_0s = None;
    let mut parts = Vec::new();
    // CLDR patterns use quoting for escaping, thus '.' for a literal FULL
    // STOP, as opposed to . for the decimal separator.  A doubled
    // APOSTROPHE ('') represents a single one.
    // See https://www.unicode.org/reports/tr35/tr35-numbers.html#Special_Pattern_Characters.
    // We process the pattern in chunks delimited by ', which are
    // alternatingly unescaped and escaped.
    for (i, chunk) in pattern.split('\'').enumerate() {
        let escaped = i % 2 == 1;
        if escaped {
            if chunk.is_empty() {
                // '' means '.
                parts.push(PatternItemCow::Literal(Cow::Borrowed("\'")));
            } else {
                // Anything else wrapped in apostrophes is literal text.
                parts.push(PatternItemCow::Literal(Cow::Borrowed(chunk)));
            }
        } else {
            // We are in unquoted text, so we need to check for the
            // symbols defined in https://www.unicode.org/reports/tr35/tr35-numbers.html#Number_Pattern_Character_Definitions.
            if chunk
                .chars()
                .any(|c| ('1'..'9').contains(&c) || "@#.-,E+%‰,¤*'".contains(c))
            {
                return Err(
                    format!("Unsupported symbol in compact decimal pattern {pattern}").into(),
                );
            }
            // Given the chunk "me0000w", the prefix is "me", and
            // additional_0s_and_suffix is "000w"; given the chunk
            // "me0w", these are "me" and "w" respectively.
            if let Some((prefix, additional_0s_and_suffix)) = chunk.split_once('0') {
                if number_of_0s.is_some() {
                    return Err(format!(
                        "Multiple placeholders in compact decimal pattern {pattern})"
                    )
                    .into());
                }
                // The prefix goes into the literal text, and the position
                // of the placeholder is then at the end of the accumulated
                // text, at variable.len().
                parts.push(PatternItemCow::Literal(Cow::Borrowed(prefix)));
                if let Some((middle_0s, suffix)) = additional_0s_and_suffix.rsplit_once('0') {
                    // More than one 0; in the "me0000w" example, middle_0s
                    // is "00", suffix is "w".
                    if !middle_0s.chars().all(|c| c == '0') {
                        return Err(format!(
                            "Multiple placeholders in compact decimal pattern {pattern}"
                        )
                        .into());
                    }
                    number_of_0s = Some(
                        u8::try_from(middle_0s.len() + 2)
                            .map_err(|_| format!("Too many 0s in pattern {pattern}"))?,
                    );
                    parts.push(PatternItemCow::Placeholder(SinglePlaceholderKey::Singleton));
                    parts.push(PatternItemCow::Literal(Cow::Borrowed(suffix)));
                } else {
                    // Only one 0, we are in the "me0w" case.
                    number_of_0s = Some(1);
                    parts.push(PatternItemCow::Placeholder(SinglePlaceholderKey::Singleton));
                    parts.push(PatternItemCow::Literal(Cow::Borrowed(
                        additional_0s_and_suffix,
                    )));
                }
            } else {
                // No symbols, all literal text.
                parts.push(PatternItemCow::Literal(Cow::Borrowed(chunk)));
            }
        }
    }
    Ok(ParsedPattern {
        pattern: SinglePlaceholderPattern::try_from_items(parts.into_iter()).unwrap(),
        number_of_0s,
    })
}

impl TryFrom<&DecimalFormat> for CompactDecimalPatternData<'static> {
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

            let count = &*pattern.count;

            parsed_patterns
                .entry(log10_type)
                .or_default()
                .insert(count, parse(&pattern.pattern)?)
                .map_or_else(
                    // TODO(egg): This should be try_insert.
                    || Ok(()),
                    |_| {
                        Err(format!(
                            "Plural case {count:?} is duplicated for type 10^{log10_type}"
                        ))
                    },
                )?;
        }

        let mut patterns: BTreeMap<u8, PluralElements<(u8, Box<SinglePlaceholderPattern>)>> =
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

            if log10_type < other_number_of_0s {
                return Err(format!(
                    "Too many 0s in type 10^{}, ({}, implying nonpositive exponent c={})",
                    log10_type,
                    other_number_of_0s,
                    log10_type as i8 - other_number_of_0s as i8 + 1
                )
                .into());
            }
            let exponent = log10_type - other_number_of_0s + 1;

            patterns.insert(
                log10_type,
                parsed_plural_elements.map(|pattern| (exponent, pattern.pattern)),
            );
        }

        if !patterns
            .values()
            .tuple_windows()
            .all(|(low, high)| low.other().0 <= high.other().0)
        {
            Err(format!(
                "Compact decimal exponents should be nondecreasing: {:?}",
                patterns
                    .values()
                    .map(|plural_map| plural_map.other().0)
                    .collect::<Vec<_>>(),
            ))?;
        }

        // Deduplicate sequences of types that have the same plural map, keeping the lowest type.
        let deduplicated_patterns = patterns
            .into_iter()
            // Skip leading 0 patterns
            .skip_while(|(_, pattern)| {
                pattern.as_ref().map(|(_, p)| p.as_ref())
                    == PluralElements::new(SinglePlaceholderPattern::PASS_THROUGH)
            })
            .coalesce(|low, high| {
                // The high pattern can never be exactly one of the low pattern, so we can ignore that value
                if low.1.as_ref().with_explicit_one_value(None) == high.1.as_ref() {
                    Ok(low)
                } else {
                    Err((low, high))
                }
            });

        Ok(CompactDecimalPatternData {
            patterns:
                VarZeroVecOwned::try_from_elements(
                    &deduplicated_patterns
                        .map(|(log10_type, plural_map)| {
                            encode_varule_to_box(&VarTuple {
                                sized: log10_type,
                                variable: plural_map.map(|(exponent, pattern)| {
                                    // Store the exponent as a difference from the log10_type, i.e. the number of zeros
                                    // in the pattern, minus 1. No pattern should have more than 16 zeros.
                                    (
                                        FourBitMetadata::try_from_byte(
                                            log10_type.checked_sub(exponent).unwrap(),
                                        )
                                        .unwrap(),
                                        pattern,
                                    )
                                }),
                            })
                        })
                        .collect::<Vec<
                            Box<VarTupleULE<u8, PluralElementsPackedULE<SinglePlaceholderPattern>>>,
                        >>(),
                )
                .unwrap()
                .into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::type_complexity)]
    fn convert_for_test(
        data: CompactDecimalPatternData,
    ) -> Box<[(u8, PluralElements<(u8, Box<SinglePlaceholderPattern>)>)]> {
        data.patterns
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
            CompactDecimalPatternData::try_from(
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
            CompactDecimalPatternData::try_from(
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
            CompactDecimalPatternData::try_from(
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
    fn test_pattern_syntax_errors() {
        assert_eq!(
            parse("M.").err().unwrap(),
            "Unsupported symbol in compact decimal pattern M."
        );
        assert_eq!(
            parse("M'.'").unwrap().pattern,
            SinglePlaceholderPattern::try_from_str("M.", Default::default()).unwrap()
        );
        assert_eq!(
            parse("0 0").err().unwrap(),
            "Multiple placeholders in compact decimal pattern 0 0"
        );
        assert_eq!(
            parse("0 '0'").unwrap().pattern,
            SinglePlaceholderPattern::try_from_str("{0} 0", Default::default()).unwrap()
        );
        let zeros = str::repeat("0", 256);
        assert_eq!(
            parse(&zeros).err().unwrap(),
            String::from("Too many 0s in pattern ") + &zeros
        );
        assert_eq!(
            parse(&zeros[..255]).unwrap().pattern,
            SinglePlaceholderPattern::try_from_str("{0}", Default::default()).unwrap()
        );
    }

    #[test]
    fn test_inter_pattern_errors() {
        assert_eq!(
            CompactDecimalPatternData::try_from(
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
            CompactDecimalPatternData::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1-count-one": "0" }"#).unwrap()
            )
            .err()
            .unwrap(),
            "Missing other case for type 10^0"
        );

        assert_eq!(
            CompactDecimalPatternData::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "k" }"#).unwrap()
            )
            .err()
            .unwrap(),
            "Missing placeholder in other case of type 10^3"
        );

        // Given this data, it is ambiguous whether the 10 000 should be formatted as 10 thousand or 1 myriad.
        assert_eq!(
            CompactDecimalPatternData::try_from(
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
            CompactDecimalPatternData::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "00000 tenths" }"#)
                    .unwrap()
            )
            .err()
            .unwrap(),
            "Too many 0s in type 10^3, (5, implying nonpositive exponent c=-1)"
        );

        let long_pattern = format!("thous{}nds (0)", str::repeat("a", 244));
        let overlong_pattern = format!("thous{}nds (0)", str::repeat("a", 245));

        CompactDecimalPatternData::try_from(
            &serde_json::from_str::<DecimalFormat>(
                format!(r#"{{ "1000-count-other": "{overlong_pattern}" }}"#).as_str(),
            )
            .unwrap(),
        )
        .unwrap();

        CompactDecimalPatternData::try_from(
            &serde_json::from_str::<DecimalFormat>(
                format!(r#"{{ "1000-count-other": "{long_pattern}" }}"#).as_str(),
            )
            .unwrap(),
        )
        .unwrap()
        .patterns
        .iter()
        .find_map(|v| (v.sized == 3).then_some(v.variable.get_default()))
        .unwrap();
    }
}
