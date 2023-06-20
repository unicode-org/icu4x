// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;
use std::collections::HashMap;
use tinystr::TinyAsciiStr;

#[derive(PartialEq, Debug, Deserialize)]
pub enum Rtl {
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Metadata {
    pub rtl: Rtl,
}

// cldr-core/scriptMetadata.json
#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "scriptMetadata")]
    pub script_metadata: HashMap<TinyAsciiStr<4>, Metadata>,
}
