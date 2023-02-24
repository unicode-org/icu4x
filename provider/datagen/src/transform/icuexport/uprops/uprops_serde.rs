// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::toml::CodePointTrieToml;

pub mod binary {
    #[derive(serde::Deserialize)]
    pub struct BinaryProperty {
        pub long_name: String,
        #[serde(skip)]
        pub short_name: String,
        pub ranges: Vec<(u32, u32)>,
        pub strings: Option<Vec<String>>,
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

    #[derive(serde::Deserialize)]
    pub struct EnumeratedPropertyValue {
        pub discr: u16,
        pub long: String,
        pub short: Option<String>,
        #[serde(default)]
        pub aliases: Vec<String>,
    }

    #[derive(serde::Deserialize)]
    pub struct EnumeratedPropertyMap {
        pub long_name: String,
        pub short_name: String,
        pub values: Vec<EnumeratedPropertyValue>,
        pub ranges: Vec<EnumeratedPropertyMapRange>,
        pub code_point_trie: super::CodePointTrieToml,
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
    use super::CodePointTrieToml;

    #[derive(serde::Deserialize)]
    pub struct ScriptWithExtensionsPropertyV1Property {
        pub long_name: String,
        pub short_name: String,
        pub script_code_array: Vec<Vec<u16>>,
        pub code_point_trie: CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub struct Main {
        #[serde(skip)]
        pub binary_property: (),
        #[serde(skip)]
        pub enum_property: (),
        #[serde(default)]
        pub script_extensions: Vec<ScriptWithExtensionsPropertyV1Property>,
    }
}
