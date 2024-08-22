// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use core::marker::PhantomData;
#[cfg(feature = "datagen")]
use core::{fmt::Debug, str::FromStr};
use icu_pattern::{Pattern, PatternBackend, SinglePlaceholderPattern};
use icu_plurals::PluralCategory;
use icu_provider::prelude::*;
use zerovec::{ule::AsULE, VarZeroVec, ZeroMap};

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

/// Relative time format V1 data struct.

#[icu_provider::data_struct(
    LongSecondRelativeTimeFormatDataV1Marker = "relativetime/long/second@1",
    ShortSecondRelativeTimeFormatDataV1Marker = "relativetime/short/second@1",
    NarrowSecondRelativeTimeFormatDataV1Marker = "relativetime/narrow/second@1",
    LongMinuteRelativeTimeFormatDataV1Marker = "relativetime/long/minute@1",
    ShortMinuteRelativeTimeFormatDataV1Marker = "relativetime/short/minute@1",
    NarrowMinuteRelativeTimeFormatDataV1Marker = "relativetime/narrow/minute@1",
    LongHourRelativeTimeFormatDataV1Marker = "relativetime/long/hour@1",
    ShortHourRelativeTimeFormatDataV1Marker = "relativetime/short/hour@1",
    NarrowHourRelativeTimeFormatDataV1Marker = "relativetime/narrow/hour@1",
    LongDayRelativeTimeFormatDataV1Marker = "relativetime/long/day@1",
    ShortDayRelativeTimeFormatDataV1Marker = "relativetime/short/day@1",
    NarrowDayRelativeTimeFormatDataV1Marker = "relativetime/narrow/day@1",
    LongWeekRelativeTimeFormatDataV1Marker = "relativetime/long/week@1",
    ShortWeekRelativeTimeFormatDataV1Marker = "relativetime/short/week@1",
    NarrowWeekRelativeTimeFormatDataV1Marker = "relativetime/narrow/week@1",
    LongMonthRelativeTimeFormatDataV1Marker = "relativetime/long/month@1",
    ShortMonthRelativeTimeFormatDataV1Marker = "relativetime/short/month@1",
    NarrowMonthRelativeTimeFormatDataV1Marker = "relativetime/narrow/month@1",
    LongQuarterRelativeTimeFormatDataV1Marker = "relativetime/long/quarter@1",
    ShortQuarterRelativeTimeFormatDataV1Marker = "relativetime/short/quarter@1",
    NarrowQuarterRelativeTimeFormatDataV1Marker = "relativetime/narrow/quarter@1",
    LongYearRelativeTimeFormatDataV1Marker = "relativetime/long/year@1",
    ShortYearRelativeTimeFormatDataV1Marker = "relativetime/short/year@1",
    NarrowYearRelativeTimeFormatDataV1Marker = "relativetime/narrow/year@1"
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct RelativeTimePatternDataV1<'data> {
    /// Mapping for relative times with unique names.
    /// Example.
    /// In English, "-1" corresponds to "yesterday", "1" corresponds to "tomorrow".
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub relatives: ZeroMap<'data, i8, str>,
    /// How to display times in the past.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past: PluralElements<'data, SinglePlaceholderPattern<str>>,
    /// How to display times in the future.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future: PluralElements<'data, SinglePlaceholderPattern<str>>,
}

#[derive(Debug)]
#[zerovec::make_varule(PluralCategoryStrULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
/// A tuple of [`PluralCategory`] and [`str`].
pub struct PluralCategoryStr<'data>(pub PluralCategory, pub Cow<'data, str>);

/// Display specification for relative times, split over potential plural patterns.
#[derive(Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct PluralElements<'data, T: ?Sized> {
    /// Optional entries for categories other than PluralCategory::Other
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub specials: VarZeroVec<'data, PluralCategoryStrULE>,
    /// The entry for PluralCategory::Other
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub other: Cow<'data, str>,
    /// Keeps track of T
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _phantom: PhantomData<T>,
}

impl<'data, T: ?Sized> Clone for PluralElements<'data, T> {
    fn clone(&self) -> Self {
        Self {
            specials: self.specials.clone(),
            other: self.other.clone(),
            _phantom: self._phantom,
        }
    }
}

impl<'data, B: PatternBackend<Store = str>> PluralElements<'data, Pattern<B, str>> {
    /// Creates a [`PluralElements`] from the given patterns.
    #[cfg(feature = "datagen")]
    pub fn try_new_pattern(
        other: &str,
        zero: Option<&str>,
        one: Option<&str>,
        two: Option<&str>,
        few: Option<&str>,
        many: Option<&str>,
    ) -> Result<Self, icu_pattern::PatternError>
    where
        B::PlaceholderKeyCow<'data>: FromStr,
        <B::PlaceholderKeyCow<'data> as FromStr>::Err: Debug,
    {
        let optional_convert = |category, pattern: Option<&str>| {
            pattern
                .filter(|p| *p != other)
                .map(|s| {
                    Ok(PluralCategoryStr(
                        category,
                        // TODO: Make pattern support apostrophes
                        Pattern::<B, String>::from_str(&s.replace('\'', "''"))
                            .map(|p| Pattern::<B, _>::from_store_unchecked(p.take_store().into()))?
                            .take_store(),
                    ))
                })
                .transpose()
        };

        Ok(Self {
            specials: (&[
                (optional_convert(PluralCategory::Zero, zero)?),
                (optional_convert(PluralCategory::One, one)?),
                (optional_convert(PluralCategory::Two, two)?),
                (optional_convert(PluralCategory::Few, few)?),
                (optional_convert(PluralCategory::Many, many)?),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>())
                .into(),
            // TODO: Make pattern support apostrophes
            other: Pattern::<B, String>::from_str(&other.replace('\'', "''"))?
                .take_store()
                .into(),
            _phantom: PhantomData::<Pattern<B, str>>,
        })
    }

    /// Returns the pattern for the given [`PluralCategory`].
    pub fn get_pattern(&'data self, c: PluralCategory) -> &'data Pattern<B, str> {
        Pattern::from_ref_store_unchecked(if c == PluralCategory::Other {
            &*self.other
        } else {
            self.specials
                .iter()
                .filter_map(|ule| (ule.0 == c.to_unaligned()).then_some(&ule.1))
                .next()
                .unwrap_or(&*self.other)
        })
    }
}

impl<'data> PluralElements<'data, str> {
    /// Creates a [`PluralElements`] from the given strings.
    #[cfg(feature = "datagen")]
    pub fn try_new(
        other: &str,
        zero: Option<&str>,
        one: Option<&str>,
        two: Option<&str>,
        few: Option<&str>,
        many: Option<&str>,
    ) -> Self
where {
        Self {
            specials: (&[
                zero.filter(|p| *p != other)
                    .map(|s| PluralCategoryStr(PluralCategory::Zero, s.into())),
                one.filter(|p| *p != other)
                    .map(|s| PluralCategoryStr(PluralCategory::One, s.into())),
                two.filter(|p| *p != other)
                    .map(|s| PluralCategoryStr(PluralCategory::Two, s.into())),
                few.filter(|p| *p != other)
                    .map(|s| PluralCategoryStr(PluralCategory::Few, s.into())),
                many.filter(|p| *p != other)
                    .map(|s| PluralCategoryStr(PluralCategory::Many, s.into())),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>())
                .into(),
            other: other.to_owned().into(),
            _phantom: PhantomData::<str>,
        }
    }

    /// Returns the string for the given [`PluralCategory`].
    pub fn get_str(&'data self, c: PluralCategory) -> &'data str {
        if c == PluralCategory::Other {
            &self.other
        } else {
            self.specials
                .iter()
                .filter_map(|ule| (ule.0 == c.to_unaligned()).then_some(&ule.1))
                .next()
                .unwrap_or(&*self.other)
        }
    }
}

#[test]
fn plural_patterns_niche() {
    assert_eq!(
        core::mem::size_of::<PluralElements<SinglePlaceholderPattern<str>>>(),
        48
    );
    assert_eq!(
        core::mem::size_of::<Option<PluralElements<SinglePlaceholderPattern<str>>>>(),
        48
    );
}

pub(crate) struct ErasedRelativeTimeFormatV1Marker;

impl DynamicDataMarker for ErasedRelativeTimeFormatV1Marker {
    type DataStruct = RelativeTimePatternDataV1<'static>;
}
