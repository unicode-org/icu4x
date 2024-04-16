// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::toml::CodePointTrieToml;

pub(in crate::provider) mod binary {
    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct BinaryProperty {
        pub(in crate::provider) long_name: String,
        #[serde(skip)]
        pub(in crate::provider) short_name: String,
        pub(in crate::provider) ranges: Vec<(u32, u32)>,
        pub(in crate::provider) strings: Option<Vec<String>>,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct Main {
        #[serde(default)]
        pub(in crate::provider) binary_property: Vec<BinaryProperty>,
    }
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct PropertyValue {
    pub(in crate::provider) discr: u32,
    pub(in crate::provider) long: String,
    pub(in crate::provider) short: Option<String>,
    #[serde(default)]
    pub(in crate::provider) aliases: Vec<String>,
}

pub(in crate::provider) mod enumerated {
    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct EnumeratedPropertyMapRange {
        pub(in crate::provider) a: u32,
        pub(in crate::provider) b: u32,
        pub(in crate::provider) v: u32,
        pub(in crate::provider) name: String,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct EnumeratedPropertyMap {
        pub(in crate::provider) long_name: String,
        pub(in crate::provider) short_name: String,
        pub(in crate::provider) values: Vec<super::PropertyValue>,
        pub(in crate::provider) ranges: Vec<EnumeratedPropertyMapRange>,
        pub(in crate::provider) code_point_trie: super::CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct Main {
        #[serde(default)]
        pub(in crate::provider) enum_property: Vec<EnumeratedPropertyMap>,
    }
}

pub(in crate::provider) mod code_point_prop {
    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct CodePointPropertyMapRange {
        pub(in crate::provider) a: u32,
        pub(in crate::provider) b: u32,
        pub(in crate::provider) v: u32,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct CodePointPropertyMap {
        pub(in crate::provider) long_name: String,
        pub(in crate::provider) short_name: String,
        pub(in crate::provider) ranges: Vec<CodePointPropertyMapRange>,
        pub(in crate::provider) code_point_trie: super::CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct Main {
        // TODO: update icuexportdata to print a different TOML header than "enum_property"
        #[serde(default)]
        pub(in crate::provider) enum_property: Vec<CodePointPropertyMap>,
    }
}

pub(in crate::provider) mod mask {
    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct MaskPropertyMap {
        pub(in crate::provider) long_name: String,
        pub(in crate::provider) short_name: String,
        pub(in crate::provider) mask_for: String,
        pub(in crate::provider) values: Vec<super::PropertyValue>,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct Main {
        #[serde(default)]
        pub(in crate::provider) mask_property: Vec<MaskPropertyMap>,
    }
}

pub(in crate::provider) mod script_extensions {
    use super::CodePointTrieToml;

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct ScriptWithExtensionsPropertyV1Property {
        pub(in crate::provider) long_name: String,
        pub(in crate::provider) short_name: String,
        pub(in crate::provider) script_code_array: Vec<Vec<u16>>,
        pub(in crate::provider) code_point_trie: CodePointTrieToml,
    }

    #[derive(serde::Deserialize)]
    pub(in crate::provider) struct Main {
        #[serde(default)]
        pub(in crate::provider) script_extensions: Vec<ScriptWithExtensionsPropertyV1Property>,
    }
}
