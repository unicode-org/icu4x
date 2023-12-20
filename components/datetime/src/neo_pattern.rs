// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::pattern::{runtime, PatternError};

#[derive(Debug)]
pub struct DateTimePattern {
    pattern: runtime::Pattern<'static>,
}

impl DateTimePattern {
    pub fn try_from_pattern_str(pattern_str: &str) -> Result<Self, PatternError> {
        let pattern = runtime::Pattern::from_str(pattern_str)?;
        Ok(Self {
            pattern
        })
    }

    #[doc(hidden)] // TODO(#4467): Internal API
    pub fn from_runtime_pattern(pattern: runtime::Pattern<'static>) -> Self {
        Self {
            pattern
        }
    }

    pub(crate) fn as_borrowed(&self) -> CustomDateTimePatternBorrowed {
        CustomDateTimePatternBorrowed {
            pattern: &self.pattern
        }
    }
}

impl FromStr for DateTimePattern {
    type Err = PatternError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_pattern_str(s)
    }
}

// Check equality on the borrowed variant since it flattens the difference
// between the three `Single`` pattern variants, which is not public-facing
impl PartialEq for DateTimePattern {
    fn eq(&self, other: &Self) -> bool {
        self.as_borrowed().eq(&other.as_borrowed())
    }
}

impl Eq for DateTimePattern {}

// Not clear if this needs to be public. For now, make it private.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct CustomDateTimePatternBorrowed<'a> {
    pattern: &'a runtime::Pattern<'a>
}
