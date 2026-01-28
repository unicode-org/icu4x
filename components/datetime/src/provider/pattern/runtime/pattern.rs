// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::exhaustive_structs)] // part of data struct and internal API

use super::super::{reference, PatternError, PatternItem, TimeGranularity};
use crate::provider::fields::{self, Field, FieldSymbol};
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
#[derive(Debug, Copy, Clone)]
pub(crate) struct PatternBorrowed<'data> {
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
    const PREFER_KEEP_MINUTES_MASK: u8 = 0x08;

    #[inline]
    pub(crate) fn time_granularity(self) -> TimeGranularity {
        TimeGranularity::from_ordinal(self.0 & !Self::PREFER_KEEP_MINUTES_MASK)
    }

    #[inline]
    pub(crate) fn prefer_keep_minutes(self) -> bool {
        (self.0 & Self::PREFER_KEEP_MINUTES_MASK) != 0
    }

    pub(crate) fn from_items(items: &[PatternItem]) -> Self {
        Self::from_iter_items(items.iter().copied())
    }

    pub(crate) fn from_iter_items(iter_items: impl Iterator<Item = PatternItem>) -> Self {
        let mut time_granularity = TimeGranularity::None;
        let mut prefer_keep_minutes = false;

        for item in iter_items {
            let item_granularity = TimeGranularity::from(item);
            if item_granularity > time_granularity {
                time_granularity = item_granularity;
            }
            if !prefer_keep_minutes {
                if let PatternItem::Field(Field {
                    symbol: FieldSymbol::Hour(fields::Hour::H23),
                    ..
                }) = item
                {
                    prefer_keep_minutes = true;
                }
            }
        }

        let mut result = Self::from_time_granularity(time_granularity);
        if prefer_keep_minutes {
            result.0 |= Self::PREFER_KEEP_MINUTES_MASK;
        }
        result
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

    /// Sets the `prefer_keep_minutes` flag.
    #[inline]
    pub const fn with_prefer_keep_minutes(self, prefer_keep_minutes: bool) -> Self {
        if prefer_keep_minutes {
            Self(self.0 | Self::PREFER_KEEP_MINUTES_MASK)
        } else {
            Self(self.0 & !Self::PREFER_KEEP_MINUTES_MASK)
        }
    }

    #[cfg(feature = "datagen")]
    #[inline]
    pub(crate) fn set_time_granularity(&mut self, time_granularity: TimeGranularity) {
        let prefer_keep_minutes = self.prefer_keep_minutes();
        self.0 = time_granularity.ordinal();
        if prefer_keep_minutes {
            self.0 |= Self::PREFER_KEEP_MINUTES_MASK;
        }
    }

    pub(crate) fn to_four_bit_metadata(self) -> FourBitMetadata {
        #[expect(clippy::unwrap_used)] // valid values for self.0 are 0-15
        FourBitMetadata::try_from_byte(self.0).unwrap()
    }

    pub(crate) fn from_u8(other: u8) -> Self {
        let granularity = TimeGranularity::from_ordinal(other & !Self::PREFER_KEEP_MINUTES_MASK);
        let mut result = Self::from_time_granularity(granularity);
        if (other & Self::PREFER_KEEP_MINUTES_MASK) != 0 {
            result.0 |= Self::PREFER_KEEP_MINUTES_MASK;
        }
        result
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
        let mut metadata = PatternMetadata::from_time_granularity(input.time_granularity);
        for item in &input.items {
            if let PatternItem::Field(Field {
                symbol: FieldSymbol::Hour(fields::Hour::H23),
                ..
            }) = item
            {
                metadata = metadata.with_prefer_keep_minutes(true);
                break;
            }
        }
        Self {
            items: ZeroVec::alloc_from_slice(&input.items),
            metadata,
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
        let prefer_keep_minutes = self.prefer_keep_minutes();
        if prefer_keep_minutes {
            databake::quote! {
                icu_datetime::provider::pattern::runtime::PatternMetadata::from_time_granularity(#time_granularity)
                    .with_prefer_keep_minutes(true)
            }
        } else {
            databake::quote! {
                icu_datetime::provider::pattern::runtime::PatternMetadata::from_time_granularity(#time_granularity)
            }
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

#[test]
fn test_prefer_keep_minutes() {
    use crate::provider::fields::FieldLength;
    let items_h23 = [PatternItem::Field(Field {
        symbol: FieldSymbol::Hour(fields::Hour::H23),
        length: FieldLength::Two,
    })];
    let metadata_h23 = PatternMetadata::from_items(&items_h23);
    assert!(
        metadata_h23.prefer_keep_minutes(),
        "H23 should set prefer_keep_minutes"
    );

    let items_h12 = [PatternItem::Field(Field {
        symbol: FieldSymbol::Hour(fields::Hour::H12),
        length: FieldLength::Two,
    })];
    let metadata_h12 = PatternMetadata::from_items(&items_h12);
    assert!(
        !metadata_h12.prefer_keep_minutes(),
        "H12 should NOT set prefer_keep_minutes"
    );
}
