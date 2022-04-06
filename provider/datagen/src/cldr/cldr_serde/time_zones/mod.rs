// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod bcp47_tzid;
pub mod time_zone_names;

use litemap::LiteMap;
use time_zone_names::TimeZoneNames;

#[derive(Debug)]
pub struct CldrTimeZonesData<'a> {
    pub time_zone_names: &'a TimeZoneNames,
    pub bcp47_tzids: &'a LiteMap<String, String>,
}
