// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Japanese calendar

use crate::iso::{Iso, IsoDateInner};
use crate::provider::{self, EraStartDate};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use icu_provider::prelude::*;
use tinystr::{tinystr16, TinyStr16};

#[derive(Clone, Debug, Default)]
/// The Japanese Calendar
pub struct Japanese {
    eras: DataPayload<provider::JapaneseErasV1Marker>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing Date<Japanese>
pub struct JapaneseDateInner {
    inner: IsoDateInner,
    era: TinyStr16,
    era_start: EraStartDate,
}

impl Japanese {
    /// Creates a new [`Japanese`] from locale data and an options bag.
    pub fn try_new<D: DataProvider<provider::JapaneseErasV1Marker> + ?Sized>(
        data_provider: &D,
    ) -> Result<Self, DataError> {
        let eras = data_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::JAPANESE_ERAS_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: None,
                    },
                },
            })?
            .take_payload()?;
        Ok(Self { eras })
    }
}

impl Calendar for Japanese {
    type DateInner = JapaneseDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> JapaneseDateInner {
        let (era_start, era) = self.era_for(iso.inner());
        JapaneseDateInner {
            inner: *iso.inner(),
            era,
            era_start,
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
        Iso.offset_date(&mut date.inner, offset.cast_unit())
    }

    #[allow(clippy::field_reassign_with_default)] // it's more clear this way
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Iso.until(&date1.inner, &date2.inner, largest_unit, smallest_unit)
            .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, _date: &Self::DateInner) -> types::Year {
        unimplemented!()
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        Iso.month(&date.inner)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Iso.day_of_month(&date.inner)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, _date: &Self::DateInner) -> types::DayOfYearInfo {
        unimplemented!()
    }

    fn debug_name() -> &'static str {
        "Japanese"
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

const FALLBACK_ERA: (EraStartDate, TinyStr16) = (REIWA_START, tinystr16!("reiwa"));

impl Japanese {
    /// Given an ISO date, obtain the era data
    fn era_for(&self, date: &IsoDateInner) -> (EraStartDate, TinyStr16) {
        let date: EraStartDate = date.into();
        let era_data = self.eras.get();
        let deref_tuple = |x: (&EraStartDate, &TinyStr16)| (*x.0, *x.1);
        if date >= MEIJI_START {
            if era_data.dates_to_eras.len() == 5 {
                // Fast path in case eras have not changed since this code was written
                if date >= REIWA_START {
                    (REIWA_START, tinystr16!("reiwa"))
                } else if date >= HEISEI_START {
                    (HEISEI_START, tinystr16!("heisei"))
                } else if date >= SHOWA_START {
                    (SHOWA_START, tinystr16!("showa"))
                } else if date >= TAISHO_START {
                    (TAISHO_START, tinystr16!("taisho"))
                } else {
                    (MEIJI_START, tinystr16!("meiji"))
                }
            } else {
                era_data
                    .dates_to_eras
                    .iter()
                    .rev()
                    .find(|(k, _)| **k < date)
                    .map(deref_tuple)
                    .unwrap_or(FALLBACK_ERA)
            }
        } else {
            let historical = &era_data.dates_to_historical_eras;
            match historical.find_index(&date) {
                Ok(index) => historical
                    .get_indexed(index)
                    .map(deref_tuple)
                    .unwrap_or(FALLBACK_ERA),
                Err(index) if index == 0 => historical
                    .get_indexed(index)
                    .map(deref_tuple)
                    .unwrap_or(FALLBACK_ERA),
                Err(index) => historical
                    .get_indexed(index - 1)
                    .or_else(|| historical.get_indexed(historical.len() - 1))
                    .map(deref_tuple)
                    .unwrap_or(FALLBACK_ERA),
            }
        }
    }
}
