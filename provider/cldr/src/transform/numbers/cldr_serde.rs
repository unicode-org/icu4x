// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_langid::CldrLangID;
use itertools::Itertools;
use serde::Deserialize;
use serde_aux::prelude::*;
use std::collections::HashMap;
use tinystr::{TinyStr8, TinyStrAuto};

pub mod numbers_json {
    //! Serde structs representing CLDR JSON numbers.json files.
    //!
    //! Sample file:
    //! https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/numbers.json

    use super::*;

    use serde::de::{Deserialize, Deserializer, Error, MapAccess, Unexpected, Visitor};

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

    #[derive(PartialEq, Debug, Default)]
    pub struct NumberingSystemData {
        /// Map from numbering system to symbols
        pub symbols: HashMap<TinyStr8, Symbols>,
        /// Map from numbering system to decimal formats
        pub formats: HashMap<TinyStr8, DecimalFormats>,
    }

    pub struct NumberingSystemDataVisitor;

    impl<'de> Visitor<'de> for NumberingSystemDataVisitor {
        type Value = NumberingSystemData;

        // Format a message stating what data this Visitor expects to receive.
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("formatting data by numbering system")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut result = NumberingSystemData::default();
            while let Some(key) = access.next_key::<String>()? {
                // Key is of the form: "symbols-numberSystem-latn"
                let (stype, _, numsys) = match key.split('-').next_tuple() {
                    Some(v) => v,
                    None => continue, // Not what we were looking for; ignore.
                };
                let numsys: TinyStr8 = numsys.parse().map_err(|_| {
                    M::Error::invalid_value(Unexpected::Str(&key), &"numsys to be valid TinyStr8")
                })?;
                match stype {
                    "symbols" => {
                        let value: Symbols = access.next_value()?;
                        result.symbols.insert(numsys, value);
                    }
                    "decimalFormats" => {
                        let value: DecimalFormats = access.next_value()?;
                        result.formats.insert(numsys, value);
                    }
                    _ => {
                        // When needed, consume "scientificFormats", "percentFormats", ...
                        // For now, ignore them.
                    }
                }
            }
            Ok(result)
        }
    }

    impl<'de> Deserialize<'de> for NumberingSystemData {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(NumberingSystemDataVisitor)
        }
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Numbers {
        #[serde(rename = "defaultNumberingSystem")]
        pub default_numbering_system: TinyStr8,
        #[serde(rename = "minimumGroupingDigits")]
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub minimum_grouping_digits: u8,
        #[serde(flatten)]
        pub numsys_data: NumberingSystemData,
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

pub mod numbering_systems_json {
    //! Serde structs representing CLDR JSON numberingSystem.json files.
    //!
    //! Sample file:
    //! https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-core/supplemental/numberingSystems.json

    use super::*;

    #[derive(PartialEq, Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum NumberingSystemType {
        Numeric,
        Algorithmic,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct NumberingSystem {
        #[serde(rename = "_type")]
        pub nstype: NumberingSystemType,
        #[serde(rename = "_digits")]
        pub digits: Option<String>,
        #[serde(rename = "_rules")]
        pub rules: Option<TinyStrAuto>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct SupplementalData {
        #[serde(rename = "numberingSystems")]
        pub numbering_systems: HashMap<TinyStr8, NumberingSystem>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub supplemental: SupplementalData,
    }
}
