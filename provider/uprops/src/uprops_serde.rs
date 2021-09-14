// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod binary {
    #[derive(serde::Deserialize)]
    pub struct BinaryProperty {
        pub long_name: String,
        pub short_name: String,
        pub ranges: Vec<(u32, u32)>,
    }

    #[derive(serde::Deserialize)]
    pub struct Level1 {
        pub data: BinaryProperty,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        pub binary_property: Level1,
    }
}

pub mod enumerated {
    #[derive(serde::Deserialize)]
    pub struct EnumeratedPropertyMapRange {
        pub a: u32,
        pub b: u32,
        pub v: u32,
        pub name: String,
    }

    #[derive(serde::Deserialize)]
    pub struct EnumeratedPropertyMap {
        pub long_name: String,
        pub short_name: String,
        pub ranges: Vec<EnumeratedPropertyMapRange>,
    }

    #[derive(serde::Deserialize)]
    pub struct Level1 {
        pub data: EnumeratedPropertyMap,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        pub enum_property: Level1,
        // omitted: enum_property.code_point_trie
    }
}
