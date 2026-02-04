// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

pub(crate) mod binary {
    #[derive(serde::Deserialize)]
    pub(crate) struct BinaryProperty {
        pub(crate) long_name: String,
        pub(crate) short_name: Option<String>,
        pub(crate) ranges: Vec<(u32, u32)>,
        pub(crate) strings: Option<Vec<String>>,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        #[serde(default)]
        pub(crate) binary_property: Vec<BinaryProperty>,
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct PropertyValue<T> {
    pub(crate) discr: T,
    pub(crate) long: String,
    pub(crate) short: Option<String>,
    #[serde(default)]
    pub(crate) aliases: Vec<String>,
}

pub(crate) mod enumerated {
    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    pub(crate) struct EnumeratedPropertyMapRange<T> {
        pub(crate) a: u32,
        pub(crate) b: u32,
        pub(crate) v: T,
        pub(crate) name: Option<String>,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct EnumeratedPropertyMap {
        pub(crate) long_name: String,
        pub(crate) short_name: String,
        #[serde(default)]
        pub(crate) values: Vec<super::PropertyValue<u16>>,
        pub(crate) ranges: Vec<EnumeratedPropertyMapRange<u16>>,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        #[serde(default)]
        pub(crate) enum_property: Vec<EnumeratedPropertyMap>,
    }
}

pub(crate) mod mask {
    #[derive(serde::Deserialize)]
    pub(crate) struct MaskPropertyMap {
        pub(crate) long_name: String,
        pub(crate) short_name: String,
        pub(crate) mask_for: String,
        pub(crate) values: Vec<super::PropertyValue<u32>>,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        #[serde(default)]
        pub(crate) mask_property: Vec<MaskPropertyMap>,
    }
}

pub(crate) mod script_extensions {
    use super::CodePointTrieToml;

    #[derive(serde::Deserialize)]
    pub(crate) struct ScriptWithExtensionsPropertyProperty {
        pub(crate) long_name: String,
        pub(crate) short_name: String,
        pub(crate) script_code_array: Vec<Vec<u16>>,
        pub(crate) code_point_trie: CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        #[serde(default)]
        pub(crate) script_extensions: Vec<ScriptWithExtensionsPropertyProperty>,
    }
}
