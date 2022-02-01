// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
use zerovec::map::ZeroMap;

#[icu_provider::data_struct("datetime/symbols@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct DateSymbolsV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub months: months::ContextsV1<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub weekdays: weekdays::ContextsV1<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub day_periods: day_periods::ContextsV1<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub eras: Eras<'data>,
}

#[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct Eras<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub names: ZeroMap<'data, str, str>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub abbr: ZeroMap<'data, str, str>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub narrow: ZeroMap<'data, str, str>,
}

macro_rules! symbols {
    ($name: ident, $symbols: item) => {
        pub mod $name {
            use super::*;

            #[derive(Debug, PartialEq, Clone, Default, ZeroCopyFrom, Yokeable)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            $symbols

            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1<'data> {
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub abbreviated: SymbolsV1<'data>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub narrow: SymbolsV1<'data>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub short: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub wide: SymbolsV1<'data>,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1<'data> {
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub abbreviated: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub narrow: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub short: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub wide: Option<SymbolsV1<'data>>,
            }

            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1<'data> {
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub format: FormatWidthsV1<'data>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub stand_alone: Option<StandAloneWidthsV1<'data>>,
            }
        }
    };
}

symbols!(
    months,
    pub struct SymbolsV1<'data>(
        #[cfg_attr(
            feature = "provider_serde",
            serde(
                borrow,
                deserialize_with = "icu_provider::serde::borrow_de_utils::array_of_cow"
            )
        )]
        pub [Cow<'data, str>; 12],
    );
);

symbols!(
    weekdays,
    pub struct SymbolsV1<'data>(
        #[cfg_attr(
            feature = "provider_serde",
            serde(
                borrow,
                deserialize_with = "icu_provider::serde::borrow_de_utils::array_of_cow"
            )
        )]
        pub [Cow<'data, str>; 7],
    );
);

symbols!(
    day_periods,
    pub struct SymbolsV1<'data> {
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub am: Cow<'data, str>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub pm: Cow<'data, str>,
        #[cfg_attr(
            feature = "provider_serde",
            serde(
                borrow,
                deserialize_with = "icu_provider::serde::borrow_de_utils::option_of_cow"
            )
        )]
        pub noon: Option<Cow<'data, str>>,
        #[cfg_attr(
            feature = "provider_serde",
            serde(
                borrow,
                deserialize_with = "icu_provider::serde::borrow_de_utils::option_of_cow"
            )
        )]
        pub midnight: Option<Cow<'data, str>>,
    }
);

#[cfg(all(test, feature = "provider_serde"))]
mod test {
    use super::*;

    fn serialize() -> Vec<u8> {
        let months = months::SymbolsV1([
            Cow::Owned("January".to_string()),
            Cow::Owned("February".to_string()),
            Cow::Owned("March".to_string()),
            Cow::Owned("April".to_string()),
            Cow::Owned("May".to_string()),
            Cow::Owned("June".to_string()),
            Cow::Owned("July".to_string()),
            Cow::Owned("August".to_string()),
            Cow::Owned("September".to_string()),
            Cow::Owned("October".to_string()),
            Cow::Owned("November".to_string()),
            Cow::Owned("December".to_string()),
        ]);

        let weekdays = weekdays::SymbolsV1([
            Cow::Owned("Monday".to_string()),
            Cow::Owned("Tuesday".to_string()),
            Cow::Owned("Wednesday".to_string()),
            Cow::Owned("Thursday".to_string()),
            Cow::Owned("Friday".to_string()),
            Cow::Owned("Saturday".to_string()),
            Cow::Owned("Sunday".to_string()),
        ]);

        let day_periods = day_periods::SymbolsV1 {
            am: Cow::Owned("am".to_string()),
            pm: Cow::Owned("pm".to_string()),
            noon: Some(Cow::Owned("noon".to_string())),
            midnight: None,
        };

        bincode::serialize(&DateSymbolsV1 {
            months: months::ContextsV1 {
                format: months::FormatWidthsV1 {
                    abbreviated: months.clone(),
                    narrow: months.clone(),
                    short: Some(months.clone()),
                    wide: months.clone(),
                },
                stand_alone: Some(months::StandAloneWidthsV1 {
                    abbreviated: Some(months.clone()),
                    narrow: Some(months.clone()),
                    short: Some(months.clone()),
                    wide: Some(months.clone()),
                }),
            },
            weekdays: weekdays::ContextsV1 {
                format: weekdays::FormatWidthsV1 {
                    abbreviated: weekdays.clone(),
                    narrow: weekdays.clone(),
                    short: Some(weekdays.clone()),
                    wide: weekdays.clone(),
                },
                stand_alone: Some(weekdays::StandAloneWidthsV1 {
                    abbreviated: Some(weekdays.clone()),
                    narrow: Some(weekdays.clone()),
                    short: Some(weekdays.clone()),
                    wide: Some(weekdays.clone()),
                }),
            },
            day_periods: day_periods::ContextsV1 {
                format: day_periods::FormatWidthsV1 {
                    abbreviated: day_periods.clone(),
                    narrow: day_periods.clone(),
                    short: Some(day_periods.clone()),
                    wide: day_periods.clone(),
                },
                stand_alone: Some(day_periods::StandAloneWidthsV1 {
                    abbreviated: Some(day_periods.clone()),
                    narrow: Some(day_periods.clone()),
                    short: Some(day_periods.clone()),
                    wide: Some(day_periods.clone()),
                }),
            },
            eras: Eras {
                names: ZeroMap::new(),
                abbr: ZeroMap::new(),
                narrow: ZeroMap::new(),
            },
        })
        .unwrap()
    }

    #[test]
    fn months_borrows() {
        let bytes = serialize();
        let de = bincode::deserialize::<DateSymbolsV1>(&bytes).unwrap();

        assert!(matches!(de.months.format.narrow.0[2], Cow::Borrowed(_)));
        assert!(matches!(
            de.months.format.short.as_ref().unwrap().0[11],
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn weekdays_borrows() {
        let bytes = serialize();
        let de = bincode::deserialize::<DateSymbolsV1>(&bytes).unwrap();

        assert!(matches!(de.weekdays.format.narrow.0[2], Cow::Borrowed(_)));
        assert!(matches!(
            de.weekdays.format.short.as_ref().unwrap().0[4],
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn day_periods_borrows() {
        let bytes = serialize();
        let de = bincode::deserialize::<DateSymbolsV1>(&bytes).unwrap();

        assert!(matches!(
            de.day_periods.format.narrow.noon,
            Some(Cow::Borrowed(_))
        ));
        assert!(matches!(
            de.day_periods.format.short.as_ref().unwrap().noon,
            Some(Cow::Borrowed(_))
        ));

        assert!(matches!(de.day_periods.format.narrow.am, Cow::Borrowed(_)));
        assert!(matches!(
            de.day_periods.format.short.as_ref().unwrap().am,
            Cow::Borrowed(_)
        ));
    }
}
