// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod binary {
    #[derive(serde::Deserialize)]
    pub struct BinaryProperty {
        pub long_name: String,
        pub name: String,
        pub serialized: Vec<u32>,
        pub ranges: Vec<(u32, u32)>,
    }

    #[derive(serde::Deserialize)]
    pub struct Level1 {
        pub data: BinaryProperty,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        pub unicode_set: Level1,
    }
}

pub mod enumerated {
    // TODO
}
