// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::arithmetic;
use crate::date::*;
use std::str::FromStr;
use tinystr::tinystr8;

use super::{
    datetime::MockDateTime,
    timezone::{GmtOffset, MockTimeZone},
};
#[derive(Debug, Default)]
pub struct MockZonedDateTime {
    pub datetime: MockDateTime,
    pub time_zone: MockTimeZone,
}

impl MockZonedDateTime {
    pub const fn new(datetime: MockDateTime, time_zone: MockTimeZone) -> Self {
        Self {
            datetime,
            time_zone,
        }
    }

    pub fn try_new(datetime: MockDateTime, time_zone: MockTimeZone) -> Result<Self, DateTimeError> {
        Ok(Self {
            datetime,
            time_zone,
        })
    }
}

impl FromStr for MockZonedDateTime {
    type Err = DateTimeError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let datetime = MockDateTime::from_str(input)?;
        let time_zone = match input.rfind(|c| c == '+' || c == '-' || c == 'Z') {
            Some(index) => MockTimeZone::from_str(&input[index..])?,
            None => return Err(DateTimeError::MissingTimeZoneOffset),
        };

        Ok(MockZonedDateTime {
            datetime,
            time_zone,
        })
    }
}

impl DateInput for MockZonedDateTime {
    fn year(&self) -> Option<Year> {
        Some(arithmetic::iso_year_to_gregorian(self.datetime.year))
    }

    fn month(&self) -> Option<Month> {
        Some(Month {
            number: self.datetime.month + 1,
            // TODO(#486): Implement month codes
            code: MonthCode(tinystr8!("TODO")),
        })
    }

    fn day_of_month(&self) -> Option<DayOfMonth> {
        Some(DayOfMonth(self.datetime.day + 1))
    }

    fn iso_weekday(&self) -> Option<IsoWeekday> {
        Some(arithmetic::iso_date_to_weekday(
            self.datetime.year,
            self.datetime.month as usize,
            self.datetime.day as usize,
        ))
    }

    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        unimplemented!()
    }
}

impl IsoTimeInput for MockZonedDateTime {
    fn hour(&self) -> Option<IsoHour> {
        Some(self.datetime.hour)
    }

    fn minute(&self) -> Option<IsoMinute> {
        Some(self.datetime.minute)
    }

    fn second(&self) -> Option<IsoSecond> {
        Some(self.datetime.second)
    }

    fn fraction(&self) -> Option<FractionalSecond> {
        None
    }
}

impl TimeZoneInput for MockZonedDateTime {
    fn gmt_offset(&self) -> GmtOffset {
        self.time_zone.gmt_offset()
    }

    fn time_zone_id(&self) -> Option<&str> {
        self.time_zone.time_zone_id()
    }

    fn metazone_id(&self) -> Option<&str> {
        self.time_zone.metazone_id()
    }

    fn time_variant(&self) -> Option<&str> {
        self.time_zone.time_variant()
    }

    fn country_code(&self) -> Option<&str> {
        self.time_zone.country_code()
    }
}
