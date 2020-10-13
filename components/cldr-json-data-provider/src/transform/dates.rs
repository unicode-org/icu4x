// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::support::DataKeySupport;
use crate::CldrPaths;
use icu_data_provider::iter::DataEntryCollection;
use icu_data_provider::prelude::*;
use icu_data_provider::structs::dates::*;
use std::convert::TryFrom;
use std::marker::PhantomData;

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

impl TryFrom<&str> for DatesProvider<'_> {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let mut resource: cldr_json::Resource =
            serde_json::from_str(input).map_err(|e| Error::Json(e, None))?;
        data.append(&mut resource.main.0);

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> DataKeySupport for DatesProvider<'d> {
    fn supports_key(data_key: &DataKey) -> Result<(), DataError> {
        if data_key.category != DataCategory::Dates {
            return Err((&data_key.category).into());
        }
        if data_key.version != 1 {
            return Err(data_key.into());
        }
        Ok(())
    }
}

impl<'d> DatesProvider<'d> {
    fn get_dates_for(
        &self,
        data_key: &DataKey,
        langid: &CldrLangID,
    ) -> Result<&cldr_json::Dates, DataError> {
        DatesProvider::supports_key(data_key)?;

        if let Ok(idx) = self.data.binary_search_by_key(&langid, |(lid, _)| lid) {
            Ok(&self.data[idx].1.dates)
        } else {
            Err(DataError::UnsupportedDataKey(*data_key))
        }
    }
}

impl<'d> DataProvider<'d> for DatesProvider<'d> {
    fn load(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError> {
        let cldr_langid = req.data_entry.langid.clone().into();

        let dates = self.get_dates_for(&req.data_key, &cldr_langid)?;
        Ok(DataResponseBuilder {
            data_langid: req.data_entry.langid.clone(),
        }
        .with_owned_payload(gregory::DatesV1::from(dates)))
    }
}

impl<'d> DataEntryCollection for DatesProvider<'d> {
    fn iter_for_key(
        &self,
        _data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, DataError> {
        let list: Vec<DataEntry> = self
            .data
            .iter()
            .map(|(l, _)| DataEntry {
                variant: None,
                // TODO: Avoid the clone
                langid: l.langid.clone(),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_json::StylePatterns> for gregory::patterns::StylePatternsV1 {
    fn from(other: &cldr_json::StylePatterns) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other.full.get_pattern().clone(),
            long: other.long.get_pattern().clone(),
            medium: other.medium.get_pattern().clone(),
            short: other.short.get_pattern().clone(),
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
                date_time: (&other.calendars.gregorian.date_time_formats).into(),
            },
        }
    }
}

macro_rules! symbols_from {
    ([$name: ident, $name2: ident], [ $($element: ident),* ]) => {
        impl From<&cldr_json::$name::Symbols> for gregory::$name2::SymbolsV1 {
            fn from(other: &cldr_json::$name::Symbols) -> Self {
                Self([
                    $(
                        other.$element.clone(),
                    )*
                ])
            }
        }
        symbols_from!([$name, $name2]);
    };
    ([$name: ident, $name2: ident], { $($element: ident),* }) => {
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
            pub fn get_unaliased(&self, other: &cldr_json::$name::Symbols) -> Option<Self> {
                if self != other {
                    Some(self.clone())
                } else {
                    None
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
    [m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12]
);

symbols_from!([days, weekdays], [sun, mon, tue, wed, thu, fri, sat]);

symbols_from!(
    [
        day_periods,
        day_periods
    ],
    {
        am,
        pm
    }
);

/// Serde structs for the CLDR JSON dates files.
pub(self) mod cldr_json {
    use crate::cldr_langid::CldrLangID;
    use serde::Deserialize;
    use std::borrow::Cow;

    macro_rules! symbols {
        ($name: ident, $([$alias: expr, $element: ident, $ty: ty]),*) => {
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
        ($name: ident, $([$element: ident, $ty: ty]),*) => {
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
        ["1", m1, Cow<'static, str>],
        ["2", m2, Cow<'static, str>],
        ["3", m3, Cow<'static, str>],
        ["4", m4, Cow<'static, str>],
        ["5", m5, Cow<'static, str>],
        ["6", m6, Cow<'static, str>],
        ["7", m7, Cow<'static, str>],
        ["8", m8, Cow<'static, str>],
        ["9", m9, Cow<'static, str>],
        ["10", m10, Cow<'static, str>],
        ["11", m11, Cow<'static, str>],
        ["12", m12, Cow<'static, str>]
    );

    symbols!(
        days,
        [sun, Cow<'static, str>],
        [mon, Cow<'static, str>],
        [tue, Cow<'static, str>],
        [wed, Cow<'static, str>],
        [thu, Cow<'static, str>],
        [fri, Cow<'static, str>],
        [sat, Cow<'static, str>]
    );

    symbols!(
        day_periods,
        ["am", am, Cow<'static, str>],
        ["pm", pm, Cow<'static, str>]
    );

    #[derive(PartialEq, Debug, Deserialize)]
    #[serde(untagged)]
    pub enum StylePattern {
        Plain(Cow<'static, str>),
        WithNumberingSystems {
            #[serde(rename = "_value")]
            pattern: Cow<'static, str>,
            #[serde(rename = "_numbers")]
            numbering_systems: Cow<'static, str>,
        },
    }

    impl StylePattern {
        /// Get the pattern, dropping the numbering system if present.
        pub fn get_pattern(&self) -> &Cow<'static, str> {
            match self {
                Self::Plain(pattern) => pattern,
                Self::WithNumberingSystems { pattern, .. } => pattern,
            }
        }
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct StylePatterns {
        pub full: StylePattern,
        pub long: StylePattern,
        pub medium: StylePattern,
        pub short: StylePattern,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct GregoryDates {
        pub months: months::Contexts,
        pub days: days::Contexts,
        #[serde(rename = "dayPeriods")]
        pub day_periods: day_periods::Contexts,
        #[serde(rename = "dateFormats")]
        pub date_formats: StylePatterns,
        #[serde(rename = "timeFormats")]
        pub time_formats: StylePatterns,
        #[serde(rename = "dateTimeFormats")]
        pub date_time_formats: StylePatterns,
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
    use icu_locale_macros::langid;
    use std::borrow::Cow;

    let json_str = std::fs::read_to_string("tests/testdata/cs-ca-gregorian.json").unwrap();
    let provider = DatesProvider::try_from(json_str.as_str()).unwrap();

    let cs_dates: Cow<gregory::DatesV1> = provider
        .load(&DataRequest {
            data_key: icu_data_key!(dates: gregory@1),
            data_entry: DataEntry {
                variant: None,
                langid: langid!("cs"),
            },
        })
        .unwrap()
        .take_payload()
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
    use icu_locale_macros::langid;
    use std::borrow::Cow;

    let json_str = std::fs::read_to_string("tests/testdata/haw-ca-gregorian.json").unwrap();
    let provider = DatesProvider::try_from(json_str.as_str()).unwrap();

    let cs_dates: Cow<gregory::DatesV1> = provider
        .load(&DataRequest {
            data_key: icu_data_key!(dates: gregory@1),
            data_entry: DataEntry {
                variant: None,
                langid: langid!("haw"),
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!("d MMM y", cs_dates.patterns.date.medium);
    // TODO(#308): Support numbering system variations. We currently throw them away.
    assert_eq!("d/M/yy", cs_dates.patterns.date.short);
}

#[test]
fn unalias_contexts() {
    use icu_locale_macros::langid;
    use std::borrow::Cow;

    let json_str = std::fs::read_to_string("tests/testdata/cs-ca-gregorian.json").unwrap();
    let provider = DatesProvider::try_from(json_str.as_str()).unwrap();

    let cs_dates: Cow<gregory::DatesV1> = provider
        .load(&DataRequest {
            data_key: icu_data_key!(dates: gregory@1),
            data_entry: DataEntry {
                variant: None,
                langid: langid!("cs"),
            },
        })
        .unwrap()
        .take_payload()
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
