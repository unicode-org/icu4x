// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::BidiControlV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    28u8, 6u8, 0u8, 0u8, 29u8, 6u8, 0u8, 0u8, 14u8, 32u8, 0u8, 0u8, 16u8, 32u8,
                    0u8, 0u8, 42u8, 32u8, 0u8, 0u8, 47u8, 32u8, 0u8, 0u8, 102u8, 32u8, 0u8, 0u8,
                    106u8, 32u8, 0u8, 0u8,
                ])
            },
            12usize,
        )
    });
