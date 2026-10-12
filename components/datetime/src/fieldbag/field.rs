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
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.era = Some(Era::Long);
    ///
    /// assert_writeable_eq!(bag, "GGGG");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Long,
    /// Example: A
    ///
    /// Skeleton: `GGGGG`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.era = Some(Era::Narrow);
    ///
    /// assert_writeable_eq!(bag, "GGGGG");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Narrow,
}

impl Era {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Short, Self::Long, Self::Narrow];
}

/// Options for the year length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Year {
    /// Example: 2003
    ///
    /// Skeleton: `y`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Year;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.year = Some(Year::Numeric);
    ///
    /// assert_writeable_eq!(bag, "y");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Numeric,
    /// Example: 03
    ///
    /// Skeleton: `yy`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Year;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.year = Some(Year::TwoDigit);
    ///
    /// assert_writeable_eq!(bag, "yy");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    TwoDigit,
}

impl Year {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Numeric, Self::TwoDigit];
}

/// Options for the month length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Month {
    /// Example: 3
    ///
    /// Skeleton: `M`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Month;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.month = Some(Month::Numeric);
    ///
    /// assert_writeable_eq!(bag, "M");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Numeric,
    /// Example: 03
    ///
    /// Skeleton: `MM`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Month;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.month = Some(Month::TwoDigit);
    ///
    /// assert_writeable_eq!(bag, "MM");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    TwoDigit,
    /// Example: Mar
    ///
    /// Skeleton: `MMM`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Month;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.month = Some(Month::Short);
    ///
    /// assert_writeable_eq!(bag, "MMM");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Short,
    /// Example: March
    ///
    /// Skeleton: `MMMM`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Month;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.month = Some(Month::Long);
    ///
    /// assert_writeable_eq!(bag, "MMMM");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Long,
    /// Example: M
    ///
    /// Note: Two months may have the same narrow style for some locales. For example,
    /// both March's and May's narrow styles are M in the en-US locale. If this is
    /// not acceptable, use [`Self::Short`].
    ///
    /// Skeleton: `MMMMM`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Month;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.month = Some(Month::Narrow);
    ///
    /// assert_writeable_eq!(bag, "MMMMM");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Narrow,
}

impl Month {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[
        Self::Numeric,
        Self::TwoDigit,
        Self::Short,
        Self::Long,
        Self::Narrow,
    ];
}

/// Options for the day of month length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Day {
    /// Example: 8
    ///
    /// Skeleton: `d`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Day;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.day = Some(Day::Numeric);
    ///
    /// assert_writeable_eq!(bag, "d");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Numeric,
    /// Example: 08
    ///
    /// Skeleton: `dd`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Day;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.day = Some(Day::TwoDigit);
    ///
    /// assert_writeable_eq!(bag, "dd");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    TwoDigit,
}

impl Day {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Numeric, Self::TwoDigit];
}

/// Options for the weekday length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Weekday {
    /// Example: Thu
    ///
    /// Skeleton: `E`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Weekday;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.weekday = Some(Weekday::Short);
    ///
    /// assert_writeable_eq!(bag, "E");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Short,
    /// Example: Thursday
    ///
    /// Skeleton: `EEEE`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Weekday;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.weekday = Some(Weekday::Long);
    ///
    /// assert_writeable_eq!(bag, "EEEE");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Long,
    /// Example: T
    ///
    /// Note: Two weekdays may have the same narrow style for some locales. For example,
    /// both Tuesday's and Thursday's narrow styles are T in the en-US locale. If this is
    /// not acceptable, use [`Self::Short`].
    ///
    /// Skeleton: `EEEEE`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Weekday;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.weekday = Some(Weekday::Narrow);
    ///
    /// assert_writeable_eq!(bag, "EEEEE");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Narrow,
}

impl Weekday {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Short, Self::Long, Self::Narrow];
}

/// Options for the day period length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DayPeriod {
    /// Example: in the evening
    ///
    /// Skeleton: `B`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::DayPeriod;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.day_period = Some(DayPeriod::FlexibleShort);
    ///
    /// assert_writeable_eq!(bag, "B");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    FlexibleShort,
    /// Example: in the evening
    ///
    /// Skeleton: `BBBB`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::DayPeriod;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.day_period = Some(DayPeriod::FlexibleLong);
    ///
    /// assert_writeable_eq!(bag, "BBBB");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    FlexibleLong,
    /// Example: in the evening
    ///
    /// Skeleton: `BBBBB`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::DayPeriod;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.day_period = Some(DayPeriod::FlexibleNarrow);
    ///
    /// assert_writeable_eq!(bag, "BBBBB");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    FlexibleNarrow,
}

impl DayPeriod {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[
        Self::FlexibleShort,
        Self::FlexibleLong,
        Self::FlexibleNarrow,
    ];
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
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Hour;
    /// use icu::datetime::fieldbag::field::HourKind;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.hour = Some(Hour::Numeric);
    /// bag.hour_kind = Some(HourKind::Clock12);
    ///
    /// assert_writeable_eq!(bag, "h");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Clock12,
    /// Example: 16
    ///
    /// Skeleton: `H` or `HH`
    ///
    /// This corresponds to `hour12: false` in ECMA-402.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Hour;
    /// use icu::datetime::fieldbag::field::HourKind;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.hour = Some(Hour::Numeric);
    /// bag.hour_kind = Some(HourKind::Clock24);
    ///
    /// assert_writeable_eq!(bag, "H");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Clock24,
}

impl HourKind {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Clock12, Self::Clock24];
}

/// Options for the hour length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Hour {
    /// Example: 6
    ///
    /// Skeleton: `h`, `H`, or `j` (depending on [`HourKind`])
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Hour;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.hour = Some(Hour::Numeric);
    ///
    /// assert_writeable_eq!(bag, "j");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Numeric,
    /// Example: 06
    ///
    /// Skeleton: `hh`, `HH`, or `jj` (depending on [`HourKind`])
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Hour;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.hour = Some(Hour::TwoDigit);
    ///
    /// assert_writeable_eq!(bag, "jj");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    TwoDigit,
}

impl Hour {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Numeric, Self::TwoDigit];
}

/// Options for the minute length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Minute {
    /// Example: 5
    ///
    /// Skeleton: `m`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Minute;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.minute = Some(Minute::Numeric);
    ///
    /// assert_writeable_eq!(bag, "m");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Numeric,
    /// Example: 05
    ///
    /// Skeleton: `mm`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Minute;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.minute = Some(Minute::TwoDigit);
    ///
    /// assert_writeable_eq!(bag, "mm");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    TwoDigit,
}

impl Minute {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Numeric, Self::TwoDigit];
}

/// Options for the second length, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Second {
    /// Example: 2
    ///
    /// Skeleton: `s`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Second;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.second = Some(Second::Numeric);
    ///
    /// assert_writeable_eq!(bag, "s");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    Numeric,
    /// Example: 02
    ///
    /// Skeleton: `ss`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Second;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.second = Some(Second::TwoDigit);
    ///
    /// assert_writeable_eq!(bag, "ss");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    TwoDigit,
}

impl Second {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::Numeric, Self::TwoDigit];
}

/// Options for the fractional second digits, corresponding to ECMA-402 widths and UTS#35 skeletons.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FractionalSecondDigits {
    /// One fraction digit for seconds.
    ///
    /// Skeleton: `S`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::FractionalSecondDigits;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.fractional_second_digits = Some(FractionalSecondDigits::F1);
    ///
    /// assert_writeable_eq!(bag, "S");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    F1,
    /// Two fraction digits for seconds.
    ///
    /// Skeleton: `SS`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::FractionalSecondDigits;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.fractional_second_digits = Some(FractionalSecondDigits::F2);
    ///
    /// assert_writeable_eq!(bag, "SS");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    F2,
    /// Three fraction digits for seconds.
    ///
    /// Skeleton: `SSS`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::FractionalSecondDigits;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.fractional_second_digits = Some(FractionalSecondDigits::F3);
    ///
    /// assert_writeable_eq!(bag, "SSS");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    F3,
}

impl FractionalSecondDigits {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[Self::F1, Self::F2, Self::F3];
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
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::TimeZoneName;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.time_zone_name = Some(TimeZoneName::ShortSpecific);
    ///
    /// assert_writeable_eq!(bag, "z");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    ShortSpecific,
    /// Long localized form (example: Pacific Standard Time, Nordamerikanische Westküsten-Normalzeit).
    ///
    /// Skeleton: `zzzz`
    ///
    /// This corresponds to the "long" option in ECMA-402.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::TimeZoneName;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.time_zone_name = Some(TimeZoneName::LongSpecific);
    ///
    /// assert_writeable_eq!(bag, "zzzz");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    LongSpecific,
    /// Short localized GMT format (example: GMT-8).
    ///
    /// Skeleton: `O`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::TimeZoneName;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.time_zone_name = Some(TimeZoneName::ShortOffset);
    ///
    /// assert_writeable_eq!(bag, "O");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    ShortOffset,
    /// Long localized GMT format (example: GMT-08:00).
    ///
    /// Skeleton: `OOOO`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::TimeZoneName;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.time_zone_name = Some(TimeZoneName::LongOffset);
    ///
    /// assert_writeable_eq!(bag, "OOOO");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    LongOffset,
    /// Short generic non-location format (example: PT, Los Angeles Zeit).
    ///
    /// Skeleton: `v`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::TimeZoneName;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.time_zone_name = Some(TimeZoneName::ShortGeneric);
    ///
    /// assert_writeable_eq!(bag, "v");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    ShortGeneric,
    /// Long generic non-location format (example: Pacific Time, Nordamerikanische Westküstenzeit).
    ///
    /// Skeleton: `vvvv`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::TimeZoneName;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.time_zone_name = Some(TimeZoneName::LongGeneric);
    ///
    /// assert_writeable_eq!(bag, "vvvv");
    /// assert_eq!(
    ///     bag.to_string().parse::<DateTimeFieldBag>(),
    ///     Ok(bag)
    /// );
    /// ```
    LongGeneric,
}

impl TimeZoneName {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[
        Self::ShortSpecific,
        Self::LongSpecific,
        Self::ShortOffset,
        Self::LongOffset,
        Self::ShortGeneric,
        Self::LongGeneric,
    ];
}
