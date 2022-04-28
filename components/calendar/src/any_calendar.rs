// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Module for working with multiple calendars at once

use crate::buddhist::Buddhist;
use crate::coptic::Coptic;
use crate::gregorian::Gregorian;
use crate::indian::Indian;
use crate::iso::Iso;
use crate::japanese::Japanese;

use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};

/// This is a calendar that encompasses all formattable calendars supported by this crate
///
/// This allows for the construction of [`Date`] objects that have their calendar known at runtime.
#[non_exhaustive]
pub enum AnyCalendar {
    Gregorian(Gregorian),
    Buddhist(Buddhist),
    Japanese(Japanese),
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
            Self::Indian(_) => "AnyCalendar (Indian)",
            Self::Coptic(_) => "AnyCalendar (Coptic)",
            Self::Iso(_) => "AnyCalendar (Iso)",
        }
    }
}

impl AnyCalendar {
    fn calendar_name(&self) -> &'static str {
        match *self {
            Self::Gregorian(_) => "Gregorian",
            Self::Buddhist(_) => "Buddhist",
            Self::Japanese(_) => "Japanese",
            Self::Indian(_) => "Indian",
            Self::Coptic(_) => "Coptic",
            Self::Iso(_) => "Iso",
        }
    }
}

impl AnyDateInner {
    fn calendar_name(&self) -> &'static str {
        match *self {
            AnyDateInner::Gregorian(_) => "Gregorian",
            AnyDateInner::Buddhist(_) => "Buddhist",
            AnyDateInner::Japanese(_) => "Japanese",
            AnyDateInner::Indian(_) => "Indian",
            AnyDateInner::Coptic(_) => "Coptic",
            AnyDateInner::Iso(_) => "Iso",
        }
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
