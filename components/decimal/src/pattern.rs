// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::str::FromStr;

/// Error type for decimal pattern parsing
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DecimalPatternError {
    /// The pattern contains an unclosed quote
    UnclosedQuote,
    /// The pattern contains an unexpected character
    UnexpectedChar,
}

impl core::fmt::Display for DecimalPatternError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnclosedQuote => write!(f, "Unclosed quote in pattern"),
            Self::UnexpectedChar => write!(f, "Unexpected character in pattern"),
        }
    }
}

/// An item in a decimal pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalPatternItem {
    /// A literal string
    Literal(String),
    /// A mandatory digit '0'
    MandatoryDigit,
    /// An optional digit '#'
    OptionalDigit,
    /// A decimal separator '.'
    DecimalSeparator,
    /// A grouping separator ','
    GroupingSeparator,
    /// A currency symbol '¤'
    Currency,
    /// A percent sign '%'
    Percent,
    /// A per mille sign '‰'
    PerMille,
    /// A plus sign '+'
    PlusSign,
    /// A minus sign '-'
    MinusSign,
    /// An exponent separator 'E'
    Exponent,
}

/// A parsed decimal pattern
///
/// # Examples
///
/// ```
/// use icu::decimal::pattern::DecimalPattern;
///
/// let pattern: DecimalPattern = "#,##0.00".parse().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DecimalPattern {
    /// The items in the pattern
    pub items: Vec<DecimalPatternItem>,
}

impl DecimalPattern {
    /// Parses a decimal pattern string
    pub fn parse(s: &str) -> Result<Self, DecimalPatternError> {
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
                        // Escaped quote
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
                        // Unquoted literal
                        let mut temp = String::new();
                        temp.push(c);
                        append_literal(&mut items, &temp);
                    }
                }
            }
        }

        if in_quote {
            return Err(DecimalPatternError::UnclosedQuote);
        }

        Ok(DecimalPattern { items })
    }
}

impl FromStr for DecimalPattern {
    type Err = DecimalPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
