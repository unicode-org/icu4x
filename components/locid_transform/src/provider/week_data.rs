// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use zerovec::ZeroVec;

/// ICU4X representation of CLDR's weekData.
/// See CLDR-JSON's weekData.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(
    WeekDataV2Marker,
    // TODO: [CODE REVIEW]
    // not sure if we should use `@2` here because of `WeekDataV2`,
    // or if we should use `@1` because the path is different from the existing data `datetime/week_data@1`,
    // or if this means we should name this `WeekDataV1` instead.
    "locid_transform/week_data@2", 
    // TODO: [CODE REVIEW]
    // not sure which arguments we should put here.
    singleton
))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_locid_transform::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)]
#[yoke(prove_covariance_manually)]
pub struct WeekDataV2<'data> {
    /// The first day of a week.
    pub first_weekday: IsoWeekday,
    /// For a given week, the minimum number of that week's days present in a given month or year for the week to be considered part of that month or year.
    pub min_week_days: u8,
    /// Weekdays values that are part of the 'weekend', for calendar purposes.
    /// The number of days can be different between locales, and may not be contiguous.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekend: ZeroVec<'data, IsoWeekday>,
}