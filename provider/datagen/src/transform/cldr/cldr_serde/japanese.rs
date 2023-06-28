// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;
use std::collections::HashMap;

// cldr-core/supplemental/calendarData.json
#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "calendarData")]
    pub calendar_data: CalendarDatas,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct CalendarDatas {
    pub japanese: CalendarData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct CalendarData {
    pub eras: HashMap<String, EraStart>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct EraStart {
    #[serde(rename = "_start")]
    pub start: Option<String>,
}
