// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::decimal::DecimalFormat;
use icu_compactdecimal::provider::CompactDecimalPatternDataV1;
use icu_compactdecimal::provider::*;
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use zerovec::ule::encode_varule_to_box;

#[derive(PartialEq, Clone)]
struct ParsedPlaceholder {
    pub index: usize,
    pub number_of_0s: i8,
}

#[derive(PartialEq, Clone)]
struct ParsedPattern {
    pub literal_text: Cow<'static, str>,
    pub placeholder: Option<ParsedPlaceholder>,
}

fn parse(pattern: &str) -> Result<Option<ParsedPattern>, Cow<'static, str>> {
    if pattern == "0" {
        return Ok(None);
    } else {
        let mut placeholder: Option<ParsedPlaceholder> = None;
        let mut literal_text = String::with_capacity(pattern.len());
        for (i, chunk) in pattern.split('\'').enumerate() {
            let escaped = i % 2 == 1;
            if escaped {
                if chunk.is_empty() {
                    // '' means '.
                    literal_text.push('\'');
                } else {
                    // Anything else wrapped in apostrophes is literal text.
                    literal_text.push_str(chunk);
                }
            } else {
                // We are in unquoted text, so we need to check for the
                // symbols defined in https://www.unicode.org/reports/tr35/tr35-numbers.html#Number_Pattern_Character_Definitions.
                if chunk
                    .chars()
                    .any(|c| ('1'..'9').contains(&c) || "@#.-,E+%‰,¤*'".contains(c))
                {
                    return Err(format!(
                        "Unsupported symbol in compact decimal pattern {}",
                        pattern
                    )
                    .into());
                }
                if let Some((prefix, additional_0s_and_suffix)) = chunk.split_once('0') {
                    if placeholder.is_some() {
                        return Err(format!(
                            "Multiple placeholders in compact decimal pattern {})",
                            pattern
                        )
                        .into());
                    }
                    literal_text.push_str(prefix);
                    if let Some((middle_0s, suffix)) = additional_0s_and_suffix.rsplit_once('0') {
                        if !middle_0s.chars().all(|c| c == '0') {
                            return Err(format!(
                                "Multiple placeholders in compact decimal pattern {}",
                                pattern
                            )
                            .into());
                        }
                        placeholder = Some(ParsedPlaceholder {
                            index: literal_text.len(),
                            number_of_0s: i8::try_from(middle_0s.len() + 2)
                                .map_err(|_| format!("Too many 0s in pattern {}", pattern))?,
                        });
                        literal_text.push_str(suffix);
                    } else {
                        placeholder = Some(ParsedPlaceholder {
                            index: literal_text.len(),
                            number_of_0s: 1,
                        });
                        literal_text.push_str(additional_0s_and_suffix);
                    }
                } else {
                    // No symbols, all literal text.
                    literal_text.push_str(chunk);
                }
            }
        }
        Ok(Some(ParsedPattern {
            literal_text: Cow::Owned(literal_text),
            placeholder: placeholder,
        }))
    }
}

#[cfg(feature = "experimental")]
impl TryFrom<&DecimalFormat> for CompactDecimalPatternDataV1<'static> {
    type Error = Cow<'static, str>;

    fn try_from(other: &DecimalFormat) -> Result<Self, Self::Error> {
        let mut parsed_patterns: BTreeMap<i8, BTreeMap<Count, Option<ParsedPattern>>> =
            BTreeMap::new();
        for pattern in other.patterns.iter() {
            let mut type_bytes = pattern.compact_decimal_type.bytes();

            if !(type_bytes.next() == Some(b'1') && type_bytes.all(|b| b == b'0')) {
                return Err(format!("Ill-formed type {}", pattern.compact_decimal_type).into());
            }
            let log10_type = i8::try_from(pattern.compact_decimal_type.len() - 1)
                .map_err(|_| format!("Too many digits in type {}", pattern.compact_decimal_type))?;
            let count = match &*pattern.compact_decimal_count {
                "zero" => Count::Zero,
                "one" => Count::One,
                "two" => Count::Two,
                "few" => Count::Few,
                "many" => Count::Many,
                "other" => Count::Other,
                "1" => Count::Explicit1,
                _ => {
                    return Err(format!(
                        "Invalid count {} in type {}",
                        pattern.compact_decimal_count, pattern.compact_decimal_type
                    )
                    .into())
                }
            };
            let plural_map = parsed_patterns.entry(log10_type).or_insert(BTreeMap::new());
            plural_map
                .insert(count, parse(&pattern.pattern)?)
                .map_or_else(
                    // TODO(egg): This should be try_insert.
                    || Ok(()),
                    |_| {
                        Err(format!(
                            "Plural case {:?} is duplicated for type 10^{}",
                            count, log10_type
                        )
                        .to_string())
                    },
                )?;
        }
        let plural_cases: BTreeSet<Count> = parsed_patterns
            .iter()
            .flat_map(|(_, plural_map)| plural_map.keys())
            .copied()
            .filter(|count| count != &Count::Explicit1)
            .collect();
        for log10_type in 0..=parsed_patterns.iter().last().map_or(0, |(key, _)| *key) {
            for plural_case in &plural_cases {
                parsed_patterns
                    .entry(log10_type)
                    .or_insert(BTreeMap::new())
                    .entry(*plural_case)
                    .or_insert(None);
            }
        }
        let mut patterns: BTreeMap<i8, BTreeMap<Count, Pattern>> = BTreeMap::new();
        for (log10_type, parsed_plural_map) in parsed_patterns {
            let plural_map = patterns.entry(log10_type).or_insert(BTreeMap::new());
            let other_pattern = parsed_plural_map
                .get(&Count::Other)
                .ok_or_else(|| format!("Missing other case for type 10^{}", log10_type))?
                .clone();
            let exponent: i8;
            match &other_pattern {
                None => {
                    if !parsed_plural_map.iter().all(|(_, p)| p.is_none()) {
                        return Err(format!(
                            "Non-0 pattern for type 10^{} whose pattern for count=other is 0",
                            log10_type,
                        )
                        .into());
                    }
                    exponent = 0;
                }
                Some(other_pattern) => {
                    let other_placeholder =
                        other_pattern.placeholder.as_ref().ok_or_else(|| {
                            format!(
                                "Missing placeholder in other case of type 10^{}",
                                log10_type
                            )
                        })?;
                    for (count, pattern) in &parsed_plural_map {
                        if let Some(pattern) = pattern {
                            if let Some(placeholder) = &pattern.placeholder {
                                if placeholder.number_of_0s != other_placeholder.number_of_0s {
                                    return Err(
                            format!(
                                "Inconsistent placeholders within type 10^{}: {} 0s for other, {} 0s for {:?}",
                                log10_type,
                                other_placeholder.number_of_0s,
                                placeholder.number_of_0s,
                                count
                            )
                            .into()
                        );
                                }
                            }
                        }
                    }
                    exponent = log10_type - other_placeholder.number_of_0s + 1;
                    if exponent < 1 {
                        return Err(format!(
                            "Too many 0s in type 10^{}, ({}, implying nonpositive exponent c={})",
                            log10_type, other_placeholder.number_of_0s, exponent
                        )
                        .into());
                    }
                }
            }
            for (count, optional_pattern) in parsed_plural_map {
                // Omit duplicates of the other case.
                // TODO(egg): optional_pattern.is_some_and(|p| p == other_pattern)
                if count != Count::Other && optional_pattern == other_pattern {
                    continue;
                }
                plural_map.insert(
                    count,
                    match optional_pattern {
                        None => Pattern {
                            exponent: 0,
                            literal_text: std::borrow::Cow::Borrowed(""),
                            index: 0,
                        },
                        Some(pattern) => Pattern {
                            exponent: exponent,
                            literal_text: pattern.literal_text,
                            index: pattern
                                .placeholder
                                .map_or(Some(u8::MAX), |p| {
                                    u8::try_from(p.index)
                                        .ok()
                                        .and_then(|i| (i < u8::MAX).then(|| i))
                                })
                                .ok_or_else(|| {
                                    format!(
                                        "Placeholder index is too large in type=10^{}, count={:?}",
                                        log10_type, count
                                    )
                                })?,
                        },
                    },
                );
            }
        }
        // Deduplicate sequences of types that have the same plural map (up to =1), keeping the lowest type.
        // The pattern 0 for type 1 is implicit.
        let deduplicated_patterns = patterns
            .iter()
            .coalesce(
                |(log10_low_type, low_plural_map), (log10_high_type, high_plural_map)| {
                    if low_plural_map.contains_key(&Count::Explicit1)
                        && low_plural_map
                            .iter()
                            .filter(|(count, _)| **count != Count::Explicit1)
                            .all(|(k, v)| high_plural_map.get(k) == Some(v))
                        && high_plural_map
                            .iter()
                            .all(|(k, v)| low_plural_map.get(k) == Some(v))
                    {
                        Ok((log10_low_type, low_plural_map))
                    } else if low_plural_map == high_plural_map {
                        Ok((log10_low_type, low_plural_map))
                    } else {
                        Err((
                            (log10_low_type, low_plural_map),
                            (log10_high_type, high_plural_map),
                        ))
                    }
                },
            )
            .filter(|(log10_type, plural_map)| {
                **log10_type != 0 || !plural_map.iter().all(|(_, pattern)| pattern.exponent == 0)
            });
        Ok(CompactDecimalPatternDataV1 {
            patterns: deduplicated_patterns
                .flat_map(|(log10_type, plural_map)| {
                    plural_map.iter().map(|(count, pattern)| {
                        (*log10_type, *count, encode_varule_to_box(pattern))
                    })
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_provider::zerofrom::ZeroFrom;
    use zerovec::ule::AsULE;

    #[test]
    fn test_french_compressibility() {
        // French compact-long thousands as of CLDR 42.
        // The type=1000, count=one case is incorrect (it is inconsistent with the
        // plural rules), but it is interesting because it forces a distinction
        // between 1000 and 10000 to be made in the ICU4X data.
        let cldr_42_long_french_data = CompactDecimalPatternDataV1::try_from(
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
        .unwrap();
        let cldr_42_long_french: Box<[(i8, Count, Pattern)]> = cldr_42_long_french_data
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            cldr_42_long_french.as_ref(),
            [
                (
                    3,
                    Count::One,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" millier")
                    }
                ),
                (
                    3,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" mille")
                    }
                ),
                (
                    3,
                    Count::Explicit1,
                    Pattern {
                        index: 255,
                        exponent: 3,
                        literal_text: Cow::Borrowed("mille")
                    }
                ),
                (
                    4,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" mille")
                    }
                ),
            ]
        );
        // French compact-long thousands, with the anomalous « millier » removed.
        // This allows 10000 and 1000 to be collapsed.
        let compressible_long_french_data = CompactDecimalPatternDataV1::try_from(
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
        .unwrap();
        let compressible_long_french: Box<[(i8, Count, Pattern)]> = compressible_long_french_data
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            compressible_long_french.as_ref(),
            [
                (
                    3,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" mille")
                    }
                ),
                (
                    3,
                    Count::Explicit1,
                    Pattern {
                        index: 255,
                        exponent: 3,
                        literal_text: Cow::Borrowed("mille")
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_holes() {
        // Spanish compact-short data as of CLDR 42, up to 10¹¹.
        // Note that the abbreviation for 10⁹ is used only starting with 10¹⁰.
        let spanish_data = CompactDecimalPatternDataV1::try_from(
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
        .unwrap();
        let spanish: Box<[(i8, Count, Pattern)]> = spanish_data
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1()
                    .map(move |(k, v)| (key0, Count::from_unaligned(*k), Pattern::zero_from(v)))
            })
            .collect();
        assert_eq!(
            spanish.as_ref(),
            [
                (
                    3,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 3,
                        literal_text: Cow::Borrowed(" mil")
                    }
                ),
                (
                    6,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 6,
                        literal_text: Cow::Borrowed(" M")
                    }
                ),
                (
                    10,
                    Count::Other,
                    Pattern {
                        index: 0,
                        exponent: 9,
                        literal_text: Cow::Borrowed(" mil M")
                    }
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
        assert_eq!(parse("M'.'").unwrap().unwrap().literal_text, "M.");
        assert_eq!(
            parse("0 0").err().unwrap(),
            "Multiple placeholders in compact decimal pattern 0 0"
        );
        assert_eq!(parse("0 '0'").unwrap().unwrap().literal_text, " 0");
        let zeros = str::repeat("0", 256);
        assert_eq!(
            parse(&zeros[..128]).err().unwrap(),
            String::from("Too many 0s in pattern ") + &zeros[..128]
        );
        assert_eq!(parse(&zeros[..127]).unwrap().unwrap().literal_text, "");
    }

    #[test]
    fn test_inter_pattern_errors() {
        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(
                    r#"{ "1000-count-other": "0k", "1000-count-other": "0K" }"#,
                )
                .unwrap(),
            )
            .err()
            .unwrap(),
            "Plural case Other is duplicated for type 10^3"
        );

        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1-count-one": "0" }"#).unwrap()
            )
            .err()
            .unwrap(),
            "Missing other case for type 10^0"
        );

        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(
                    r#"{ "1000-count-one": "0k", "1000-count-other": "0" }"#
                )
                .unwrap()
            )
            .err()
            .unwrap(),
            "Non-0 pattern for type 10^3 whose pattern for count=other is 0"
        );

        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "k" }"#).unwrap()
            )
            .err()
            .unwrap(),
            "Missing placeholder in other case of type 10^3"
        );

        // Given this data, it is ambiguous whether the 10 000 should be formatted as 10 thousand or 1 myriad.
        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
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
            "Inconsistent placeholders within type 10^4: 2 0s for other, 1 0s for One"
        );

        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(r#"{ "1000-count-other": "00000 tenths" }"#)
                    .unwrap()
            )
            .err()
            .unwrap(),
            "Too many 0s in type 10^3, (5, implying nonpositive exponent c=-1)"
        );

        let long_pattern = format!("thous{}nds (0)", str::repeat("a", 244));
        let overlong_pattern = format!("thous{}nds (0)", str::repeat("a", 245));

        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(
                    format!(r#"{{ "1000-count-other": "{}" }}"#, overlong_pattern).as_str()
                )
                .unwrap()
            )
            .err()
            .unwrap(),
            "Placeholder index is too large in type=10^3, count=Other"
        );

        assert_eq!(
            CompactDecimalPatternDataV1::try_from(
                &serde_json::from_str::<DecimalFormat>(
                    format!(r#"{{ "1000-count-other": "{}" }}"#, long_pattern).as_str()
                )
                .unwrap()
            )
            .unwrap()
            .patterns
            .get0(&3)
            .and_then(|plural_map| plural_map.get1(&Count::Other))
            .unwrap()
            .index,
            254
        );
    }
}
