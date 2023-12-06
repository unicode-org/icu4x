// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{marker::PhantomData, str::FromStr};

use super::{runtime, PatternError};
use crate::options::length;
use crate::Error;
use crate::{provider::neo::*, CldrCalendar};
use icu_provider::prelude::*;

enum DateTimePatternInner {
    Custom(runtime::Pattern<'static>),
    Date(DataPayload<ErasedDatePatternV1Marker>),
    Time(DataPayload<ErasedTimePatternV1Marker>),
    DateTime {
        date_pattern: DataPayload<ErasedDatePatternV1Marker>,
        time_pattern: DataPayload<ErasedTimePatternV1Marker>,
        glue_pattern: DataPayload<DateTimePatternV1Marker>,
    },
}

pub struct TypedDateTimePattern<C: CldrCalendar> {
    inner: DateTimePatternInner,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> TypedDateTimePattern<C> {
    pub fn try_from_pattern_str(pattern_str: &str) -> Result<Self, PatternError> {
        let pattern = runtime::Pattern::from_str(pattern_str)?;
        Ok(Self {
            inner: DateTimePatternInner::Custom(pattern),
            _calendar: PhantomData,
        })
    }

    pub(crate) fn as_borrowed(&self) -> DateTimePatternBorrowed {
        let borrowed_inner = match self.inner {
            DateTimePatternInner::Custom(ref pattern) => {
                DateTimePatternBorrowedInner::Single(pattern)
            }
            DateTimePatternInner::Date(ref payload) => {
                DateTimePatternBorrowedInner::Single(&payload.get().pattern)
            }
            DateTimePatternInner::Time(ref payload) => {
                DateTimePatternBorrowedInner::Single(&payload.get().pattern)
            }
            DateTimePatternInner::DateTime {
                ref date_pattern,
                ref time_pattern,
                ref glue_pattern,
            } => DateTimePatternBorrowedInner::DateTime {
                date_pattern: &date_pattern.get().pattern,
                time_pattern: &time_pattern.get().pattern,
                glue_pattern: &glue_pattern.get().pattern,
            },
        };
        DateTimePatternBorrowed {
            inner: borrowed_inner,
        }
    }
}

impl<C: CldrCalendar> FromStr for TypedDateTimePattern<C> {
    type Err = PatternError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_pattern_str(s)
    }
}

#[derive(Debug, Copy, Clone)]
enum DateTimePatternBorrowedInner<'a> {
    Single(&'a runtime::Pattern<'a>),
    DateTime {
        date_pattern: &'a runtime::Pattern<'a>,
        time_pattern: &'a runtime::Pattern<'a>,
        glue_pattern: &'a runtime::GenericPattern<'a>,
    },
}

// Not clear if this needs to be public. For now, make it private.
#[derive(Debug, Copy, Clone)]
pub(crate) struct DateTimePatternBorrowed<'a> {
    inner: DateTimePatternBorrowedInner<'a>,
}
