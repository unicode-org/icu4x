// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::{self, runtime};
use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap};

/// Symbols used for representing the year name
///
/// This uses an auxiliary key for length.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(BuddhistYearSymbolsV1Marker, "datetime/symbols/buddhist/years@1"),
    marker(ChineseYearSymbolsV1Marker, "datetime/symbols/chinese/years@1"),
    marker(CopticYearSymbolsV1Marker, "datetime/symbols/coptic/years@1"),
    marker(DangiYearSymbolsV1Marker, "datetime/symbols/dangi/years@1"),
    marker(EthiopianYearSymbolsV1Marker, "datetime/symbols/ethiopic/years@1"),
    marker(GregorianYearSymbolsV1Marker, "datetime/symbols/gregory/years@1"),
    marker(HebrewYearSymbolsV1Marker, "datetime/symbols/hebrew/years@1"),
    marker(IndianYearSymbolsV1Marker, "datetime/symbols/indian/years@1"),
    marker(IslamicYearSymbolsV1Marker, "datetime/symbols/islamic/years@1"),
    marker(JapaneseYearSymbolsV1Marker, "datetime/symbols/japanese/years@1"),
    marker(
        JapaneseExtendedYearSymbolsV1Marker,
        "datetime/symbols/japanext/years@1"
    ),
    marker(PersianYearSymbolsV1Marker, "datetime/symbols/persian/years@1"),
    marker(RocYearSymbolsV1Marker, "datetime/symbols/roc/years@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum YearSymbolsV1<'data> {
    /// This calendar uses eras with numeric years, this stores the era names mapped from
    /// era code to the name
    Eras(
        #[cfg_attr(feature = "serde", serde(borrow))]
        ZeroMap<'data, UnvalidatedTinyAsciiStr<16>, str>,
    ),
    /// This calendar is cyclic (Chinese, Dangi), so it uses cyclic year names without any eras
    Cyclic(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
}

/// Symbols used for representing the month name
///
/// This uses an auxiliary key for length.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(BuddhistMonthSymbolsV1Marker, "datetime/symbols/buddhist/months@1"),
    marker(ChineseMonthSymbolsV1Marker, "datetime/symbols/chinese/months@1"),
    marker(CopticMonthSymbolsV1Marker, "datetime/symbols/coptic/months@1"),
    marker(DangiMonthSymbolsV1Marker, "datetime/symbols/dangi/months@1"),
    marker(EthiopianMonthSymbolsV1Marker, "datetime/symbols/ethiopic/months@1"),
    marker(GregorianMonthSymbolsV1Marker, "datetime/symbols/gregory/months@1"),
    marker(HebrewMonthSymbolsV1Marker, "datetime/symbols/hebrew/months@1"),
    marker(IndianMonthSymbolsV1Marker, "datetime/symbols/indian/months@1"),
    marker(IslamicMonthSymbolsV1Marker, "datetime/symbols/islamic/months@1"),
    marker(JapaneseMonthSymbolsV1Marker, "datetime/symbols/japanese/months@1"),
    marker(
        JapaneseExtendedMonthSymbolsV1Marker,
        "datetime/symbols/japanext/months@1"
    ),
    marker(PersianMonthSymbolsV1Marker, "datetime/symbols/persian/months@1"),
    marker(RocMonthSymbolsV1Marker, "datetime/symbols/roc/months@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum MonthSymbolsV1<'data> {
    /// Month codes M01, M02, M03, .. (can allow for M13 onwards)
    ///
    /// Found for solar and pure lunar calendars
    Numeric(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
    /// Month code map that can handle arbitrary month codes including leap months
    ///
    /// Found for lunisolar and lunisidereal calendars
    Map(
        #[cfg_attr(feature = "serde", serde(borrow))]
        ZeroMap<'data, UnvalidatedTinyAsciiStr<4>, str>,
    ),
}

/// Symbols that can be stored as a simple linear array.
///
/// - For weekdays, element 0 is Sunday
/// - For dayperiods, the elements are in order: AM, PM, (noon), (midnight), where the latter two are optional.
///   In the case noon is missing but midnight is present, the noon value can be the empty string. This is unlikely.
/// - For day names element 0 is the first day of the month
///
/// This uses an auxiliary key for length.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(WeekdaySymbolsV1Marker, "datetime/symbols/weekdays@1"),
    marker(DayPeriodSymbolsV1Marker, "datetime/symbols/dayperiods@1"),
    marker(ChineseDaySymbolsV1Marker, "datetime/symbols/chinese/days@1"),
    marker(DangiDaySymbolsV1Marker, "datetime/symbols/dangi/days@1"),
    // for calendars that don't use day symbols
    marker(PlaceholderDaySymbolsV1Marker, "datetime/symbols/placeholder/days@1"),
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LinearSymbolsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    // This uses a VarZeroVec rather than a fixed-size array for weekdays to save stack space
    symbols: VarZeroVec<'data, str>,
}

/// The default per-length patterns associated with dates and times
///
/// This uses an auxiliary key for length. time@1 additionally uses
/// the auxiliary key for representing hour cycle preferences.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(DatePatternV1Marker, "datetime/patterns/date@1"),
    marker(TimePatternV1Marker, "datetime/patterns/time@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PatternV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pattern: runtime::Pattern<'data>,
}

/// The default hour cycle intended to be used with a locale
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(PreferredHourCycleV1Marker, "datetime/patterns/hourcycle@1"))]
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PreferredHourCycleV1 {
    cycle: pattern::CoarseHourCycle,
}

/// The default per-length patterns used for combining dates and times into datetimes
///
/// This uses an auxiliary key for length.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(DateTimePatternV1Marker, "datetime/patterns/datetime@1"))]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DateTimePatternV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pattern: runtime::GenericPattern<'data>,
}
