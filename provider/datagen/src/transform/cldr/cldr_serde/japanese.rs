// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;
use std::collections::HashMap;

// cldr-core/supplemental/calendarData.json
#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Supplemental {
    #[serde(rename = "calendarData")]
    pub(in crate::provider) calendar_data: CalendarDatas,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct CalendarDatas {
    pub(in crate::provider) japanese: CalendarData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct CalendarData {
    pub(in crate::provider) eras: HashMap<String, EraStart>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct EraStart {
    #[serde(rename = "_start")]
    pub(in crate::provider) start: Option<String>,
}
