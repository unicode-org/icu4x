// @generated
type DataStruct = & 'static < :: icu_properties :: provider :: RegionalIndicatorV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::CodePointSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    230u8, 241u8, 1u8, 0u8, 0u8, 242u8, 1u8, 0u8,
                ])
            },
            26usize,
        )
    });
