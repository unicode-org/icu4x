// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{collections::HashSet, num::NonZeroU8};

use icu_calendar::{
    cal::*,
    options::{DateFromFieldsOptions, MissingFieldsStrategy},
    types::DateFields,
    Calendar, Date, Ref,
};

fn test_reference_year_impl<C>(cal: C)
where
    C: Calendar,
{
    let mut month_days_seen = HashSet::new();
    let mut rd = Date::try_new_iso(1972, 12, 31).unwrap().to_rata_die();
    for _ in 1..1000 {
        let date = Date::from_rata_die(rd, Ref(&cal));
        let month_day = (date.month().standard_code, date.day_of_month().0);
        let mut fields = DateFields::default();
        fields.month_code = Some(month_day.0);
        fields.day = NonZeroU8::new(month_day.1);
        let mut options = DateFromFieldsOptions::default();
        options.missing_fields_strategy = Some(MissingFieldsStrategy::Ecma);
        let reference_date = Date::try_from_fields(fields, options, Ref(&cal)).unwrap();
        if month_days_seen.contains(&month_day) {
            assert_ne!(date, reference_date);
        } else {
            assert_eq!(date, reference_date);
            month_days_seen.insert(month_day);
        }
        rd -= 1;
    }
}

#[test]
fn test_reference_year_buddhist() {
    test_reference_year_impl(Buddhist)
}

#[test]
#[ignore] // TODO: not yet implemented
fn test_reference_year_chinese() {
    test_reference_year_impl(Chinese::new())
}

#[test]
fn test_reference_year_coptic() {
    test_reference_year_impl(Coptic)
}

#[test]
#[ignore] // TODO: not yet implemented
fn test_reference_year_dangi() {
    test_reference_year_impl(Dangi::new())
}

#[test]
fn test_reference_year_ethiopian() {
    test_reference_year_impl(Ethiopian::new())
}

#[test]
fn test_reference_year_ethiopian_amete_alem() {
    test_reference_year_impl(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
}

#[test]
fn test_reference_year_gregorian() {
    test_reference_year_impl(Gregorian)
}

#[test]
fn test_reference_year_hebrew() {
    test_reference_year_impl(Hebrew)
}

#[test]
#[ignore] // TODO: not yet implemented
fn test_reference_year_indian() {
    test_reference_year_impl(Indian)
}

#[test]
fn test_reference_year_hijri_tabular_type_ii_friday() {
    test_reference_year_impl(Hijri::new_tabular(
        HijriTabularLeapYears::TypeII,
        HijriTabularEpoch::Friday,
    ))
}

#[test]
fn test_reference_year_hijri_simulated_mecca() {
    test_reference_year_impl(Hijri::new_simulated_mecca())
}

#[test]
fn test_reference_year_hijri_tabular_type_ii_thursday() {
    test_reference_year_impl(Hijri::new_tabular(
        HijriTabularLeapYears::TypeII,
        HijriTabularEpoch::Thursday,
    ))
}

#[test]
fn test_reference_year_hijri_umm_al_qura() {
    test_reference_year_impl(Hijri::new_umm_al_qura())
}

#[test]
fn test_reference_year_iso() {
    test_reference_year_impl(Iso)
}

#[test]
#[ignore] // TODO: not yet implemented
fn test_reference_year_japanese() {
    test_reference_year_impl(Japanese::new())
}

#[test]
#[ignore] // TODO: not yet implemented
fn test_reference_year_japanese_extended() {
    test_reference_year_impl(JapaneseExtended::new())
}

#[test]
#[ignore] // TODO: not yet implemented
fn test_reference_year_persian() {
    test_reference_year_impl(Persian)
}

#[test]
fn test_reference_year_roc() {
    test_reference_year_impl(Roc)
}
