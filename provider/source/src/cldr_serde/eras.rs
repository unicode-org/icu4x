// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::calendar::provider::EraStartDate;
use serde::{de::Error, Deserialize, Deserializer};
use std::{borrow::Cow, collections::BTreeMap};

// cldr-core/supplemental/calendarData.json
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "calendarData")]
    pub(crate) calendar_data: BTreeMap<String, CalendarData>,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct CalendarData {
    #[serde(default)]
    pub(crate) eras: BTreeMap<String, EraData>,
    #[serde(rename = "inheritEras")]
    pub(crate) inherit_eras: Option<InheritEras>,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct InheritEras {
    #[serde(rename = "_calendar")]
    pub(crate) calendar: String,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct EraData {
    #[serde(rename = "_start", default, deserialize_with = "parse_era_start_date")]
    pub(crate) start: Option<EraStartDate>,
    #[serde(rename = "_end", default, deserialize_with = "parse_era_start_date")]
    pub(crate) end: Option<EraStartDate>,
    #[serde(rename = "_code")]
    pub(crate) code: Option<String>,
    #[serde(rename = "_aliases")]
    pub(crate) aliases: Option<String>,
    /// EraYear::era_index
    #[serde(skip)]
    pub(crate) icu4x_era_index: Option<u8>,
}

fn parse_era_start_date<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<EraStartDate>, D::Error> {
    let s = Cow::<str>::deserialize(de)?;
    let mut s = &*s;
    let sign = if let Some(suffix) = s.strip_prefix('-') {
        s = suffix;
        -1
    } else {
        1
    };

    let mut split = s.split('-');
    let year = split
        .next()
        .ok_or(D::Error::custom("EraStartData format"))?
        .parse::<i32>()
        .map_err(|_| D::Error::custom("EraStartData format"))?
        * sign;
    let month = split
        .next()
        .ok_or(D::Error::custom("EraStartData format"))?
        .parse()
        .map_err(|_| D::Error::custom("EraStartData format"))?;
    let day = split
        .next()
        .ok_or(D::Error::custom("EraStartData format"))?
        .parse()
        .map_err(|_| D::Error::custom("EraStartData format"))?;

    Ok(Some(EraStartDate { year, month, day }))
}
