// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locid::LanguageIdentifier;
use serde::{Deserialize, Serialize};

pub mod key {
    use crate::resource::ResourceKey;
    pub const LIKELY_SUBTAGS_V1: ResourceKey = resource_key!(likelysubtags, "likelysubtags", 1);
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct LikelySubtagsV1 {
    pub entries: Vec<(LanguageIdentifier, LanguageIdentifier)>,
}
