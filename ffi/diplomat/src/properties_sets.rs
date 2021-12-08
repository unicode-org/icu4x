// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_properties::{
        provider::UnicodePropertyV1Marker,
        sets::{self, UnisetResult},
    };
    use icu_provider::prelude::DataPayload;

    use crate::{provider::ffi::ICU4XDataProvider, provider::ffi::ICU4XStaticDataProvider};

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html) for more information.
    pub struct ICU4XCodePointSetData(DataPayload<UnicodePropertyV1Marker>);

    pub struct ICU4XCodePointSetDataResult {
        /// The [`ICU4XCodePointSetData`], if creation was successful.
        pub data: Option<Box<ICU4XCodePointSetData>>,
        /// Whether creating the [`ICU4XCodePointSetData`] was successful.
        pub success: bool,
    }

    impl ICU4XCodePointSetData {
        /// Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html) for more information.
        pub fn try_get_ascii_hex_digit(
            provider: &ICU4XDataProvider,
        ) -> ICU4XCodePointSetDataResult {
            let provider = provider.0.as_ref().as_dyn_serde_provider();
            Self::prepare_result(sets::get_ascii_hex_digit(&provider))
        }

        /// Gets a set for Unicode property ascii_hex_digit from a [`ICU4XStaticDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html) for more information.
        pub fn try_get_ascii_hex_digit_from_static(
            provider: &ICU4XStaticDataProvider,
        ) -> ICU4XCodePointSetDataResult {
            let provider = provider.0.as_ref();
            Self::prepare_result(sets::get_ascii_hex_digit(provider))
        }

        fn prepare_result(result: UnisetResult) -> ICU4XCodePointSetDataResult {
            match result {
                Ok(data) => ICU4XCodePointSetDataResult {
                    data: Some(Box::new(ICU4XCodePointSetData(data))),
                    success: true,
                },
                Err(_) => ICU4XCodePointSetDataResult {
                    data: None,
                    success: false,
                },
            }
        }

        /// Checks whether the code point is in the set.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains) for more information.
        pub fn contains(&self, cp: char) -> bool {
            self.0.get().inv_list.contains(cp)
        }
    }
}
