// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::exhaustive_structs)] // part of data struct and internal API

use super::super::{PatternError, PatternItem, TimeGranularity, reference};
use alloc::vec::Vec;
use core::str::FromStr;
use icu_plurals::provider::FourBitMetadata;
use icu_provider::prelude::*;
use zerovec::{ZeroSlice, ZeroVec};

/// A raw, low-level pattern for datetime formatting.
///
/// It consists of an owned-or-borrowed list of [`PatternItem`]s corresponding
/// to either fields or literal characters.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Eq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::pattern::runtime))]
#[zerovec::make_varule(PatternULE)]
#[zerovec::derive(Debug)]
#[zerovec::skip_derive(Ord)]
#[cfg_attr(feature = "serde", zerovec::derive(Deserialize))]
#[cfg_attr(feature = "datagen", zerovec::derive(Serialize))]
pub struct Pattern<'data> {
    /// The list of [`PatternItem`]s.
    pub items: ZeroVec<'data, PatternItem>,
    /// Pre-computed metadata about the pattern.
    ///
    /// This field should contain the smallest time unit from the `items` vec.
    /// If it doesn't, unexpected results for day periods may be encountered.
    pub metadata: PatternMetadata,
}

/// Fully borrowed version of [`Pattern`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PatternBorrowed<'data> {
    pub(crate) items: &'data ZeroSlice<PatternItem>,
    pub(crate) metadata: PatternMetadata,
}

/// Metadata associated with a [`Pattern`].
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[zerovec::make_ule(PatternMetadataULE)]
#[zerovec::skip_derive(Ord)]
pub struct PatternMetadata(u8);

impl PatternMetadata {
    pub(crate) const DEFAULT: PatternMetadata = Self::from_time_granularity(TimeGranularity::None);

    #[inline]
    pub(crate) fn time_granularity(self) -> TimeGranularity {
        TimeGranularity::from_ordinal(self.0)
    }

    pub(crate) fn from_items(items: &[PatternItem]) -> Self {
        Self::from_iter_items(items.iter().copied())
    }

    pub(crate) fn from_iter_items(iter_items: impl Iterator<Item = PatternItem>) -> Self {
        let time_granularity: TimeGranularity =
            iter_items.map(Into::into).max().unwrap_or_default();
        Self::from_time_granularity(time_granularity)
    }

    /// Merges the metadata from a date pattern and a time pattern into one.
    #[inline]
    pub(crate) fn merge_date_and_time_metadata(
        _date: PatternMetadata,
        time: PatternMetadata,
    ) -> PatternMetadata {
        // Currently we only have time granularity so we ignore the date metadata.
        time
    }

    /// Creates a [`PatternMetadata`] from the [`TimeGranularity`] enum.
    #[inline]
    pub const fn from_time_granularity(time_granularity: TimeGranularity) -> Self {
        Self(time_granularity.ordinal())
    }

    #[cfg(feature = "datagen")]
    #[inline]
    pub(crate) fn set_time_granularity(&mut self, time_granularity: TimeGranularity) {
        self.0 = time_granularity.ordinal();
    }

    pub(crate) fn to_four_bit_metadata(self) -> FourBitMetadata {
        #[expect(clippy::unwrap_used)] // valid values for self.0 are 0, 1, 2, 3, or 4
        FourBitMetadata::try_from_byte(self.0).unwrap()
    }

    pub(crate) fn from_u8(other: u8) -> Self {
        Self(TimeGranularity::from_ordinal(other).ordinal())
    }
}

impl Default for PatternMetadata {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Pattern<'_> {
    pub(crate) fn into_owned(self) -> Pattern<'static> {
        Pattern {
            items: self.items.into_owned(),
            metadata: self.metadata,
        }
    }

    pub(crate) fn as_borrowed(&self) -> PatternBorrowed<'_> {
        PatternBorrowed {
            items: &self.items,
            metadata: self.metadata,
        }
    }

    /// Borrows a [`Pattern`] from another [`Pattern`].
    pub fn as_ref(&self) -> Pattern<'_> {
        self.as_borrowed().as_pattern()
    }
}

impl<'data> PatternBorrowed<'data> {
    pub(crate) const DEFAULT: PatternBorrowed<'static> = PatternBorrowed {
        items: ZeroSlice::new_empty(),
        metadata: PatternMetadata::DEFAULT,
    };

    pub(crate) fn as_pattern(self) -> Pattern<'data> {
        Pattern {
            items: self.items.as_zerovec(),
            metadata: self.metadata,
        }
    }

    /// Returns the index of the first repeated field in the pattern, if any.
    ///
    /// This is used to find the split point for range patterns.
    /// Returns the index of the first repeated field in the pattern, if any.
    ///
    /// This is used to find the split point for range patterns.
    #[allow(dead_code, reason = "#5448")]
    fn first_repeated_field_index(&self) -> usize {
        struct Seen {
            // A bitset where the i-th bit is set if we have seen a field of type index i.
            // Since there are at most 16 field types, a u16 is sufficient.
            mask: u16,
        }
        impl Seen {
            fn new() -> Self {
                Self { mask: 0 }
            }
            fn insert_or_contains(&mut self, symbol: crate::provider::fields::FieldSymbol) -> bool {
                let type_idx = symbol.type_idx();
                let bit = 1 << type_idx;
                if (self.mask & bit) != 0 {
                    true
                } else {
                    self.mask |= bit;
                    false
                }
            }
        }

        let mut seen = Seen::new();
        for (idx, item) in self.items.iter().enumerate() {
            match item {
                PatternItem::Field(field) if seen.insert_or_contains(field.symbol) => {
                    return idx;
                }
                _ => {}
            }
        }
        self.items.len()
    }

    /// Splits the pattern into two halves at the first repeated field.
    ///
    /// If there are no repeated fields, the second half will be empty.
    #[allow(dead_code, reason = "#5448")]
    pub(crate) fn split_on_repeated_field(&self) -> (Self, Self) {
        let idx = self.first_repeated_field_index();
        let ule_slice = self.items.as_ule_slice();
        let (start_ule, end_ule) = ule_slice.split_at_checked(idx).unwrap_or((ule_slice, &[]));
        (
            Self {
                items: ZeroSlice::from_ule_slice(start_ule),
                metadata: self.metadata,
            },
            Self {
                items: ZeroSlice::from_ule_slice(end_ule),
                metadata: if end_ule.is_empty() {
                    PatternMetadata::default()
                } else {
                    self.metadata
                },
            },
        )
    }
}

impl<'data> zerofrom::ZeroFrom<'data, PatternULE> for PatternBorrowed<'data> {
    #[inline]
    fn zero_from(ule: &'data PatternULE) -> Self {
        use zerovec::ule::AsULE;
        Self {
            items: &ule.items,
            metadata: <PatternMetadata as AsULE>::from_unaligned(ule.metadata),
        }
    }
}

impl From<Vec<PatternItem>> for Pattern<'_> {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            metadata: PatternMetadata::from_items(&items),
            items: ZeroVec::alloc_from_slice(&items),
        }
    }
}

impl FromIterator<PatternItem> for Pattern<'_> {
    fn from_iter<T: IntoIterator<Item = PatternItem>>(iter: T) -> Self {
        let items = iter.into_iter().collect::<ZeroVec<PatternItem>>();
        Self {
            metadata: PatternMetadata::from_iter_items(items.iter()),
            items,
        }
    }
}

impl From<&reference::Pattern> for Pattern<'_> {
    fn from(input: &reference::Pattern) -> Self {
        Self {
            items: ZeroVec::alloc_from_slice(&input.items),
            metadata: PatternMetadata::from_time_granularity(input.time_granularity),
        }
    }
}

impl From<&Pattern<'_>> for reference::Pattern {
    fn from(input: &Pattern<'_>) -> Self {
        Self {
            items: input.items.to_vec(),
            time_granularity: input.metadata.time_granularity(),
        }
    }
}

impl FromStr for Pattern<'_> {
    type Err = PatternError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reference = reference::Pattern::from_str(input)?;
        Ok(Self::from(&reference))
    }
}

impl Default for Pattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::new(),
            metadata: PatternMetadata::default(),
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for PatternMetadata {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("icu_datetime");
        let time_granularity = databake::Bake::bake(&self.time_granularity(), ctx);
        databake::quote! {
            icu_datetime::provider::pattern::runtime::PatternMetadata::from_time_granularity(#time_granularity)
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::BakeSize for PatternMetadata {
    fn borrows_size(&self) -> usize {
        0
    }
}

#[test]
#[cfg(feature = "datagen")]
fn databake() {
    databake::test_bake!(
        PatternMetadata,
        const,
        crate::provider::pattern::runtime::PatternMetadata::from_time_granularity(
            crate::provider::pattern::TimeGranularity::Hours
        ),
        icu_datetime,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_split_on_repeated_field() {
        let cases = [
            // No repetition
            ("yMd", ("yMd", "")),
            ("yM-d", ("yM-d", "")),
            ("HH:mm:ss", ("HH:mm:ss", "")),
            // Repetition of same field type (e.g. in range patterns)
            ("yMd-d", ("yMd-", "d")),
            ("yMd-yMd", ("yMd-", "yMd")),
            ("h:mm a - h:mm a", ("h:mm a - ", "h:mm a")),
        ];

        for &(pattern_str, (expected_start, expected_end)) in &cases {
            let pattern = Pattern::from_str(pattern_str).unwrap();
            let pattern_borrowed = pattern.as_borrowed();
            let (start, end) = pattern_borrowed.split_on_repeated_field();

            let expected_start_pattern = Pattern::from_str(expected_start).unwrap();
            let expected_end_pattern = Pattern::from_str(expected_end).unwrap();

            assert_eq!(
                start,
                expected_start_pattern.as_borrowed(),
                "Pattern: {}",
                pattern_str
            );
            assert_eq!(
                end,
                expected_end_pattern.as_borrowed(),
                "Pattern: {}",
                pattern_str
            );
        }
    }
}
