// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Japanese calendar

use crate::iso::{Iso, IsoDateInner};
use crate::provider;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use icu_provider::prelude::*;

#[derive(Clone, Debug, Default)]
/// The Japanese Calendar
pub struct Japanese {
    eras: DataPayload<provider::JapaneseErasV1Marker>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing Date<Japanese>
pub struct JapaneseDateInner(IsoDateInner);

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
        JapaneseDateInner(*iso.inner())
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Date::from_raw(date.0, Iso)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Iso.months_in_year(&date.0)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Iso.days_in_year(&date.0)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Iso.days_in_month(&date.0)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Iso.offset_date(&mut date.0, offset.cast_unit())
    }

    #[allow(clippy::field_reassign_with_default)] // it's more clear this way
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Iso.until(&date1.0, &date2.0, largest_unit, smallest_unit)
            .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, _date: &Self::DateInner) -> types::Year {
        unimplemented!()
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        Iso.month(&date.0)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Iso.day_of_month(&date.0)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, _date: &Self::DateInner) -> types::DayOfYearInfo {
        unimplemented!()
    }

    fn debug_name() -> &'static str {
        "Japanese"
    }
}
