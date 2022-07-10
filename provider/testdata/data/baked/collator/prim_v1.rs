// @generated
type DataStruct = & 'static < :: icu_collator :: provider :: CollationSpecialPrimariesV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct = &::icu_collator::provider::CollationSpecialPrimariesV1 {
    last_primaries: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[6u8, 5u8, 0u8, 12u8, 137u8, 13u8, 0u8, 14u8])
    },
    numeric_primary: 15u8,
};
