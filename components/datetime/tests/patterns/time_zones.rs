// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::neo_skeleton::{NeoSkeletonLength, NeoTimeZoneSkeleton, NeoTimeZoneStyle};
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

pub fn pattern_to_semantic_skeleton(p: &str) -> Option<NeoTimeZoneSkeleton> {
    Some(match p {
        "vvvv" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Long,
            NeoTimeZoneStyle::Generic,
        ),
        "v" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Short,
            NeoTimeZoneStyle::Generic,
        ),
        "VVVV" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Long,
            NeoTimeZoneStyle::Location,
        ),
        "zzzz" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Long,
            NeoTimeZoneStyle::Specific,
        ),
        "z" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Short,
            NeoTimeZoneStyle::Specific,
        ),
        "OOOO" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Long,
            NeoTimeZoneStyle::Offset,
        ),
        "O" => NeoTimeZoneSkeleton::for_length_and_components(
            NeoSkeletonLength::Short,
            NeoTimeZoneStyle::Offset,
        ),
        _ => return None,
    })
}
