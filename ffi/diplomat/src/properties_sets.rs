// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

struct ErasedCodePointSetMarkerV1;

use icu_provider::DataMarker;
impl DataMarker for ErasedCodePointSetMarkerV1 {
    type Yokeable = <icu_properties::provider::AsciiHexDigitV1Marker as DataMarker>::Yokeable;
}

#[diplomat::bridge]
pub mod ffi {
    use super::ErasedCodePointSetMarkerV1;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_properties::sets;
    use icu_provider::prelude::DataPayload;

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu_properties, Mod)]
    pub struct ICU4XCodePointSetData(DataPayload<ErasedCodePointSetMarkerV1>);

    impl ICU4XCodePointSetData {
        /// Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_properties::sets::get_ascii_hex_digit, Fn)]
        pub fn try_get_ascii_hex_digit(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::get_ascii_hex_digit(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(DataPayload::cast(data))))
                .map_err(Into::into)
                .into()
        }

        /// Checks whether the code point is in the set.
        #[diplomat::rust_link(icu_uniset::UnicodeSet::contains, FnInStruct)]
        pub fn contains(&self, cp: char) -> bool {
            self.0.get().inv_list.contains(cp)
        }
    }
}
