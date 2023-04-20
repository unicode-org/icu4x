// @generated
#![cfg(feature = "icu_segmenter")]
type DataStruct = < :: icu_segmenter :: provider :: DictionaryForWordOnlyAutoV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    icu_provider::DataLocale::from(icu_locid::locale!("ja"))
        .eq(locale)
        .then(|| &JA)
}
static JA: DataStruct = include!("ja.rs.data");
