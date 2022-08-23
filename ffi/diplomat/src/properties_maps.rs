// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_collections::codepointtrie::TrieValue;
    use icu_properties::{maps, PropertiesError};

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    use icu_provider::prelude::BufferProvider;
    use icu_provider::serde::{AsDeserializingBufferProvider, DeserializingBufferProvider};

    #[diplomat::opaque]
    /// An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
    ///
    /// For properties whose values fit into 8 bits.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapData, Struct)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed, Struct)]
    pub struct ICU4XCodePointMapData8(maps::CodePointMapData<u8>);

    impl ICU4XCodePointMapData8 {
        /// Gets a map for Unicode property General_Category from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_general_category, Fn)]
        pub fn try_get_general_category(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_general_category(p))
        }

        /// Gets a map for Unicode property Bidi_Class from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_bidi_class, Fn)]
        pub fn try_get_bidi_class(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_bidi_class(p))
        }

        /// Gets a map for Unicode property East_Asian_Width from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_east_asian_width, Fn)]
        pub fn try_get_east_asian_width(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_east_asian_width(p))
        }

        /// Gets a map for Unicode property Line_Break from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_line_break, Fn)]
        pub fn try_get_line_break(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_line_break(p))
        }

        /// Gets a map for Unicode property Grapheme_Cluster_Break from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_grapheme_cluster_break, Fn)]
        pub fn try_grapheme_cluster_break(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_grapheme_cluster_break(p))
        }

        /// Gets a map for Unicode property Word_Break from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_word_break, Fn)]
        pub fn try_get_word_break(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_word_break(p))
        }

        /// Gets a map for Unicode property Sentence_Break from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_sentence_break, Fn)]
        pub fn try_get_sentence_break(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError> {
            Self::try_get_prop_inner(provider, |p| maps::load_sentence_break(p))
        }

        fn try_get_prop_inner<P, F>(
            provider: &ICU4XDataProvider,
            f: F,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData8>, ICU4XError>
        where
            P: TrieValue + 'static,
            F: FnOnce(
                &DeserializingBufferProvider<'_, dyn BufferProvider>,
            ) -> Result<maps::CodePointMapData<P>, PropertiesError>,
        {
            let provider = provider.0.as_deserializing();
            #[allow(clippy::expect_used)] // infallible
            f(&provider)
                .map(|data| {
                    Box::new(ICU4XCodePointMapData8(
                        data.try_into_converted()
                            .expect("try_into_converted to u8 must be infallible"),
                    ))
                })
                .map_err(Into::into)
                .into()
        }

        /// Gets the value for a code point.
        #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed::get, FnInStruct)]
        pub fn get(&self, cp: char) -> u8 {
            self.0.as_borrowed().get(cp)
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
    ///
    /// For properties whose values fit into 16 bits.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapData, Struct)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed, Struct)]
    pub struct ICU4XCodePointMapData16(maps::CodePointMapData<u16>);

    impl ICU4XCodePointMapData16 {
        /// Gets a map for Unicode property Script from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu::properties::maps::load_script, Fn)]
        pub fn try_get_script(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointMapData16>, ICU4XError> {
            let provider = provider.0.as_deserializing();
            #[allow(clippy::expect_used)] // infallible
            maps::load_script(&provider)
                .map(|data| {
                    Box::new(ICU4XCodePointMapData16(
                        data.try_into_converted()
                            .expect("try_into_converted to u16 must be infallible"),
                    ))
                })
                .map_err(Into::into)
                .into()
        }

        /// Gets the value for a code point.
        #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed::get, FnInStruct)]
        pub fn get(&self, cp: char) -> u16 {
            self.0.as_borrowed().get(cp)
        }
    }
}
