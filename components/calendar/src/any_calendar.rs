// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Module for working with multiple calendars at once

use crate::buddhist::Buddhist;
use crate::coptic::Coptic;
use crate::ethiopian::{Ethiopian, EthiopianEraStyle};
use crate::gregorian::Gregorian;
use crate::indian::Indian;
use crate::iso::Iso;
use crate::japanese::{Japanese, JapaneseExtended};
use crate::persian::Persian;
use crate::{
    types, AsCalendar, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime, Ref,
};

use icu_locid::extensions::unicode::{key, value, Value};
use icu_locid::subtags::language;
use icu_locid::Locale;
use icu_provider::prelude::*;

use core::fmt;

/// This is a calendar that encompasses all formattable calendars supported by this crate
///
/// This allows for the construction of [`Date`] objects that have their calendar known at runtime.
///
/// This can be constructed by calling `.into()` on a concrete calendar type if the calendar type is known at
/// compile time. When the type is known at runtime, the [`AnyCalendar::new()`] and sibling methods may be used.
///
/// [`Date`](crate::Date) can also be converted to [`AnyCalendar`]-compatible ones
/// via [`Date::to_any()`](crate::Date::to_any()).
///
/// There are many ways of constructing an AnyCalendar'd date:
/// ```
/// use icu::calendar::{AnyCalendar, AnyCalendarKind, DateTime, japanese::Japanese, types::Time};
/// use icu::locid::locale;
/// # use std::str::FromStr;
/// # use std::rc::Rc;
/// # use std::convert::TryInto;
///
/// let locale = locale!("en-u-ca-japanese"); // English with the Japanese calendar
///
/// let calendar = AnyCalendar::new_for_locale(&locale.into());
/// let calendar = Rc::new(calendar); // Avoid cloning it each time
///                                   // If everything is a local reference, you may use icu_calendar::Ref instead.
///
/// // manually construct a datetime in this calendar
/// let manual_time = Time::try_new(12, 33, 12, 0).expect("failed to construct Time");
/// // construct from era code, year, month code, day, time, and a calendar
/// // This is March 28, 15 Heisei
/// let manual_datetime = DateTime::try_new_from_codes("heisei".parse().unwrap(), 15, "M03".parse().unwrap(), 28,
///                                                manual_time, calendar.clone())
///                     .expect("Failed to construct DateTime manually");
///
///
/// // construct another datetime by converting from ISO
/// let iso_datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct ISO DateTime.");
/// let iso_converted = iso_datetime.to_calendar(calendar);
///
/// // Construct a datetime in the appropriate typed calendar and convert
/// let japanese_calendar = Japanese::new();
/// let japanese_datetime = DateTime::try_new_japanese_datetime("heisei".parse().unwrap(), 15, 3, 28,
///                                                         12, 33, 12, japanese_calendar).unwrap();
/// // This is a DateTime<AnyCalendar>
/// let any_japanese_datetime = japanese_datetime.to_any();
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum AnyCalendar {
    /// A [`Gregorian`] calendar
    Gregorian(Gregorian),
    /// A [`Buddhist`] calendar
    Buddhist(Buddhist),
    /// A [`Japanese`] calendar
    Japanese(Japanese),
    /// A [`JapaneseExtended`] calendar
    JapaneseExtended(JapaneseExtended),
    /// An [`Ethiopian`] calendar
    Ethiopian(Ethiopian),
    /// An [`Indian`] calendar
    Indian(Indian),
    /// A [`Persian`] calendar
    Persian(Persian),
    /// A [`Coptic`] calendar
    Coptic(Coptic),
    /// An [`Iso`] calendar
    Iso(Iso),
}

// TODO(#3469): Decide on the best way to implement Ord.
/// The inner date type for [`AnyCalendar`]
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum AnyDateInner {
    /// A date for a [`Gregorian`] calendar
    Gregorian(<Gregorian as Calendar>::DateInner),
    /// A date for a [`Buddhist`] calendar
    Buddhist(<Buddhist as Calendar>::DateInner),
    /// A date for a [`Japanese`] calendar
    Japanese(<Japanese as Calendar>::DateInner),
    /// A date for a [`JapaneseExtended`] calendar
    JapaneseExtended(<JapaneseExtended as Calendar>::DateInner),
    /// A date for an [`Ethiopian`] calendar
    Ethiopian(<Ethiopian as Calendar>::DateInner),
    /// A date for an [`Indian`] calendar
    Indian(<Indian as Calendar>::DateInner),
    /// A date for a [`Persian`] calendar
    Persian(<Persian as Calendar>::DateInner),
    /// A date for a [`Coptic`] calendar
    Coptic(<Coptic as Calendar>::DateInner),
    /// A date for an [`Iso`] calendar
    Iso(<Iso as Calendar>::DateInner),
}

macro_rules! match_cal_and_date {
    (match ($cal:ident, $date:ident): ($cal_matched:ident, $date_matched:ident) => $e:expr) => {
        match ($cal, $date) {
            (&Self::Gregorian(ref $cal_matched), &AnyDateInner::Gregorian(ref $date_matched)) => $e,
            (&Self::Buddhist(ref $cal_matched), &AnyDateInner::Buddhist(ref $date_matched)) => $e,
            (&Self::Japanese(ref $cal_matched), &AnyDateInner::Japanese(ref $date_matched)) => $e,
            (
                &Self::JapaneseExtended(ref $cal_matched),
                &AnyDateInner::JapaneseExtended(ref $date_matched),
            ) => $e,
            (&Self::Ethiopian(ref $cal_matched), &AnyDateInner::Ethiopian(ref $date_matched)) => $e,
            (&Self::Indian(ref $cal_matched), &AnyDateInner::Indian(ref $date_matched)) => $e,
            (&Self::Persian(ref $cal_matched), &AnyDateInner::Persian(ref $date_matched)) => $e,
            (&Self::Coptic(ref $cal_matched), &AnyDateInner::Coptic(ref $date_matched)) => $e,
            (&Self::Iso(ref $cal_matched), &AnyDateInner::Iso(ref $date_matched)) => $e,
            _ => panic!(
                "Found AnyCalendar with mixed calendar type {} and date type {}!",
                $cal.calendar_name(),
                $date.calendar_name()
            ),
        }
    };
}

impl Calendar for AnyCalendar {
    type DateInner = AnyDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let ret = match *self {
            Self::Gregorian(ref c) => {
                AnyDateInner::Gregorian(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Buddhist(ref c) => {
                AnyDateInner::Buddhist(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Japanese(ref c) => {
                AnyDateInner::Japanese(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::JapaneseExtended(ref c) => {
                AnyDateInner::JapaneseExtended(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Ethiopian(ref c) => {
                AnyDateInner::Ethiopian(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Indian(ref c) => {
                AnyDateInner::Indian(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Persian(ref c) => {
                AnyDateInner::Persian(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Coptic(ref c) => {
                AnyDateInner::Coptic(c.date_from_codes(era, year, month_code, day)?)
            }
            Self::Iso(ref c) => AnyDateInner::Iso(c.date_from_codes(era, year, month_code, day)?),
        };
        Ok(ret)
    }
    fn date_from_iso(&self, iso: Date<Iso>) -> AnyDateInner {
        match *self {
            Self::Gregorian(ref c) => AnyDateInner::Gregorian(c.date_from_iso(iso)),
            Self::Buddhist(ref c) => AnyDateInner::Buddhist(c.date_from_iso(iso)),
            Self::Japanese(ref c) => AnyDateInner::Japanese(c.date_from_iso(iso)),
            Self::JapaneseExtended(ref c) => AnyDateInner::JapaneseExtended(c.date_from_iso(iso)),
            Self::Ethiopian(ref c) => AnyDateInner::Ethiopian(c.date_from_iso(iso)),
            Self::Indian(ref c) => AnyDateInner::Indian(c.date_from_iso(iso)),
            Self::Coptic(ref c) => AnyDateInner::Coptic(c.date_from_iso(iso)),
            Self::Iso(ref c) => AnyDateInner::Iso(c.date_from_iso(iso)),
            Self::Persian(ref c) => AnyDateInner::Persian(c.date_from_iso(iso)),
        }
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        match_cal_and_date!(match (self, date): (c, d) => c.date_to_iso(d))
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        match_cal_and_date!(match (self, date): (c, d) => c.months_in_year(d))
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        match_cal_and_date!(match (self, date): (c, d) => c.days_in_year(d))
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        match_cal_and_date!(match (self, date): (c, d) => c.days_in_month(d))
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        match (self, date) {
            (Self::Gregorian(c), &mut AnyDateInner::Gregorian(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Buddhist(c), &mut AnyDateInner::Buddhist(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Japanese(c), &mut AnyDateInner::Japanese(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::JapaneseExtended(c), &mut AnyDateInner::JapaneseExtended(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Ethiopian(c), &mut AnyDateInner::Ethiopian(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Indian(c), &mut AnyDateInner::Indian(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Coptic(c), &mut AnyDateInner::Coptic(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Iso(c), &mut AnyDateInner::Iso(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (Self::Persian(c), &mut AnyDateInner::Persian(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            // This is only reached from misuse of from_raw, a semi-internal api
            #[allow(clippy::panic)]
            (_, d) => panic!(
                "Found AnyCalendar with mixed calendar type {} and date type {}!",
                self.calendar_name(),
                d.calendar_name()
            ),
        }
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        match (self, calendar2, date1, date2) {
            (
                Self::Gregorian(c1),
                Self::Gregorian(c2),
                AnyDateInner::Gregorian(d1),
                AnyDateInner::Gregorian(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::Buddhist(c1),
                Self::Buddhist(c2),
                AnyDateInner::Buddhist(d1),
                AnyDateInner::Buddhist(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::Japanese(c1),
                Self::Japanese(c2),
                AnyDateInner::Japanese(d1),
                AnyDateInner::Japanese(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::JapaneseExtended(c1),
                Self::JapaneseExtended(c2),
                AnyDateInner::JapaneseExtended(d1),
                AnyDateInner::JapaneseExtended(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::Ethiopian(c1),
                Self::Ethiopian(c2),
                AnyDateInner::Ethiopian(d1),
                AnyDateInner::Ethiopian(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::Indian(c1),
                Self::Indian(c2),
                AnyDateInner::Indian(d1),
                AnyDateInner::Indian(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::Persian(c1),
                Self::Persian(c2),
                AnyDateInner::Persian(d1),
                AnyDateInner::Persian(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                Self::Coptic(c1),
                Self::Coptic(c2),
                AnyDateInner::Coptic(d1),
                AnyDateInner::Coptic(d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (Self::Iso(c1), Self::Iso(c2), AnyDateInner::Iso(d1), AnyDateInner::Iso(d2)) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            _ => {
                // attempt to convert
                let iso = calendar2.date_to_iso(date2);

                match_cal_and_date!(match (self, date1):
                    (c1, d1) => {
                        let d2 = c1.date_from_iso(iso);
                        let until = c1.until(d1, &d2, c1, largest_unit, smallest_unit);
                        until.cast_unit::<AnyCalendar>()
                    }
                )
            }
        }
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        match_cal_and_date!(match (self, date): (c, d) => c.year(d))
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        match_cal_and_date!(match (self, date): (c, d) => c.month(d))
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        match_cal_and_date!(match (self, date): (c, d) => c.day_of_month(d))
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        match_cal_and_date!(match (self, date): (c, d) => c.day_of_year_info(d))
    }

    fn debug_name(&self) -> &'static str {
        match *self {
            Self::Gregorian(_) => "AnyCalendar (Gregorian)",
            Self::Buddhist(_) => "AnyCalendar (Buddhist)",
            Self::Japanese(_) => "AnyCalendar (Japanese)",
            Self::JapaneseExtended(_) => "AnyCalendar (Japanese, Historical Era Data)",
            Self::Ethiopian(_) => "AnyCalendar (Ethiopian)",
            Self::Indian(_) => "AnyCalendar (Indian)",
            Self::Coptic(_) => "AnyCalendar (Coptic)",
            Self::Iso(_) => "AnyCalendar (Iso)",
            Self::Persian(_) => "AnyCalendar (Persian)",
        }
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(self.kind())
    }
}

impl AnyCalendar {
    /// Constructs an AnyCalendar for a given calendar kind.
    ///
    /// As this requires a valid [`AnyCalendarKind`] to work, it does not do any kind of locale-based
    /// fallbacking. If this is desired, use [`Self::new_for_locale()`].
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub const fn new(kind: AnyCalendarKind) -> Self {
        match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => AnyCalendar::Japanese(Japanese::new()),
            AnyCalendarKind::JapaneseExtended => {
                AnyCalendar::JapaneseExtended(JapaneseExtended::new())
            }
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::new)]
    pub fn try_new_with_any_provider<P>(
        provider: &P,
        kind: AnyCalendarKind,
    ) -> Result<Self, CalendarError>
    where
        P: AnyProvider + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => {
                AnyCalendar::Japanese(Japanese::try_new_with_any_provider(provider)?)
            }
            AnyCalendarKind::JapaneseExtended => AnyCalendar::JapaneseExtended(
                JapaneseExtended::try_new_with_any_provider(provider)?,
            ),
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
        })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::new)]
    pub fn try_new_with_buffer_provider<P>(
        provider: &P,
        kind: AnyCalendarKind,
    ) -> Result<Self, CalendarError>
    where
        P: BufferProvider + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => {
                AnyCalendar::Japanese(Japanese::try_new_with_buffer_provider(provider)?)
            }
            AnyCalendarKind::JapaneseExtended => AnyCalendar::JapaneseExtended(
                JapaneseExtended::try_new_with_buffer_provider(provider)?,
            ),
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P, kind: AnyCalendarKind) -> Result<Self, CalendarError>
    where
        P: DataProvider<crate::provider::JapaneseErasV1Marker>
            + DataProvider<crate::provider::JapaneseExtendedErasV1Marker>
            + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => {
                AnyCalendar::Japanese(Japanese::try_new_unstable(provider)?)
            }
            AnyCalendarKind::JapaneseExtended => {
                AnyCalendar::JapaneseExtended(JapaneseExtended::try_new_unstable(provider)?)
            }
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
        })
    }

    /// Constructs an AnyCalendar for a given calendar kind.
    ///
    /// In case the locale's calendar is unknown or unspecified, it will attempt to load the default
    /// calendar for the locale, falling back to gregorian.
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub fn new_for_locale(locale: &DataLocale) -> Self {
        let kind = AnyCalendarKind::from_data_locale_with_fallback(locale);
        Self::new(kind)
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: skip,
        error: CalendarError,
        #[cfg(skip)]
        functions: [
            new_for_locale,
            try_new_for_locale_with_any_provider,
            try_new_for_locale_with_buffer_provider,
            try_new_for_locale_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_for_locale)]
    pub fn try_new_for_locale_unstable<P>(
        provider: &P,
        locale: &DataLocale,
    ) -> Result<Self, CalendarError>
    where
        P: DataProvider<crate::provider::JapaneseErasV1Marker>
            + DataProvider<crate::provider::JapaneseExtendedErasV1Marker>
            + ?Sized,
    {
        let kind = AnyCalendarKind::from_data_locale_with_fallback(locale);
        Self::try_new_unstable(provider, kind)
    }

    fn calendar_name(&self) -> &'static str {
        match *self {
            Self::Gregorian(_) => "Gregorian",
            Self::Buddhist(_) => "Buddhist",
            Self::Japanese(_) => "Japanese",
            Self::JapaneseExtended(_) => "Japanese (Historical era data)",
            Self::Ethiopian(_) => "Ethiopian",
            Self::Indian(_) => "Indian",
            Self::Coptic(_) => "Coptic",
            Self::Iso(_) => "Iso",
            Self::Persian(_) => "Persian",
        }
    }

    /// The [`AnyCalendarKind`] corresponding to the calendar this contains
    pub fn kind(&self) -> AnyCalendarKind {
        match *self {
            Self::Gregorian(_) => AnyCalendarKind::Gregorian,
            Self::Buddhist(_) => AnyCalendarKind::Buddhist,
            Self::Japanese(_) => AnyCalendarKind::Japanese,
            Self::JapaneseExtended(_) => AnyCalendarKind::JapaneseExtended,
            #[allow(clippy::expect_used)] // Invariant known at compile time
            Self::Ethiopian(ref e) => e
                .any_calendar_kind()
                .expect("Ethiopian calendar known to have an AnyCalendarKind"),
            Self::Indian(_) => AnyCalendarKind::Indian,
            Self::Coptic(_) => AnyCalendarKind::Coptic,
            Self::Iso(_) => AnyCalendarKind::Iso,
            Self::Persian(_) => AnyCalendarKind::Persian,
        }
    }

    /// Given an AnyCalendar date, convert that date to another AnyCalendar date in this calendar,
    /// if conversion is needed
    pub fn convert_any_date<'a>(
        &'a self,
        date: &Date<impl AsCalendar<Calendar = AnyCalendar>>,
    ) -> Date<Ref<'a, AnyCalendar>> {
        if self.kind() != date.calendar.as_calendar().kind() {
            Date::new_from_iso(date.to_iso(), Ref(self))
        } else {
            Date {
                inner: date.inner.clone(),
                calendar: Ref(self),
            }
        }
    }

    /// Given an AnyCalendar datetime, convert that date to another AnyCalendar datetime in this calendar,
    /// if conversion is needed
    pub fn convert_any_datetime<'a>(
        &'a self,
        date: &DateTime<impl AsCalendar<Calendar = AnyCalendar>>,
    ) -> DateTime<Ref<'a, AnyCalendar>> {
        DateTime {
            time: date.time,
            date: self.convert_any_date(&date.date),
        }
    }
}

impl AnyDateInner {
    fn calendar_name(&self) -> &'static str {
        match *self {
            AnyDateInner::Gregorian(_) => "Gregorian",
            AnyDateInner::Buddhist(_) => "Buddhist",
            AnyDateInner::Japanese(_) => "Japanese",
            AnyDateInner::JapaneseExtended(_) => "Japanese (Historical era data)",
            AnyDateInner::Ethiopian(_) => "Ethiopian",
            AnyDateInner::Indian(_) => "Indian",
            AnyDateInner::Coptic(_) => "Coptic",
            AnyDateInner::Iso(_) => "Iso",
            AnyDateInner::Persian(_) => "Persian",
        }
    }
}

/// Convenient type for selecting the kind of AnyCalendar to construct
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AnyCalendarKind {
    /// The kind of a [`Gregorian`] calendar
    Gregorian,
    /// The kind of a [`Buddhist`] calendar
    Buddhist,
    /// The kind of a [`Japanese`] calendar
    Japanese,
    /// The kind of a [`JapaneseExtended`] calendar
    JapaneseExtended,
    /// The kind of an [`Ethiopian`] calendar, with Amete Mihret era
    Ethiopian,
    /// The kind of an [`Ethiopian`] calendar, with Amete Alem era
    EthiopianAmeteAlem,
    /// The kind of a [`Indian`] calendar
    Indian,
    /// The kind of a [`Coptic`] calendar
    Coptic,
    /// The kind of an [`Iso`] calendar
    Iso,
    /// The kind of a [`Persian`] calendar
    Persian,
}

impl AnyCalendarKind {
    /// Construct from a BCP-47 string
    ///
    /// Returns None if the calendar is unknown. If you prefer an error, use
    /// [`CalendarError::unknown_any_calendar_kind`].
    pub fn get_for_bcp47_string(x: &str) -> Option<Self> {
        Self::get_for_bcp47_bytes(x.as_bytes())
    }
    /// Construct from a BCP-47 byte string
    ///
    /// Returns None if the calendar is unknown. If you prefer an error, use
    /// [`CalendarError::unknown_any_calendar_kind`].
    pub fn get_for_bcp47_bytes(x: &[u8]) -> Option<Self> {
        Some(match x {
            b"gregory" => AnyCalendarKind::Gregorian,
            b"buddhist" => AnyCalendarKind::Buddhist,
            b"japanese" => AnyCalendarKind::Japanese,
            b"japanext" => AnyCalendarKind::JapaneseExtended,
            b"indian" => AnyCalendarKind::Indian,
            b"coptic" => AnyCalendarKind::Coptic,
            b"iso" => AnyCalendarKind::Iso,
            b"ethiopic" => AnyCalendarKind::Ethiopian,
            b"ethioaa" => AnyCalendarKind::EthiopianAmeteAlem,
            b"persian" => AnyCalendarKind::Persian,
            _ => return None,
        })
    }
    /// Construct from a BCP-47 [`Value`]
    ///
    /// Returns None if the calendar is unknown. If you prefer an error, use
    /// [`CalendarError::unknown_any_calendar_kind`].
    pub fn get_for_bcp47_value(x: &Value) -> Option<Self> {
        Some(if *x == value!("gregory") {
            AnyCalendarKind::Gregorian
        } else if *x == value!("buddhist") {
            AnyCalendarKind::Buddhist
        } else if *x == value!("japanese") {
            AnyCalendarKind::Japanese
        } else if *x == value!("japanext") {
            AnyCalendarKind::JapaneseExtended
        } else if *x == value!("indian") {
            AnyCalendarKind::Indian
        } else if *x == value!("coptic") {
            AnyCalendarKind::Coptic
        } else if *x == value!("iso") {
            AnyCalendarKind::Iso
        } else if *x == value!("ethiopic") {
            AnyCalendarKind::Ethiopian
        } else if *x == value!("ethioaa") {
            AnyCalendarKind::EthiopianAmeteAlem
        } else if *x == value!("persian") {
            AnyCalendarKind::Persian
        } else {
            return None;
        })
    }

    /// Convert to a BCP-47 string
    pub fn as_bcp47_string(self) -> &'static str {
        match self {
            AnyCalendarKind::Gregorian => "gregory",
            AnyCalendarKind::Buddhist => "buddhist",
            AnyCalendarKind::Japanese => "japanese",
            AnyCalendarKind::JapaneseExtended => "japanext",
            AnyCalendarKind::Indian => "indian",
            AnyCalendarKind::Coptic => "coptic",
            AnyCalendarKind::Iso => "iso",
            AnyCalendarKind::Ethiopian => "ethiopic",
            AnyCalendarKind::EthiopianAmeteAlem => "ethioaa",
            AnyCalendarKind::Persian => "persian",
        }
    }

    /// Convert to a BCP-47 `Value`
    pub fn as_bcp47_value(self) -> Value {
        match self {
            AnyCalendarKind::Gregorian => value!("gregory"),
            AnyCalendarKind::Buddhist => value!("buddhist"),
            AnyCalendarKind::Japanese => value!("japanese"),
            AnyCalendarKind::JapaneseExtended => value!("japanext"),
            AnyCalendarKind::Indian => value!("indian"),
            AnyCalendarKind::Coptic => value!("coptic"),
            AnyCalendarKind::Iso => value!("iso"),
            AnyCalendarKind::Ethiopian => value!("ethiopic"),
            AnyCalendarKind::EthiopianAmeteAlem => value!("ethioaa"),
            AnyCalendarKind::Persian => value!("persian"),
        }
    }

    /// Extract the calendar component from a [`Locale`]
    ///
    /// Returns None if the calendar is not specified or unknown. If you prefer an error, use
    /// [`CalendarError::unknown_any_calendar_kind`].
    pub fn get_for_locale(l: &Locale) -> Option<Self> {
        l.extensions
            .unicode
            .keywords
            .get(&key!("ca"))
            .and_then(Self::get_for_bcp47_value)
    }

    /// Extract the calendar component from a [`DataLocale`]
    ///
    /// Returns None if the calendar is not specified or unknown. If you prefer an error, use
    /// [`CalendarError::unknown_any_calendar_kind`].
    fn get_for_data_locale(l: &DataLocale) -> Option<Self> {
        l.get_unicode_ext(&key!("ca"))
            .and_then(|v| Self::get_for_bcp47_value(&v))
    }

    // Do not make public, this will eventually need fallback
    // data from the provider
    fn from_data_locale_with_fallback(l: &DataLocale) -> Self {
        if let Some(kind) = Self::get_for_data_locale(l) {
            kind
        } else {
            let lang = l.language();
            if lang == language!("th") {
                Self::Buddhist
            // Other known fallback routes for currently-unsupported calendars
            // } else if lang == language!("sa") {
            //     Self::IslamicUmalqura
            // } else if lang == language!("af") || lang == language!("ir") {
            //     Self::Persian
            } else {
                Self::Gregorian
            }
        }
    }
}

impl fmt::Display for AnyCalendarKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<C: IntoAnyCalendar> From<C> for AnyCalendar {
    fn from(c: C) -> AnyCalendar {
        c.to_any()
    }
}

/// Trait for calendars that may be converted to [`AnyCalendar`]
pub trait IntoAnyCalendar: Calendar + Sized {
    /// Convert this calendar into an [`AnyCalendar`], moving it
    ///
    /// You should not need to call this method directly
    fn to_any(self) -> AnyCalendar;

    /// Convert this calendar into an [`AnyCalendar`], cloning it
    ///
    /// You should not need to call this method directly
    fn to_any_cloned(&self) -> AnyCalendar;
    /// Convert a date for this calendar into an [`AnyDateInner`]
    ///
    /// You should not need to call this method directly
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner;
}

impl IntoAnyCalendar for Gregorian {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Gregorian(Gregorian)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Gregorian(Gregorian)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Gregorian(*d)
    }
}

impl IntoAnyCalendar for Buddhist {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Buddhist(Buddhist)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Buddhist(Buddhist)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Buddhist(*d)
    }
}

impl IntoAnyCalendar for Japanese {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Japanese(self)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Japanese(self.clone())
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Japanese(*d)
    }
}

impl IntoAnyCalendar for JapaneseExtended {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::JapaneseExtended(self)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::JapaneseExtended(self.clone())
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::JapaneseExtended(*d)
    }
}

impl IntoAnyCalendar for Ethiopian {
    // Amete Mihret calendars are the default
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Ethiopian(self)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Ethiopian(*self)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Ethiopian(*d)
    }
}

impl IntoAnyCalendar for Indian {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Indian(Indian)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Indian(Indian)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Indian(*d)
    }
}

impl IntoAnyCalendar for Coptic {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Coptic(Coptic)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Coptic(Coptic)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Coptic(*d)
    }
}

impl IntoAnyCalendar for Persian {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Persian(Persian)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Persian(Persian)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Persian(*d)
    }
}

impl IntoAnyCalendar for Iso {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Iso(Iso)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Iso(Iso)
    }
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Iso(*d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ref;
    use core::convert::TryInto;

    fn single_test_roundtrip(
        calendar: Ref<AnyCalendar>,
        era: &str,
        year: i32,
        month_code: &str,
        day: u8,
    ) {
        let era = types::Era(era.parse().expect("era must parse"));
        let month = types::MonthCode(month_code.parse().expect("month code must parse"));

        let date = Date::try_new_from_codes(era, year, month, day, calendar).unwrap_or_else(|e| {
            panic!(
                "Failed to construct date for {} with {:?}, {}, {}, {}: {}",
                calendar.debug_name(),
                era,
                year,
                month,
                day,
                e,
            )
        });

        let roundtrip_year = date.year();
        let roundtrip_era = roundtrip_year.era;
        let roundtrip_year = roundtrip_year.number;
        let roundtrip_month = date.month().code;
        let roundtrip_day = date.day_of_month().0.try_into().expect("Must fit in u8");

        assert_eq!(
            (era, year, month, day),
            (
                roundtrip_era,
                roundtrip_year,
                roundtrip_month,
                roundtrip_day
            ),
            "Failed to roundtrip for calendar {}",
            calendar.debug_name()
        );

        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            date, reconstructed,
            "Failed to roundtrip via iso with {era:?}, {year}, {month}, {day}"
        )
    }

    fn single_test_error(
        calendar: Ref<AnyCalendar>,
        era: &str,
        year: i32,
        month_code: &str,
        day: u8,
        error: CalendarError,
    ) {
        let era = types::Era(era.parse().expect("era must parse"));
        let month = types::MonthCode(month_code.parse().expect("month code must parse"));

        let date = Date::try_new_from_codes(era, year, month, day, calendar);
        assert_eq!(
            date,
            Err(error),
            "Construction with {era:?}, {year}, {month}, {day} did not return {error:?}"
        )
    }

    #[test]
    fn test_any_construction() {
        let buddhist = AnyCalendar::new(AnyCalendarKind::Buddhist);
        let coptic = AnyCalendar::new(AnyCalendarKind::Coptic);
        let ethiopian = AnyCalendar::new(AnyCalendarKind::Ethiopian);
        let ethioaa = AnyCalendar::new(AnyCalendarKind::EthiopianAmeteAlem);
        let gregorian = AnyCalendar::new(AnyCalendarKind::Gregorian);
        let indian = AnyCalendar::new(AnyCalendarKind::Indian);
        let japanese = AnyCalendar::new(AnyCalendarKind::Japanese);
        let japanext = AnyCalendar::new(AnyCalendarKind::JapaneseExtended);
        let persian = AnyCalendar::new(AnyCalendarKind::Persian);
        let buddhist = Ref(&buddhist);
        let coptic = Ref(&coptic);
        let ethiopian = Ref(&ethiopian);
        let ethioaa = Ref(&ethioaa);
        let gregorian = Ref(&gregorian);
        let indian = Ref(&indian);
        let japanese = Ref(&japanese);
        let japanext = Ref(&japanext);
        let persian = Ref(&persian);

        single_test_roundtrip(buddhist, "be", 100, "M03", 1);
        single_test_roundtrip(buddhist, "be", 2000, "M03", 1);
        single_test_roundtrip(buddhist, "be", -100, "M03", 1);
        single_test_error(
            buddhist,
            "be",
            100,
            "M13",
            1,
            CalendarError::UnknownMonthCode("M13".parse().unwrap(), "Buddhist"),
        );

        single_test_roundtrip(coptic, "ad", 100, "M03", 1);
        single_test_roundtrip(coptic, "ad", 2000, "M03", 1);
        // fails ISO roundtrip
        // single_test_roundtrip(coptic, "bd", 100, "M03", 1);
        single_test_roundtrip(coptic, "ad", 100, "M13", 1);
        single_test_error(
            coptic,
            "ad",
            100,
            "M14",
            1,
            CalendarError::UnknownMonthCode("M14".parse().unwrap(), "Coptic"),
        );
        single_test_error(coptic, "ad", 0, "M03", 1, CalendarError::OutOfRange);
        single_test_error(coptic, "bd", 0, "M03", 1, CalendarError::OutOfRange);

        single_test_roundtrip(ethiopian, "incar", 100, "M03", 1);
        single_test_roundtrip(ethiopian, "incar", 2000, "M03", 1);
        single_test_roundtrip(ethiopian, "incar", 2000, "M13", 1);
        // Fails ISO roundtrip due to https://github.com/unicode-org/icu4x/issues/2254
        // single_test_roundtrip(ethiopian, "pre-incar", 100, "M03", 1);
        single_test_error(ethiopian, "incar", 0, "M03", 1, CalendarError::OutOfRange);
        single_test_error(
            ethiopian,
            "pre-incar",
            0,
            "M03",
            1,
            CalendarError::OutOfRange,
        );
        single_test_error(
            ethiopian,
            "incar",
            100,
            "M14",
            1,
            CalendarError::UnknownMonthCode("M14".parse().unwrap(), "Ethiopian"),
        );

        single_test_roundtrip(ethioaa, "mundi", 7000, "M13", 1);
        single_test_roundtrip(ethioaa, "mundi", 7000, "M13", 1);
        // Fails ISO roundtrip due to https://github.com/unicode-org/icu4x/issues/2254
        // single_test_roundtrip(ethioaa, "mundi", 100, "M03", 1);
        single_test_error(
            ethiopian,
            "mundi",
            100,
            "M14",
            1,
            CalendarError::UnknownMonthCode("M14".parse().unwrap(), "Ethiopian"),
        );

        single_test_roundtrip(gregorian, "ce", 100, "M03", 1);
        single_test_roundtrip(gregorian, "ce", 2000, "M03", 1);
        single_test_roundtrip(gregorian, "bce", 100, "M03", 1);
        single_test_error(gregorian, "ce", 0, "M03", 1, CalendarError::OutOfRange);
        single_test_error(gregorian, "bce", 0, "M03", 1, CalendarError::OutOfRange);

        single_test_error(
            gregorian,
            "bce",
            100,
            "M13",
            1,
            CalendarError::UnknownMonthCode("M13".parse().unwrap(), "Gregorian"),
        );

        single_test_roundtrip(indian, "saka", 100, "M03", 1);
        single_test_roundtrip(indian, "saka", 2000, "M12", 1);
        single_test_roundtrip(indian, "saka", -100, "M03", 1);
        single_test_roundtrip(indian, "saka", 0, "M03", 1);
        single_test_error(
            indian,
            "saka",
            100,
            "M13",
            1,
            CalendarError::UnknownMonthCode("M13".parse().unwrap(), "Indian"),
        );
        single_test_roundtrip(japanese, "reiwa", 3, "M03", 1);
        single_test_roundtrip(japanese, "heisei", 6, "M12", 1);
        single_test_roundtrip(japanese, "meiji", 10, "M03", 1);
        single_test_roundtrip(japanese, "ce", 1000, "M03", 1);
        single_test_roundtrip(japanese, "bce", 10, "M03", 1);
        single_test_error(japanese, "ce", 0, "M03", 1, CalendarError::OutOfRange);
        single_test_error(japanese, "bce", 0, "M03", 1, CalendarError::OutOfRange);

        single_test_error(
            japanese,
            "reiwa",
            2,
            "M13",
            1,
            CalendarError::UnknownMonthCode("M13".parse().unwrap(), "Japanese (Modern eras only)"),
        );

        single_test_roundtrip(japanext, "reiwa", 3, "M03", 1);
        single_test_roundtrip(japanext, "heisei", 6, "M12", 1);
        single_test_roundtrip(japanext, "meiji", 10, "M03", 1);
        single_test_roundtrip(japanext, "tenpyokampo-749", 1, "M04", 20);
        single_test_roundtrip(japanext, "ce", 100, "M03", 1);
        single_test_roundtrip(japanext, "bce", 10, "M03", 1);
        single_test_error(japanext, "ce", 0, "M03", 1, CalendarError::OutOfRange);
        single_test_error(japanext, "bce", 0, "M03", 1, CalendarError::OutOfRange);

        single_test_error(
            japanext,
            "reiwa",
            2,
            "M13",
            1,
            CalendarError::UnknownMonthCode(
                "M13".parse().unwrap(),
                "Japanese (With historical eras)",
            ),
        );

        single_test_roundtrip(persian, "ah", 477, "M03", 1);
        single_test_roundtrip(persian, "ah", 2083, "M07", 21);
        single_test_roundtrip(persian, "ah", 1600, "M12", 20);
        single_test_error(
            persian,
            "ah",
            100,
            "M9",
            1,
            CalendarError::UnknownMonthCode("M9".parse().unwrap(), "Persian"),
        );
    }
}
