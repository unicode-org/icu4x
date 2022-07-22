// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Japanese calendar.
//!
//! ```rust
//! use icu::calendar::{types::Era, Date, DateTime};
//! use icu::calendar::japanese::{Japanese, JapaneseEraStyle};
//! use tinystr::tinystr;
//!
//! // `icu_testdata::get_provider` contains information specifying era dates.
//! // Production code should probably use its own data provider
//! let provider = icu_testdata::get_provider();
//! let japanese_calendar = Japanese::try_new(&provider, JapaneseEraStyle::Modern).expect("Cannot load japanese data");
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
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value};
use icu_provider::prelude::*;
use tinystr::{tinystr, TinyStr16};

/// Which eras to include in the calendar.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum JapaneseEraStyle {
    /// This includes all "modern" (post-Meiji) eras,
    /// using Gregorian eras for dates preceding the Meiji era
    Modern,
    /// This includes all known eras, using Gregorian eras for dates
    /// preceding the earliest known era
    All,
}

impl Default for JapaneseEraStyle {
    fn default() -> Self {
        Self::Modern
    }
}

/// The Japanese Calendar
#[derive(Clone, Debug, Default)]
pub struct Japanese {
    eras: DataPayload<provider::JapaneseErasV1Marker>,
    pub(crate) japanext: bool,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing Date<Japanese>
pub struct JapaneseDateInner {
    inner: IsoDateInner,
    adjusted_year: i32,
    era: TinyStr16,
}

impl Japanese {
    /// Creates a new [`Japanese`] from locale data and an options bag.
    ///
    /// Setting `historical_eras` will load historical (pre-meiji) era data
    #[allow(clippy::expect_used)] // can be removed after #1800
    pub fn try_new<D: DataProvider<provider::JapaneseErasV1Marker> + ?Sized>(
        data_provider: &D,
        era_style: JapaneseEraStyle,
    ) -> Result<Self, DataError> {
        let japanext = era_style == JapaneseEraStyle::All;

        let mut locale = icu_locid::Locale::default();
        locale.extensions.unicode.keywords.set(
            key!("ca"),
            if japanext {
                value!("japanext")
            } else {
                value!("japanese")
            },
        );

        let eras = data_provider
            .load(DataRequest {
                locale: (&locale).into(),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { eras, japanext })
    }
}

impl Calendar for Japanese {
    type DateInner = JapaneseDateInner;
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
        "Japanese"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        if self.japanext {
            Some(AnyCalendarKind::Japanext)
        } else {
            Some(AnyCalendarKind::Japanese)
        }
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
}
