// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod adapter;

use core::ops::Range;

use crate::pattern::runtime;
use alloc::borrow::Cow;
use icu_provider::prelude::*;
use zerovec::ule::UnvalidatedStr;
use zerovec::{VarZeroVec, ZeroMap};

/// Helpers involving the auxiliary subtags used for date symbols.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[allow(missing_docs)]
pub mod aux {
    use icu_locid::extensions::private::{subtag, Subtag};

    pub const NUMERIC: Subtag = subtag!("1");
    pub const ABBR: Subtag = subtag!("3");
    pub const NARROW: Subtag = subtag!("4");
    pub const WIDE: Subtag = subtag!("5");
    pub const SHORT: Subtag = subtag!("6");
    pub const ABBR_STANDALONE: Subtag = subtag!("3s");
    pub const NARROW_STANDALONE: Subtag = subtag!("4s");
    pub const WIDE_STANDALONE: Subtag = subtag!("5s");
    pub const SHORT_STANDALONE: Subtag = subtag!("6s");

    pub const PATTERN_FULL: Subtag = subtag!("f");
    pub const PATTERN_LONG: Subtag = subtag!("l");
    pub const PATTERN_MEDIUM: Subtag = subtag!("m");
    pub const PATTERN_SHORT: Subtag = subtag!("s");

    pub const PATTERN_FULL12: Subtag = subtag!("f12");
    pub const PATTERN_LONG12: Subtag = subtag!("l12");
    pub const PATTERN_MEDIUM12: Subtag = subtag!("m12");
    pub const PATTERN_SHORT12: Subtag = subtag!("s12");

    pub const PATTERN_FULL24: Subtag = subtag!("f24");
    pub const PATTERN_LONG24: Subtag = subtag!("l24");
    pub const PATTERN_MEDIUM24: Subtag = subtag!("m24");
    pub const PATTERN_SHORT24: Subtag = subtag!("s24");

    /// Field lengths supported in auxiliary subtags.
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

    /// Field contexts supported in auxiliary subtags.
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

    /// Parses an aux key subtag to enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn subtag_info(subtag: Subtag) -> Option<(Context, Length)> {
        use {Context::*, Length::*};
        match subtag {
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
}

/// Symbols used for representing the year name
///
/// This uses an auxiliary subtag for length. The subtag is simply the number of
/// characters in the equivalent CLDR field syntax name, plus "s" for standalone contexts. For example,
/// "abbreviated" (e.g. `MMM`) is `-x-3` or `-x-3s` depending on whether it is format or standalone
/// respectively.
///
/// The full list is:
/// - 3 is "abbreviated"
/// - 4 is "narrow"
/// - 5 is "wide"
/// - 6 is "short" (weekdays only)
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
    Eras(#[cfg_attr(feature = "serde", serde(borrow))] ZeroMap<'data, UnvalidatedStr, str>),
    /// This calendar is cyclic (Chinese, Dangi), so it uses cyclic year names without any eras
    Cyclic(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
}

/// Symbols used for representing the month name
///
/// This uses an auxiliary subtag for length. See [`YearSymbolsV1`] for more information on the scheme. This
/// has an additional `-x-1` subtag value used for numeric symbols, only found for calendars with leap months.
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
    pub(crate) fn get_prefix(&self) -> &str {
        self.debug_unwrap_range(0..self.subst_index)
    }
    pub(crate) fn get_suffix(&self) -> &str {
        self.debug_unwrap_range(self.subst_index..self.pattern.len())
    }
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

/// Symbols that can be stored as a simple linear array.
///
/// - For weekdays, element 0 is Sunday
/// - For dayperiods, the elements are in order: AM, PM, (noon), (midnight), where the latter two are optional.
///   In the case noon is missing but midnight is present, the noon value can be the empty string. This is unlikely.
/// - For day names element 0 is the first day of the month
///
/// This uses an auxiliary subtag for length. See [`YearSymbolsV1`] for more information on the scheme.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(WeekdaySymbolsV1Marker, "datetime/symbols/weekdays@1"),
    marker(DayPeriodSymbolsV1Marker, "datetime/symbols/dayperiods@1"),

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
pub struct LinearSymbolsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The symbols, in order. Order specified on the struct docs.
    // This uses a VarZeroVec rather than a fixed-size array for weekdays to save stack space
    pub symbols: VarZeroVec<'data, str>,
}

/// The default per-length patterns associated with dates
///
/// This uses an auxiliary subtag for length. The subtag can be "f", "l", "m", "s" for
/// "full", "long", "medium", or "short".
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

/// The default per-length patterns associated with times
///
/// This uses an auxiliary subtag for length. See [`DatePatternV1`] for more information on the scheme.
///
/// It also uses the subtag to track hour cycles; the data for the default hour cycle will
/// use a regular length auxiliary subtag (e.g. `-x-f` for full), and the non-default
/// one will tack on a `h` or `k` depending on whether it is H11H12 or H23H24
/// (`-x-fk` for full, non-default, 23/24 hours)
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

/// The default per-length patterns used for combining dates and times into datetimes
///
/// This uses an auxiliary subtag for length. See [`DatePatternV1`] for more information on the scheme.
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

// pub(crate) struct ErasedYearSymbolsV1Marker;
// impl DataMarker for ErasedYearSymbolsV1Marker {
//     type Yokeable = YearSymbolsV1<'static>;
// }

// pub(crate) struct ErasedMonthSymbolsV1Marker;
// impl DataMarker for ErasedMonthSymbolsV1Marker {
//     type Yokeable = MonthSymbolsV1<'static>;
// }
