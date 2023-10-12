// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::{self, runtime};
use icu_provider::prelude::*;

use zerovec::{ule::UnvalidatedStr, VarZeroVec, ZeroMap};

/// Symbols used for representing the year name
///
/// This uses an auxiliary key for length.
#[icu_provider::data_struct(marker(YearSymbolsV1Marker, "datetime/symbols/years@1"))]
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
    Eras(#[cfg_attr(feature = "serde", serde(borrow))] ZeroMap<'data, UnvalidatedStr, str>),
    /// This calendar is cyclic (Chinese, Dangi), so it uses cyclic year names without any eras
    Cyclic(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
}

/// Symbols used for representing the month name
///
/// This uses an auxiliary key for length.
#[icu_provider::data_struct(marker(MonthSymbolsV1Marker, "datetime/symbols/months@1"))]
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
    Map(#[cfg_attr(feature = "serde", serde(borrow))] ZeroMap<'data, UnvalidatedStr, str>),
}

/// Symbols that can be stored as a simple linear array.
///
/// - For weekdays, element 0 is Monday
/// - For dayperiods, the elements are in order: AM, PM, (noon), (midnight), where the latter two are optional
/// - For day names element 0 is the first day of the month
///
/// This uses an auxiliary key for length.
#[icu_provider::data_struct(
    marker(WeekdaySymbolsV1Marker, "datetime/symbols/weekdays@1"),
    marker(DayPeriodSymbolsV1Marker, "datetime/symbols/dayperiods@1"),
    marker(DaySymbolsV1Marker, "datetime/symbols/days@1")
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
    symbols: VarZeroVec<'data, str>,
}

/// The default per-length patterns associated with dates and times
///
/// This uses an auxiliary key for length. time@1 additionally uses
/// the auxiliary key for representing hour cycle preferences.
#[icu_provider::data_struct(
    marker(DateLengthsV1Marker, "datetime/lengths/date@1"),
    marker(TimeLengthsV1Marker, "datetime/lengths/time@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LengthsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pattern: runtime::Pattern<'data>,
}

/// The default hour cycle intended to be used with a locale
#[icu_provider::data_struct(marker(PreferredHourCycleV1Marker, "datetime/lengths/hourcycle@1"))]
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
#[icu_provider::data_struct(marker(DateTimeLengthsV1Marker, "datetime/lengths/datetime@1"))]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DateTimeLengthsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pattern: runtime::GenericPattern<'data>,
}
