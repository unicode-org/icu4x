// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{fieldset::dynamic::TimeZoneStyleWithLength, neo_skeleton::{NeoSkeletonLength, NeoTimeZoneStyle}};
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

pub fn pattern_to_semantic_skeleton(p: &str) -> Option<TimeZoneStyleWithLength> {
    Some(match p {
        "vvvv" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Generic,
            NeoSkeletonLength::Long,
        ),
        "v" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Generic,
            NeoSkeletonLength::Short,
        ),
        "VVVV" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Location,
            NeoSkeletonLength::Long,
        ),
        "zzzz" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Specific,
            NeoSkeletonLength::Long,
        ),
        "z" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Specific,
            NeoSkeletonLength::Short,
        ),
        "OOOO" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Offset,
            NeoSkeletonLength::Long,
        ),
        "O" => TimeZoneStyleWithLength::from_style_and_length(
            NeoTimeZoneStyle::Offset,
            NeoSkeletonLength::Short,
        ),
        _ => return None,
    })
}
