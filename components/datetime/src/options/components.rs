use crate::fields::{self, Field, FieldLength, FieldSymbol, FieldType};
use std::fmt;

#[derive(Debug)]
pub struct Bag {
    pub era: Text,
    pub year: Numeric,
    pub month: Month,
    pub day: Numeric,
    pub weekday: Text,

    pub hour: Numeric,
    pub minute: Numeric,
    pub second: Numeric,
    pub hour_cycle: HourCycle,

    pub time_zone_name: TimeZoneName,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            era: Text::default(),
            year: Numeric::Numeric,
            month: Month::Long,
            day: Numeric::Numeric,
            weekday: Text::default(),

            hour: Numeric::Numeric,
            minute: Numeric::Numeric,
            second: Numeric::Numeric,
            hour_cycle: HourCycle::default(),

            time_zone_name: TimeZoneName::default(),
        }
    }
}

impl Bag {
    pub fn write_skeleton(&self, w: &mut impl fmt::Write) -> fmt::Result {
        for field in self.skeleton() {
            field.write_pattern(w)?;
        }
        Ok(())
    }

    fn get(&self, ft: &FieldType) -> Option<Field> {
        match ft {
            FieldType::Era => self.era.get_field(FieldSymbol::Era),
            FieldType::Year => self
                .year
                .get_field(FieldSymbol::Year(fields::Year::Calendar)),
            FieldType::Month => self
                .month
                .get_field(FieldSymbol::Month(fields::Month::Format)),
            FieldType::Day => self
                .day
                .get_field(FieldSymbol::Day(fields::Day::DayOfMonth)),
            FieldType::Hour => self
                .hour
                .get_field(FieldSymbol::Hour(self.hour_cycle.field())),
        }
    }

    pub fn skeleton(&self) -> impl Iterator<Item = Field> + '_ {
        FieldType::iter().filter_map(move |field_type| self.get(field_type))
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_skeleton(f)
    }
}

pub trait ComponentType {
    fn get_length(&self) -> Option<FieldLength>;

    fn get_field(&self, symbol: FieldSymbol) -> Option<Field> {
        self.get_length().map(|length| Field { symbol, length })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HourCycle {
    H24,
    H23,
    H12,
    H11,
    None,
}

impl Default for HourCycle {
    fn default() -> Self {
        Self::None
    }
}

impl HourCycle {
    pub fn field(&self) -> fields::Hour {
        match self {
            Self::H11 => fields::Hour::H11,
            Self::H12 => fields::Hour::H12,
            Self::H23 => fields::Hour::H23,
            Self::H24 => fields::Hour::H24,
            Self::None => fields::Hour::Preferred,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Numeric {
    Numeric,
    TwoDigit,
    None,
}

impl Default for Numeric {
    fn default() -> Self {
        Self::None
    }
}

impl ComponentType for Numeric {
    fn get_length(&self) -> Option<FieldLength> {
        match self {
            Self::Numeric => Some(FieldLength::One),
            Self::TwoDigit => Some(FieldLength::TwoDigit),
            Self::None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Text {
    Long,
    Short,
    Narrow,
    None,
}

impl Default for Text {
    fn default() -> Self {
        Self::None
    }
}

impl ComponentType for Text {
    fn get_length(&self) -> Option<FieldLength> {
        match self {
            Self::Short => Some(FieldLength::Six),
            Self::Long => Some(FieldLength::Wide),
            Self::Narrow => Some(FieldLength::Narrow),
            Self::None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Month {
    Numeric,
    TwoDigit,
    Long,
    Short,
    Narrow,
    None,
}

impl Default for Month {
    fn default() -> Self {
        Self::None
    }
}

impl ComponentType for Month {
    fn get_length(&self) -> Option<FieldLength> {
        match self {
            Self::Numeric => Some(FieldLength::One),
            Self::TwoDigit => Some(FieldLength::TwoDigit),
            Self::Long => Some(FieldLength::Wide),
            Self::Short => Some(FieldLength::Abbreviated),
            Self::Narrow => Some(FieldLength::Narrow),
            Self::None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeZoneName {
    Long,
    Short,
    None,
}

impl Default for TimeZoneName {
    fn default() -> Self {
        Self::None
    }
}

impl ComponentType for TimeZoneName {
    fn get_length(&self) -> Option<FieldLength> {
        match self {
            Self::Short => Some(FieldLength::One),
            Self::Long => Some(FieldLength::Wide),
            Self::None => None,
        }
    }
}
