// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures for packed range patterns.

#[cfg(feature = "datagen")]
use crate::provider::packed_pattern::GenericUnpackedPatterns;
use crate::provider::packed_pattern::{GenericPackedPatterns, PackedPatternsBuilderHelper};
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

    /// Parses a symbol into a `DateGreatestDifferenceField`.
    #[cfg(feature = "datagen")]
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "d" => Some(Self::Day),
            "M" => Some(Self::Month),
            "y" => Some(Self::Year),
            "G" => Some(Self::Era),
            _ => None,
        }
    }
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

    /// Parses a symbol into a `TimeGreatestDifferenceField`.
    #[cfg(feature = "datagen")]
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "m" => Some(Self::Minute),
            "h" | "H" => Some(Self::Hour),
            "B" => Some(Self::DayPeriodB),
            "a" => Some(Self::DayPeriodA),
            _ => None,
        }
    }
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
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
///
/// ### Fallback Behavior
/// At runtime, if a pattern for a specific greatest difference field is requested but not present
/// in this struct, the resolver will fall back to the next larger present field (e.g., Day -> Month
/// -> Year -> Era).
///
/// ### Datagen Deduplication
/// To minimize data size, datagen leverages this fallback behavior to omit redundant patterns.
/// If a smaller field (e.g., Day) has the exact same pattern as a larger field (e.g., Month),
/// the smaller field is marked as absent in the header and its pattern is omitted. At runtime,
/// requesting the smaller field will correctly fall back to the larger field and retrieve the
/// identical pattern.
#[derive(Debug, PartialEq, Eq, Clone, yoke::Yokeable, zerofrom::ZeroFrom, Default)]
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

    /// Construct from an iterator of (`bit_index`, pattern).
    ///
    /// The `bit_index` must be <= 3.
    ///
    /// This function automatically sorts the inputs by `bit_index` and performs
    /// fallback-based deduplication: if a pattern for a smaller field is identical
    /// to the pattern for the next larger present field, the smaller field's pattern
    /// is omitted, as it will naturally fall back to the larger field at runtime.
    #[cfg(feature = "datagen")]
    pub fn try_from_patterns<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (u8, Pattern<'data>)>,
    {
        use alloc::vec::Vec;

        // 1. Collect and sort the input.
        let mut original_input = iter.into_iter().collect::<Vec<_>>();
        original_input.sort_by_key(|(bit, _)| *bit);

        if original_input.is_empty() {
            return Err("PatternsByGreatestDifference cannot be empty");
        }

        // Validate bit range and check for duplicates.
        let mut prev_bit = None;
        for &(bit, _) in original_input.iter() {
            if bit > 3 {
                return Err("GreatestDifference bit index must be 0, 1, 2, or 3");
            }
            if prev_bit == Some(bit) {
                return Err("Duplicate greatest difference fields are not allowed");
            }
            prev_bit = Some(bit);
        }

        // 2. Deduplicate patterns using a simple one-pass filter.
        // We iterate from smallest to largest field. We keep a pattern only if
        // it is different from the next present pattern. The last pattern is always kept.
        // This leverages the fallback behavior in `resolve_fallback`.
        let mut minimized_patterns = Vec::new();
        let mut header_val = 0u8;

        let mut it = original_input.into_iter().peekable();
        while let Some((bit, pattern)) = it.next() {
            let keep = if let Some((_, next_pattern)) = it.peek() {
                &pattern != next_pattern
            } else {
                true // Always keep the last element
            };

            if keep {
                header_val |= 1 << bit;
                minimized_patterns.push(pattern);
            }
        }

        let varzerovec = VarZeroVec::from(minimized_patterns.as_slice());

        Ok(Self {
            header: GreatestDifferenceHeader::new(header_val),
            patterns: varzerovec,
        })
    }

    /// Construct from an iterator of (`DateGreatestDifferenceField`, pattern).
    #[cfg(feature = "datagen")]
    pub fn try_from_date_patterns<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (DateGreatestDifferenceField, Pattern<'data>)>,
    {
        Self::try_from_patterns(iter.into_iter().map(|(f, p)| (f as u8, p)))
    }

    /// Construct from an iterator of (`TimeGreatestDifferenceField`, pattern).
    #[cfg(feature = "datagen")]
    pub fn try_from_time_patterns<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (TimeGreatestDifferenceField, Pattern<'data>)>,
    {
        Self::try_from_patterns(iter.into_iter().map(|(f, p)| (f as u8, p)))
    }
}

impl PackedPatternsBuilderHelper for PatternsByGreatestDifferenceULE {
    type Unpacked<'a> = PatternsByGreatestDifference<'a>;
    fn pack(elements: &[Self::Unpacked<'_>]) -> VarZeroVec<'static, Self> {
        VarZeroVec::from(elements)
    }

    #[cfg(feature = "datagen")]
    fn unpack<'a>(
        packed: &'a GenericPackedPatterns<Self>,
    ) -> GenericUnpackedPatterns<Self::Unpacked<'a>> {
        use crate::provider::packed_pattern::constants;
        let variant_indices = packed.variant_indices();
        let elements = packed
            .elements
            .iter()
            .map(|pgd_ule| {
                <PatternsByGreatestDifference<'a> as zerofrom::ZeroFrom<
                    PatternsByGreatestDifferenceULE,
                >>::zero_from(pgd_ule)
            })
            .collect();
        GenericUnpackedPatterns {
            has_explicit_medium: (packed.header & constants::M_DIFFERS) != 0,
            has_explicit_short: (packed.header & constants::S_DIFFERS) != 0,
            variant_indices,
            elements,
        }
    }
}

#[cfg(feature = "datagen")]
impl<'a>
    crate::provider::packed_pattern::GenericPackedPatternsBuilder<PatternsByGreatestDifference<'a>>
{
    /// Builds a packed range pattern representation from the builder.
    pub fn build(self) -> PackedRangePatterns<'static> {
        let mut builder = self;
        let unpacked = builder.build_unpacked();
        unpacked.build::<PatternsByGreatestDifferenceULE>()
    }
}

/// The main data structure for packed range/interval patterns.
pub type PackedRangePatterns<'data> = GenericPackedPatterns<'data, PatternsByGreatestDifferenceULE>;

icu_provider::data_struct!(
    PackedRangePatterns<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `DatetimePatternsRangeGlueV1`
    DatetimePatternsRangeGlueV1,
    crate::provider::semantic_skeletons::GluePattern<'static>
);

icu_provider::data_marker!(
    /// `DatetimePatternsRangeTimeV1`
    DatetimePatternsRangeTimeV1,
    PackedRangePatterns<'static>
);

icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateBuddhistV1`
    DatetimePatternsRangeDateBuddhistV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateChineseV1`
    DatetimePatternsRangeDateChineseV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateCopticV1`
    DatetimePatternsRangeDateCopticV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateDangiV1`
    DatetimePatternsRangeDateDangiV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateEthiopianV1`
    DatetimePatternsRangeDateEthiopianV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateGregorianV1`
    DatetimePatternsRangeDateGregorianV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateHebrewV1`
    DatetimePatternsRangeDateHebrewV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateIndianV1`
    DatetimePatternsRangeDateIndianV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateHijriV1`
    DatetimePatternsRangeDateHijriV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateJapaneseV1`
    DatetimePatternsRangeDateJapaneseV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDatePersianV1`
    DatetimePatternsRangeDatePersianV1,
    PackedRangePatterns<'static>
);
icu_provider::data_marker!(
    /// `DatetimePatternsRangeDateRocV1`
    DatetimePatternsRangeDateRocV1,
    PackedRangePatterns<'static>
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

#[cfg(feature = "serde")]
mod _serde {
    use super::*;
    use crate::provider::packed_pattern::_serde::PackedPatternsSerdeHelper;
    use alloc::vec::Vec;
    use serde::Deserialize;
    #[cfg(feature = "datagen")]
    use serde::Serialize;

    #[derive(Debug, Clone, Default, Deserialize)]
    #[cfg_attr(feature = "datagen", derive(Serialize))]
    pub struct PatternsByGreatestDifferenceHuman {
        pub header: GreatestDifferenceHeader,
        pub patterns: Vec<crate::provider::pattern::reference::Pattern>,
    }

    impl PackedPatternsSerdeHelper for PatternsByGreatestDifferenceULE {
        type Human = PatternsByGreatestDifferenceHuman;

        fn human_to_unpacked_element<'a>(human: &'a Self::Human) -> Self::Unpacked<'a> {
            let patterns: Vec<Pattern<'a>> = human
                .patterns
                .iter()
                .map(|p| p.to_runtime_pattern())
                .collect();
            PatternsByGreatestDifference {
                header: human.header,
                patterns: VarZeroVec::from(&patterns),
            }
        }

        #[cfg(feature = "datagen")]
        fn unpacked_element_to_human<S: serde::Serializer>(
            element: Self::Unpacked<'_>,
        ) -> Result<Self::Human, S::Error> {
            let patterns = element
                .patterns
                .iter()
                .map(|p| {
                    let runtime_pattern = <Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from(p);
                    crate::provider::pattern::reference::Pattern::from(&runtime_pattern)
                })
                .collect();
            Ok(PatternsByGreatestDifferenceHuman {
                header: element.header,
                patterns,
            })
        }
    }

    impl<'de, 'data> Deserialize<'de> for PackedRangePatterns<'data>
    where
        'de: 'data,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            GenericPackedPatterns::deserialize_impl(deserializer)
        }
    }

    #[cfg(feature = "datagen")]
    impl Serialize for PackedRangePatterns<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.serialize_impl(serializer)
        }
    }
}

#[cfg(test)]
mod try_from_patterns_tests {
    use super::*;
    use crate::provider::pattern::runtime::Pattern;
    use core::str::FromStr;

    fn get_pattern<'a>(pgd: &'a PatternsByGreatestDifference<'_>, idx: usize) -> Pattern<'a> {
        let ule = pgd.patterns.get(idx).unwrap();
        <Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from(ule)
    }

    #[test]
    fn test_try_from_patterns_dedup() {
        let pat_a = Pattern::from_str("y").unwrap();
        let pat_b = Pattern::from_str("m").unwrap();

        // 1. All different -> no dedup
        let input = vec![(0, pat_a.clone()), (1, pat_b.clone()), (2, pat_a.clone())];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        assert_eq!(pgd.header.0, 0b111); // Day, Month, Year present
        assert_eq!(pgd.patterns.len(), 3);
        assert_eq!(get_pattern(&pgd, 0), pat_a);
        assert_eq!(get_pattern(&pgd, 1), pat_b);
        assert_eq!(get_pattern(&pgd, 2), pat_a);

        // 2. All identical -> dedup to 1 (at the largest field, Year)
        let input = vec![(0, pat_a.clone()), (1, pat_a.clone()), (2, pat_a.clone())];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        assert_eq!(pgd.header.0, 0b100); // Only Year (2) present
        assert_eq!(pgd.patterns.len(), 1);
        assert_eq!(get_pattern(&pgd, 0), pat_a);

        // 3. Day and Month identical, Year different -> dedup Day to Month
        let input = vec![(0, pat_a.clone()), (1, pat_a.clone()), (2, pat_b.clone())];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        assert_eq!(pgd.header.0, 0b110); // Month (1) and Year (2) present
        assert_eq!(pgd.patterns.len(), 2);
        assert_eq!(get_pattern(&pgd, 0), pat_a);
        assert_eq!(get_pattern(&pgd, 1), pat_b);

        // 4. Day and Year identical, Month different -> no dedup (since Month is in between and different)
        let input = vec![(0, pat_a.clone()), (1, pat_b.clone()), (2, pat_a.clone())];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        assert_eq!(pgd.header.0, 0b111); // All present
        assert_eq!(pgd.patterns.len(), 3);

        // 5. Unsorted input -> should be sorted and deduped correctly
        let input = vec![(2, pat_a.clone()), (0, pat_a.clone()), (1, pat_a.clone())];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        assert_eq!(pgd.header.0, 0b100); // Only Year (2) present
        assert_eq!(pgd.patterns.len(), 1);

        // 6. Duplicate keys -> error
        let input = vec![(0, pat_a.clone()), (0, pat_b.clone())];
        assert!(PatternsByGreatestDifference::try_from_patterns(input).is_err());

        // 7. Out of bound keys -> error
        let input = vec![(4, pat_a.clone())];
        assert!(PatternsByGreatestDifference::try_from_patterns(input).is_err());
    }
}
