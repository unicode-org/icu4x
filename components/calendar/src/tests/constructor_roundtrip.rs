// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DateFromFieldsError;
use crate::options::{DateFromFieldsOptions, Overflow};
use crate::types::{DateFields, Month};
use crate::Calendar;
use crate::Date;
use calendrical_calculations::rata_die::RataDie;

// We could use the `bitflags` crate here, but we are using raw constants
// to avoid pulling in an extra dependency.
type Flags = u32;
const EXTENDED_YEAR: u32 = 1 << 0;
const ERA: u32 = 1 << 1;
const ERA_YEAR: u32 = 1 << 2;
const MONTH: u32 = 1 << 3;
const MONTH_CODE: u32 = 1 << 4;
const ORDINAL_MONTH: u32 = 1 << 5;
const DAY: u32 = 1 << 6;
const MAKE_INCONSISTENT_YEAR: u32 = 1 << 7;
const MAKE_INCONSISTENT_MONTH: u32 = 1 << 8;

fn try_from_fields_helper<C: Calendar + Copy>(
    flags: Flags,
    rd: RataDie,
    options: DateFromFieldsOptions,
    cal: C,
) -> Result<Date<C>, DateFromFieldsError> {
    let date = Date::from_rata_die(rd, cal);
    let mut fields = DateFields::default();

    if flags & EXTENDED_YEAR != 0 {
        fields.extended_year = Some(date.year().extended_year());
    }

    let era_bytes = date.year().era().map(|ey| ey.era);
    if flags & ERA != 0 {
        if let Some(ref eb) = era_bytes {
            fields.era = Some(eb.as_bytes());
        }
    }
    if flags & ERA_YEAR != 0 {
        if let Some(ey) = date.year().era() {
            fields.era_year = Some(ey.year);
        }
    }

    let month_input = date.month().to_input();
    if flags & MONTH != 0 {
        fields.month = Some(month_input);
    }

    let month_code_holder = month_input.code();
    if flags & MONTH_CODE != 0 {
        fields.month_code = Some(month_code_holder.0.as_bytes());
    }
    if flags & ORDINAL_MONTH != 0 {
        fields.ordinal_month = Some(date.month().ordinal);
    }
    if flags & DAY != 0 {
        fields.day = Some(date.day_of_month().0);
    }

    if flags & MAKE_INCONSISTENT_YEAR != 0 {
        if let Some(ref mut y) = fields.extended_year {
            *y += 1;
        }
    }

    if flags & MAKE_INCONSISTENT_MONTH != 0 {
        if let Some(ref mut m) = fields.ordinal_month {
            *m = if *m == 1 { 2 } else { 1 };
        }
    }

    Date::try_from_fields(fields, options, cal)
}

fn get_interesting_rds() -> Vec<RataDie> {
    let mut rds = Vec::new();

    // Iso / Gregorian leap day
    let date = Date::try_new_iso(2020, 2, 29).expect("2020-02-29 is a valid ISO date");
    rds.push(date.to_rata_die());

    // Hebrew leap month (Adar I) in leap year 5784
    let date = Date::try_new_hebrew_v2(5784, Month::leap(5), 15).expect("Valid Hebrew date");
    rds.push(date.to_rata_die());

    // Hebrew Adar II in leap year 5784
    let date = Date::try_new_hebrew_v2(5784, Month::new(6), 15).expect("Valid Hebrew date");
    rds.push(date.to_rata_die());

    // Hebrew non-Adar month in leap year 5784
    let date = Date::try_new_hebrew_v2(5784, Month::new(1), 15).expect("Valid Hebrew date");
    rds.push(date.to_rata_die());

    // Chinese leap month date
    let date =
        Date::try_new_chinese_traditional(2023, Month::leap(2), 1).expect("Valid Chinese date");
    rds.push(date.to_rata_die());

    // Chinese leap year 2023, non-leap month
    let date =
        Date::try_new_chinese_traditional(2023, Month::new(1), 1).expect("Valid Chinese date");
    rds.push(date.to_rata_die());

    // Japanese era transition
    let date = Date::try_new_japanese_with_calendar("reiwa", 1, 5, 1, crate::cal::Japanese::new())
        .expect("Valid Japanese date");
    rds.push(date.to_rata_die());

    // Ethiopic month 13
    let date = Date::try_new_ethiopian(crate::cal::EthiopianEraStyle::AmeteMihret, 2015, 13, 5)
        .expect("Valid Ethiopian date");
    rds.push(date.to_rata_die());

    // Coptic month 13
    let date = Date::try_new_coptic(1686, 13, 5).expect("Valid Coptic date");
    rds.push(date.to_rata_die());

    // Add some more general dates
    rds.push(RataDie::new(0));
    rds.push(RataDie::new(1));
    rds.push(RataDie::new(-1));
    rds.push(RataDie::new(730000)); // ~2000 CE
    rds.push(RataDie::new(100));
    rds.push(RataDie::new(-100));
    rds.push(RataDie::new(1000000)); // ~2739 CE
    rds.push(RataDie::new(-1000000)); // ~ -2739 CE
    rds.push(RataDie::new(123456));
    rds.push(RataDie::new(-123456));

    rds
}

super::test_all_cals!(
    fn check_constructor_roundtrip<C: Calendar + Copy>(cal: C) {
        let rds = get_interesting_rds();

        for rd in rds {
            let date = Date::from_rata_die(rd, cal);

            for overflow in [Overflow::Reject, Overflow::Constrain] {
                let options = DateFromFieldsOptions {
                    overflow: Some(overflow),
                    ..Default::default()
                };

                // Test 1: extended_year + month + day
                let result = try_from_fields_helper(EXTENDED_YEAR | MONTH | DAY, rd, options, cal);
                assert_eq!(
                    result.unwrap().to_rata_die(),
                    rd,
                    "Failed roundtrip for extended_year + month + day at RD {:?}",
                    rd
                );

                // Test 2: extended_year + month_code + day
                let result =
                    try_from_fields_helper(EXTENDED_YEAR | MONTH_CODE | DAY, rd, options, cal);
                assert_eq!(
                    result.unwrap().to_rata_die(),
                    rd,
                    "Failed roundtrip for extended_year + month_code + day at RD {:?}",
                    rd
                );

                // Test 3: extended_year + ordinal_month + day
                let result =
                    try_from_fields_helper(EXTENDED_YEAR | ORDINAL_MONTH | DAY, rd, options, cal);
                assert_eq!(
                    result.unwrap().to_rata_die(),
                    rd,
                    "Failed roundtrip for extended_year + ordinal_month + day at RD {:?}",
                    rd
                );

                // Test 4: era + era_year + month + day (if era exists)
                if date.year().era().is_some() {
                    let result =
                        try_from_fields_helper(ERA | ERA_YEAR | MONTH | DAY, rd, options, cal);
                    assert_eq!(
                        result.unwrap().to_rata_die(),
                        rd,
                        "Failed roundtrip for era + era_year + month + day at RD {:?}",
                        rd
                    );
                }

                // Test 5: Both extended_year and era+era_year
                if date.year().era().is_some() {
                    let result = try_from_fields_helper(
                        EXTENDED_YEAR | ERA | ERA_YEAR | MONTH | DAY,
                        rd,
                        options,
                        cal,
                    );
                    assert_eq!(
                        result.unwrap().to_rata_die(),
                        rd,
                        "Failed roundtrip for both year fields at RD {:?}",
                        rd
                    );
                }

                // Test 6: Date::try_new
                let extended_year = date.year().extended_year();
                let month = date.month().to_input();
                let day = date.day_of_month().0;
                let reconstructed = Date::try_new(extended_year.into(), month, day, cal).unwrap();
                assert_eq!(
                    reconstructed.to_rata_die(),
                    rd,
                    "Failed roundtrip for Date::try_new at RD {:?}",
                    rd
                );

                // Negative Test 1: Inconsistent year
                if date.year().era().is_some() {
                    let result = try_from_fields_helper(
                        EXTENDED_YEAR | ERA | ERA_YEAR | MONTH | DAY | MAKE_INCONSISTENT_YEAR,
                        rd,
                        options,
                        cal,
                    );
                    assert_eq!(
                        result.map(|d| d.to_rata_die()),
                        Err(DateFromFieldsError::InconsistentYear),
                        "Expected InconsistentYear for RD {:?}",
                        rd
                    );
                }

                // Negative Test 2: Inconsistent month
                if date.months_in_year() >= 2 {
                    let result = try_from_fields_helper(
                        EXTENDED_YEAR | MONTH | ORDINAL_MONTH | DAY | MAKE_INCONSISTENT_MONTH,
                        rd,
                        options,
                        cal,
                    );
                    assert_eq!(
                        result.map(|d| d.to_rata_die()),
                        Err(DateFromFieldsError::InconsistentMonth),
                        "Expected InconsistentMonth for RD {:?}",
                        rd
                    );
                }

                // Test insufficient fields: Missing Day
                {
                    let result = try_from_fields_helper(EXTENDED_YEAR | MONTH, rd, options, cal);
                    assert!(
                        result.is_err(),
                        "Expected error for missing day at RD {:?}",
                        rd
                    );
                }

                // Test insufficient fields: Missing Year
                {
                    let result = try_from_fields_helper(MONTH | DAY, rd, options, cal);
                    assert!(
                        result.is_err(),
                        "Expected error for missing year at RD {:?}",
                        rd
                    );
                }
            }
        }
    }
);

#[test]
fn test_calendar_specific_constructors() {
    let rds = get_interesting_rds();
    for rd in rds {
        // ISO
        {
            let date = Date::from_rata_die(rd, crate::cal::Iso);
            let reconstructed = Date::try_new_iso(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "ISO failed for RD {:?}",
                rd
            );
        }
        // Gregorian
        {
            let date = Date::from_rata_die(rd, crate::cal::Gregorian);
            let reconstructed = Date::try_new_gregorian(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Gregorian failed for RD {:?}",
                rd
            );
        }
        // Hebrew
        {
            let date = Date::from_rata_die(rd, crate::cal::Hebrew);
            let reconstructed = Date::try_new_hebrew_v2(
                date.year().extended_year(),
                date.month().to_input(),
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Hebrew failed for RD {:?}",
                rd
            );
        }
        // Buddhist
        {
            let date = Date::from_rata_die(rd, crate::cal::Buddhist);
            let reconstructed = Date::try_new_buddhist(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Buddhist failed for RD {:?}",
                rd
            );
        }
        // Coptic
        {
            let date = Date::from_rata_die(rd, crate::cal::Coptic);
            let reconstructed = Date::try_new_coptic(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Coptic failed for RD {:?}",
                rd
            );
        }
        // Ethiopian
        {
            let date = Date::from_rata_die(
                rd,
                crate::cal::Ethiopian::new_with_era_style(
                    crate::cal::EthiopianEraStyle::AmeteMihret,
                ),
            );
            let reconstructed = Date::try_new_ethiopian(
                crate::cal::EthiopianEraStyle::AmeteMihret,
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Ethiopian failed for RD {:?}",
                rd
            );
        }
        // Indian
        {
            let date = Date::from_rata_die(rd, crate::cal::Indian);
            let reconstructed = Date::try_new_indian(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Indian failed for RD {:?}",
                rd
            );
        }
        // Julian
        {
            let date = Date::from_rata_die(rd, crate::cal::Julian);
            let reconstructed = Date::try_new_julian(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Julian failed for RD {:?}",
                rd
            );
        }
        // Persian
        {
            let date = Date::from_rata_die(rd, crate::cal::Persian);
            let reconstructed = Date::try_new_persian(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Persian failed for RD {:?}",
                rd
            );
        }
        // Roc
        {
            let date = Date::from_rata_die(rd, crate::cal::Roc);
            let reconstructed = Date::try_new_roc(
                date.year().extended_year(),
                date.month().ordinal,
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Roc failed for RD {:?}",
                rd
            );
        }
        // Japanese
        {
            let date = Date::from_rata_die(rd, crate::cal::Japanese::new());
            let reconstructed = Date::try_new_japanese_with_calendar(
                date.era_year().era.as_str(),
                date.era_year().year,
                date.month().ordinal,
                date.day_of_month().0,
                crate::cal::Japanese::new(),
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Japanese failed for RD {:?}",
                rd
            );
        }
        // Chinese
        {
            let date = Date::from_rata_die(rd, crate::cal::ChineseTraditional::new());
            let reconstructed = Date::try_new_chinese_traditional(
                date.year().extended_year(),
                date.month().to_input(),
                date.day_of_month().0,
            )
            .unwrap();
            assert_eq!(
                reconstructed.to_rata_die(),
                rd,
                "Chinese failed for RD {:?}",
                rd
            );
        }
    }
}
