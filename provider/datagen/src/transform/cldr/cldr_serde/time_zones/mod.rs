// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod bcp47_tzid;
pub mod meta_zones;
pub mod time_zone_names;

use icu_datetime::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};
use litemap::LiteMap;
use meta_zones::ZonePeriod;
use time_zone_names::TimeZoneNames;

#[derive(Debug)]
pub struct CldrTimeZonesData {
    pub time_zone_names: TimeZoneNames,
    pub bcp47_tzids: LiteMap<String, TimeZoneBcp47Id>,
    pub meta_zone_ids: LiteMap<String, MetaZoneId>,
    pub meta_zone_periods: LiteMap<String, ZonePeriod>,
}
