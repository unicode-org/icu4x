// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::*;
use std::str::FromStr;

use super::{datetime::MockDateTime, timezone::MockTimeZone};
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
        let time_zone = match input
            .rfind(|c| c == '+' || /* ASCII */ c == '-' || /* U+2212 */ c == '−' || c == 'Z')
        {
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
        self.datetime.year()
    }

    fn month(&self) -> Option<Month> {
        self.datetime.month()
    }

    fn day_of_month(&self) -> Option<DayOfMonth> {
        self.datetime.day_of_month()
    }

    fn iso_weekday(&self) -> Option<IsoWeekday> {
        self.datetime.iso_weekday()
    }

    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        self.datetime.day_of_year_info()
    }
}

impl IsoTimeInput for MockZonedDateTime {
    fn hour(&self) -> Option<IsoHour> {
        self.datetime.hour()
    }

    fn minute(&self) -> Option<IsoMinute> {
        self.datetime.minute()
    }

    fn second(&self) -> Option<IsoSecond> {
        self.datetime.second()
    }

    fn fraction(&self) -> Option<FractionalSecond> {
        self.datetime.fraction()
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
}
