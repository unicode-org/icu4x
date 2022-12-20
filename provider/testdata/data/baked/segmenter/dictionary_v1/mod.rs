// @generated
#![cfg(feature = "icu_segmenter")]
type DataStruct = < :: icu_segmenter :: provider :: UCharDictionaryBreakDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 2usize] = ["ja", "th"];
    static DATA: [&DataStruct; 2usize] = [&include!("ja.rs.data"), &include!("th.rs.data")];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
