// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod binary {
    #[derive(serde::Deserialize)]
    pub(crate) struct BinaryProperty {
        pub(crate) long_name: String,
        pub(crate) short_name: Option<String>,
        pub(crate) ranges: Vec<(u32, u32)>,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        pub(crate) binary_property: Vec<BinaryProperty>,
    }
}
