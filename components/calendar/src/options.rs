// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options used by types in this crate

/// Options bag for [`Date::try_from_fields`](crate::Date::try_from_fields).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct DateFromFieldsOptions {
    /// How to behave with out-of-bounds fields.
    pub overflow: Option<Overflow>,
    /// How to behave when the fields that are present do not fully constitute a Date.
    pub missing_fields_strategy: Option<MissingFieldsStrategy>,
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
    /// options.overflow = Some(Overflow::Constrain);
    ///
    /// // 5784, a leap year, contains M05L, but the day is too big.
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(5784);
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
    /// // Constrained to the 30th day of M05L of year 5784
    /// assert_eq!(date.year().extended_year(), 5784);
    /// assert_eq!(date.month().standard_code, MonthCode(tinystr!(4, "M05L")));
    /// assert_eq!(date.day_of_month().0, 30);
    ///
    /// // 5785, a common year, does not contain M05L.
    /// fields.extended_year = Some(5785);
    ///
    /// let date = Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// )
    /// .unwrap();
    ///
    /// // Constrained to the 29th day of M06 of year 5785
    /// assert_eq!(date.year().extended_year(), 5785);
    /// assert_eq!(date.month().standard_code, MonthCode(tinystr!(4, "M06")));
    /// assert_eq!(date.day_of_month().0, 29);
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
    /// options.overflow = Some(Overflow::Reject);
    ///
    /// // 5784, a leap year, contains M05L, but the day is too big.
    /// let mut fields = DateFields::default();
    /// fields.extended_year = Some(5784);
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
    /// fields.extended_year = Some(5785);
    ///
    /// let err = Date::try_from_fields(
    ///     fields,
    ///     options,
    ///     Hebrew
    /// )
    /// .expect_err("Month is out of bounds");
    /// assert!(matches!(err, DateError::UnknownMonthCode(_)));
    /// ```
    Reject,
}

/// How to infer missing fields when the fields that are present do not fully constitute a Date.
///
/// In order for the fields to fully constitute a Date, they must identify a year, a month,
/// and a day. The fields `era`, `era_year`, and `extended_year` identify the year:
///
/// | Era? | Era Year? | Extended Year? | Outcome |
/// |------|-----------|----------------|---------|
/// | -    | -         | -              | Error |
/// | Some | -         | -              | Error |
/// | -    | Some      | -              | Error |
/// | -    | -         | Some           | OK |
/// | Some | Some      | -              | OK |
/// | Some | -         | Some           | Error (era requires era year) |
/// | -    | Some      | Some           | Error (era year requires era) |
/// | Some | Some      | Some           | OK (but error if inconsistent) |
///
/// The fields `month_code` and `ordinal_month` identify the month:
///
/// | Month Code? | Ordinal Month? | Outcome |
/// |-------------|----------------|---------|
/// | -           | -              | Error |
/// | Some        | -              | OK |
/// | -           | Some           | OK |
/// | Some        | Some           | OK (but error if inconsistent) |
///
/// The field `day` identifies the day.
///
/// If the fields identify a year, a month, and a day, then there are no missing fields, so
/// the strategy chosen here has no effect (fields that are out-of-bounds or inconsistent
/// are handled by other errors).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum MissingFieldsStrategy {
    /// If the fields that are present do not fully constitute a Date,
    /// return [`DateError::NotEnoughFields`].
    ///
    /// [`DateError::NotEnoughFields`]: crate::DateError::NotEnoughFields
    #[default]
    Reject,
    /// If the fields that are present do not fully constitute a Date,
    /// follow the ECMAScript specification when possible.
    ///
    /// ⚠️ This option causes a year or day to be implicitly added to the Date!
    ///
    /// This strategy makes the following changes:
    ///
    /// 1. If the fields identify a year and a month, but not a day, then set `day` to 1.
    /// 2. If `month_code` and `day` are set but nothing else, then set the year to a
    ///    _reference year_: some year in the calendar that contains the specified month
    ///    and day, according to the ECMAScript specification.
    ///
    /// Note that the reference year is _not_ added if an ordinal month is present, since
    /// the identity of an ordinal month changes from year to year.
    Ecma,
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{DateFields, MonthCode},
        Date, DateError, Gregorian,
    };
    use core::num::NonZeroU8;
    use itertools::Itertools;
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;

    #[test]
    #[allow(clippy::field_reassign_with_default)] // want out-of-crate code style
    fn test_missing_fields_strategy() {
        // The sets of fields that identify a year, according to the table in the docs
        let valid_year_field_sets = [
            &["era", "era_year"][..],
            &["extended_year"][..],
            &["era", "era_year", "extended_year"][..],
        ]
        .into_iter()
        .map(|field_names| field_names.iter().copied().collect())
        .collect::<Vec<BTreeSet<&str>>>();

        // The sets of fields that identify a month, according to the table in the docs
        let valid_month_field_sets = [
            &["month_code"][..],
            &["ordinal_month"][..],
            &["month_code", "ordinal_month"][..],
        ]
        .into_iter()
        .map(|field_names| field_names.iter().copied().collect())
        .collect::<Vec<BTreeSet<&str>>>();

        // The sets of fields that identify a day, according to the table in the docs
        let valid_day_field_sets = [&["day"][..]]
            .into_iter()
            .map(|field_names| field_names.iter().copied().collect())
            .collect::<Vec<BTreeSet<&str>>>();

        // All possible valid sets of fields
        let all_valid_field_sets = valid_year_field_sets
            .iter()
            .cartesian_product(valid_month_field_sets.iter())
            .cartesian_product(valid_day_field_sets.iter())
            .map(|((year_fields, month_fields), day_fields)| {
                year_fields
                    .iter()
                    .chain(month_fields.iter())
                    .chain(day_fields.iter())
                    .copied()
                    .collect::<BTreeSet<&str>>()
            })
            .collect::<BTreeSet<BTreeSet<&str>>>();

        // Field sets with year and month but without day that ECMA accepts
        let field_sets_without_day = valid_year_field_sets
            .iter()
            .cartesian_product(valid_month_field_sets.iter())
            .map(|(year_fields, month_fields)| {
                year_fields
                    .iter()
                    .chain(month_fields.iter())
                    .copied()
                    .collect::<BTreeSet<&str>>()
            })
            .collect::<BTreeSet<BTreeSet<&str>>>();

        // Field sets with month and day but without year that ECMA accepts
        let field_sets_without_year = [&["month_code", "day"][..]]
            .into_iter()
            .map(|field_names| field_names.iter().copied().collect())
            .collect::<Vec<BTreeSet<&str>>>();

        // A map from field names to a function that sets that field
        let mut field_fns = BTreeMap::<&str, &dyn Fn(&mut DateFields)>::new();
        field_fns.insert("era", &|fields| fields.era = Some("ad"));
        field_fns.insert("era_year", &|fields| fields.era_year = Some(2000));
        field_fns.insert("extended_year", &|fields| fields.extended_year = Some(2000));
        field_fns.insert("month_code", &|fields| {
            fields.month_code = MonthCode::new_normal(4)
        });
        field_fns.insert("ordinal_month", &|fields| {
            fields.ordinal_month = NonZeroU8::new(4)
        });
        field_fns.insert("day", &|fields| fields.day = NonZeroU8::new(20));

        for field_set in field_fns.keys().copied().powerset() {
            let field_set = field_set.into_iter().collect::<BTreeSet<&str>>();

            // Check whether this case should succeed: whether it identifies a year,
            // a month, and a day.
            let should_succeed_rejecting = all_valid_field_sets.contains(&field_set);

            // Check whether it should succeed in ECMA mode.
            let should_succeed_ecma = should_succeed_rejecting
                || field_sets_without_day.contains(&field_set)
                || field_sets_without_year.contains(&field_set);

            // Populate the fields in the field set
            let mut fields = Default::default();
            for field_name in field_set {
                field_fns.get(field_name).unwrap()(&mut fields);
            }

            // Check whether we were able to successfully construct the date
            let mut options = DateFromFieldsOptions::default();
            options.missing_fields_strategy = Some(MissingFieldsStrategy::Reject);
            match Date::try_from_fields(fields, options, Gregorian) {
                Ok(_) => assert!(
                    should_succeed_rejecting,
                    "Succeeded, but should have rejected: {fields:?}"
                ),
                Err(DateError::NotEnoughFields) => assert!(
                    !should_succeed_rejecting,
                    "Rejected, but should have succeeded: {fields:?}"
                ),
                Err(e) => panic!("Unexpected error: {e} for {fields:?}"),
            }

            // Check ECMA mode
            let mut options = DateFromFieldsOptions::default();
            options.missing_fields_strategy = Some(MissingFieldsStrategy::Ecma);
            match Date::try_from_fields(fields, options, Gregorian) {
                Ok(_) => assert!(
                    should_succeed_ecma,
                    "Succeeded, but should have rejected (ECMA): {fields:?}"
                ),
                Err(DateError::NotEnoughFields) => assert!(
                    !should_succeed_ecma,
                    "Rejected, but should have succeeded (ECMA): {fields:?}"
                ),
                Err(e) => panic!("Unexpected error: {e} for {fields:?}"),
            }
        }
    }
}
