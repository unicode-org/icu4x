use crate::fields;

#[derive(Debug)]
pub struct Bag {
    pub hour_cycle: HourCycle,
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
