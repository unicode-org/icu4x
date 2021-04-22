// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::{provider::*, skeleton::SkeletonError};
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::marker::PhantomData;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::GREGORY_V1, //
];

/// A data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct DatesProvider<'d> {
    data: Vec<(CldrLangID, cldr_json::LangDates)>,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for DatesProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_dates()?.join("main");

        let locale_dirs = get_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("ca-gregorian.json");

            let mut resource: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.append(&mut resource.main.0);
        }

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for DatesProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::Dates {
            return Err((&resc_key.category).into());
        }
        if resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d> DataProvider<'d, gregory::DatesV1> for DatesProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, gregory::DatesV1>, DataError> {
        DatesProvider::supports_key(&req.resource_path.key)?;
        let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
        let dates = match self
            .data
            .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
        {
            Ok(idx) => &self.data[idx].1.dates,
            Err(_) => return Err(DataError::UnavailableResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: DataPayload {
                cow: Some(Cow::Owned(gregory::DatesV1::from(dates))),
            },
        })
    }
}

icu_provider::impl_dyn_provider!(DatesProvider<'d>, gregory::DatesV1, SERDE_SE, 'd, 's);

impl<'d> IterableDataProviderCore for DatesProvider<'d> {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            .map(|(l, _)| ResourceOptions {
                variant: None,
                // TODO: Avoid the clone
                langid: Some(l.langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_json::LengthPatterns> for gregory::patterns::LengthPatternsV1 {
    fn from(other: &cldr_json::LengthPatterns) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: Cow::Owned(other.full.get_pattern().clone()),
            long: Cow::Owned(other.long.get_pattern().clone()),
            medium: Cow::Owned(other.medium.get_pattern().clone()),
            short: Cow::Owned(other.short.get_pattern().clone()),
        }
    }
}

impl From<&cldr_json::DateTimeFormats> for gregory::patterns::DateTimeFormatsV1 {
    fn from(other: &cldr_json::DateTimeFormats) -> Self {
        use gregory::patterns::{PatternV1, SkeletonV1, SkeletonsV1};
        use litemap::LiteMap;

        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            length_patterns: gregory::patterns::LengthPatternsV1 {
                full: Cow::Owned(other.full.get_pattern().clone()),
                long: Cow::Owned(other.long.get_pattern().clone()),
                medium: Cow::Owned(other.medium.get_pattern().clone()),
                short: Cow::Owned(other.short.get_pattern().clone()),
            },
            skeletons: {
                let mut skeletons = SkeletonsV1(LiteMap::new());

                // The CLDR keys for available_formats can have duplicate skeletons with either
                // an additional variant, or with multiple variants for different plurals.
                for (skeleton_str, pattern_str) in other.available_formats.0.iter() {
                    let mut unique_skeleton = None;
                    let mut variant_parts = Vec::new();

                    for part in skeleton_str.split('-') {
                        match unique_skeleton {
                            None => {
                                unique_skeleton = Some(part);
                            }
                            Some(_) => variant_parts.push(part),
                        }
                    }

                    let unique_skeleton = unique_skeleton.expect("Expected to find a skeleton.");

                    let skeleton_fields_v1 = match SkeletonV1::try_from(unique_skeleton) {
                        Ok(s) => s,
                        Err(err) => match err {
                            // Ignore unimplemented fields for now.
                            SkeletonError::SymbolUnimplemented(_) => continue,
                            _ => panic!("{:?} {}", unique_skeleton, err),
                        },
                    };

                    if !variant_parts.is_empty() {
                        eprintln!(
                            "This skeleton string is not yet supported: {:?}",
                            skeleton_str
                        );
                        continue;
                    }

                    let pattern_v1 = PatternV1::try_from(pattern_str as &str)
                        .expect("Unable to parse a pattern");

                    skeletons.0.insert(skeleton_fields_v1, pattern_v1);
                }

                skeletons
            },
        }
    }
}

impl From<&cldr_json::Dates> for gregory::DatesV1 {
    fn from(other: &cldr_json::Dates) -> Self {
        Self {
            symbols: gregory::DateSymbolsV1 {
                months: (&other.calendars.gregorian.months).into(),
                weekdays: (&other.calendars.gregorian.days).into(),
                day_periods: (&other.calendars.gregorian.day_periods).into(),
            },
            patterns: gregory::PatternsV1 {
                date: (&other.calendars.gregorian.date_formats).into(),
                time: (&other.calendars.gregorian.time_formats).into(),
                datetime: (&other.calendars.gregorian.datetime_formats).into(),
            },
        }
    }
}

macro_rules! symbols_from {
    ([$name: ident, $name2: ident $(,)?], [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl From<&cldr_json::$name::Symbols> for gregory::$name2::SymbolsV1 {
            fn from(other: &cldr_json::$name::Symbols) -> Self {
                Self([
                    $(
                        Cow::Owned(other.$element.clone()),
                    )*
                ])
            }
        }
        symbols_from!([$name, $name2]);
    };
    ([$name: ident, $name2: ident $(,)?], { $($element: ident),+ $(,)? } $(,)?) => {
        impl From<&cldr_json::$name::Symbols> for gregory::$name2::SymbolsV1 {
            fn from(other: &cldr_json::$name::Symbols) -> Self {
                Self {
                    $(
                        $element: other.$element.clone(),
                    )*
                }
            }
        }
        symbols_from!([$name, $name]);
    };
    ([$name: ident, $name2: ident]) => {
        impl cldr_json::$name::Symbols {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &Self) -> Option<Self> {
                if self == other {
                    None
                } else {
                    Some(self.clone())
                }
            }
        }

        impl From<&cldr_json::$name::Contexts> for gregory::$name2::ContextsV1 {
            fn from(other: &cldr_json::$name::Contexts) -> Self {
                Self {
                    format: (&other.format).into(),
                    stand_alone: other.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&other.format)
                    }).map(|ref stand_alone| stand_alone.into())
                }
            }
        }

        impl cldr_json::$name::StandAloneWidths {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &cldr_json::$name::FormatWidths) -> Option<Self> {
                let abbreviated = self.abbreviated.as_ref().and_then(|v| v.get_unaliased(&other.abbreviated));
                let narrow = self.narrow.as_ref().and_then(|v| v.get_unaliased(&other.narrow));
                let short = if self.short == other.short {
                    None
                } else {
                    self.short.clone()
                };
                let wide = self.wide.as_ref().and_then(|v| v.get_unaliased(&other.wide));

                if abbreviated.is_none() && narrow.is_none() && wide.is_none() && short.is_none() {
                    None
                } else {
                    Some(Self {
                        abbreviated,
                        narrow,
                        short,
                        wide,
                    })
                }
            }
        }

        impl From<&cldr_json::$name::FormatWidths> for gregory::$name2::FormatWidthsV1 {
            fn from(other: &cldr_json::$name::FormatWidths) -> Self {
                Self {
                    abbreviated: (&other.abbreviated).into(),
                    narrow: (&other.narrow).into(),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: (&other.wide).into(),
                }
            }
        }

        impl From<&cldr_json::$name::StandAloneWidths> for gregory::$name2::StandAloneWidthsV1 {
            fn from(other: &cldr_json::$name::StandAloneWidths) -> Self {
                Self {
                    abbreviated: other.abbreviated.as_ref().map(|width| width.into()),
                    narrow: other.narrow.as_ref().map(|width| width.into()),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: other.wide.as_ref().map(|width| width.into()),
                }
            }
        }
    };
}

symbols_from!(
    [months, months],
    [m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12],
);

symbols_from!([days, weekdays], [sun, mon, tue, wed, thu, fri, sat]);

symbols_from!(
    [
        day_periods,
        day_periods,
    ],
    {
        am,
        pm,
        noon,
        midnight,
    },
);

/// Serde structs for the CLDR JSON dates files.
pub(self) mod cldr_json {
    use crate::cldr_langid::CldrLangID;
    use serde::Deserialize;
    use std::borrow::Cow;

    macro_rules! symbols {
        ($name: ident, $([$alias: expr, $element: ident, $ty: ty]),+ $(,)?) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Deserialize)]
                pub struct Symbols {
                    $(
                        #[serde(rename = $alias)]
                        pub $element: $ty
                    ),*
                }

                symbols!();
            }
        };
        ($name: ident, $([$element: ident, $ty: ty]),+ $(,)?) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Deserialize)]
                pub struct Symbols {
                    $(pub $element: $ty),*
                }

                symbols!();
            }
        };
        () => {
            #[derive(Debug, PartialEq, Clone, Deserialize)]
            pub struct FormatWidths {
                pub abbreviated: Symbols,
                pub narrow: Symbols,
                pub short: Option<Symbols>,
                pub wide: Symbols,
            }

            #[derive(Debug, PartialEq, Clone, Deserialize)]
            pub struct StandAloneWidths {
                pub abbreviated: Option<Symbols>,
                pub narrow: Option<Symbols>,
                pub short: Option<Symbols>,
                pub wide: Option<Symbols>,
            }

            #[derive(Debug, PartialEq, Clone, Deserialize)]
            pub struct Contexts {
                pub format: FormatWidths,
                #[serde(rename = "stand-alone")]
                pub stand_alone: Option<StandAloneWidths>,
            }
        }
    }

    symbols!(
        months,
        ["1", m1, String],
        ["2", m2, String],
        ["3", m3, String],
        ["4", m4, String],
        ["5", m5, String],
        ["6", m6, String],
        ["7", m7, String],
        ["8", m8, String],
        ["9", m9, String],
        ["10", m10, String],
        ["11", m11, String],
        ["12", m12, String],
    );

    symbols!(
        days,
        [sun, String],
        [mon, String],
        [tue, String],
        [wed, String],
        [thu, String],
        [fri, String],
        [sat, String],
    );

    // The day period symbols are Cow<'static, str> instead of String because the Option
    // needs to be retained when converting them into Cow for the data provider.
    symbols!(
        day_periods,
        ["am", am, Cow<'static, str>],
        ["pm", pm, Cow<'static, str>],
        ["noon", noon, Option<Cow<'static, str>>],
        ["midnight", midnight, Option<Cow<'static, str>>],
    );

    #[derive(PartialEq, Debug, Deserialize)]
    #[serde(untagged)]
    pub enum LengthPattern {
        Plain(String),
        WithNumberingSystems {
            #[serde(rename = "_value")]
            pattern: String,
            #[serde(rename = "_numbers")]
            numbering_systems: String,
        },
    }

    impl LengthPattern {
        /// Get the pattern, dropping the numbering system if present.
        pub fn get_pattern(&self) -> &String {
            match self {
                Self::Plain(pattern) => pattern,
                Self::WithNumberingSystems { pattern, .. } => pattern,
            }
        }
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LengthPatterns {
        pub full: LengthPattern,
        pub long: LengthPattern,
        pub medium: LengthPattern,
        pub short: LengthPattern,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct DateTimeFormats {
        pub full: LengthPattern,
        pub long: LengthPattern,
        pub medium: LengthPattern,
        pub short: LengthPattern,
        #[serde(rename = "availableFormats")]
        pub available_formats: AvailableFormats,
    }

    #[derive(PartialEq, Clone, Debug, Deserialize)]
    pub struct AvailableFormats(
        #[serde(with = "tuple_vec_map")] pub(crate) Vec<(Cow<'static, str>, Cow<'static, str>)>,
    );

    /// This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key
    /// "main.LANGID.dates.calendars.gregorian" where "LANGID" is the identifier.
    ///
    /// e.g.
    /// https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json
    #[derive(PartialEq, Debug, Deserialize)]
    pub struct GregoryDates {
        pub months: months::Contexts,
        pub days: days::Contexts,
        #[serde(rename = "dayPeriods")]
        pub day_periods: day_periods::Contexts,
        #[serde(rename = "dateFormats")]
        pub date_formats: LengthPatterns,
        #[serde(rename = "timeFormats")]
        pub time_formats: LengthPatterns,
        #[serde(rename = "dateTimeFormats")]
        pub datetime_formats: DateTimeFormats,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Calendars {
        pub gregorian: GregoryDates,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Dates {
        pub calendars: Calendars,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangDates {
        pub dates: Dates,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangData(#[serde(with = "tuple_vec_map")] pub(crate) Vec<(CldrLangID, LangDates)>);

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub main: LangData,
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;
    use std::borrow::Cow;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: Cow<gregory::DatesV1> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("cs")),
                },
            },
        })
        .unwrap()
        .payload
        .take()
        .unwrap();

    assert_eq!("srpna", cs_dates.symbols.months.format.wide.0[7]);

    assert_eq!(
        "po",
        cs_dates.symbols.weekdays.format.short.as_ref().unwrap().0[1]
    );

    assert_eq!("d. M. y", cs_dates.patterns.date.medium);
}

#[test]
fn test_with_numbering_system() {
    use icu_locid_macros::langid;
    use std::borrow::Cow;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: Cow<gregory::DatesV1> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("haw")),
                },
            },
        })
        .unwrap()
        .payload
        .take()
        .unwrap();

    assert_eq!("d MMM y", cs_dates.patterns.date.medium);
    // TODO(#308): Support numbering system variations. We currently throw them away.
    assert_eq!("d/M/yy", cs_dates.patterns.date.short);
}

#[test]
fn unalias_contexts() {
    use icu_locid_macros::langid;
    use std::borrow::Cow;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: Cow<gregory::DatesV1> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("cs")),
                },
            },
        })
        .unwrap()
        .payload
        .take()
        .unwrap();

    // Czech months are not unaliased because `wide` differs.
    assert!(cs_dates.symbols.months.stand_alone.is_some());

    // Czech months are not unaliased because `wide` differs.
    assert!(cs_dates
        .symbols
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .abbreviated
        .is_none());
    assert!(cs_dates
        .symbols
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .short
        .is_none());
    assert!(cs_dates
        .symbols
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .narrow
        .is_none());
    assert!(cs_dates
        .symbols
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .wide
        .is_some());

    // Czech weekdays are unaliased because they completely overlap.
    assert!(cs_dates.symbols.weekdays.stand_alone.is_none());
}
