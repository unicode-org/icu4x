// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_pattern::SinglePlaceholderPattern;
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
    pub past: SinglePlaceholderPluralPattern<'data>,
    /// How to display times in the future.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future: SinglePlaceholderPluralPattern<'data>,
}

/// Display specification for relative times, split over potential plural patterns.
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct SinglePlaceholderPluralPattern<'data> {
    /// Mapping for PluralCategory::Zero or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub zero: Option<SinglePlaceholderPattern<Cow<'data, str>>>,
    /// Mapping for PluralCategory::One or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub one: Option<SinglePlaceholderPattern<Cow<'data, str>>>,
    /// Mapping for PluralCategory::Two or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub two: Option<SinglePlaceholderPattern<Cow<'data, str>>>,
    /// Mapping for PluralCategory::Few or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub few: Option<SinglePlaceholderPattern<Cow<'data, str>>>,
    /// Mapping for PluralCategory::Many or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub many: Option<SinglePlaceholderPattern<Cow<'data, str>>>,
    /// Mapping for PluralCategory::Other
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub other: SinglePlaceholderPattern<Cow<'data, str>>,
}

impl<'data> SinglePlaceholderPluralPattern<'data> {
    pub(crate) fn get(
        &self,
        c: icu_plurals::PluralCategory,
    ) -> &SinglePlaceholderPattern<Cow<'data, str>> {
        match c {
            icu_plurals::PluralCategory::Zero => self.zero.as_ref(),
            icu_plurals::PluralCategory::One => self.one.as_ref(),
            icu_plurals::PluralCategory::Two => self.two.as_ref(),
            icu_plurals::PluralCategory::Few => self.few.as_ref(),
            icu_plurals::PluralCategory::Many => self.many.as_ref(),
            icu_plurals::PluralCategory::Other => None,
        }
        .unwrap_or(&self.other)
    }
}

pub(crate) struct ErasedRelativeTimeFormatV1Marker;

impl DynamicDataMarker for ErasedRelativeTimeFormatV1Marker {
    type DataStruct = RelativeTimePatternDataV1<'static>;
}
