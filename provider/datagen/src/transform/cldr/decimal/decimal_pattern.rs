// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Functions for dealing with UTS-35 number patterns.
//!
//! Spec reference: <https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns>

use displaydoc::Display;
use icu_decimal::provider::AffixesV1;
use itertools::Itertools;
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Display, Debug, PartialEq)]
pub enum Error {
    #[displaydoc("No body in decimal subpattern")]
    NoBodyInSubpattern,
    #[displaydoc("Unknown decimal body: {0}")]
    UnknownPatternBody(String),
}

impl std::error::Error for Error {}

/// Representation of a UTS-35 number subpattern (part of a number pattern between ';'s).
#[derive(Debug, PartialEq)]
pub struct DecimalSubPattern {
    pub prefix: String,
    pub suffix: String,
    pub primary_grouping: u8,
    pub secondary_grouping: u8,
    pub min_fraction_digits: u8,
    pub max_fraction_digits: u8,
}

impl FromStr for DecimalSubPattern {
    type Err = Error;

    #[allow(clippy::many_single_char_names)]
    fn from_str(subpattern: &str) -> Result<Self, Self::Err> {
        // Split the subpattern into prefix, body, and suffix.
        // TODO(#567): Handle quoted literals in prefix and suffix.
        // i = boundary between prefix and body
        // j = boundary between body and suffix
        let i = subpattern.find(|c: char| matches!(c, '#' | '0' | ',' | '.'));
        let i = match i {
            Some(i) => i,
            None => return Err(Error::NoBodyInSubpattern),
        };
        let j = subpattern[i..]
            .find(|c: char| !matches!(c, '#' | '0' | ',' | '.'))
            .unwrap_or(subpattern.len() - i)
            + i;
        let prefix = &subpattern[..i];
        let body = &subpattern[i..j];
        let suffix = &subpattern[j..];

        // For now, we expect one of a handful of pattern bodies.
        // TODO(#567): Generalize this to support all of UTS 35.
        let (a, b, c, d) = match body {
            "#,##0.###" => (3, 3, 0, 3),
            "#,##,##0.###" => (3, 2, 0, 3),
            "0.######" => (0, 0, 0, 6),
            _ => return Err(Error::UnknownPatternBody(body.to_string())),
        };
        Ok(Self {
            prefix: prefix.into(),
            suffix: suffix.into(),
            primary_grouping: a,
            secondary_grouping: b,
            min_fraction_digits: c,
            max_fraction_digits: d,
        })
    }
}

/// Representation of a UTS-35 number pattern, including positive subpattern (required) and negative
/// subpattern (optional).
#[derive(Debug, PartialEq)]
pub struct DecimalPattern {
    pub positive: DecimalSubPattern,
    pub negative: Option<DecimalSubPattern>,
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
    pub fn localize_sign(&self, sign_str: &str) -> AffixesV1<'static> {
        // UTS 35: the absence of a negative pattern means a single prefixed sign
        let signed_affixes = self
            .negative
            .as_ref()
            .map(|subpattern| (subpattern.prefix.as_str(), subpattern.suffix.as_str()))
            .unwrap_or_else(|| ("-", ""));
        AffixesV1 {
            prefix: Cow::Owned(signed_affixes.0.replace('-', sign_str)),
            suffix: Cow::Owned(signed_affixes.1.replace('-', sign_str)),
        }
    }
}

#[test]
fn test_basic() {
    #[derive(Debug)]
    struct TestCase<'s> {
        pub pattern: &'s str,
        pub expected: Result<DecimalPattern, Error>,
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
        TestCase {
            pattern: "aaa#0#bbb",
            expected: Err(Error::UnknownPatternBody("#0#".to_string())),
        },
    ];
    for cas in &cases {
        let actual = DecimalPattern::from_str(cas.pattern);
        assert_eq!(cas.expected, actual, "Pattern: {}", cas.pattern);
    }
}
