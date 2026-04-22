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

#[derive(serde::Deserialize)]
pub(crate) struct PropertyValue<T> {
    pub(crate) discr: T,
    pub(crate) short: String,
}

pub(crate) mod enumerated {
    #[derive(serde::Deserialize)]
    pub(crate) struct EnumeratedPropertyMap {
        pub(crate) long_name: String,
        pub(crate) short_name: String,
        pub(crate) values: Vec<super::PropertyValue<u16>>,
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
