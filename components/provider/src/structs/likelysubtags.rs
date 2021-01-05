// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
#[cfg(feature = "invariant")]
use crate::prelude::*;
use icu_locid::LanguageIdentifier;
use serde::{Deserialize, Serialize};

pub mod key {
    use crate::data_key::DataKey;
    pub const LIKELY_SUBTAGS_V1: DataKey = data_key!(likelysubtags, "likelysubtags", 1);
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct LikelySubtagsV1 {
    pub entries: Vec<(LanguageIdentifier, LanguageIdentifier)>,
}

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    match *data_key {
        key::LIKELY_SUBTAGS_V1 => make_inv_response::<LikelySubtagsV1>(),
        _ => None,
    }
}
