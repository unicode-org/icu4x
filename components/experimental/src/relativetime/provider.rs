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

icu_provider::data_marker!(
    /// Relative time formatting data for seconds.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeSecondV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for minutes.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeMinuteV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for hours.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeHourV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for days.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeDayV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for weeks.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeWeekV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for months.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeMonthV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for quarters.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeQuarterV1,
    RelativeTimePatternData<'static>,
);
icu_provider::data_marker!(
    /// Relative time formatting data for years.
    ///
    /// Uses [`DataMarkerAttributes`] to distinguish width:
    /// - [`RelativeTimePatternData::LONG`] (`"L"`)
    /// - [`RelativeTimePatternData::SHORT`] (`"S"`)
    /// - [`RelativeTimePatternData::NARROW`] (`"N"`)
    DatetimeRelativeYearV1,
    RelativeTimePatternData<'static>,
);

/// Relative time format  data struct.
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
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

impl RelativeTimePatternData<'_> {
    /// The marker attributes for long (wide) relative time formatting.
    pub const LONG: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("L");
    #[doc(hidden)]
    pub const LONG_STR: &'static str = Self::LONG.as_str();
    /// The marker attributes for short relative time formatting.
    pub const SHORT: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("S");
    #[doc(hidden)]
    pub const SHORT_STR: &'static str = Self::SHORT.as_str();
    /// The marker attributes for narrow relative time formatting.
    pub const NARROW: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("N");
    #[doc(hidden)]
    pub const NARROW_STR: &'static str = Self::NARROW.as_str();
}

icu_provider::data_struct!(RelativeTimePatternData<'_>, #[cfg(feature = "datagen")]);
