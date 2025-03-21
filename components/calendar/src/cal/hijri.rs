// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Hijri calendars.
//!
//! ```rust
//! use icu::calendar::cal::HijriObservational;
//! use icu::calendar::Date;
//!
//! let hijri = HijriObservational::new_cairo_always_calculating();
//! let hijri_date = Date::try_new_observational_hijri_with_calendar(
//!     1348, 10, 11, hijri,
//! )
//! .expect("Failed to initialize Hijri Date instance.");
//!
//! assert_eq!(hijri_date.year().era_year_or_extended(), 1348);
//! assert_eq!(hijri_date.month().ordinal, 10);
//! assert_eq!(hijri_date.day_of_month().0, 11);
//! ```

use crate::cal::Iso;
use crate::calendar_arithmetic::PrecomputedDataSource;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::error::DateError;
use crate::provider::hijri::{
    CalendarHijriObservationalCairoV1, CalendarHijriUmmalquraV1, HijriCache, PackedHijriYearInfo,
};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use crate::{AsCalendar, RangeError};
use calendrical_calculations::islamic::{
    IslamicBasedMarker, ObservationalCairoIslamicMarker, SaudiIslamicMarker,
};
use calendrical_calculations::rata_die::RataDie;
use core::marker::PhantomData;
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
/// This calendar uses a single era code, `islamic` (alias `ah`), Anno Hegirae.
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
pub enum HijriObservationalLocation {
    Cairo,
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
    data: Option<DataPayload<CalendarHijriUmmalquraV1>>,
}

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
    /// Creates a new [`HijriObservational`] for reference location Cairo, with some compiled data containing precomputed calendrical calculations.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_cairo() -> Self {
        Self {
            location: HijriObservationalLocation::Cairo,
            data: Some(DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_HIJRI_OBSERVATIONAL_CAIRO_V1,
            )),
        }
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_cairo_with_buffer_provider,
            try_new_cairo_unstable,
            Self,
    ]);

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_cairo)]
    pub fn try_new_cairo_unstable<D: DataProvider<CalendarHijriObservationalCairoV1> + ?Sized>(
        provider: &D,
    ) -> Result<Self, DataError> {
        Ok(Self {
            location: HijriObservationalLocation::Cairo,
            data: Some(provider.load(Default::default())?.payload.cast()),
        })
    }

    /// Construct a new [`HijriObservational`] for reference location Cairo, without any precomputed calendrical calculations.
    pub fn new_cairo_always_calculating() -> Self {
        Self {
            location: HijriObservationalLocation::Cairo,
            data: None,
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
            data: Some(DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_HIJRI_UMMALQURA_V1,
            )),
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
            data: Some(provider.load(Default::default())?.payload),
        })
    }

    /// Construct a new [`HijriUmmAlQura`] without any precomputed calendrical calculations.
    pub fn new_always_calculating() -> Self {
        Self { data: None }
    }
}

impl HijriTabular {
    /// Construct a new [`HijriTabular`]
    pub fn new() -> Self {
        Self
    }
}

/// Compact representation of the length of a Hijri year.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum HijriYearLength {
    /// Long (355-day) Hijri year
    L355,
    /// Short (354-day) Hijri year
    L354,
    /// Unexpectedly Short (353-day) Hijri year
    ///
    /// It is probably a bug when this year length is returned. See:
    /// <https://github.com/unicode-org/icu4x/issues/4930>
    L353,
}

impl Default for HijriYearLength {
    fn default() -> Self {
        Self::L354
    }
}

impl HijriYearLength {
    fn try_from_int(value: i64) -> Option<Self> {
        match value {
            355 => Some(Self::L355),
            354 => Some(Self::L354),
            353 => Some(Self::L353),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HijriYearInfo {
    packed_data: PackedHijriYearInfo,
    prev_year_length: HijriYearLength,
}

impl HijriYearInfo {
    pub(crate) const LONG_YEAR_LEN: u16 = 355;
    const SHORT_YEAR_LEN: u16 = 354;
    pub(crate) fn new(
        prev_packed: PackedHijriYearInfo,
        this_packed: PackedHijriYearInfo,
        extended_year: i32,
    ) -> (Self, i32) {
        let days_in_year = prev_packed.days_in_year();
        let days_in_year = match HijriYearLength::try_from_int(days_in_year as i64) {
            Some(x) => x,
            None => {
                debug_assert!(false, "Found wrong year length for Hijri year {extended_year}: Expected 355, 354, or 353, got {days_in_year}");
                Default::default()
            }
        };
        let year_info = Self {
            prev_year_length: days_in_year,
            packed_data: this_packed,
        };
        (year_info, extended_year)
    }

    fn compute<IB: IslamicBasedMarker>(extended_year: i32) -> Self {
        let ny = IB::fixed_from_islamic(extended_year, 1, 1);
        let packed_data = PackedHijriYearInfo::compute_with_ny::<IB>(extended_year, ny);
        let prev_ny = IB::fixed_from_islamic(extended_year - 1, 1, 1);
        let rd_diff = ny - prev_ny;
        let rd_diff = match HijriYearLength::try_from_int(rd_diff) {
            Some(x) => x,
            None => {
                debug_assert!(false, "({}) Found wrong year length for Hijri year {extended_year}: Expected 355, 354, or 353, got {rd_diff}", IB::DEBUG_NAME);
                Default::default()
            }
        };
        Self {
            prev_year_length: rd_diff,
            packed_data,
        }
    }
    /// Get the new year R.D. given the extended year that this yearinfo is for    
    fn new_year<IB: IslamicBasedMarker>(self, extended_year: i32) -> RataDie {
        IB::mean_synodic_ny(extended_year) + i64::from(self.packed_data.ny_offset())
    }

    /// Get the date's R.D. given (y, m, d) in this info's year
    fn rd_for<IB: IslamicBasedMarker>(self, extended_year: i32, month: u8, day: u8) -> RataDie {
        let ny = self.new_year::<IB>(extended_year);
        let month_offset = if month == 1 {
            0
        } else {
            self.packed_data.last_day_of_month(month - 1)
        };
        // -1 since the offset is 1-indexed but the new year is also day 1
        ny - 1 + month_offset.into() + day.into()
    }
}

/// Contains any loaded precomputed data. If constructed with Default, will
/// *not* contain any extra data and will always compute stuff from scratch
#[derive(Default)]
pub(crate) struct HijriPrecomputedData<'a, IB: IslamicBasedMarker> {
    data: Option<&'a HijriCache<'a>>,
    _ib: PhantomData<IB>,
}

impl<IB: IslamicBasedMarker> PrecomputedDataSource<HijriYearInfo> for HijriPrecomputedData<'_, IB> {
    fn load_or_compute_info(&self, extended_year: i32) -> HijriYearInfo {
        self.data
            .and_then(|d| d.get_for_extended_year(extended_year))
            .unwrap_or_else(|| HijriYearInfo::compute::<IB>(extended_year))
    }
}

/// Given a year info and the first month it is possible for this date to be in, return the
/// month and day this is in
fn compute_month_day(info: HijriYearInfo, mut possible_month: u8, day_of_year: u16) -> (u8, u8) {
    let mut last_day_of_month = info.packed_data.last_day_of_month(possible_month);
    let mut last_day_of_prev_month = if possible_month == 1 {
        0
    } else {
        info.packed_data.last_day_of_month(possible_month - 1)
    };

    while day_of_year > last_day_of_month && possible_month <= 12 {
        possible_month += 1;
        last_day_of_prev_month = last_day_of_month;
        last_day_of_month = info.packed_data.last_day_of_month(possible_month);
    }
    let day = u8::try_from(day_of_year - last_day_of_prev_month);
    debug_assert!(
        day.is_ok(),
        "Found day {} that doesn't fit in month!",
        day_of_year - last_day_of_prev_month
    );
    (possible_month, day.unwrap_or(29))
}
impl<'b, IB: IslamicBasedMarker> HijriPrecomputedData<'b, IB> {
    pub(crate) fn new(data: Option<&'b HijriCache<'b>>) -> Self {
        Self {
            data,
            _ib: PhantomData,
        }
    }
    /// Given an ISO date (in both ArithmeticDate and R.D. format), returns the [`HijriYearInfo`] and extended year for that date, loading
    /// from cache or computing.
    fn load_or_compute_info_for_iso(&self, fixed: RataDie) -> (HijriYearInfo, i32, u8, u8) {
        let cached = self.data.and_then(|d| d.get_for_fixed::<IB>(fixed));
        if let Some((cached, year)) = cached {
            let ny = cached.packed_data.ny::<IB>(year);
            let day_of_year = (fixed - ny) as u16 + 1;
            debug_assert!(day_of_year < 360);
            // We divide by 30, not 29, to account for the case where all months before this
            // were length 30 (possible near the beginning of the year)
            // We add +1 because months are 1-indexed
            let possible_month = u8::try_from(1 + (day_of_year / 30)).unwrap_or(1);
            let (m, d) = compute_month_day(cached, possible_month, day_of_year);
            return (cached, year, m, d);
        };
        // compute

        let (y, m, d) = IB::islamic_from_fixed(fixed);
        let info = HijriYearInfo::compute::<IB>(y);
        let ny = info.packed_data.ny::<IB>(y);
        let day_of_year = (fixed - ny) as u16 + 1;
        // We can't use the m/d from islamic_from_fixed because that code
        // occasionally throws up 31-day months, which we normalize out. So we instead back-compute, starting with the previous month
        let (m, d) = if m > 1 {
            compute_month_day(info, m - 1, day_of_year)
        } else {
            (m, d)
        };
        (info, y, m, d)
    }
}

/// The inner date type used for representing [`Date`]s of [`HijriObservational`]. See [`Date`] and [`HijriObservational`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriDateInner(ArithmeticDate<HijriObservational>);

impl CalendarArithmetic for HijriObservational {
    type YearInfo = HijriYearInfo;

    fn month_days(_year: i32, month: u8, year_info: HijriYearInfo) -> u8 {
        year_info.packed_data.days_in_month(month)
    }

    fn months_for_every_year(_year: i32, _year_info: HijriYearInfo) -> u8 {
        12
    }

    fn days_in_provided_year(_year: i32, year_info: HijriYearInfo) -> u16 {
        year_info.packed_data.days_in_year()
    }

    // As an true lunar calendar, it does not have leap years.
    fn is_leap_year(_year: i32, year_info: HijriYearInfo) -> bool {
        year_info.packed_data.days_in_year() != HijriYearInfo::SHORT_YEAR_LEN
    }

    fn last_month_day_in_year(year: i32, year_info: HijriYearInfo) -> (u8, u8) {
        let days = Self::month_days(year, 12, year_info);

        (12, days)
    }
}

impl Calendar for HijriObservational {
    type DateInner = HijriDateInner;
    fn date_from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            Some("islamic" | "ah") | None => year,
            Some(_) => return Err(DateError::UnknownEra),
        };
        let Some((month, false)) = month_code.parsed() else {
            return Err(DateError::UnknownMonthCode(month_code));
        };
        Ok(HijriDateInner(ArithmeticDate::new_from_ordinals_with_info(
            year,
            month,
            day,
            self.precomputed_data().load_or_compute_info(year),
        )?))
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        let fixed_iso = Iso::to_fixed(iso);

        let (year_info, y, m, d) = self
            .precomputed_data()
            .load_or_compute_info_for_iso(fixed_iso);
        HijriDateInner(ArithmeticDate::new_unchecked_with_info(y, m, d, year_info))
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        let fixed = date.0.year_info.rd_for::<ObservationalCairoIslamicMarker>(
            date.0.year,
            date.0.month,
            date.0.day,
        );
        Iso::from_fixed(fixed)
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

    fn day_of_week(&self, date: &Self::DateInner) -> types::Weekday {
        Iso.day_of_week(self.date_to_iso(date).inner())
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
        year_as_hijri(tinystr!(16, "islamic"), date.0.year)
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::is_leap_year(date.0.year, date.0.year_info)
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

impl HijriObservational {
    fn precomputed_data(&self) -> HijriPrecomputedData<ObservationalCairoIslamicMarker> {
        match self.location {
            HijriObservationalLocation::Cairo => {
                HijriPrecomputedData::new(self.data.as_ref().map(|x| x.get()))
            }
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
    /// let hijri = HijriObservational::new_cairo_always_calculating();
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
        let year_info = calendar
            .as_calendar()
            .precomputed_data()
            .load_or_compute_info(year);
        ArithmeticDate::new_from_ordinals_with_info(year, month, day, year_info)
            .map(HijriDateInner)
            .map(|inner| Date::from_raw(inner, calendar))
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`HijriUmmAlQura`]. See [`Date`] and [`HijriUmmAlQura`] for more details.
pub struct HijriUmmAlQuraDateInner(ArithmeticDate<HijriUmmAlQura>);

impl CalendarArithmetic for HijriUmmAlQura {
    type YearInfo = HijriYearInfo;

    fn month_days(_year: i32, month: u8, year_info: HijriYearInfo) -> u8 {
        year_info.packed_data.days_in_month(month)
    }

    fn months_for_every_year(_year: i32, _year_info: HijriYearInfo) -> u8 {
        12
    }

    fn days_in_provided_year(_year: i32, year_info: HijriYearInfo) -> u16 {
        year_info.packed_data.days_in_year()
    }

    // As an true lunar calendar, it does not have leap years.
    fn is_leap_year(_year: i32, year_info: HijriYearInfo) -> bool {
        year_info.packed_data.days_in_year() != HijriYearInfo::SHORT_YEAR_LEN
    }

    fn last_month_day_in_year(year: i32, year_info: HijriYearInfo) -> (u8, u8) {
        let days = Self::month_days(year, 12, year_info);

        (12, days)
    }
}

impl Calendar for HijriUmmAlQura {
    type DateInner = HijriUmmAlQuraDateInner;
    fn date_from_codes(
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
        Ok(HijriUmmAlQuraDateInner(
            ArithmeticDate::new_from_ordinals_with_info(
                year,
                month,
                day,
                self.precomputed_data().load_or_compute_info(year),
            )?,
        ))
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::to_fixed(iso);

        let (year_info, y, m, d) = self
            .precomputed_data()
            .load_or_compute_info_for_iso(fixed_iso);
        HijriUmmAlQuraDateInner(ArithmeticDate::new_unchecked_with_info(y, m, d, year_info))
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed =
            date.0
                .year_info
                .rd_for::<SaudiIslamicMarker>(date.0.year, date.0.month, date.0.day);
        Iso::from_fixed(fixed)
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
        year_as_hijri(tinystr!(16, "islamic-umalqura"), date.0.year)
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::is_leap_year(date.0.year, date.0.year_info)
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

impl HijriUmmAlQura {
    fn precomputed_data(&self) -> HijriPrecomputedData<SaudiIslamicMarker> {
        HijriPrecomputedData::new(self.data.as_ref().map(|x| x.get()))
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
    /// let hijri = HijriUmmAlQura::new_always_calculating();
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
        let year_info = calendar
            .as_calendar()
            .precomputed_data()
            .load_or_compute_info(year);
        Ok(Date::from_raw(
            HijriUmmAlQuraDateInner(ArithmeticDate::new_from_ordinals_with_info(
                year, month, day, year_info,
            )?),
            calendar,
        ))
    }
}

/// The inner date type used for representing [`Date`]s of [`HijriCivil`]. See [`Date`] and [`HijriCivil`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HijriCivilDateInner(ArithmeticDate<HijriCivil>);

impl CalendarArithmetic for HijriCivil {
    type YearInfo = ();

    fn month_days(year: i32, month: u8, _data: ()) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 if Self::is_leap_year(year, ()) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_for_every_year(_year: i32, _data: ()) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32, _data: ()) -> u16 {
        if Self::is_leap_year(year, ()) {
            HijriYearInfo::LONG_YEAR_LEN
        } else {
            HijriYearInfo::SHORT_YEAR_LEN
        }
    }

    fn is_leap_year(year: i32, _data: ()) -> bool {
        (14 + 11 * year).rem_euclid(30) < 11
    }

    fn last_month_day_in_year(year: i32, _data: ()) -> (u8, u8) {
        if Self::is_leap_year(year, ()) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for HijriCivil {
    type DateInner = HijriCivilDateInner;

    fn date_from_codes(
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

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::to_fixed(iso);
        Self::hijri_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_hijri = Self::fixed_from_hijri(*date);
        Iso::from_fixed(fixed_hijri)
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

    fn day_of_week(&self, date: &Self::DateInner) -> types::Weekday {
        Iso.day_of_week(self.date_to_iso(date).inner())
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
        Self::is_leap_year(date.0.year, ())
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

impl HijriCivil {
    fn fixed_from_hijri(i_date: HijriCivilDateInner) -> RataDie {
        calendrical_calculations::islamic::fixed_from_islamic_civil(
            i_date.0.year,
            i_date.0.month,
            i_date.0.day,
        )
    }

    fn hijri_from_fixed(date: RataDie) -> Date<HijriCivil> {
        let (y, m, d) = calendrical_calculations::islamic::islamic_civil_from_fixed(date);

        debug_assert!(Date::try_new_hijri_civil_with_calendar(y, m, d, HijriCivil).is_ok());
        Date::from_raw(
            HijriCivilDateInner(ArithmeticDate::new_unchecked(y, m, d)),
            HijriCivil,
        )
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
    type YearInfo = ();

    fn month_days(year: i32, month: u8, _data: ()) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 if Self::is_leap_year(year, ()) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_for_every_year(_year: i32, _data: ()) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32, _data: ()) -> u16 {
        if Self::is_leap_year(year, ()) {
            HijriYearInfo::LONG_YEAR_LEN
        } else {
            HijriYearInfo::SHORT_YEAR_LEN
        }
    }

    fn is_leap_year(year: i32, _data: ()) -> bool {
        (14 + 11 * year).rem_euclid(30) < 11
    }

    fn last_month_day_in_year(year: i32, _data: ()) -> (u8, u8) {
        if Self::is_leap_year(year, ()) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for HijriTabular {
    type DateInner = HijriTabularDateInner;

    fn date_from_codes(
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

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::to_fixed(iso);
        Self::hijri_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_hijri = Self::fixed_from_hijri(*date);
        Iso::from_fixed(fixed_hijri)
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

    fn day_of_week(&self, date: &Self::DateInner) -> types::Weekday {
        Iso.day_of_week(self.date_to_iso(date).inner())
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
        Self::is_leap_year(date.0.year, ())
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

impl HijriTabular {
    fn fixed_from_hijri(i_date: HijriTabularDateInner) -> RataDie {
        calendrical_calculations::islamic::fixed_from_islamic_tabular(
            i_date.0.year,
            i_date.0.month,
            i_date.0.day,
        )
    }

    fn hijri_from_fixed(date: RataDie) -> Date<HijriTabular> {
        let (y, m, d) = calendrical_calculations::islamic::islamic_tabular_from_fixed(date);

        debug_assert!(Date::try_new_hijri_civil_with_calendar(y, m, d, HijriCivil).is_ok());
        Date::from_raw(
            HijriTabularDateInner(ArithmeticDate::new_unchecked(y, m, d)),
            HijriTabular,
        )
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

    static TEST_FIXED_DATE: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];
    // Removed: 601716 and 727274 fixed dates
    static TEST_FIXED_DATE_UMMALQURA: [i64; 31] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 613424, 626596, 645554, 664224,
        671401, 694799, 704424, 708842, 709409, 709580, 728714, 744313, 764652,
    ];

    static UMMALQURA_DATE_EXPECTED: [DateCase; 31] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 11,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 26,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 3,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 8,
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
            day: 22,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 3,
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
            year: 1091,
            month: 6,
            day: 4,
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
            day: 8,
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

    static OBSERVATIONAL_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 11,
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
            day: 30,
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
    fn test_observational_hijri_from_fixed() {
        let calendar = HijriObservational::new_cairo();
        let calendar = Ref(&calendar);
        for (case, f_date) in OBSERVATIONAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_observational_hijri_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            let iso = Iso::from_fixed(RataDie::new(*f_date));

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_fixed_from_observational_hijri() {
        let calendar = HijriObservational::new_cairo();
        let calendar = Ref(&calendar);
        for (case, f_date) in OBSERVATIONAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_observational_hijri_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(
                Iso::to_fixed(date.to_iso()),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_fixed_from_hijri() {
        let calendar = HijriCivil::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date =
                Date::try_new_hijri_civil_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            assert_eq!(
                Iso::to_fixed(date.to_iso()),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_hijri_from_fixed() {
        let calendar = HijriCivil::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date =
                Date::try_new_hijri_civil_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            let iso = Iso::from_fixed(RataDie::new(*f_date));

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_fixed_from_hijri_tbla() {
        let calendar = HijriTabular::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in TABULAR_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            assert_eq!(
                Iso::to_fixed(date.to_iso()),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_hijri_tbla_from_fixed() {
        let calendar = HijriTabular::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in TABULAR_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_hijri_tabular_with_calendar(
                case.year, case.month, case.day, calendar,
            )
            .unwrap();
            let iso = Iso::from_fixed(RataDie::new(*f_date));

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_saudi_hijri_from_fixed() {
        let calendar = HijriUmmAlQura::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in UMMALQURA_DATE_EXPECTED
            .iter()
            .zip(TEST_FIXED_DATE_UMMALQURA.iter())
        {
            let date =
                Date::try_new_ummalqura_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            let iso = Iso::from_fixed(RataDie::new(*f_date));

            assert_eq!(iso.to_calendar(calendar).inner, date.inner, "{case:?}");
        }
    }

    #[test]
    fn test_fixed_from_saudi_hijri() {
        let calendar = HijriUmmAlQura::new();
        let calendar = Ref(&calendar);
        for (case, f_date) in UMMALQURA_DATE_EXPECTED
            .iter()
            .zip(TEST_FIXED_DATE_UMMALQURA.iter())
        {
            let date =
                Date::try_new_ummalqura_with_calendar(case.year, case.month, case.day, calendar)
                    .unwrap();
            assert_eq!(
                Iso::to_fixed(date.to_iso()),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[ignore] // slow
    #[test]
    fn test_days_in_provided_year_observational() {
        let calendar = HijriObservational::new_cairo();
        let calendar = Ref(&calendar);
        // -1245 1 1 = -214526 (R.D Date)
        // 1518 1 1 = 764589 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| {
                HijriObservational::days_in_provided_year(
                    year,
                    HijriYearInfo::compute::<ObservationalCairoIslamicMarker>(year),
                ) as i64
            })
            .sum();
        let expected_number_of_days = (Iso::to_fixed(
            (Date::try_new_observational_hijri_with_calendar(END_YEAR, 1, 1, calendar).unwrap())
                .to_iso(),
        )) - Iso::to_fixed(
            (Date::try_new_observational_hijri_with_calendar(START_YEAR, 1, 1, calendar).unwrap())
                .to_iso(),
        ); // The number of days between Hijri years -1245 and 1518
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
                HijriUmmAlQura::days_in_provided_year(
                    year,
                    HijriYearInfo::compute::<SaudiIslamicMarker>(year),
                ) as i64
            })
            .sum();
        let expected_number_of_days = (Iso::to_fixed(
            (Date::try_new_ummalqura_with_calendar(END_YEAR, 1, 1, calendar).unwrap()).to_iso(),
        )) - Iso::to_fixed(
            (Date::try_new_ummalqura_with_calendar(START_YEAR, 1, 1, calendar).unwrap()).to_iso(),
        ); // The number of days between Umm al-Qura Hijri years -1245 and 1518

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
        let cal = HijriUmmAlQura::new_always_calculating();
        let era = "islamic-umalqura";
        let year = -6823;
        let month_code = MonthCode(tinystr!(4, "M01"));
        let dt = cal.date_from_codes(Some(era), year, month_code, 1).unwrap();
        assert_eq!(dt.0.day, 1);
        assert_eq!(dt.0.month, 1);
        assert_eq!(dt.0.year, -6823);
    }
}
