// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON person name files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-person-names-full/main/en/personNames.json>

use litemap::LiteMap;
use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct FormalityFormatting(pub(in crate::provider) LiteMap<String, String>);

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct ReferringFormatting(
    pub(in crate::provider) LiteMap<String, FormalityFormatting>,
);

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct SizedFormatting(
    pub(in crate::provider) LiteMap<String, ReferringFormatting>,
);

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct OrderFormatting(
    pub(in crate::provider) LiteMap<String, SizedFormatting>,
);

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct PersonNames {
    #[serde(rename = "givenFirst")]
    pub(in crate::provider) given_first: Vec<String>,
    #[serde(rename = "surnameFirst")]
    pub(in crate::provider) surname_first: Vec<String>,
    #[serde(rename = "foreignSpaceReplacement")]
    pub(in crate::provider) foreign_space_replacement: String,
    pub(in crate::provider) initial: String,
    #[serde(rename = "initialSequence")]
    pub(in crate::provider) initial_sequence: String,
    #[serde(rename = "personName")]
    pub(in crate::provider) formatting_pattern: OrderFormatting,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct CldrData {
    #[serde(rename = "personNames")]
    pub(in crate::provider) person_names: PersonNames,
}

pub(in crate::provider) type Resource = super::super::LocaleResource<CldrData>;
