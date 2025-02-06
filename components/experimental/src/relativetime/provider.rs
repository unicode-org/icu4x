// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

#[cfg(feature = "datagen")]
use core::fmt::Debug;
use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::provider::PluralElementsPackedCow;
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

/// Relative time format  data struct.

#[icu_provider::data_struct(
    LongSecondRelativeV1 = "relativetime/long/second@1",
    ShortSecondRelativeV1 = "relativetime/short/second@1",
    NarrowSecondRelativeV1 = "relativetime/narrow/second@1",
    LongMinuteRelativeV1 = "relativetime/long/minute@1",
    ShortMinuteRelativeV1 = "relativetime/short/minute@1",
    NarrowMinuteRelativeV1 = "relativetime/narrow/minute@1",
    LongHourRelativeV1 = "relativetime/long/hour@1",
    ShortHourRelativeV1 = "relativetime/short/hour@1",
    NarrowHourRelativeV1 = "relativetime/narrow/hour@1",
    LongDayRelativeV1 = "relativetime/long/day@1",
    ShortDayRelativeV1 = "relativetime/short/day@1",
    NarrowDayRelativeV1 = "relativetime/narrow/day@1",
    LongWeekRelativeV1 = "relativetime/long/week@1",
    ShortWeekRelativeV1 = "relativetime/short/week@1",
    NarrowWeekRelativeV1 = "relativetime/narrow/week@1",
    LongMonthRelativeV1 = "relativetime/long/month@1",
    ShortMonthRelativeV1 = "relativetime/short/month@1",
    NarrowMonthRelativeV1 = "relativetime/narrow/month@1",
    LongQuarterRelativeV1 = "relativetime/long/quarter@1",
    ShortQuarterRelativeV1 = "relativetime/short/quarter@1",
    NarrowQuarterRelativeV1 = "relativetime/narrow/quarter@1",
    LongYearRelativeV1 = "relativetime/long/year@1",
    ShortYearRelativeV1 = "relativetime/short/year@1",
    NarrowYearRelativeV1 = "relativetime/narrow/year@1"
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::relativetime::provider))]
#[yoke(prove_covariance_manually)]
pub struct RelativeTimePatternData<'data> {
    /// Mapping for relative times with unique names.
    /// Example.
    /// In English, "-1" corresponds to "yesterday", "1" corresponds to "tomorrow".
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub relatives: ZeroMap<'data, i8, str>,
    /// How to display times in the past.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past: PluralElementsPackedCow<'data, SinglePlaceholderPattern>,
    /// How to display times in the future.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future: PluralElementsPackedCow<'data, SinglePlaceholderPattern>,
}
