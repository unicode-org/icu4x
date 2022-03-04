// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::ResourceKey;

pub fn get_all_keys() -> Vec<ResourceKey> {
    // TODO(#1512): Use central key repository
    let mut v = vec![];
    v.extend(icu_provider_cldr::ALL_KEYS);
    v.extend(icu_properties::provider::key::ALL_MAP_KEYS);
    v.extend(icu_properties::provider::key::ALL_SCRIPT_EXTENSIONS_KEYS);
    v.extend(icu_properties::provider::key::ALL_SET_KEYS);
    v.extend(icu_segmenter::ALL_KEYS);
    v
}
