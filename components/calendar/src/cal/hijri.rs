// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Hijri calendars.
//!
//! ```rust
//! use icu::calendar::cal::Hijri;
//! use icu::calendar::Date;
//!
//! let hijri = Hijri::new_simulated_mecca();
//! let hijri_date =
//!     Date::try_new_hijri_with_calendar(1348, 10, 11, hijri)
//!         .expect("Failed to initialize Hijri Date instance.");
//!
//! assert_eq!(hijri_date.era_year().year, 1348);
//! assert_eq!(hijri_date.month().ordinal, 10);
//! assert_eq!(hijri_date.day_of_month().0, 11);
//! ```

use crate::cal::iso::{Iso, IsoDateInner};
use crate::calendar_arithmetic::PrecomputedDataSource;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::error::{range_check, DateError};
use crate::types::EraYear;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use crate::{AsCalendar, RangeError};
use calendrical_calculations::islamic::{
    ISLAMIC_EPOCH_FRIDAY, ISLAMIC_EPOCH_THURSDAY, WELL_BEHAVED_ASTRONOMICAL_RANGE,
};
use calendrical_calculations::rata_die::RataDie;
use core::fmt::Debug;
use icu_locale_core::preferences::extensions::unicode::keywords::{
    CalendarAlgorithm, HijriCalendarAlgorithm,
};
use icu_provider::prelude::*;
use tinystr::tinystr;

#[path = "hijri/simulated_mecca_data.rs"]
mod simulated_mecca_data;
#[path = "hijri/ummalqura_data.rs"]
mod ummalqura_data;

fn era_year(monotonic_year: i32) -> EraYear {
    if monotonic_year > 0 {
        types::EraYear {
            era: tinystr!(16, "ah"),
            era_index: Some(0),
            year: monotonic_year,
            monotonic_year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        }
    } else {
        types::EraYear {
            era: tinystr!(16, "bh"),
            era_index: Some(1),
            year: 1 - monotonic_year,
            monotonic_year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        }
    }
}

/// The [Hijri Calendar](https://en.wikipedia.org/wiki/Islamic_calendar)
///
/// There are many variants of this calendar, using different lunar observations or calculations
/// (see [`HijriSighting`]).
///
/// # Era codes
///
/// This calendar uses two era codes: `ah`, and `bh`, corresponding to the Anno Hegirae and Before Hijrah eras
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Clone, Debug, Default, Copy)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct Hijri<S>(pub S);

/// Defines a variant of the [`Hijri`] calendar.
///
/// "Sightings" can either be actual observations, or agreed-upon rules.
///
/// This crate includes the [`UmmAlQura`], [`AstronomicalSimulation`], and [`TabularAlgorithm`], other
/// sightings can be implemented by users.
pub trait HijriSighting: Clone + Debug {
    /// The first day of the year.
    fn start_day(&self, monotonic_year: i32) -> RataDie;

    /// Returns whether the given (1-indexed) month has 30 days.
    fn is_month_long(&self, monotonic_year: i32, month: u8) -> bool;

    /// [`Self::is_month_long`] for a whole year at a time.
    fn month_lengths(&self, monotonic_year: i32) -> [bool; 12] {
        core::array::from_fn(|i| self.is_month_long(monotonic_year, i as u8 + 1))
    }

    /// If calculations can be reused between [`Self::start_day`] and [`Self::month_lengths`],
    /// implement this method; it is the one actually called by calendar code.
    fn start_day_and_month_lengths(&self, monotonic_year: i32) -> (RataDie, [bool; 12]) {
        (
            self.start_day(monotonic_year),
            self.month_lengths(monotonic_year),
        )
    }

    /// The BCP-47 [`CalendarAlgorithm`] for the Hijri calendar using this sighting, if defined.
    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        None
    }

    /// The debug name for this sighting.
    fn debug_name(&self) -> &'static str {
        "Hijri (custom sighting)"
    }
}

/// An astrononmical simulation for a particular location.
///
/// These simulations are known to not necessarily match sightings on the ground, but are included for
/// completeness.
#[derive(Clone, Debug)]
pub struct AstronomicalSimulation {
    pub(crate) location: HijriSimulatedLocation,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub(crate) enum HijriSimulatedLocation {
    Mecca,
}

impl HijriSighting for AstronomicalSimulation {
    fn debug_name(&self) -> &'static str {
        "Hijri (simulated)"
    }

    fn start_day(&self, monotonic_year: i32) -> RataDie {
        self.start_day_and_month_lengths(monotonic_year).0
    }

    fn is_month_long(&self, monotonic_year: i32, month: u8) -> bool {
        self.start_day_and_month_lengths(monotonic_year)
            .1
            .get(month as usize - 1)
            .copied()
            .unwrap_or_default()
    }

    fn start_day_and_month_lengths(&self, monotonic_year: i32) -> (RataDie, [bool; 12]) {
        if let Some(&packed) = usize::try_from(monotonic_year - simulated_mecca_data::STARTING_YEAR)
            .ok()
            .and_then(|i| simulated_mecca_data::DATA.get(i))
        {
            return packed.unpack(monotonic_year);
        }

        let location = match self.location {
            HijriSimulatedLocation::Mecca => calendrical_calculations::islamic::MECCA,
        };

        let start_day = calendrical_calculations::islamic::fixed_from_observational_islamic(
            monotonic_year,
            1,
            1,
            location,
        );
        let next_start_day = calendrical_calculations::islamic::fixed_from_observational_islamic(
            monotonic_year + 1,
            1,
            1,
            location,
        );
        match (next_start_day - start_day) as u16 {
            LONG_YEAR_LEN | SHORT_YEAR_LEN => (),
            353 => {
                icu_provider::log::trace!(
                    "({}) Found year {monotonic_year} AH with length {}. See <https://github.com/unicode-org/icu4x/issues/4930>",
                    self.debug_name(),
                    next_start_day - start_day
                );
            }
            other => {
                debug_assert!(
                    !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&start_day),
                    "({}) Found year {monotonic_year} AH with length {}!",
                    self.debug_name(),
                    other
                )
            }
        }

        let month_lengths = {
            let mut excess_days = 0;
            let mut month_lengths = core::array::from_fn(|month_idx| {
                let days_in_month =
                    calendrical_calculations::islamic::observational_islamic_month_days(
                        monotonic_year,
                        month_idx as u8 + 1,
                        location,
                    );
                match days_in_month {
                    29 => false,
                    30 => true,
                    31 => {
                        icu_provider::log::trace!(
                            "({}) Found year {monotonic_year} AH with month length {days_in_month} for month {}.",
                            self.debug_name(),
                            month_idx + 1
                        );
                        excess_days += 1;
                        true
                    }
                    _ => {
                        debug_assert!(
                            !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&start_day),
                            "({}) Found year {monotonic_year} AH with month length {days_in_month} for month {}!",
                            self.debug_name(),
                            month_idx + 1
                        );
                        false
                    }
                }
            });
            // To maintain invariants for calendar arithmetic, if astronomy finds
            // a 31-day month, "move" the day to the first 29-day month in the
            // same year to maintain all months at 29 or 30 days.
            if excess_days != 0 {
                debug_assert!(
                    excess_days == 1 || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&start_day),
                    "({}) Found year {monotonic_year} AH with more than one excess day!",
                    self.debug_name()
                );
                if let Some(l) = month_lengths.iter_mut().find(|l| !(**l)) {
                    *l = true;
                }
            }
            month_lengths
        };
        (start_day, month_lengths)
    }
}

/// The [Umm al-Qura Hijri sighting](https://en.wikipedia.org/wiki/Islamic_calendar#Saudi_Arabia's_Umm_al-Qura_calendar)
///
/// This is the official calendar in Saudi Arabia.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct UmmAlQura;

impl HijriSighting for UmmAlQura {
    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        Some(CalendarAlgorithm::Hijri(Some(
            HijriCalendarAlgorithm::Umalqura,
        )))
    }

    fn debug_name(&self) -> &'static str {
        "Hijri (Umm al-Qura)"
    }

    fn start_day(&self, monotonic_year: i32) -> RataDie {
        self.start_day_and_month_lengths(monotonic_year).0
    }

    fn is_month_long(&self, monotonic_year: i32, month: u8) -> bool {
        self.start_day_and_month_lengths(monotonic_year)
            .1
            .get(month as usize - 1)
            .copied()
            .unwrap_or_default()
    }

    fn start_day_and_month_lengths(&self, monotonic_year: i32) -> (RataDie, [bool; 12]) {
        if let Some(&packed) = usize::try_from(monotonic_year - ummalqura_data::STARTING_YEAR)
            .ok()
            .and_then(|i| ummalqura_data::DATA.get(i))
        {
            packed.unpack(monotonic_year)
        } else {
            TabularAlgorithm {
                leap_years: HijriTabularLeapYears::TypeII,
                epoch: HijriTabularEpoch::Friday,
            }
            .start_day_and_month_lengths(monotonic_year)
        }
    }
}

/// See [`HijriTabularEpoch`] and [`HijriTabularLeapYears`] for customization.
///
/// The most common version of this sighting uses [`HijriTabularEpoch::Friday`] and [`HijriTabularLeapYears::TypeII`].
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct TabularAlgorithm {
    pub(crate) leap_years: HijriTabularLeapYears,
    pub(crate) epoch: HijriTabularEpoch,
}

impl TabularAlgorithm {
    /// Construct a new [`TabularAlgorithm`] with the given leap year rule and epoch.
    pub const fn new(leap_years: HijriTabularLeapYears, epoch: HijriTabularEpoch) -> Self {
        Self { epoch, leap_years }
    }
}

impl HijriSighting for TabularAlgorithm {
    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        Some(match (self.epoch, self.leap_years) {
            (HijriTabularEpoch::Friday, HijriTabularLeapYears::TypeII) => {
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil))
            }
            (HijriTabularEpoch::Thursday, HijriTabularLeapYears::TypeII) => {
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla))
            }
        })
    }

    fn debug_name(&self) -> &'static str {
        match self.epoch {
            HijriTabularEpoch::Friday => "Hijri (civil)",
            HijriTabularEpoch::Thursday => "Hijri (astronomical)",
        }
    }

    fn is_month_long(&self, monotonic_year: i32, month: u8) -> bool {
        month % 2 == 1
            || month == 12
                && match self.leap_years {
                    HijriTabularLeapYears::TypeII => (14 + 11 * monotonic_year).rem_euclid(30) < 11,
                }
    }

    fn start_day(&self, monotonic_year: i32) -> RataDie {
        calendrical_calculations::islamic::fixed_from_tabular_islamic(
            monotonic_year,
            1,
            1,
            self.epoch.rata_die(),
        )
    }
}

impl Hijri<AstronomicalSimulation> {
    /// Use [`Self::new_simulated_mecca`].
    #[cfg(feature = "compiled_data")]
    #[deprecated(since = "2.1.0", note = "use `Hijri::new_simulated_mecca`")]
    pub const fn new_mecca() -> Self {
        Self::new_simulated_mecca()
    }

    /// Creates a new [`Hijri`] using simulated sightings at Mecca, with some compiled data containing precomputed data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub const fn new_simulated_mecca() -> Self {
        Self(AstronomicalSimulation {
            location: HijriSimulatedLocation::Mecca,
        })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::new)]
    #[deprecated(since = "2.1.0", note = "use `Hijri::new_simulated_mecca`")]
    pub fn try_new_mecca_with_buffer_provider(
        _provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Ok(Self::new_simulated_mecca())
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_mecca)]
    #[deprecated(since = "2.1.0", note = "use `Hijri::new_simulated_mecca`")]
    pub fn try_new_mecca_unstable<D: ?Sized>(_provider: &D) -> Result<Self, DataError> {
        Ok(Self::new_simulated_mecca())
    }

    /// Use [`Self::new_simulated_mecca`].
    #[deprecated(since = "2.1.0", note = "use `Hijri::new_simulated_mecca`")]
    pub const fn new_mecca_always_calculating() -> Self {
        Self::new_simulated_mecca()
    }
}

impl Hijri<UmmAlQura> {
    /// Use [`Self::new_umm_al_qura`]
    #[deprecated(since = "2.1.0", note = "use `Self::new_umm_al_qura`")]
    pub const fn new() -> Self {
        Self(UmmAlQura)
    }

    /// Creates a Hijri calendar using [`UmmAlQura`].
    pub const fn new_umm_al_qura() -> Self {
        Self(UmmAlQura)
    }
}

/// The epoch for the [`TabularAlgorithm`] sighting.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum HijriTabularEpoch {
    /// Thusday July 15, 622 AD (0622-07-18 ISO)
    Thursday,
    /// Friday July 16, 622 AD (0622-07-19 ISO)
    Friday,
}

impl HijriTabularEpoch {
    fn rata_die(self) -> RataDie {
        match self {
            Self::Thursday => ISLAMIC_EPOCH_THURSDAY,
            Self::Friday => ISLAMIC_EPOCH_FRIDAY,
        }
    }
}

/// The leap year rule for the [`TabularAlgorithm`] sighting.
///
/// This specifies which years of a 30-year cycle have an additional day at
/// the end of the year.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum HijriTabularLeapYears {
    /// Leap years 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29
    TypeII,
}

impl Hijri<TabularAlgorithm> {
    /// Use [`Self::new_tabular`]
    #[deprecated(since = "2.1.0", note = "use `Hijri::new_tabular`")]
    pub const fn new(leap_years: HijriTabularLeapYears, epoch: HijriTabularEpoch) -> Self {
        Hijri::new_tabular(leap_years, epoch)
    }

    /// Construct a new [`Hijri`] with tabular sighting and the given leap year rule and epoch.
    pub const fn new_tabular(leap_years: HijriTabularLeapYears, epoch: HijriTabularEpoch) -> Self {
        Self(TabularAlgorithm::new(leap_years, epoch))
    }
}

/// Information about a Hijri year.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HijriYearInfo {
    month_lengths: [bool; 12],
    start_day: RataDie,
    monotonic_year: i32,
}

impl From<HijriYearInfo> for i32 {
    fn from(value: HijriYearInfo) -> Self {
        value.monotonic_year
    }
}

const LONG_YEAR_LEN: u16 = 355;
const SHORT_YEAR_LEN: u16 = 354;

impl HijriYearInfo {
    /// The number of days in a given 1-indexed month
    fn days_in_month(self, month: u8) -> u8 {
        let Some(zero_month) = month.checked_sub(1) else {
            return 29;
        };

        if self.month_lengths.get(zero_month as usize) == Some(&true) {
            30
        } else {
            29
        }
    }

    fn days_in_year(self) -> u16 {
        self.last_day_of_month(12)
    }

    /// Get the date's R.D. given (m, d) in this info's year
    fn md_to_rd(self, month: u8, day: u8) -> RataDie {
        let month_offset = if month == 1 {
            0
        } else {
            self.last_day_of_month(month - 1)
        };
        self.start_day + month_offset as i64 + (day - 1) as i64
    }

    fn md_from_rd(self, rd: RataDie) -> (u8, u8) {
        let day_of_year = (rd - self.start_day) as u16;
        debug_assert!(day_of_year < 360 || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd));
        // We divide by 30, not 29, to account for the case where all months before this
        // were length 30 (possible near the beginning of the year)
        let mut month = (day_of_year / 30) as u8 + 1;

        let day_of_year = day_of_year + 1;
        let mut last_day_of_month = self.last_day_of_month(month);
        let mut last_day_of_prev_month = if month == 1 {
            0
        } else {
            self.last_day_of_month(month - 1)
        };

        while day_of_year > last_day_of_month && month <= 12 {
            month += 1;
            last_day_of_prev_month = last_day_of_month;
            last_day_of_month = self.last_day_of_month(month);
        }
        debug_assert!(
            day_of_year - last_day_of_prev_month <= 30
                || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Found day {} that doesn't fit in month!",
            day_of_year - last_day_of_prev_month
        );
        let day = (day_of_year - last_day_of_prev_month) as u8;
        (month, day)
    }

    // Which day of year is the last day of a month (month is 1-indexed)
    fn last_day_of_month(self, month: u8) -> u16 {
        29 * month as u16
            + self
                .month_lengths
                .get(..month as usize)
                .unwrap_or_default()
                .iter()
                .filter(|&&x| x)
                .count() as u16
    }
}

impl<A: AsCalendar<Calendar = Hijri<AstronomicalSimulation>>> Date<A> {
    /// Deprecated
    #[deprecated(since = "2.1.0", note = "use `Date::try_new_hijri_with_calendar`")]
    pub fn try_new_simulated_hijri_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        Date::try_new_hijri_with_calendar(year, month, day, calendar)
    }
}

#[allow(clippy::derived_hash_with_manual_eq)] // bounds
#[derive(Clone, Debug, Hash)]
/// The inner date type used for representing [`Date`]s of [`HijriObservational`]. See [`Date`] and [`HijriObservational`] for more details.
pub struct HijriDateInner<S: HijriSighting>(ArithmeticDate<Hijri<S>>);

impl<S: HijriSighting> Copy for HijriDateInner<S> {}
impl<S: HijriSighting> PartialEq for HijriDateInner<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<S: HijriSighting> Eq for HijriDateInner<S> {}

impl<S: HijriSighting> CalendarArithmetic for Hijri<S> {
    type YearInfo = HijriYearInfo;

    fn days_in_provided_month(year: Self::YearInfo, month: u8) -> u8 {
        year.days_in_month(month)
    }

    fn months_in_provided_year(_year: Self::YearInfo) -> u8 {
        12
    }

    fn days_in_provided_year(year: Self::YearInfo) -> u16 {
        year.days_in_year()
    }

    // As an true lunar calendar, it does not have leap years.
    fn provided_year_is_leap(year: Self::YearInfo) -> bool {
        year.days_in_year() != SHORT_YEAR_LEN
    }

    fn last_month_day_in_provided_year(year: Self::YearInfo) -> (u8, u8) {
        let days = Self::days_in_provided_month(year, 12);

        (12, days)
    }
}

impl<S: HijriSighting> crate::cal::scaffold::UnstableSealed for Hijri<S> {}
impl<S: HijriSighting> Calendar for Hijri<S> {
    type DateInner = HijriDateInner<S>;
    type Year = types::EraYear;
    fn from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            None => year,
            Some("ah") => range_check(year, "year", 1..)?,
            Some("bh") => 1 - range_check(year, "year", 1..)?,
            Some(_) => return Err(DateError::UnknownEra),
        };
        let Some((month, false)) = month_code.parsed() else {
            return Err(DateError::UnknownMonthCode(month_code));
        };
        Ok(HijriDateInner(ArithmeticDate::new_from_ordinals(
            self.load_or_compute_info(year),
            month,
            day,
        )?))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        // +1 because the epoch is new year of year 1
        // truncating instead of flooring does not matter, as this is well-defined for
        // positive years only
        let extended_year = ((rd - calendrical_calculations::islamic::ISLAMIC_EPOCH_FRIDAY) as f64
            / calendrical_calculations::islamic::MEAN_YEAR_LENGTH)
            as i32
            + 1;

        let year = self.load_or_compute_info(extended_year);

        let y = if rd < year.start_day {
            self.load_or_compute_info(extended_year - 1)
        } else {
            let next_year = self.load_or_compute_info(extended_year + 1);
            if rd < next_year.start_day {
                year
            } else {
                next_year
            }
        };
        let (m, d) = y.md_from_rd(rd);
        HijriDateInner(ArithmeticDate::new_unchecked(y, m, d))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        date.0.year.md_to_rd(date.0.month, date.0.day)
    }

    fn from_iso(&self, iso: IsoDateInner) -> Self::DateInner {
        self.from_rata_die(Iso.to_rata_die(&iso))
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        Iso.from_rata_die(self.to_rata_die(date))
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset, self)
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        _largest_unit: DateDurationUnit,
        _smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        date1.0.until(date2.0, _largest_unit, _smallest_unit)
    }

    fn debug_name(&self) -> &'static str {
        self.0.debug_name()
    }

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        era_year(date.0.monotonic_year())
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::provided_year_is_leap(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        date.0.day_of_year()
    }

    fn calendar_algorithm(&self) -> Option<crate::preferences::CalendarAlgorithm> {
        self.0.calendar_algorithm()
    }
}

impl<S: HijriSighting> PrecomputedDataSource<HijriYearInfo> for Hijri<S> {
    fn load_or_compute_info(&self, monotonic_year: i32) -> HijriYearInfo {
        let (start_day, month_lengths) = self.0.start_day_and_month_lengths(monotonic_year);
        HijriYearInfo {
            month_lengths,
            start_day,
            monotonic_year,
        }
    }
}

impl<A: AsCalendar<Calendar = Hijri<S>>, S: HijriSighting> Date<A> {
    /// Construct new Hijri Date.
    ///
    /// ```rust
    /// use icu::calendar::cal::Hijri;
    /// use icu::calendar::Date;
    ///
    /// let hijri = Hijri::new_simulated_mecca();
    ///
    /// let date_hijri =
    ///     Date::try_new_hijri_with_calendar(1392, 4, 25, hijri)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.era_year().year, 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_hijri_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Self, RangeError> {
        let y = calendar.as_calendar().load_or_compute_info(year);
        Ok(Date::from_raw(
            HijriDateInner(ArithmeticDate::new_from_ordinals(y, month, day)?),
            calendar,
        ))
    }
}

impl Date<Hijri<UmmAlQura>> {
    /// Construct new Hijri Umm al-Qura Date.
    ///
    /// ```rust
    /// use icu::calendar::cal::Hijri;
    /// use icu::calendar::Date;
    ///
    /// let date_hijri = Date::try_new_ummalqura(1392, 4, 25)
    ///     .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.era_year().year, 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_ummalqura(year: i32, month: u8, day: u8) -> Result<Self, RangeError> {
        Date::try_new_hijri_with_calendar(year, month, day, Hijri::new_umm_al_qura())
    }
}

impl<A: AsCalendar<Calendar = Hijri<TabularAlgorithm>>> Date<A> {
    /// Deprecated
    #[deprecated(since = "2.1.0", note = "use `Date::try_new_hijri_with_calendar")]
    pub fn try_new_hijri_tabular_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        Date::try_new_hijri_with_calendar(year, month, day, calendar)
    }
}

#[cfg(test)]
mod test {
    use types::MonthCode;

    use super::*;
    use crate::Ref;

    const START_YEAR: i32 = -1245;
    const END_YEAR: i32 = 1518;

    #[derive(Debug)]
    struct DateCase {
        year: i32,
        month: u8,
        day: u8,
    }

    static TEST_RD: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];

    static UMMALQURA_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 9,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 23,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 1,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 6,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 17,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 13,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 22,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 7,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 20,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 6,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 1,
        },
        DateCase {
            year: 960,
            month: 9,
            day: 30,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 27,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 18,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 4,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 3,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 10,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 11,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 20,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 7,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 14,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 7,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 12,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 13,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 5,
        },
    ];

    static SIMULATED_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 10,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 25,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 2,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 7,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 13,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 22,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 7,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 21,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 793,
            month: 6,
            day: 29,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 6,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 960,
            month: 9,
            day: 30,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 27,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 18,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 4,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 4,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 10,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 11,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 20,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 19,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 7,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 13,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 7,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 12,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 12,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 5,
        },
    ];

    static ARITHMETIC_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 9,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 23,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 1,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 6,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 17,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 13,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 22,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 7,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 20,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 6,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 1,
        },
        DateCase {
            year: 960,
            month: 9,
            day: 30,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 27,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 18,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 4,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 3,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 10,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 11,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 19,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 8,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 13,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 7,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 13,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 12,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 5,
        },
    ];

    static ASTRONOMICAL_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 10,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 24,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 2,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 7,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 4,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 14,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 23,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 8,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 8,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 21,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 2,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 960,
            month: 10,
            day: 1,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 28,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 19,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 5,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 4,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 11,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 12,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 22,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 20,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 9,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 14,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 8,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 14,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 13,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 6,
        },
    ];

    #[test]
    fn test_simulated_hijri_from_rd() {
        let calendar = Hijri::new_simulated_mecca();
        let calendar = Ref(&calendar);
        for (case, f_date) in SIMULATED_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar)
                .unwrap();
            let iso = Date::from_rata_die(RataDie::new(*f_date), Iso);

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_simulated_hijri() {
        let calendar = Hijri::new_simulated_mecca();
        let calendar = Ref(&calendar);
        for (case, f_date) in SIMULATED_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar)
                .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_hijri() {
        let calendar = Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Friday);
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar)
                .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_hijri_from_rd() {
        let calendar = Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Friday);
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar)
                .unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_hijri_tbla() {
        let calendar =
            Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday);
        let calendar = Ref(&calendar);
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar)
                .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_hijri_tbla_from_rd() {
        let calendar =
            Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday);
        let calendar = Ref(&calendar);
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar)
                .unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_saudi_hijri_from_rd() {
        let calendar = Hijri::new_umm_al_qura();
        let calendar = Ref(&calendar);
        for (case, f_date) in UMMALQURA_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_ummalqura(case.year, case.month, case.day).unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_saudi_hijri() {
        for (case, f_date) in UMMALQURA_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_ummalqura(case.year, case.month, case.day).unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[ignore] // slow
    #[test]
    fn test_days_in_provided_year_simulated() {
        let calendar = Hijri::new_simulated_mecca();
        let calendar = Ref(&calendar);
        // -1245 1 1 = -214526 (R.D Date)
        // 1518 1 1 = 764589 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| {
                Hijri::new_simulated_mecca()
                    .load_or_compute_info(year)
                    .days_in_year() as i64
            })
            .sum();
        let expected_number_of_days = Date::try_new_hijri_with_calendar(END_YEAR, 1, 1, calendar)
            .unwrap()
            .to_rata_die()
            - Date::try_new_hijri_with_calendar(START_YEAR, 1, 1, calendar)
                .unwrap()
                .to_rata_die(); // The number of days between Hijri years -1245 and 1518
        let tolerance = 1; // One day tolerance (See Astronomical::month_length for more context)

        assert!(
            (sum_days_in_year - expected_number_of_days).abs() <= tolerance,
            "Difference between sum_days_in_year and expected_number_of_days is more than the tolerance"
        );
    }

    #[ignore] // slow
    #[test]
    fn test_days_in_provided_year_ummalqura() {
        // -1245 1 1 = -214528 (R.D Date)
        // 1518 1 1 = 764588 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| {
                Hijri::new_umm_al_qura()
                    .load_or_compute_info(year)
                    .days_in_year() as i64
            })
            .sum();
        let expected_number_of_days = Date::try_new_ummalqura(END_YEAR, 1, 1)
            .unwrap()
            .to_rata_die()
            - (Date::try_new_ummalqura(START_YEAR, 1, 1).unwrap()).to_rata_die(); // The number of days between Umm al-Qura Hijri years -1245 and 1518

        assert_eq!(sum_days_in_year, expected_number_of_days);
    }

    #[test]
    fn test_regression_3868() {
        // This date used to panic on creation
        let iso = Date::try_new_iso(2011, 4, 4).unwrap();
        let hijri = iso.to_calendar(Hijri::new_umm_al_qura());
        // Data from https://www.ummulqura.org.sa/Index.aspx
        assert_eq!(hijri.day_of_month().0, 30);
        assert_eq!(hijri.month().ordinal, 4);
        assert_eq!(hijri.era_year().year, 1432);
    }

    #[test]
    fn test_regression_4914() {
        // https://github.com/unicode-org/icu4x/issues/4914
        let dt = Hijri::new_umm_al_qura()
            .from_codes(Some("bh"), 6824, MonthCode::new_normal(1).unwrap(), 1)
            .unwrap();
        assert_eq!(dt.0.day, 1);
        assert_eq!(dt.0.month, 1);
        assert_eq!(dt.0.year.monotonic_year, -6823);
    }

    #[test]
    fn test_regression_5069_uaq() {
        let cached = Hijri::new_umm_al_qura();

        let cached = crate::Ref(&cached);

        let dt_cached = Date::try_new_ummalqura(1391, 1, 29).unwrap();

        assert_eq!(dt_cached.to_iso().to_calendar(cached), dt_cached);
    }

    #[test]
    fn test_regression_5069_obs() {
        let cal = Hijri::new_simulated_mecca();

        let cal = crate::Ref(&cal);

        let dt = Date::try_new_hijri_with_calendar(1390, 1, 30, cal).unwrap();

        assert_eq!(dt.to_iso().to_calendar(cal), dt);

        let dt = Date::try_new_iso(2000, 5, 5).unwrap();

        assert!(dt.to_calendar(cal).day_of_month().0 > 0);
    }

    #[test]
    fn test_regression_6197() {
        let cached = Hijri::new_umm_al_qura();

        let cached = crate::Ref(&cached);

        let iso = Date::try_new_iso(2025, 2, 26).unwrap();

        let cached = iso.to_calendar(cached);

        // Data from https://www.ummulqura.org.sa/
        assert_eq!(
            (
                cached.day_of_month().0,
                cached.month().ordinal,
                cached.era_year().year
            ),
            (27, 8, 1446)
        );
    }
}
