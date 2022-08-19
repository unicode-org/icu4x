// @generated
#![cfg(feature = "icu_properties")]
type DataStruct = < :: icu_properties :: provider :: LogicalOrderExceptionV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    64u8, 14u8, 0u8, 0u8, 69u8, 14u8, 0u8, 0u8, 192u8, 14u8, 0u8, 0u8, 197u8, 14u8,
                    0u8, 0u8, 181u8, 25u8, 0u8, 0u8, 184u8, 25u8, 0u8, 0u8, 186u8, 25u8, 0u8, 0u8,
                    187u8, 25u8, 0u8, 0u8, 181u8, 170u8, 0u8, 0u8, 183u8, 170u8, 0u8, 0u8, 185u8,
                    170u8, 0u8, 0u8, 186u8, 170u8, 0u8, 0u8, 187u8, 170u8, 0u8, 0u8, 189u8, 170u8,
                    0u8, 0u8,
                ])
            },
            19usize,
        )
    });
