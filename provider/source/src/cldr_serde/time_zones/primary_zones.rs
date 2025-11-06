// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON primaryZones.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/primaryZones.json>

use icu::locale::subtags::Region;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "primaryZones")]
    pub(crate) primary_zones: BTreeMap<Region, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
