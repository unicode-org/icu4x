// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod binary {
    #[derive(serde::Deserialize)]
    pub struct BinaryProperty {
        pub long_name: String,
        #[serde(skip)]
        pub short_name: String,
        pub ranges: Vec<(u32, u32)>,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        #[serde(default)]
        pub binary_property: Vec<BinaryProperty>,
        #[serde(skip)]
        pub enum_property: (),
        #[serde(skip)]
        pub script_extensions: (),
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

    #[allow(clippy::upper_case_acronyms)]
    #[derive(serde::Deserialize)]
    pub struct EnumeratedPropertyCodePointTrie {
        #[serde(skip)]
        pub short_name: String,
        #[serde(skip)]
        pub long_name: String,
        #[serde(skip)]
        pub name: String,
        pub index: Vec<u16>,
        pub data_8: Option<Vec<u8>>,
        pub data_16: Option<Vec<u16>>,
        pub data_32: Option<Vec<u32>>,
        #[serde(skip)]
        pub index_length: u32,
        #[serde(skip)]
        pub data_length: u32,
        #[serde(rename = "highStart")]
        pub high_start: u32,
        #[serde(rename = "shifted12HighStart")]
        pub shifted12_high_start: u16,
        #[serde(rename = "type")]
        pub trie_type_enum_val: u8,
        #[serde(rename = "valueWidth")]
        pub value_width_enum_val: u8,
        #[serde(rename = "index3NullOffset")]
        pub index3_null_offset: u16,
        #[serde(rename = "dataNullOffset")]
        pub data_null_offset: u32,
        #[serde(rename = "nullValue")]
        pub null_value: u32,
    }

    #[derive(serde::Deserialize)]
    pub struct EnumeratedPropertyMap {
        pub long_name: String,
        pub short_name: String,
        pub ranges: Vec<EnumeratedPropertyMapRange>,
        pub code_point_trie: EnumeratedPropertyCodePointTrie,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        #[serde(skip)]
        pub binary_property: (),
        #[serde(default)]
        pub enum_property: Vec<EnumeratedPropertyMap>,
        #[serde(skip)]
        pub script_extensions: (),
    }
}

pub mod script_extensions {
    use super::enumerated::EnumeratedPropertyCodePointTrie;

    #[derive(serde::Deserialize)]
    pub struct ScriptWithExtensionsProperty {
        pub long_name: String,
        pub short_name: String,
        pub script_code_array: Vec<Vec<u16>>,
        pub code_point_trie: EnumeratedPropertyCodePointTrie,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        #[serde(skip)]
        pub binary_property: (),
        #[serde(skip)]
        pub enum_property: (),
        #[serde(default)]
        pub script_extensions: Vec<ScriptWithExtensionsProperty>,
    }
}

#[cfg(feature = "casemap")]
pub mod case {
    #[derive(serde::Deserialize)]
    pub struct Exceptions {
	pub exceptions: Vec<u16>,
    }

    #[derive(serde::Deserialize)]
    pub struct Unfold {
	pub unfold: Vec<u16>,
    }

    #[derive(serde::Deserialize)]
    pub struct Level1 {
	pub code_point_trie: super::enumerated::EnumeratedPropertyCodePointTrie,
	pub exceptions: Exceptions,
	pub unfold: Unfold
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        pub ucase: Level1,
    }
}
