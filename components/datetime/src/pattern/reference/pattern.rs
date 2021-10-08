// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{PatternError, PatternItem, TimeGranularity},
    Parser,
};
use alloc::fmt::{self, Write};
#[cfg(feature = "provider_serde")]
use alloc::format;
use alloc::string::String;
#[cfg(feature = "provider_serde")]
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
#[cfg(feature = "provider_serde")]
use serde::{
    de,
    ser::{self, SerializeSeq},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pattern {
    pub items: Vec<PatternItem>,
    pub(crate) time_granularity: TimeGranularity,
}

impl Pattern {
    pub fn items(&self) -> &[PatternItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [PatternItem] {
        &mut self.items
    }

    pub fn from_bytes(input: &str) -> Result<Self, PatternError> {
        Parser::new(input).parse().map(Self::from)
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(
        input: &str,
        date: Self,
        time: Self,
    ) -> Result<Self, PatternError> {
        Parser::new(input)
            .parse_placeholders(vec![time, date])
            .map(Self::from)
    }
}

impl From<Vec<PatternItem>> for Pattern {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            time_granularity: items.iter().map(Into::into).max().unwrap_or_default(),
            items,
        }
    }
}

impl From<&str> for Pattern {
    fn from(items: &str) -> Self {
        Self {
            time_granularity: TimeGranularity::default(),
            items: items.chars().map(|ch| ch.into()).collect(),
        }
    }
}

pub(crate) fn dump_buffer_into_formatter(
    literal: &str,
    formatter: &mut fmt::Formatter,
) -> fmt::Result {
    if literal.is_empty() {
        return Ok(());
    }
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
                formatter.write_char(*ch)?;
                ch_iter.next();
            } else {
                break;
            }
        }

        // Wrap in "'" and escape "'".
        formatter.write_char('\'')?;
        for ch in ch_iter {
            if ch == '\'' {
                // Escape a single quote.
                formatter.write_char('\\')?;
            }
            formatter.write_char(ch)?;
        }
        formatter.write_char('\'')?;

        // Add the trailing whitespace
        for ch in literal.chars().rev() {
            if ch.is_whitespace() {
                formatter.write_char(ch)?;
            } else {
                break;
            }
        }
    } else {
        formatter.write_str(literal)?;
    }
    Ok(())
}

/// This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35
/// pattern string. It could also be implemented as the Writeable trait, but at the time of writing
/// this was not done, as this code would need to implement the [`write_len()`] method, which would
/// need to duplicate the branching logic of the [`fmt`](std::fmt) method here. This code is used in generating
/// the data providers and is not as performance sensitive.
impl fmt::Display for Pattern {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for pattern_item in self.items.iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    dump_buffer_into_formatter(&buffer, formatter)?;
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(*ch);
                }
            }
        }
        dump_buffer_into_formatter(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}

#[cfg(feature = "provider_serde")]
#[allow(clippy::upper_case_acronyms)]
pub(crate) struct DeserializePatternUTS35String;

#[cfg(feature = "provider_serde")]
impl<'de> de::Visitor<'de> for DeserializePatternUTS35String {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Expected to find a valid pattern.")
    }

    fn visit_str<E>(self, pattern_string: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // Parse a string into a list of fields.
        Pattern::from_bytes(pattern_string).map_err(|err| {
            de::Error::invalid_value(
                de::Unexpected::Other(&format!("{}", err)),
                &"a valid UTS 35 pattern string",
            )
        })
    }
}

#[cfg(feature = "provider_serde")]
pub(crate) struct DeserializePatternBinary;

#[cfg(feature = "provider_serde")]
impl<'de> de::Visitor<'de> for DeserializePatternBinary {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Unable to deserialize a bincode Pattern.")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Pattern, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(item) = seq.next_element()? {
            items.push(item)
        }
        Ok(Pattern::from(items))
    }
}

#[cfg(feature = "provider_serde")]
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(DeserializePatternUTS35String)
        } else {
            deserializer.deserialize_seq(DeserializePatternBinary)
        }
    }
}

#[cfg(feature = "provider_serde")]
impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            // Serialize into the UTS 35 string representation.
            let string: String = self.to_string();
            serializer.serialize_str(&string)
        } else {
            // Serialize into a bincode-friendly representation. This means that pattern parsing
            // will not be needed when deserializing.
            let mut seq = serializer.serialize_seq(Some(self.items.len()))?;
            for item in self.items.iter() {
                seq.serialize_element(item)?;
            }
            seq.end()
        }
    }
}
