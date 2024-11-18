// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{
    fieldset::{self, dynamic::ZoneFieldSet},
    options::NeoSkeletonLength,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTests(pub Vec<TimeZoneTest>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTest {
    pub locale: String,
    pub datetime: String,
    pub expectations: HashMap<String, String>,
}

pub fn pattern_to_semantic_skeleton(p: &str) -> Option<ZoneFieldSet> {
    Some(match p {
        "vvvv" => ZoneFieldSet::V(fieldset::V::with_length(NeoSkeletonLength::Long)),
        "v" => ZoneFieldSet::V(fieldset::V::with_length(NeoSkeletonLength::Short)),
        "VVVV" => ZoneFieldSet::L(fieldset::L::with_length(NeoSkeletonLength::Long)),
        "zzzz" => ZoneFieldSet::Z(fieldset::Z::with_length(NeoSkeletonLength::Long)),
        "z" => ZoneFieldSet::Z(fieldset::Z::with_length(NeoSkeletonLength::Short)),
        "OOOO" => ZoneFieldSet::O(fieldset::O::with_length(NeoSkeletonLength::Long)),
        "O" => ZoneFieldSet::O(fieldset::O::with_length(NeoSkeletonLength::Short)),
        _ => return None,
    })
}
