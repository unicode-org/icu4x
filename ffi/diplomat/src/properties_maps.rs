// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_properties::{
        maps::{self, CodePointMapResult},
        provider::ScriptV1Marker,
    };
    use icu_provider::prelude::DataPayload;

    use crate::provider::ffi::ICU4XDataProvider;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.
    #[diplomat::rust_link(icu_properties, Mod)]
    pub struct ICU4XCodePointMapData16(DataPayload<ScriptV1Marker>);

    pub struct ICU4XCodePointMapData16Response {
        /// The [`ICU4XCodePointMapData16`], if creation was successful.
        pub data: Option<Box<ICU4XCodePointMapData16>>,
        /// Whether creating the [`ICU4XCodePointMapData16`] was successful.
        pub success: bool,
    }

    impl ICU4XCodePointMapData16 {
        /// Gets a map for Unicode property Script from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_properties::maps::get_script, Fn)]
        pub fn try_get_script(provider: &ICU4XDataProvider) -> ICU4XCodePointMapData16Response {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            Self::prepare_result_from_script(maps::get_script(&provider))
        }

        fn prepare_result_from_script(
            result: CodePointMapResult<ScriptV1Marker>,
        ) -> ICU4XCodePointMapData16Response {
            match result {
                Ok(data) => ICU4XCodePointMapData16Response {
                    data: Some(Box::new(ICU4XCodePointMapData16(data))),
                    success: true,
                },
                Err(_) => ICU4XCodePointMapData16Response {
                    data: None,
                    success: false,
                },
            }
        }

        /// Gets the value for a code point.
        #[diplomat::rust_link(icu_codepointtrie::codepointtrie::CodePointTrie::get_u32, FnInStruct)]
        pub fn get(&self, cp: char) -> u16 {
            self.0.get().code_point_trie.get(cp.into()).0
        }
    }
}
