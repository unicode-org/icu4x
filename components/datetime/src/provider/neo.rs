// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod adapter;

use crate::pattern::runtime::{self, PatternULE};
use alloc::borrow::Cow;
use icu_provider::prelude::*;
use zerovec::ule::{AsULE, UnvalidatedStr, ULE};
use zerovec::{VarZeroVec, ZeroMap};

#[cfg(feature = "experimental")]
use crate::neo_skeleton::NeoSkeletonLength;
#[cfg(feature = "experimental")]
use crate::pattern::runtime::PatternBorrowed;
#[cfg(feature = "experimental")]
use core::ops::Range;

/// Helpers involving the data marker attributes used for date symbols.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[allow(missing_docs)]
pub mod marker_attrs {
    use icu_provider::DataMarkerAttributes;

    pub const NUMERIC: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("1");
    pub const ABBR: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("3");
    pub const NARROW: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("4");
    pub const WIDE: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("5");
    pub const SHORT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("6");
    pub const ABBR_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("3s");
    pub const NARROW_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("4s");
    pub const WIDE_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("5s");
    pub const SHORT_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("6s");

    pub const PATTERN_LONG: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("l");
    pub const PATTERN_MEDIUM: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("m");
    pub const PATTERN_SHORT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("s");

    // TODO: The 12-hour and 24-hour DataMarkerAttributes can probably be deleted

    pub const PATTERN_LONG12: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("l12");
    pub const PATTERN_MEDIUM12: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("m12");
    pub const PATTERN_SHORT12: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("s12");

    pub const PATTERN_LONG24: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("l24");
    pub const PATTERN_MEDIUM24: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("m24");
    pub const PATTERN_SHORT24: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("s24");

    pub const PATTERN_LONG_DT: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ldt");
    pub const PATTERN_MEDIUM_DT: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mdt");
    pub const PATTERN_SHORT_DT: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("sdt");

    pub const PATTERN_LONG_DZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ldz");
    pub const PATTERN_MEDIUM_DZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mdz");
    pub const PATTERN_SHORT_DZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("sdz");

    pub const PATTERN_LONG_TZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ltz");
    pub const PATTERN_MEDIUM_TZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mtz");
    pub const PATTERN_SHORT_TZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("stz");

    pub const PATTERN_LONG_DTZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ldtz");
    pub const PATTERN_MEDIUM_DTZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mdtz");
    pub const PATTERN_SHORT_DTZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("sdtz");

    pub const NUMERIC_STR: &str = NUMERIC.as_str();
    pub const ABBR_STR: &str = ABBR.as_str();
    pub const NARROW_STR: &str = NARROW.as_str();
    pub const WIDE_STR: &str = WIDE.as_str();
    pub const SHORT_STR: &str = SHORT.as_str();
    pub const ABBR_STANDALONE_STR: &str = ABBR_STANDALONE.as_str();
    pub const NARROW_STANDALONE_STR: &str = NARROW_STANDALONE.as_str();
    pub const WIDE_STANDALONE_STR: &str = WIDE_STANDALONE.as_str();
    pub const SHORT_STANDALONE_STR: &str = SHORT_STANDALONE.as_str();

    pub const PATTERN_LONG_STR: &str = PATTERN_LONG.as_str();
    pub const PATTERN_MEDIUM_STR: &str = PATTERN_MEDIUM.as_str();
    pub const PATTERN_SHORT_STR: &str = PATTERN_SHORT.as_str();

    pub const PATTERN_LONG12_STR: &str = PATTERN_LONG12.as_str();
    pub const PATTERN_MEDIUM12_STR: &str = PATTERN_MEDIUM12.as_str();
    pub const PATTERN_SHORT12_STR: &str = PATTERN_SHORT12.as_str();

    pub const PATTERN_LONG24_STR: &str = PATTERN_LONG24.as_str();
    pub const PATTERN_MEDIUM24_STR: &str = PATTERN_MEDIUM24.as_str();
    pub const PATTERN_SHORT24_STR: &str = PATTERN_SHORT24.as_str();

    pub const PATTERN_LONG_DT_STR: &str = PATTERN_LONG_DT.as_str();
    pub const PATTERN_MEDIUM_DT_STR: &str = PATTERN_MEDIUM_DT.as_str();
    pub const PATTERN_SHORT_DT_STR: &str = PATTERN_SHORT_DT.as_str();

    pub const PATTERN_LONG_DZ_STR: &str = PATTERN_LONG_DZ.as_str();
    pub const PATTERN_MEDIUM_DZ_STR: &str = PATTERN_MEDIUM_DZ.as_str();
    pub const PATTERN_SHORT_DZ_STR: &str = PATTERN_SHORT_DZ.as_str();

    pub const PATTERN_LONG_TZ_STR: &str = PATTERN_LONG_TZ.as_str();
    pub const PATTERN_MEDIUM_TZ_STR: &str = PATTERN_MEDIUM_TZ.as_str();
    pub const PATTERN_SHORT_TZ_STR: &str = PATTERN_SHORT_TZ.as_str();

    pub const PATTERN_LONG_DTZ_STR: &str = PATTERN_LONG_DTZ.as_str();
    pub const PATTERN_MEDIUM_DTZ_STR: &str = PATTERN_MEDIUM_DTZ.as_str();
    pub const PATTERN_SHORT_DTZ_STR: &str = PATTERN_SHORT_DTZ.as_str();

    /// Field lengths supported in data marker attribute.
    ///
    /// For a stable version of this enum, use [`FieldLength`].
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`FieldLength`]: crate::fields::FieldLength
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(clippy::exhaustive_enums)] // documented as unstable
    pub enum Length {
        Abbr,
        Narrow,
        Wide,
        Short,
        Numeric,
    }

    /// Pattern lengths supported in data marker attributes.
    ///
    /// For a stable version of this enum, use [`length::Date`] or [`length::Time`].
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`length::Date`]: crate::options::length::Date
    /// [`length::Time`]: crate::options::length::Time
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum PatternLength {
        Long,
        Medium,
        Short,
    }

    /// Field contexts supported in data marker attributes.
    ///
    /// For a stable version of this enum, use one of the specific field symbol enums in [`fields`].
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`fields`]: crate::fields
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(clippy::exhaustive_enums)] // documented as unstable
    pub enum Context {
        Format,
        Standalone,
    }

    /// Date, time, and time zone combinations supported in data marker attributes.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`fields`]: crate::fields
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(clippy::exhaustive_enums)] // documented as unstable
    pub enum GlueType {
        DateTime,
        DateZone,
        TimeZone,
        DateTimeZone,
    }

    /// Parses a symbol data marker attribute to enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn symbol_marker_attr_info(
        marker_attr: &DataMarkerAttributes,
    ) -> Option<(Context, Length)> {
        use {Context::*, Length::*};
        match &**marker_attr {
            NUMERIC_STR => Some((Format, Numeric)),
            ABBR_STR => Some((Format, Abbr)),
            NARROW_STR => Some((Format, Narrow)),
            WIDE_STR => Some((Format, Wide)),
            SHORT_STR => Some((Format, Short)),
            ABBR_STANDALONE_STR => Some((Standalone, Abbr)),
            NARROW_STANDALONE_STR => Some((Standalone, Narrow)),
            WIDE_STANDALONE_STR => Some((Standalone, Wide)),
            SHORT_STANDALONE_STR => Some((Standalone, Short)),
            _ => None,
        }
    }

    /// Parses a pattern data marker attribute to enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn pattern_marker_attr_info_for_glue(
        marker_attr: &DataMarkerAttributes,
    ) -> Option<(PatternLength, GlueType)> {
        use {GlueType::*, PatternLength::*};
        match &**marker_attr {
            PATTERN_LONG_DT_STR => Some((Long, DateTime)),
            PATTERN_MEDIUM_DT_STR => Some((Medium, DateTime)),
            PATTERN_SHORT_DT_STR => Some((Short, DateTime)),

            PATTERN_LONG_DZ_STR => Some((Long, DateZone)),
            PATTERN_MEDIUM_DZ_STR => Some((Medium, DateZone)),
            PATTERN_SHORT_DZ_STR => Some((Short, DateZone)),

            PATTERN_LONG_TZ_STR => Some((Long, TimeZone)),
            PATTERN_MEDIUM_TZ_STR => Some((Medium, TimeZone)),
            PATTERN_SHORT_TZ_STR => Some((Short, TimeZone)),

            PATTERN_LONG_DTZ_STR => Some((Long, DateTimeZone)),
            PATTERN_MEDIUM_DTZ_STR => Some((Medium, DateTimeZone)),
            PATTERN_SHORT_DTZ_STR => Some((Short, DateTimeZone)),

            _ => None,
        }
    }

    /// Creates a symbol data marker attribute from the enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn symbol_attr_for(context: Context, length: Length) -> &'static DataMarkerAttributes {
        use {Context::*, Length::*};
        match (context, length) {
            (Format, Numeric) => NUMERIC,
            (Format, Abbr) => ABBR,
            (Format, Narrow) => NARROW,
            (Format, Wide) => WIDE,
            (Format, Short) => SHORT,
            (Standalone, Numeric) => NUMERIC,
            (Standalone, Abbr) => ABBR_STANDALONE,
            (Standalone, Narrow) => NARROW_STANDALONE,
            (Standalone, Wide) => WIDE_STANDALONE,
            (Standalone, Short) => SHORT_STANDALONE,
        }
    }

    pub fn pattern_marker_attr_for_glue(
        length: PatternLength,
        glue_type: GlueType,
    ) -> &'static DataMarkerAttributes {
        use {GlueType::*, PatternLength::*};
        match (length, glue_type) {
            (Long, DateTime) => PATTERN_LONG_DT,
            (Medium, DateTime) => PATTERN_MEDIUM_DT,
            (Short, DateTime) => PATTERN_SHORT_DT,

            (Long, DateZone) => PATTERN_LONG_DZ,
            (Medium, DateZone) => PATTERN_MEDIUM_DZ,
            (Short, DateZone) => PATTERN_SHORT_DZ,

            (Long, TimeZone) => PATTERN_LONG_TZ,
            (Medium, TimeZone) => PATTERN_MEDIUM_TZ,
            (Short, TimeZone) => PATTERN_SHORT_TZ,

            (Long, DateTimeZone) => PATTERN_LONG_DTZ,
            (Medium, DateTimeZone) => PATTERN_MEDIUM_DTZ,
            (Short, DateTimeZone) => PATTERN_SHORT_DTZ,
        }
    }
}

size_test!(YearNamesV1, year_names_v1_size, 48);

/// Symbols used for representing the year name
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
#[icu_provider::data_struct(
    marker(BuddhistYearNamesV1Marker, "datetime/symbols/buddhist/years@1"),
    marker(ChineseYearNamesV1Marker, "datetime/symbols/chinese/years@1"),
    marker(CopticYearNamesV1Marker, "datetime/symbols/coptic/years@1"),
    marker(DangiYearNamesV1Marker, "datetime/symbols/dangi/years@1"),
    marker(EthiopianYearNamesV1Marker, "datetime/symbols/ethiopic/years@1"),
    marker(GregorianYearNamesV1Marker, "datetime/symbols/gregory/years@1"),
    marker(HebrewYearNamesV1Marker, "datetime/symbols/hebrew/years@1"),
    marker(IndianYearNamesV1Marker, "datetime/symbols/indian/years@1"),
    marker(IslamicYearNamesV1Marker, "datetime/symbols/islamic/years@1"),
    marker(JapaneseYearNamesV1Marker, "datetime/symbols/japanese/years@1"),
    marker(JapaneseExtendedYearNamesV1Marker, "datetime/symbols/japanext/years@1"),
    marker(PersianYearNamesV1Marker, "datetime/symbols/persian/years@1"),
    marker(RocYearNamesV1Marker, "datetime/symbols/roc/years@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum YearNamesV1<'data> {
    /// This calendar uses eras with numeric years, this stores the era names mapped from
    /// era code to the name
    Eras(#[cfg_attr(feature = "serde", serde(borrow))] ZeroMap<'data, UnvalidatedStr, str>),
    /// This calendar is cyclic (Chinese, Dangi), so it uses cyclic year names without any eras
    Cyclic(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
}

size_test!(MonthNamesV1, month_names_v1_size, 32);

/// Symbols used for representing the month name
///
/// This uses a data marker attribute for length. See [`YearNamesV1`] for more information on the scheme. This
/// has an additional `1` value used for numeric symbols, only found for calendars with leap months.
#[doc = month_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(BuddhistMonthNamesV1Marker, "datetime/symbols/buddhist/months@1"),
    marker(ChineseMonthNamesV1Marker, "datetime/symbols/chinese/months@1"),
    marker(CopticMonthNamesV1Marker, "datetime/symbols/coptic/months@1"),
    marker(DangiMonthNamesV1Marker, "datetime/symbols/dangi/months@1"),
    marker(EthiopianMonthNamesV1Marker, "datetime/symbols/ethiopic/months@1"),
    marker(GregorianMonthNamesV1Marker, "datetime/symbols/gregory/months@1"),
    marker(HebrewMonthNamesV1Marker, "datetime/symbols/hebrew/months@1"),
    marker(IndianMonthNamesV1Marker, "datetime/symbols/indian/months@1"),
    marker(IslamicMonthNamesV1Marker, "datetime/symbols/islamic/months@1"),
    marker(JapaneseMonthNamesV1Marker, "datetime/symbols/japanese/months@1"),
    marker(
        JapaneseExtendedMonthNamesV1Marker,
        "datetime/symbols/japanext/months@1"
    ),
    marker(PersianMonthNamesV1Marker, "datetime/symbols/persian/months@1"),
    marker(RocMonthNamesV1Marker, "datetime/symbols/roc/months@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum MonthNamesV1<'data> {
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
    LeapNumeric(#[cfg_attr(feature = "serde", serde(borrow))] SimpleSubstitutionPattern<'data>),
}

/// Represents a simple substitution pattern;
/// i.e. a string with a single placeholder
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
 feature = "datagen",
 derive(serde::Serialize, databake::Bake),
 databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SimpleSubstitutionPattern<'data> {
    /// The pattern
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: Cow<'data, str>,
    /// The byte index in which to substitute stuff. Weak invariant: `subst_index <= pattern.len()`
    pub subst_index: usize,
}

impl SimpleSubstitutionPattern<'_> {
    #[cfg(feature = "experimental")]
    pub(crate) fn get_prefix(&self) -> &str {
        self.debug_unwrap_range(0..self.subst_index)
    }
    #[cfg(feature = "experimental")]
    pub(crate) fn get_suffix(&self) -> &str {
        self.debug_unwrap_range(self.subst_index..self.pattern.len())
    }
    #[cfg(feature = "experimental")]
    fn debug_unwrap_range(&self, range: Range<usize>) -> &str {
        match self.pattern.get(range) {
            Some(s) => s,
            None => {
                debug_assert!(false, "Invalid pattern: {self:?}");
                ""
            }
        }
    }
}

size_test!(LinearNamesV1, linear_names_v1_size, 24);

/// Symbols that can be stored as a simple linear array.
///
/// - For weekdays, element 0 is Sunday
/// - For dayperiods, the elements are in order: AM, PM, (noon), (midnight), where the latter two are optional.
///   In the case noon is missing but midnight is present, the noon value can be the empty string. This is unlikely.
/// - For day names element 0 is the first day of the month
///
/// This uses a data marker attribute for length. See [`YearNamesV1`] for more information on the scheme.
#[doc = linear_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(WeekdayNamesV1Marker, "datetime/symbols/weekdays@1"),
    marker(DayPeriodNamesV1Marker, "datetime/symbols/dayperiods@1"),

    // We're not producing or using day symbols yet, but this is where they would go
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
pub struct LinearNamesV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The symbols, in order. Order specified on the struct docs.
    // This uses a VarZeroVec rather than a fixed-size array for weekdays to save stack space
    pub symbols: VarZeroVec<'data, str>,
}

impl<'data> LinearNamesV1<'data> {
    /// Gets the 'am' symbol assuming this struct contains day period data.
    #[cfg(feature = "experimental")]
    pub(crate) fn am(&self) -> Option<&str> {
        self.symbols.get(0)
    }
    /// Gets the 'pm' symbol assuming this struct contains day period data.
    #[cfg(feature = "experimental")]
    pub(crate) fn pm(&self) -> Option<&str> {
        self.symbols.get(1)
    }
    /// Gets the 'noon' symbol assuming this struct contains day period data.
    #[cfg(feature = "experimental")]
    pub(crate) fn noon(&self) -> Option<&str> {
        self.symbols
            .get(2)
            .and_then(|s| if s.is_empty() { None } else { Some(s) })
    }
    /// Gets the 'midnight' symbol assuming this struct contains day period data.
    #[cfg(feature = "experimental")]
    pub(crate) fn midnight(&self) -> Option<&str> {
        self.symbols.get(3)
    }
}

// TODO: We may need to support plural forms here. Something like
// pub enum NeoPatternPlurals<'data> {
//     SingleDate(runtime::Pattern<'data>),
//     WeekPlurals(ZeroMap<'data, PluralCategory, runtime::PatternULE>),
// }

size_test!(GluePatternV1, glue_pattern_v1_size, 24);

/// The default per-length patterns used for combining dates, times, and timezones into formatted strings.
///
#[doc = glue_pattern_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(GluePatternV1Marker, "datetime/patterns/glue@1"))]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct GluePatternV1<'data> {
    /// The pattern
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: runtime::GenericPattern<'data>,
}

// conceptually:
// {
//   has_long: bool,
//   has_medium: bool,
//   has_short: bool,
//   index: u16, // index of first pattern (long if present, else medium, else short)
// }
#[allow(missing_docs)] // TODO
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SkeletonDataIndex {
    /// If true, the first pattern is for `Long`.
    /// If false, fall back to the next pattern (`Medium``).
    pub has_long: bool,
    /// If true, the next pattern is for `Long`.
    /// If false, fall back to the next pattern (`Short`).
    pub has_medium: bool,
    /// If true, there are 6 plural variants for each pattern.
    /// If false, it is just a single variant.
    pub has_plurals: bool,
    /// If true, there are 2 era variants for each pattern.
    /// The first does not have an era, and the second has an era.
    /// if false, there is no era variant.
    pub has_eras: bool,
}

#[cfg(feature = "experimental")]
#[derive(Debug, Copy, Clone)]
pub(crate) struct PatternSelectionOptions {
    pub(crate) length: NeoSkeletonLength,
    pub(crate) should_display_era: Option<bool>,
}

impl SkeletonDataIndex {
    // TODO: This should handle plurals
    #[cfg(feature = "experimental")]
    pub(crate) fn index_for(self, options: PatternSelectionOptions) -> u8 {
        let chunk_number = match (options.length, self.has_long, self.has_medium) {
            (NeoSkeletonLength::Long, _, _) => 0,
            (NeoSkeletonLength::Medium, true, _) => 1,
            (NeoSkeletonLength::Medium, false, _) => 0,
            (NeoSkeletonLength::Short, true, true) => 2,
            (NeoSkeletonLength::Short, true, false) => 1,
            (NeoSkeletonLength::Short, false, true) => 1,
            (NeoSkeletonLength::Short, false, false) => 0,
        };
        if !self.has_eras {
            // chunks are size 1
            return chunk_number;
        }
        let offset = match options.should_display_era {
            Some(false) => 0,
            Some(true) | None => 1,
        };
        let chunk_size = 2;
        offset + chunk_number * chunk_size
    }
}

/// Bit-packed [`ULE`] variant of [`SkeletonDataIndex`].
#[derive(Debug, Copy, Clone, ULE)]
#[repr(transparent)]
pub struct SkeletonDataIndexULE(u8);

impl AsULE for SkeletonDataIndex {
    type ULE = SkeletonDataIndexULE;

    fn to_unaligned(self) -> Self::ULE {
        let mut flags = 0;
        flags |= (self.has_long as u8) << 7;
        flags |= (self.has_medium as u8) << 6;
        flags |= (self.has_plurals as u8) << 5;
        flags |= (self.has_eras as u8) << 4;
        SkeletonDataIndexULE(flags)
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let flags = unaligned.0;
        // TODO: `flags` could have more bits set, but we don't check that here.
        SkeletonDataIndex {
            has_long: (flags & (1 << 7)) != 0,
            has_medium: (flags & (1 << 6)) != 0,
            has_plurals: (flags & (1 << 5)) != 0,
            has_eras: (flags & (1 << 4)) != 0,
        }
    }
}

#[icu_provider::data_struct(
    // date patterns
    marker(BuddhistDateNeoSkeletonPatternsV1Marker, "datetime/patterns/buddhist/skeleton@1"),
    marker(ChineseDateNeoSkeletonPatternsV1Marker, "datetime/patterns/chinese/skeleton@1"),
    marker(CopticDateNeoSkeletonPatternsV1Marker, "datetime/patterns/coptic/skeleton@1"),
    marker(DangiDateNeoSkeletonPatternsV1Marker, "datetime/patterns/dangi/skeleton@1"),
    marker(EthiopianDateNeoSkeletonPatternsV1Marker, "datetime/patterns/ethiopic/skeleton@1"),
    marker(GregorianDateNeoSkeletonPatternsV1Marker, "datetime/patterns/gregory/skeleton@1"),
    marker(HebrewDateNeoSkeletonPatternsV1Marker, "datetime/patterns/hebrew/skeleton@1"),
    marker(IndianDateNeoSkeletonPatternsV1Marker, "datetime/patterns/indian/skeleton@1"),
    marker(IslamicDateNeoSkeletonPatternsV1Marker, "datetime/patterns/islamic/skeleton@1"),
    marker(JapaneseDateNeoSkeletonPatternsV1Marker, "datetime/patterns/japanese/skeleton@1"),
    marker(JapaneseExtendedDateNeoSkeletonPatternsV1Marker, "datetime/patterns/japanext/skeleton@1"),
    marker(PersianDateNeoSkeletonPatternsV1Marker, "datetime/patterns/persian/skeleton@1"),
    marker(RocDateNeoSkeletonPatternsV1Marker, "datetime/patterns/roc/skeleton@1"),
    // Time patterns
    marker(TimeNeoSkeletonPatternsV1Marker, "datetime/patterns/time_skeleton@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(missing_docs)] // TODO
pub struct PackedSkeletonDataV1<'data> {
    #[allow(missing_docs)] // TODO
    // TODO: Use the bitpacked version here
    pub index_info: SkeletonDataIndex,
    // TODO: This should support plurals
    #[allow(missing_docs)] // TODO
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, PatternULE>,
}

impl<'data> PackedSkeletonDataV1<'data> {
    #[cfg(feature = "experimental")]
    /// Gets a pattern according to a length and a numeric variant.
    pub(crate) fn get_pattern(&self, options: PatternSelectionOptions) -> PatternBorrowed {
        match self
            .patterns
            .get(self.index_info.index_for(options) as usize)
        {
            Some(pattern_ule) => pattern_ule.as_borrowed(),
            None => {
                debug_assert!(false, "failed to load a pattern for {options:?}");
                PatternBorrowed::DEFAULT
            }
        }
    }
}

#[icu_provider::data_struct(marker(
    DateTimeSkeletonPatternsV1Marker,
    "datetime/patterns/datetime_skeleton@1"
))]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
#[allow(missing_docs)] // TODO
pub struct DateTimeSkeletonsV1<'data> {
    // will typically be small, there are only a couple special cases like E B h m
    // TODO: This should support plurals
    // TODO: The key of this map should be Skeleton
    #[allow(missing_docs)] // TODO
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroMap<'data, str, PatternULE>,
}

/// Calendar-agnostic year name data marker
#[derive(Debug)]
pub struct YearNamesV1Marker;
impl DynamicDataMarker for YearNamesV1Marker {
    type DataStruct = YearNamesV1<'static>;
}

/// Calendar-agnostic month name data marker
#[derive(Debug)]
pub struct MonthNamesV1Marker;
impl DynamicDataMarker for MonthNamesV1Marker {
    type DataStruct = MonthNamesV1<'static>;
}

/// Calendar-agnostic date/time skeleta data marker
#[derive(Debug)]
pub struct SkeletaV1Marker;
impl DynamicDataMarker for SkeletaV1Marker {
    type DataStruct = PackedSkeletonDataV1<'static>;
}
