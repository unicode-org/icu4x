// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::UnifiedIdeographV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    0u8, 52u8, 0u8, 0u8, 192u8, 77u8, 0u8, 0u8, 0u8, 78u8, 0u8, 0u8, 0u8, 160u8,
                    0u8, 0u8, 14u8, 250u8, 0u8, 0u8, 16u8, 250u8, 0u8, 0u8, 17u8, 250u8, 0u8, 0u8,
                    18u8, 250u8, 0u8, 0u8, 19u8, 250u8, 0u8, 0u8, 21u8, 250u8, 0u8, 0u8, 31u8,
                    250u8, 0u8, 0u8, 32u8, 250u8, 0u8, 0u8, 33u8, 250u8, 0u8, 0u8, 34u8, 250u8,
                    0u8, 0u8, 35u8, 250u8, 0u8, 0u8, 37u8, 250u8, 0u8, 0u8, 39u8, 250u8, 0u8, 0u8,
                    42u8, 250u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 224u8, 166u8, 2u8, 0u8, 0u8, 167u8,
                    2u8, 0u8, 58u8, 183u8, 2u8, 0u8, 64u8, 183u8, 2u8, 0u8, 30u8, 184u8, 2u8, 0u8,
                    32u8, 184u8, 2u8, 0u8, 162u8, 206u8, 2u8, 0u8, 176u8, 206u8, 2u8, 0u8, 225u8,
                    235u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 75u8, 19u8, 3u8, 0u8, 80u8, 19u8, 3u8,
                    0u8, 176u8, 35u8, 3u8, 0u8,
                ])
            },
            97058usize,
        )
    });
