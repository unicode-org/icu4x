// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_decimal::provider::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::SYMBOLS_V1, //
];

/// A data provider reading from CLDR JSON plural rule files.
#[derive(PartialEq, Debug)]
pub struct NumbersProvider {
    data: Vec<(CldrLangID, cldr_json::LangNumbers)>,
}

impl TryFrom<&dyn CldrPaths> for NumbersProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_numbers()?.join("main");

        let locale_dirs = get_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("numbers.json");

            let mut resource: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.append(&mut resource.main.0);
        }

        Ok(Self { data })
    }
}

impl TryFrom<&str> for NumbersProvider {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let mut resource: cldr_json::Resource =
            serde_json::from_str(input).map_err(|e| Error::Json(e, None))?;
        data.append(&mut resource.main.0);

        Ok(Self { data })
    }
}

impl KeyedDataProvider for NumbersProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::Decimal {
            return Err((&resc_key.category).into());
        }
        if resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d> DataProvider<'d, DecimalSymbolsV1> for NumbersProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, DecimalSymbolsV1>, DataError> {
        NumbersProvider::supports_key(&req.resource_path.key)?;
        let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
        let numbers = match self
            .data
            .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
        {
            Ok(idx) => &self.data[idx].1.numbers,
            Err(_) => return Err(DataError::UnavailableResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(Cow::Owned(DecimalSymbolsV1::try_from(numbers)?)),
        })
    }
}

icu_provider::impl_erased!(NumbersProvider, 'd);

impl<'d> IterableDataProvider<'d> for NumbersProvider {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        unimplemented!()
    }
}

impl TryFrom<&cldr_json::Numbers> for DecimalSymbolsV1 {
    type Error = DataError;

    fn try_from(other: &cldr_json::Numbers) -> Result<Self, Self::Error> {
        // TODO(#510): Select from non-default numbering systems
        let symbols: cldr_json::Symbols = serde_json::from_value(
            other
                .extra
                .get(&format!(
                    "symbols-numberSystem-{}",
                    other.default_numbering_system
                ))
                .ok_or("Could not find symbols for default numbering system")?
                .clone(),
        )
        .map_err(DataError::new_resc_error)?;
        let formats: cldr_json::DecimalFormats = serde_json::from_value(
            other
                .extra
                .get(&format!(
                    "decimalFormats-numberSystem-{}",
                    other.default_numbering_system
                ))
                .ok_or("Could not find formats for default numbering system")?
                .clone(),
        )
        .map_err(DataError::new_resc_error)?;
        let parsed_pattern: helpers::DecimalPatternParseResult = formats.standard.parse()?;

        Ok(Self {
            minus_sign_affixes: parsed_pattern.localize_sign(&symbols.minus_sign),
            plus_sign_affixes: parsed_pattern.localize_sign(&symbols.plus_sign),
            decimal_separator: symbols.decimal.into(),
            grouping_separator: symbols.group.into(),
            grouping_sizes: GroupingSizesV1 {
                primary: parsed_pattern.primary_grouping,
                secondary: parsed_pattern.secondary_grouping,
                min_grouping: other.minimum_grouping_digits,
            },
        })
    }
}

/// Serde structs for the CLDR JSON dates files.
pub(self) mod cldr_json {
    use crate::cldr_langid::CldrLangID;
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::HashMap;
    use tinystr::TinyStr8;

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Symbols {
        // This list is not comprehensive; add more fields when needed
        pub decimal: String,
        pub group: String,
        #[serde(rename = "minusSign")]
        pub minus_sign: String,
        #[serde(rename = "plusSign")]
        pub plus_sign: String,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct DecimalFormats {
        pub standard: String,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Numbers {
        #[serde(rename = "defaultNumberingSystem")]
        pub default_numbering_system: TinyStr8,
        #[serde(rename = "minimumGroupingDigits")]
        pub minimum_grouping_digits: u8,
        #[serde(flatten)]
        pub extra: HashMap<String, Value>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangNumbers {
        pub numbers: Numbers,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangData(#[serde(with = "tuple_vec_map")] pub(crate) Vec<(CldrLangID, LangNumbers)>);

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub main: LangData,
    }
}

/// Helper functions for the transformation
pub(self) mod helpers {
    use super::*;
    use std::str::FromStr;
    type SmallString8 = smallstr::SmallString<[u8; 8]>;

    pub struct DecimalPatternParseResult {
        pub unsigned_affixes: (SmallString8, SmallString8),
        pub signed_affixes: Option<(SmallString8, SmallString8)>,
        pub primary_grouping: u8,
        pub secondary_grouping: u8,
        pub min_fraction_digits: u8,
        pub max_fraction_digits: u8,
    }

    impl FromStr for DecimalPatternParseResult {
        type Err = &'static str;

        fn from_str(pattern: &str) -> Result<Self, Self::Err> {
            // Example patterns:
            // #,##,##0.###
            // #,##0.00;#,##0.00-
            // TODO
            Ok(Self {
                unsigned_affixes: ("".into(), "".into()),
                signed_affixes: None,
                primary_grouping: 3,
                secondary_grouping: 3,
                min_fraction_digits: 0,
                max_fraction_digits: 3,
            })
        }
    }

    impl DecimalPatternParseResult {
        pub fn localize_sign(&self, sign_str: &str) -> AffixesV1 {
            // UTS 35: the absence of a negative pattern means a single prefixed sign
            let signed_affixes = self
                .signed_affixes.clone()
                .unwrap_or_else(|| ("-".into(), "".into()));
            AffixesV1 {
                prefix: signed_affixes.0.replace("-", sign_str).into(),
                suffix: signed_affixes.1.replace("-", sign_str).into(),
            }
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;
    use std::borrow::Cow;

    let json_str = std::fs::read_to_string("tests/testdata/my-numbers.json").unwrap();
    let provider = NumbersProvider::try_from(json_str.as_str()).unwrap();

    println!("{:?}", provider);

    let my_decimal: Cow<DecimalSymbolsV1> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::SYMBOLS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("my")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    /*
    assert_eq!("srpna", cs_dates.symbols.months.format.wide.0[7]);

    assert_eq!(
        "po",
        cs_dates.symbols.weekdays.format.short.as_ref().unwrap().0[1]
    );

    assert_eq!("d. M. y", cs_dates.patterns.date.medium);
    */
}
