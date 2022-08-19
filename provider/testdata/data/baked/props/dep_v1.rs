// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::DeprecatedV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    73u8, 1u8, 0u8, 0u8, 74u8, 1u8, 0u8, 0u8, 115u8, 6u8, 0u8, 0u8, 116u8, 6u8,
                    0u8, 0u8, 119u8, 15u8, 0u8, 0u8, 120u8, 15u8, 0u8, 0u8, 121u8, 15u8, 0u8, 0u8,
                    122u8, 15u8, 0u8, 0u8, 163u8, 23u8, 0u8, 0u8, 165u8, 23u8, 0u8, 0u8, 106u8,
                    32u8, 0u8, 0u8, 112u8, 32u8, 0u8, 0u8, 41u8, 35u8, 0u8, 0u8, 43u8, 35u8, 0u8,
                    0u8, 1u8, 0u8, 14u8, 0u8, 2u8, 0u8, 14u8, 0u8,
                ])
            },
            15usize,
        )
    });
