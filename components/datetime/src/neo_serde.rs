// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde definitions for semantic skeleta

use crate::neo_skeleton::*;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Bring `Day`, `Hour`, ... into scope in this file. They are used in multiple places
use FieldSetField::*;

#[derive(displaydoc::Display)]
pub(crate) enum Error {
    #[displaydoc("at least one field is required")]
    NoFields,
    #[displaydoc("the given combination of fields does not create a valid semantic skeleton")]
    InvalidFields,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SemanticSkeletonSerde {
    #[serde(rename = "fieldSet")]
    pub(crate) field_set: NeoComponents,
    pub(crate) length: NeoSkeletonLength,
    pub(crate) alignment: Option<Alignment>,
    #[serde(rename = "yearStyle")]
    pub(crate) year_style: Option<YearStyle>,
    #[serde(rename = "timePrecision")]
    pub(crate) time_precision: Option<TimePrecision>,
}

impl From<NeoSkeleton> for SemanticSkeletonSerde {
    fn from(value: NeoSkeleton) -> Self {
        Self {
            field_set: value.components,
            length: value.length,
            alignment: value.alignment,
            year_style: value.year_style,
            time_precision: value.time_precision,
        }
    }
}

impl TryFrom<SemanticSkeletonSerde> for NeoSkeleton {
    type Error = core::convert::Infallible;
    fn try_from(value: SemanticSkeletonSerde) -> Result<Self, Self::Error> {
        Ok(NeoSkeleton {
            length: value.length,
            components: value.field_set,
            alignment: value.alignment,
            year_style: value.year_style,
            time_precision: value.time_precision,
        })
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum TimePrecisionSerde {
    HourPlus,
    HourExact,
    MinutePlus,
    MinuteExact,
    SecondPlus,
    SecondF0,
    SecondF1,
    SecondF2,
    SecondF3,
    SecondF4,
    SecondF5,
    SecondF6,
    SecondF7,
    SecondF8,
    SecondF9,
}

impl From<TimePrecision> for TimePrecisionSerde {
    fn from(value: TimePrecision) -> Self {
        match value {
            TimePrecision::HourPlus => TimePrecisionSerde::HourPlus,
            TimePrecision::HourExact => TimePrecisionSerde::HourExact,
            TimePrecision::MinutePlus => TimePrecisionSerde::MinutePlus,
            TimePrecision::MinuteExact => TimePrecisionSerde::MinuteExact,
            TimePrecision::SecondPlus => TimePrecisionSerde::SecondPlus,
            TimePrecision::SecondExact(FractionalSecondDigits::F0) => TimePrecisionSerde::SecondF0,
            TimePrecision::SecondExact(FractionalSecondDigits::F1) => TimePrecisionSerde::SecondF1,
            TimePrecision::SecondExact(FractionalSecondDigits::F2) => TimePrecisionSerde::SecondF2,
            TimePrecision::SecondExact(FractionalSecondDigits::F3) => TimePrecisionSerde::SecondF3,
            TimePrecision::SecondExact(FractionalSecondDigits::F4) => TimePrecisionSerde::SecondF4,
            TimePrecision::SecondExact(FractionalSecondDigits::F5) => TimePrecisionSerde::SecondF5,
            TimePrecision::SecondExact(FractionalSecondDigits::F6) => TimePrecisionSerde::SecondF6,
            TimePrecision::SecondExact(FractionalSecondDigits::F7) => TimePrecisionSerde::SecondF7,
            TimePrecision::SecondExact(FractionalSecondDigits::F8) => TimePrecisionSerde::SecondF8,
            TimePrecision::SecondExact(FractionalSecondDigits::F9) => TimePrecisionSerde::SecondF9,
        }
    }
}

impl From<TimePrecisionSerde> for TimePrecision {
    fn from(value: TimePrecisionSerde) -> Self {
        match value {
            TimePrecisionSerde::HourPlus => TimePrecision::HourPlus,
            TimePrecisionSerde::HourExact => TimePrecision::HourExact,
            TimePrecisionSerde::MinutePlus => TimePrecision::MinutePlus,
            TimePrecisionSerde::MinuteExact => TimePrecision::MinuteExact,
            TimePrecisionSerde::SecondPlus => TimePrecision::SecondPlus,
            TimePrecisionSerde::SecondF0 => TimePrecision::SecondExact(FractionalSecondDigits::F0),
            TimePrecisionSerde::SecondF1 => TimePrecision::SecondExact(FractionalSecondDigits::F1),
            TimePrecisionSerde::SecondF2 => TimePrecision::SecondExact(FractionalSecondDigits::F2),
            TimePrecisionSerde::SecondF3 => TimePrecision::SecondExact(FractionalSecondDigits::F3),
            TimePrecisionSerde::SecondF4 => TimePrecision::SecondExact(FractionalSecondDigits::F4),
            TimePrecisionSerde::SecondF5 => TimePrecision::SecondExact(FractionalSecondDigits::F5),
            TimePrecisionSerde::SecondF6 => TimePrecision::SecondExact(FractionalSecondDigits::F6),
            TimePrecisionSerde::SecondF7 => TimePrecision::SecondExact(FractionalSecondDigits::F7),
            TimePrecisionSerde::SecondF8 => TimePrecision::SecondExact(FractionalSecondDigits::F8),
            TimePrecisionSerde::SecondF9 => TimePrecision::SecondExact(FractionalSecondDigits::F9),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum FieldSetField {
    // Day and Date Fields
    Year = 1,
    Month = 2,
    Day = 3,
    Weekday = 4,
    WeekOfYear = 5,
    WeekOfMonth = 6,
    // Time Fields
    Time = 16,
    // Zone Fields
    ZoneGeneric = 32,
    ZoneGenericShort = 33,
    ZoneGenericLong = 34,
    ZoneSpecific = 35,
    ZoneSpecificShort = 36,
    ZoneSpecificLong = 37,
    ZoneLocation = 38,
    ZoneOffset = 39,
}

impl FieldSetField {
    const VALUES: &'static [FieldSetField] = &[
        Year,
        Month,
        Day,
        Weekday,
        Time,
        WeekOfYear,
        WeekOfMonth,
        ZoneGeneric,
        ZoneSpecific,
        ZoneLocation,
        ZoneOffset,
    ];
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct FieldSetHumanReadableSerde {
    fields: Vec<FieldSetField>,
}

impl From<FieldSetSerde> for FieldSetHumanReadableSerde {
    fn from(value: FieldSetSerde) -> Self {
        let mut fields = Vec::with_capacity(value.bit_fields.count_ones() as usize);
        for i in 0..(8 * core::mem::size_of::<u64>()) {
            if (value.bit_fields & (1 << i)) != 0 {
                // Note: This could be made more efficient, but for now it is only used in
                // human-readable deserialization which is not a hot path
                let Some(field) = FieldSetField::VALUES
                    .iter()
                    .find(|field| i == **field as usize)
                else {
                    debug_assert!(false, "unknown field discriminant: {i}");
                    break;
                };
                fields.push(*field);
            }
        }
        Self { fields }
    }
}

impl From<FieldSetHumanReadableSerde> for FieldSetSerde {
    fn from(value: FieldSetHumanReadableSerde) -> Self {
        Self::from_fields(&value.fields)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct FieldSetSerde {
    pub(crate) bit_fields: u64,
}

impl FieldSetSerde {
    // Day Components
    const DAY: Self = Self::from_fields(&[Day]);
    const MONTH_DAY: Self = Self::from_fields(&[Month, Day]);
    const YEAR_MONTH_DAY: Self = Self::from_fields(&[Year, Month, Day]);
    const DAY_WEEKDAY: Self = Self::from_fields(&[Day, Weekday]);
    const MONTH_DAY_WEEKDAY: Self = Self::from_fields(&[Month, Day, Weekday]);
    const YEAR_MONTH_DAY_WEEKDAY: Self = Self::from_fields(&[Year, Month, Day, Weekday]);
    const WEEKDAY: Self = Self::from_fields(&[Weekday]);

    // Date Components
    const MONTH: Self = Self::from_fields(&[Month]);
    const YEAR_MONTH: Self = Self::from_fields(&[Year, Month]);
    const YEAR: Self = Self::from_fields(&[Year]);
    const YEAR_WEEK: Self = Self::from_fields(&[Year, WeekOfYear]);

    // Time Components
    const TIME: Self = Self::from_fields(&[Time]);

    // Zone Components
    const ZONE_GENERIC: Self = Self::from_fields(&[ZoneGeneric]);
    const ZONE_SPECIFIC: Self = Self::from_fields(&[ZoneSpecific]);
    const ZONE_LOCATION: Self = Self::from_fields(&[ZoneLocation]);
    const ZONE_OFFSET: Self = Self::from_fields(&[ZoneOffset]);

    const fn from_fields(fields: &[FieldSetField]) -> Self {
        let mut bit_fields = 0;
        let mut i = 0;
        #[allow(clippy::indexing_slicing)] // const function, guarded by loop condition
        while i < fields.len() {
            bit_fields |= 1 << (fields[i] as usize);
            i += 1;
        }
        Self { bit_fields }
    }

    const fn date_only(self) -> Self {
        Self {
            bit_fields: self.bit_fields & 0x000000000000ffff,
        }
    }

    const fn time_only(self) -> Self {
        Self {
            bit_fields: self.bit_fields & 0x00000000ffff0000,
        }
    }

    const fn zone_only(self) -> Self {
        Self {
            bit_fields: self.bit_fields & 0x0000ffff00000000,
        }
    }

    const fn is_empty(self) -> bool {
        self.bit_fields == 0
    }

    fn extend(self, other: FieldSetSerde) -> Self {
        Self {
            bit_fields: self.bit_fields | other.bit_fields,
        }
    }
}

impl From<FieldSetField> for FieldSetSerde {
    fn from(value: FieldSetField) -> Self {
        Self {
            bit_fields: 1 << (value as usize),
        }
    }
}

impl Serialize for FieldSetSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            let human_readable = FieldSetHumanReadableSerde::from(*self);
            human_readable.serialize(serializer)
        } else {
            self.bit_fields.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for FieldSetSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let human_readable = FieldSetHumanReadableSerde::deserialize(deserializer)?;
            Ok(Self::from(human_readable))
        } else {
            let bit_fields = <u64>::deserialize(deserializer)?;
            Ok(Self { bit_fields })
        }
    }
}

impl From<NeoDateComponents> for FieldSetSerde {
    fn from(value: NeoDateComponents) -> Self {
        match value {
            NeoDateComponents::Day => Self::DAY,
            NeoDateComponents::MonthDay => Self::MONTH_DAY,
            NeoDateComponents::YearMonthDay => Self::YEAR_MONTH_DAY,
            NeoDateComponents::DayWeekday => Self::DAY_WEEKDAY,
            NeoDateComponents::MonthDayWeekday => Self::MONTH_DAY_WEEKDAY,
            NeoDateComponents::YearMonthDayWeekday => Self::YEAR_MONTH_DAY_WEEKDAY,
            NeoDateComponents::Weekday => Self::WEEKDAY,
            // TODO: support auto?
            NeoDateComponents::Auto => Self::YEAR_MONTH_DAY,
            NeoDateComponents::AutoWeekday => Self::YEAR_MONTH_DAY_WEEKDAY,
        }
    }
}

impl TryFrom<FieldSetSerde> for NeoDateComponents {
    type Error = Error;
    fn try_from(value: FieldSetSerde) -> Result<Self, Self::Error> {
        match value {
            FieldSetSerde::DAY => Ok(Self::Day),
            FieldSetSerde::MONTH_DAY => Ok(Self::MonthDay),
            FieldSetSerde::YEAR_MONTH_DAY => Ok(Self::YearMonthDay),
            FieldSetSerde::DAY_WEEKDAY => Ok(Self::DayWeekday),
            FieldSetSerde::MONTH_DAY_WEEKDAY => Ok(Self::MonthDayWeekday),
            FieldSetSerde::YEAR_MONTH_DAY_WEEKDAY => Ok(Self::YearMonthDayWeekday),
            FieldSetSerde::WEEKDAY => Ok(Self::Weekday),
            _ => Err(Error::InvalidFields),
        }
    }
}

impl From<NeoCalendarPeriodComponents> for FieldSetSerde {
    fn from(value: NeoCalendarPeriodComponents) -> Self {
        match value {
            NeoCalendarPeriodComponents::Month => Self::MONTH,
            NeoCalendarPeriodComponents::YearMonth => Self::YEAR_MONTH,
            NeoCalendarPeriodComponents::Year => Self::YEAR,
            NeoCalendarPeriodComponents::YearWeek => Self::YEAR_WEEK,
        }
    }
}

impl TryFrom<FieldSetSerde> for NeoCalendarPeriodComponents {
    type Error = Error;
    fn try_from(value: FieldSetSerde) -> Result<Self, Self::Error> {
        match value {
            FieldSetSerde::MONTH => Ok(Self::Month),
            FieldSetSerde::YEAR_MONTH => Ok(Self::YearMonth),
            FieldSetSerde::YEAR => Ok(Self::Year),
            FieldSetSerde::YEAR_WEEK => Ok(Self::YearWeek),
            _ => Err(Error::InvalidFields),
        }
    }
}

impl From<NeoTimeComponents> for FieldSetSerde {
    fn from(value: NeoTimeComponents) -> Self {
        match value {
            NeoTimeComponents::Hour => Self::TIME,
            NeoTimeComponents::HourMinute => Self::TIME,
            NeoTimeComponents::HourMinuteSecond => Self::TIME,
            // TODO: support auto?
            NeoTimeComponents::Auto => Self::TIME,
            _ => todo!(),
        }
    }
}

impl TryFrom<FieldSetSerde> for NeoTimeComponents {
    type Error = Error;
    fn try_from(value: FieldSetSerde) -> Result<Self, Self::Error> {
        match value {
            FieldSetSerde::TIME => Ok(Self::Hour),
            _ => Err(Error::InvalidFields),
        }
    }
}

impl From<NeoTimeZoneStyle> for FieldSetSerde {
    fn from(value: NeoTimeZoneStyle) -> Self {
        match value {
            NeoTimeZoneStyle::Location => Self::ZONE_LOCATION,
            NeoTimeZoneStyle::Generic => Self::ZONE_GENERIC,
            NeoTimeZoneStyle::Specific => Self::ZONE_SPECIFIC,
            NeoTimeZoneStyle::Offset => Self::ZONE_OFFSET,
            _ => todo!(),
        }
    }
}

impl TryFrom<FieldSetSerde> for NeoTimeZoneStyle {
    type Error = Error;
    fn try_from(value: FieldSetSerde) -> Result<Self, Self::Error> {
        match value {
            FieldSetSerde::ZONE_LOCATION => Ok(NeoTimeZoneStyle::Location),
            FieldSetSerde::ZONE_GENERIC => Ok(NeoTimeZoneStyle::Generic),
            FieldSetSerde::ZONE_SPECIFIC => Ok(NeoTimeZoneStyle::Specific),
            FieldSetSerde::ZONE_OFFSET => Ok(NeoTimeZoneStyle::Offset),
            _ => Err(Error::InvalidFields),
        }
    }
}

impl From<NeoComponents> for FieldSetSerde {
    fn from(value: NeoComponents) -> Self {
        match value {
            NeoComponents::Date(date) => FieldSetSerde::from(date),
            NeoComponents::CalendarPeriod(calendar_period) => FieldSetSerde::from(calendar_period),
            NeoComponents::Time(time) => FieldSetSerde::from(time),
            NeoComponents::Zone(zone) => FieldSetSerde::from(zone),
            NeoComponents::DateTime(day, time) => {
                FieldSetSerde::from(day).extend(FieldSetSerde::from(time))
            }
            NeoComponents::DateZone(date, zone) => {
                FieldSetSerde::from(date).extend(FieldSetSerde::from(zone))
            }
            NeoComponents::TimeZone(time, zone) => {
                FieldSetSerde::from(time).extend(FieldSetSerde::from(zone))
            }
            NeoComponents::DateTimeZone(day, time, zone) => FieldSetSerde::from(day)
                .extend(FieldSetSerde::from(time))
                .extend(FieldSetSerde::from(zone)),
        }
    }
}

impl TryFrom<FieldSetSerde> for NeoComponents {
    type Error = Error;
    fn try_from(value: FieldSetSerde) -> Result<Self, Self::Error> {
        let date = value.date_only();
        let time = value.time_only();
        let zone = value.zone_only();
        match (!date.is_empty(), !time.is_empty(), !zone.is_empty()) {
            (true, false, false) => NeoDateComponents::try_from(date)
                .map(NeoComponents::from)
                .or_else(|_| NeoCalendarPeriodComponents::try_from(date).map(NeoComponents::from)),
            (false, true, false) => Ok(NeoComponents::Time(time.try_into()?)),
            (false, false, true) => Ok(NeoComponents::Zone(zone.try_into()?)),
            (true, true, false) => Ok(NeoComponents::DateTime(date.try_into()?, time.try_into()?)),
            (true, false, true) => Ok(NeoComponents::DateZone(date.try_into()?, zone.try_into()?)),
            (false, true, true) => Ok(NeoComponents::TimeZone(time.try_into()?, zone.try_into()?)),
            (true, true, true) => Ok(NeoComponents::DateTimeZone(
                date.try_into()?,
                time.try_into()?,
                zone.try_into()?,
            )),
            (false, false, false) => Err(Error::NoFields),
        }
    }
}

#[test]
fn test_basic() {
    let skeleton = NeoSkeleton {
        components: NeoComponents::DateTimeZone(
            NeoDateComponents::YearMonthDayWeekday,
            NeoTimeComponents::Hour,
            NeoTimeZoneStyle::Generic,
        ),
        length: NeoSkeletonLength::Medium,
        alignment: Some(Alignment::Column),
        year_style: Some(YearStyle::Always),
        time_precision: Some(TimePrecision::SecondExact(FractionalSecondDigits::F3)),
    };

    let json_string = serde_json::to_string(&skeleton).unwrap();
    assert_eq!(
        json_string,
        r#"{"fieldSet":["year","month","day","weekday","time","zoneGeneric"],"length":"medium","alignment":"column","yearStyle":"always","timePrecision":"secondF3"}"#
    );
    let json_skeleton = serde_json::from_str::<NeoSkeleton>(&json_string).unwrap();
    assert_eq!(skeleton, json_skeleton);

    let bincode_bytes = bincode::serialize(&skeleton).unwrap();
    let bincode_skeleton = bincode::deserialize::<NeoSkeleton>(&bincode_bytes).unwrap();
    assert_eq!(skeleton, bincode_skeleton);
}
