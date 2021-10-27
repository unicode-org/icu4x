use icu_locid::LanguageIdentifier;
use serde::Deserialize;

pub mod list_patterns_json {
    //! Serde structs representing CLDR JSON numbers.json files.
    //!
    //! Sample file:
    //! https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-misc-full/main/en/listPatterns.json

    use super::*;

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct ListPattern {
        pub start: String,
        pub middle: String,
        pub end: String,
        #[serde(rename = "2")]
        pub pair: String,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct ListPatterns {
        #[serde(rename = "listPattern-type-standard")]
        pub standard: ListPattern,
        #[serde(rename = "listPattern-type-standard-narrow")]
        pub standard_narrow: ListPattern,
        #[serde(rename = "listPattern-type-standard-short")]
        pub standard_short: ListPattern,
        #[serde(rename = "listPattern-type-or")]
        pub or: ListPattern,
        #[serde(rename = "listPattern-type-or-narrow")]
        pub or_narrow: ListPattern,
        #[serde(rename = "listPattern-type-or-short")]
        pub or_short: ListPattern,
        #[serde(rename = "listPattern-type-unit")]
        pub unit: ListPattern,
        #[serde(rename = "listPattern-type-unit-narrow")]
        pub unit_narrow: ListPattern,
        #[serde(rename = "listPattern-type-unit-short")]
        pub unit_short: ListPattern,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangListPatterns {
        #[serde(rename = "listPatterns")]
        pub list_patterns: ListPatterns,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangData(
        #[serde(with = "tuple_vec_map")] pub(crate) Vec<(LanguageIdentifier, LangListPatterns)>,
    );

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub main: LangData,
    }
}
