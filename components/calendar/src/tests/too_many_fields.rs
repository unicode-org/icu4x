// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DateFromFieldsError;
use crate::types::{DateFields, Month};
use crate::Date;

#[test]
fn test_from_fields_too_many_fields() {
    let mut fields = DateFields::default();
    fields.extended_year = Some(2025);
    fields.month = Some(Month::new(1));
    fields.month_code = Some(b"M01");
    fields.day = Some(1);

    let result = Date::try_from_fields(fields, Default::default(), crate::cal::Gregorian);
    assert_eq!(result.err(), Some(DateFromFieldsError::TooManyFields));

    let result = Date::try_from_fields(fields, Default::default(), crate::cal::Iso);
    assert_eq!(result.err(), Some(DateFromFieldsError::TooManyFields));

    let result = Date::try_from_fields(fields, Default::default(), crate::cal::Hebrew::new());
    assert_eq!(result.err(), Some(DateFromFieldsError::TooManyFields));
}
