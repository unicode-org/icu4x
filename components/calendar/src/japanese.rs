// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Japanese calendar.
//!
//! ```rust
//! use icu::calendar::{types::Era, Date, DateTime};
//! use icu::calendar::japanese::Japanese;
//! use tinystr::tinystr;
//!
//! // `icu_testdata::get_provider` contains information specifying era dates.
//! // Production code should probably use its own data provider
//! let provider = icu_testdata::get_provider();
//! let japanese_calendar = Japanese::try_new_with_buffer_provider(&provider).expect("Cannot load japanese data");
//!
//! // `Date` type
//! let date_iso = Date::new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_japanese = Date::new_from_iso(date_iso, japanese_calendar.clone());
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_japanese = DateTime::new_from_iso(datetime_iso, japanese_calendar.clone());
//!
//! // `Date` checks
//! assert_eq!(date_japanese.year().number, 45);
//! assert_eq!(date_japanese.month().ordinal, 1);
//! assert_eq!(date_japanese.day_of_month().0, 2);
//! assert_eq!(date_japanese.year().era, Era(tinystr!(16, "showa")));
//!
//! // `DateTime` type
//! assert_eq!(datetime_japanese.date.year().number, 45);
//! assert_eq!(datetime_japanese.date.month().ordinal, 1);
//! assert_eq!(datetime_japanese.date.day_of_month().0, 2);
//! assert_eq!(
//!     datetime_japanese.date.year().era,
//!     Era(tinystr!(16, "showa"))
//! );
//! assert_eq!(datetime_japanese.time.hour.number(), 13);
//! assert_eq!(datetime_japanese.time.minute.number(), 1);
//! assert_eq!(datetime_japanese.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::iso::{Iso, IsoDateInner};
use crate::provider::{self, EraStartDate};
use crate::{
    types, AsCalendar, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError, Ref,
};
use icu_provider::prelude::*;
use tinystr::{tinystr, TinyStr16};

/// The Japanese Calendar
#[derive(Clone, Debug, Default)]
pub struct Japanese {
    eras: DataPayload<provider::JapaneseErasV1Marker>,
}

/// The Japanese Calendar with historical eras
#[derive(Clone, Debug, Default)]
pub struct JapaneseExtended(Japanese);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing Date<Japanese>
pub struct JapaneseDateInner {
    inner: IsoDateInner,
    adjusted_year: i32,
    era: TinyStr16,
}

impl Japanese {
    /// Creates a new [`Japanese`] from locale data using only modern eras (post-meiji).
    pub fn try_new_unstable<D: DataProvider<provider::JapaneseErasV1Marker> + ?Sized>(
        data_provider: &D,
    ) -> Result<Self, DataError> {
        let eras = data_provider
            .load(DataRequest {
                locale: Default::default(),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { eras })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: DataError);

    fn japanese_date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
        debug_name: &'static str,
    ) -> Result<JapaneseDateInner, DateTimeError> {
        let month = crate::calendar_arithmetic::ordinal_solar_month_from_code(month_code);
        let month = if let Some(month) = month {
            month
        } else {
            return Err(DateTimeError::UnknownMonthCode(month_code.0, debug_name));
        };

        if month > 12 {
            return Err(DateTimeError::UnknownMonthCode(month_code.0, debug_name));
        }

        self.new_japanese_date_inner(era, year, month, day)
    }
}

impl JapaneseExtended {
    /// Creates a new [`Japanese`] from locale data using all eras (including pre-meiji).
    pub fn try_new_unstable<D: DataProvider<provider::JapaneseExtendedErasV1Marker> + ?Sized>(
        data_provider: &D,
    ) -> Result<Self, DataError> {
        let eras = data_provider
            .load(DataRequest {
                locale: Default::default(),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self(Japanese { eras: eras.cast() }))
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: DataError);
}

impl Calendar for Japanese {
    type DateInner = JapaneseDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateTimeError> {
        self.japanese_date_from_codes(era, year, month_code, day, self.debug_name())
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> JapaneseDateInner {
        let (adjusted_year, era) = self.adjusted_year_for(iso.inner());
        JapaneseDateInner {
            inner: *iso.inner(),
            adjusted_year,
            era,
        }
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Date::from_raw(date.inner, Iso)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Iso.months_in_year(&date.inner)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Iso.days_in_year(&date.inner)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Iso.days_in_month(&date.inner)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Iso.offset_date(&mut date.inner, offset.cast_unit());
        let (adjusted_year, era) = self.adjusted_year_for(&date.inner);
        date.adjusted_year = adjusted_year;
        date.era = era
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Iso.until(
            &date1.inner,
            &date2.inner,
            &Iso,
            largest_unit,
            smallest_unit,
        )
        .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(date.era),
            number: date.adjusted_year,
            related_iso: None,
        }
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        Iso.month(&date.inner)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Iso.day_of_month(&date.inner)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_dec_31 = IsoDateInner::dec_31(date.inner.0.year - 1);
        let next_jan_1 = IsoDateInner::jan_1(date.inner.0.year + 1);

        let prev_dec_31 = self.date_from_iso(Date::from_raw(prev_dec_31, Iso));
        let next_jan_1 = self.date_from_iso(Date::from_raw(next_jan_1, Iso));
        types::DayOfYearInfo {
            day_of_year: Iso::days_in_year_direct(date.inner.0.year),
            days_in_year: Iso::days_in_year_direct(date.inner.0.year),
            prev_year: self.year(&prev_dec_31),
            days_in_prev_year: Iso::days_in_year_direct(prev_dec_31.inner.0.year),
            next_year: self.year(&next_jan_1),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Japanese (Modern eras only)"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Japanese)
    }
}

impl Calendar for JapaneseExtended {
    type DateInner = JapaneseDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateTimeError> {
        self.0
            .japanese_date_from_codes(era, year, month_code, day, self.debug_name())
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> JapaneseDateInner {
        Japanese::date_from_iso(&self.0, iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Japanese::date_to_iso(&self.0, date)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Japanese::months_in_year(&self.0, date)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Japanese::days_in_year(&self.0, date)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Japanese::days_in_month(&self.0, date)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Japanese::offset_date(&self.0, date, offset.cast_unit())
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Japanese::until(
            &self.0,
            date1,
            date2,
            &calendar2.0,
            largest_unit,
            smallest_unit,
        )
        .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Japanese::year(&self.0, date)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        Japanese::month(&self.0, date)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Japanese::day_of_month(&self.0, date)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        Japanese::day_of_year_info(&self.0, date)
    }

    fn debug_name(&self) -> &'static str {
        "Japanese (With historical eras)"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::JapaneseExtended)
    }
}

impl Date<Japanese> {
    /// Construct a new Japanese Date.
    ///
    /// Years are specified in the era provided, and must be in range for Japanese
    /// eras (e.g. dates past April 30 Heisei 31 must be in Reiwa; "Jun 5 Heisei 31" and "Jan 1 Heisei 32"
    /// will not be adjusted to being in Reiwa 1 and 2 respectively)
    ///
    /// However, dates may always be specified in "bce" or "ce" and they will be adjusted as necessary.
    ///
    /// ```rust
    /// use icu::calendar::{types, Date, Ref};
    /// use icu::calendar::japanese::Japanese;
    /// use std::convert::TryFrom;
    /// use tinystr::tinystr;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let japanese_calendar = Japanese::try_new_with_buffer_provider(&provider).expect("Cannot load japanese data");
    /// // for easy sharing
    /// let japanese_calendar = Ref(&japanese_calendar);
    ///
    /// let era = types::Era(tinystr!(16, "heisei"));
    ///
    /// let date = Date::new_japanese_date(era, 14, 1, 2, japanese_calendar)
    ///     .expect("Constructing a date should succeed");
    ///
    /// assert_eq!(date.year().era, era);
    /// assert_eq!(date.year().number, 14);
    /// assert_eq!(date.month().ordinal, 1);
    /// assert_eq!(date.day_of_month().0, 2);
    ///
    /// // This function will error for eras that are out of bounds:
    /// // (Heisei was 32 years long, Heisei 33 is in Reiwa)
    /// let oob_date = Date::new_japanese_date(era, 33, 1, 2, japanese_calendar);
    /// assert!(oob_date.is_err());
    ///
    /// // and for unknown eras
    /// let fake_era = types::Era(tinystr!(16, "neko")); // 🐱
    /// let fake_date = Date::new_japanese_date(fake_era, 10, 1, 2, japanese_calendar);
    /// assert!(fake_date.is_err());
    /// ```
    pub fn new_japanese_date<A: AsCalendar<Calendar = Japanese>>(
        era: types::Era,
        year: i32,
        month: u8,
        day: u8,
        japanese_calendar: A,
    ) -> Result<Date<A>, DateTimeError> {
        let inner = japanese_calendar
            .as_calendar()
            .new_japanese_date_inner(era, year, month, day)?;
        Ok(Date::from_raw(inner, japanese_calendar))
    }
}

impl Date<JapaneseExtended> {
    /// Construct a new Japanese Date with all eras.
    ///
    /// Years are specified in the era provided, and must be in range for Japanese
    /// eras (e.g. dates past April 30 Heisei 31 must be in Reiwa; "Jun 5 Heisei 31" and "Jan 1 Heisei 32"
    /// will not be adjusted to being in Reiwa 1 and 2 respectively)
    ///
    /// However, dates may always be specified in "bce" or "ce" and they will be adjusted as necessary.
    ///
    /// ```rust
    /// use icu::calendar::{types, Date, Ref};
    /// use icu::calendar::japanese::{JapaneseExtended};
    /// use std::convert::TryFrom;
    /// use tinystr::tinystr;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let japanext_calendar = JapaneseExtended::try_new_with_buffer_provider(&provider).expect("Cannot load japanese data");
    /// // for easy sharing
    /// let japanext_calendar = Ref(&japanext_calendar);
    ///
    /// let era = types::Era(tinystr!(16, "kansei-1789"));
    ///
    /// let date = Date::new_japanese_extended_date(era, 7, 1, 2, japanext_calendar)
    ///     .expect("Constructing a date should succeed");
    ///
    /// assert_eq!(date.year().era, era);
    /// assert_eq!(date.year().number, 7);
    /// assert_eq!(date.month().ordinal, 1);
    /// assert_eq!(date.day_of_month().0, 2);
    /// ```
    pub fn new_japanese_extended_date<A: AsCalendar<Calendar = JapaneseExtended>>(
        era: types::Era,
        year: i32,
        month: u8,
        day: u8,
        japanext_calendar: A,
    ) -> Result<Date<A>, DateTimeError> {
        let inner = japanext_calendar
            .as_calendar()
            .0
            .new_japanese_date_inner(era, year, month, day)?;
        Ok(Date::from_raw(inner, japanext_calendar))
    }

    /// For testing era fallback in icu_datetime
    #[doc(hidden)]
    pub fn into_japanese_date(self) -> Date<Japanese> {
        Date::from_raw(self.inner, self.calendar.0)
    }
}

impl DateTime<Japanese> {
    /// Construct a new Japanese datetime from integers.
    ///
    /// Years are specified in the era provided.
    ///
    /// ```rust
    /// use icu::calendar::{types, DateTime};
    /// use icu::calendar::japanese::Japanese;
    /// use std::convert::TryFrom;
    /// use tinystr::tinystr;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let japanese_calendar = Japanese::try_new_with_buffer_provider(&provider).expect("Cannot load japanese data");
    ///
    /// let era = types::Era(tinystr!(16, "heisei"));
    ///
    /// let datetime = DateTime::new_japanese_datetime(era, 14, 1, 2, 13, 1, 0, japanese_calendar)
    ///     .expect("Constructing a date should succeed");
    ///
    /// assert_eq!(datetime.date.year().era, era);
    /// assert_eq!(datetime.date.year().number, 14);
    /// assert_eq!(datetime.date.month().ordinal, 1);
    /// assert_eq!(datetime.date.day_of_month().0, 2);
    /// assert_eq!(datetime.time.hour.number(), 13);
    /// assert_eq!(datetime.time.minute.number(), 1);
    /// assert_eq!(datetime.time.second.number(), 0);
    /// ```
    #[allow(clippy::too_many_arguments)] // it's more convenient to have this many arguments
                                         // if people wish to construct this by parts they can use
                                         // Date::new_japanese_date() + DateTime::new(date, time)
    pub fn new_japanese_datetime<A: AsCalendar<Calendar = Japanese>>(
        era: types::Era,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        japanese_calendar: A,
    ) -> Result<DateTime<A>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_japanese_date(era, year, month, day, japanese_calendar)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl DateTime<JapaneseExtended> {
    /// Construct a new Japanese datetime from integers with all eras.
    ///
    /// Years are specified in the era provided.
    ///
    /// ```rust
    /// use icu::calendar::{types, DateTime};
    /// use icu::calendar::japanese::JapaneseExtended;
    /// use std::convert::TryFrom;
    /// use tinystr::tinystr;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let japanext_calendar = JapaneseExtended::try_new_with_buffer_provider(&provider).expect("Cannot load japanese data");
    ///
    /// let era = types::Era(tinystr!(16, "kansei-1789"));
    ///
    /// let datetime = DateTime::new_japanese_extended_datetime(era, 7, 1, 2, 13, 1, 0, japanext_calendar)
    ///     .expect("Constructing a date should succeed");
    ///
    /// assert_eq!(datetime.date.year().era, era);
    /// assert_eq!(datetime.date.year().number, 7);
    /// assert_eq!(datetime.date.month().ordinal, 1);
    /// assert_eq!(datetime.date.day_of_month().0, 2);
    /// assert_eq!(datetime.time.hour.number(), 13);
    /// assert_eq!(datetime.time.minute.number(), 1);
    /// assert_eq!(datetime.time.second.number(), 0);
    /// ```
    #[allow(clippy::too_many_arguments)] // it's more convenient to have this many arguments
                                         // if people wish to construct this by parts they can use
                                         // Date::new_japanese_date() + DateTime::new(date, time)
    pub fn new_japanese_extended_datetime<A: AsCalendar<Calendar = JapaneseExtended>>(
        era: types::Era,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        japanext_calendar: A,
    ) -> Result<DateTime<A>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_japanese_extended_date(era, year, month, day, japanext_calendar)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

const MEIJI_START: EraStartDate = EraStartDate {
    year: 1868,
    month: 9,
    day: 8,
};
const TAISHO_START: EraStartDate = EraStartDate {
    year: 1912,
    month: 7,
    day: 30,
};
const SHOWA_START: EraStartDate = EraStartDate {
    year: 1926,
    month: 12,
    day: 25,
};
const HEISEI_START: EraStartDate = EraStartDate {
    year: 1989,
    month: 1,
    day: 8,
};
const REIWA_START: EraStartDate = EraStartDate {
    year: 2019,
    month: 5,
    day: 1,
};

const FALLBACK_ERA: (EraStartDate, TinyStr16) = (REIWA_START, tinystr!(16, "reiwa"));

impl Japanese {
    /// Given an ISO date, give year and era for that date in the Japanese calendar
    ///
    /// This will also use Gregorian eras for eras that are before the earliest era
    fn adjusted_year_for(&self, date: &IsoDateInner) -> (i32, TinyStr16) {
        let date: EraStartDate = date.into();
        let (start, era) = self.japanese_era_for(date);
        // The year in which an era starts is Year 1, and it may be short
        // The only time this function will experience dates that are *before*
        // the era start date are for the first era (Currently, taika-645
        // for japanext, meiji for japanese),
        // In such a case, we instead fall back to Gregorian era codes
        if date < start {
            if date.year < 0 {
                (1 - date.year, tinystr!(16, "bce"))
            } else {
                (date.year, tinystr!(16, "ce"))
            }
        } else {
            (date.year - start.year + 1, era)
        }
    }

    /// Given an date, obtain the era data (not counting spliced gregorian eras)
    fn japanese_era_for(&self, date: EraStartDate) -> (EraStartDate, TinyStr16) {
        let era_data = self.eras.get();
        // We optimize for the five "modern" post-Meiji eras, which are stored in a smaller
        // array and also hardcoded. The hardcoded version is not used if data indicates the
        // presence of newer eras.
        if date >= MEIJI_START
            && era_data.dates_to_eras.last().map(|x| x.1) == Some(tinystr!(16, "reiwa"))
        {
            // Fast path in case eras have not changed since this code was written
            return if date >= REIWA_START {
                (REIWA_START, tinystr!(16, "reiwa"))
            } else if date >= HEISEI_START {
                (HEISEI_START, tinystr!(16, "heisei"))
            } else if date >= SHOWA_START {
                (SHOWA_START, tinystr!(16, "showa"))
            } else if date >= TAISHO_START {
                (TAISHO_START, tinystr!(16, "taisho"))
            } else {
                (MEIJI_START, tinystr!(16, "meiji"))
            };
        }
        let data = &era_data.dates_to_eras;
        match data.binary_search_by(|(d, _)| d.cmp(&date)) {
            Ok(index) => data.get(index),
            Err(index) if index == 0 => data.get(index),
            Err(index) => data.get(index - 1).or_else(|| data.iter().next_back()),
        }
        .unwrap_or(FALLBACK_ERA)
    }

    /// Returns the range of dates for a given Japanese era code,
    /// not handling "bce" or "ce"
    ///
    /// Returns (era_start, era_end)
    fn japanese_era_range_for(
        &self,
        era: TinyStr16,
    ) -> Result<(EraStartDate, Option<EraStartDate>), DateTimeError> {
        // Avoid linear search by trying well known eras
        if era == tinystr!(16, "reiwa") {
            // Check if we're the last
            if let Some(last) = self.eras.get().dates_to_eras.last() {
                if last.1 == era {
                    return Ok((REIWA_START, None));
                }
            }
        } else if era == tinystr!(16, "heisei") {
            return Ok((HEISEI_START, Some(REIWA_START)));
        } else if era == tinystr!(16, "showa") {
            return Ok((SHOWA_START, Some(HEISEI_START)));
        } else if era == tinystr!(16, "taisho") {
            return Ok((TAISHO_START, Some(SHOWA_START)));
        } else if era == tinystr!(16, "meiji") {
            return Ok((MEIJI_START, Some(TAISHO_START)));
        }

        let era_data = self.eras.get();
        let data = &era_data.dates_to_eras;
        // Try to avoid linear search by binary searching for the year suffix
        if let Some(year) = era.split('-').nth(1) {
            if let Ok(ref int) = year.parse::<i32>() {
                if let Ok(index) = data.binary_search_by(|(d, _)| d.year.cmp(int)) {
                    #[allow(clippy::expect_used)] // see expect message
                    let (era_start, code) = data
                        .get(index)
                        .expect("Indexing from successful binary search must succeed");
                    // There is a slight chance we hit the case where there are two eras in the same year
                    // There are a couple of rare cases of this, but it's not worth writing a range-based binary search
                    // to catch them since this is an optimization
                    if code == era {
                        return Ok((era_start, data.get(index + 1).map(|e| e.0)));
                    }
                }
            }
        }

        // Avoidance didn't work. Let's find the era manually, searching back from the present
        if let Some((index, (start, _))) = data.iter().enumerate().rev().find(|d| d.1 .1 == era) {
            return Ok((start, data.get(index + 1).map(|e| e.0)));
        }

        Err(DateTimeError::UnknownEra(era, self.debug_name()))
    }

    fn new_japanese_date_inner(
        &self,
        era: types::Era,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<JapaneseDateInner, DateTimeError> {
        let cal = Ref(self);
        if era.0 == tinystr!(16, "bce") {
            if year <= 0 {
                return Err(DateTimeError::OutOfRange);
            }
            return Ok(Date::new_iso_date(1 - year, month, day)?
                .to_calendar(cal)
                .inner);
        } else if era.0 == tinystr!(16, "ce") {
            if year <= 0 {
                return Err(DateTimeError::OutOfRange);
            }
            return Ok(Date::new_iso_date(year, month, day)?.to_calendar(cal).inner);
        }

        let (era_start, next_era_start) = self.japanese_era_range_for(era.0)?;

        let date_in_iso = EraStartDate {
            year: era_start.year + year - 1,
            month,
            day,
        };

        if date_in_iso < era_start {
            return Err(DateTimeError::OutOfRange);
        } else if let Some(next_era_start) = next_era_start {
            if date_in_iso >= next_era_start {
                return Err(DateTimeError::OutOfRange);
            }
        }

        let iso = Date::new_iso_date(date_in_iso.year, date_in_iso.month, date_in_iso.day)?;
        Ok(JapaneseDateInner {
            inner: iso.inner,
            adjusted_year: year,
            era: era.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ref;

    fn single_test_roundtrip(calendar: Ref<Japanese>, era: &str, year: i32, month: u8, day: u8) {
        let era = types::Era(era.parse().expect("era must parse"));

        let date = Date::new_japanese_date(era, year, month, day, calendar).unwrap_or_else(|e| {
            panic!(
                "Failed to construct date with {:?}, {}, {}, {}: {}",
                era, year, month, day, e
            )
        });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            date, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day}"
        )
    }

    fn single_test_roundtrip_ext(
        calendar: Ref<JapaneseExtended>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
    ) {
        let era = types::Era(era.parse().expect("era must parse"));

        let date = Date::new_japanese_extended_date(era, year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to construct date with {:?}, {}, {}, {}: {}",
                    era, year, month, day, e
                )
            });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            date, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day}"
        )
    }

    // test that the Gregorian eras roundtrip to Japanese ones
    fn single_test_gregorian_roundtrip_ext(
        calendar: Ref<JapaneseExtended>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        era2: &str,
        year2: i32,
    ) {
        let era = types::Era(era.parse().expect("era must parse"));
        let era2 = types::Era(era2.parse().expect("era must parse"));

        let expected = Date::new_japanese_extended_date(era2, year2, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to construct expectation date with {:?}, {}, {}, {}: {}",
                    era2, year2, month, day, e
                )
            });

        let date = Date::new_japanese_extended_date(era, year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to construct date with {:?}, {}, {}, {}: {}",
                    era, year, month, day, e
                )
            });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            expected, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day} == {era2:?}, {year}"
        )
    }

    fn single_test_error(
        calendar: Ref<Japanese>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        error: DateTimeError,
    ) {
        let era = types::Era(era.parse().expect("era must parse"));

        let date = Date::new_japanese_date(era, year, month, day, calendar);
        assert_eq!(
            date,
            Err(error),
            "Construction with {era:?}, {year}, {month}, {day} did not return {error:?}"
        )
    }

    fn single_test_error_ext(
        calendar: Ref<JapaneseExtended>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        error: DateTimeError,
    ) {
        let era = types::Era(era.parse().expect("era must parse"));

        let date = Date::new_japanese_extended_date(era, year, month, day, calendar);
        assert_eq!(
            date,
            Err(error),
            "Construction with {era:?}, {year}, {month}, {day} did not return {error:?}"
        )
    }

    #[test]
    fn test_japanese() {
        let provider = icu_testdata::get_provider();
        let calendar = Japanese::try_new_unstable(&provider).expect("Cannot load japanese data");
        let calendar_ext =
            JapaneseExtended::try_new_unstable(&provider).expect("Cannot load japanese data");
        let calendar = Ref(&calendar);
        let calendar_ext = Ref(&calendar_ext);

        single_test_roundtrip(calendar, "heisei", 12, 3, 1);
        single_test_roundtrip(calendar, "taisho", 3, 3, 1);
        // Heisei did not start until later in the year
        single_test_error(calendar, "heisei", 1, 1, 1, DateTimeError::OutOfRange);

        single_test_roundtrip_ext(calendar_ext, "heisei", 12, 3, 1);
        single_test_roundtrip_ext(calendar_ext, "taisho", 3, 3, 1);
        single_test_error_ext(calendar_ext, "heisei", 1, 1, 1, DateTimeError::OutOfRange);

        single_test_roundtrip_ext(calendar_ext, "hakuho-672", 4, 3, 1);
        single_test_error(
            calendar,
            "hakuho-672",
            4,
            3,
            1,
            DateTimeError::UnknownEra("hakuho-672".parse().unwrap(), "Japanese (Modern eras only)"),
        );

        // handle bce/ce
        single_test_roundtrip(calendar, "bce", 100, 3, 1);
        single_test_roundtrip(calendar, "bce", 1, 3, 1);
        single_test_roundtrip(calendar, "ce", 1, 3, 1);
        single_test_roundtrip(calendar, "ce", 100, 3, 1);
        single_test_roundtrip_ext(calendar_ext, "ce", 100, 3, 1);
        single_test_roundtrip(calendar, "ce", 1000, 3, 1);
        single_test_error(calendar, "ce", 0, 3, 1, DateTimeError::OutOfRange);
        single_test_error(calendar, "bce", -1, 3, 1, DateTimeError::OutOfRange);

        // handle the cases where bce/ce get adjusted to different eras
        // single_test_gregorian_roundtrip(calendar, "ce", 2021, 3, 1, "reiwa", 3);
        single_test_gregorian_roundtrip_ext(calendar_ext, "ce", 1000, 3, 1, "choho-999", 2);
        single_test_gregorian_roundtrip_ext(calendar_ext, "ce", 749, 5, 10, "tenpyokampo-749", 1);
        single_test_gregorian_roundtrip_ext(calendar_ext, "bce", 10, 3, 1, "bce", 10);

        // There were multiple eras in this year
        // This one is from Apr 14 to July 2
        single_test_roundtrip_ext(calendar_ext, "tenpyokampo-749", 1, 4, 20);
        single_test_roundtrip_ext(calendar_ext, "tenpyokampo-749", 1, 4, 14);
        single_test_roundtrip_ext(calendar_ext, "tenpyokampo-749", 1, 7, 1);
        single_test_error_ext(
            calendar_ext,
            "tenpyokampo-749",
            1,
            7,
            5,
            DateTimeError::OutOfRange,
        );
        single_test_error_ext(
            calendar_ext,
            "tenpyokampo-749",
            1,
            4,
            13,
            DateTimeError::OutOfRange,
        );
    }
}
