// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::RadicalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    128u8, 46u8, 0u8, 0u8, 154u8, 46u8, 0u8, 0u8, 155u8, 46u8, 0u8, 0u8, 244u8,
                    46u8, 0u8, 0u8, 0u8, 47u8, 0u8, 0u8, 214u8, 47u8, 0u8, 0u8,
                ])
            },
            329usize,
        )
    });
