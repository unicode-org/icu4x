// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_decimal::provider::*;
use icu_provider::prelude::*;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::SYMBOLS_V1, //
];

/// A data provider reading from CLDR JSON plural rule files.
#[derive(PartialEq, Debug)]
pub struct NumbersProvider {
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

        unimplemented!()
        // Ok(Self {
        //     data,
        // })
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
        unimplemented!()
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

/// Serde structs for the CLDR JSON dates files.
pub(self) mod cldr_json {
    use crate::cldr_langid::CldrLangID;
    use serde::Deserialize;
    use tinystr::TinyStr8;

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Numbers {
        pub default_numbering_system: TinyStr8,
        pub minimum_grouping_dicits: TinyStr8,

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
