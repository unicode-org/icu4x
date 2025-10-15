// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for functions in the calendar crate

use crate::{options::Overflow, types::MonthCode};
use displaydoc::Display;

#[derive(Debug, Copy, Clone, PartialEq, Display)]
/// Error type for date creation via [`Date::try_new_from_codes`].
///
/// [`Date::try_new_from_codes`]: crate::Date::try_new_from_codes
#[non_exhaustive]
pub enum DateError {
    /// A field is out of range for its domain.
    #[displaydoc("The {field} = {value} argument is out of range {min}..={max}")]
    Range {
        /// The field that is out of range, such as "year"
        field: &'static str,
        /// The actual value
        value: i32,
        /// The minimum value (inclusive). This might not be tight.
        min: i32,
        /// The maximum value (inclusive). This might not be tight.
        max: i32,
    },
    /// The era code is invalid for the calendar.
    #[displaydoc("Unknown era")]
    UnknownEra,
    /// The month code is invalid for the calendar or year.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::DateError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::MonthCode;
    /// use tinystr::tinystr;
    ///
    /// let month_code = MonthCode(tinystr!(4, "M05L"));
    ///
    /// Date::try_new_from_codes(
    ///     None,
    ///     5784,
    ///     month_code,
    ///     1,
    ///     Hebrew
    /// )
    /// .expect("5784 is a leap year");
    ///
    /// let err = Date::try_new_from_codes(
    ///     None,
    ///     5785,
    ///     month_code,
    ///     1,
    ///     Hebrew
    /// )
    /// .expect_err("5785 is a common year");
    ///
    /// assert!(matches!(err, DateError::UnknownMonthCode(_)));
    /// ```
    #[displaydoc("Unknown month code {0:?}")]
    UnknownMonthCode(MonthCode),
}

impl core::error::Error for DateError {}

/// Error type for date creation via [`Date::try_from_fields`].
///
/// [`Date::try_from_fields`]: crate::Date::try_from_fields
#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum DateFromFieldsError {
    /// A field is out of range for its domain.
    #[displaydoc("{0}")]
    Range(RangeError),
    /// The era code is invalid for the calendar.
    #[displaydoc("Unknown era or invalid syntax")]
    UnknownEra,
    /// The month code syntax is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::error::DateFromFieldsError;
    /// use icu_calendar::Iso;
    /// use icu_calendar::types::MonthCode;
    /// use icu_calendar::types::DateFields;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(2000);
    /// fields.month_code = Some(MonthCode(tinystr!(4, "????")));
    /// fields.day = Some(1);
    ///
    /// let err = Date::try_from_fields(
    ///     fields, Default::default(), Iso
    /// )
    /// .expect_err("month code is invalid");
    ///
    /// assert!(matches!(err, DateFromFieldsError::InvalidMonthCode));
    /// ```
    #[displaydoc("Invalid month code syntax")]
    InvalidMonthCode,
    /// The specified month code does not exist in this calendar.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::error::DateFromFieldsError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(5783);
    /// fields.month_code = Some(MonthCode::new_normal(13).unwrap());
    /// fields.day = Some(1);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("no month M13 in Hebrew");
    /// assert!(matches!(err, DateFromFieldsError::UnknownMonthCodeForCalendar));
    /// ```
    #[displaydoc("The specified month code does not exist in this calendar")]
    UnknownMonthCodeForCalendar,
    /// The specified month code exists in this calendar, but not in the specified year.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::error::DateFromFieldsError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(5783);
    /// fields.month_code = Some(MonthCode::new_leap(5).unwrap());
    /// fields.day = Some(1);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("no month M05L in Hebrew year 5783");
    /// assert!(matches!(err, DateFromFieldsError::UnknownMonthCodeForYear));
    /// ```
    #[displaydoc("The specified month code exists in calendar, but not for this year")]
    UnknownMonthCodeForYear,
    /// The year was specified in multiple inconsistent ways.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::error::DateFromFieldsError;
    /// use icu_calendar::cal::Japanese;
    /// use icu_calendar::types::DateFields;
    ///
    /// let mut fields = DateFields::default();
    /// fields.era = Some("reiwa");
    /// fields.era_year = Some(6);
    /// fields.ordinal_month = Some(1);
    /// fields.day = Some(1);
    ///
    /// Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Japanese::new()
    /// )
    /// .expect("a well-defined Japanese date");
    ///
    /// fields.extended_year = Some(1900);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Japanese::new()
    /// )
    /// .expect_err("year 1900 is not the same as 6 Reiwa");
    ///
    /// assert!(matches!(err, DateFromFieldsError::InconsistentYear));
    /// ```
    #[displaydoc("Inconsistent year")]
    InconsistentYear,
    /// The month was specified in multiple inconsistent ways.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::error::DateFromFieldsError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(5783);
    /// fields.month_code = Some(MonthCode(tinystr!(4, "M06")));
    /// fields.ordinal_month = Some(6);
    /// fields.day = Some(1);
    ///
    /// Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect("a well-defined Hebrew date in a common year");
    ///
    /// fields.extended_year = Some(5784);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("month M06 is not the 6th month in leap year 5784");
    ///
    /// assert!(matches!(err, DateFromFieldsError::InconsistentMonth));
    /// ```
    #[displaydoc("Inconsistent month")]
    InconsistentMonth,
    /// Not enough fields were specified to form a well-defined date.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::error::DateFromFieldsError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    ///
    /// fields.ordinal_month = Some(3);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("need more than just an ordinal month");
    /// assert!(matches!(err, DateFromFieldsError::NotEnoughFields));
    ///
    /// fields.era_year = Some(5783);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("need more than an ordinal month and an era year");
    /// assert!(matches!(err, DateFromFieldsError::NotEnoughFields));
    ///
    /// fields.extended_year = Some(5783);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("era year still needs an era");
    /// assert!(matches!(err, DateFromFieldsError::NotEnoughFields));
    ///
    /// fields.era = Some("am");
    ///
    /// let date = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("still missing the day");
    /// assert!(matches!(err, DateFromFieldsError::NotEnoughFields));
    ///
    /// fields.day = Some(1);
    /// let date = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect("we have enough fields!");
    /// ```
    #[displaydoc("Not enough fields")]
    NotEnoughFields,
}

impl core::error::Error for DateFromFieldsError {}

impl From<RangeError> for DateFromFieldsError {
    #[inline]
    fn from(value: RangeError) -> Self {
        DateFromFieldsError::Range(value)
    }
}

/// Internal narrow error type for functions that only fail on unknown eras
pub(crate) struct UnknownEraError;

impl From<UnknownEraError> for DateError {
    #[inline]
    fn from(_value: UnknownEraError) -> Self {
        DateError::UnknownEra
    }
}

impl From<UnknownEraError> for DateFromFieldsError {
    #[inline]
    fn from(_value: UnknownEraError) -> Self {
        DateFromFieldsError::UnknownEra
    }
}

/// An error when validating a month code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
#[non_exhaustive]
pub enum MonthCodeParseError {
    /// The month code had invalid syntax.
    #[displaydoc("The month code had invalid syntax")]
    InvalidSyntax,
}

impl From<MonthCodeParseError> for DateFromFieldsError {
    #[inline]
    fn from(value: MonthCodeParseError) -> Self {
        match value {
            MonthCodeParseError::InvalidSyntax => DateFromFieldsError::InvalidMonthCode,
        }
    }
}

impl From<MonthCodeParseError> for EcmaReferenceYearError {
    #[inline]
    fn from(value: MonthCodeParseError) -> Self {
        match value {
            MonthCodeParseError::InvalidSyntax => EcmaReferenceYearError::InvalidMonthCode,
        }
    }
}

impl From<MonthCodeParseError> for MonthCodeError {
    #[inline]
    fn from(value: MonthCodeParseError) -> Self {
        match value {
            MonthCodeParseError::InvalidSyntax => MonthCodeError::InvalidMonthCode,
        }
    }
}

/// Internal narrow error type for functions that only fail on month code operations
pub(crate) enum MonthCodeError {
    InvalidMonthCode,
    UnknownMonthCodeForCalendar,
    UnknownMonthCodeForYear,
}

impl From<MonthCodeError> for DateFromFieldsError {
    #[inline]
    fn from(value: MonthCodeError) -> Self {
        match value {
            MonthCodeError::InvalidMonthCode => DateFromFieldsError::InvalidMonthCode,
            MonthCodeError::UnknownMonthCodeForCalendar => {
                DateFromFieldsError::UnknownMonthCodeForCalendar
            }
            MonthCodeError::UnknownMonthCodeForYear => DateFromFieldsError::UnknownMonthCodeForYear,
        }
    }
}

mod inner {
    /// Internal narrow error type for calculating the ECMA reference year
    ///
    /// Public but unstable because it is used on hijri::Rules
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[allow(missing_docs)] // TODO: fix when graduating
    #[non_exhaustive]
    pub enum EcmaReferenceYearError {
        InvalidMonthCode,
        UnknownMonthCodeForCalendar,
        NotEnoughFields,
    }
}

#[cfg(feature = "unstable")]
pub use inner::EcmaReferenceYearError;
#[cfg(not(feature = "unstable"))]
pub(crate) use inner::EcmaReferenceYearError;

impl From<EcmaReferenceYearError> for DateFromFieldsError {
    #[inline]
    fn from(value: EcmaReferenceYearError) -> Self {
        match value {
            EcmaReferenceYearError::InvalidMonthCode => DateFromFieldsError::InvalidMonthCode,
            EcmaReferenceYearError::UnknownMonthCodeForCalendar => {
                DateFromFieldsError::UnknownMonthCodeForCalendar
            }
            EcmaReferenceYearError::NotEnoughFields => DateFromFieldsError::NotEnoughFields,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Display)]
/// An argument is out of range for its domain.
#[displaydoc("The {field} = {value} argument is out of range {min}..={max}")]
#[allow(clippy::exhaustive_structs)]
pub struct RangeError {
    /// The argument that is out of range, such as "year"
    pub field: &'static str,
    /// The actual value
    pub value: i32,
    /// The minimum value (inclusive). This might not be tight.
    pub min: i32,
    /// The maximum value (inclusive). This might not be tight.
    pub max: i32,
}

impl core::error::Error for RangeError {}

impl From<RangeError> for DateError {
    #[inline]
    fn from(value: RangeError) -> Self {
        let RangeError {
            field,
            value,
            min,
            max,
        } = value;
        DateError::Range {
            field,
            value,
            min,
            max,
        }
    }
}

pub(crate) fn range_check_with_overflow<T: Ord + Into<i32> + Copy>(
    value: T,
    field: &'static str,
    bounds: core::ops::RangeInclusive<T>,
    overflow: Overflow,
) -> Result<T, RangeError> {
    if matches!(overflow, Overflow::Constrain) {
        Ok(value.clamp(*bounds.start(), *bounds.end()))
    } else {
        range_check(value, field, bounds)?;

        Ok(value)
    }
}

pub(crate) fn range_check<T: Ord + Into<i32> + Copy>(
    value: T,
    field: &'static str,
    bounds: core::ops::RangeInclusive<T>,
) -> Result<(), RangeError> {
    if !bounds.contains(&value) {
        return Err(RangeError {
            field,
            value: value.into(),
            min: (*bounds.start()).into(),
            max: (*bounds.end()).into(),
        });
    }
    Ok(())
}
