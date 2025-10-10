// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{options::Overflow, types::MonthCode};
use displaydoc::Display;

#[derive(Debug, Copy, Clone, PartialEq, Display)]
/// Error type for date creation.
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
    /// The year was specified in multiple inconsistent ways.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::DateError;
    /// use icu_calendar::cal::Japanese;
    /// use icu_calendar::types::DateFields;
    /// use std::num::NonZeroU8;
    ///
    /// let mut fields = DateFields::default();
    /// fields.era = Some("reiwa");
    /// fields.era_year = Some(6);
    /// fields.ordinal_month = NonZeroU8::new(1);
    /// fields.day = NonZeroU8::new(1);
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
    /// assert!(matches!(err, DateError::InconsistentYear));
    /// ```
    #[displaydoc("Inconsistent year")]
    InconsistentYear,
    /// The month was specified in multiple inconsistent ways.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::DateError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use std::num::NonZeroU8;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(5783);
    /// fields.month_code = Some(MonthCode(tinystr!(4, "M06")));
    /// fields.ordinal_month = NonZeroU8::new(6);
    /// fields.day = NonZeroU8::new(1);
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
    /// assert!(matches!(err, DateError::InconsistentMonth));
    /// ```
    #[displaydoc("Inconsistent month")]
    InconsistentMonth,
    /// Not enough fields were specified to form a well-defined date.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::DateError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use std::num::NonZeroU8;
    /// use tinystr::tinystr;
    ///
    /// let mut fields = DateFields::default();
    ///
    /// fields.ordinal_month = NonZeroU8::new(3);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("need more than just an ordinal month");
    /// assert!(matches!(err, DateError::NotEnoughFields));
    ///
    /// fields.era_year = Some(5783);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("need more than an ordinal month and an era year");
    /// assert!(matches!(err, DateError::NotEnoughFields));
    ///
    /// fields.extended_year = Some(5783);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("era year still needs an era");
    /// assert!(matches!(err, DateError::NotEnoughFields));
    ///
    /// fields.era = Some("am");
    ///
    /// let date = Date::try_from_fields(
    ///     fields,
    ///     Default::default(),
    ///     Hebrew
    /// )
    /// .expect_err("still missing the day");
    /// assert!(matches!(err, DateError::NotEnoughFields));
    ///
    /// fields.day = NonZeroU8::new(1);
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

impl core::error::Error for DateError {}

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

impl RangeError {
    #[inline]
    pub(crate) fn maybe_with_month_code(self, month_code: Option<MonthCode>) -> DateError {
        if let Some(month_code) = month_code {
            if self.field == "month" {
                return DateError::UnknownMonthCode(month_code);
            }
        }
        self.into()
    }
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
