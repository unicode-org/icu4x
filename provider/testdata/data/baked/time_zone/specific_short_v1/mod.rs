// @generated
#![cfg(feature = "icu_datetime")]
type DataStruct = < :: icu_datetime :: provider :: time_zones :: MetazoneSpecificNamesShortV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 19usize] = [
        "ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja",
        "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 19usize] = [
        &include!("ar|ar-EG.rs.data"),
        &include!("ar|ar-EG.rs.data"),
        &include!("bn|ccp.rs.data"),
        &include!("bn|ccp.rs.data"),
        &include!("en.rs.data"),
        &include!("en-001.rs.data"),
        &include!("en-ZA.rs.data"),
        &include!("es|sr|sr-Cyrl|sr-Latn.rs.data"),
        &include!("es-AR.rs.data"),
        &include!("fil|fr|ru|th|tr|und.rs.data"),
        &include!("fil|fr|ru|th|tr|und.rs.data"),
        &include!("ja.rs.data"),
        &include!("fil|fr|ru|th|tr|und.rs.data"),
        &include!("es|sr|sr-Cyrl|sr-Latn.rs.data"),
        &include!("es|sr|sr-Cyrl|sr-Latn.rs.data"),
        &include!("es|sr|sr-Cyrl|sr-Latn.rs.data"),
        &include!("fil|fr|ru|th|tr|und.rs.data"),
        &include!("fil|fr|ru|th|tr|und.rs.data"),
        &include!("fil|fr|ru|th|tr|und.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
