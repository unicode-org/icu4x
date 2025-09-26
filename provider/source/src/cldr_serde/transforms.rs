// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Direction {
    Forward,
    Backward,
    Both,
}

#[derive(PartialEq, Debug, Default, Copy, Clone, serde_derive::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Visibility {
    Internal,
    #[default]
    External,
}

// cldr-transforms/transforms/<lang>.json
#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct Resource {
    #[serde(rename = "_rulesFile")]
    pub(crate) rules_file: String,
    #[serde(rename = "_direction")]
    pub(crate) direction: Direction,
    #[serde(rename = "_visibility", default)]
    pub(crate) visibility: Visibility,
    #[serde(rename = "_source")]
    pub(crate) source: Option<String>,
    #[serde(rename = "_target")]
    pub(crate) target: Option<String>,
    #[serde(rename = "_variant")]
    pub(crate) variant: Option<String>,
    #[serde(rename = "_alias", default)]
    pub(crate) alias: Option<String>,
    #[serde(rename = "_backwardAlias", default)]
    pub(crate) backward_alias: Option<String>,
    #[serde(rename = "_aliasBcp47", default)]
    pub(crate) alias_bcp47: Option<String>,
    #[serde(rename = "_backwardAliasBcp47", default)]
    pub(crate) backward_alias_bcp47: Option<String>,
}
