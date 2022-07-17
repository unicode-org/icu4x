// @generated
type DataStruct = & 'static < :: icu_properties :: provider :: JoinControlV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct = &::icu_properties::provider::PropertyUnicodeSetV1::InversionList(unsafe {
    #[allow(unused_unsafe)]
    ::icu_uniset::UnicodeSet::from_parts_unchecked(
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[12u8, 32u8, 0u8, 0u8, 14u8, 32u8, 0u8, 0u8])
        },
        2usize,
    )
});
