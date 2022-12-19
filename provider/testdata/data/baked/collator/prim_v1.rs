// @generated
#![cfg(feature = "icu_collator")]
type DataStruct = < :: icu_collator :: provider :: CollationSpecialPrimariesV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct = &::icu_collator::provider::CollationSpecialPrimariesV1 {
    last_primaries: unsafe {
        ::zerovec::ZeroVec::from_bytes_unchecked(&[6u8, 5u8, 0u8, 12u8, 138u8, 13u8, 0u8, 14u8])
    },
    numeric_primary: 15u8,
};
