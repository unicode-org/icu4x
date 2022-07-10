// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Module for working with multiple calendars at once

use crate::buddhist::Buddhist;
use crate::coptic::Coptic;
use crate::ethiopic::Ethiopic;
use crate::gregorian::Gregorian;
use crate::indian::Indian;
use crate::iso::Iso;
use crate::japanese::Japanese;
use crate::{types, AsCalendar, Calendar, Date, DateDuration, DateDurationUnit, DateTime, Ref};

use icu_locid::{
    extensions::unicode::Value, extensions_unicode_key as key, extensions_unicode_value as value,
    Locale,
};

use icu_provider::prelude::*;

use core::fmt;

/// This is a calendar that encompasses all formattable calendars supported by this crate
///
/// This allows for the construction of [`Date`] objects that have their calendar known at runtime.
///
/// This can be constructed by calling `.into()` on a concrete calendar type if the calendar type is known at
/// compile time. When the type is known at runtime, the [`AnyCalendar::try_new_with_any_provider()`],
/// [`AnyCalendar::try_new_with_buffer_provider()`], and [`AnyCalendar::try_new_unstable()`] methods may be used.
///
/// [`Date`](crate::Date) can also be converted to [`AnyCalendar`]-compatible ones
/// via [`Date::to_any()`](crate::Date::to_any()).
#[non_exhaustive]
pub enum AnyCalendar {
    Gregorian(Gregorian),
    Buddhist(Buddhist),
    Japanese(Japanese),
    Ethiopic(Ethiopic),
    Indian(Indian),
    Coptic(Coptic),
    Iso(Iso),
}

/// The inner date type for [`AnyCalendar`]
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum AnyDateInner {
    Gregorian(<Gregorian as Calendar>::DateInner),
    Buddhist(<Buddhist as Calendar>::DateInner),
    Japanese(<Japanese as Calendar>::DateInner),
    Ethiopic(<Ethiopic as Calendar>::DateInner),
    Indian(<Indian as Calendar>::DateInner),
    Coptic(<Coptic as Calendar>::DateInner),
    Iso(<Iso as Calendar>::DateInner),
}

macro_rules! match_cal_and_date {
    (match ($cal:ident, $date:ident): ($cal_matched:ident, $date_matched:ident) => $e:expr) => {
        match ($cal, $date) {
            (&Self::Gregorian(ref $cal_matched), &AnyDateInner::Gregorian(ref $date_matched)) => $e,
            (&Self::Buddhist(ref $cal_matched), &AnyDateInner::Buddhist(ref $date_matched)) => $e,
            (&Self::Japanese(ref $cal_matched), &AnyDateInner::Japanese(ref $date_matched)) => $e,
            (&Self::Ethiopic(ref $cal_matched), &AnyDateInner::Ethiopic(ref $date_matched)) => $e,
            (&Self::Indian(ref $cal_matched), &AnyDateInner::Indian(ref $date_matched)) => $e,
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
    fn date_from_iso(&self, iso: Date<Iso>) -> AnyDateInner {
        match *self {
            Self::Gregorian(ref c) => AnyDateInner::Gregorian(c.date_from_iso(iso)),
            Self::Buddhist(ref c) => AnyDateInner::Buddhist(c.date_from_iso(iso)),
            Self::Japanese(ref c) => AnyDateInner::Japanese(c.date_from_iso(iso)),
            Self::Ethiopic(ref c) => AnyDateInner::Ethiopic(c.date_from_iso(iso)),
            Self::Indian(ref c) => AnyDateInner::Indian(c.date_from_iso(iso)),
            Self::Coptic(ref c) => AnyDateInner::Coptic(c.date_from_iso(iso)),
            Self::Iso(ref c) => AnyDateInner::Iso(c.date_from_iso(iso)),
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
            (&Self::Gregorian(ref c), &mut AnyDateInner::Gregorian(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (&Self::Buddhist(ref c), &mut AnyDateInner::Buddhist(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (&Self::Japanese(ref c), &mut AnyDateInner::Japanese(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (&Self::Ethiopic(ref c), &mut AnyDateInner::Ethiopic(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (&Self::Indian(ref c), &mut AnyDateInner::Indian(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (&Self::Coptic(ref c), &mut AnyDateInner::Coptic(ref mut d)) => {
                c.offset_date(d, offset.cast_unit())
            }
            (&Self::Iso(ref c), &mut AnyDateInner::Iso(ref mut d)) => {
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
                &Self::Gregorian(ref c1),
                &Self::Gregorian(ref c2),
                &AnyDateInner::Gregorian(ref d1),
                &AnyDateInner::Gregorian(ref d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                &Self::Buddhist(ref c1),
                &Self::Buddhist(ref c2),
                &AnyDateInner::Buddhist(ref d1),
                &AnyDateInner::Buddhist(ref d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                &Self::Japanese(ref c1),
                &Self::Japanese(ref c2),
                &AnyDateInner::Japanese(ref d1),
                &AnyDateInner::Japanese(ref d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                &Self::Ethiopic(ref c1),
                &Self::Ethiopic(ref c2),
                &AnyDateInner::Ethiopic(ref d1),
                &AnyDateInner::Ethiopic(ref d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                &Self::Indian(ref c1),
                &Self::Indian(ref c2),
                &AnyDateInner::Indian(ref d1),
                &AnyDateInner::Indian(ref d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                &Self::Coptic(ref c1),
                &Self::Coptic(ref c2),
                &AnyDateInner::Coptic(ref d1),
                &AnyDateInner::Coptic(ref d2),
            ) => c1
                .until(d1, d2, c2, largest_unit, smallest_unit)
                .cast_unit(),
            (
                &Self::Iso(ref c1),
                &Self::Iso(ref c2),
                &AnyDateInner::Iso(ref d1),
                &AnyDateInner::Iso(ref d2),
            ) => c1
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
    fn year(&self, date: &Self::DateInner) -> types::Year {
        match_cal_and_date!(match (self, date): (c, d) => c.year(d))
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
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
            Self::Ethiopic(_) => "AnyCalendar (Ethiopic)",
            Self::Indian(_) => "AnyCalendar (Indian)",
            Self::Coptic(_) => "AnyCalendar (Coptic)",
            Self::Iso(_) => "AnyCalendar (Iso)",
        }
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(self.kind())
    }
}

impl AnyCalendar {
    /// Constructs an AnyCalendar for a given calendar kind and [`AnyProvider`] data source
    ///
    /// For calendars that need data, will attempt to load the appropriate data from the source.
    ///
    /// This API needs the `calendar/japanese@1` data key if working with Japanese calendars.
    pub fn try_new_with_any_provider<P>(
        kind: AnyCalendarKind,
        provider: &P,
    ) -> Result<Self, DataError>
    where
        P: AnyProvider + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => {
                let p = provider.as_downcasting();
                AnyCalendar::Japanese(Japanese::try_new(&p)?)
            }
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopic => {
                AnyCalendar::Ethiopic(Ethiopic::new_with_amete_alem(false))
            }
            AnyCalendarKind::Ethioaa => AnyCalendar::Ethiopic(Ethiopic::new_with_amete_alem(true)),
        })
    }

    /// Constructs an AnyCalendar for a given calendar kind and [`BufferProvider`] data source
    ///
    /// For calendars that need data, will attempt to load the appropriate data from the source.
    ///
    /// This API needs the `calendar/japanese@1` data key if working with Japanese calendars.
    ///
    /// This needs the `"serde"` feature to be enabled to be used
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider<P>(
        kind: AnyCalendarKind,
        provider: &P,
    ) -> Result<Self, DataError>
    where
        P: BufferProvider + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => {
                let p = provider.as_deserializing();
                AnyCalendar::Japanese(Japanese::try_new(&p)?)
            }
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopic => {
                AnyCalendar::Ethiopic(Ethiopic::new_with_amete_alem(false))
            }
            AnyCalendarKind::Ethioaa => AnyCalendar::Ethiopic(Ethiopic::new_with_amete_alem(true)),
        })
    }

    /// Constructs an AnyCalendar for a given calendar kind and data source.
    ///
    /// **This method is unstable; the bounds on `P` might expand over time as more calendars are added**
    ///
    /// For calendars that need data, will attempt to load the appropriate data from the source
    pub fn try_new_unstable<P>(kind: AnyCalendarKind, provider: &P) -> Result<Self, DataError>
    where
        P: ResourceProvider<crate::provider::JapaneseErasV1Marker> + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Japanese => AnyCalendar::Japanese(Japanese::try_new(provider)?),
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Ethiopic => {
                AnyCalendar::Ethiopic(Ethiopic::new_with_amete_alem(false))
            }
            AnyCalendarKind::Ethioaa => AnyCalendar::Ethiopic(Ethiopic::new_with_amete_alem(true)),
        })
    }

    fn calendar_name(&self) -> &'static str {
        match *self {
            Self::Gregorian(_) => "Gregorian",
            Self::Buddhist(_) => "Buddhist",
            Self::Japanese(_) => "Japanese",
            Self::Ethiopic(_) => "Ethiopic",
            Self::Indian(_) => "Indian",
            Self::Coptic(_) => "Coptic",
            Self::Iso(_) => "Iso",
        }
    }

    pub fn kind(&self) -> AnyCalendarKind {
        match *self {
            Self::Gregorian(_) => AnyCalendarKind::Gregorian,
            Self::Buddhist(_) => AnyCalendarKind::Buddhist,
            Self::Japanese(_) => AnyCalendarKind::Japanese,
            #[allow(clippy::expect_used)] // Invariant known at compile time
            Self::Ethiopic(ref e) => e
                .any_calendar_kind()
                .expect("Ethiopic calendar known to have an AnyCalendarKind"),
            Self::Indian(_) => AnyCalendarKind::Indian,
            Self::Coptic(_) => AnyCalendarKind::Coptic,
            Self::Iso(_) => AnyCalendarKind::Iso,
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
            AnyDateInner::Ethiopic(_) => "Ethiopic",
            AnyDateInner::Indian(_) => "Indian",
            AnyDateInner::Coptic(_) => "Coptic",
            AnyDateInner::Iso(_) => "Iso",
        }
    }
}

/// Convenient type for selecting the kind of AnyCalendar to construct
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AnyCalendarKind {
    Gregorian,
    Buddhist,
    Japanese,
    Indian,
    Coptic,
    Iso,
    Ethiopic,
    /// Ethiopic with Amete Alem era
    Ethioaa,
}

impl AnyCalendarKind {
    pub fn from_bcp47_string(x: &str) -> Option<Self> {
        Some(match x {
            "gregory" => AnyCalendarKind::Gregorian,
            "buddhist" => AnyCalendarKind::Buddhist,
            "japanese" => AnyCalendarKind::Japanese,
            "indian" => AnyCalendarKind::Indian,
            "coptic" => AnyCalendarKind::Coptic,
            "iso" => AnyCalendarKind::Iso,
            "ethiopic" => AnyCalendarKind::Ethiopic,
            "ethioaa" => AnyCalendarKind::Ethioaa,
            _ => return None,
        })
    }

    pub fn from_bcp47(x: &Value) -> Option<Self> {
        Some(if *x == value!("gregory") {
            AnyCalendarKind::Gregorian
        } else if *x == value!("buddhist") {
            AnyCalendarKind::Buddhist
        } else if *x == value!("japanese") {
            AnyCalendarKind::Japanese
        } else if *x == value!("indian") {
            AnyCalendarKind::Indian
        } else if *x == value!("coptic") {
            AnyCalendarKind::Coptic
        } else if *x == value!("iso") {
            AnyCalendarKind::Iso
        } else if *x == value!("ethiopic") {
            AnyCalendarKind::Ethiopic
        } else if *x == value!("ethioaa") {
            AnyCalendarKind::Ethioaa
        } else {
            return None;
        })
    }

    pub fn as_bcp47(&self) -> &'static str {
        match *self {
            AnyCalendarKind::Gregorian => "gregory",
            AnyCalendarKind::Buddhist => "buddhist",
            AnyCalendarKind::Japanese => "japanese",
            AnyCalendarKind::Indian => "indian",
            AnyCalendarKind::Coptic => "coptic",
            AnyCalendarKind::Iso => "iso",
            AnyCalendarKind::Ethiopic => "ethiopic",
            AnyCalendarKind::Ethioaa => "ethioaa",
        }
    }

    pub fn from_locale(l: &Locale) -> Option<Self> {
        l.extensions
            .unicode
            .keywords
            .get(&key!("ca"))
            .and_then(Self::from_bcp47)
    }
}

impl fmt::Display for AnyCalendarKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<C: IncludedInAnyCalendar> From<C> for AnyCalendar {
    fn from(c: C) -> AnyCalendar {
        c.to_any()
    }
}

/// Trait for calendars that may be converted to [`AnyCalendar`]
pub trait IncludedInAnyCalendar: Calendar + Sized {
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
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner;
}

impl IncludedInAnyCalendar for Gregorian {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Gregorian(Gregorian)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Gregorian(Gregorian)
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Gregorian(*d)
    }
}

impl IncludedInAnyCalendar for Buddhist {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Buddhist(Buddhist)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Buddhist(Buddhist)
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Buddhist(*d)
    }
}

impl IncludedInAnyCalendar for Japanese {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Japanese(self)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Japanese(self.clone())
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Japanese(*d)
    }
}

impl IncludedInAnyCalendar for Ethiopic {
    // Amete Mihret calendars are the default
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Ethiopic(self)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Ethiopic(*self)
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Ethiopic(*d)
    }
}

impl IncludedInAnyCalendar for Indian {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Indian(Indian)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Indian(Indian)
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Indian(*d)
    }
}

impl IncludedInAnyCalendar for Coptic {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Coptic(Coptic)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Coptic(Coptic)
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Coptic(*d)
    }
}

impl IncludedInAnyCalendar for Iso {
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Iso(Iso)
    }
    fn to_any_cloned(&self) -> AnyCalendar {
        AnyCalendar::Iso(Iso)
    }
    fn date_to_any(d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Iso(*d)
    }
}
