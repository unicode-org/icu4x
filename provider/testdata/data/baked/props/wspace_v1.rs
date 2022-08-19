// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::WhiteSpaceV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    9u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 33u8, 0u8, 0u8,
                    0u8, 133u8, 0u8, 0u8, 0u8, 134u8, 0u8, 0u8, 0u8, 160u8, 0u8, 0u8, 0u8, 161u8,
                    0u8, 0u8, 0u8, 128u8, 22u8, 0u8, 0u8, 129u8, 22u8, 0u8, 0u8, 0u8, 32u8, 0u8,
                    0u8, 11u8, 32u8, 0u8, 0u8, 40u8, 32u8, 0u8, 0u8, 42u8, 32u8, 0u8, 0u8, 47u8,
                    32u8, 0u8, 0u8, 48u8, 32u8, 0u8, 0u8, 95u8, 32u8, 0u8, 0u8, 96u8, 32u8, 0u8,
                    0u8, 0u8, 48u8, 0u8, 0u8, 1u8, 48u8, 0u8, 0u8,
                ])
            },
            25usize,
        )
    });
