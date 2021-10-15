// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{PatternError, PatternItem, TimeGranularity},
    Parser,
};
use crate::error::DateTimeFormatError as Error;
use crate::{
    date::{DateTimeInput, LocalizedDateTimeInput},
    fields::{Field, FieldSymbol, Week},
};
use alloc::fmt::{self, Write};
#[cfg(feature = "provider_serde")]
use alloc::format;
use alloc::string::String;
#[cfg(feature = "provider_serde")]
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use either::Either;
use icu_plurals::{PluralCategory, PluralRules};
use litemap::LiteMap;

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
    use crate::fields::Week;
    use alloc::format;
    use core::convert::TryInto;
    use serde::{self, de, Deserialize};

    pub fn serialize<S>(week: &Week, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_char((*week).into())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Week, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        char::deserialize(deserializer)?.try_into().map_err(|err| {
            de::Error::invalid_value(
                de::Unexpected::Other(&format!("{}", err)),
                &"a valid UTS 35 week symbol",
            )
        })
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
    variants: LiteMap<PluralCategory, Pattern>,
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
            variants: LiteMap::new(),
        };
        plural_pattern.add_variant(plural_category, pattern);
        Ok(plural_pattern)
    }

    /// Returns which week field determines the [icu_plurals::PluralCategory] used to select a pattern variant for a given date.
    pub fn pivot_field(&self) -> Week {
        self.pivot_field
    }

    /// Adds `pattern` associated with `plural_category` to this pattern collection.
    pub fn add_variant(&mut self, plural_category: PluralCategory, pattern: Pattern) {
        self.variants.insert(plural_category, pattern);
    }

    /// Returns an iterator over all of this collection's patterns.
    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern> {
        self.variants.iter_values()
    }

    /// Returns a mutable iterator over all of this collection's patterns.
    pub fn patterns_iter_mut(&mut self) -> impl Iterator<Item = &mut Pattern> {
        self.variants.iter_mut().map(|(_, pattern)| pattern)
    }

    // Returns the pattern with given category, falling back to the pattern with the Other category if not present.
    fn get(&self, category: PluralCategory) -> &Pattern {
        self.variants
            .get(&category)
            .or_else(|| self.variants.get(&PluralCategory::Other))
            .expect("Category 'other' must be present.")
    }

    /// Removes any patterns that are redundant with [icu_plurals::PluralCategory::Other].
    ///
    /// Panics if no [PluralCategory::Other] pattern is present.
    fn normalize(&mut self) {
        let other_pattern = self
            .variants
            .get(&PluralCategory::Other)
            .expect("PluralPatterns must have an PluralCategory::Other pattern");
        let duplicates: Vec<PluralCategory> = self
            .variants
            .iter()
            .filter_map(|(c, p)| {
                if c != &PluralCategory::Other && p == other_pattern {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect();
        for category in duplicates {
            self.variants.remove(&category);
        }
    }
}

#[test]
fn build_plural_pattern() {
    let red_pattern = Pattern::from_bytes("'red' w").unwrap();
    let blue_pattern = Pattern::from_bytes("'blue' w").unwrap();
    let mut patterns = PluralPattern::new(PluralCategory::Zero, red_pattern.clone())
        .expect("PluralPattern::new failed");
    patterns.add_variant(PluralCategory::One, blue_pattern.clone());
    patterns.add_variant(PluralCategory::Two, red_pattern.clone());
    patterns.add_variant(PluralCategory::Other, blue_pattern.clone());
    patterns.add_variant(PluralCategory::Few, red_pattern.clone());
    patterns.add_variant(PluralCategory::Many, blue_pattern.clone());
    patterns.normalize();

    assert_eq!(patterns.pivot_field, Week::WeekOfYear);
    assert_eq!(
        patterns.variants.into_tuple_vec(),
        vec![
            (PluralCategory::Zero, red_pattern.clone()),
            (PluralCategory::Two, red_pattern.clone()),
            (PluralCategory::Few, red_pattern),
            (PluralCategory::Other, blue_pattern)
        ]
    )
}

/// Either a single Pattern or a collection of pattern when there are plural variants.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PatternPlurals {
    /// A collection of pattern variants for when plurals differ.
    MultipleVariants(PluralPattern),
    /// A single pattern.
    SinglePattern(Pattern),
}

impl PatternPlurals {
    // Removes redundant patterns & transforms singleton [PatternPlurals::MultipleVariants] into a [PatternPlurals::SinglePattern].
    pub fn normalize(&mut self) {
        if let PatternPlurals::MultipleVariants(patterns) = self {
            patterns.normalize();
            if patterns.variants.len() == 1 {
                let (_, pattern) = core::mem::take(&mut patterns.variants)
                    .into_tuple_vec()
                    .pop()
                    .unwrap();
                *self = PatternPlurals::SinglePattern(pattern);
            }
        }
    }

    /// Returns an iterator over all of this collection's patterns.
    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern> {
        match self {
            PatternPlurals::SinglePattern(pattern) => Either::Left(core::iter::once(pattern)),
            PatternPlurals::MultipleVariants(plural_pattern) => {
                Either::Right(plural_pattern.patterns_iter())
            }
        }
    }

    /// Returns a mutable iterator over all of this collection's patterns.
    pub fn patterns_iter_mut(&mut self) -> impl Iterator<Item = &mut Pattern> {
        match self {
            PatternPlurals::SinglePattern(pattern) => Either::Left(core::iter::once(pattern)),
            PatternPlurals::MultipleVariants(plural_pattern) => {
                Either::Right(plural_pattern.patterns_iter_mut())
            }
        }
    }

    /// Returns the contained [`SinglePattern`] pattern, consuming `self`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`MultipleVariants`] with a custom panic message provided by
    /// `msg`.
    pub fn expect_pattern(self, msg: &str) -> Pattern {
        match self {
            PatternPlurals::SinglePattern(pattern) => pattern,
            _ => panic!("expect_pattern failed: {}", msg),
        }
    }

    /// Returns a reference to the contained [`SinglePattern`] pattern, consuming `self`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`MultipleVariants`] with a custom panic message provided by
    /// `msg`.
    pub fn expect_pattern_ref(&self, msg: &str) -> &Pattern {
        match self {
            PatternPlurals::SinglePattern(pattern) => pattern,
            _ => panic!("expect_pattern failed: {}", msg),
        }
    }

    // Returns the pattern to use according to loc_datetime and ordinal_rules.
    pub fn select<T>(
        &self,
        loc_datetime: &impl LocalizedDateTimeInput<T>,
        ordinal_rules: Option<&PluralRules>,
    ) -> Result<&Pattern, Error>
    where
        T: DateTimeInput,
    {
        match self {
            PatternPlurals::SinglePattern(pattern) => Ok(pattern),
            PatternPlurals::MultipleVariants(plural_pattern) => {
                let week_number = match plural_pattern.pivot_field() {
                    Week::WeekOfMonth => loc_datetime.week_of_month().0,
                    Week::WeekOfYear => loc_datetime.week_of_year()?.0,
                };
                let category = ordinal_rules
                    .expect("ordinal_rules must be set with PatternPlurals::MultipleVariants")
                    .select(week_number);
                Ok(plural_pattern.get(category))
            }
        }
    }
}

#[test]
fn normalize_pattern_plurals_normalizes_plural_patterns() {
    let red_pattern = Pattern::from_bytes("'red' w").unwrap();
    let blue_pattern = Pattern::from_bytes("'blue' w").unwrap();
    let mut patterns = PluralPattern::new(PluralCategory::Zero, red_pattern.clone())
        .expect("PluralPattern::new failed");
    patterns.add_variant(PluralCategory::One, blue_pattern.clone());
    patterns.add_variant(PluralCategory::Two, red_pattern);
    patterns.add_variant(PluralCategory::Other, blue_pattern);
    let mut normalized_patterns = patterns.clone();
    normalized_patterns.normalize();
    let mut plural_patterns: PatternPlurals = PatternPlurals::MultipleVariants(patterns);

    plural_patterns.normalize();

    assert_eq!(
        plural_patterns,
        PatternPlurals::MultipleVariants(normalized_patterns)
    );
}

#[test]
fn normalize_pattern_plurals_switches_singletons_to_single_pattern() {
    let pattern = Pattern::from_bytes("'red' w").unwrap();
    let patterns = PluralPattern::new(PluralCategory::Other, pattern.clone())
        .expect("PluralPattern::new failed");
    let mut plural_patterns: PatternPlurals = PatternPlurals::MultipleVariants(patterns);

    plural_patterns.normalize();

    assert_eq!(plural_patterns, PatternPlurals::SinglePattern(pattern));
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
