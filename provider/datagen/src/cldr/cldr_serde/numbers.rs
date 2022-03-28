// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/numbers.json>

use icu_locid::LanguageIdentifier;
use itertools::Itertools;
use litemap::LiteMap;
use serde::de::{Deserializer, Error, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use serde_aux::prelude::*;
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

#[derive(PartialEq, Debug, Default)]
pub struct NumberingSystemData {
    /// Map from numbering system to symbols
    pub symbols: LiteMap<TinyStr8, Symbols>,
    /// Map from numbering system to decimal formats
    pub formats: LiteMap<TinyStr8, DecimalFormats>,
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
pub struct LangData(pub LiteMap<LanguageIdentifier, LangNumbers>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
