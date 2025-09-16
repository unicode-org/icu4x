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

impl DateFromFieldsOptions {
    #[inline]
    pub(crate) fn overflow(self) -> Overflow {
        self.overflow.unwrap_or_default()
    }
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
    /// options.overflow = Some(Overflow::Reject);
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

/// How to infer missing fields when the fields that are present do not fully constitute a Date.
///
/// In order for the fields to fully constitute a Date, they must identify a year, a month,
/// and a day. The following rules apply:
/// 
/// 1. At least one of extended year or era + era year must be present.
/// 2. At least one of month code or ordinal month must be present.
/// 3. If era is present, era year must also be present.
/// 4. If era year is present, era must also be present.
/// 
/// If any of these rules are violated, then ICU4X uses the strategy specified here.
/// 
/// If all of the rules are satisfied, then this strategy has no effect. Fields that are
/// out of bounds or inconsistent are handled by other error types.
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
    /// Missing fields are populated as follows:
    /// 
    /// 1. If the year and month are present
    ///
    /// | Fields Present  | Missing Fields Behavior               |
    /// |-----------------|---------------------------------------|
    /// | Year, Month     | set day to 1                          |
    /// | month code, day | set extended year to a reference year |
    ///
    /// If the fields that are present do not cover any of these sets, then
    /// [`DateError::NotEnoughFields`] is returned.
    ///
    /// The reference year is _some_ year in the calendar that contains the specified month code
    /// and day. The algorithm for choosing the reference year is specified by ECMAScript.
    ///
    /// [`DateError::NotEnoughFields`]: crate::DateError::NotEnoughFields
    Ecma,
}

#[cfg(test)]
mod tests {
    use crate::{types::{DateFields, MonthCode}, Date, DateError, Gregorian};
    use core::num::NonZeroU8;
    use std::collections::{BTreeMap, BTreeSet};
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_missing_fields_strategy_reject() {
        // A fully populated DateFields that is internally consistent
        let all_date_fields = DateFields {
            era: Some("ad"),
            era_year: Some(2000),
            monotonic_year: Some(2000),
            month_code: MonthCode::new_normal(4),
            ordinal_month: NonZeroU8::new(4),
            day: NonZeroU8::new(20),
        };

        // The sets of fields that are defined to fully constitute a Date
        let mut minimal_sets = Vec::<BTreeSet::<&str>>::new();
        minimal_sets.push(["era", "era_year", "month_code", "day"].into_iter().collect());
        minimal_sets.push(["extended_year", "month_code", "day"].into_iter().collect());
        minimal_sets.push(["era", "era_year", "ordinal_month", "day"].into_iter().collect());
        minimal_sets.push(["extended_year", "ordinal_month", "day"].into_iter().collect());

        // A map from field names to a function that unsets that field
        let mut field_fns = BTreeMap::<&str, &dyn Fn(&mut DateFields)>::new();
        field_fns.insert("era", &|fields| fields.era = None);
        field_fns.insert("era_year", &|fields| fields.era_year = None);
        field_fns.insert("monotonic_year", &|fields| fields.monotonic_year = None);
        field_fns.insert("month_code", &|fields| fields.month_code = None);
        field_fns.insert("ordinal_month", &|fields| fields.ordinal_month = None);
        field_fns.insert("day", &|fields| fields.day = None);

        for fields_to_remove in field_fns.keys().copied().powerset() {
            let fields_to_remove = fields_to_remove.into_iter().collect::<BTreeSet<&str>>();
            let should_succeed = minimal_sets.iter().any(|set| set.is_disjoint(&fields_to_remove));

            // Clone the base fields and unset all fields in this permutation
            let mut fields = all_date_fields.clone();
            for field_name in fields_to_remove {
                field_fns.get(field_name).unwrap()(&mut fields);
            }

            // Check whether we were able to successfully construct the date
            let mut options = DateFromFieldsOptions::default();
            options.missing_fields_strategy = Some(MissingFieldsStrategy::Reject);
            match Date::try_from_fields(fields, options, Gregorian) {
                Ok(_) => assert!(should_succeed, "Succeeded, but should have rejected: {fields:?}"),
                Err(DateError::NotEnoughFields) => assert!(!should_succeed, "Rejected, but should have succeeded: {fields:?}"),
                Err(e) => panic!("Unexpected error: {e} for {fields:?}"),
            }
        }
    }
}
