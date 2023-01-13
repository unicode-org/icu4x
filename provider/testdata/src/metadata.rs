// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::vec::Vec;
use icu_locid::LanguageIdentifier;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceMetadata {
    pub locales: Vec<LanguageIdentifier>,
    pub cldr_json_gitref: String,
    pub icuexportdata_gitref: String,
}

pub fn load() -> SourceMetadata {
    #[allow(clippy::unwrap_used)] // the TOML source is a constant
    toml::from_str(include_str!("../metadata.toml")).unwrap()
}
