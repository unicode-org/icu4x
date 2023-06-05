// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON person name files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-person-names-full/main/en/personNames.json>

use std::collections::HashMap;

use serde::Deserialize;

use icu_locid::LanguageIdentifier;
use indexmap::IndexMap;

#[derive(PartialEq, Debug, Deserialize)]
pub struct FormalityFormatting(
    /// IndexMap is used here since the order infile matters.
    pub IndexMap<String, String>,
);

#[derive(PartialEq, Debug, Deserialize)]
pub struct ReferringFormatting(pub HashMap<String, FormalityFormatting>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct SizedFormatting(pub HashMap<String, ReferringFormatting>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct OrderFormatting(pub HashMap<String, SizedFormatting>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct PersonNames {
    #[serde(rename = "givenFirst")]
    pub given_first: Vec<String>,
    #[serde(rename = "surnameFirst")]
    pub surname_first: Vec<String>,
    #[serde(rename = "foreignSpaceReplacement")]
    pub foreign_space_replacement: String,
    pub initial: String,
    #[serde(rename = "initialSequence")]
    pub initial_sequence: String,
    #[serde(rename = "personName")]
    pub formatting_pattern: OrderFormatting,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct CldrData {
    #[serde(rename = "personNames")]
    pub person_names: PersonNames,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(pub HashMap<LanguageIdentifier, CldrData>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
