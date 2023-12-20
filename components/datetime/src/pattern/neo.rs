// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use super::{runtime, PatternError};
use crate::provider::neo::*;
use icu_provider::prelude::*;

#[derive(Debug)]
enum CustomDateTimePatternInner {
    Custom(runtime::Pattern<'static>),
    Date(DataPayload<ErasedDatePatternV1Marker>),
    Time(DataPayload<TimePatternV1Marker>),
    DateTime {
        date_pattern: DataPayload<ErasedDatePatternV1Marker>,
        time_pattern: DataPayload<TimePatternV1Marker>,
        glue_pattern: DataPayload<DateTimePatternV1Marker>,
    },
}

#[derive(Debug)]
pub struct CustomDateTimePattern {
    inner: CustomDateTimePatternInner,
}

impl CustomDateTimePattern {
    pub fn try_from_pattern_str(pattern_str: &str) -> Result<Self, PatternError> {
        let pattern = runtime::Pattern::from_str(pattern_str)?;
        Ok(Self {
            inner: CustomDateTimePatternInner::Custom(pattern),
        })
    }

    #[doc(hidden)] // TODO(#4467): Internal API
    pub fn from_runtime_pattern(pattern: runtime::Pattern<'static>) -> Self {
        Self {
            inner: CustomDateTimePatternInner::Custom(pattern),
        }
    }

    pub(crate) fn as_borrowed(&self) -> CustomDateTimePatternBorrowed {
        let borrowed_inner = match self.inner {
            CustomDateTimePatternInner::Custom(ref pattern) => {
                CustomDateTimePatternBorrowedInner::Single(pattern)
            }
            CustomDateTimePatternInner::Date(ref payload) => {
                CustomDateTimePatternBorrowedInner::Single(&payload.get().pattern)
            }
            CustomDateTimePatternInner::Time(ref payload) => {
                CustomDateTimePatternBorrowedInner::Single(&payload.get().pattern)
            }
            CustomDateTimePatternInner::DateTime {
                ref date_pattern,
                ref time_pattern,
                ref glue_pattern,
            } => CustomDateTimePatternBorrowedInner::DateTime {
                date_pattern: &date_pattern.get().pattern,
                time_pattern: &time_pattern.get().pattern,
                glue_pattern: &glue_pattern.get().pattern,
            },
        };
        CustomDateTimePatternBorrowed {
            inner: borrowed_inner,
        }
    }
}

impl FromStr for CustomDateTimePattern {
    type Err = PatternError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_pattern_str(s)
    }
}

// Check equality on the borrowed variant since it flattens the difference
// between the three `Single`` pattern variants, which is not public-facing
impl PartialEq for CustomDateTimePattern {
    fn eq(&self, other: &Self) -> bool {
        self.as_borrowed().eq(&other.as_borrowed())
    }
}

impl Eq for CustomDateTimePattern {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CustomDateTimePatternBorrowedInner<'a> {
    Single(&'a runtime::Pattern<'a>),
    DateTime {
        date_pattern: &'a runtime::Pattern<'a>,
        time_pattern: &'a runtime::Pattern<'a>,
        glue_pattern: &'a runtime::GenericPattern<'a>,
    },
}

// Not clear if this needs to be public. For now, make it private.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct CustomDateTimePatternBorrowed<'a> {
    inner: CustomDateTimePatternBorrowedInner<'a>,
}
