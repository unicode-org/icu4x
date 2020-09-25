use super::preferences;

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

    pub time_zone_name: TimeZoneName,

    pub preferences: Option<preferences::Bag>,
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

            time_zone_name: TimeZoneName::default(),

            preferences: None,
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
