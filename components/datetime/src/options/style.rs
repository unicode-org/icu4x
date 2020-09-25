use super::preferences;

#[derive(Debug)]
pub struct Bag {
    pub date: Date,
    pub time: Time,
    pub preferences: Option<preferences::Bag>,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            date: Date::Long,
            time: Time::Long,
            preferences: None,
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
