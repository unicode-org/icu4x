// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod continuity_test;
mod exhaustive;
mod extrema;
mod not_enough_fields;

macro_rules! test_all_cals {
    ($(#[$meta:meta])* fn $name:ident<C: Calendar>($cal:ident: Ref<C>) $tt:tt) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            fn test<C: crate::Calendar>(cal: C) {
                let $cal = crate::Ref(&cal);
                $tt
            }

            $(#[$meta])*
            #[test]
            fn buddhist() {
                test(crate::cal::Buddhist);
            }

            $(#[$meta])*
            #[test]
            fn chinese_traditional() {
                test(crate::cal::east_asian_traditional::EastAsianTraditional(crate::cal::east_asian_traditional_internal::EastAsianTraditionalYears::new(crate::cal::east_asian_traditional::China::default())));
            }

            $(#[$meta])*
            #[test]
            fn coptic() {
                test(crate::cal::Coptic);
            }

            $(#[$meta])*
            #[test]
            fn korean_traditional() {
                test(crate::cal::east_asian_traditional::EastAsianTraditional(crate::cal::east_asian_traditional_internal::EastAsianTraditionalYears::new(crate::cal::east_asian_traditional::Korea::default())));
            }

            $(#[$meta])*
            #[test]
            fn ethiopian() {
                test(crate::cal::Ethiopian::new());
            }

            $(#[$meta])*
            #[test]
            fn ethiopian_amete_alem() {
                test(crate::cal::Ethiopian::new_with_era_style(crate::cal::EthiopianEraStyle::AmeteAlem));
            }

            $(#[$meta])*
            #[test]
            fn gregorian() {
                test(crate::cal::Gregorian);
            }

            $(#[$meta])*
            #[test]
            fn hebrew() {
                test(crate::cal::Hebrew::new());
            }

            $(#[$meta])*
            #[test]
            fn hijri_tabular_friday() {
                test(crate::cal::Hijri::new_tabular(crate::cal::hijri::TabularAlgorithmLeapYears::TypeII, crate::cal::hijri::TabularAlgorithmEpoch::Friday));
            }

            $(#[$meta])*
            #[test]
            fn hijri_tabular_thursday() {
                test(crate::cal::Hijri::new_tabular(crate::cal::hijri::TabularAlgorithmLeapYears::TypeII, crate::cal::hijri::TabularAlgorithmEpoch::Thursday));
            }

            $(#[$meta])*
            #[test]
            fn hijri_uaq() {
                test(crate::cal::Hijri::new_umm_al_qura());
            }

            $(#[$meta])*
            #[test]
            fn indian() {
                test(crate::cal::Indian::new());
            }

            $(#[$meta])*
            #[test]
            fn iso() {
                test(crate::cal::Iso::new());
            }

            $(#[$meta])*
            #[test]
            fn julian() {
                test(crate::cal::Julian::new());
            }

            $(#[$meta])*
            #[test]
            fn japanese() {
                test(crate::cal::Japanese::new());
            }

            $(#[$meta])*
            #[test]
            fn persian() {
                test(crate::cal::Persian::new());
            }

            $(#[$meta])*
            #[test]
            fn roc() {
                test(crate::cal::Roc);
            }
        }
    };
}
use test_all_cals;
