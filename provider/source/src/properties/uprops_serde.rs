// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

pub(crate) mod binary {
    #[derive(serde::Deserialize)]
    pub(crate) struct BinaryProperty {
        #[serde(rename = "long_name")]
        pub(crate) _long_name: String,
        #[serde(skip)]
        #[serde(rename = "short_name")]
        pub(crate) _short_name: String,
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
pub(crate) struct PropertyValue {
    pub(crate) discr: u32,
    pub(crate) long: String,
    pub(crate) short: Option<String>,
    #[serde(default)]
    pub(crate) aliases: Vec<String>,
}

pub(crate) mod enumerated {
    #[derive(serde::Deserialize)]
    pub(crate) struct EnumeratedPropertyMapRange {
        #[serde(rename = "a")]
        pub(crate) _a: u32,
        #[serde(rename = "b")]
        pub(crate) _b: u32,
        #[serde(rename = "v")]
        pub(crate) _v: u32,
        #[serde(rename = "name")]
        pub(crate) _name: String,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct EnumeratedPropertyMap {
        #[serde(rename = "long_name")]
        pub(crate) _long_name: String,
        #[serde(rename = "short_name")]
        pub(crate) _short_name: String,
        pub(crate) values: Vec<super::PropertyValue>,
        #[serde(rename = "ranges")]
        pub(crate) _ranges: Vec<EnumeratedPropertyMapRange>,
        pub(crate) code_point_trie: super::CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        #[serde(default)]
        pub(crate) enum_property: Vec<EnumeratedPropertyMap>,
    }
}

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
pub(crate) mod code_point_prop {
    #[derive(serde::Deserialize)]
    pub(crate) struct CodePointPropertyMap {
        pub(crate) code_point_trie: super::CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        // TODO: update icuexportdata to print a different TOML header than "enum_property"
        #[serde(default)]
        pub(crate) enum_property: Vec<CodePointPropertyMap>,
    }
}

pub(crate) mod mask {
    #[derive(serde::Deserialize)]
    pub(crate) struct MaskPropertyMap {
        #[serde(rename = "long_name")]
        pub(crate) _long_name: String,
        #[serde(rename = "short_name")]
        pub(crate) _short_name: String,
        #[serde(rename = "mask_for")]
        pub(crate) _mask_for: String,
        pub(crate) values: Vec<super::PropertyValue>,
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
        #[serde(rename = "long_name")]
        pub(crate) _long_name: String,
        #[serde(rename = "short_name")]
        pub(crate) _short_name: String,
        pub(crate) script_code_array: Vec<Vec<u16>>,
        pub(crate) code_point_trie: CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct Main {
        #[serde(default)]
        pub(crate) script_extensions: Vec<ScriptWithExtensionsPropertyProperty>,
    }
}
