// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON person name files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-person-names-full/main/en/personNames.json>

use litemap::LiteMap;

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct FormalityFormatting(pub(crate) LiteMap<String, String>);

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct ReferringFormatting(pub(crate) LiteMap<String, FormalityFormatting>);

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct SizedFormatting(pub(crate) LiteMap<String, ReferringFormatting>);

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct OrderFormatting(pub(crate) LiteMap<String, SizedFormatting>);

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct PersonNames {
    #[serde(rename = "givenFirst")]
    pub(crate) given_first: Vec<String>,
    #[serde(rename = "surnameFirst")]
    pub(crate) surname_first: Vec<String>,
    #[serde(rename = "foreignSpaceReplacement")]
    pub(crate) foreign_space_replacement: String,
    pub(crate) initial: String,
    #[serde(rename = "initialSequence")]
    pub(crate) initial_sequence: String,
    #[serde(rename = "personName")]
    pub(crate) formatting_pattern: OrderFormatting,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct CldrData {
    #[serde(rename = "personNames")]
    pub(crate) person_names: PersonNames,
}

pub(crate) type Resource = super::super::LocaleResource<CldrData>;
