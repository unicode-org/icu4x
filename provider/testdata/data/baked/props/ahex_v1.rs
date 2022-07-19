// @generated
type DataStruct = & 'static < :: icu_properties :: provider :: AsciiHexDigitV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::CodePointSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    48u8, 0u8, 0u8, 0u8, 58u8, 0u8, 0u8, 0u8, 65u8, 0u8, 0u8, 0u8, 71u8, 0u8, 0u8,
                    0u8, 97u8, 0u8, 0u8, 0u8, 103u8, 0u8, 0u8, 0u8,
                ])
            },
            22usize,
        )
    });
