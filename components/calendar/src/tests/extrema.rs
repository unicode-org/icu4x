// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::calendar_arithmetic::{VALID_RD_RANGE, VALID_YEAR_RANGE};
use crate::types::MonthCode;
use crate::Date;
use calendrical_calculations::gregorian::fixed_from_gregorian;
use calendrical_calculations::rata_die::RataDie;

// Minimum and maximum dates allowed in ECMA-262 Temporal.
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date#the_epoch_timestamps_and_invalid_date
const MIN_TEMPORAL: RataDie = fixed_from_gregorian(1970, 1, 1).add(-100_000_000);
const MAX_TEMPORAL: RataDie = fixed_from_gregorian(1970, 1, 1).add(100_000_000);

super::test_all_cals!(
    fn check_ecma_extrema<C: Calendar>(cal: Ref<C>) {
        // Round-trips
        assert_eq!(
            Date::from_rata_die(MIN_TEMPORAL, cal).to_rata_die(),
            MIN_TEMPORAL
        );
        assert_eq!(
            Date::from_rata_die(MAX_TEMPORAL, cal).to_rata_die(),
            MAX_TEMPORAL
        );
    }
);

super::test_all_cals!(
    fn check_representation_extrema<C: Calendar>(cal: Ref<C>) {
        // Round-trips
        assert_eq!(
            Date::from_rata_die(*VALID_RD_RANGE.start(), cal).to_rata_die(),
            *VALID_RD_RANGE.start()
        );
        assert_eq!(
            Date::from_rata_die(*VALID_RD_RANGE.end(), cal).to_rata_die(),
            *VALID_RD_RANGE.end()
        );

        // Saturates
        assert_eq!(
            Date::from_rata_die(*VALID_RD_RANGE.start() - 1, cal).to_rata_die(),
            *VALID_RD_RANGE.start()
        );
        assert_eq!(
            Date::from_rata_die(*VALID_RD_RANGE.end() + 1, cal).to_rata_die(),
            *VALID_RD_RANGE.end()
        );
    }
);

super::test_all_cals!(
    fn check_from_codes_extrema<C: Calendar>(cal: Ref<C>) {
        // Success
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.start(),
            MonthCode::new_normal(1).unwrap(),
            1,
            cal,
        )
        .unwrap();
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.end(),
            MonthCode::new_normal(1).unwrap(),
            1,
            cal,
        )
        .unwrap();

        // Error
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.start() - 1,
            MonthCode::new_normal(1).unwrap(),
            1,
            cal,
        )
        .unwrap_err();
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.end() + 1,
            MonthCode::new_normal(1).unwrap(),
            1,
            cal,
        )
        .unwrap_err();

        if let crate::types::YearInfo::Era(y) = Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.start(),
            MonthCode::new_normal(1).unwrap(),
            1,
            cal,
        )
        .unwrap()
        .year()
        {
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.start() - 1,
                MonthCode::new_normal(1).unwrap(),
                1,
                cal,
            )
            .unwrap_err();
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.end() + 1,
                MonthCode::new_normal(1).unwrap(),
                1,
                cal,
            )
            .unwrap_err();
        }

        if let crate::types::YearInfo::Era(y) = Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.end(),
            MonthCode::new_normal(1).unwrap(),
            1,
            cal,
        )
        .unwrap()
        .year()
        {
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.start() - 1,
                MonthCode::new_normal(1).unwrap(),
                1,
                cal,
            )
            .unwrap_err();
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.end() + 1,
                MonthCode::new_normal(1).unwrap(),
                1,
                cal,
            )
            .unwrap_err();
        }
    }
);
