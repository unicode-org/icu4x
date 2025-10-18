// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    error::DateFromFieldsError,
    options::{DateFromFieldsOptions, MissingFieldsStrategy},
    types::{DateFields, MonthCode},
    AnyCalendar, AnyCalendarKind, Date, Ref,
};

static INVALID_SYNTAX: &[&str] = &[
    "M", "M0", "M1", "01L", "L01", "M001", "M110", "MxxL", "m01", "M02l",
];

static NOT_IN_ANY_CALENDAR: &[&str] = &["M00", "M14", "M99", "M13L"];

static CHINESE_ONLY: &[&str] = &[
    "M01L", "M02L", "M03L", "M04L", "M06L", "M07L", "M08L", "M09L", "M10L", "M11L", "M12L",
];

static CHINESE_HEBREW: &[&str] = &["M05L"];

static COPTIC_ONLY: &[&str] = &["M13"];

static UNIVERSAL_MONTH_CODES: &[&str] = &[
    "M01", "M02", "M03", "M04", "M05", "M06", "M07", "M08", "M09", "M10", "M11", "M12",
];

#[test]
fn test_month_code_errors() {
    for kind in [
        AnyCalendarKind::Buddhist,
        AnyCalendarKind::Chinese,
        AnyCalendarKind::Coptic,
        AnyCalendarKind::Dangi,
        AnyCalendarKind::Ethiopian,
        AnyCalendarKind::EthiopianAmeteAlem,
        AnyCalendarKind::Gregorian,
        AnyCalendarKind::Hebrew,
        AnyCalendarKind::HijriUmmAlQura,
    ] {
        let cal = AnyCalendar::new(kind);

        let mut valid_month_codes = UNIVERSAL_MONTH_CODES.to_vec();
        let mut invalid_month_codes = NOT_IN_ANY_CALENDAR.to_vec();

        if matches!(kind, AnyCalendarKind::Chinese | AnyCalendarKind::Dangi) {
            &mut valid_month_codes
        } else {
            &mut invalid_month_codes
        }
        .extend_from_slice(CHINESE_ONLY);

        if matches!(
            kind,
            AnyCalendarKind::Chinese | AnyCalendarKind::Dangi | AnyCalendarKind::Hebrew
        ) {
            &mut valid_month_codes
        } else {
            &mut invalid_month_codes
        }
        .extend_from_slice(CHINESE_HEBREW);

        if matches!(
            kind,
            AnyCalendarKind::Coptic
                | AnyCalendarKind::Ethiopian
                | AnyCalendarKind::EthiopianAmeteAlem
        ) {
            &mut valid_month_codes
        } else {
            &mut invalid_month_codes
        }
        .extend_from_slice(COPTIC_ONLY);

        // Test with full dates
        for extended_year in -100..=100 {
            let options = DateFromFieldsOptions::default();
            let mut fields = DateFields::default();
            fields.extended_year = Some(extended_year);
            fields.day = Some(1);
            for month_code in valid_month_codes.iter() {
                fields.month_code = Some(MonthCode(month_code.parse().unwrap()));
                match Date::try_from_fields(fields, options, Ref(&cal)) {
                    Ok(_) => (),
                    Err(DateFromFieldsError::UnknownMonthCodeForYear) => (),
                    Err(e) => {
                        panic!("Should have succeeded, but failed: {kind:?} {extended_year} {month_code} {e:?}");
                    }
                }
            }
            for month_code in invalid_month_codes.iter() {
                fields.month_code = Some(MonthCode(month_code.parse().unwrap()));
                match Date::try_from_fields(fields, options, Ref(&cal)) {
                    Err(DateFromFieldsError::UnknownMonthCodeForCalendar) => (),
                    Ok(_) => {
                        panic!("Should have failed, but succeeded: {kind:?} {extended_year} {month_code}");
                    }
                    Err(e) => {
                        panic!(
                            "Failed with wrong error: {kind:?} {extended_year} {month_code} {e:?}"
                        );
                    }
                }
            }
            for month_code in INVALID_SYNTAX.iter() {
                fields.month_code = Some(MonthCode(month_code.parse().unwrap()));
                match Date::try_from_fields(fields, options, Ref(&cal)) {
                    Err(DateFromFieldsError::InvalidMonthCode) => (),
                    Ok(_) => {
                        panic!("Should have failed, but succeeded: {kind:?} {extended_year} {month_code}");
                    }
                    Err(e) => {
                        panic!(
                            "Failed with wrong error: {kind:?} {extended_year} {month_code} {e:?}"
                        );
                    }
                }
            }
        }

        // Test with reference year
        let mut fields = DateFields::default();
        fields.day = Some(1);
        let mut options = DateFromFieldsOptions::default();
        options.missing_fields_strategy = Some(MissingFieldsStrategy::Ecma);
        for month_code in valid_month_codes.iter() {
            fields.month_code = Some(MonthCode(month_code.parse().unwrap()));
            match Date::try_from_fields(fields, options, Ref(&cal)) {
                Ok(_) => (),
                Err(e) => {
                    panic!("Should have succeeded, but failed: {kind:?} {month_code} {e:?} (reference year)");
                }
            }
        }
        for month_code in invalid_month_codes.iter() {
            fields.month_code = Some(MonthCode(month_code.parse().unwrap()));
            match Date::try_from_fields(fields, options, Ref(&cal)) {
                Err(DateFromFieldsError::UnknownMonthCodeForCalendar) => (),
                Ok(_) => {
                    panic!(
                        "Should have failed, but succeeded: {kind:?} {month_code} (reference year)"
                    );
                }
                Err(e) => {
                    panic!("Failed with wrong error: {kind:?} {month_code} {e:?} (reference year)");
                }
            }
        }
        for month_code in INVALID_SYNTAX.iter() {
            fields.month_code = Some(MonthCode(month_code.parse().unwrap()));
            match Date::try_from_fields(fields, options, Ref(&cal)) {
                Err(DateFromFieldsError::InvalidMonthCode) => (),
                Ok(_) => {
                    panic!(
                        "Should have failed, but succeeded: {kind:?} {month_code} (reference year)"
                    );
                }
                Err(e) => {
                    panic!("Failed with wrong error: {kind:?} {month_code} {e:?} (reference year)");
                }
            }
        }
    }
}
