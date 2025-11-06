// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/windowsZones.json>

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct WindowsResource {
    pub(crate) supplemental: WindowsSupplemental,
}

#[derive(Deserialize)]
pub(crate) struct WindowsSupplemental {
    #[serde(rename = "windowsZones")]
    pub(crate) windows_zones: WindowsZones,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WindowsZones {
    #[serde(rename = "mapTimezones")]
    pub(crate) mapped_zones: Vec<MappedWindowsTimeZone>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MappedWindowsTimeZone {
    #[serde(rename = "mapZone")]
    pub(crate) map_zone: MapZone,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MapZone {
    #[serde(rename = "_other")]
    pub(crate) windows_id: String,
    #[serde(rename = "_type")]
    pub(crate) iana_identifier: String,
    #[serde(rename = "_territory")]
    pub(crate) territory: String,
}
