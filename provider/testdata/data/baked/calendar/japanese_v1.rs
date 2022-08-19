// @generated
#![cfg(feature = "icu_calendar")]
type DataStruct =
    <::icu_calendar::provider::JapaneseErasV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct = &::icu_calendar::provider::JapaneseErasV1 {
    dates_to_eras: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[
            76u8, 7u8, 0u8, 0u8, 9u8, 8u8, 109u8, 101u8, 105u8, 106u8, 105u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 120u8, 7u8, 0u8, 0u8, 7u8, 30u8, 116u8, 97u8, 105u8,
            115u8, 104u8, 111u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 134u8, 7u8, 0u8,
            0u8, 12u8, 25u8, 115u8, 104u8, 111u8, 119u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 197u8, 7u8, 0u8, 0u8, 1u8, 8u8, 104u8, 101u8, 105u8, 115u8, 101u8,
            105u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 227u8, 7u8, 0u8, 0u8, 5u8,
            1u8, 114u8, 101u8, 105u8, 119u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8,
        ])
    },
};
