// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Hijri calendars.
//!
//! ```rust
//! use icu::calendar::cal::HijriObservational;
//! use icu::calendar::Date;
//!
//! let hijri = HijriObservational::new_mecca_always_calculating();
//! let hijri_date = Date::try_new_observational_hijri_with_calendar(
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
#[cfg(feature = "datagen")]
use crate::provider::hijri::PackedHijriYearInfo;
use crate::provider::hijri::{
    CalendarHijriObservationalMeccaV1, CalendarHijriUmmalquraV1, HijriCache,
};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use crate::{AsCalendar, RangeError};
use calendrical_calculations::islamic::ISLAMIC_EPOCH_FRIDAY;
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

/// The [observational Hijri Calendar](https://en.wikipedia.org/wiki/Islamic_calendar)
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
pub struct HijriObservational {
    pub(crate) location: HijriObservationalLocation,
    data: Option<DataPayload<ErasedMarker<HijriCache<'static>>>>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub(crate) enum HijriObservationalLocation {
    Mecca,
}

impl HijriObservationalLocation {
    fn location(self) -> calendrical_calculations::islamic::Location {
        match self {
            Self::Mecca => calendrical_calculations::islamic::MECCA,
        }
    }
}

/// The [tabular Hijri Calendar](https://en.wikipedia.org/wiki/Tabular_Islamic_calendar) (civil epoch)
///
/// This is a tabular/arithmetic Hijri calendar with leap years (1-based) 2, 5, 7, 10,
/// 13, 16, 18, 21, 24, 26, and 29 in the 30-year cycle with civil/Friday epoch. That is,
/// the AH era starts on Friday July 16, 622 Julian / July 19, 622 proleptic Gregorian.
///
/// For an astronomic/Thusday-epoch version, see [`HijriTabular`][HijriTabular].
///
/// # Era codes
///
/// This calendar uses a single era code, `islamic-civil` (aliases `ah`, `islamicc`), Anno Hegirae.
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // unit struct
pub struct HijriCivil;

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
pub struct HijriUmmAlQura {
    data: DataPayload<CalendarHijriUmmalquraV1>,
}

#[derive(Debug, Clone, Copy)]
pub struct HijriUmmAlQuraMarker;

/// The [tabular Hijri Calendar](https://en.wikipedia.org/wiki/Tabular_Islamic_calendar) (astronomical epoch)
///
/// This is a tabular/arithmetic Hijri calendar with leap years (1-based) 2, 5, 7, 10,
/// 13, 16, 18, 21, 24, 26, and 29 in the 30-year cycle with astronomical/Thursday epoch.
/// That is, the AH era starts on Thusday July 15, 622 Julian / July 18, 622 proleptic
/// Gregorian.
///
/// For an civil/Friday-epoch version, see [`HijriCivil`][HijriCivil].
///
/// # Era codes
///
/// This calendar uses a single era code, `islamic-tbla` (alias `ah`), Anno Hegirae.
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // unit struct
pub struct HijriTabular;

impl HijriObservational {
    /// Creates a new [`HijriObservational`] for reference location Mecca, with some compiled data containing precomputed calendrical calculations.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_mecca() -> Self {
        Self {
            location: HijriObservationalLocation::Mecca,
            data: Some(DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_HIJRI_OBSERVATIONAL_MECCA_V1,
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
    pub fn try_new_mecca_unstable<D: DataProvider<CalendarHijriObservationalMeccaV1> + ?Sized>(
        provider: &D,
    ) -> Result<Self, DataError> {
        Ok(Self {
            location: HijriObservationalLocation::Mecca,
            data: Some(provider.load(Default::default())?.payload.cast()),
        })
    }

    /// Construct a new [`HijriObservational`] for reference location Mecca, without any precomputed calendrical calculations.
    pub const fn new_mecca_always_calculating() -> Self {
        Self {
            location: HijriObservationalLocation::Mecca,
            data: None,
        }
    }

    /// Compute a cache for this calendar
    #[cfg(feature = "datagen")]
    pub fn build_cache(&self, extended_years: core::ops::Range<i32>) -> HijriCache<'static> {
        let data = extended_years
            .clone()
            .map(|year| HijriYearInfo::compute_for_year(year, self.location).pack())
            .collect();
        HijriCache {
            first_extended_year: extended_years.start,
            data,
        }
    }
}

impl HijriCivil {
    /// Construct a new [`HijriCivil`]
    pub fn new() -> Self {
        Self
    }
}

impl HijriUmmAlQura {
    /// Creates a new [`HijriUmmAlQura`] with some compiled data containing precomputed calendrical calculations.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_HIJRI_UMMALQURA_V1,
            ),
        }
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
    ]);

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<D: DataProvider<CalendarHijriUmmalquraV1> + ?Sized>(
        provider: &D,
    ) -> Result<Self, DataError> {
        Ok(Self {
            data: provider.load(Default::default())?.payload,
        })
    }

    /// Compute a cache for this calendar
    #[cfg(feature = "datagen")]
    pub fn build_cache(
        first_extended_year: i32,
        month_lengths_and_year_starts: impl Iterator<Item = ([bool; 12], i64)>,
    ) -> HijriCache<'static> {
        let data = month_lengths_and_year_starts
            .enumerate()
            .map(|(offset, (month_lengths, year_start))| {
                HijriYearInfo::from_parts(
                    first_extended_year + offset as i32,
                    month_lengths,
                    ISLAMIC_EPOCH_FRIDAY + year_start,
                    HijriUmmAlQuraMarker,
                )
                .pack()
            })
            .collect();
        HijriCache {
            first_extended_year,
            data,
        }
    }
}

impl HijriTabular {
    /// Construct a new [`HijriTabular`]
    pub fn new() -> Self {
        Self
    }
}

pub trait CacheableHijri: Copy {
    /// The name of the calendar for debugging.
    const DEBUG_NAME: &'static str;
    /// Convert a Hijri date in this calendar to a R.D.
    fn fixed_from_hijri(&self, year: i32, month: u8, day: u8) -> RataDie;
    /// Convert an R.D. to a Hijri date in this calendar
    fn hijri_from_fixed(&self, date: RataDie) -> (i32, u8, u8);
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HijriYearInfo<IB: CacheableHijri> {
    month_lengths: [bool; 12],
    ny_offset: i64,
    model: IB,
    value: i32,
}

impl<IB: CacheableHijri> From<HijriYearInfo<IB>> for i32 {
    fn from(value: HijriYearInfo<IB>) -> Self {
        value.value
    }
}

impl HijriCache<'_> {
    /// Get the cached data for a given extended year
    fn get<IB: CacheableHijri>(&self, extended_year: i32, model: IB) -> Option<HijriYearInfo<IB>> {
        let (month_lengths, ny_offset) = self
            .data
            .get(usize::try_from(extended_year - self.first_extended_year).ok()?)?
            .unpack();

        Some(HijriYearInfo {
            month_lengths,
            ny_offset,
            model,
            value: extended_year,
        })
    }
}

const LONG_YEAR_LEN: u16 = 355;
const SHORT_YEAR_LEN: u16 = 354;

impl<IB: CacheableHijri> HijriYearInfo<IB> {
    #[cfg(feature = "datagen")]
    fn from_parts(
        extended_year: i32,
        month_lengths: [bool; 12],
        year_start: RataDie,
        model: IB,
    ) -> Self {
        Self {
            month_lengths,
            ny_offset: year_start - Self::mean_synodic_ny(extended_year),
            model,
            value: extended_year,
        }
    }

    fn compute_for_year(extended_year: i32, model: IB) -> Self {
        let ny = model.fixed_from_hijri(extended_year, 1, 1);
        let next_ny = model.fixed_from_hijri(extended_year + 1, 1, 1);
        match (next_ny - ny) as u16 {
            LONG_YEAR_LEN | SHORT_YEAR_LEN => (),
            353 => {
                icu_provider::log::trace!(
                    "({}) Found year {extended_year} AH with length {}. See <https://github.com/unicode-org/icu4x/issues/4930>",
                    IB::DEBUG_NAME,
                    next_ny - ny
                );
            }
            other => {
                debug_assert!(
                    false,
                    "({}) Found year {extended_year} AH with length {}!",
                    IB::DEBUG_NAME,
                    other
                )
            }
        }

        let ny_offset = ny - Self::mean_synodic_ny(extended_year);

        let month_lengths = {
            let mut excess_days = 0;
            let mut start_of_month = ny;
            let mut month_lengths = core::array::from_fn(|month_idx| {
                let end_of_month = if month_idx < 11 {
                    model.fixed_from_hijri(extended_year, month_idx as u8 + 2, 1)
                } else {
                    next_ny
                };
                let days_in_month = end_of_month - start_of_month;
                start_of_month = end_of_month;
                match days_in_month {
                    29 => false,
                    30 => true,
                    31 => {
                        icu_provider::log::trace!(
                            "({}) Found year {extended_year} AH with month length {days_in_month} for month {}.",
                            IB::DEBUG_NAME,
                            month_idx + 1
                        );
                        excess_days += 1;
                        true
                    }
                    _ => {
                        debug_assert!(
                            false,
                            "({}) Found year {extended_year} AH with month length {days_in_month} for month {}!",
                            IB::DEBUG_NAME,
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
                    IB::DEBUG_NAME
                );
                if let Some(l) = month_lengths.iter_mut().find(|l| !(**l)) {
                    *l = true;
                }
            }
            month_lengths
        };
        Self {
            month_lengths,
            ny_offset,
            model,
            value: extended_year,
        }
    }

    fn compute_for_rd(rd: RataDie, model: IB) -> Self {
        let (y, _m, _d) = model.hijri_from_fixed(rd);
        Self::compute_for_year(y, model)
    }

    #[cfg(feature = "datagen")]
    fn pack(&self) -> PackedHijriYearInfo {
        PackedHijriYearInfo::new(self.month_lengths, self.ny_offset)
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

    fn mean_synodic_ny(extended_year: i32) -> RataDie {
        // -1 because the epoch is new year of year 1
        // truncating instead of flooring does not matter, as this is well-defined for
        // positive years only
        ISLAMIC_EPOCH_FRIDAY
            + ((extended_year - 1) as f64 * calendrical_calculations::islamic::MEAN_YEAR_LENGTH)
                as i64
    }

    /// Get the new year R.D. given the extended year that this yearinfo is for    
    fn new_year(self) -> RataDie {
        Self::mean_synodic_ny(self.value) + self.ny_offset
    }

    /// Get the date's R.D. given (m, d) in this info's year
    fn md_to_rd(self, month: u8, day: u8) -> RataDie {
        let ny = self.new_year();
        let month_offset = if month == 1 {
            0
        } else {
            self.last_day_of_month(month - 1)
        };
        // -1 since the offset is 1-indexed but the new year is also day 1
        ny - 1 + month_offset.into() + day.into()
    }

    fn md_from_rd(self, rd: RataDie) -> (u8, u8) {
        let day_of_year = (rd - self.new_year()) as u16;
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

/// Contains any loaded precomputed data. If constructed with Default, will
/// *not* contain any extra data and will always compute stuff from scratch
#[derive(Default)]
struct HijriPrecomputedData<'a, IB: CacheableHijri> {
    data: Option<&'a HijriCache<'a>>,
    model: IB,
}

impl<'b, IB: CacheableHijri> HijriPrecomputedData<'b, IB> {
    fn new(data: Option<&'b HijriCache<'b>>, model: IB) -> Self {
        Self { data, model }
    }

    /// Returns the [`HijriYearInfo`] loading from cache or computing.
    fn load_or_compute_info_for_rd(&self, rd: RataDie) -> HijriYearInfo<IB> {
        self.data
            .and_then(|d| {
                // +1 because the epoch is new year of year 1
                // truncating instead of flooring does not matter, as this is well-defined for
                // positive years only
                let extended_year = ((rd - calendrical_calculations::islamic::ISLAMIC_EPOCH_FRIDAY)
                    as f64
                    / calendrical_calculations::islamic::MEAN_YEAR_LENGTH)
                    as i32
                    + 1;

                let year = d.get(extended_year, self.model)?;

                if rd < year.new_year() {
                    d.get(extended_year - 1, self.model)
                } else {
                    let next_year = d.get(extended_year + 1, self.model)?;
                    Some(if rd < next_year.new_year() {
                        year
                    } else {
                        next_year
                    })
                }
            })
            .unwrap_or_else(|| HijriYearInfo::compute_for_rd(rd, self.model))
    }
}

impl<IB: CacheableHijri> PrecomputedDataSource<HijriYearInfo<IB>> for HijriPrecomputedData<'_, IB> {
    fn load_or_compute_info(&self, extended_year: i32) -> HijriYearInfo<IB> {
        self.data
            .and_then(|d| d.get(extended_year, self.model))
            .unwrap_or_else(|| HijriYearInfo::compute_for_year(extended_year, self.model))
    }
}

/// The inner date type used for representing [`Date`]s of [`HijriObservational`]. See [`Date`] and [`HijriObservational`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriDateInner(ArithmeticDate<HijriObservational>);

impl CalendarArithmetic for HijriObservational {
    type YearInfo = HijriYearInfo<HijriObservationalLocation>;

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

impl Calendar for HijriObservational {
    type DateInner = HijriDateInner;
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
        Ok(HijriDateInner(ArithmeticDate::new_from_ordinals(
            self.precomputed_data().load_or_compute_info(year),
            month,
            day,
        )?))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let y = self.precomputed_data().load_or_compute_info_for_rd(rd);
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
        date.0.offset_date(offset, &self.precomputed_data())
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

impl CacheableHijri for HijriObservationalLocation {
    const DEBUG_NAME: &'static str = HijriObservational::DEBUG_NAME;

    fn fixed_from_hijri(&self, year: i32, month: u8, day: u8) -> RataDie {
        calendrical_calculations::islamic::fixed_from_observational_islamic(
            year,
            month,
            day,
            self.location(),
        )
    }
    fn hijri_from_fixed(&self, date: RataDie) -> (i32, u8, u8) {
        calendrical_calculations::islamic::observational_islamic_from_fixed(date, self.location())
    }
}

impl HijriObservational {
    fn precomputed_data(&self) -> HijriPrecomputedData<HijriObservationalLocation> {
        match self.location {
            HijriObservationalLocation::Mecca => HijriPrecomputedData::new(
                self.data.as_ref().map(|x| x.get()),
                HijriObservationalLocation::Mecca,
            ),
        }
    }

    pub(crate) const DEBUG_NAME: &'static str = "Hijri (observational)";
}

impl<A: AsCalendar<Calendar = HijriObservational>> Date<A> {
    /// Construct new Hijri Observational Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::cal::HijriObservational;
    /// use icu::calendar::Date;
    ///
    /// let hijri = HijriObservational::new_mecca_always_calculating();
    ///
    /// let date_hijri =
    ///     Date::try_new_observational_hijri_with_calendar(1392, 4, 25, hijri)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.year().era_year_or_extended(), 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_observational_hijri_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        let y = calendar
            .as_calendar()
            .precomputed_data()
            .load_or_compute_info(year);
        ArithmeticDate::new_from_ordinals(y, month, day)
            .map(HijriDateInner)
            .map(|inner| Date::from_raw(inner, calendar))
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`HijriUmmAlQura`]. See [`Date`] and [`HijriUmmAlQura`] for more details.
pub struct HijriUmmAlQuraDateInner(ArithmeticDate<HijriUmmAlQura>);

impl CalendarArithmetic for HijriUmmAlQura {
    type YearInfo = HijriYearInfo<HijriUmmAlQuraMarker>;

    fn days_in_provided_month(year: Self::YearInfo, month: u8) -> u8 {
        year.days_in_month(month)
    }

    fn months_in_provided_year(_year: HijriYearInfo<HijriUmmAlQuraMarker>) -> u8 {
        12
    }

    fn days_in_provided_year(year: Self::YearInfo) -> u16 {
        year.days_in_year()
    }

    // As an true lunar calendar, it does not have leap years.
    fn provided_year_is_leap(year: Self::YearInfo) -> bool {
        year.days_in_year() != SHORT_YEAR_LEN
    }

    fn last_month_day_in_provided_year(year: HijriYearInfo<HijriUmmAlQuraMarker>) -> (u8, u8) {
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
            self.precomputed_data().load_or_compute_info(year),
            month,
            day,
        )?))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let y = self.precomputed_data().load_or_compute_info_for_rd(rd);
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
        date.0.offset_date(offset, &self.precomputed_data())
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

impl CacheableHijri for HijriUmmAlQuraMarker {
    const DEBUG_NAME: &'static str = HijriUmmAlQura::DEBUG_NAME;
    fn fixed_from_hijri(&self, year: i32, month: u8, day: u8) -> RataDie {
        calendrical_calculations::islamic::fixed_from_islamic_civil(year, month, day)
    }
    fn hijri_from_fixed(&self, date: RataDie) -> (i32, u8, u8) {
        calendrical_calculations::islamic::islamic_civil_from_fixed(date)
    }
}

impl HijriUmmAlQura {
    fn precomputed_data(&self) -> HijriPrecomputedData<HijriUmmAlQuraMarker> {
        HijriPrecomputedData::new(Some(self.data.get()), HijriUmmAlQuraMarker)
    }

    pub(crate) const DEBUG_NAME: &'static str = "Hijri (Umm al-Qura)";
}

impl<A: AsCalendar<Calendar = HijriUmmAlQura>> Date<A> {
    /// Construct new Hijri Umm al-Qura Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::cal::HijriUmmAlQura;
    /// use icu::calendar::Date;
    ///
    /// let hijri = HijriUmmAlQura::new();
    ///
    /// let date_hijri =
    ///     Date::try_new_ummalqura_with_calendar(1392, 4, 25, hijri)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.year().era_year_or_extended(), 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_ummalqura_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        let y = calendar
            .as_calendar()
            .precomputed_data()
            .load_or_compute_info(year);
        Ok(Date::from_raw(
            HijriUmmAlQuraDateInner(ArithmeticDate::new_from_ordinals(y, month, day)?),
            calendar,
        ))
    }
}

/// The inner date type used for representing [`Date`]s of [`HijriCivil`]. See [`Date`] and [`HijriCivil`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriCivilDateInner(ArithmeticDate<HijriCivil>);

impl CalendarArithmetic for HijriCivil {
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

impl Calendar for HijriCivil {
    type DateInner = HijriCivilDateInner;

    fn from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            Some("islamic-civil" | "islamicc" | "ah") | None => year,
            Some(_) => return Err(DateError::UnknownEra),
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(HijriCivilDateInner)
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let (y, m, d) = calendrical_calculations::islamic::islamic_civil_from_fixed(rd);

        debug_assert!(Date::try_new_hijri_civil_with_calendar(y, m, d, HijriCivil).is_ok());
        HijriCivilDateInner(ArithmeticDate::new_unchecked(y, m, d))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        calendrical_calculations::islamic::fixed_from_islamic_civil(
            date.0.year,
            date.0.month,
            date.0.day,
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
        "Hijri (civil)"
    }

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        year_as_hijri(tinystr!(16, "islamic-civil"), date.0.year)
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

impl<A: AsCalendar<Calendar = HijriCivil>> Date<A> {
    /// Construct new Civil Hijri Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::cal::HijriCivil;
    /// use icu::calendar::Date;
    ///
    /// let hijri = HijriCivil::new();
    ///
    /// let date_hijri =
    ///     Date::try_new_hijri_civil_with_calendar(1392, 4, 25, hijri)
    ///         .expect("Failed to initialize Hijri Date instance.");
    ///
    /// assert_eq!(date_hijri.year().era_year_or_extended(), 1392);
    /// assert_eq!(date_hijri.month().ordinal, 4);
    /// assert_eq!(date_hijri.day_of_month().0, 25);
    /// ```
    pub fn try_new_hijri_civil_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, RangeError> {
        ArithmeticDate::new_from_ordinals(year, month, day)
            .map(HijriCivilDateInner)
            .map(|inner| Date::from_raw(inner, calendar))
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
            Some("islamic-tbla" | "ah") | None => year,
            Some(_) => return Err(DateError::UnknownEra),
        };
        ArithmeticDate::new_from_codes(self, year, month_code, day).map(HijriTabularDateInner)
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let (y, m, d) = calendrical_calculations::islamic::islamic_tabular_from_fixed(rd);

        debug_assert!(Date::try_new_hijri_civil_with_calendar(y, m, d, HijriCivil).is_ok());
        HijriTabularDateInner(ArithmeticDate::new_unchecked(y, m, d))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        calendrical_calculations::islamic::fixed_from_islamic_tabular(
            date.0.year,
            date.0.month,
            date.0.day,
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
        "Hijri (tabular)"
    }

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        year_as_hijri(tinystr!(16, "islamic-tbla"), date.0.year)
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
    /// let hijri = HijriTabular::new();
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

    static OBSERVATIONAL_CASES: [DateCase; 33] = [
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

    static TABULAR_CASES: [DateCase; 33] = [
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
    fn test_observational_hijri_from_rd() {
        let calendar = HijriObservational::new_mecca();
        let calendar = Ref(&calendar);
        for (case, f_date) in OBSERVATIONAL_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_observational_hijri_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            let iso = Date::from_rata_die(RataDie::new(*f_date), Iso);

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_observational_hijri() {
        let calendar = HijriObservational::new_mecca();
        let calendar = Ref(&calendar);
        for (case, f_date) in OBSERVATIONAL_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_observational_hijri_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_hijri() {
        let calendar = HijriCivil::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_RD.iter()) {
            let date =
                Date::try_new_hijri_civil_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_hijri_from_rd() {
        let calendar = HijriCivil::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_RD.iter()) {
            let date =
                Date::try_new_hijri_civil_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_hijri_tbla() {
        let calendar = HijriTabular::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in TABULAR_CASES.iter().zip(TEST_RD.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[test]
    fn test_hijri_tbla_from_rd() {
        let calendar = HijriTabular::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in TABULAR_CASES.iter().zip(TEST_RD.iter()) {
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
            let date =
                Date::try_new_ummalqura_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

            assert_eq!(date, date_rd, "{case:?}");
        }
    }

    #[test]
    fn test_rd_from_saudi_hijri() {
        let calendar = HijriUmmAlQura::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in UMMALQURA_CASES.iter().zip(TEST_RD.iter()) {
            let date =
                Date::try_new_ummalqura_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
        }
    }

    #[ignore] // slow
    #[test]
    fn test_days_in_provided_year_observational() {
        let calendar = HijriObservational::new_mecca();
        let calendar = Ref(&calendar);
        // -1245 1 1 = -214526 (R.D Date)
        // 1518 1 1 = 764589 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| {
                HijriObservational::days_in_provided_year(HijriYearInfo::compute_for_year(
                    year,
                    HijriObservationalLocation::Mecca,
                )) as i64
            })
            .sum();
        let expected_number_of_days =
            Date::try_new_observational_hijri_with_calendar(END_YEAR, 1, 1, calendar)
                .unwrap()
                .to_rata_die()
                - Date::try_new_observational_hijri_with_calendar(START_YEAR, 1, 1, calendar)
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
        let calendar = HijriUmmAlQura::new();
        let calendar = Ref(&calendar);
        // -1245 1 1 = -214528 (R.D Date)
        // 1518 1 1 = 764588 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| {
                HijriUmmAlQura::days_in_provided_year(HijriYearInfo::compute_for_year(
                    year,
                    HijriUmmAlQuraMarker,
                )) as i64
            })
            .sum();
        let expected_number_of_days =
            Date::try_new_ummalqura_with_calendar(END_YEAR, 1, 1, calendar)
                .unwrap()
                .to_rata_die()
                - (Date::try_new_ummalqura_with_calendar(START_YEAR, 1, 1, calendar).unwrap())
                    .to_rata_die(); // The number of days between Umm al-Qura Hijri years -1245 and 1518

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

        let dt_cached = Date::try_new_ummalqura_with_calendar(1391, 1, 29, cached).unwrap();

        assert_eq!(dt_cached.to_iso().to_calendar(cached), dt_cached);
    }

    #[test]
    fn test_regression_5069_obs() {
        let cached = HijriObservational::new_mecca();
        let comp = HijriObservational::new_mecca_always_calculating();

        let cached = crate::Ref(&cached);
        let comp = crate::Ref(&comp);

        let dt_cached =
            Date::try_new_observational_hijri_with_calendar(1390, 1, 30, cached).unwrap();
        let dt_comp = Date::try_new_observational_hijri_with_calendar(1390, 1, 30, comp).unwrap();

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
}
