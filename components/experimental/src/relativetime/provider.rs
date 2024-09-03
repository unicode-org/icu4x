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
use icu_pattern::{Pattern, PatternBackend, SinglePlaceholder};
use icu_plurals::{PluralCategory, PluralOperands, PluralRules};
#[cfg(feature = "datagen")]
use icu_plurals::PluralElements;
use icu_provider::prelude::*;
use zerovec::ZeroMap;

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
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::relativetime::provider))]
#[yoke(prove_covariance_manually)]
pub struct RelativeTimePatternDataV1<'data> {
    /// Mapping for relative times with unique names.
    /// Example.
    /// In English, "-1" corresponds to "yesterday", "1" corresponds to "tomorrow".
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub relatives: ZeroMap<'data, i8, str>,
    /// How to display times in the past.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past: PluralPatterns<'data, SinglePlaceholder>,
    /// How to display times in the future.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future: PluralPatterns<'data, SinglePlaceholder>,
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
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::relativetime::provider))]
#[yoke(prove_covariance_manually)]
pub struct PluralPatterns<'data, B> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[doc(hidden)] // databake only
    pub strings: icu_plurals::provider::PluralElementsV1<'data>,
    #[cfg_attr(feature = "serde", serde(skip))]
    #[doc(hidden)] // databake only
    pub _phantom: PhantomData<B>,
}

impl<'data, B> Clone for PluralPatterns<'data, B> {
    fn clone(&self) -> Self {
        Self {
            strings: self.strings.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<'data, B: PatternBackend<Store = str>> PluralPatterns<'data, B> {
    /// Returns the pattern for the given [`PluralCategory`].
    pub fn get(&'data self, op: PluralOperands, rules: &PluralRules) -> &'data Pattern<B, str> {
        Pattern::from_ref_store_unchecked(self.strings.get(op, rules))
    }
}

#[cfg(feature = "datagen")]
impl<'data, B: PatternBackend<Store = str>> TryFrom<PluralElements<'data, str>>
    for PluralPatterns<'static, B>
where
    B::PlaceholderKeyCow<'data>: FromStr,
    <B::PlaceholderKeyCow<'data> as FromStr>::Err: Debug,
{
    type Error = icu_pattern::PatternError;

    fn try_from(elements: PluralElements<str>) -> Result<Self, Self::Error> {
        let make_pattern = |s: &str|
            // TODO: Make pattern support apostrophes
            Pattern::<B, String>::from_str(&s.replace('\'', "''")).map(|p| p.take_store());

        Ok(Self {
            strings: PluralElements::new(make_pattern(elements.other)?.as_str())
                .with_zero_value(elements.zero.map(make_pattern).transpose()?.as_deref())
                .with_one_value(elements.one.map(make_pattern).transpose()?.as_deref())
                .with_two_value(elements.two.map(make_pattern).transpose()?.as_deref())
                .with_few_value(elements.few.map(make_pattern).transpose()?.as_deref())
                .with_many_value(elements.many.map(make_pattern).transpose()?.as_deref())
                .with_explicit_zero_value(
                    elements
                        .explicit_zero
                        .map(make_pattern)
                        .transpose()?
                        .as_deref(),
                )
                .with_explicit_one_value(
                    elements
                        .explicit_one
                        .map(make_pattern)
                        .transpose()?
                        .as_deref(),
                )
                .into(),
            _phantom: PhantomData,
        })
    }
}

pub(crate) struct ErasedRelativeTimeFormatV1Marker;

impl DynamicDataMarker for ErasedRelativeTimeFormatV1Marker {
    type DataStruct = RelativeTimePatternDataV1<'static>;
}
