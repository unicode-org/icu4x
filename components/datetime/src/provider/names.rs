// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for datetime names.

use crate::provider::pattern::runtime;
use crate::size_test_macro::size_test;
use alloc::borrow::Cow;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use zerovec::{ule::tuplevar::Tuple2VarULE, VarZeroCow, VarZeroSlice, VarZeroVec};

icu_provider::data_marker!(
    /// `DatetimeNamesYearBuddhistV1`
    DatetimeNamesYearBuddhistV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearChineseV1`
    DatetimeNamesYearChineseV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearCopticV1`
    DatetimeNamesYearCopticV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearDangiV1`
    DatetimeNamesYearDangiV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearEthiopianV1`
    DatetimeNamesYearEthiopianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearGregorianV1`
    DatetimeNamesYearGregorianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearHebrewV1`
    DatetimeNamesYearHebrewV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearIndianV1`
    DatetimeNamesYearIndianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearHijriV1`
    DatetimeNamesYearHijriV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearJapaneseV1`
    DatetimeNamesYearJapaneseV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearPersianV1`
    DatetimeNamesYearPersianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearRocV1`
    DatetimeNamesYearRocV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);

icu_provider::data_marker!(
    /// `DatetimeNamesMonthBuddhistV1`
    DatetimeNamesMonthBuddhistV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthChineseV1`
    DatetimeNamesMonthChineseV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthCopticV1`
    DatetimeNamesMonthCopticV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthDangiV1`
    DatetimeNamesMonthDangiV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthEthiopianV1`
    DatetimeNamesMonthEthiopianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthGregorianV1`
    DatetimeNamesMonthGregorianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthHebrewV1`
    DatetimeNamesMonthHebrewV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthIndianV1`
    DatetimeNamesMonthIndianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthHijriV1`
    DatetimeNamesMonthHijriV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthJapaneseV1`
    DatetimeNamesMonthJapaneseV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthPersianV1`
    DatetimeNamesMonthPersianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthRocV1`
    DatetimeNamesMonthRocV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);

icu_provider::data_marker!(
    /// `DatetimeNamesWeekdayV1`
    DatetimeNamesWeekdayV1,
    LinearNames<'static>,
);
icu_provider::data_marker!(
    /// `DatetimeNamesDayperiodV1`
    DatetimeNamesDayperiodV1,
    LinearNames<'static>,
);
// We're not producing or using day names yet, but this is where they would go
icu_provider::data_marker!(
    /// `DatetimeNamesDayChineseV1`
    DatetimeNamesDayChineseV1,
    LinearNames<'static>,
);
icu_provider::data_marker!(
    /// `DatetimeNamesDayDangiV1`
    DatetimeNamesDayDangiV1,
    LinearNames<'static>,
);
// for calendars that don't use day names
icu_provider::data_marker!(
    /// `DatetimeNamesDayPlaceholderV1`
    DatetimeNamesDayPlaceholderV1,
    LinearNames<'static>,
);

size_test!(YearNames, year_names_v1_size, 32);

/// Names used for representing the year.
///
/// This uses a data marker attribute for length. The value is simply the number of
/// characters in the equivalent CLDR field syntax name, plus "s" for standalone contexts. For example,
/// "abbreviated" (e.g. `MMM`) is `3` or `3s` depending on whether it is format or standalone
/// respectively.
///
/// The full list is:
/// - 3 is "abbreviated"
/// - 4 is "narrow"
/// - 5 is "wide"
/// - 6 is "short" (weekdays only)
#[doc = year_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum YearNames<'data> {
    /// This calendar has a small, fixed set of eras.
    ///
    /// See [`era_index`](icu_calendar::types::EraYear::era_index) for how this is keyed.
    FixedEras(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
    /// This calendar has a variable set of eras with numeric years, this stores the era names mapped from
    /// era code to the name.
    VariableEras(#[cfg_attr(feature = "serde", serde(borrow))] YearNamesMap<'data>),
    /// This calendar is cyclic (Chinese, Dangi), so it uses cyclic year names without any eras
    Cyclic(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
}

icu_provider::data_struct!(
    YearNames<'_>,
    #[cfg(feature = "datagen")]
);

type YearNamesMap<'data> =
    VarZeroCow<'data, Tuple2VarULE<VarZeroSlice<PotentialUtf8>, VarZeroSlice<str>>>;

pub(crate) fn get_year_name_from_map<'a>(
    map: &'a YearNamesMap<'_>,
    year: &PotentialUtf8,
) -> Option<&'a str> {
    let idx = map.a().binary_search_by(|x| x.cmp(year)).ok()?;
    map.b().get(idx)
}

size_test!(MonthNames, month_names_v1_size, 32);

/// Names used for representing the month.
///
/// This uses a data marker attribute for length. See [`YearNames`] for more information on the scheme. This
/// has an additional `1` value used for numeric names, only found for calendars with leap months.
#[doc = month_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum MonthNames<'data> {
    /// Month codes M01, M02, M03, .. (can allow for M13 onwards)
    ///
    /// Found for solar and pure lunar calendars
    Linear(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
    /// Month codes M01, M02, M03, .. M01L, M02L, ...
    ///
    /// Empty entries for non-present month codes. Will have an equal number of leap and non-leap
    /// entries.
    ///
    /// Found for lunisolar and lunisidereal calendars
    LeapLinear(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),

    /// This represents the formatting to apply to numeric values to produce the corresponding
    /// leap month symbol.
    ///
    /// For numeric formatting only, on calendars with leap months
    LeapNumeric(
        #[cfg_attr(
            feature = "serde",
            serde(
                borrow,
                deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
            )
        )]
        Cow<'data, SinglePlaceholderPattern>,
    ),
}

icu_provider::data_struct!(
    MonthNames<'_>,
    #[cfg(feature = "datagen")]
);

size_test!(LinearNames, linear_names_v1_size, 24);

/// Names that can be stored as a simple linear array.
///
/// - For weekdays, element 0 is Sunday
/// - For dayperiods, the elements are in order: AM, PM, (noon), (midnight), where the latter two are optional.
///   In the case noon is missing but midnight is present, the noon value can be the empty string. This is unlikely.
/// - For day names element 0 is the first day of the month
///
/// This uses a data marker attribute for length. See [`YearNames`] for more information on the scheme.
#[doc = linear_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LinearNames<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The names, in order. Order specified on the struct docs.
    // This uses a VarZeroVec rather than a fixed-size array for weekdays to save stack space
    pub names: VarZeroVec<'data, str>,
}

icu_provider::data_struct!(
    LinearNames<'_>,
    #[cfg(feature = "datagen")]
);

impl LinearNames<'_> {
    /// Gets the 'am' name assuming this struct contains day period data.
    pub(crate) fn am(&self) -> Option<&str> {
        self.names.get(0)
    }
    /// Gets the 'pm' name assuming this struct contains day period data.
    pub(crate) fn pm(&self) -> Option<&str> {
        self.names.get(1)
    }
    /// Gets the 'noon' name assuming this struct contains day period data.
    pub(crate) fn noon(&self) -> Option<&str> {
        self.names
            .get(2)
            .and_then(|s| if s.is_empty() { None } else { Some(s) })
    }
    /// Gets the 'midnight' name assuming this struct contains day period data.
    pub(crate) fn midnight(&self) -> Option<&str> {
        self.names.get(3)
    }
}

/// Calendar-agnostic year name data marker
#[derive(Debug)]
pub struct YearNamesV1;
impl DynamicDataMarker for YearNamesV1 {
    type DataStruct = YearNames<'static>;
}

/// Calendar-agnostic month name data marker
#[derive(Debug)]
pub struct MonthNamesV1;
impl DynamicDataMarker for MonthNamesV1 {
    type DataStruct = MonthNames<'static>;
}

/// Re-export of weekday names marker for more consistency
pub use DatetimeNamesWeekdayV1 as WeekdayNamesV1;

/// Re-export of day period names marker for more consistency
pub use DatetimeNamesDayperiodV1 as DayPeriodNamesV1;

size_test!(GluePattern, glue_pattern_v1_size, 24);

/// The default per-length patterns used for combining dates, times, and timezones into formatted strings.
#[doc = glue_pattern_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct GluePattern<'data> {
    /// The pattern
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: runtime::GenericPattern<'data>,
}

icu_provider::data_struct!(
    GluePattern<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `DatetimePatternsGlueV1`
    DatetimePatternsGlueV1,
    GluePattern<'static>
);
