// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures for packed range patterns.

use crate::provider::packed_pattern::GenericPackedPatterns;
use crate::provider::pattern::runtime::{Pattern, PatternULE};
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

/// The date fields that can have a greatest difference.
///
/// Ordered from smallest to largest field difference.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum DateGreatestDifferenceField {
    /// Day (`d`)
    Day = 0,
    /// Month (`M`)
    Month = 1,
    /// Year (`y`)
    Year = 2,
    /// Era (`G`)
    Era = 3,
}

impl DateGreatestDifferenceField {
    /// The maximum value of a `DateGreatestDifferenceField`.
    pub const MAX_VALUE: u8 = Self::Era as u8;
}

/// The time fields that can have a greatest difference.
///
/// Ordered from smallest to largest field difference.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum TimeGreatestDifferenceField {
    /// Minute (`m`)
    Minute = 0,
    /// Hour (`h`/`H`)
    Hour = 1,
    /// Flexible Day Period (`B`)
    DayPeriodB = 2,
    /// Day Period (`a`)
    DayPeriodA = 3,
}

impl TimeGreatestDifferenceField {
    /// The maximum value of a `TimeGreatestDifferenceField`.
    pub const MAX_VALUE: u8 = Self::DayPeriodA as u8;
}

/// A bitset encoding which fields are present in the `GreatestDifference` pattern list.
///
/// The bitset maps greatest-difference field IDs to their presence in the patterns list,
/// ordered from smallest to largest field difference.
///
/// Date and time fields reuse the same bits (0-3) because they are stored in separate
/// range pattern collections (date vs time).
///
/// For date skeleta, the fields are:
/// * Bit 0: Day (`d`)
/// * Bit 1: Month (`M`)
/// * Bit 2: Year (`y`)
/// * Bit 3: Era (`G`)
///
/// For time skeleta, the fields are:
/// * Bit 0: Minute (`m`)
/// * Bit 1: Hour (`h`/`H`)
/// * Bit 2: Flexible Day Period (`B`)
/// * Bit 3: Day Period (`a`)
///
/// This sparse representation allows quickly finding the pattern corresponding to a
/// differing field by counting the number of set bits before it.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[zerovec::make_ule(GreatestDifferenceHeaderULE)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::range_patterns))]
#[zerovec::skip_derive(Ord)]
pub struct GreatestDifferenceHeader(pub u8);

impl GreatestDifferenceHeader {
    /// Creates a new `GreatestDifferenceHeader` from a u8 value.
    pub const fn new(val: u8) -> Self {
        Self(val)
    }

    /// Returns whether the given date field is present in this header.
    pub const fn is_date_field_present(self, field: DateGreatestDifferenceField) -> bool {
        (self.0 & (1 << (field as u8))) != 0
    }

    /// Returns whether the given time field is present in this header.
    pub const fn is_time_field_present(self, field: TimeGreatestDifferenceField) -> bool {
        (self.0 & (1 << (field as u8))) != 0
    }
}

/// A list of patterns that represent the greatest difference format for a given skeleton.
#[derive(Debug, PartialEq, Eq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::range_patterns))]
#[zerovec::make_varule(PatternsByGreatestDifferenceULE)]
#[zerovec::derive(Debug)]
#[zerovec::skip_derive(Ord)]
#[cfg_attr(feature = "serde", zerovec::derive(Deserialize))]
#[cfg_attr(feature = "datagen", zerovec::derive(Serialize))]
pub struct PatternsByGreatestDifference<'data> {
    /// The bitset header marking which greatestDifference fields are present in the patterns list.
    pub header: GreatestDifferenceHeader,
    /// The list of greatestDifference patterns.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, PatternULE>,
}

/// Finds the smallest present bit index >= `requested` in `header` up to `max_value`.
/// Returns `Some((found_bit_index, pattern_index))` or `None`.
fn resolve_fallback(header: u8, requested: u8, max_value: u8) -> Option<(u8, usize)> {
    for i in requested..=max_value {
        if (header & (1 << i)) != 0 {
            let mask = (1 << i) - 1;
            let pattern_index = (header & mask).count_ones() as usize;
            return Some((i, pattern_index));
        }
    }
    None
}

impl<'data> PatternsByGreatestDifference<'data> {
    /// Gets the pattern for the given date field, falling back to larger fields if necessary.
    pub fn get_date_pattern<'a>(
        &'a self,
        field: DateGreatestDifferenceField,
    ) -> Option<Pattern<'a>> {
        let (_, pattern_index) = resolve_fallback(
            self.header.0,
            field as u8,
            DateGreatestDifferenceField::MAX_VALUE,
        )?;
        self.patterns
            .get(pattern_index)
            .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from)
    }

    /// Gets the pattern for the given time field, falling back to larger fields if necessary.
    pub fn get_time_pattern<'a>(
        &'a self,
        field: TimeGreatestDifferenceField,
    ) -> Option<Pattern<'a>> {
        let (_, pattern_index) = resolve_fallback(
            self.header.0,
            field as u8,
            TimeGreatestDifferenceField::MAX_VALUE,
        )?;
        self.patterns
            .get(pattern_index)
            .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from)
    }

    /// Construct from a strictly sorted iterator of (`bit_index`, pattern).
    /// The `bit_index` must be <= 3.
    #[cfg(feature = "datagen")]
    fn try_from_sorted<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (u8, Pattern<'data>)>,
    {
        use alloc::vec::Vec;
        let mut header_val = 0u8;
        let mut patterns = Vec::new();
        let mut last_bit = None;

        for (bit, pattern) in iter {
            if bit > 3 {
                return Err("GreatestDifference bit index must be 0, 1, 2, or 3");
            }
            if let Some(last) = last_bit {
                if bit <= last {
                    return Err(
                        "Iterator must be strictly sorted by bit index and have no duplicates",
                    );
                }
            }
            last_bit = Some(bit);
            header_val |= 1 << bit;
            patterns.push(pattern);
        }

        if patterns.is_empty() {
            return Err("PatternsByGreatestDifference cannot be empty");
        }

        let varzerovec = VarZeroVec::from(patterns.as_slice());

        Ok(Self {
            header: GreatestDifferenceHeader::new(header_val),
            patterns: varzerovec,
        })
    }

    /// Construct from a `BTreeMap` of (`DateGreatestDifferenceField`, pattern).
    #[cfg(feature = "datagen")]
    pub fn try_from_date_patterns(
        map: alloc::collections::BTreeMap<DateGreatestDifferenceField, Pattern<'data>>,
    ) -> Result<Self, &'static str> {
        Self::try_from_sorted(map.into_iter().map(|(f, p)| (f as u8, p)))
    }

    /// Construct from a `BTreeMap` of (`TimeGreatestDifferenceField`, pattern).
    #[cfg(feature = "datagen")]
    pub fn try_from_time_patterns(
        map: alloc::collections::BTreeMap<TimeGreatestDifferenceField, Pattern<'data>>,
    ) -> Result<Self, &'static str> {
        Self::try_from_sorted(map.into_iter().map(|(f, p)| (f as u8, p)))
    }
}

/// The main data structure for packed range/interval patterns.
pub type PackedRangePatterns<'data> = GenericPackedPatterns<'data, PatternsByGreatestDifferenceULE>;

icu_provider::data_struct!(
    PackedRangePatterns<'_>,
    #[cfg(feature = "datagen")]
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::pattern::reference;

    #[test]
    fn test_get_pattern() {
        let pattern1 = "d"
            .parse::<reference::Pattern>()
            .unwrap()
            .to_runtime_pattern();
        let pattern2 = "y"
            .parse::<reference::Pattern>()
            .unwrap()
            .to_runtime_pattern();

        let patterns = vec![pattern1.clone(), pattern2.clone()];
        let varzerovec = VarZeroVec::from(patterns.as_slice());

        // Header with Day (bit 0) and Year (bit 2) present.
        let header = GreatestDifferenceHeader(1 | 4);

        let pgd = PatternsByGreatestDifference {
            header,
            patterns: varzerovec,
        };

        assert!(
            pgd.header
                .is_date_field_present(DateGreatestDifferenceField::Day)
        );
        assert!(
            !pgd.header
                .is_date_field_present(DateGreatestDifferenceField::Month)
        );
        assert!(
            pgd.header
                .is_date_field_present(DateGreatestDifferenceField::Year)
        );

        // Day difference should return Day pattern.
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Day),
            Some(pattern1.clone())
        );
        // Month difference should fall back to Year pattern (since Month is absent but Year is present).
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Month),
            Some(pattern2.clone())
        );
        // Year difference should return Year pattern.
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Year),
            Some(pattern2.clone())
        );
        // Era difference should return None (since Era is absent and no larger field is present).
        assert_eq!(pgd.get_date_pattern(DateGreatestDifferenceField::Era), None);
    }

    #[test]
    #[cfg(feature = "datagen")]
    fn test_try_from_patterns() {
        let pattern_d = "d"
            .parse::<reference::Pattern>()
            .unwrap()
            .to_runtime_pattern();
        let pattern_y = "y"
            .parse::<reference::Pattern>()
            .unwrap()
            .to_runtime_pattern();

        // Valid date patterns
        let pgd = PatternsByGreatestDifference::try_from_date_patterns(
            alloc::collections::BTreeMap::from([
                (
                    DateGreatestDifferenceField::Day,
                    zerofrom::ZeroFrom::zero_from(&pattern_d),
                ),
                (
                    DateGreatestDifferenceField::Year,
                    zerofrom::ZeroFrom::zero_from(&pattern_y),
                ),
            ]),
        )
        .unwrap();

        assert_eq!(pgd.header.0, 1 | 4);
        assert_eq!(pgd.patterns.len(), 2);
        assert_eq!(
            pgd.patterns
                .get(0)
                .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from),
            Some(pattern_d.clone())
        );
        assert_eq!(
            pgd.patterns
                .get(1)
                .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from),
            Some(pattern_y.clone())
        );

        // Unsorted input in BTreeMap::from is automatically sorted
        let pgd2 = PatternsByGreatestDifference::try_from_date_patterns(
            alloc::collections::BTreeMap::from([
                (
                    DateGreatestDifferenceField::Year,
                    zerofrom::ZeroFrom::zero_from(&pattern_y),
                ),
                (
                    DateGreatestDifferenceField::Day,
                    zerofrom::ZeroFrom::zero_from(&pattern_d),
                ),
            ]),
        )
        .unwrap();
        assert_eq!(pgd2.header.0, 1 | 4);
        assert_eq!(
            pgd2.patterns
                .get(0)
                .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from),
            Some(pattern_d.clone())
        );

        // Empty
        let err = PatternsByGreatestDifference::try_from_date_patterns(
            alloc::collections::BTreeMap::new(),
        );
        assert!(err.is_err());
    }
}
