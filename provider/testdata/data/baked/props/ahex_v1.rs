// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::AsciiHexDigitV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    48u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 65u8, 0u8, 0u8, 0u8, 71u8, 0u8, 0u8,
                    0u8, 97u8, 0u8, 0u8, 0u8, 103u8, 0u8, 0u8, 0u8,
                ])
            },
            22usize,
        )
    });
