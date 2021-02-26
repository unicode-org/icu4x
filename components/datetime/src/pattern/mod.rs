// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod error;
mod parser;

use crate::fields::{self, Field, FieldLength, FieldSymbol};
pub use error::Error;
use parser::Parser;
use std::iter::FromIterator;
use std::{convert::TryFrom, fmt};

#[cfg(feature = "provider_serde")]
use serde::{de, ser, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub enum PatternItem {
    Field(fields::Field),
    Literal(String),
}

impl From<(FieldSymbol, FieldLength)> for PatternItem {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self::Field(Field {
            symbol: input.0,
            length: input.1,
        })
    }
}

impl TryFrom<(FieldSymbol, u8)> for PatternItem {
    type Error = Error;
    fn try_from(input: (FieldSymbol, u8)) -> Result<Self, Self::Error> {
        let length = FieldLength::try_from(input.1).map_err(|_| Error::FieldTooLong(input.0))?;
        Ok(Self::Field(Field {
            symbol: input.0,
            length,
        }))
    }
}

impl<'p> From<&str> for PatternItem {
    fn from(input: &str) -> Self {
        Self::Literal(input.into())
    }
}

impl<'p> From<String> for PatternItem {
    fn from(input: String) -> Self {
        Self::Literal(input)
    }
}

/// The granularity of time represented in a pattern item.
/// Ordered from least granular to most granular for comparsion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum TimeGranularity {
    Hours,
    Minutes,
    Seconds,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pattern {
    items: Vec<PatternItem>,
    time_granularity: Option<TimeGranularity>,
}

/// Retrieves the granularity of time represented by a `PatternItem`.
/// If the `PatternItem` is not time-related, returns `None`.
fn get_time_granularity(item: &PatternItem) -> Option<TimeGranularity> {
    match item {
        PatternItem::Field(field) => match field.symbol {
            fields::FieldSymbol::Hour(_) => Some(TimeGranularity::Hours),
            fields::FieldSymbol::Minute => Some(TimeGranularity::Minutes),
            fields::FieldSymbol::Second(_) => Some(TimeGranularity::Seconds),
            _ => None,
        },
        _ => None,
    }
}

impl Pattern {
    pub fn items(&self) -> &[PatternItem] {
        &self.items
    }

    pub fn from_bytes(input: &str) -> Result<Self, Error> {
        Parser::new(input).parse().map(Pattern::from)
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(input: &str, date: Self, time: Self) -> Result<Self, Error> {
        Parser::new(input)
            .parse_placeholders(vec![time, date])
            .map(Pattern::from)
    }

    pub(super) fn most_granular_time(&self) -> Option<TimeGranularity> {
        self.time_granularity
    }
}

impl From<Vec<PatternItem>> for Pattern {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            time_granularity: items.iter().filter_map(get_time_granularity).max(),
            items,
        }
    }
}

impl From<&Pattern> for String {
    fn from(pattern: &Pattern) -> Self {
        let mut string = String::new();

        for pattern_item in pattern.items().iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        string.push(ch);
                    }
                }
                PatternItem::Literal(literal) => {
                    // Determine if the literal contains any characters that would need to be escaped.
                    let mut needs_escaping = false;
                    for ch in literal.chars() {
                        if ch.is_ascii_alphabetic() || ch == '\'' {
                            needs_escaping = true;
                            break;
                        }
                    }

                    if needs_escaping {
                        let mut ch_iter = literal.trim_end().chars().peekable();

                        // Do not escape the leading whitespace.
                        while let Some(ch) = ch_iter.peek() {
                            if ch.is_whitespace() {
                                string.push(*ch);
                                ch_iter.next();
                            } else {
                                break;
                            }
                        }

                        // Wrap in "'" and escape "'".
                        string.push('\'');
                        for ch in ch_iter {
                            if ch == '\'' {
                                // Escape a single quote.
                                string.push('\\');
                            }
                            string.push(ch);
                        }
                        string.push('\'');

                        // Add the trailing whitespace
                        for ch in literal.chars().rev() {
                            if ch.is_whitespace() {
                                string.push(ch);
                            } else {
                                break;
                            }
                        }
                    } else {
                        string.push_str(literal);
                    }
                }
            }
        }

        string
    }
}

impl FromIterator<PatternItem> for Pattern {
    fn from_iter<I: IntoIterator<Item = PatternItem>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(feature = "provider_serde")]
struct DeserializePatternVisitor;

#[cfg(feature = "provider_serde")]
impl<'de> de::Visitor<'de> for DeserializePatternVisitor {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Expected to find a valid pattern.")
    }

    fn visit_str<E>(self, pattern_string: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // Parse a string into a list of fields.
        let pattern = match Pattern::from_bytes(pattern_string) {
            Ok(p) => p,
            Err(err) => {
                return Err(E::custom(format!(
                    "The pattern \"{}\" could not be parsed: {:?}",
                    pattern_string, err
                )));
            }
        };
        Ok(pattern)
    }
}

#[cfg(feature = "provider_serde")]
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Pattern, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DeserializePatternVisitor)
    }
}

#[cfg(feature = "provider_serde")]
impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let string: String = String::from(self);
        serializer.serialize_str(&string)
    }
}

#[cfg(all(test, feature = "provider_serde"))]
mod test {
    use super::*;

    // Provide a few patterns to sanity check serialization.
    const PATTERNS: [&str; 6] = [
        "d",
        "E, M/d/y",
        "h:mm:ss a",
        // TODO(#502) - The W field isn't supported yet, so swap it out for a field we do know.
        "'week' d 'of' MMMM",
        // Arabic "week of" in the availableFormats.
        "الأسبوع d من MMMM",
        // Cherokee "week of" in the availableFormats.
        "ᏒᎾᏙᏓᏆᏍᏗ’ d ’ᎾᎿ’ MMMM",
    ];

    #[test]
    fn test_pattern_serialization_roundtrip() {
        for pattern_string in &PATTERNS {
            // Wrap the string in quotes so it's a JSON string.
            let json_in: String = serde_json::to_string(pattern_string).unwrap();

            let pattern: Pattern = match serde_json::from_str(&json_in) {
                Ok(p) => p,
                Err(err) => {
                    panic!(
                        "Unable to parse the pattern {:?}. {:?}",
                        pattern_string, err
                    );
                }
            };

            let json_out = match serde_json::to_string(&pattern) {
                Ok(s) => s,
                Err(err) => {
                    panic!(
                        "Unable to re-serialize the pattern {:?}. {:?}",
                        pattern_string, err
                    );
                }
            };

            assert_eq!(
                json_in, json_out,
                "The roundtrip serialization for the pattern matched."
            );
        }
    }
}
