use super::components;

#[derive(Debug)]
pub struct Bag {
    pub date: Date,
    pub time: Time,
    pub hour_cycle: components::HourCycle,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            date: Date::Long,
            time: Time::Long,
            hour_cycle: components::HourCycle::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Date {
    Full,
    Long,
    Medium,
    Short,
    None,
}

impl Default for Date {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Full,
    Long,
    Medium,
    Short,
    None,
}

impl Default for Time {
    fn default() -> Self {
        Self::None
    }
}
