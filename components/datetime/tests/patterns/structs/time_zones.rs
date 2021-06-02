// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::{Deserialize, Serialize};
use tinystr::TinyStr8;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneConfig {
    pub time_zone_id: Option<String>,
    pub metazone_id: Option<String>,
    pub time_variant: Option<TinyStr8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTests(pub Vec<TimeZoneTest>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTest {
    pub locale: String,
    pub config: TimeZoneConfig,
    pub datetime: String,
    pub expectations: Vec<TimeZoneExpectation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneExpectation {
    pub patterns: Vec<String>,
    pub expected: String,
}
