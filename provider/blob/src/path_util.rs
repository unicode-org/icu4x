// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

pub fn resource_path_to_string(resource_path: &ResourcePath) -> String {
    let key_components = resource_path.key.get_components();
    let opt_components = resource_path.options.get_components();
    let all_components: Vec<&str> = key_components.iter().chain(opt_components.iter()).collect();
    "/".to_string() + &all_components.join("/")
}
