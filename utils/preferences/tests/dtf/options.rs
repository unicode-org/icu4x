// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug, PartialEq)]
pub enum DayPeriod {
    Short,
}

impl Default for DayPeriod {
    fn default() -> Self {
        Self::Short
    }
}

#[derive(Debug, PartialEq)]
pub enum LocaleMatcher {
    BestFit,
}

impl Default for LocaleMatcher {
    fn default() -> Self {
        Self::BestFit
    }
}
