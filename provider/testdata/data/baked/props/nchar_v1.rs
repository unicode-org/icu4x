// @generated
#![cfg(feature = "icu_properties")]
type DataStruct = < :: icu_properties :: provider :: NoncharacterCodePointV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    208u8, 253u8, 0u8, 0u8, 240u8, 253u8, 0u8, 0u8, 254u8, 255u8, 0u8, 0u8, 0u8,
                    0u8, 1u8, 0u8, 254u8, 255u8, 1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 254u8, 255u8, 2u8,
                    0u8, 0u8, 0u8, 3u8, 0u8, 254u8, 255u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 254u8,
                    255u8, 4u8, 0u8, 0u8, 0u8, 5u8, 0u8, 254u8, 255u8, 5u8, 0u8, 0u8, 0u8, 6u8,
                    0u8, 254u8, 255u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 254u8, 255u8, 7u8, 0u8, 0u8,
                    0u8, 8u8, 0u8, 254u8, 255u8, 8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 254u8, 255u8, 9u8,
                    0u8, 0u8, 0u8, 10u8, 0u8, 254u8, 255u8, 10u8, 0u8, 0u8, 0u8, 11u8, 0u8, 254u8,
                    255u8, 11u8, 0u8, 0u8, 0u8, 12u8, 0u8, 254u8, 255u8, 12u8, 0u8, 0u8, 0u8, 13u8,
                    0u8, 254u8, 255u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 254u8, 255u8, 14u8, 0u8,
                    0u8, 0u8, 15u8, 0u8, 254u8, 255u8, 15u8, 0u8, 0u8, 0u8, 16u8, 0u8,
                ])
            },
            64usize,
        )
    });
