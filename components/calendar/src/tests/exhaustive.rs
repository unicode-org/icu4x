// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use crate::calendar_arithmetic::{VALID_RD_RANGE, VALID_YEAR_RANGE};
use crate::*;

// Check rd -> date -> iso -> date -> rd for whole range
super::test_all_cals!(
    #[ignore] // takes about 200 seconds in release-with-assertions
    fn check_round_trip<C: Calendar>(cal: Ref<C>) {
        let low = *VALID_RD_RANGE.start();
        let high = *VALID_RD_RANGE.end();
        let mut prev = Date::from_rata_die(low, cal);
        let mut curr = low + 1;
        while curr <= high {
            let date = Date::from_rata_die(curr, cal);
            assert!(prev < date);

            let rd = date.to_rata_die();
            assert_eq!(rd, curr, "{}", cal.as_calendar().debug_name());

            let date2 = Date::new_from_iso(date.to_iso(), cal);
            assert_eq!(date, date2, "{:?}", cal.as_calendar().debug_name());

            prev = date;
            curr += 1;
        }
    }
);

super::test_all_cals!(
    #[ignore] // takes about 90 seconds in release-with-assertions
    fn check_from_fields<C: Calendar>(cal: Ref<C>) {
        let months = (1..19)
            .flat_map(|i| [types::Month::new(i).code(), types::Month::leap(i).code()].into_iter())
            .collect::<Vec<_>>();
        for year in VALID_YEAR_RANGE {
            if year % 50000 == 0 {
                println!("{} {year:?}", cal.as_calendar().debug_name());
            }
            for overflow in [options::Overflow::Constrain, options::Overflow::Reject] {
                let mut options = options::DateFromFieldsOptions::default();
                options.overflow = Some(overflow);
                for mut fields in months
                    .iter()
                    .map(|m| {
                        let mut fields = types::DateFields::default();
                        fields.month_code = Some(m.0.as_bytes());
                        fields
                    })
                    .chain((1..20).map(|m| {
                        let mut fields = types::DateFields::default();
                        fields.ordinal_month = Some(m);
                        fields
                    }))
                {
                    for day in 1..50 {
                        fields.extended_year = Some(year);
                        fields.day = Some(day);
                        let _ = Date::try_from_fields(fields, options, Ref(&cal));
                    }
                }
            }
        }
    }
);
