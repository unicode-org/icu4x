// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Functions for dealing with UTS-35 number patterns.
//!
//! Spec reference: <https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns>

use displaydoc::Display;
#[cfg(feature = "experimental")]
use icu_pattern::{DoublePlaceholderKey, PatternItemCow};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Display, Debug, PartialEq)]
pub(crate) enum Error {
    #[displaydoc("No body in decimal subpattern")]
    NoBodyInSubpattern,
    #[displaydoc("Unclosed quote in pattern")]
    UnclosedQuote,
    #[displaydoc("Non-literal item in prefix or suffix")]
    InvalidAffixItem,
    #[displaydoc("More than two grouping separators in pattern")]
    TooManyGroupingSeparators,
    #[displaydoc("Invalid item in pattern body")]
    InvalidBodyItem,
    #[displaydoc("Mandatory digit after optional digit in fraction")]
    MandatoryAfterOptional,
}

/// An item in a decimal pattern (used during parsing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DecimalPatternItem {
    Literal(String),
    MandatoryDigit,
    OptionalDigit,
    DecimalSeparator,
    GroupingSeparator,
    Currency,
    Percent,
    PerMille,
    PlusSign,
    MinusSign,
    Exponent,
}

/// Tokenizes a decimal pattern string into items.
/// Handles quoted literals and special pattern characters.
fn tokenize_pattern(s: &str) -> Result<Vec<DecimalPatternItem>, Error> {
    let mut items = Vec::new();
    let mut chars = s.chars().peekable();
    let mut in_quote = false;
    let mut string_buffer = String::new();

    fn append_literal(items: &mut Vec<DecimalPatternItem>, s: &str) {
        if let Some(DecimalPatternItem::Literal(last)) = items.last_mut() {
            last.push_str(s);
        } else {
            items.push(DecimalPatternItem::Literal(s.to_string()));
        }
    }

    while let Some(c) = chars.next() {
        if in_quote {
            if c == '\'' {
                if chars.peek() == Some(&'\'') {
                    // Escaped quote ''
                    string_buffer.push('\'');
                    chars.next();
                } else {
                    // End of quote
                    in_quote = false;
                    if !string_buffer.is_empty() {
                        append_literal(&mut items, &string_buffer);
                        string_buffer.clear();
                    }
                }
            } else {
                string_buffer.push(c);
            }
        } else {
            match c {
                '\'' => {
                    in_quote = true;
                }
                '0' => items.push(DecimalPatternItem::MandatoryDigit),
                '#' => items.push(DecimalPatternItem::OptionalDigit),
                '.' => items.push(DecimalPatternItem::DecimalSeparator),
                ',' => items.push(DecimalPatternItem::GroupingSeparator),
                '¤' => items.push(DecimalPatternItem::Currency),
                '%' => items.push(DecimalPatternItem::Percent),
                '‰' => items.push(DecimalPatternItem::PerMille),
                '+' => items.push(DecimalPatternItem::PlusSign),
                '-' => items.push(DecimalPatternItem::MinusSign),
                'E' => items.push(DecimalPatternItem::Exponent),
                _ => {
                    // Unquoted literal character
                    let mut temp = String::new();
                    temp.push(c);
                    append_literal(&mut items, &temp);
                }
            }
        }
    }

    if in_quote {
        return Err(Error::UnclosedQuote);
    }

    Ok(items)
}

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

impl FromStr for DecimalSubPattern {
    type Err = Error;

    fn from_str(subpattern: &str) -> Result<Self, Self::Err> {
        let items = tokenize_pattern(subpattern)?;

        // Find the first body token (digit placeholder, separator)
        let body_start = items.iter().position(|item| {
            matches!(
                item,
                DecimalPatternItem::MandatoryDigit
                    | DecimalPatternItem::OptionalDigit
                    | DecimalPatternItem::GroupingSeparator
                    | DecimalPatternItem::DecimalSeparator
            )
        });

        let body_start = match body_start {
            Some(i) => i,
            None => return Err(Error::NoBodyInSubpattern),
        };

        // Find the last body token
        let body_end = items.iter().rposition(|item| {
            matches!(
                item,
                DecimalPatternItem::MandatoryDigit
                    | DecimalPatternItem::OptionalDigit
                    | DecimalPatternItem::GroupingSeparator
                    | DecimalPatternItem::DecimalSeparator
            )
        });
        let body_end = body_end.unwrap_or(body_start);

        // Validate prefix: all items before body must be literals
        for item in &items[..body_start] {
            if !matches!(item, DecimalPatternItem::Literal(_)) {
                return Err(Error::InvalidAffixItem);
            }
        }

        // Validate suffix: all items after body must be literals
        for item in &items[body_end + 1..] {
            if !matches!(item, DecimalPatternItem::Literal(_)) {
                return Err(Error::InvalidAffixItem);
            }
        }

        // Extract prefix
        let prefix: String = items[..body_start]
            .iter()
            .filter_map(|item| {
                if let DecimalPatternItem::Literal(s) = item {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect();

        // Extract suffix
        let suffix: String = items[body_end + 1..]
            .iter()
            .filter_map(|item| {
                if let DecimalPatternItem::Literal(s) = item {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect();

        // Validate body: only digit placeholders and separators are allowed
        let body_items = &items[body_start..=body_end];
        for item in body_items {
            match item {
                DecimalPatternItem::DecimalSeparator
                | DecimalPatternItem::GroupingSeparator
                | DecimalPatternItem::MandatoryDigit
                | DecimalPatternItem::OptionalDigit => {}
                _ => return Err(Error::InvalidBodyItem),
            }
        }

        // Find decimal separator position
        let decimal_pos = body_items
            .iter()
            .position(|item| matches!(item, DecimalPatternItem::DecimalSeparator));

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
                if matches!(item, DecimalPatternItem::GroupingSeparator) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // Reject if there are more than two grouping separators
        if grouping_positions.len() > 2 {
            return Err(Error::TooManyGroupingSeparators);
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
                        DecimalPatternItem::MandatoryDigit | DecimalPatternItem::OptionalDigit
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
                            DecimalPatternItem::MandatoryDigit | DecimalPatternItem::OptionalDigit
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
                    DecimalPatternItem::MandatoryDigit => {
                        if seen_optional {
                            return Err(Error::MandatoryAfterOptional);
                        }
                    }
                    DecimalPatternItem::OptionalDigit => {
                        seen_optional = true;
                    }
                    _ => {}
                }
            }

            let mandatory: u8 = fraction_items
                .iter()
                .filter(|item| matches!(item, DecimalPatternItem::MandatoryDigit))
                .count() as u8;
            let optional: u8 = fraction_items
                .iter()
                .filter(|item| matches!(item, DecimalPatternItem::OptionalDigit))
                .count() as u8;
            (mandatory, mandatory + optional)
        } else {
            (0, 0)
        };

        Ok(Self {
            prefix,
            suffix,
            primary_grouping,
            secondary_grouping,
            min_fraction_digits,
            max_fraction_digits,
        })
    }
}

impl DecimalSubPattern {
    #[cfg(feature = "experimental")]
    pub(crate) fn to_pattern_items(&self) -> Vec<PatternItemCow<'_, DoublePlaceholderKey>> {
        use std::borrow::Cow;
        vec![
            PatternItemCow::Literal(Cow::Borrowed(&self.prefix)),
            PatternItemCow::Placeholder(DoublePlaceholderKey::Place0),
            PatternItemCow::Literal(Cow::Borrowed(&self.suffix)),
        ]
    }
}

/// Representation of a UTS-35 number pattern, including positive subpattern (required) and negative
/// subpattern (optional).
#[derive(Debug, PartialEq)]
pub(crate) struct DecimalPattern {
    pub(crate) positive: DecimalSubPattern,
    pub(crate) negative: Option<DecimalSubPattern>,
}

impl FromStr for DecimalPattern {
    type Err = Error;

    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
        // Example patterns:
        // #,##0
        // #,##,##0.###
        // #,##0.00;#,##0.00-
        // 0;0-
        let (positive, negative) = match pattern.split(';').next_tuple() {
            Some((u, s)) => (u.parse()?, Some(s.parse()?)),
            None => (pattern.parse()?, None),
        };
        Ok(Self { positive, negative })
    }
}

impl DecimalPattern {
    // Returns affixes in the form (prefix, suffix)
    pub(crate) fn localize_sign(&self, sign_str: &str) -> (String, String) {
        // UTS 35: the absence of a negative pattern means a single prefixed sign
        let signed_affixes = self
            .negative
            .as_ref()
            .map(|subpattern| (subpattern.prefix.as_str(), subpattern.suffix.as_str()))
            .unwrap_or_else(|| ("-", ""));
        (
            signed_affixes.0.replace('-', sign_str),
            signed_affixes.1.replace('-', sign_str),
        )
    }
}

#[test]
fn test_basic() {
    #[derive(Debug)]
    struct TestCase<'s> {
        pub(crate) pattern: &'s str,
        pub(crate) expected: Result<DecimalPattern, Error>,
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
            expected: Err(Error::NoBodyInSubpattern),
        },
        TestCase {
            pattern: "xyz;abc",
            expected: Err(Error::NoBodyInSubpattern),
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
        let actual = DecimalPattern::from_str(cas.pattern);
        assert_eq!(cas.expected, actual, "Pattern: {}", cas.pattern);
    }
}

#[test]
fn test_quoted_literals() {
    // Test escaped quote
    let pattern = DecimalPattern::from_str("'O''clock'#,##0.###").unwrap();
    assert_eq!(pattern.positive.prefix, "O'clock");

    // Test quoted special characters
    let pattern = DecimalPattern::from_str("'#'#,##0.###").unwrap();
    assert_eq!(pattern.positive.prefix, "#");
}

#[test]
fn test_reject_three_grouping_separators() {
    // Three grouping separators should be rejected
    let result = DecimalPattern::from_str("#,##,##,##0");
    assert_eq!(result, Err(Error::TooManyGroupingSeparators));
}

#[test]
fn test_reject_mandatory_after_optional_in_fraction() {
    // Pattern like #,##0.#0 is invalid (mandatory after optional)
    let result = DecimalPattern::from_str("#,##0.#0");
    assert_eq!(result, Err(Error::MandatoryAfterOptional));

    // Valid: mandatory then optional
    let result = DecimalPattern::from_str("#,##0.00##");
    assert!(result.is_ok());
    let pattern = result.unwrap();
    assert_eq!(pattern.positive.min_fraction_digits, 2);
    assert_eq!(pattern.positive.max_fraction_digits, 4);
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
    let result = DecimalPattern::from_str("#,##0%0.###");
    assert_eq!(result, Err(Error::InvalidBodyItem));
}
