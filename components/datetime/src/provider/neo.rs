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
    use crate::pattern::CoarseHourCycle;
    use tinystr::{tinystr, TinyAsciiStr};

    pub const NUMERIC: TinyAsciiStr<8> = tinystr!(8, "1");
    pub const ABBR: TinyAsciiStr<8> = tinystr!(8, "3");
    pub const NARROW: TinyAsciiStr<8> = tinystr!(8, "4");
    pub const WIDE: TinyAsciiStr<8> = tinystr!(8, "5");
    pub const SHORT: TinyAsciiStr<8> = tinystr!(8, "6");
    pub const ABBR_STANDALONE: TinyAsciiStr<8> = tinystr!(8, "3s");
    pub const NARROW_STANDALONE: TinyAsciiStr<8> = tinystr!(8, "4s");
    pub const WIDE_STANDALONE: TinyAsciiStr<8> = tinystr!(8, "5s");
    pub const SHORT_STANDALONE: TinyAsciiStr<8> = tinystr!(8, "6s");

    pub const PATTERN_FULL: TinyAsciiStr<8> = tinystr!(8, "f");
    pub const PATTERN_LONG: TinyAsciiStr<8> = tinystr!(8, "l");
    pub const PATTERN_MEDIUM: TinyAsciiStr<8> = tinystr!(8, "m");
    pub const PATTERN_SHORT: TinyAsciiStr<8> = tinystr!(8, "s");

    pub const PATTERN_FULL12: TinyAsciiStr<8> = tinystr!(8, "f12");
    pub const PATTERN_LONG12: TinyAsciiStr<8> = tinystr!(8, "l12");
    pub const PATTERN_MEDIUM12: TinyAsciiStr<8> = tinystr!(8, "m12");
    pub const PATTERN_SHORT12: TinyAsciiStr<8> = tinystr!(8, "s12");

    pub const PATTERN_FULL24: TinyAsciiStr<8> = tinystr!(8, "f24");
    pub const PATTERN_LONG24: TinyAsciiStr<8> = tinystr!(8, "l24");
    pub const PATTERN_MEDIUM24: TinyAsciiStr<8> = tinystr!(8, "m24");
    pub const PATTERN_SHORT24: TinyAsciiStr<8> = tinystr!(8, "s24");

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
        Full,
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

    /// Parses a symbol data marker attribute to enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn symbol_marker_attr_info(marker_attr: TinyAsciiStr<8>) -> Option<(Context, Length)> {
        use {Context::*, Length::*};
        match marker_attr {
            NUMERIC => Some((Format, Numeric)),
            ABBR => Some((Format, Abbr)),
            NARROW => Some((Format, Narrow)),
            WIDE => Some((Format, Wide)),
            SHORT => Some((Format, Short)),
            ABBR_STANDALONE => Some((Standalone, Abbr)),
            NARROW_STANDALONE => Some((Standalone, Narrow)),
            WIDE_STANDALONE => Some((Standalone, Wide)),
            SHORT_STANDALONE => Some((Standalone, Short)),
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
    pub fn pattern_marker_attr_info(
        marker_attr: TinyAsciiStr<8>,
    ) -> Option<(PatternLength, Option<CoarseHourCycle>)> {
        use {CoarseHourCycle::*, PatternLength::*};
        match marker_attr {
            PATTERN_FULL => Some((Full, None)),
            PATTERN_LONG => Some((Long, None)),
            PATTERN_MEDIUM => Some((Medium, None)),
            PATTERN_SHORT => Some((Short, None)),

            PATTERN_FULL12 => Some((Full, Some(H11H12))),
            PATTERN_LONG12 => Some((Long, Some(H11H12))),
            PATTERN_MEDIUM12 => Some((Medium, Some(H11H12))),
            PATTERN_SHORT12 => Some((Short, Some(H11H12))),

            PATTERN_FULL24 => Some((Full, Some(H23H24))),
            PATTERN_LONG24 => Some((Long, Some(H23H24))),
            PATTERN_MEDIUM24 => Some((Medium, Some(H23H24))),
            PATTERN_SHORT24 => Some((Short, Some(H23H24))),
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
    pub fn symbol_attr_for(context: Context, length: Length) -> TinyAsciiStr<8> {
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

    /// Creates a pattern data marker attribute from the enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn pattern_marker_attr_for(
        length: PatternLength,
        hour_cycle: Option<CoarseHourCycle>,
    ) -> TinyAsciiStr<8> {
        use {CoarseHourCycle::*, PatternLength::*};
        match (length, hour_cycle) {
            (Full, None) => PATTERN_FULL,
            (Long, None) => PATTERN_LONG,
            (Medium, None) => PATTERN_MEDIUM,
            (Short, None) => PATTERN_SHORT,

            (Full, Some(H11H12)) => PATTERN_FULL12,
            (Long, Some(H11H12)) => PATTERN_LONG12,
            (Medium, Some(H11H12)) => PATTERN_MEDIUM12,
            (Short, Some(H11H12)) => PATTERN_SHORT12,

            (Full, Some(H23H24)) => PATTERN_FULL24,
            (Long, Some(H23H24)) => PATTERN_LONG24,
            (Medium, Some(H23H24)) => PATTERN_MEDIUM24,
            (Short, Some(H23H24)) => PATTERN_SHORT24,
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

size_test!(DatePatternV1, date_pattern_v1_size, 32);

/// The default per-length patterns associated with dates
///
/// This uses a data marker attribute for length. The value can be "f", "l", "m", "s" for
/// "full", "long", "medium", or "short".
#[doc = date_pattern_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    // date patterns
    marker(BuddhistDatePatternV1Marker, "datetime/patterns/buddhist/date@1"),
    marker(ChineseDatePatternV1Marker, "datetime/patterns/chinese/date@1"),
    marker(CopticDatePatternV1Marker, "datetime/patterns/coptic/date@1"),
    marker(DangiDatePatternV1Marker, "datetime/patterns/dangi/date@1"),
    marker(EthiopianDatePatternV1Marker, "datetime/patterns/ethiopic/date@1"),
    marker(GregorianDatePatternV1Marker, "datetime/patterns/gregory/date@1"),
    marker(HebrewDatePatternV1Marker, "datetime/patterns/hebrew/date@1"),
    marker(IndianDatePatternV1Marker, "datetime/patterns/indian/date@1"),
    marker(IslamicDatePatternV1Marker, "datetime/patterns/islamic/date@1"),
    marker(JapaneseDatePatternV1Marker, "datetime/patterns/japanese/date@1"),
    marker(JapaneseExtendedDatePatternV1Marker, "datetime/patterns/japanext/date@1"),
    marker(PersianDatePatternV1Marker, "datetime/patterns/persian/date@1"),
    marker(RocDatePatternV1Marker, "datetime/patterns/roc/date@1")
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DatePatternV1<'data> {
    /// The pattern
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: runtime::Pattern<'data>,
}

// TODO: We may need to support plural forms here. Something like
// pub enum NeoPatternPlurals<'data> {
//     SingleDate(runtime::Pattern<'data>),
//     WeekPlurals(ZeroMap<'data, PluralCategory, runtime::PatternULE>),
// }

size_test!(TimePatternV1, time_pattern_v1_size, 32);

/// The default per-length patterns associated with times
///
/// This uses an data marker attribute for length. See [`DatePatternV1`] for more information on the scheme.
///
/// It also uses the attribute to track hour cycles; the data for the default hour cycle will
/// use a regular length attribute (e.g. `f` for full), and the non-default
/// one will tack on a `h` or `k` depending on whether it is H11H12 or H23H24
/// (`fk` for full, non-default, 23/24 hours)
#[doc = time_pattern_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(TimePatternV1Marker, "datetime/patterns/time@1"))]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::neo),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimePatternV1<'data> {
    /// The pattern
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: runtime::Pattern<'data>,
}

size_test!(DateTimePatternV1, date_time_pattern_v1_size, 24);

/// The default per-length patterns used for combining dates and times into datetimes
///
/// This uses a data marker attribute for length. See [`DatePatternV1`] for more information on the scheme.
#[doc = date_time_pattern_v1_size!()]
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
}

impl SkeletonDataIndex {
    // TODO: This should handle plurals
    #[cfg(feature = "experimental")]
    pub(crate) fn index_for(self, length: NeoSkeletonLength) -> u8 {
        match (length, self.has_long, self.has_medium) {
            (NeoSkeletonLength::Long, _, _) => 0,
            (NeoSkeletonLength::Medium, true, _) => 1,
            (NeoSkeletonLength::Medium, false, _) => 0,
            (NeoSkeletonLength::Short, true, true) => 2,
            (NeoSkeletonLength::Short, true, false) => 1,
            (NeoSkeletonLength::Short, false, true) => 1,
            (NeoSkeletonLength::Short, false, false) => 0,
        }
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
        SkeletonDataIndexULE(flags)
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let flags = unaligned.0;
        // TODO: `flags` could have more bits set, but we don't check that here.
        SkeletonDataIndex {
            has_long: (flags & (1 << 7)) != 0,
            has_medium: (flags & (1 << 6)) != 0,
            has_plurals: (flags & (1 << 5)) != 0,
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
    // TODO: Handle plurals
    pub(crate) fn get_pattern(&self, length: NeoSkeletonLength) -> PatternBorrowed {
        match self
            .patterns
            .get(self.index_info.index_for(length) as usize)
        {
            Some(pattern_ule) => pattern_ule.as_borrowed(),
            None => {
                debug_assert!(false, "failed to load a pattern for length {length:?}");
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
impl DynDataMarker for YearNamesV1Marker {
    type Yokeable = YearNamesV1<'static>;
}

/// Calendar-agnostic month name data marker
#[derive(Debug)]
pub struct MonthNamesV1Marker;
impl DynDataMarker for MonthNamesV1Marker {
    type Yokeable = MonthNamesV1<'static>;
}

/// Calendar-agnostic date pattern data marker
#[derive(Debug)]
pub struct DatePatternV1Marker;
impl DynDataMarker for DatePatternV1Marker {
    type Yokeable = DatePatternV1<'static>;
}

/// Calendar-agnostic date skeleta data marker
#[derive(Debug)]
pub struct SkeletaV1Marker;
impl DynDataMarker for SkeletaV1Marker {
    type Yokeable = PackedSkeletonDataV1<'static>;
}
