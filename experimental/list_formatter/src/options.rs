// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Represents the type of a list. See
/// https://unicode.org/reports/tr35/tr35-general.html#ListPatterns
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    /// A conjunction
    And,
    /// A disjunction
    Or,
    /// A list of units
    Unit,
}

/// Represents the type of a list. See
/// https://unicode.org/reports/tr35/tr35-general.html#ListPatterns
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Width {
    /// A typical list
    Wide,
    /// A shorter list
    Short,
    /// The shortest type of list
    Narrow,
}

impl Default for Type {
    fn default() -> Self {
        Self::And
    }
}

impl Default for Width {
    fn default() -> Self {
        Self::Wide
    }
}
