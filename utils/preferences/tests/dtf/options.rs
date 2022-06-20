#[derive(Debug)]
pub enum DayPeriod {
    Narrow,
    Short,
    Long,
}

impl Default for DayPeriod {
    fn default() -> Self {
        Self::Short
    }
}

#[derive(Debug)]
pub enum LocaleMatcher {
    BestFit,
}

impl Default for LocaleMatcher {
    fn default() -> Self {
        Self::BestFit
    }
}
