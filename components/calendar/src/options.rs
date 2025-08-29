// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options used by types in this crate

/// Options bag for [`Date::try_from_fields`](crate::Date::try_from_fields).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct DateFromFieldsOptions {
    /// How to behave with out-of-bounds fields.
    pub overflow: Overflow,
}

/// Whether to constrain or reject out-of-bounds values when constructing a Date.
///
/// The behavior conforms to the ECMAScript Temporal specification.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum Overflow {
    /// Constrain out-of-bounds fields to the nearest in-bounds value.
    ///
    /// Only the out-of-bounds field is constrained. If the other fields are not themselves
    /// out of bounds, they are not changed.
    ///
    /// This is the [default option](
    /// https://tc39.es/proposal-temporal/#sec-temporal-gettemporaloverflowoption),
    /// following the ECMAScript Temporal specification. See also the [docs on MDN](
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate#invalid_date_clamping).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::DateError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::options::DateFromFieldsOptions;
    /// use icu_calendar::options::Overflow;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use std::num::NonZeroU8;
    /// use tinystr::tinystr;
    ///
    /// let mut options = DateFromFieldsOptions::default();
    /// options.overflow = Overflow::Constrain;
    ///
    /// // 5784, a leap year, contains M05L, but the day is too big.
    /// let mut fields = DateFields::default();
    /// fields.monotonic_year = Some(5784);
    /// fields.month_code = Some(MonthCode(tinystr!(4, "M05L")));
    /// fields.day = NonZeroU8::new(50);
    ///
    /// let date = Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// )
    /// .unwrap();
    ///
    /// // Constrained to the 20th day of M05L of year 5784
    /// assert_eq!(date.year().monotonic_year(), 5784);
    /// assert_eq!(date.month().standard_code, MonthCode(tinystr!(4, "M05L")));
    /// assert_eq!(date.day_of_month().0, 20);
    ///
    /// // 5785, a common year, does not contain M05L.
    /// fields.monotonic_year = Some(5785);
    ///
    /// let date = Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// )
    /// .unwrap();
    ///
    /// // Constrained to the 20th day of M06 of year 5785
    /// assert_eq!(date.year().monotonic_year(), 5785);
    /// assert_eq!(date.month().standard_code, MonthCode(tinystr!(4, "M06")));
    /// assert_eq!(date.day_of_month().0, 20);
    /// ```
    #[default]
    Constrain,
    /// Return an error if any fields are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::Date;
    /// use icu_calendar::DateError;
    /// use icu_calendar::cal::Hebrew;
    /// use icu_calendar::options::DateFromFieldsOptions;
    /// use icu_calendar::options::Overflow;
    /// use icu_calendar::types::DateFields;
    /// use icu_calendar::types::MonthCode;
    /// use std::num::NonZeroU8;
    /// use tinystr::tinystr;
    ///
    /// let mut options = DateFromFieldsOptions::default();
    /// options.overflow = Overflow::Reject;
    ///
    /// // 5784, a leap year, contains M05L, but the day is too big.
    /// let mut fields = DateFields::default();
    /// fields.monotonic_year = Some(5784);
    /// fields.month_code = Some(MonthCode(tinystr!(4, "M05L")));
    /// fields.day = NonZeroU8::new(50);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// )
    /// .expect_err("Day is out of bounds");
    /// assert!(matches!(err, DateError::Range { .. }));
    ///
    /// // Set the day to one that exists
    /// fields.day = NonZeroU8::new(1);
    /// Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// ).expect("A valid Hebrew date");
    ///
    /// // 5785, a common year, does not contain M05L.
    /// fields.monotonic_year = Some(5785);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// )
    /// .expect_err("Month is out of bounds");
    /// assert!(matches!(err, DateError::Range { .. }));
    /// ```
    Reject,
}
