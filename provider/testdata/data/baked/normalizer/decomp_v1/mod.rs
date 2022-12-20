// @generated
#![cfg(feature = "icu_normalizer")]
type DataStruct = < :: icu_normalizer :: provider :: NonRecursiveDecompositionSupplementV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static DATA: &DataStruct = &include!("und.rs.data");
    locale.is_empty().then(|| DATA)
}
