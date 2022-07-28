// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_properties::{maps, Script};

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.
    #[diplomat::rust_link(icu::properties, Mod)]
    pub struct ICU4XCodePointMapData16(maps::CodePointMapData<Script>);

    pub struct ICU4XCodePointMapData16Response {
        /// The [`ICU4XCodePointMapData16`], if creation was successful.
        pub data: Option<Box<ICU4XCodePointMapData16>>,
        /// Whether creating the [`ICU4XCodePointMapData16`] was successful.
        pub success: bool,
    }

    impl ICU4XCodePointMapData16 {
        /// Gets a map for Unicode property Script from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::get_script, Fn)]
        pub fn try_get_script(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData16>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            maps::get_script(&provider)
                .map(|data| Box::new(ICU4XCodePointMapData16(data)))
                .map_err(Into::into)
                .into()
        }

        /// Gets the value for a code point.
        #[diplomat::rust_link(
            icu::codepointtrie::codepointtrie::CodePointTrie::get_u32,
            FnInStruct
        )]
        pub fn get(&self, cp: char) -> u16 {
            self.0.get(cp).0
        }
    }
}
