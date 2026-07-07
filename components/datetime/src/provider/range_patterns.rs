// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures for packed range patterns.

#[cfg(feature = "datagen")]
use crate::provider::packed_pattern::GenericUnpackedPatterns;
use crate::provider::packed_pattern::{GenericPackedPatterns, PackedPatternsBuilderHelper};
use crate::provider::pattern::runtime::PatternBorrowed;
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

/// The structure of a range pattern for a specific field.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RangeStructure {
    /// No pattern is present for this field. (0 patterns)
    Absent = 0b00,
    /// Symmetric pattern: `[sub] [shared_glue] [sub]`. (1 pattern)
    Symmetric = 0b01,
    /// Full Range pattern: the entire range pattern is stored. (1 pattern)
    FullRange = 0b10,
}

impl RangeStructure {
    /// Creates a `RangeStructure` from a 2-bit value.
    pub const fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b00 => Self::Absent,
            0b01 => Self::Symmetric,
            0b10 => Self::FullRange,
            _ => Self::Absent, // fallback, mathematically impossible
        }
    }

    /// Returns the number of patterns stored for this structure.
    pub const fn num_patterns(self) -> usize {
        match self {
            Self::Absent => 0,
            Self::Symmetric => 1,
            Self::FullRange => 1,
        }
    }
}

/// Runtime information about a range pattern, including its structure and sub-patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangePatternInfo<'a> {
    /// Symmetric pattern: `[sub] [glue] [sub]`. (1 pattern)
    ///
    /// The pattern is formed by formatting the start pattern, then the shared
    /// fallback glue, then the end pattern.
    ///
    /// **Example**: `d – d` (English Day range) with fallback glue ` – `.
    /// Decomposes to `Symmetric(d)`.
    Symmetric(Pattern<'a>),
    /// Full Range pattern: the entire range pattern is stored. (1 pattern)
    ///
    /// **Example**: `y M d – M d` (English Month range, e.g., `2020 Nov 5 – Dec 6`)
    /// or `HH至HH` (Chinese Hour range).
    FullRange(Pattern<'a>),
}

impl<'a> RangePatternInfo<'a> {
    /// Gets the inner pattern.
    pub fn pattern(&self) -> &Pattern<'a> {
        match self {
            Self::Symmetric(p) => p,
            Self::FullRange(p) => p,
        }
    }
}

/// Fully borrowed version of [`RangePatternInfo`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RangePatternInfoBorrowed<'a> {
    /// Symmetric pattern: `[sub] [glue] [sub]`, where the sub-pattern is shared.
    Symmetric(PatternBorrowed<'a>),
    /// Full Range pattern: the entire range pattern is stored.
    FullRange(PatternBorrowed<'a>),
}

impl<'a> From<RangePatternInfoBorrowed<'a>> for RangePatternInfo<'a> {
    fn from(other: RangePatternInfoBorrowed<'a>) -> Self {
        match other {
            RangePatternInfoBorrowed::Symmetric(p) => RangePatternInfo::Symmetric(p.as_pattern()),
            RangePatternInfoBorrowed::FullRange(p) => RangePatternInfo::FullRange(p.as_pattern()),
        }
    }
}

impl<'a> zerofrom::ZeroFrom<'a, RangePatternInfoBorrowed<'a>> for RangePatternInfo<'a> {
    fn zero_from(other: &'a RangePatternInfoBorrowed<'a>) -> Self {
        Self::from(*other)
    }
}

/// A bitset encoding which fields are present in the `GreatestDifference` pattern list
/// and their range structure.
///
/// Date and time fields reuse the same bits (0-7) because they are stored in separate
/// range pattern collections (date vs time).
///
/// Each of the 4 fields uses 2 bits to encode its `RangeStructure`, going from smallest field to largest:
/// * Bits 0-1: Field 0 (Day/Minute)
/// * Bits 2-3: Field 1 (Month/Hour)
/// * Bits 4-5: Field 2 (Year/Flexible Day Period)
/// * Bits 6-7: Field 3 (Era/Day Period)
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

    /// Get the range structure for a specific field index (0-3).
    pub const fn get_state(self, field_idx: u8) -> RangeStructure {
        let shift = field_idx * 2;
        let bits = (self.0 >> shift) & 0b11;
        RangeStructure::from_bits(bits)
    }

    /// Set the range structure for a specific field index (0-3).
    /// Used during datagen.
    #[cfg(any(feature = "datagen", test))]
    pub fn set_state(&mut self, field_idx: u8, state: RangeStructure) {
        let shift = field_idx * 2;
        let mask = !(0b11 << shift);
        self.0 = (self.0 & mask) | ((state as u8) << shift);
    }

    /// Returns whether the given date field is present in this header.
    pub const fn is_date_field_present(self, field: DateGreatestDifferenceField) -> bool {
        !matches!(self.get_state(field as u8), RangeStructure::Absent)
    }

    /// Returns whether the given time field is present in this header.
    pub const fn is_time_field_present(self, field: TimeGreatestDifferenceField) -> bool {
        !matches!(self.get_state(field as u8), RangeStructure::Absent)
    }

    /// Calculate the starting index in the flat patterns list for a given field index (0-3).
    pub const fn get_pattern_index(self, field_idx: u8) -> usize {
        let mask = (1u16 << (field_idx * 2)) - 1;
        let masked = (self.0 as u16) & mask;
        masked.count_ones() as usize
    }

    /// Calculate the total number of patterns stored under this header.
    pub const fn total_patterns(self) -> usize {
        self.get_pattern_index(4)
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

/// Finds the smallest present field index >= `requested` in `header` up to `max_value`.
/// A field is present if its state is not `Absent`.
fn resolve_fallback(header: GreatestDifferenceHeader, requested: u8, max_value: u8) -> Option<u8> {
    for i in requested..=max_value {
        match header.get_state(i) {
            RangeStructure::Absent => {}
            _ => return Some(i),
        }
    }
    None
}

impl PatternsByGreatestDifferenceULE {
    fn get_pattern_internal<'a>(&'a self, resolved_field_idx: u8) -> RangePatternInfoBorrowed<'a> {
        use zerovec::ule::AsULE;
        let header = <GreatestDifferenceHeader as AsULE>::from_unaligned(self.header);
        let state = header.get_state(resolved_field_idx);
        let start_idx = header.get_pattern_index(resolved_field_idx);

        let get_pat = |offset| {
            self.patterns
                .get(start_idx + offset)
                .map(<PatternBorrowed as zerofrom::ZeroFrom<PatternULE>>::zero_from)
                .unwrap_or(PatternBorrowed::DEFAULT)
        };

        match state {
            RangeStructure::Absent => RangePatternInfoBorrowed::Symmetric(PatternBorrowed::DEFAULT),
            RangeStructure::Symmetric => RangePatternInfoBorrowed::Symmetric(get_pat(0)),
            RangeStructure::FullRange => RangePatternInfoBorrowed::FullRange(get_pat(0)),
        }
    }

    /// Gets the pattern info for the given date field, falling back to larger fields if necessary.
    pub fn get_date_pattern<'a>(
        &'a self,
        field: DateGreatestDifferenceField,
    ) -> Option<RangePatternInfoBorrowed<'a>> {
        use zerovec::ule::AsULE;
        let header = <GreatestDifferenceHeader as AsULE>::from_unaligned(self.header);
        let resolved_field_idx =
            resolve_fallback(header, field as u8, DateGreatestDifferenceField::MAX_VALUE)?;
        Some(self.get_pattern_internal(resolved_field_idx))
    }

    /// Gets the pattern info for the given time field, falling back to larger fields if necessary.
    pub fn get_time_pattern<'a>(
        &'a self,
        field: TimeGreatestDifferenceField,
    ) -> Option<RangePatternInfoBorrowed<'a>> {
        use zerovec::ule::AsULE;
        let header = <GreatestDifferenceHeader as AsULE>::from_unaligned(self.header);
        let resolved_field_idx =
            resolve_fallback(header, field as u8, TimeGreatestDifferenceField::MAX_VALUE)?;
        Some(self.get_pattern_internal(resolved_field_idx))
    }
}

impl<'data> PatternsByGreatestDifference<'data> {
    /// Construct from an iterator of (`bit_index`, `pattern_info`).
    ///
    /// The `bit_index` must be <= 3.
    ///
    /// This function automatically sorts the inputs by `bit_index` and performs
    /// fallback-based deduplication: if a pattern info for a smaller field is identical
    /// to the pattern info for the next larger present field, the smaller field's pattern
    /// is omitted, as it will naturally fall back to the larger field at runtime.
    #[cfg(any(feature = "datagen", test))]
    pub fn try_from_patterns<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (u8, RangePatternInfo<'data>)>,
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

        // 2. Deduplicate patterns and pack them in a single pass.
        // We iterate from smallest to largest field. We keep a pattern info only if
        // it is different from the next present pattern info. The last pattern info is always kept.
        // This leverages the fallback behavior in `resolve_fallback`.
        let mut flat_patterns = Vec::new();
        let mut header = GreatestDifferenceHeader::new(0);

        let mut it = original_input.into_iter().peekable();
        while let Some((bit, info)) = it.next() {
            let keep = if let Some((_, next_info)) = it.peek() {
                &info != next_info
            } else {
                true // Always keep the last element
            };

            if keep {
                let state = match info {
                    RangePatternInfo::Symmetric(pat) => {
                        flat_patterns.push(pat);
                        RangeStructure::Symmetric
                    }
                    RangePatternInfo::FullRange(pat) => {
                        flat_patterns.push(pat);
                        RangeStructure::FullRange
                    }
                };
                header.set_state(bit, state);
            }
        }

        let varzerovec = VarZeroVec::from(flat_patterns.as_slice());

        Ok(Self {
            header,
            patterns: varzerovec,
        })
    }

    /// Construct from an iterator of (`DateGreatestDifferenceField`, `pattern_info`).
    #[cfg(any(feature = "datagen", test))]
    pub fn try_from_date_patterns<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (DateGreatestDifferenceField, RangePatternInfo<'data>)>,
    {
        Self::try_from_patterns(iter.into_iter().map(|(f, p)| (f as u8, p)))
    }

    /// Construct from an iterator of (`TimeGreatestDifferenceField`, `pattern_info`).
    #[cfg(any(feature = "datagen", test))]
    pub fn try_from_time_patterns<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (TimeGreatestDifferenceField, RangePatternInfo<'data>)>,
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

        // Header with Day (Field 0) Symmetric (01) and Year (Field 2) Symmetric (01) present.
        // Bits: 00 01 00 01 = 17 (0x11)
        let header = GreatestDifferenceHeader(17);

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

        let pgd_ule_box = zerovec::ule::encode_varule_to_box(&pgd);
        let pgd: &PatternsByGreatestDifferenceULE = &pgd_ule_box;

        let info_a = RangePatternInfo::Symmetric(pattern1);
        let info_b = RangePatternInfo::Symmetric(pattern2);

        // Day difference should return Day pattern.
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Day)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );
        // Month difference should fall back to Year pattern (since Month is absent but Year is present).
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Month)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_b)
        );
        // Year difference should return Year pattern.
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Year)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_b)
        );
        // Era difference should return None (since Era is absent and no larger field is present).
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Era)
                .as_ref(),
            None
        );
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

        let info_d = RangePatternInfo::Symmetric(pattern_d);
        let info_y = RangePatternInfo::Symmetric(pattern_y);

        // Valid date patterns
        let pgd = PatternsByGreatestDifference::try_from_date_patterns(
            alloc::collections::BTreeMap::from([
                (DateGreatestDifferenceField::Day, info_d.clone()),
                (DateGreatestDifferenceField::Year, info_y.clone()),
            ]),
        )
        .unwrap();

        // Header: Day (01), Month (00), Year (01) -> 17
        assert_eq!(pgd.header.0, 17);
        assert_eq!(pgd.patterns.len(), 2);
        assert_eq!(
            pgd.patterns
                .get(0)
                .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from)
                .as_ref(),
            Some(info_d.pattern())
        );
        assert_eq!(
            pgd.patterns
                .get(1)
                .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from)
                .as_ref(),
            Some(info_y.pattern())
        );

        // Unsorted input in BTreeMap::from is automatically sorted
        let pgd2 = PatternsByGreatestDifference::try_from_date_patterns(
            alloc::collections::BTreeMap::from([
                (DateGreatestDifferenceField::Year, info_y),
                (DateGreatestDifferenceField::Day, info_d.clone()),
            ]),
        )
        .unwrap();
        assert_eq!(pgd2.header.0, 17);
        assert_eq!(
            pgd2.patterns
                .get(0)
                .map(<Pattern as zerofrom::ZeroFrom<PatternULE>>::zero_from)
                .as_ref(),
            Some(info_d.pattern())
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

    #[test]
    fn test_try_from_patterns_dedup() {
        let pat_a = Pattern::from_str("y").unwrap();
        let pat_b = Pattern::from_str("m").unwrap();

        let info_a = RangePatternInfo::Symmetric(pat_a.clone());
        let info_b = RangePatternInfo::Symmetric(pat_b);
        let info_c = RangePatternInfo::FullRange(pat_a);

        // 1. All different -> no dedup
        let input = vec![
            (0, info_a.clone()),
            (1, info_b.clone()),
            (2, info_c.clone()),
        ];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        // Header:
        // Field 0 (Day): Symmetric (01) -> bits 0-1 = 01
        // Field 1 (Month): Symmetric (01) -> bits 2-3 = 01
        // Field 2 (Year): FullRange (10) -> bits 4-5 = 10
        // Field 3 (Era): Absent (00) -> bits 6-7 = 00
        // Header value: 0b00100101 = 0x25
        assert_eq!(pgd.header.0, 0x25);
        assert_eq!(pgd.patterns.len(), 3); // 1 (Symmetric) + 1 (Symmetric) + 1 (FullRange) = 3

        let pgd_ule_box = zerovec::ule::encode_varule_to_box(&pgd);
        let pgd: &PatternsByGreatestDifferenceULE = &pgd_ule_box;
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Day)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Month)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_b)
        );
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Year)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_c)
        );

        // 2. All identical -> dedup to 1 (at the largest field, Year)
        let input = vec![
            (0, info_a.clone()),
            (1, info_a.clone()),
            (2, info_a.clone()),
        ];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        // Header:
        // Field 0 (Day): Absent (00)
        // Field 1 (Month): Absent (00)
        // Field 2 (Year): Symmetric (01) -> bits 4-5 = 01
        // Header value: 0b00010000 = 0x10
        assert_eq!(pgd.header.0, 0x10);
        assert_eq!(pgd.patterns.len(), 1);

        let pgd_ule_box = zerovec::ule::encode_varule_to_box(&pgd);
        let pgd: &PatternsByGreatestDifferenceULE = &pgd_ule_box;
        // Day and Month should fallback to Year (info_a)
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Day)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Month)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Year)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );

        // 3. Day and Month identical, Year different -> dedup Day to Month
        let input = vec![
            (0, info_a.clone()),
            (1, info_a.clone()),
            (2, info_b.clone()),
        ];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        // Header:
        // Field 0 (Day): Absent (00)
        // Field 1 (Month): Symmetric (01) -> bits 2-3 = 01
        // Field 2 (Year): Symmetric (01) -> bits 4-5 = 01
        // Header value: 0b00010100 = 0x14
        assert_eq!(pgd.header.0, 0x14);
        assert_eq!(pgd.patterns.len(), 2);

        let pgd_ule_box = zerovec::ule::encode_varule_to_box(&pgd);
        let pgd: &PatternsByGreatestDifferenceULE = &pgd_ule_box;
        // Day should fallback to Month (info_a)
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Day)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Month)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_a)
        );
        assert_eq!(
            pgd.get_date_pattern(DateGreatestDifferenceField::Year)
                .map(RangePatternInfo::from)
                .as_ref(),
            Some(&info_b)
        );

        // 4. Day and Year identical, Month different -> no dedup (since Month is in between and different)
        let input = vec![
            (0, info_a.clone()),
            (1, info_b.clone()),
            (2, info_a.clone()),
        ];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        // Header: Day (01), Month (01), Year (01) -> 0b00010101 = 0x15
        assert_eq!(pgd.header.0, 0x15);
        assert_eq!(pgd.patterns.len(), 3);

        // 5. Unsorted input -> should be sorted and deduped correctly
        let input = vec![
            (2, info_a.clone()),
            (0, info_a.clone()),
            (1, info_a.clone()),
        ];
        let pgd = PatternsByGreatestDifference::try_from_patterns(input).unwrap();
        assert_eq!(pgd.header.0, 0x10); // Only Year (2) present
        assert_eq!(pgd.patterns.len(), 1);

        // 6. Duplicate keys -> error
        let input = vec![(0, info_a.clone()), (0, info_b)];
        assert!(PatternsByGreatestDifference::try_from_patterns(input).is_err());

        // 7. Out of bound keys -> error
        let input = vec![(4, info_a)];
        assert!(PatternsByGreatestDifference::try_from_patterns(input).is_err());
    }
}
