// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;
use core::str::FromStr;

use super::{runtime, PatternError};
use crate::options::length;
use crate::provider::neo::*;
use crate::CldrCalendar;
use crate::Error;
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
pub struct CustomTypedDateTimePattern<C: CldrCalendar> {
    inner: CustomDateTimePatternInner,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> CustomTypedDateTimePattern<C> {
    pub fn try_from_pattern_str(pattern_str: &str) -> Result<Self, PatternError> {
        let pattern = runtime::Pattern::from_str(pattern_str)?;
        Ok(Self {
            inner: CustomDateTimePatternInner::Custom(pattern),
            _calendar: PhantomData,
        })
    }

    /// Loads a [`TypedDateTimePattern`] for a date length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::pattern::neo::TypedDateTimePattern;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    ///
    /// let expected_pattern = TypedDateTimePattern::<Gregorian>::try_from_pattern_str("d MMM y").unwrap();
    /// let actual_pattern = TypedDateTimePattern::<Gregorian>::try_new_date_with_length_unstable(
    ///     &icu::datetime::provider::Baked,
    ///     &locale!("fr").into(),
    ///     length::Date::Medium,
    /// ).unwrap();
    ///
    /// assert_eq!(expected_pattern, actual_pattern);
    /// ```
    pub fn try_new_date_with_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, Error>
    where
        P: DataProvider<C::DatePatternV1Marker> + ?Sized,
    {
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::pattern_subtag_for(
            match length {
                length::Date::Full => aux::PatternLength::Full,
                length::Date::Long => aux::PatternLength::Long,
                length::Date::Medium => aux::PatternLength::Medium,
                length::Date::Short => aux::PatternLength::Short,
            },
            None, // no hour cycle for date patterns
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self {
            inner: CustomDateTimePatternInner::Date(payload.cast()),
            _calendar: PhantomData,
        })
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

impl<C: CldrCalendar> FromStr for CustomTypedDateTimePattern<C> {
    type Err = PatternError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_pattern_str(s)
    }
}

// Check equality on the borrowed variant since it flattens the difference
// between the three `Single`` pattern variants, which is not public-facing
impl<C: CldrCalendar> PartialEq for CustomTypedDateTimePattern<C> {
    fn eq(&self, other: &Self) -> bool {
        self.as_borrowed().eq(&other.as_borrowed())
    }
}

impl<C: CldrCalendar> Eq for CustomTypedDateTimePattern<C> {}

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
