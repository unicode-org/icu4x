// @generated
type DataStruct =
    &'static <::icu_properties::provider::HexDigitV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    48u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 65u8, 0u8, 0u8, 0u8, 71u8, 0u8, 0u8,
                    0u8, 97u8, 0u8, 0u8, 0u8, 103u8, 0u8, 0u8, 0u8, 16u8, 255u8, 0u8, 0u8, 26u8,
                    255u8, 0u8, 0u8, 33u8, 255u8, 0u8, 0u8, 39u8, 255u8, 0u8, 0u8, 65u8, 255u8,
                    0u8, 0u8, 71u8, 255u8, 0u8, 0u8,
                ])
            },
            44usize,
        )
    },
};
