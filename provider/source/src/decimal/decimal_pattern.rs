// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Functions for dealing with UTS-35 number patterns.
//!
//! Spec reference: <https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns>

use icu_provider::DataError;

#[cfg(test)]
use crate::cldr_serde::numbers::NumberPattern;
use crate::cldr_serde::numbers::NumberPatternItem;

/// Representation of a UTS-35 number subpattern (part of a number pattern between ';'s).
#[derive(Debug, PartialEq)]
pub(crate) struct DecimalSubPattern {
    pub(crate) prefix: String,
    pub(crate) suffix: String,
    pub(crate) primary_grouping: u8,
    pub(crate) secondary_grouping: u8,
    pub(crate) min_fraction_digits: u8,
    pub(crate) max_fraction_digits: u8,
}

impl DecimalSubPattern {
    pub fn try_from_items(items: &[NumberPatternItem]) -> Result<Self, DataError> {
        // Find the first body token (digit placeholder, separator)
        let body_start = items.iter().position(|item| {
            matches!(
                item,
                NumberPatternItem::MandatoryDigit
                    | NumberPatternItem::OptionalDigit
                    | NumberPatternItem::GroupingSeparator
                    | NumberPatternItem::DecimalSeparator
            )
        });

        let body_start = match body_start {
            Some(i) => i,
            None => return Err(DataError::custom("NoBodyInSubpattern")),
        };

        // Find the last body token
        let body_end = items.iter().rposition(|item| {
            matches!(
                item,
                NumberPatternItem::MandatoryDigit
                    | NumberPatternItem::OptionalDigit
                    | NumberPatternItem::GroupingSeparator
                    | NumberPatternItem::DecimalSeparator
            )
        });
        let body_end = body_end.unwrap_or(body_start);

        // Validate and extract prefix: must be literals or affix symbols (¤, %, ‰, +, -)
        let mut prefix = String::new();
        for item in &items[..body_start] {
            match item {
                NumberPatternItem::Literal(s) => prefix.push_str(s),
                _ => return Err(DataError::custom("InvalidAffixItem")),
            }
        }

        // Validate and extract suffix: same rules as prefix
        let mut suffix = String::new();
        for item in &items[body_end + 1..] {
            match item {
                NumberPatternItem::Literal(s) => suffix.push_str(s),
                _ => return Err(DataError::custom("InvalidAffixItem")),
            }
        }

        // Validate body: only digit placeholders and separators are allowed
        let body_items = &items[body_start..=body_end];
        for item in body_items {
            match item {
                NumberPatternItem::DecimalSeparator
                | NumberPatternItem::GroupingSeparator
                | NumberPatternItem::MandatoryDigit
                | NumberPatternItem::OptionalDigit => {}
                _ => return Err(DataError::custom("InvalidBodyItem")),
            }
        }

        // Find decimal separator position
        let decimal_pos = body_items
            .iter()
            .position(|item| matches!(item, NumberPatternItem::DecimalSeparator));

        // Calculate grouping sizes from the integer part
        let integer_items = if let Some(pos) = decimal_pos {
            &body_items[..pos]
        } else {
            body_items
        };

        // Find grouping positions (positions of GroupingSeparator)
        let grouping_positions: Vec<usize> = integer_items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                if matches!(item, NumberPatternItem::GroupingSeparator) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // Reject if there are more than two grouping separators
        if grouping_positions.len() > 2 {
            return Err(DataError::custom("TooManyGroupingSeparators"));
        }

        // Count digits after each grouping separator to determine grouping sizes
        let (primary_grouping, secondary_grouping) = if grouping_positions.is_empty() {
            (0, 0)
        } else {
            // Primary grouping: digits from last separator to end of integer part
            let last_sep = grouping_positions.last().unwrap();
            let digits_after_last: u8 = integer_items[last_sep + 1..]
                .iter()
                .filter(|item| {
                    matches!(
                        item,
                        NumberPatternItem::MandatoryDigit | NumberPatternItem::OptionalDigit
                    )
                })
                .count() as u8;

            // Secondary grouping: if there's more than one separator, measure between them
            let secondary = if grouping_positions.len() > 1 {
                let second_last_sep = grouping_positions[grouping_positions.len() - 2];
                integer_items[second_last_sep + 1..*last_sep]
                    .iter()
                    .filter(|item| {
                        matches!(
                            item,
                            NumberPatternItem::MandatoryDigit | NumberPatternItem::OptionalDigit
                        )
                    })
                    .count() as u8
            } else {
                digits_after_last
            };

            (digits_after_last, secondary)
        };

        // Calculate fraction digits from the fractional part
        let (min_fraction_digits, max_fraction_digits) = if let Some(pos) = decimal_pos {
            let fraction_items = &body_items[pos + 1..];

            // Validate: mandatory digits must come before optional digits
            let mut seen_optional = false;
            for item in fraction_items {
                match item {
                    NumberPatternItem::MandatoryDigit => {
                        if seen_optional {
                            return Err(DataError::custom("MandatoryAfterOptional"));
                        }
                    }
                    NumberPatternItem::OptionalDigit => {
                        seen_optional = true;
                    }
                    _ => {}
                }
            }

            let mandatory: u8 = fraction_items
                .iter()
                .filter(|item| matches!(item, NumberPatternItem::MandatoryDigit))
                .count() as u8;
            let optional: u8 = fraction_items
                .iter()
                .filter(|item| matches!(item, NumberPatternItem::OptionalDigit))
                .count() as u8;
            (mandatory, mandatory + optional)
        } else {
            (0, 0)
        };

        Ok(DecimalSubPattern {
            prefix,
            suffix,
            primary_grouping,
            secondary_grouping,
            min_fraction_digits,
            max_fraction_digits,
        })
    }
}

#[test]
fn test_basic() {
    #[derive(PartialEq, Debug)]
    struct DecimalPattern {
        positive: DecimalSubPattern,
        negative: Option<DecimalSubPattern>,
    }

    #[derive(Debug)]
    struct TestCase<'s> {
        pub(crate) pattern: &'s str,
        pub(crate) expected: Result<DecimalPattern, DataError>,
    }
    let cases = [
        TestCase {
            pattern: "#,##0.###",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "".into(),
                    suffix: "".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: None,
            }),
        },
        TestCase {
            pattern: "a#,##0.###",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "a".into(),
                    suffix: "".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: None,
            }),
        },
        TestCase {
            pattern: "#,##0.###b",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "".into(),
                    suffix: "b".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: None,
            }),
        },
        TestCase {
            pattern: "aaa#,##0.###bbb",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "aaa".into(),
                    suffix: "bbb".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: None,
            }),
        },
        TestCase {
            pattern: "aaa#,##0.###bbb;ccc#,##0.###ddd",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "aaa".into(),
                    suffix: "bbb".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: Some(DecimalSubPattern {
                    prefix: "ccc".into(),
                    suffix: "ddd".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                }),
            }),
        },
        TestCase {
            pattern: "xyz",
            expected: Err(DataError::custom("NoBodyInSubpattern")),
        },
        TestCase {
            pattern: "xyz;abc",
            expected: Err(DataError::custom("NoBodyInSubpattern")),
        },
        // Test quoted literals
        TestCase {
            pattern: "'Prefix'#,##0.###",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "Prefix".into(),
                    suffix: "".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: None,
            }),
        },
        // Test Indic grouping pattern
        TestCase {
            pattern: "#,##,##0.###",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "".into(),
                    suffix: "".into(),
                    primary_grouping: 3,
                    secondary_grouping: 2,
                    min_fraction_digits: 0,
                    max_fraction_digits: 3,
                },
                negative: None,
            }),
        },
        // Test fixed fraction digits
        TestCase {
            pattern: "#,##0.00",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "".into(),
                    suffix: "".into(),
                    primary_grouping: 3,
                    secondary_grouping: 3,
                    min_fraction_digits: 2,
                    max_fraction_digits: 2,
                },
                negative: None,
            }),
        },
        // Test no grouping
        TestCase {
            pattern: "0.######",
            expected: Ok(DecimalPattern {
                positive: DecimalSubPattern {
                    prefix: "".into(),
                    suffix: "".into(),
                    primary_grouping: 0,
                    secondary_grouping: 0,
                    min_fraction_digits: 0,
                    max_fraction_digits: 6,
                },
                negative: None,
            }),
        },
    ];
    for cas in &cases {
        let actual = NumberPattern::try_from_str(cas.pattern).and_then(|a| {
            Ok(DecimalPattern {
                positive: DecimalSubPattern::try_from_items(&a.positive)?,
                negative: a
                    .negative
                    .as_ref()
                    .map(|n| DecimalSubPattern::try_from_items(n))
                    .transpose()?,
            })
        });
        assert_eq!(cas.expected, actual, "Pattern: {}", cas.pattern);
    }
}

#[test]
fn test_quoted_literals() {
    // Test escaped quote
    let pattern = NumberPattern::try_from_str("'O''clock'#,##0.###").unwrap();
    assert_eq!(
        pattern.positive[0],
        NumberPatternItem::Literal("O'clock".into())
    );

    // Test quoted special characters
    let pattern = NumberPattern::try_from_str("'#'#,##0.###").unwrap();
    assert_eq!(pattern.positive[0], NumberPatternItem::Literal("#".into()));
}

#[test]
fn test_reject_three_grouping_separators() {
    // Three grouping separators should be rejected
    let result = DecimalSubPattern::try_from_items(
        &NumberPattern::try_from_str("#,##,##,##0").unwrap().positive,
    );
    assert_eq!(result, Err(DataError::custom("TooManyGroupingSeparators")));
}

#[test]
fn test_reject_mandatory_after_optional_in_fraction() {
    // Pattern like #,##0.#0 is invalid (mandatory after optional)
    let result = DecimalSubPattern::try_from_items(
        &NumberPattern::try_from_str("#,##0.#0").unwrap().positive,
    );
    assert_eq!(result, Err(DataError::custom("MandatoryAfterOptional")));

    // Valid: mandatory then optional
    let result = DecimalSubPattern::try_from_items(
        &NumberPattern::try_from_str("#,##0.00##").unwrap().positive,
    );
    assert!(result.is_ok());
    let pattern = result.unwrap();
    assert_eq!(pattern.min_fraction_digits, 2);
    assert_eq!(pattern.max_fraction_digits, 4);
}

#[test]
fn test_reject_invalid_body_item() {
    // A literal inside the body should be rejected
    // This would happen if someone wrote something like #,##0a.### where 'a' is between digits
    // However, our tokenizer treats unquoted 'a' as a literal, and body detection
    // considers it as ending the body. So let's test a quoted literal in the body.
    // Actually, the current logic defines body as the span from first to last body token,
    // so a literal in between would be caught.

    // Test: pattern with percent in the body (not allowed)
    let result = DecimalSubPattern::try_from_items(
        &NumberPattern::try_from_str("#,##0%0.###").unwrap().positive,
    );
    assert_eq!(result, Err(DataError::custom("InvalidBodyItem")));
}
