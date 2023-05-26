// @generated
#![cfg(feature = "icu_segmenter")]
type DataStruct = < icu_segmenter :: provider :: DictionaryForWordLineExtendedV1Marker as icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    icu_provider::DataLocale::from(icu_locid::locale!("th"))
        .eq(locale)
        .then(|| &TH)
}
static TH: DataStruct = include!("th.rs.data");
