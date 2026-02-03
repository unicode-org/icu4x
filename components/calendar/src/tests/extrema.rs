// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::calendar_arithmetic::{VALID_RD_RANGE, VALID_YEAR_RANGE};
use crate::types::Month;
use crate::Date;
use calendrical_calculations::gregorian::fixed_from_gregorian;
use calendrical_calculations::rata_die::RataDie;

// Minimum and maximum dates allowed in ECMA-262 Temporal.
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date#the_epoch_timestamps_and_invalid_date
const MIN_TEMPORAL: RataDie = fixed_from_gregorian(1970, 1, 1).add(-100_000_000);
const MAX_TEMPORAL: RataDie = fixed_from_gregorian(1970, 1, 1).add(100_000_000);

super::test_all_cals!(
    fn check_ecma_extrema<C: Calendar + Copy>(cal: C) {
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
    fn check_representation_extrema<C: Calendar + Copy>(cal: C) {
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
    fn check_from_codes_extrema<C: Calendar + Copy>(cal: C) {
        // Success
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.start(),
            Month::new(1).code(),
            1,
            cal,
        )
        .unwrap();
        Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(1).code(), 1, cal)
            .unwrap();

        // Error
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.start() - 1,
            Month::new(1).code(),
            1,
            cal,
        )
        .unwrap_err();
        Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.end() + 1,
            Month::new(1).code(),
            1,
            cal,
        )
        .unwrap_err();

        if let crate::types::YearInfo::Era(y) = Date::try_new_from_codes(
            None,
            *VALID_YEAR_RANGE.start(),
            Month::new(1).code(),
            1,
            cal,
        )
        .unwrap()
        .year()
        {
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.start() - 1,
                Month::new(1).code(),
                1,
                cal,
            )
            .unwrap_err();
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.end() + 1,
                Month::new(1).code(),
                1,
                cal,
            )
            .unwrap_err();
        }

        if let crate::types::YearInfo::Era(y) =
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(1).code(), 1, cal)
                .unwrap()
                .year()
        {
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.start() - 1,
                Month::new(1).code(),
                1,
                cal,
            )
            .unwrap_err();
            Date::try_new_from_codes(
                Some(&y.era),
                *VALID_YEAR_RANGE.end() + 1,
                Month::new(1).code(),
                1,
                cal,
            )
            .unwrap_err();
        }
    }
);

mod check_convenience_constructors {
    use crate::cal::{
        ChineseTraditional, EthiopianEraStyle, Hijri, HijriTabularEpoch, HijriTabularLeapYears,
        Japanese, KoreanTraditional,
    };

    use super::*;
    #[test]
    fn buddhist() {
        Date::try_new_buddhist(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_buddhist(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    #[allow(deprecated)]
    fn chinese_traditional() {
        Date::try_new_chinese_traditional(*VALID_YEAR_RANGE.start() - 1, Month::new(1), 1)
            .unwrap_err();
        Date::try_new_chinese_traditional(*VALID_YEAR_RANGE.end() + 1, Month::new(1), 1)
            .unwrap_err();
        #[allow(deprecated)]
        {
            let c = ChineseTraditional::new();
            Date::try_new_chinese_with_calendar(*VALID_YEAR_RANGE.start() - 1, 1, 1, c)
                .unwrap_err();
            Date::try_new_chinese_with_calendar(*VALID_YEAR_RANGE.end() + 1, 1, 1, c).unwrap_err();
        }
    }
    #[test]
    fn coptic() {
        Date::try_new_coptic(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_coptic(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    fn korean_traditional() {
        Date::try_new_korean_traditional(*VALID_YEAR_RANGE.start() - 1, Month::new(1), 1)
            .unwrap_err();
        Date::try_new_korean_traditional(*VALID_YEAR_RANGE.end() + 1, Month::new(1), 1)
            .unwrap_err();
        #[allow(deprecated)]
        {
            let c = KoreanTraditional::new();
            Date::try_new_dangi_with_calendar(*VALID_YEAR_RANGE.start() - 1, 1, 1, c).unwrap_err();
            Date::try_new_dangi_with_calendar(*VALID_YEAR_RANGE.end() + 1, 1, 1, c).unwrap_err();
        }
    }
    #[test]
    fn ethiopian() {
        Date::try_new_ethiopian(
            EthiopianEraStyle::AmeteMihret,
            *VALID_YEAR_RANGE.start() - 1,
            1,
            1,
        )
        .unwrap_err();
        Date::try_new_ethiopian(
            EthiopianEraStyle::AmeteMihret,
            *VALID_YEAR_RANGE.end() + 1,
            1,
            1,
        )
        .unwrap_err();
    }
    #[test]
    fn ethiopian_amete_alem() {
        Date::try_new_ethiopian(
            EthiopianEraStyle::AmeteAlem,
            *VALID_YEAR_RANGE.start() - 1,
            1,
            1,
        )
        .unwrap_err();
        Date::try_new_ethiopian(
            EthiopianEraStyle::AmeteAlem,
            *VALID_YEAR_RANGE.end() + 1,
            1,
            1,
        )
        .unwrap_err();
    }
    #[test]
    fn gregorian() {
        Date::try_new_gregorian(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_gregorian(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    fn hebrew() {
        Date::try_new_hebrew_v2(*VALID_YEAR_RANGE.start() - 1, Month::new(1), 1).unwrap_err();
        Date::try_new_hebrew_v2(*VALID_YEAR_RANGE.end() + 1, Month::new(1), 1).unwrap_err();
        #[allow(deprecated)]
        {
            Date::try_new_hebrew(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
            Date::try_new_hebrew(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
        }
    }
    #[test]
    fn hijri_tabular_friday() {
        let c = Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Friday);
        Date::try_new_hijri_with_calendar(*VALID_YEAR_RANGE.start() - 1, 1, 1, c).unwrap_err();
        Date::try_new_hijri_with_calendar(*VALID_YEAR_RANGE.end() + 1, 1, 1, c).unwrap_err();
    }
    #[test]
    fn hijri_tabular_thursday() {
        let c = Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday);
        Date::try_new_hijri_with_calendar(*VALID_YEAR_RANGE.start() - 1, 1, 1, c).unwrap_err();
        Date::try_new_hijri_with_calendar(*VALID_YEAR_RANGE.end() + 1, 1, 1, c).unwrap_err();
    }
    #[test]
    fn hijri_uaq() {
        let c = Hijri::new_umm_al_qura();
        Date::try_new_hijri_with_calendar(*VALID_YEAR_RANGE.start() - 1, 1, 1, c).unwrap_err();
        Date::try_new_hijri_with_calendar(*VALID_YEAR_RANGE.end() + 1, 1, 1, c).unwrap_err();
    }
    #[test]
    fn indian() {
        Date::try_new_indian(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_indian(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    fn iso() {
        Date::try_new_iso(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_iso(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    fn julian() {
        Date::try_new_julian(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_julian(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    fn japanese() {
        let cal = Japanese::new();
        Date::try_new_japanese_with_calendar("reiwa", *VALID_YEAR_RANGE.start() - 1, 1, 1, cal)
            .unwrap_err();
        Date::try_new_japanese_with_calendar("reiwa", *VALID_YEAR_RANGE.end() + 1, 1, 1, cal)
            .unwrap_err();
    }
    #[test]
    fn persian() {
        Date::try_new_persian(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_persian(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
    #[test]
    fn roc() {
        Date::try_new_roc(*VALID_YEAR_RANGE.start() - 1, 1, 1).unwrap_err();
        Date::try_new_roc(*VALID_YEAR_RANGE.end() + 1, 1, 1).unwrap_err();
    }
}
