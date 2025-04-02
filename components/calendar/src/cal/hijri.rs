// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Hijri calendars.
//!
//! ```rust
//! use icu::calendar::cal::HijriSimulated;
//! use icu::calendar::Date;
//!
//! let hijri = HijriSimulated::new_mecca_always_calculating();
//! let hijri_date = Date::try_new_simulated_hijri_with_calendar(
//!     1348, 10, 11, hijri,
//! )
//! .expect("Failed to initialize Hijri Date instance.");
//!
//! assert_eq!(hijri_date.year().era_year_or_extended(), 1348);
//! assert_eq!(hijri_date.month().ordinal, 10);
//! assert_eq!(hijri_date.day_of_month().0, 11);
//! ```

use crate::cal::iso::{Iso, IsoDateInner};
use crate::calendar_arithmetic::PrecomputedDataSource;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::error::DateError;
use crate::provider::hijri::PackedHijriYearInfo;
use crate::provider::hijri::{CalendarHijriSimulatedMeccaV1, HijriData};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use crate::{AsCalendar, RangeError};
use calendrical_calculations::islamic::{ISLAMIC_EPOCH_FRIDAY, ISLAMIC_EPOCH_THURSDAY};
use calendrical_calculations::rata_die::RataDie;
use icu_provider::marker::ErasedMarker;
use icu_provider::prelude::*;
use tinystr::tinystr;

fn year_as_hijri(standard_era: tinystr::TinyStr16, year: i32) -> types::YearInfo {
    types::YearInfo::new(
        year,
        types::EraYear {
            formatting_era: types::FormattingEra::Index(0, tinystr!(16, "AH")),
            standard_era: standard_era.into(),
            era_year: year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        },
    )
}

/// The [simulated Hijri Calendar](https://en.wikipedia.org/wiki/Islamic_calendar)
///
/// # Era codes
///
/// This calendar uses a single era code, `islamic-rgsa` (alias `ah`), Anno Hegirae.
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Clone, Debug)]
pub struct HijriSimulated {
    pub(crate) location: HijriSimulatedLocation,
    data: Option<DataPayload<ErasedMarker<HijriData<'static>>>>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub(crate) enum HijriSimulatedLocation {
    Mecca,
}

impl HijriSimulatedLocation {
    fn location(self) -> calendrical_calculations::islamic::Location {
        match self {
            Self::Mecca => calendrical_calculations::islamic::MECCA,
        }
    }
}

/// The [Umm al-Qura Hijri Calendar](https://en.wikipedia.org/wiki/Islamic_calendar#Saudi_Arabia's_Umm_al-Qura_calendar)
///
/// This calendar is the official calendar in Saudi Arabia.
///
/// # Era codes
///
/// This calendar uses a single era code, `islamic-umalqura` (alias `ah`), Anno Hegirae.
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct HijriUmmAlQura;

/// The [tabular Hijri Calendar](https://en.wikipedia.org/wiki/Tabular_Islamic_calendar) (astronomical epoch)
///
/// This is a tabular/arithmetic Hijri calendar with leap years (1-based) 2, 5, 7, 10,
/// 13, 16, 18, 21, 24, 26, and 29 in the 30-year cycle.
///
/// # Era codes
///
/// In civil mode, this calendar uses a single era code, `islamic-civil` (aliases `ah`, `islamicc`), Anno Hegirae.
///
/// In astronomical mode, it uses a single era code, `islamic-tbla` (alias `ah`), Anno Hegirae.
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriTabular(pub(crate) RataDie);

impl HijriSimulated {
    /// Creates a new [`HijriSimulated`] for reference location Mecca, with some compiled data containing precomputed calendrical calculations.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_mecca() -> Self {
        Self {
            location: HijriSimulatedLocation::Mecca,
            data: Some(DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_HIJRI_SIMULATED_MECCA_V1,
            )),
        }
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_mecca_with_buffer_provider,
            try_new_mecca_unstable,
            Self,
    ]);

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_mecca)]
    pub fn try_new_mecca_unstable<D: DataProvider<CalendarHijriSimulatedMeccaV1> + ?Sized>(
        provider: &D,
    ) -> Result<Self, DataError> {
        Ok(Self {
            location: HijriSimulatedLocation::Mecca,
            data: Some(provider.load(Default::default())?.payload.cast()),
        })
    }

    /// Construct a new [`HijriSimulated`] for reference location Mecca, without any precomputed calendrical calculations.
    pub const fn new_mecca_always_calculating() -> Self {
        Self {
            location: HijriSimulatedLocation::Mecca,
            data: None,
        }
    }

    /// Compute a cache for this calendar
    #[cfg(feature = "datagen")]
    pub fn build_cache(&self, extended_years: core::ops::Range<i32>) -> HijriData<'static> {
        let data = extended_years
            .clone()
            .map(|year| self.location.compute_year_info(year).pack())
            .collect();
        HijriData {
            first_extended_year: extended_years.start,
            data,
        }
    }
}

impl HijriUmmAlQura {
    /// Creates a new [`HijriUmmAlQura`].
    pub const fn new() -> Self {
        Self
    }
}

impl HijriTabular {
    /// Construct a new [`HijriTabular`] with the civil/Friday epoch. That is,
    /// the year 1 starts on Friday July 16, 622 AD (0622-07-19 ISO).
    ///
    /// This is the most common version of the tabular Hijri calendar.
    pub const fn new_civil_epoch() -> Self {
        Self(ISLAMIC_EPOCH_FRIDAY)
    }

    /// Construct a new [`HijriTabular`] with the astronomical/Thursday epoch.
    /// That is, the AH era starts on Thusday July 15, 622 AD (0622-07-18 ISO).
    ///
    /// This version of the calendar is also known as the "Microsoft Kuwaiti calendar".
    pub const fn new_astronomical_epoch() -> Self {
        Self(ISLAMIC_EPOCH_THURSDAY)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HijriYearInfo {
    month_lengths: [bool; 12],
    start_day: RataDie,
    value: i32,
}

impl From<HijriYearInfo> for i32 {
    fn from(value: HijriYearInfo) -> Self {
        value.value
    }
}

impl HijriData<'_> {
    /// Get the cached data for a given extended year
    fn get(&self, extended_year: i32) -> Option<HijriYearInfo> {
        Some(HijriYearInfo::unpack(
            extended_year,
            self.data
                .get(usize::try_from(extended_year - self.first_extended_year).ok()?)?,
        ))
    }
}

const LONG_YEAR_LEN: u16 = 355;
const SHORT_YEAR_LEN: u16 = 354;

impl HijriYearInfo {
    #[cfg(feature = "datagen")]
    fn pack(&self) -> PackedHijriYearInfo {
        PackedHijriYearInfo::new(self.value, self.month_lengths, self.start_day)
    }

    fn unpack(extended_year: i32, packed: PackedHijriYearInfo) -> Self {
        let (month_lengths, start_day) = packed.unpack(extended_year);

        HijriYearInfo {
            month_lengths,
            start_day,
            value: extended_year,
        }
    }

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
        debug_assert!(day_of_year < 360);
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
            day_of_year - last_day_of_prev_month <= 30,
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

impl PrecomputedDataSource<HijriYearInfo> for HijriSimulated {
    fn load_or_compute_info(&self, extended_year: i32) -> HijriYearInfo {
        self.data
            .as_ref()
            .and_then(|d| d.get().get(extended_year))
            .unwrap_or_else(|| self.location.compute_year_info(extended_year))
    }
}

/// The inner date type used for representing [`Date`]s of [`HijriSimulated`]. See [`Date`] and [`HijriSimulated`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriSimulatedDateInner(ArithmeticDate<HijriSimulated>);

impl CalendarArithmetic for HijriSimulated {
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

impl Calendar for HijriSimulated {
    type DateInner = HijriSimulatedDateInner;
    fn from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            Some("islamic-rgsa" | "ah") | None => year,
            Some(_) => return Err(DateError::UnknownEra),
        };
        let Some((month, false)) = month_code.parsed() else {
            return Err(DateError::UnknownMonthCode(month_code));
        };
        Ok(HijriSimulatedDateInner(ArithmeticDate::new_from_ordinals(
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
        HijriSimulatedDateInner(ArithmeticDate::new_unchecked(y, m, d))
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
        Self::DEBUG_NAME
    }

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        year_as_hijri(tinystr!(16, "islamic-rgsa"), date.0.year.value)
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

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(crate::any_calendar::IntoAnyCalendar::kind(self))
    }
}

impl HijriSimulatedLocation {
    fn compute_year_info(self, extended_year: i32) -> HijriYearInfo {
        let start_day = calendrical_calculations::islamic::fixed_from_observational_islamic(
            extended_year,
            1,
            1,
            self.location(),
        );
        let next_start_day = calendrical_calculations::islamic::fixed_from_observational_islamic(
            extended_year + 1,
            1,
            1,
            self.location(),
        );
        match (next_start_day - start_day) as u16 {
            LONG_YEAR_LEN | SHORT_YEAR_LEN => (),
            353 => {
                icu_provider::log::trace!(
                    "({}) Found year {extended_year} AH with length {}. See <https://github.com/unicode-org/icu4x/issues/4930>",
                    HijriSimulated::DEBUG_NAME,
                    next_start_day - start_day
                );
            }
            other => {
                debug_assert!(
                    false,
                    "({}) Found year {extended_year} AH with length {}!",
                    HijriSimulated::DEBUG_NAME,
                    other
                )
            }
        }

        let month_lengths = {
            let mut excess_days = 0;
            let mut month_lengths = core::array::from_fn(|month_idx| {
                let days_in_month =
                    calendrical_calculations::islamic::observational_islamic_month_days(
                        extended_year,
                        month_idx as u8 + 1,
                        self.location(),
                    );
                match days_in_month {
                    29 => false,
                    30 => true,
                    31 => {
                        icu_provider::log::trace!(
                            "({}) Found year {extended_year} AH with month length {days_in_month} for month {}.",
                            HijriSimulated::DEBUG_NAME,
                            month_idx + 1
                        );
                        excess_days += 1;
                        true
                    }
                    _ => {
                        debug_assert!(
                            false,
                            "({}) Found year {extended_year} AH with month length {days_in_month} for month {}!",
                            HijriSimulated::DEBUG_NAME,
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
                debug_assert_eq!(
                    excess_days,
                    1,
                    "({}) Found year {extended_year} AH with more than one excess day!",
                    HijriSimulated::DEBUG_NAME
                );
                if let Some(l) = month_lengths.iter_mut().find(|l| !(**l)) {
                    *l = true;
                }
            }
            month_lengths
        };
        HijriYearInfo {
            month_lengths,
            start_day,
            value: extended_year,
        }
    }
}

impl HijriSimulated {
    pub(crate) const DEBUG_NAME: &'static str = "Hijri (simulated)";
}

impl<A: AsCalendar<Calendar = HijriSimulated>> Date<A> {
    /// Construct new simulated Hijri Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::cal::HijriSimulated;
    /// use icu::calendar::Date;
    ///
    /// let hijri = HijriSimulated::new_mecca_always_calculating();
    ///
    /// let date_hijri =
    ///     Date::try_new_simulated_hijri_with_calendar(1392, 4, 25, hijri)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.year().era_year_or_extended(), 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_simulated_hijri_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        let y = calendar.as_calendar().load_or_compute_info(year);
        ArithmeticDate::new_from_ordinals(y, month, day)
            .map(HijriSimulatedDateInner)
            .map(|inner| Date::from_raw(inner, calendar))
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`HijriUmmAlQura`]. See [`Date`] and [`HijriUmmAlQura`] for more details.
pub struct HijriUmmAlQuraDateInner(ArithmeticDate<HijriUmmAlQura>);

impl CalendarArithmetic for HijriUmmAlQura {
    type YearInfo = HijriYearInfo;

    fn days_in_provided_month(year: Self::YearInfo, month: u8) -> u8 {
        year.days_in_month(month)
    }

    fn months_in_provided_year(_year: HijriYearInfo) -> u8 {
        12
    }

    fn days_in_provided_year(year: Self::YearInfo) -> u16 {
        year.days_in_year()
    }

    // As an true lunar calendar, it does not have leap years.
    fn provided_year_is_leap(year: Self::YearInfo) -> bool {
        year.days_in_year() != SHORT_YEAR_LEN
    }

    fn last_month_day_in_provided_year(year: HijriYearInfo) -> (u8, u8) {
        let days = Self::days_in_provided_month(year, 12);

        (12, days)
    }
}

impl Calendar for HijriUmmAlQura {
    type DateInner = HijriUmmAlQuraDateInner;
    fn from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            Some("islamic-umalqura" | "ah") | None => year,
            Some(_) => return Err(DateError::UnknownEra),
        };
        let Some((month, false)) = month_code.parsed() else {
            return Err(DateError::UnknownMonthCode(month_code));
        };
        Ok(HijriUmmAlQuraDateInner(ArithmeticDate::new_from_ordinals(
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
        HijriUmmAlQuraDateInner(ArithmeticDate::new_unchecked(y, m, d))
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
        date.0.offset_date(offset, &HijriUmmAlQura)
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
        Self::DEBUG_NAME
    }

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        year_as_hijri(tinystr!(16, "islamic-umalqura"), date.0.year.value)
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

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(crate::any_calendar::IntoAnyCalendar::kind(self))
    }
}

impl PrecomputedDataSource<HijriYearInfo> for HijriUmmAlQura {
    fn load_or_compute_info(&self, year: i32) -> HijriYearInfo {
        #[rustfmt::skip]
        const DATA: [PackedHijriYearInfo; 1601 - 1300] = {
            use calendrical_calculations::iso::fixed_from_iso;
            let l = true; // long
            let s = false; // short
            [
                PackedHijriYearInfo::new(1300, [l, s, l, s, l, s, l, s, l, s, l, s], fixed_from_iso(1882, 11, 12)),
                PackedHijriYearInfo::new(1301, [l, l, s, l, s, l, s, l, s, l, s, s], fixed_from_iso(1883, 11, 1)),
                PackedHijriYearInfo::new(1302, [l, l, l, s, l, l, s, s, l, s, s, l], fixed_from_iso(1884, 10, 20)),
                PackedHijriYearInfo::new(1303, [s, l, l, s, l, l, s, l, s, l, s, s], fixed_from_iso(1885, 10, 10)),
                PackedHijriYearInfo::new(1304, [s, l, l, s, l, l, l, s, l, s, l, s], fixed_from_iso(1886, 9, 29)),
                PackedHijriYearInfo::new(1305, [s, s, l, l, s, l, l, s, l, l, s, s], fixed_from_iso(1887, 9, 19)),
                PackedHijriYearInfo::new(1306, [l, s, l, s, l, s, l, s, l, l, s, l], fixed_from_iso(1888, 9, 7)),
                PackedHijriYearInfo::new(1307, [s, l, s, l, s, l, s, l, s, l, s, l], fixed_from_iso(1889, 8, 28)),
                PackedHijriYearInfo::new(1308, [s, l, l, s, l, s, l, s, l, s, s, l], fixed_from_iso(1890, 8, 17)),
                PackedHijriYearInfo::new(1309, [s, l, l, l, l, s, s, l, s, s, l, s], fixed_from_iso(1891, 8, 6)),
                PackedHijriYearInfo::new(1310, [l, s, l, l, l, s, l, s, l, s, s, l], fixed_from_iso(1892, 7, 25)),
                PackedHijriYearInfo::new(1311, [s, l, s, l, l, l, s, l, s, l, s, s], fixed_from_iso(1893, 7, 15)),
                PackedHijriYearInfo::new(1312, [l, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(1894, 7, 4)),
                PackedHijriYearInfo::new(1313, [s, l, s, l, s, l, s, l, l, l, s, s], fixed_from_iso(1895, 6, 24)),
                PackedHijriYearInfo::new(1314, [l, l, s, l, s, s, l, s, l, l, s, l], fixed_from_iso(1896, 6, 12)),
                PackedHijriYearInfo::new(1315, [s, l, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(1897, 6, 2)),
                PackedHijriYearInfo::new(1316, [s, l, l, l, s, l, s, s, l, s, l, s], fixed_from_iso(1898, 5, 22)),
                PackedHijriYearInfo::new(1317, [l, s, l, l, s, l, s, l, s, l, s, s], fixed_from_iso(1899, 5, 11)),
                PackedHijriYearInfo::new(1318, [l, s, l, l, s, l, l, s, l, s, l, s], fixed_from_iso(1900, 4, 30)),
                PackedHijriYearInfo::new(1319, [s, l, s, l, l, s, l, s, l, l, s, l], fixed_from_iso(1901, 4, 20)),
                PackedHijriYearInfo::new(1320, [s, l, s, s, l, s, l, s, l, l, l, s], fixed_from_iso(1902, 4, 10)),
                PackedHijriYearInfo::new(1321, [l, s, l, s, s, l, s, s, l, l, l, l], fixed_from_iso(1903, 3, 30)),
                PackedHijriYearInfo::new(1322, [s, l, s, l, s, s, s, l, s, l, l, l], fixed_from_iso(1904, 3, 19)),
                PackedHijriYearInfo::new(1323, [s, l, l, s, l, s, s, s, l, s, l, l], fixed_from_iso(1905, 3, 8)),
                PackedHijriYearInfo::new(1324, [s, l, l, s, l, s, l, s, s, l, s, l], fixed_from_iso(1906, 2, 25)),
                PackedHijriYearInfo::new(1325, [l, s, l, s, l, l, s, l, s, l, s, l], fixed_from_iso(1907, 2, 14)),
                PackedHijriYearInfo::new(1326, [s, s, l, s, l, l, s, l, s, l, l, s], fixed_from_iso(1908, 2, 4)),
                PackedHijriYearInfo::new(1327, [l, s, s, l, s, l, s, l, l, s, l, l], fixed_from_iso(1909, 1, 23)),
                PackedHijriYearInfo::new(1328, [s, l, s, s, l, s, s, l, l, l, s, l], fixed_from_iso(1910, 1, 13)),
                PackedHijriYearInfo::new(1329, [l, s, l, s, s, l, s, s, l, l, s, l], fixed_from_iso(1911, 1, 2)),
                PackedHijriYearInfo::new(1330, [l, l, s, l, s, s, l, s, s, l, l, s], fixed_from_iso(1911, 12, 22)),
                PackedHijriYearInfo::new(1331, [l, l, s, l, l, s, s, l, s, l, s, l], fixed_from_iso(1912, 12, 10)),
                PackedHijriYearInfo::new(1332, [s, l, s, l, l, s, l, s, l, l, s, s], fixed_from_iso(1913, 11, 30)),
                PackedHijriYearInfo::new(1333, [l, s, s, l, l, s, l, l, s, l, l, s], fixed_from_iso(1914, 11, 19)),
                PackedHijriYearInfo::new(1334, [s, s, l, s, l, s, l, l, l, s, l, s], fixed_from_iso(1915, 11, 9)),
                PackedHijriYearInfo::new(1335, [l, s, l, s, s, l, s, l, l, s, l, l], fixed_from_iso(1916, 10, 28)),
                PackedHijriYearInfo::new(1336, [s, l, s, l, s, s, l, s, l, s, l, l], fixed_from_iso(1917, 10, 18)),
                PackedHijriYearInfo::new(1337, [l, s, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(1918, 10, 7)),
                PackedHijriYearInfo::new(1338, [s, l, l, s, l, l, s, s, l, s, l, s], fixed_from_iso(1919, 9, 26)),
                PackedHijriYearInfo::new(1339, [l, s, l, s, l, l, l, s, l, s, s, l], fixed_from_iso(1920, 9, 14)),
                PackedHijriYearInfo::new(1340, [s, s, l, s, l, l, l, l, s, l, s, s], fixed_from_iso(1921, 9, 4)),
                PackedHijriYearInfo::new(1341, [l, s, s, l, s, l, l, l, s, l, l, s], fixed_from_iso(1922, 8, 24)),
                PackedHijriYearInfo::new(1342, [s, s, l, s, l, s, l, l, s, l, l, s], fixed_from_iso(1923, 8, 14)),
                PackedHijriYearInfo::new(1343, [l, s, s, l, s, l, s, l, s, l, l, s], fixed_from_iso(1924, 8, 2)),
                PackedHijriYearInfo::new(1344, [l, s, l, s, l, l, s, s, l, s, l, s], fixed_from_iso(1925, 7, 22)),
                PackedHijriYearInfo::new(1345, [l, s, l, l, l, s, l, s, s, l, s, s], fixed_from_iso(1926, 7, 11)),
                PackedHijriYearInfo::new(1346, [l, s, l, l, l, l, s, l, s, s, l, s], fixed_from_iso(1927, 6, 30)),
                PackedHijriYearInfo::new(1347, [s, l, s, l, l, l, s, l, l, s, s, l], fixed_from_iso(1928, 6, 19)),
                PackedHijriYearInfo::new(1348, [s, s, l, s, l, l, s, l, l, l, s, s], fixed_from_iso(1929, 6, 9)),
                PackedHijriYearInfo::new(1349, [l, s, s, l, s, l, l, s, l, l, s, l], fixed_from_iso(1930, 5, 29)),
                PackedHijriYearInfo::new(1350, [s, l, s, l, s, l, s, s, l, l, s, l], fixed_from_iso(1931, 5, 19)),
                PackedHijriYearInfo::new(1351, [l, s, l, s, l, s, l, s, s, l, s, l], fixed_from_iso(1932, 5, 7)),
                PackedHijriYearInfo::new(1352, [l, s, l, l, s, l, s, l, s, s, l, s], fixed_from_iso(1933, 4, 26)),
                PackedHijriYearInfo::new(1353, [l, s, l, l, l, s, l, s, s, l, s, l], fixed_from_iso(1934, 4, 15)),
                PackedHijriYearInfo::new(1354, [s, l, s, l, l, s, l, l, s, l, s, s], fixed_from_iso(1935, 4, 5)),
                PackedHijriYearInfo::new(1355, [l, s, s, l, l, s, l, l, s, l, l, s], fixed_from_iso(1936, 3, 24)),
                PackedHijriYearInfo::new(1356, [s, l, s, l, s, l, s, l, s, l, l, l], fixed_from_iso(1937, 3, 14)),
                PackedHijriYearInfo::new(1357, [s, s, l, s, l, s, s, l, s, l, l, l], fixed_from_iso(1938, 3, 4)),
                PackedHijriYearInfo::new(1358, [s, l, s, l, s, l, s, s, l, s, l, l], fixed_from_iso(1939, 2, 21)),
                PackedHijriYearInfo::new(1359, [s, l, l, s, l, s, l, s, s, s, l, l], fixed_from_iso(1940, 2, 10)),
                PackedHijriYearInfo::new(1360, [s, l, l, l, s, l, s, l, s, s, l, s], fixed_from_iso(1941, 1, 29)),
                PackedHijriYearInfo::new(1361, [l, s, l, l, s, l, l, s, s, l, s, l], fixed_from_iso(1942, 1, 18)),
                PackedHijriYearInfo::new(1362, [s, l, s, l, s, l, l, s, l, s, l, s], fixed_from_iso(1943, 1, 8)),
                PackedHijriYearInfo::new(1363, [l, s, l, s, l, s, l, s, l, s, l, l], fixed_from_iso(1943, 12, 28)),
                PackedHijriYearInfo::new(1364, [s, l, s, l, s, s, l, s, l, s, l, l], fixed_from_iso(1944, 12, 17)),
                PackedHijriYearInfo::new(1365, [l, l, s, s, l, s, s, l, s, l, s, l], fixed_from_iso(1945, 12, 6)),
                PackedHijriYearInfo::new(1366, [l, l, s, l, s, l, s, s, l, s, l, s], fixed_from_iso(1946, 11, 25)),
                PackedHijriYearInfo::new(1367, [l, l, s, l, l, s, l, s, s, l, s, l], fixed_from_iso(1947, 11, 14)),
                PackedHijriYearInfo::new(1368, [s, l, s, l, l, l, s, s, l, s, l, s], fixed_from_iso(1948, 11, 3)),
                PackedHijriYearInfo::new(1369, [l, s, l, s, l, l, s, l, s, l, l, s], fixed_from_iso(1949, 10, 23)),
                PackedHijriYearInfo::new(1370, [l, s, s, l, s, l, s, l, s, l, l, l], fixed_from_iso(1950, 10, 13)),
                PackedHijriYearInfo::new(1371, [s, l, s, s, l, s, l, s, l, s, l, l], fixed_from_iso(1951, 10, 3)),
                PackedHijriYearInfo::new(1372, [l, s, s, l, s, l, s, s, l, s, l, l], fixed_from_iso(1952, 9, 21)),
                PackedHijriYearInfo::new(1373, [l, s, l, s, l, s, l, s, s, l, s, l], fixed_from_iso(1953, 9, 10)),
                PackedHijriYearInfo::new(1374, [l, s, l, l, s, l, s, l, s, s, l, s], fixed_from_iso(1954, 8, 30)),
                PackedHijriYearInfo::new(1375, [l, s, l, l, s, l, l, s, l, s, l, s], fixed_from_iso(1955, 8, 19)),
                PackedHijriYearInfo::new(1376, [s, l, s, l, s, l, l, l, s, l, s, l], fixed_from_iso(1956, 8, 8)),
                PackedHijriYearInfo::new(1377, [s, s, l, s, s, l, l, l, s, l, l, s], fixed_from_iso(1957, 7, 29)),
                PackedHijriYearInfo::new(1378, [l, s, s, s, l, s, l, l, s, l, l, l], fixed_from_iso(1958, 7, 18)),
                PackedHijriYearInfo::new(1379, [s, l, s, s, s, l, s, l, l, s, l, l], fixed_from_iso(1959, 7, 8)),
                PackedHijriYearInfo::new(1380, [s, l, s, l, s, l, s, l, s, l, s, l], fixed_from_iso(1960, 6, 26)),
                PackedHijriYearInfo::new(1381, [s, l, s, l, l, s, l, s, l, s, s, l], fixed_from_iso(1961, 6, 15)),
                PackedHijriYearInfo::new(1382, [s, l, s, l, l, s, l, l, s, l, s, s], fixed_from_iso(1962, 6, 4)),
                PackedHijriYearInfo::new(1383, [l, s, s, l, l, l, s, l, l, s, l, s], fixed_from_iso(1963, 5, 24)),
                PackedHijriYearInfo::new(1384, [s, l, s, s, l, l, s, l, l, l, s, l], fixed_from_iso(1964, 5, 13)),
                PackedHijriYearInfo::new(1385, [s, s, l, s, s, l, l, s, l, l, l, s], fixed_from_iso(1965, 5, 3)),
                PackedHijriYearInfo::new(1386, [l, s, s, l, s, s, l, l, s, l, l, s], fixed_from_iso(1966, 4, 22)),
                PackedHijriYearInfo::new(1387, [l, s, l, s, l, s, l, s, l, s, l, s], fixed_from_iso(1967, 4, 11)),
                PackedHijriYearInfo::new(1388, [l, l, s, l, s, l, s, l, s, l, s, s], fixed_from_iso(1968, 3, 30)),
                PackedHijriYearInfo::new(1389, [l, l, s, l, l, s, l, l, s, s, l, s], fixed_from_iso(1969, 3, 19)),
                PackedHijriYearInfo::new(1390, [s, l, s, l, l, l, s, l, s, l, s, l], fixed_from_iso(1970, 3, 9)),
                PackedHijriYearInfo::new(1391, [s, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(1971, 2, 27)),
                PackedHijriYearInfo::new(1392, [l, s, s, l, s, l, s, l, l, s, l, l], fixed_from_iso(1972, 2, 16)),
                PackedHijriYearInfo::new(1393, [s, l, s, s, l, s, l, s, l, s, l, l], fixed_from_iso(1973, 2, 5)),
                PackedHijriYearInfo::new(1394, [l, s, l, s, s, l, s, l, s, l, s, l], fixed_from_iso(1974, 1, 25)),
                PackedHijriYearInfo::new(1395, [l, s, l, l, s, l, s, s, l, s, s, l], fixed_from_iso(1975, 1, 14)),
                PackedHijriYearInfo::new(1396, [l, s, l, l, s, l, l, s, s, l, s, s], fixed_from_iso(1976, 1, 3)),
                PackedHijriYearInfo::new(1397, [l, s, l, l, s, l, l, l, s, s, s, l], fixed_from_iso(1976, 12, 22)),
                PackedHijriYearInfo::new(1398, [s, l, s, l, l, s, l, l, s, l, s, s], fixed_from_iso(1977, 12, 12)),
                PackedHijriYearInfo::new(1399, [l, s, l, s, l, s, l, l, s, l, s, l], fixed_from_iso(1978, 12, 1)),
                PackedHijriYearInfo::new(1400, [l, s, l, s, s, l, s, l, s, l, s, l], fixed_from_iso(1979, 11, 21)),
                PackedHijriYearInfo::new(1401, [l, l, s, l, s, s, l, s, s, l, s, l], fixed_from_iso(1980, 11, 9)),
                PackedHijriYearInfo::new(1402, [l, l, l, s, l, s, s, l, s, s, l, s], fixed_from_iso(1981, 10, 29)),
                PackedHijriYearInfo::new(1403, [l, l, l, s, l, l, s, s, l, s, s, l], fixed_from_iso(1982, 10, 18)),
                PackedHijriYearInfo::new(1404, [s, l, l, s, l, l, s, l, s, l, s, s], fixed_from_iso(1983, 10, 8)),
                PackedHijriYearInfo::new(1405, [l, s, l, s, l, l, l, s, l, s, s, l], fixed_from_iso(1984, 9, 26)),
                PackedHijriYearInfo::new(1406, [l, s, s, l, s, l, l, s, l, s, l, l], fixed_from_iso(1985, 9, 16)),
                PackedHijriYearInfo::new(1407, [s, l, s, s, l, s, l, s, l, s, l, l], fixed_from_iso(1986, 9, 6)),
                PackedHijriYearInfo::new(1408, [l, s, l, s, l, s, s, l, s, s, l, l], fixed_from_iso(1987, 8, 26)),
                PackedHijriYearInfo::new(1409, [l, l, s, l, s, l, s, s, l, s, s, l], fixed_from_iso(1988, 8, 14)),
                PackedHijriYearInfo::new(1410, [l, l, s, l, l, s, l, s, s, l, s, s], fixed_from_iso(1989, 8, 3)),
                PackedHijriYearInfo::new(1411, [l, l, s, l, l, s, l, l, s, s, l, s], fixed_from_iso(1990, 7, 23)),
                PackedHijriYearInfo::new(1412, [l, s, l, s, l, s, l, l, l, s, s, l], fixed_from_iso(1991, 7, 13)),
                PackedHijriYearInfo::new(1413, [s, l, s, s, l, s, l, l, l, s, l, s], fixed_from_iso(1992, 7, 2)),
                PackedHijriYearInfo::new(1414, [l, s, l, s, s, l, s, l, l, s, l, l], fixed_from_iso(1993, 6, 21)),
                PackedHijriYearInfo::new(1415, [s, l, s, l, s, s, l, s, l, s, l, l], fixed_from_iso(1994, 6, 11)),
                PackedHijriYearInfo::new(1416, [l, s, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(1995, 5, 31)),
                PackedHijriYearInfo::new(1417, [l, s, l, l, s, s, l, s, l, s, l, s], fixed_from_iso(1996, 5, 19)),
                PackedHijriYearInfo::new(1418, [l, s, l, l, s, l, s, l, s, l, s, l], fixed_from_iso(1997, 5, 8)),
                PackedHijriYearInfo::new(1419, [s, l, s, l, s, l, s, l, l, l, s, s], fixed_from_iso(1998, 4, 28)),
                PackedHijriYearInfo::new(1420, [s, l, s, s, l, s, l, l, l, l, s, l], fixed_from_iso(1999, 4, 17)),
                PackedHijriYearInfo::new(1421, [s, s, l, s, s, s, l, l, l, l, s, l], fixed_from_iso(2000, 4, 6)),
                PackedHijriYearInfo::new(1422, [l, s, s, l, s, s, s, l, l, l, s, l], fixed_from_iso(2001, 3, 26)),
                PackedHijriYearInfo::new(1423, [l, s, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(2002, 3, 15)),
                PackedHijriYearInfo::new(1424, [l, s, l, l, s, l, s, s, l, s, l, s], fixed_from_iso(2003, 3, 4)),
                PackedHijriYearInfo::new(1425, [l, s, l, l, s, l, s, l, l, s, l, s], fixed_from_iso(2004, 2, 21)),
                PackedHijriYearInfo::new(1426, [s, l, s, l, s, l, l, s, l, l, s, l], fixed_from_iso(2005, 2, 10)),
                PackedHijriYearInfo::new(1427, [s, s, l, s, l, s, l, l, s, l, l, s], fixed_from_iso(2006, 1, 31)),
                PackedHijriYearInfo::new(1428, [l, s, s, l, s, s, l, l, l, s, l, l], fixed_from_iso(2007, 1, 20)),
                PackedHijriYearInfo::new(1429, [s, l, s, s, l, s, s, l, l, s, l, l], fixed_from_iso(2008, 1, 10)),
                PackedHijriYearInfo::new(1430, [s, l, l, s, s, l, s, l, s, l, s, l], fixed_from_iso(2008, 12, 29)),
                PackedHijriYearInfo::new(1431, [s, l, l, s, l, s, l, s, l, s, s, l], fixed_from_iso(2009, 12, 18)),
                PackedHijriYearInfo::new(1432, [s, l, l, l, s, l, s, l, s, l, s, s], fixed_from_iso(2010, 12, 7)),
                PackedHijriYearInfo::new(1433, [l, s, l, l, s, l, l, s, l, s, l, s], fixed_from_iso(2011, 11, 26)),
                PackedHijriYearInfo::new(1434, [s, l, s, l, s, l, l, s, l, l, s, s], fixed_from_iso(2012, 11, 15)),
                PackedHijriYearInfo::new(1435, [l, s, l, s, l, s, l, s, l, l, s, l], fixed_from_iso(2013, 11, 4)),
                PackedHijriYearInfo::new(1436, [s, l, s, l, s, l, s, l, s, l, s, l], fixed_from_iso(2014, 10, 25)),
                PackedHijriYearInfo::new(1437, [l, s, l, l, s, s, l, s, l, s, s, l], fixed_from_iso(2015, 10, 14)),
                PackedHijriYearInfo::new(1438, [l, s, l, l, l, s, s, l, s, s, l, s], fixed_from_iso(2016, 10, 2)),
                PackedHijriYearInfo::new(1439, [l, s, l, l, l, s, l, s, l, s, s, l], fixed_from_iso(2017, 9, 21)),
                PackedHijriYearInfo::new(1440, [s, l, s, l, l, l, s, l, s, l, s, s], fixed_from_iso(2018, 9, 11)),
                PackedHijriYearInfo::new(1441, [l, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(2019, 8, 31)),
                PackedHijriYearInfo::new(1442, [s, l, s, l, s, l, s, l, l, s, l, s], fixed_from_iso(2020, 8, 20)),
                PackedHijriYearInfo::new(1443, [l, s, l, s, l, s, l, s, l, s, l, l], fixed_from_iso(2021, 8, 9)),
                PackedHijriYearInfo::new(1444, [s, l, s, l, l, s, s, l, s, l, s, l], fixed_from_iso(2022, 7, 30)),
                PackedHijriYearInfo::new(1445, [s, l, l, l, s, l, s, s, l, s, s, l], fixed_from_iso(2023, 7, 19)),
                PackedHijriYearInfo::new(1446, [s, l, l, l, s, l, l, s, s, l, s, s], fixed_from_iso(2024, 7, 7)),
                PackedHijriYearInfo::new(1447, [l, s, l, l, l, s, l, s, l, s, l, s], fixed_from_iso(2025, 6, 26)),
                PackedHijriYearInfo::new(1448, [s, l, s, l, l, s, l, l, s, l, s, l], fixed_from_iso(2026, 6, 16)),
                PackedHijriYearInfo::new(1449, [s, s, l, s, l, s, l, l, s, l, l, s], fixed_from_iso(2027, 6, 6)),
                PackedHijriYearInfo::new(1450, [l, s, l, s, s, l, s, l, s, l, l, s], fixed_from_iso(2028, 5, 25)),
                PackedHijriYearInfo::new(1451, [l, l, l, s, s, l, s, s, l, l, s, l], fixed_from_iso(2029, 5, 14)),
                PackedHijriYearInfo::new(1452, [l, s, l, l, s, s, l, s, s, l, s, l], fixed_from_iso(2030, 5, 4)),
                PackedHijriYearInfo::new(1453, [l, s, l, l, s, l, s, l, s, s, l, s], fixed_from_iso(2031, 4, 23)),
                PackedHijriYearInfo::new(1454, [l, s, l, l, s, l, l, s, l, s, l, s], fixed_from_iso(2032, 4, 11)),
                PackedHijriYearInfo::new(1455, [s, l, s, l, l, s, l, s, l, l, s, l], fixed_from_iso(2033, 4, 1)),
                PackedHijriYearInfo::new(1456, [s, s, l, s, l, s, l, s, l, l, l, s], fixed_from_iso(2034, 3, 22)),
                PackedHijriYearInfo::new(1457, [l, s, s, l, s, s, l, s, l, l, l, l], fixed_from_iso(2035, 3, 11)),
                PackedHijriYearInfo::new(1458, [s, l, s, s, l, s, s, l, s, l, l, l], fixed_from_iso(2036, 2, 29)),
                PackedHijriYearInfo::new(1459, [s, l, l, s, s, l, s, s, l, s, l, l], fixed_from_iso(2037, 2, 17)),
                PackedHijriYearInfo::new(1460, [s, l, l, s, l, s, l, s, s, l, s, l], fixed_from_iso(2038, 2, 6)),
                PackedHijriYearInfo::new(1461, [s, l, l, s, l, s, l, s, l, l, s, s], fixed_from_iso(2039, 1, 26)),
                PackedHijriYearInfo::new(1462, [l, s, l, s, l, l, s, l, s, l, l, s], fixed_from_iso(2040, 1, 15)),
                PackedHijriYearInfo::new(1463, [s, l, s, l, s, l, s, l, l, l, s, l], fixed_from_iso(2041, 1, 4)),
                PackedHijriYearInfo::new(1464, [s, l, s, s, l, s, s, l, l, l, s, l], fixed_from_iso(2041, 12, 25)),
                PackedHijriYearInfo::new(1465, [l, s, l, s, s, l, s, s, l, l, s, l], fixed_from_iso(2042, 12, 14)),
                PackedHijriYearInfo::new(1466, [l, l, s, l, s, s, s, l, s, l, l, s], fixed_from_iso(2043, 12, 3)),
                PackedHijriYearInfo::new(1467, [l, l, s, l, l, s, s, l, s, l, s, l], fixed_from_iso(2044, 11, 21)),
                PackedHijriYearInfo::new(1468, [s, l, s, l, l, s, l, s, l, s, l, s], fixed_from_iso(2045, 11, 11)),
                PackedHijriYearInfo::new(1469, [s, l, s, l, l, s, l, l, s, l, s, l], fixed_from_iso(2046, 10, 31)),
                PackedHijriYearInfo::new(1470, [s, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(2047, 10, 21)),
                PackedHijriYearInfo::new(1471, [l, s, s, l, s, l, s, l, l, s, l, l], fixed_from_iso(2048, 10, 9)),
                PackedHijriYearInfo::new(1472, [s, l, s, s, l, s, l, s, l, l, s, l], fixed_from_iso(2049, 9, 29)),
                PackedHijriYearInfo::new(1473, [s, l, s, l, l, s, s, l, s, l, s, l], fixed_from_iso(2050, 9, 18)),
                PackedHijriYearInfo::new(1474, [s, l, l, s, l, l, s, s, l, s, l, s], fixed_from_iso(2051, 9, 7)),
                PackedHijriYearInfo::new(1475, [s, l, l, s, l, l, l, s, s, l, s, s], fixed_from_iso(2052, 8, 26)),
                PackedHijriYearInfo::new(1476, [l, s, l, s, l, l, l, s, l, s, l, s], fixed_from_iso(2053, 8, 15)),
                PackedHijriYearInfo::new(1477, [s, l, s, s, l, l, l, l, s, l, s, l], fixed_from_iso(2054, 8, 5)),
                PackedHijriYearInfo::new(1478, [s, s, l, s, l, s, l, l, s, l, l, s], fixed_from_iso(2055, 7, 26)),
                PackedHijriYearInfo::new(1479, [l, s, s, l, s, l, s, l, s, l, l, s], fixed_from_iso(2056, 7, 14)),
                PackedHijriYearInfo::new(1480, [l, s, l, s, l, s, l, s, l, s, l, s], fixed_from_iso(2057, 7, 3)),
                PackedHijriYearInfo::new(1481, [l, s, l, l, s, l, s, l, s, l, s, s], fixed_from_iso(2058, 6, 22)),
                PackedHijriYearInfo::new(1482, [l, s, l, l, l, l, s, l, s, s, l, s], fixed_from_iso(2059, 6, 11)),
                PackedHijriYearInfo::new(1483, [s, l, s, l, l, l, s, l, l, s, s, l], fixed_from_iso(2060, 5, 31)),
                PackedHijriYearInfo::new(1484, [s, s, l, s, l, l, l, s, l, s, l, s], fixed_from_iso(2061, 5, 21)),
                PackedHijriYearInfo::new(1485, [l, s, s, l, s, l, l, s, l, l, s, l], fixed_from_iso(2062, 5, 10)),
                PackedHijriYearInfo::new(1486, [s, l, s, s, l, s, l, s, l, l, s, l], fixed_from_iso(2063, 4, 30)),
                PackedHijriYearInfo::new(1487, [l, s, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(2064, 4, 18)),
                PackedHijriYearInfo::new(1488, [l, s, l, l, s, l, s, s, l, s, l, s], fixed_from_iso(2065, 4, 7)),
                PackedHijriYearInfo::new(1489, [l, s, l, l, l, s, l, s, s, l, s, l], fixed_from_iso(2066, 3, 27)),
                PackedHijriYearInfo::new(1490, [s, l, s, l, l, s, l, l, s, s, l, s], fixed_from_iso(2067, 3, 17)),
                PackedHijriYearInfo::new(1491, [l, s, s, l, l, s, l, l, s, l, s, l], fixed_from_iso(2068, 3, 5)),
                PackedHijriYearInfo::new(1492, [s, l, s, s, l, l, s, l, s, l, l, s], fixed_from_iso(2069, 2, 23)),
                PackedHijriYearInfo::new(1493, [l, s, l, s, l, s, s, l, s, l, l, l], fixed_from_iso(2070, 2, 12)),
                PackedHijriYearInfo::new(1494, [s, l, s, l, s, l, s, s, s, l, l, l], fixed_from_iso(2071, 2, 2)),
                PackedHijriYearInfo::new(1495, [s, l, l, s, l, s, s, l, s, s, l, l], fixed_from_iso(2072, 1, 22)),
                PackedHijriYearInfo::new(1496, [s, l, l, l, s, l, s, s, l, s, s, l], fixed_from_iso(2073, 1, 10)),
                PackedHijriYearInfo::new(1497, [l, s, l, l, s, l, s, l, s, l, s, l], fixed_from_iso(2073, 12, 30)),
                PackedHijriYearInfo::new(1498, [s, l, s, l, s, l, l, s, l, s, l, s], fixed_from_iso(2074, 12, 20)),
                PackedHijriYearInfo::new(1499, [l, s, l, s, s, l, l, s, l, s, l, l], fixed_from_iso(2075, 12, 9)),
                PackedHijriYearInfo::new(1500, [s, l, s, l, s, s, l, s, l, s, l, l], fixed_from_iso(2076, 11, 28)),
                PackedHijriYearInfo::new(1501, [l, s, l, s, l, s, s, s, l, s, l, l], fixed_from_iso(2077, 11, 17)),
                PackedHijriYearInfo::new(1502, [l, l, s, l, s, l, s, s, s, l, l, s], fixed_from_iso(2078, 11, 6)),
                PackedHijriYearInfo::new(1503, [l, l, s, l, l, s, l, s, s, s, l, l], fixed_from_iso(2079, 10, 26)),
                PackedHijriYearInfo::new(1504, [s, l, s, l, l, l, s, s, l, s, l, s], fixed_from_iso(2080, 10, 15)),
                PackedHijriYearInfo::new(1505, [l, s, l, s, l, l, s, l, s, l, l, s], fixed_from_iso(2081, 10, 4)),
                PackedHijriYearInfo::new(1506, [s, l, s, s, l, l, s, l, l, s, l, l], fixed_from_iso(2082, 9, 24)),
                PackedHijriYearInfo::new(1507, [s, s, l, s, s, l, l, s, l, s, l, l], fixed_from_iso(2083, 9, 14)),
                PackedHijriYearInfo::new(1508, [l, s, s, l, s, l, s, s, l, s, l, l], fixed_from_iso(2084, 9, 2)),
                PackedHijriYearInfo::new(1509, [l, s, l, s, l, s, l, s, s, l, s, l], fixed_from_iso(2085, 8, 22)),
                PackedHijriYearInfo::new(1510, [l, s, l, l, s, l, s, l, s, s, l, s], fixed_from_iso(2086, 8, 11)),
                PackedHijriYearInfo::new(1511, [l, s, l, l, s, l, l, s, l, s, s, l], fixed_from_iso(2087, 7, 31)),
                PackedHijriYearInfo::new(1512, [s, l, s, l, s, l, l, l, s, l, s, l], fixed_from_iso(2088, 7, 20)),
                PackedHijriYearInfo::new(1513, [s, s, s, l, s, l, l, l, s, l, l, s], fixed_from_iso(2089, 7, 10)),
                PackedHijriYearInfo::new(1514, [l, s, s, s, l, s, l, l, s, l, l, l], fixed_from_iso(2090, 6, 29)),
                PackedHijriYearInfo::new(1515, [s, s, l, s, s, l, s, l, l, s, l, l], fixed_from_iso(2091, 6, 19)),
                PackedHijriYearInfo::new(1516, [s, l, s, l, s, s, l, s, l, s, l, l], fixed_from_iso(2092, 6, 7)),
                PackedHijriYearInfo::new(1517, [s, l, s, l, s, l, l, s, s, l, s, l], fixed_from_iso(2093, 5, 27)),
                PackedHijriYearInfo::new(1518, [s, l, s, l, l, s, l, l, s, l, s, s], fixed_from_iso(2094, 5, 16)),
                PackedHijriYearInfo::new(1519, [l, s, s, l, l, l, s, l, l, s, l, s], fixed_from_iso(2095, 5, 5)),
                PackedHijriYearInfo::new(1520, [s, l, s, s, l, l, l, s, l, l, s, l], fixed_from_iso(2096, 4, 24)),
                PackedHijriYearInfo::new(1521, [s, s, s, l, s, l, l, s, l, l, s, l], fixed_from_iso(2097, 4, 14)),
                PackedHijriYearInfo::new(1522, [l, s, s, s, l, s, l, l, s, l, l, s], fixed_from_iso(2098, 4, 3)),
                PackedHijriYearInfo::new(1523, [l, s, l, s, l, s, l, s, s, l, l, s], fixed_from_iso(2099, 3, 23)),
                PackedHijriYearInfo::new(1524, [l, l, s, l, s, l, s, l, s, s, l, s], fixed_from_iso(2100, 3, 12)),
                PackedHijriYearInfo::new(1525, [l, l, s, l, l, s, l, s, l, s, s, l], fixed_from_iso(2101, 3, 1)),
                PackedHijriYearInfo::new(1526, [s, l, s, l, l, l, s, l, s, l, s, s], fixed_from_iso(2102, 2, 19)),
                PackedHijriYearInfo::new(1527, [l, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(2103, 2, 8)),
                PackedHijriYearInfo::new(1528, [l, s, s, l, s, l, s, l, l, s, l, l], fixed_from_iso(2104, 1, 29)),
                PackedHijriYearInfo::new(1529, [s, l, s, s, l, s, l, s, l, s, l, l], fixed_from_iso(2105, 1, 18)),
                PackedHijriYearInfo::new(1530, [s, l, l, s, s, l, s, l, s, s, l, l], fixed_from_iso(2106, 1, 7)),
                PackedHijriYearInfo::new(1531, [s, l, l, l, s, s, l, s, l, s, s, l], fixed_from_iso(2106, 12, 27)),
                PackedHijriYearInfo::new(1532, [s, l, l, l, s, l, l, s, s, s, l, s], fixed_from_iso(2107, 12, 16)),
                PackedHijriYearInfo::new(1533, [l, s, l, l, l, s, l, s, l, s, s, l], fixed_from_iso(2108, 12, 4)),
                PackedHijriYearInfo::new(1534, [s, l, s, l, l, s, l, l, s, s, l, s], fixed_from_iso(2109, 11, 24)),
                PackedHijriYearInfo::new(1535, [l, s, l, s, l, s, l, l, s, l, s, l], fixed_from_iso(2110, 11, 13)),
                PackedHijriYearInfo::new(1536, [s, l, s, l, s, l, s, l, s, l, s, l], fixed_from_iso(2111, 11, 3)),
                PackedHijriYearInfo::new(1537, [l, s, l, l, s, s, l, s, s, l, s, l], fixed_from_iso(2112, 10, 22)),
                PackedHijriYearInfo::new(1538, [l, l, s, l, l, s, s, l, s, s, l, s], fixed_from_iso(2113, 10, 11)),
                PackedHijriYearInfo::new(1539, [l, l, l, s, l, l, s, s, l, s, s, l], fixed_from_iso(2114, 9, 30)),
                PackedHijriYearInfo::new(1540, [s, l, l, s, l, l, s, l, s, s, l, s], fixed_from_iso(2115, 9, 20)),
                PackedHijriYearInfo::new(1541, [l, s, l, s, l, l, l, s, l, s, s, l], fixed_from_iso(2116, 9, 8)),
                PackedHijriYearInfo::new(1542, [s, l, s, l, s, l, l, s, l, s, l, l], fixed_from_iso(2117, 8, 29)),
                PackedHijriYearInfo::new(1543, [s, l, s, s, l, s, l, s, l, s, l, l], fixed_from_iso(2118, 8, 19)),
                PackedHijriYearInfo::new(1544, [l, s, l, s, s, l, s, l, s, l, s, l], fixed_from_iso(2119, 8, 8)),
                PackedHijriYearInfo::new(1545, [l, l, s, l, s, s, l, s, l, s, s, l], fixed_from_iso(2120, 7, 27)),
                PackedHijriYearInfo::new(1546, [l, l, s, l, s, l, s, l, s, l, s, s], fixed_from_iso(2121, 7, 16)),
                PackedHijriYearInfo::new(1547, [l, l, s, l, l, s, l, s, l, s, l, s], fixed_from_iso(2122, 7, 5)),
                PackedHijriYearInfo::new(1548, [l, s, s, l, l, s, l, l, s, l, s, l], fixed_from_iso(2123, 6, 25)),
                PackedHijriYearInfo::new(1549, [s, l, s, s, l, s, l, l, l, s, l, s], fixed_from_iso(2124, 6, 14)),
                PackedHijriYearInfo::new(1550, [l, s, l, s, s, s, l, l, l, s, l, l], fixed_from_iso(2125, 6, 3)),
                PackedHijriYearInfo::new(1551, [s, l, s, s, l, s, s, l, l, s, l, l], fixed_from_iso(2126, 5, 24)),
                PackedHijriYearInfo::new(1552, [l, s, l, s, s, l, s, s, l, l, s, l], fixed_from_iso(2127, 5, 13)),
                PackedHijriYearInfo::new(1553, [l, s, l, s, l, s, l, s, l, s, l, s], fixed_from_iso(2128, 5, 1)),
                PackedHijriYearInfo::new(1554, [l, s, l, s, l, l, s, l, s, l, s, l], fixed_from_iso(2129, 4, 20)),
                PackedHijriYearInfo::new(1555, [s, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(2130, 4, 10)),
                PackedHijriYearInfo::new(1556, [l, s, s, l, s, l, s, l, l, l, s, l], fixed_from_iso(2131, 3, 30)),
                PackedHijriYearInfo::new(1557, [s, l, s, s, s, l, s, l, l, l, l, s], fixed_from_iso(2132, 3, 19)),
                PackedHijriYearInfo::new(1558, [l, s, l, s, s, s, l, s, l, l, l, s], fixed_from_iso(2133, 3, 8)),
                PackedHijriYearInfo::new(1559, [l, l, s, s, l, s, s, l, l, s, l, s], fixed_from_iso(2134, 2, 25)),
                PackedHijriYearInfo::new(1560, [l, l, s, l, s, l, s, l, s, l, s, l], fixed_from_iso(2135, 2, 14)),
                PackedHijriYearInfo::new(1561, [s, l, l, s, l, s, l, l, s, s, l, s], fixed_from_iso(2136, 2, 4)),
                PackedHijriYearInfo::new(1562, [s, l, l, s, l, s, l, l, l, s, s, l], fixed_from_iso(2137, 1, 23)),
                PackedHijriYearInfo::new(1563, [s, l, s, s, l, s, l, l, l, s, l, s], fixed_from_iso(2138, 1, 13)),
                PackedHijriYearInfo::new(1564, [l, s, l, s, s, l, s, l, l, l, s, l], fixed_from_iso(2139, 1, 2)),
                PackedHijriYearInfo::new(1565, [s, l, s, l, s, s, l, s, l, l, s, l], fixed_from_iso(2139, 12, 23)),
                PackedHijriYearInfo::new(1566, [l, s, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(2140, 12, 11)),
                PackedHijriYearInfo::new(1567, [l, s, l, l, s, l, s, l, s, s, l, s], fixed_from_iso(2141, 11, 30)),
                PackedHijriYearInfo::new(1568, [l, s, l, l, l, s, l, s, l, s, s, s], fixed_from_iso(2142, 11, 19)),
                PackedHijriYearInfo::new(1569, [l, s, l, l, l, s, l, l, s, l, s, s], fixed_from_iso(2143, 11, 8)),
                PackedHijriYearInfo::new(1570, [s, l, s, l, l, s, l, l, l, s, s, l], fixed_from_iso(2144, 10, 28)),
                PackedHijriYearInfo::new(1571, [s, s, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(2145, 10, 18)),
                PackedHijriYearInfo::new(1572, [l, s, s, l, s, l, s, l, l, s, l, s], fixed_from_iso(2146, 10, 7)),
                PackedHijriYearInfo::new(1573, [l, s, l, l, s, l, s, s, l, s, l, s], fixed_from_iso(2147, 9, 26)),
                PackedHijriYearInfo::new(1574, [l, l, s, l, l, s, l, s, s, l, s, s], fixed_from_iso(2148, 9, 14)),
                PackedHijriYearInfo::new(1575, [l, l, l, s, l, l, s, l, s, s, s, l], fixed_from_iso(2149, 9, 3)),
                PackedHijriYearInfo::new(1576, [s, l, l, s, l, l, l, s, l, s, s, s], fixed_from_iso(2150, 8, 24)),
                PackedHijriYearInfo::new(1577, [l, s, l, l, s, l, l, s, l, s, l, s], fixed_from_iso(2151, 8, 13)),
                PackedHijriYearInfo::new(1578, [s, l, s, l, s, l, l, s, l, l, s, l], fixed_from_iso(2152, 8, 2)),
                PackedHijriYearInfo::new(1579, [s, l, s, l, s, s, l, l, s, l, s, l], fixed_from_iso(2153, 7, 23)),
                PackedHijriYearInfo::new(1580, [s, l, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(2154, 7, 12)),
                PackedHijriYearInfo::new(1581, [l, l, s, l, s, l, s, s, l, s, l, s], fixed_from_iso(2155, 7, 1)),
                PackedHijriYearInfo::new(1582, [l, l, s, l, l, s, l, s, l, s, s, s], fixed_from_iso(2156, 6, 19)),
                PackedHijriYearInfo::new(1583, [l, l, s, l, l, l, s, l, s, l, s, s], fixed_from_iso(2157, 6, 8)),
                PackedHijriYearInfo::new(1584, [s, l, l, s, l, l, s, l, l, s, l, s], fixed_from_iso(2158, 5, 29)),
                PackedHijriYearInfo::new(1585, [s, l, s, l, s, l, s, l, l, s, l, l], fixed_from_iso(2159, 5, 19)),
                PackedHijriYearInfo::new(1586, [s, s, l, s, l, s, s, l, l, l, s, l], fixed_from_iso(2160, 5, 8)),
                PackedHijriYearInfo::new(1587, [s, l, l, s, s, s, l, s, l, s, l, l], fixed_from_iso(2161, 4, 27)),
                PackedHijriYearInfo::new(1588, [l, s, l, l, s, s, s, l, s, l, s, l], fixed_from_iso(2162, 4, 16)),
                PackedHijriYearInfo::new(1589, [l, s, l, l, s, l, s, s, l, s, l, s], fixed_from_iso(2163, 4, 5)),
                PackedHijriYearInfo::new(1590, [l, s, l, l, l, s, s, l, s, l, s, l], fixed_from_iso(2164, 3, 24)),
                PackedHijriYearInfo::new(1591, [s, l, s, l, l, s, l, s, l, s, l, s], fixed_from_iso(2165, 3, 14)),
                PackedHijriYearInfo::new(1592, [l, s, l, s, l, s, l, s, l, l, l, s], fixed_from_iso(2166, 3, 3)),
                PackedHijriYearInfo::new(1593, [l, s, s, l, s, s, l, s, l, l, l, s], fixed_from_iso(2167, 2, 21)),
                PackedHijriYearInfo::new(1594, [l, l, s, s, l, s, s, s, l, l, l, l], fixed_from_iso(2168, 2, 10)),
                PackedHijriYearInfo::new(1595, [s, l, s, l, s, s, l, s, s, l, l, l], fixed_from_iso(2169, 1, 30)),
                PackedHijriYearInfo::new(1596, [s, l, l, s, l, s, s, l, s, l, s, l], fixed_from_iso(2170, 1, 19)),
                PackedHijriYearInfo::new(1597, [s, l, l, s, l, s, l, s, l, s, l, s], fixed_from_iso(2171, 1, 8)),
                PackedHijriYearInfo::new(1598, [l, s, l, s, l, l, s, l, s, l, l, s], fixed_from_iso(2171, 12, 28)),
                PackedHijriYearInfo::new(1599, [s, l, s, l, s, l, s, l, l, l, s, l], fixed_from_iso(2172, 12, 17)),
                PackedHijriYearInfo::new(1600, [s, s, l, s, l, s, s, l, l, l, s, l], fixed_from_iso(2173, 12, 7)),
            ]
        };
        if let Some(&packed) = usize::try_from(year - 1300).ok().and_then(|i| DATA.get(i)) {
            HijriYearInfo::unpack(year, packed)
        } else {
            HijriYearInfo {
                value: year,
                month_lengths: core::array::from_fn(|i| {
                    HijriTabular::days_in_provided_month(year, i as u8 + 1) == 30
                }),
                start_day: calendrical_calculations::islamic::fixed_from_tabular_islamic(
                    year,
                    1,
                    1,
                    ISLAMIC_EPOCH_FRIDAY,
                ),
            }
        }
    }
}

impl HijriUmmAlQura {
    pub(crate) const DEBUG_NAME: &'static str = "Hijri (Umm al-Qura)";
}

impl Date<HijriUmmAlQura> {
    /// Construct new Hijri Umm al-Qura Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::cal::HijriUmmAlQura;
    /// use icu::calendar::Date;
    ///
    /// let date_hijri =
    ///     Date::try_new_ummalqura(1392, 4, 25)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.year().era_year_or_extended(), 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_ummalqura(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<HijriUmmAlQura>, RangeError> {
        let y = HijriUmmAlQura.load_or_compute_info(year);
        Ok(Date::from_raw(
            HijriUmmAlQuraDateInner(ArithmeticDate::new_from_ordinals(y, month, day)?),
            HijriUmmAlQura,
        ))
    }
}

/// The inner date type used for representing [`Date`]s of [`HijriTabular`]. See [`Date`] and [`HijriTabular`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriTabularDateInner(ArithmeticDate<HijriTabular>);

impl CalendarArithmetic for HijriTabular {
    type YearInfo = i32;

    fn days_in_provided_month(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 if Self::provided_year_is_leap(year) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_in_provided_year(_year: Self::YearInfo) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u16 {
        if Self::provided_year_is_leap(year) {
            LONG_YEAR_LEN
        } else {
            SHORT_YEAR_LEN
        }
    }

    fn provided_year_is_leap(year: i32) -> bool {
        (14 + 11 * year).rem_euclid(30) < 11
    }

    fn last_month_day_in_provided_year(year: i32) -> (u8, u8) {
        if Self::provided_year_is_leap(year) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for HijriTabular {
    type DateInner = HijriTabularDateInner;

    fn from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            Some("ah") | None => year,
            Some("islamic-civil" | "islamicc") if self.0 == ISLAMIC_EPOCH_FRIDAY => year,
            Some("islamic-tbla") if self.0 == ISLAMIC_EPOCH_THURSDAY => year,
            Some(_) => return Err(DateError::UnknownEra),
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(HijriTabularDateInner)
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let (y, m, d) = calendrical_calculations::islamic::tabular_islamic_from_fixed(rd, self.0);

        debug_assert!(Date::try_new_hijri_tabular_with_calendar(y, m, d, crate::Ref(self)).is_ok());
        HijriTabularDateInner(ArithmeticDate::new_unchecked(y, m, d))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        calendrical_calculations::islamic::fixed_from_tabular_islamic(
            date.0.year,
            date.0.month,
            date.0.day,
            self.0,
        )
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
        date.0.offset_date(offset, &())
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
        match self.0 {
            ISLAMIC_EPOCH_FRIDAY => "Hijri (civil)",
            _ => "Hijri (astronomical)",
        }
    }

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        year_as_hijri(
            if self.0 == ISLAMIC_EPOCH_FRIDAY {
                tinystr!(16, "islamic-civil")
            } else {
                tinystr!(16, "islamic-tbla")
            },
            date.0.year,
        )
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

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(crate::any_calendar::IntoAnyCalendar::kind(self))
    }
}

impl<A: AsCalendar<Calendar = HijriTabular>> Date<A> {
    /// Construct new Tabular Hijri Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::cal::HijriTabular;
    /// use icu::calendar::Date;
    ///
    /// let hijri = HijriTabular::new_civil_epoch();
    ///
    /// let date_hijri =
    ///     Date::try_new_hijri_tabular_with_calendar(1392, 4, 25, hijri)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.year().era_year_or_extended(), 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_hijri_tabular_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        ArithmeticDate::new_from_ordinals(year, month, day)
            .map(HijriTabularDateInner)
            .map(|inner| Date::from_raw(inner, calendar))
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
        let calendar = HijriSimulated::new_mecca();
        let calendar = Ref(&calendar);
        for (case, f_date) in SIMULATED_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_simulated_hijri_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            let iso = Date::from_rata_die(RataDie::new(*f_date), Iso);

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_simulated_hijri() {
        let calendar = HijriSimulated::new_mecca();
        let calendar = Ref(&calendar);
        for (case, f_date) in SIMULATED_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_simulated_hijri_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_hijri() {
        let calendar = HijriTabular::new_civil_epoch();
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_hijri_from_rd() {
        let calendar = HijriTabular::new_civil_epoch();
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_hijri_tbla() {
        let calendar = HijriTabular::new_astronomical_epoch();
        let calendar = Ref(&calendar);
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_hijri_tbla_from_rd() {
        let calendar = HijriTabular::new_astronomical_epoch();
        let calendar = Ref(&calendar);
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_saudi_hijri_from_rd() {
        let calendar = HijriUmmAlQura::new();
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
        let calendar = HijriSimulated::new_mecca();
        let calendar = Ref(&calendar);
        // -1245 1 1 = -214526 (R.D Date)
        // 1518 1 1 = 764589 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| {
                HijriSimulated::days_in_provided_year(
                    HijriSimulatedLocation::Mecca.compute_year_info(year),
                ) as i64
            })
            .sum();
        let expected_number_of_days =
            Date::try_new_simulated_hijri_with_calendar(END_YEAR, 1, 1, calendar)
                .unwrap()
                .to_rata_die()
                - Date::try_new_simulated_hijri_with_calendar(START_YEAR, 1, 1, calendar)
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
                HijriUmmAlQura::days_in_provided_year(HijriUmmAlQura.load_or_compute_info(year))
                    as i64
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
        let hijri = iso.to_calendar(HijriUmmAlQura::new());
        // Data from https://www.ummulqura.org.sa/Index.aspx
        assert_eq!(hijri.day_of_month().0, 30);
        assert_eq!(hijri.month().ordinal, 4);
        assert_eq!(hijri.year().era_year_or_extended(), 1432);
    }

    #[test]
    fn test_regression_4914() {
        // https://github.com/unicode-org/icu4x/issues/4914
        let cal = HijriUmmAlQura::new();
        let era = "islamic-umalqura";
        let year = -6823;
        let month_code = MonthCode(tinystr!(4, "M01"));
        let dt = cal.from_codes(Some(era), year, month_code, 1).unwrap();
        assert_eq!(dt.0.day, 1);
        assert_eq!(dt.0.month, 1);
        assert_eq!(dt.0.year.value, -6823);
    }

    #[test]
    fn test_regression_5069_uaq() {
        let cached = HijriUmmAlQura::new();

        let cached = crate::Ref(&cached);

        let dt_cached = Date::try_new_ummalqura(1391, 1, 29).unwrap();

        assert_eq!(dt_cached.to_iso().to_calendar(cached), dt_cached);
    }

    #[test]
    fn test_regression_5069_obs() {
        let cached = HijriSimulated::new_mecca();
        let comp = HijriSimulated::new_mecca_always_calculating();

        let cached = crate::Ref(&cached);
        let comp = crate::Ref(&comp);

        let dt_cached = Date::try_new_simulated_hijri_with_calendar(1390, 1, 30, cached).unwrap();
        let dt_comp = Date::try_new_simulated_hijri_with_calendar(1390, 1, 30, comp).unwrap();

        assert_eq!(dt_cached.to_iso(), dt_comp.to_iso());

        assert_eq!(dt_comp.to_iso().to_calendar(comp), dt_comp);
        assert_eq!(dt_cached.to_iso().to_calendar(cached), dt_cached);

        let dt = Date::try_new_iso(2000, 5, 5).unwrap();

        assert!(dt.to_calendar(comp).day_of_month().0 > 0);
        assert!(dt.to_calendar(cached).day_of_month().0 > 0);
    }

    #[test]
    fn test_regression_6197() {
        let cached = HijriUmmAlQura::new();

        let cached = crate::Ref(&cached);

        let iso = Date::try_new_iso(2025, 2, 26).unwrap();

        let cached = iso.to_calendar(cached);

        // Data from https://www.ummulqura.org.sa/
        assert_eq!(
            (
                cached.day_of_month().0,
                cached.month().ordinal,
                cached.year().era_year_or_extended()
            ),
            (27, 8, 1446)
        );
    }

    #[test]
    fn test_uaq_icu4c_agreement() {
        // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L87
        const ICU4C_ENCODED_MONTH_LENGTHS: [u16; 1601 - 1300] = [
            0x0AAA, 0x0D54, 0x0EC9, 0x06D4, 0x06EA, 0x036C, 0x0AAD, 0x0555, 0x06A9, 0x0792, 0x0BA9,
            0x05D4, 0x0ADA, 0x055C, 0x0D2D, 0x0695, 0x074A, 0x0B54, 0x0B6A, 0x05AD, 0x04AE, 0x0A4F,
            0x0517, 0x068B, 0x06A5, 0x0AD5, 0x02D6, 0x095B, 0x049D, 0x0A4D, 0x0D26, 0x0D95, 0x05AC,
            0x09B6, 0x02BA, 0x0A5B, 0x052B, 0x0A95, 0x06CA, 0x0AE9, 0x02F4, 0x0976, 0x02B6, 0x0956,
            0x0ACA, 0x0BA4, 0x0BD2, 0x05D9, 0x02DC, 0x096D, 0x054D, 0x0AA5, 0x0B52, 0x0BA5, 0x05B4,
            0x09B6, 0x0557, 0x0297, 0x054B, 0x06A3, 0x0752, 0x0B65, 0x056A, 0x0AAB, 0x052B, 0x0C95,
            0x0D4A, 0x0DA5, 0x05CA, 0x0AD6, 0x0957, 0x04AB, 0x094B, 0x0AA5, 0x0B52, 0x0B6A, 0x0575,
            0x0276, 0x08B7, 0x045B, 0x0555, 0x05A9, 0x05B4, 0x09DA, 0x04DD, 0x026E, 0x0936, 0x0AAA,
            0x0D54, 0x0DB2, 0x05D5, 0x02DA, 0x095B, 0x04AB, 0x0A55, 0x0B49, 0x0B64, 0x0B71, 0x05B4,
            0x0AB5, 0x0A55, 0x0D25, 0x0E92, 0x0EC9, 0x06D4, 0x0AE9, 0x096B, 0x04AB, 0x0A93, 0x0D49,
            0x0DA4, 0x0DB2, 0x0AB9, 0x04BA, 0x0A5B, 0x052B, 0x0A95, 0x0B2A, 0x0B55, 0x055C, 0x04BD,
            0x023D, 0x091D, 0x0A95, 0x0B4A, 0x0B5A, 0x056D, 0x02B6, 0x093B, 0x049B, 0x0655, 0x06A9,
            0x0754, 0x0B6A, 0x056C, 0x0AAD, 0x0555, 0x0B29, 0x0B92, 0x0BA9, 0x05D4, 0x0ADA, 0x055A,
            0x0AAB, 0x0595, 0x0749, 0x0764, 0x0BAA, 0x05B5, 0x02B6, 0x0A56, 0x0E4D, 0x0B25, 0x0B52,
            0x0B6A, 0x05AD, 0x02AE, 0x092F, 0x0497, 0x064B, 0x06A5, 0x06AC, 0x0AD6, 0x055D, 0x049D,
            0x0A4D, 0x0D16, 0x0D95, 0x05AA, 0x05B5, 0x02DA, 0x095B, 0x04AD, 0x0595, 0x06CA, 0x06E4,
            0x0AEA, 0x04F5, 0x02B6, 0x0956, 0x0AAA, 0x0B54, 0x0BD2, 0x05D9, 0x02EA, 0x096D, 0x04AD,
            0x0A95, 0x0B4A, 0x0BA5, 0x05B2, 0x09B5, 0x04D6, 0x0A97, 0x0547, 0x0693, 0x0749, 0x0B55,
            0x056A, 0x0A6B, 0x052B, 0x0A8B, 0x0D46, 0x0DA3, 0x05CA, 0x0AD6, 0x04DB, 0x026B, 0x094B,
            0x0AA5, 0x0B52, 0x0B69, 0x0575, 0x0176, 0x08B7, 0x025B, 0x052B, 0x0565, 0x05B4, 0x09DA,
            0x04ED, 0x016D, 0x08B6, 0x0AA6, 0x0D52, 0x0DA9, 0x05D4, 0x0ADA, 0x095B, 0x04AB, 0x0653,
            0x0729, 0x0762, 0x0BA9, 0x05B2, 0x0AB5, 0x0555, 0x0B25, 0x0D92, 0x0EC9, 0x06D2, 0x0AE9,
            0x056B, 0x04AB, 0x0A55, 0x0D29, 0x0D54, 0x0DAA, 0x09B5, 0x04BA, 0x0A3B, 0x049B, 0x0A4D,
            0x0AAA, 0x0AD5, 0x02DA, 0x095D, 0x045E, 0x0A2E, 0x0C9A, 0x0D55, 0x06B2, 0x06B9, 0x04BA,
            0x0A5D, 0x052D, 0x0A95, 0x0B52, 0x0BA8, 0x0BB4, 0x05B9, 0x02DA, 0x095A, 0x0B4A, 0x0DA4,
            0x0ED1, 0x06E8, 0x0B6A, 0x056D, 0x0535, 0x0695, 0x0D4A, 0x0DA8, 0x0DD4, 0x06DA, 0x055B,
            0x029D, 0x062B, 0x0B15, 0x0B4A, 0x0B95, 0x05AA, 0x0AAE, 0x092E, 0x0C8F, 0x0527, 0x0695,
            0x06AA, 0x0AD6, 0x055D, 0x029D,
        ];

        // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L264
        const ICU4C_YEAR_START_ESTIMATE_FIX: [i64; 1601 - 1300] = [
            0, 0, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, 0, 0,
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, -1,
            0, 1, 0, 0, 0, -1, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1, -1, 0, -1, 0, 1, 0, 0, 0,
            -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, 0, 0,
            -1, -1, 0, -1, 0, -1, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, -1, 0,
            1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, 0,
            0, 1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, -1, 0, 1, 0, 0, 0,
            -1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1,
            -1, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
        ];

        let icu4c = ICU4C_ENCODED_MONTH_LENGTHS
            .into_iter()
            .zip(ICU4C_YEAR_START_ESTIMATE_FIX)
            .enumerate()
            .map(
                |(years_since_1300, (encoded_months_lengths, year_start_estimate_fix))| {
                    // https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L858
                    let month_lengths =
                        core::array::from_fn(|i| (1 << (11 - i)) & encoded_months_lengths != 0);
                    // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L813
                    let year_start = ((354.36720 * years_since_1300 as f64) + 460322.05 + 0.5)
                        as i64
                        + year_start_estimate_fix;
                    HijriYearInfo {
                        value: 1300 + years_since_1300 as i32,
                        month_lengths,
                        start_day: ISLAMIC_EPOCH_FRIDAY + year_start,
                    }
                },
            )
            .collect::<Vec<_>>();

        let icu4x = (1300..=1600)
            .map(|y| HijriUmmAlQura.load_or_compute_info(y))
            .collect::<Vec<_>>();

        assert_eq!(icu4x, icu4c);
    }
}
