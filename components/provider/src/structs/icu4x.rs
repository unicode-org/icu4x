// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

// ICU4X metadata information

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod key {
    use crate::resource::ResourceKey;
    pub const HELLO_V1: ResourceKey = resource_key!(icu4x, "hello", 1);
}

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct HelloV1<'s> {
    #[serde(borrow)]
    pub hello: Cow<'s, str>,
}

impl Default for HelloV1<'_> {
    fn default() -> Self {
        HelloV1 {
            hello: Cow::Borrowed("(und) Hello World"),
        }
    }
}
