// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::DashV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    45u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 0u8, 138u8, 5u8, 0u8, 0u8, 139u8, 5u8,
                    0u8, 0u8, 190u8, 5u8, 0u8, 0u8, 191u8, 5u8, 0u8, 0u8, 0u8, 20u8, 0u8, 0u8, 1u8,
                    20u8, 0u8, 0u8, 6u8, 24u8, 0u8, 0u8, 7u8, 24u8, 0u8, 0u8, 16u8, 32u8, 0u8, 0u8,
                    22u8, 32u8, 0u8, 0u8, 83u8, 32u8, 0u8, 0u8, 84u8, 32u8, 0u8, 0u8, 123u8, 32u8,
                    0u8, 0u8, 124u8, 32u8, 0u8, 0u8, 139u8, 32u8, 0u8, 0u8, 140u8, 32u8, 0u8, 0u8,
                    18u8, 34u8, 0u8, 0u8, 19u8, 34u8, 0u8, 0u8, 23u8, 46u8, 0u8, 0u8, 24u8, 46u8,
                    0u8, 0u8, 26u8, 46u8, 0u8, 0u8, 27u8, 46u8, 0u8, 0u8, 58u8, 46u8, 0u8, 0u8,
                    60u8, 46u8, 0u8, 0u8, 64u8, 46u8, 0u8, 0u8, 65u8, 46u8, 0u8, 0u8, 93u8, 46u8,
                    0u8, 0u8, 94u8, 46u8, 0u8, 0u8, 28u8, 48u8, 0u8, 0u8, 29u8, 48u8, 0u8, 0u8,
                    48u8, 48u8, 0u8, 0u8, 49u8, 48u8, 0u8, 0u8, 160u8, 48u8, 0u8, 0u8, 161u8, 48u8,
                    0u8, 0u8, 49u8, 254u8, 0u8, 0u8, 51u8, 254u8, 0u8, 0u8, 88u8, 254u8, 0u8, 0u8,
                    89u8, 254u8, 0u8, 0u8, 99u8, 254u8, 0u8, 0u8, 100u8, 254u8, 0u8, 0u8, 13u8,
                    255u8, 0u8, 0u8, 14u8, 255u8, 0u8, 0u8, 173u8, 14u8, 1u8, 0u8, 174u8, 14u8,
                    1u8, 0u8,
                ])
            },
            30usize,
        )
    });
