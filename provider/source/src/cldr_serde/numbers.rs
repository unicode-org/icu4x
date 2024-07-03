// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/numbers.json>

use itertools::Itertools;
use serde::de::{Deserializer, Error, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use tinystr::TinyStr8;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Symbols {
    // This list is not comprehensive; add more fields when needed
    pub(crate) decimal: String,
    pub(crate) group: String,
    #[serde(rename = "minusSign")]
    pub(crate) minus_sign: String,
    #[serde(rename = "plusSign")]
    pub(crate) plus_sign: String,
    #[serde(rename = "percentSign")]
    pub(crate) percent_sign: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct DecimalFormats {
    pub(crate) standard: String,
    pub(crate) long: DecimalFormatLength,
    pub(crate) short: DecimalFormatLength,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct DecimalFormatLength {
    #[serde(rename = "decimalFormat")]
    pub(crate) decimal_format: DecimalFormat,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) struct DecimalFormat {
    pub(crate) patterns: Vec<CompactDecimalPattern>,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) struct CompactDecimalPattern {
    pub(crate) compact_decimal_type: String,
    pub(crate) compact_decimal_count: String,
    pub(crate) pattern: String,
}

impl<'de> Deserialize<'de> for DecimalFormat {
    fn deserialize<D>(deserializer: D) -> Result<DecimalFormat, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DecimalFormatVisitor)
    }
}

struct DecimalFormatVisitor;
impl<'de> Visitor<'de> for DecimalFormatVisitor {
    type Value = DecimalFormat;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map from keys of the form 10*-count-(zero|one|few|many|other) to compact decimal patterns")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = DecimalFormat::default();
        while let Some(key) = access.next_key::<String>()? {
            let (compact_decimal_type, compact_decimal_count) =
                key.split("-count-").next_tuple().ok_or_else(|| {
                    M::Error::invalid_value(Unexpected::Str(&key), &"key to contain -count-")
                })?;
            result.patterns.push(CompactDecimalPattern {
                compact_decimal_type: compact_decimal_type.to_string(),
                compact_decimal_count: compact_decimal_count.to_string(),
                pattern: access.next_value()?,
            })
        }
        Ok(result)
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct CurrencyFormattingPatterns {
    /// Standard pattern
    pub(crate) standard: String,

    /// Standard alphaNextToNumber pattern
    #[serde(rename = "standard-alphaNextToNumber")]
    pub(crate) standard_alpha_next_to_number: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct PercentFormattingPatterns {
    /// Standard pattern
    pub(crate) standard: String,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) struct NumberingSystemData {
    /// Map from numbering system to symbols
    pub(crate) symbols: HashMap<TinyStr8, Symbols>,
    /// Map from numbering system to decimal formats
    pub(crate) formats: HashMap<TinyStr8, DecimalFormats>,
    /// Map from numbering system to patterns
    pub(crate) currency_patterns: HashMap<TinyStr8, CurrencyFormattingPatterns>,
    /// Map from numbering system to percent patterns
    pub(crate) percent_patterns: HashMap<TinyStr8, PercentFormattingPatterns>,
}

pub(crate) struct NumberingSystemDataVisitor;

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
                "currencyFormats" => {
                    let value: CurrencyFormattingPatterns = access.next_value()?;
                    result.currency_patterns.insert(numsys, value);
                }
                "percentFormats" => {
                    let value: PercentFormattingPatterns = access.next_value()?;
                    result.percent_patterns.insert(numsys, value);
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
pub(crate) struct Numbers {
    #[serde(rename = "defaultNumberingSystem")]
    pub(crate) default_numbering_system: TinyStr8,
    #[serde(rename = "minimumGroupingDigits")]
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub(crate) minimum_grouping_digits: u8,
    #[serde(flatten)]
    pub(crate) numsys_data: NumberingSystemData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangNumbers {
    pub(crate) numbers: Numbers,
}

pub(crate) type Resource = super::LocaleResource<LangNumbers>;
