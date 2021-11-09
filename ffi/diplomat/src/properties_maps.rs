// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_properties::{
        maps::{self, CodePointMapResult},
        provider::UnicodePropertyMapV1Marker,
        Script,
    };
    use icu_provider::prelude::DataPayload;

    use crate::{provider::ffi::ICU4XDataProvider, provider::ffi::ICU4XStaticDataProvider};

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html) for more information.
    pub struct ICU4XUnicodeScriptMapProperty(
        DataPayload<'static, UnicodePropertyMapV1Marker<Script>>,
    );

    pub struct ICU4XUnicodeScriptMapPropertyResult {
        /// The [`ICU4XUnicodeScriptMapProperty`], if creation was successful.
        pub data: Option<Box<ICU4XUnicodeScriptMapProperty>>,
        /// Whether creating the [`ICU4XUnicodeScriptMapProperty`] was successful.
        pub success: bool,
    }

    impl ICU4XUnicodeScriptMapProperty {
        /// Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html) for more information.
        pub fn try_get(provider: &ICU4XDataProvider) -> ICU4XUnicodeScriptMapPropertyResult {
            let provider = provider.0.as_ref();
            Self::prepare_result(maps::get_script(provider))
        }

        /// Gets a set for Unicode property ascii_hex_digit from a [`ICU4XStaticDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html) for more information.
        pub fn try_get_from_static(
            provider: &ICU4XStaticDataProvider,
        ) -> ICU4XUnicodeScriptMapPropertyResult {
            let provider = provider.0.as_ref();
            Self::prepare_result(maps::get_script(provider))
        }

        fn prepare_result(
            result: CodePointMapResult<'static, Script>,
        ) -> ICU4XUnicodeScriptMapPropertyResult {
            match result {
                Ok(data) => ICU4XUnicodeScriptMapPropertyResult {
                    data: Some(Box::new(ICU4XUnicodeScriptMapProperty(data))),
                    success: true,
                },
                Err(_) => ICU4XUnicodeScriptMapPropertyResult {
                    data: None,
                    success: false,
                },
            }
        }

        /// Gets the Script for a code point.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32) for more information.
        pub fn get(&self, cp: char) -> u32 {
            self.0.get().code_point_trie.get(cp.into()).0.into()
        }
    }
}
