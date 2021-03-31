// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for FixedDecimalFormat

/// `FixedDecimalFormatOptions` is a bag of options which, together with `Locale`,
/// defines how numbers will be formatted with a `FixedDecimalFormat` instance.
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FixedDecimalFormatOptions {
    pub grouping_strategy: GroupingStrategy,
    pub sign_display: SignDisplay,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum GroupingStrategy {
    Auto,
    Never,
    Always,
    Min2,
}

impl Default for GroupingStrategy {
    fn default() -> Self {
        Self::Auto
    }
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SignDisplay {
    Auto,
    Never,
    Always,
    ExceptZero,
    Negative,
}

impl Default for SignDisplay {
    fn default() -> Self {
        Self::Auto
    }
}
