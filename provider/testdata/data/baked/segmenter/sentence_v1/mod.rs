// @generated
#![cfg(feature = "icu_segmenter")]
type DataStruct =
    <::icu_segmenter::provider::SentenceBreakDataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static DATA: &DataStruct = &include!("und.rs.data");
    locale.is_empty().then(|| DATA)
}
