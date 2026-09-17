// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Individual field enums for a [`DateTimeFieldBag`](super::DateTimeFieldBag).

/// Options for the era length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Era {
    /// Example: AD
    ///
    /// Skeleton: `G`
    ///
    /// # Examples
    ///
    /// FIXME: Add a similar test to all enum variants
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.era = Some(Era::Short);
    ///
    /// assert_writeable_eq!(bag, "G");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Short,
    /// Example: Anno Domini
    ///
    /// Skeleton: `GGGG`
    Long,
    /// Example: A
    ///
    /// Skeleton: `GGGGG`
    Narrow,
}

/// Options for the year length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Year {
    /// Skeleton: `y`
    Numeric,
    /// Skeleton: `yy`
    TwoDigit,
}

/// Options for the month length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Month {
    /// Example: 3
    ///
    /// Skeleton: `M`
    Numeric,
    /// Example: 03
    ///
    /// Skeleton: `MM`
    TwoDigit,
    /// Example: Mar
    ///
    /// Skeleton: `MMM`
    Short,
    /// Example: March
    ///
    /// Skeleton: `MMMM`
    Long,
    /// Example: M
    ///
    /// Note: Two weekdays may have the same narrow style for some locales. For example,
    /// both March's and May's narrow styles are M in the en-US locale. If this is
    /// not acceptable, use [`Self::Short`].
    ///
    /// Skeleton: `MMMMM`
    Narrow,
}

/// Options for the day of month length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Day {
    /// Example: 8
    ///
    /// Skeleton: `d`
    Numeric,
    /// Example: 08
    ///
    /// Skeleton: `dd`
    TwoDigit,
}

/// Options for the weekday length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Weekday {
    /// Example: Thu
    ///
    /// Skeleton: `E`
    Short,
    /// Example: Thursday
    ///
    /// Skeleton: `EEEE`
    Long,
    /// Example: T
    ///
    /// Note: Two weekdays may have the same narrow style for some locales. For example,
    /// both Tuesday's and Thursday's narrow styles are T in the en-US locale. If this is
    /// not acceptable, use [`Self::Short`].
    ///
    /// Skeleton: `EEEEE`
    Narrow,
}

/// Options for the day period length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DayPeriod {
    /// Example: in the evening
    ///
    /// Skeleton: `B`
    FlexibleShort,
    /// Example: in the evening
    ///
    /// Skeleton: `BBBB`
    FlexibleLong,
    /// Example: in the evening
    ///
    /// Skeleton: `BBBBB`
    FlexibleNarrow,
}

/// Options for the kind of hour symbol, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HourKind {
    /// Example: 4
    ///
    /// Skeleton: `h` or `hh`
    ///
    /// This corresponds to `hour12: true` in ECMA-402.
    Clock12,
    /// Example: 16
    ///
    /// Skeleton: `H` or `HH`
    ///
    /// This corresponds to `hour12: false` in ECMA-402.
    Clock24,
}

/// Options for the hour length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Hour {
    /// Example: 6
    ///
    /// Skeleton: `h`, `H`, or `j` (depending on [`HourKind`])
    Numeric,
    /// Example: 06
    ///
    /// Skeleton: `hh`, `HH`, or `jj` (depending on [`HourKind`])
    TwoDigit,
}

/// Options for the minute length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Minute {
    /// Example: 5
    ///
    /// Skeleton: `m`
    Numeric,
    /// Example: 05
    ///
    /// Skeleton: `mm`
    TwoDigit,
}

/// Options for the second length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Second {
    /// Example: 2
    ///
    /// Skeleton: `s`
    Numeric,
    /// Example: 02
    ///
    /// Skeleton: `ss`
    TwoDigit,
}

/// Options for the fractional second digits, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FractionalSecondDigits {
    /// One fraction digit for seconds.
    ///
    /// Skeleton: `S`
    F1,
    /// Two fraction digits for seconds.
    ///
    /// Skeleton: `SS`
    F2,
    /// Three fraction digits for seconds.
    ///
    /// Skeleton: `SSS`
    F3,
}

/// Options for the time zone name, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeZoneName {
    /// Short localized form (example: PST, GMT-8).
    ///
    /// Skeleton: `z`
    ///
    /// This corresponds to the "short" option in ECMA-402.
    ShortSpecific,
    /// Long localized form (example: Pacific Standard Time, Nordamerikanische Westküsten-Normalzeit).
    ///
    /// Skeleton: `zzzz`
    ///
    /// This corresponds to the "long" option in ECMA-402.
    LongSpecific,
    /// Short localized GMT format (example: GMT-8).
    ///
    /// Skeleton: `O`
    ShortOffset,
    /// Long localized GMT format (example: GMT-08:00).
    ///
    /// Skeleton: `OOOO`
    LongOffset,
    /// Short generic non-location format (example: PT, Los Angeles Zeit).
    ///
    /// Skeleton: `v`
    ShortGeneric,
    /// Long generic non-location format (example: Pacific Time, Nordamerikanische Westküstenzeit).
    ///
    /// Skeleton: `vvvv`
    LongGeneric,
}
