// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

// allowed for providers
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

use alloc::borrow::Cow;
use icu_calendar::types::MonthCode;
use icu_provider::{yoke, zerofrom};
use tinystr::{tinystr, TinyStr4};
use zerovec::ZeroMap;

#[icu_provider::data_struct(marker(
    DateSymbolsV1Marker,
    "datetime/datesymbols@1",
    extension_key = "ca"
))]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::calendar),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DateSymbolsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub months: months::ContextsV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekdays: weekdays::ContextsV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub eras: Eras<'data>,
}

#[icu_provider::data_struct(marker(
    TimeSymbolsV1Marker,
    "datetime/timesymbols@1",
    extension_key = "ca"
))]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::calendar),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimeSymbolsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub day_periods: day_periods::ContextsV1<'data>,
}

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::calendar),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct Eras<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, str, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub abbr: ZeroMap<'data, str, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub narrow: ZeroMap<'data, str, str>,
}

macro_rules! symbols {
    ($name: ident, $symbols: item) => {
        pub mod $name {
            use super::*;

            #[derive(Debug, PartialEq, Clone, zerofrom::ZeroFrom, yoke::Yokeable)]
            #[cfg_attr(
                feature = "datagen",
                derive(serde::Serialize, databake::Bake),
                databake(path = icu_datetime::provider::calendar::$name),
            )]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
            #[yoke(prove_covariance_manually)]
            $symbols

            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[cfg_attr(
                feature = "datagen",
                derive(serde::Serialize, databake::Bake),
                databake(path = icu_datetime::provider::calendar::$name),
            )]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
            #[yoke(prove_covariance_manually)]
            pub struct FormatWidthsV1<'data> {
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub abbreviated: SymbolsV1<'data>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub narrow: SymbolsV1<'data>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub short: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub wide: SymbolsV1<'data>,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[cfg_attr(
                feature = "datagen",
                derive(serde::Serialize, databake::Bake),
                databake(path = icu_datetime::provider::calendar::$name),
            )]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
            #[yoke(prove_covariance_manually)]
            pub struct StandAloneWidthsV1<'data> {
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub abbreviated: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub narrow: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub short: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub wide: Option<SymbolsV1<'data>>,
            }

            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[cfg_attr(
                feature = "datagen",
                derive(serde::Serialize, databake::Bake),
                databake(path = icu_datetime::provider::calendar::$name),
            )]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
            #[yoke(prove_covariance_manually)]
            pub struct ContextsV1<'data> {
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub format: FormatWidthsV1<'data>,
                #[cfg_attr(feature = "serde", serde(borrow))]
                pub stand_alone: Option<StandAloneWidthsV1<'data>>,
            }
        }
    };
}

symbols!(
    months,
    #[allow(clippy::large_enum_variant)]
    pub enum SymbolsV1<'data> {
        /// Twelve symbols for a solar calendar
        ///
        /// This is an optimization to reduce data size.
        SolarTwelve(
            #[cfg_attr(
                feature = "serde",
                serde(
                    borrow,
                    deserialize_with = "icu_provider::serde::borrow_de_utils::array_of_cow"
                )
            )]
            [Cow<'data, str>; 12],
        ),
        /// A calendar with an arbitrary number of months, potentially including leap months
        #[cfg_attr(feature = "serde", serde(borrow))]
        Other(ZeroMap<'data, MonthCode, str>),
    }
);

impl<'data> months::SymbolsV1<'data> {
    /// Get the symbol for the given month code
    pub fn get(&self, code: MonthCode) -> Option<&str> {
        match *self {
            Self::SolarTwelve(ref arr) => {
                // The tinystr macro doesn't work in match patterns
                // so we use consts first
                const CODE_1: TinyStr4 = tinystr!(4, "M01");
                const CODE_2: TinyStr4 = tinystr!(4, "M02");
                const CODE_3: TinyStr4 = tinystr!(4, "M03");
                const CODE_4: TinyStr4 = tinystr!(4, "M04");
                const CODE_5: TinyStr4 = tinystr!(4, "M05");
                const CODE_6: TinyStr4 = tinystr!(4, "M06");
                const CODE_7: TinyStr4 = tinystr!(4, "M07");
                const CODE_8: TinyStr4 = tinystr!(4, "M08");
                const CODE_9: TinyStr4 = tinystr!(4, "M09");
                const CODE_10: TinyStr4 = tinystr!(4, "M10");
                const CODE_11: TinyStr4 = tinystr!(4, "M11");
                const CODE_12: TinyStr4 = tinystr!(4, "M12");
                let idx = match code.0 {
                    CODE_1 => 0,
                    CODE_2 => 1,
                    CODE_3 => 2,
                    CODE_4 => 3,
                    CODE_5 => 4,
                    CODE_6 => 5,
                    CODE_7 => 6,
                    CODE_8 => 7,
                    CODE_9 => 8,
                    CODE_10 => 9,
                    CODE_11 => 10,
                    CODE_12 => 11,
                    _ => return None,
                };
                arr.get(idx).map(|x| &**x)
            }
            Self::Other(ref map) => map.get(&code),
        }
    }
}

impl Default for months::SymbolsV1<'_> {
    fn default() -> Self {
        Self::Other(Default::default())
    }
}

symbols!(
    weekdays,
    #[derive(Default)]
    pub struct SymbolsV1<'data>(
        #[cfg_attr(
            feature = "serde",
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
    #[derive(Default)]
    pub struct SymbolsV1<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub am: Cow<'data, str>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub pm: Cow<'data, str>,
        #[cfg_attr(
            feature = "serde",
            serde(
                borrow,
                deserialize_with = "icu_provider::serde::borrow_de_utils::option_of_cow"
            )
        )]
        pub noon: Option<Cow<'data, str>>,
        #[cfg_attr(
            feature = "serde",
            serde(
                borrow,
                deserialize_with = "icu_provider::serde::borrow_de_utils::option_of_cow"
            )
        )]
        pub midnight: Option<Cow<'data, str>>,
    }
);

#[cfg(all(test, feature = "datagen"))]
mod test {
    use super::*;
    use tinystr::tinystr;

    fn serialize_date() -> Vec<u8> {
        let months = [
            (&MonthCode(tinystr!(4, "M01")), "January"),
            (&MonthCode(tinystr!(4, "M02")), "February"),
            (&MonthCode(tinystr!(4, "M03")), "March"),
            (&MonthCode(tinystr!(4, "M04")), "April"),
            (&MonthCode(tinystr!(4, "M05")), "May"),
            (&MonthCode(tinystr!(4, "M06")), "June"),
            (&MonthCode(tinystr!(4, "M07")), "July"),
            (&MonthCode(tinystr!(4, "M08")), "August"),
            (&MonthCode(tinystr!(4, "M09")), "September"),
            (&MonthCode(tinystr!(4, "M10")), "October"),
            (&MonthCode(tinystr!(4, "M11")), "November"),
            (&MonthCode(tinystr!(4, "M12")), "December"),
        ];
        let months = months::SymbolsV1::Other(months.iter().copied().collect());

        let weekdays = weekdays::SymbolsV1([
            Cow::Owned("Monday".to_string()),
            Cow::Owned("Tuesday".to_string()),
            Cow::Owned("Wednesday".to_string()),
            Cow::Owned("Thursday".to_string()),
            Cow::Owned("Friday".to_string()),
            Cow::Owned("Saturday".to_string()),
            Cow::Owned("Sunday".to_string()),
        ]);

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
            eras: Eras {
                names: ZeroMap::new(),
                abbr: ZeroMap::new(),
                narrow: ZeroMap::new(),
            },
        })
        .unwrap()
    }

    fn serialize_time() -> Vec<u8> {
        let day_periods = day_periods::SymbolsV1 {
            am: Cow::Owned("am".to_string()),
            pm: Cow::Owned("pm".to_string()),
            noon: Some(Cow::Owned("noon".to_string())),
            midnight: None,
        };

        bincode::serialize(&TimeSymbolsV1 {
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
        })
        .unwrap()
    }

    #[test]
    fn weekdays_borrows() {
        let bytes = serialize_date();
        let de = bincode::deserialize::<DateSymbolsV1>(&bytes).unwrap();

        assert!(matches!(de.weekdays.format.narrow.0[2], Cow::Borrowed(_)));
        assert!(matches!(
            de.weekdays.format.short.as_ref().unwrap().0[4],
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn day_periods_borrows() {
        let bytes = serialize_time();
        let de = bincode::deserialize::<TimeSymbolsV1>(&bytes).unwrap();

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
