// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationMetadataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("bn", BN_JA),
        ("es", ES_ES_U_CO_TRAD_TR),
        ("es-u-co-trad", ES_ES_U_CO_TRAD_TR),
        ("ja", BN_JA),
        ("th", TH),
        ("tr", ES_ES_U_CO_TRAD_TR),
        ("und", UND),
    ]);
static BN_JA: &DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 41u32 };
static ES_ES_U_CO_TRAD_TR: &DataStruct =
    &::icu_collator::provider::CollationMetadataV1 { bits: 9u32 };
static TH: &DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 297u32 };
static UND: &DataStruct = &::icu_collator::provider::CollationMetadataV1 { bits: 1u32 };
