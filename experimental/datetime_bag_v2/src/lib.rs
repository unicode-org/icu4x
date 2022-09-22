// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::convert::TryFrom;

use icu_datetime::{fields::FieldSymbol, provider::calendar::SkeletonV1};

use bitflags::bitflags;
use displaydoc::Display;

pub mod provider;

#[derive(Display, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    #[displaydoc("invalid field symbol `{0:?}` for component")]
    InvalidFieldSymbol(FieldSymbol),
    #[displaydoc("missing required fields for component")]
    MissingFieldSymbols,
}

pub enum DateFormatKind {
    EraYearMonthDay,
    EraYearMonth,
    EraYear,
    Era,
    YearMonthDay,
    YearMonth,
    Year,
    MonthDay,
    Month,
    Day,
}

pub struct DateFormat {
    pub kind: DateFormatKind,
    pub with_weekday: bool,
}

impl TryFrom<&SkeletonV1> for DateFormat {
    type Error = Error;
    fn try_from(skeleton: &SkeletonV1) -> Result<Self, Self::Error> {
        bitflags! {
            struct DateFlags: u8 {
                const ERA = 0b0000_0001;
                const YEAR = 0b0000_0010;
                const MONTH = 0b0000_0100;
                const DAY = 0b0000_1000;
                const ERA_YEAR_MONTH_DAY = Self::ERA.bits | Self::YEAR_MONTH_DAY.bits;
                const ERA_YEAR_MONTH = Self::ERA.bits | Self::YEAR_MONTH.bits;
                const ERA_YEAR = Self::ERA.bits | Self::YEAR.bits;
                const YEAR_MONTH_DAY = Self::YEAR_MONTH.bits | Self::DAY.bits;
                const YEAR_MONTH = Self::YEAR.bits | Self::MONTH.bits;
                const MONTH_DAY = Self::MONTH.bits | Self::DAY.bits;
            }
        }

        let mut flags = DateFlags::empty();
        let mut with_weekday = false;

        for symbol in skeleton.0.as_slice().iter().map(|field| field.symbol) {
            match symbol {
                FieldSymbol::Era => flags |= DateFlags::ERA,
                FieldSymbol::Year(_) => flags |= DateFlags::YEAR,
                FieldSymbol::Month(_) => flags |= DateFlags::MONTH,
                FieldSymbol::Day(_) => flags |= DateFlags::DAY,
                FieldSymbol::Weekday(_) => with_weekday = true,
                sym => return Err(Error::InvalidFieldSymbol(sym)),
            }
        }

        let kind = match flags {
            DateFlags::ERA_YEAR_MONTH_DAY => DateFormatKind::EraYearMonthDay,
            DateFlags::ERA_YEAR_MONTH => DateFormatKind::EraYearMonth,
            DateFlags::ERA_YEAR => DateFormatKind::EraYear,
            DateFlags::ERA => DateFormatKind::Era,
            DateFlags::YEAR_MONTH_DAY => DateFormatKind::YearMonthDay,
            DateFlags::YEAR_MONTH => DateFormatKind::YearMonth,
            DateFlags::YEAR => DateFormatKind::Year,
            DateFlags::MONTH_DAY => DateFormatKind::MonthDay,
            DateFlags::MONTH => DateFormatKind::Month,
            DateFlags::DAY => DateFormatKind::Day,
            _ => return Err(Error::MissingFieldSymbols),
        };

        Ok(Self { kind, with_weekday })
    }
}

pub enum TimeFormatKind {
    HourMinuteSecondFractionalSecond,
    HourMinuteSecond,
    HourMinute,
    Hour,
}

pub struct TimeFormat {
    pub kind: TimeFormatKind,
    pub with_weekday: bool,
}

impl TryFrom<&SkeletonV1> for TimeFormat {
    type Error = Error;

    fn try_from(skeleton: &SkeletonV1) -> Result<Self, Self::Error> {
        bitflags! {
            struct TimeFlags: u8 {
                const HOUR = 0b0000_0001;
                const MINUTE = 0b0000_0010;
                const SECOND = 0b0000_0100;
                const FRACTIONAL_SECOND = 0b0000_1000;
                const HOUR_MINUTE_SECOND_FRACTIONAL_SECOND = Self::HOUR_MINUTE_SECOND.bits | Self::FRACTIONAL_SECOND.bits;
                const HOUR_MINUTE_SECOND = Self::HOUR_MINUTE.bits | Self::SECOND.bits;
                const HOUR_MINUTE = Self::HOUR.bits | Self::MINUTE.bits;
            }
        }

        let mut flags = TimeFlags::empty();
        let mut with_weekday = false;

        for symbol in skeleton.0.as_slice().iter().map(|field| field.symbol) {
            match symbol {
                FieldSymbol::Hour(_) => flags |= TimeFlags::HOUR,
                FieldSymbol::Minute => flags |= TimeFlags::MINUTE,
                FieldSymbol::Second(_) => flags |= TimeFlags::SECOND,
                // TODO: CLDR doesn't define a fractional second symbol independent of second
                FieldSymbol::Weekday(_) => with_weekday = true,
                sym => return Err(Error::InvalidFieldSymbol(sym)),
            }
        }

        let kind = match flags {
            TimeFlags::HOUR_MINUTE_SECOND_FRACTIONAL_SECOND => {
                TimeFormatKind::HourMinuteSecondFractionalSecond
            }
            TimeFlags::HOUR_MINUTE_SECOND => TimeFormatKind::HourMinuteSecond,
            TimeFlags::HOUR_MINUTE => TimeFormatKind::HourMinute,
            TimeFlags::HOUR => TimeFormatKind::Hour,
            _ => return Err(Error::MissingFieldSymbols),
        };

        Ok(Self { kind, with_weekday })
    }
}
