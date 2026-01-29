// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_decimal::pattern::{DecimalPattern, DecimalPatternItem};
use std::str::FromStr;

#[test]
fn test_basic_pattern() {
    let pattern_str = "#,##0.00";
    let pattern = DecimalPattern::from_str(pattern_str).unwrap();
    assert_eq!(
        pattern.items,
        vec![
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::GroupingSeparator,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::DecimalSeparator,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::MandatoryDigit,
        ]
    );
}

#[test]
fn test_quoted_literal() {
    let pattern_str = "'Prefix' 0";
    let pattern = DecimalPattern::from_str(pattern_str).unwrap();
    assert_eq!(
        pattern.items,
        vec![
            DecimalPatternItem::Literal("Prefix ".to_string()),
            DecimalPatternItem::MandatoryDigit,
        ]
    );
}

#[test]
fn test_complex_quotes() {
    let pattern_str = "'O''clock' 0";
    let pattern = DecimalPattern::from_str(pattern_str).unwrap();
    assert_eq!(
        pattern.items,
        vec![
            DecimalPatternItem::Literal("O'clock ".to_string()),
            DecimalPatternItem::MandatoryDigit,
        ]
    );
}

#[test]
fn test_currency() {
    let pattern_str = "¤#,##0.00";
    let pattern = DecimalPattern::from_str(pattern_str).unwrap();
    assert_eq!(
        pattern.items,
        vec![
            DecimalPatternItem::Currency,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::GroupingSeparator,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::DecimalSeparator,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::MandatoryDigit,
        ]
    );
}

#[test]
fn test_literal_and_special() {
    let pattern_str = "#,##0.00 'units'";
    let pattern = DecimalPattern::from_str(pattern_str).unwrap();
    assert_eq!(
        pattern.items,
        vec![
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::GroupingSeparator,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::OptionalDigit,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::DecimalSeparator,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::MandatoryDigit,
            DecimalPatternItem::Literal(" units".to_string()),
        ]
    );
}
