// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{PatternError, PatternItem, TimeGranularity},
    Parser,
};
use crate::fields::{Field, FieldSymbol, Week};
use alloc::fmt::{self, Write};
#[cfg(feature = "provider_serde")]
use alloc::format;
use alloc::string::String;
#[cfg(feature = "provider_serde")]
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use icu_plurals::PluralCategory;
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

#[cfg(feature = "provider_serde")]
mod week_field_serialization {
    use crate::fields::{FieldSymbol, Week};
    use alloc::format;
    use core::convert::TryInto;
    use serde::{self, de, Deserialize};

    pub fn serialize<S>(week: &Week, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_char(FieldSymbol::Week(*week).into())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Week, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let c = char::deserialize(deserializer)?;
        let symbol: FieldSymbol = c.try_into().map_err(|err| {
            de::Error::invalid_value(
                de::Unexpected::Other(&format!("{}", err)),
                &"a valid UTS 35 symbol",
            )
        })?;
        match symbol {
            FieldSymbol::Week(w) => Ok(w),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Char(c),
                &"a UTS 35 week symbol",
            )),
        }
    }
}

/// A collection of plural variants of a pattern.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PluralPattern {
    /// The field that 'variants' are predicated on.
    #[cfg_attr(feature = "provider_serde", serde(with = "week_field_serialization"))]
    pivot_field: Week,
    #[cfg_attr(feature = "provider_serde", serde(with = "tuple_vec_map"))]
    variants: Vec<(PluralCategory, Pattern)>,
}

impl PluralPattern {
    /// Creates a new PluralPattern containing the given 'pattern'. The latter must contain either a
    /// week-of-month or week-of-year field.
    pub fn new(
        plural_category: PluralCategory,
        pattern: Pattern,
    ) -> Result<PluralPattern, PatternError> {
        let pivot_field = pattern
            .items()
            .iter()
            .find_map(|pattern_item| match pattern_item {
                PatternItem::Field(Field {
                    symbol: FieldSymbol::Week(w),
                    ..
                }) => Some(w),
                _ => None,
            })
            .ok_or(PatternError::UnsupportedPluralPivot)?;

        let mut plural_pattern = PluralPattern {
            pivot_field: *pivot_field,
            variants: Vec::new(),
        };
        plural_pattern.add_variant(plural_category, pattern);
        Ok(plural_pattern)
    }

    pub fn pivot_field(&self) -> Week {
        self.pivot_field
    }

    pub fn add_variant(&mut self, plural_category: PluralCategory, pattern: Pattern) {
        self.variants.push((plural_category, pattern));
    }

    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern> {
        self.variants.iter().map(|(_, pattern)| pattern)
    }

    pub fn patterns_iter_mut(&mut self) -> impl Iterator<Item = &mut Pattern> {
        self.variants.iter_mut().map(|(_, pattern)| pattern)
    }

    // Returns the pattern with given category, falling back to the pattern with the Other category if not present.
    pub fn get(&self, category: PluralCategory) -> Option<&Pattern> {
        self.variants
            .iter()
            .filter(|(c, _)| *c == category || *c == PluralCategory::Other)
            .reduce(|a, b| if a.0 == PluralCategory::Other { b } else { a })
            .map(|(_, v)| v)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PatternPlurals {
    #[cfg_attr(feature = "provider_serde", serde(rename = "plural_patterns"))]
    MultipleVariants(PluralPattern),
    #[cfg_attr(feature = "provider_serde", serde(rename = "pattern"))]
    SinglePattern(Pattern),
}

enum PatternPluralsIterator<'a, T>
where
    T: Iterator<Item = &'a Pattern>,
{
    MultipleVariants(T),
    SinglePattern(Option<&'a Pattern>),
}

impl<'a, T> Iterator for PatternPluralsIterator<'a, T>
where
    T: Iterator<Item = &'a Pattern>,
{
    type Item = &'a Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PatternPluralsIterator::MultipleVariants(iter) => iter.next(),
            PatternPluralsIterator::SinglePattern(opt_pattern) => opt_pattern.take(),
        }
    }
}

enum PatternPluralsMutIterator<'a, T>
where
    T: Iterator<Item = &'a mut Pattern>,
{
    MultipleVariants(T),
    SinglePattern(Option<&'a mut Pattern>),
}

impl<'a, T> Iterator for PatternPluralsMutIterator<'a, T>
where
    T: Iterator<Item = &'a mut Pattern>,
{
    type Item = &'a mut Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PatternPluralsMutIterator::MultipleVariants(iter) => iter.next(),
            PatternPluralsMutIterator::SinglePattern(opt_pattern) => opt_pattern.take(),
        }
    }
}

impl PatternPlurals {
    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern> {
        match self {
            PatternPlurals::SinglePattern(pattern) => {
                PatternPluralsIterator::SinglePattern(Some(pattern))
            }
            PatternPlurals::MultipleVariants(plural_pattern) => {
                PatternPluralsIterator::MultipleVariants(plural_pattern.patterns_iter())
            }
        }
    }

    pub fn patterns_iter_mut(&mut self) -> impl Iterator<Item = &mut Pattern> {
        match self {
            PatternPlurals::SinglePattern(pattern) => {
                PatternPluralsMutIterator::SinglePattern(Some(pattern))
            }
            PatternPlurals::MultipleVariants(plural_pattern) => {
                PatternPluralsMutIterator::MultipleVariants(plural_pattern.patterns_iter_mut())
            }
        }
    }

    pub fn expect_pattern(self, msg: &str) -> Pattern {
        match self {
            PatternPlurals::SinglePattern(pattern) => pattern,
            _ => panic!("{}", msg),
        }
    }

    pub fn expect_pattern_ref(&self, msg: &str) -> &Pattern {
        match self {
            PatternPlurals::SinglePattern(pattern) => pattern,
            _ => panic!("{}", msg),
        }
    }
}

impl From<Pattern> for PatternPlurals {
    fn from(pattern: Pattern) -> PatternPlurals {
        PatternPlurals::SinglePattern(pattern)
    }
}

impl Default for PatternPlurals {
    fn default() -> Self {
        PatternPlurals::SinglePattern(Pattern::default())
    }
}
