// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

// ICU4X metadata information

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod key {
    use crate::resource::ResourceKey;
    pub const HELLO_WORLD_V1: ResourceKey = resource_key!(icu4x, "helloworld", 1);
}

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct HelloWorldV1<'s> {
    #[serde(borrow)]
    pub message: Cow<'s, str>,
}

impl Default for HelloWorldV1<'_> {
    fn default() -> Self {
        HelloWorldV1 {
            message: Cow::Borrowed("(und) Hello World"),
        }
    }
}
