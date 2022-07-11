// @generated
type DataStruct = & 'static < :: icu_collator :: provider :: CollationMetadataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[
    ("bn", BN_JA),
    ("es", ES_TR),
    ("ja", BN_JA),
    ("th", TH),
    ("tr", ES_TR),
    ("und", UND),
];
static BN_JA: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 41u32 };
static ES_TR: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 9u32 };
static TH: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 297u32 };
static UND: DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 1u32 };
