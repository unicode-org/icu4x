// @generated
type DataStruct = & 'static < :: icu_properties :: provider :: IdsBinaryOperatorV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    240u8, 47u8, 0u8, 0u8, 242u8, 47u8, 0u8, 0u8, 244u8, 47u8, 0u8, 0u8, 252u8,
                    47u8, 0u8, 0u8,
                ])
            },
            10usize,
        )
    },
};
