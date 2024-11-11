// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde definitions for semantic skeleta

use crate::{dynamic::*, fieldset, neo_skeleton::*, raw::neo::RawNeoOptions};
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
    pub(crate) field_set: FieldSetSerde,
    pub(crate) length: NeoSkeletonLength,
    pub(crate) alignment: Option<Alignment>,
    #[serde(rename = "yearStyle")]
    pub(crate) year_style: Option<YearStyle>,
    #[serde(rename = "timePrecision")]
    pub(crate) time_precision: Option<TimePrecision>,
}

impl From<CompositeFieldSet> for SemanticSkeletonSerde {
    fn from(value: CompositeFieldSet) -> Self {
        let (serde_field, options) = match value {
            CompositeFieldSet::Date(v) => FieldSetSerde::from_date_field_set(v),
            CompositeFieldSet::CalendarPeriod(v) => {
                FieldSetSerde::from_calendar_period_field_set(v)
            }
            CompositeFieldSet::Time(v) => FieldSetSerde::from_time_field_set(v),
            CompositeFieldSet::Zone(v) => {
                let zone_serde = FieldSetSerde::from_time_zone_style(v.style);
                (
                    zone_serde,
                    RawNeoOptions {
                        length: v.length,
                        alignment: None,
                        year_style: None,
                        time_precision: None,
                    },
                )
            }
            CompositeFieldSet::DateTime(v) => {
                let (date_serde, date_options) =
                    FieldSetSerde::from_date_field_set(v.to_date_field_set());
                let (time_serde, time_options) =
                    FieldSetSerde::from_time_field_set(v.to_time_field_set());
                (
                    date_serde.extend(time_serde),
                    date_options.merge(time_options),
                )
            }
            CompositeFieldSet::DateZone(v, z) => {
                let (date_serde, date_options) = FieldSetSerde::from_date_field_set(v);
                let zone_serde = FieldSetSerde::from_time_zone_style(z);
                (date_serde.extend(zone_serde), date_options)
            }
            CompositeFieldSet::TimeZone(v, z) => {
                let (time_serde, time_options) = FieldSetSerde::from_time_field_set(v);
                let zone_serde = FieldSetSerde::from_time_zone_style(z);
                (time_serde.extend(zone_serde), time_options)
            }
            CompositeFieldSet::DateTimeZone(v, z) => {
                let (date_serde, date_options) =
                    FieldSetSerde::from_date_field_set(v.to_date_field_set());
                let (time_serde, time_options) =
                    FieldSetSerde::from_time_field_set(v.to_time_field_set());
                let zone_serde = FieldSetSerde::from_time_zone_style(z);
                (
                    date_serde.extend(time_serde).extend(zone_serde),
                    date_options.merge(time_options),
                )
            }
        };
        Self {
            field_set: serde_field,
            length: options.length,
            alignment: options.alignment,
            year_style: options.year_style,
            time_precision: options.time_precision,
        }
    }
}

impl TryFrom<SemanticSkeletonSerde> for CompositeFieldSet {
    type Error = Error;
    fn try_from(value: SemanticSkeletonSerde) -> Result<Self, Self::Error> {
        let date = value.field_set.date_only();
        let time = value.field_set.time_only();
        let zone = value.field_set.zone_only();
        let options = RawNeoOptions {
            length: value.length,
            alignment: value.alignment,
            year_style: value.year_style,
            time_precision: value.time_precision,
        };
        match (!date.is_empty(), !time.is_empty(), !zone.is_empty()) {
            (true, false, false) => date
                .to_date_field_set(options)
                .map(CompositeFieldSet::Date)
                .or_else(|| {
                    date.to_calendar_period_field_set(options)
                        .map(CompositeFieldSet::CalendarPeriod)
                })
                .ok_or(Error::InvalidFields),
            (false, true, false) => time
                .to_time_field_set(options)
                .map(CompositeFieldSet::Time)
                .ok_or(Error::InvalidFields),
            (false, false, true) => zone
                .to_time_zone_style()
                .map(|style| {
                    CompositeFieldSet::Zone(TimeZoneStyleWithLength {
                        style,
                        length: options.length,
                    })
                })
                .ok_or(Error::InvalidFields),
            (true, true, false) => date
                .to_date_field_set(options)
                .map(|date_field_set| {
                    CompositeFieldSet::DateTime(
                        DateAndTimeFieldSet::from_date_field_set_with_raw_options(
                            date_field_set,
                            options,
                        ),
                    )
                })
                .ok_or(Error::InvalidFields),
            (true, false, true) => date
                .to_date_field_set(options)
                .and_then(|date_field_set| {
                    zone.to_time_zone_style()
                        .map(|style| CompositeFieldSet::DateZone(date_field_set, style))
                })
                .ok_or(Error::InvalidFields),
            (false, true, true) => time
                .to_time_field_set(options)
                .and_then(|time_field_set| {
                    zone.to_time_zone_style()
                        .map(|style| CompositeFieldSet::TimeZone(time_field_set, style))
                })
                .ok_or(Error::InvalidFields),
            (true, true, true) => date
                .to_date_field_set(options)
                .and_then(|date_field_set| {
                    zone.to_time_zone_style().map(|style| {
                        CompositeFieldSet::DateTimeZone(
                            DateAndTimeFieldSet::from_date_field_set_with_raw_options(
                                date_field_set,
                                options,
                            ),
                            style,
                        )
                    })
                })
                .ok_or(Error::InvalidFields),
            (false, false, false) => Err(Error::NoFields),
        }
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

impl FieldSetSerde {
    fn from_date_field_set(value: DateFieldSet) -> (Self, RawNeoOptions) {
        match value {
            DateFieldSet::D(v) => (Self::DAY, v.to_raw_options()),
            DateFieldSet::MD(v) => (Self::MONTH_DAY, v.to_raw_options()),
            DateFieldSet::YMD(v) => (Self::YEAR_MONTH_DAY, v.to_raw_options()),
            DateFieldSet::DE(v) => (Self::DAY_WEEKDAY, v.to_raw_options()),
            DateFieldSet::MDE(v) => (Self::MONTH_DAY_WEEKDAY, v.to_raw_options()),
            DateFieldSet::YMDE(v) => (Self::YEAR_MONTH_DAY_WEEKDAY, v.to_raw_options()),
            DateFieldSet::E(v) => (Self::WEEKDAY, v.to_raw_options()),
        }
    }

    fn to_date_field_set(self, options: RawNeoOptions) -> Option<DateFieldSet> {
        use DateFieldSet::*;
        match self {
            Self::DAY => Some(D(fieldset::D::from_raw_options(options))),
            Self::MONTH_DAY => Some(MD(fieldset::MD::from_raw_options(options))),
            Self::YEAR_MONTH_DAY => Some(YMD(fieldset::YMD::from_raw_options(options))),
            Self::DAY_WEEKDAY => Some(DE(fieldset::DE::from_raw_options(options))),
            Self::MONTH_DAY_WEEKDAY => Some(MDE(fieldset::MDE::from_raw_options(options))),
            Self::YEAR_MONTH_DAY_WEEKDAY => Some(YMDE(fieldset::YMDE::from_raw_options(options))),
            Self::WEEKDAY => Some(E(fieldset::E::from_raw_options(options))),
            _ => None,
        }
    }

    fn from_calendar_period_field_set(value: CalendarPeriodFieldSet) -> (Self, RawNeoOptions) {
        match value {
            CalendarPeriodFieldSet::M(v) => (Self::MONTH, v.to_raw_options()),
            CalendarPeriodFieldSet::YM(v) => (Self::YEAR_MONTH, v.to_raw_options()),
            CalendarPeriodFieldSet::Y(v) => (Self::YEAR, v.to_raw_options()),
        }
    }

    fn to_calendar_period_field_set(
        self,
        options: RawNeoOptions,
    ) -> Option<CalendarPeriodFieldSet> {
        use CalendarPeriodFieldSet::*;
        match self {
            Self::MONTH => Some(M(fieldset::M::from_raw_options(options))),
            Self::YEAR_MONTH => Some(YM(fieldset::YM::from_raw_options(options))),
            Self::YEAR => Some(Y(fieldset::Y::from_raw_options(options))),
            _ => None,
        }
    }

    fn from_time_field_set(value: TimeFieldSet) -> (Self, RawNeoOptions) {
        match value {
            TimeFieldSet::T(v) => (Self::TIME, v.to_raw_options()),
        }
    }

    fn to_time_field_set(self, options: RawNeoOptions) -> Option<TimeFieldSet> {
        use TimeFieldSet::*;
        match self {
            Self::TIME => Some(T(fieldset::T::from_raw_options(options))),
            _ => None,
        }
    }

    fn from_time_zone_style(value: NeoTimeZoneStyle) -> Self {
        match value {
            NeoTimeZoneStyle::Location => Self::ZONE_LOCATION,
            NeoTimeZoneStyle::Generic => Self::ZONE_GENERIC,
            NeoTimeZoneStyle::Specific => Self::ZONE_SPECIFIC,
            NeoTimeZoneStyle::Offset => Self::ZONE_OFFSET,
            _ => todo!(),
        }
    }

    fn to_time_zone_style(self) -> Option<NeoTimeZoneStyle> {
        match self {
            FieldSetSerde::ZONE_LOCATION => Some(NeoTimeZoneStyle::Location),
            FieldSetSerde::ZONE_GENERIC => Some(NeoTimeZoneStyle::Generic),
            FieldSetSerde::ZONE_SPECIFIC => Some(NeoTimeZoneStyle::Specific),
            FieldSetSerde::ZONE_OFFSET => Some(NeoTimeZoneStyle::Offset),
            _ => None,
        }
    }
}

#[test]
fn test_basic() {
    let skeleton = CompositeFieldSet::DateTimeZone(
        DateAndTimeFieldSet::YMDT(fieldset::YMDT {
            length: NeoSkeletonLength::Medium,
            alignment: Some(Alignment::Column),
            year_style: Some(YearStyle::Always),
            time_precision: Some(TimePrecision::SecondExact(FractionalSecondDigits::F3)),
        }),
        NeoTimeZoneStyle::Generic,
    );

    let json_string = serde_json::to_string(&skeleton).unwrap();
    assert_eq!(
        json_string,
        r#"{"fieldSet":["year","month","day","weekday","time","zoneGeneric"],"length":"medium","alignment":"column","yearStyle":"always","timePrecision":"secondF3"}"#
    );
    let json_skeleton = serde_json::from_str::<CompositeFieldSet>(&json_string).unwrap();
    assert_eq!(skeleton, json_skeleton);

    let bincode_bytes = bincode::serialize(&skeleton).unwrap();
    let bincode_skeleton = bincode::deserialize::<CompositeFieldSet>(&bincode_bytes).unwrap();
    assert_eq!(skeleton, bincode_skeleton);
}
