// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::calendar::provider::EraStartDate;
use serde::{de::Error, Deserialize, Deserializer};
use std::{borrow::Cow, collections::HashMap};

// cldr-core/supplemental/calendarData.json
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "calendarData")]
    pub(crate) calendar_data: CalendarDatas,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct CalendarDatas {
    pub(crate) japanese: CalendarData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct CalendarData {
    pub(crate) eras: HashMap<String, EraStart>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct EraStart {
    #[serde(rename = "_start", deserialize_with = "parse_era_start_date")]
    pub(crate) start: Option<EraStartDate>,
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
