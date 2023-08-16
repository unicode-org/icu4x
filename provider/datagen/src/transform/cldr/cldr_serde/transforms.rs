// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(PartialEq, Debug)]
pub enum Direction {
    Forward,
    Backward,
    Both,
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            "both" => Ok(Self::Both),
            _ => Err(D::Error::custom("unknown direction")),
        }
    }
}

// cldr-transforms-full/main/<lang>/metadata.json
#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub direction: Direction,
    pub source: String,
    pub target: String,
    pub alias: Vec<String>,
}
