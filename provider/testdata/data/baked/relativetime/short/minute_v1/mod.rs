// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: ShortMinuteRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 19usize] = [
        "ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja",
        "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 19usize] = [
        &include!("ar+ar-EG.rs.data"),
        &include!("ar+ar-EG.rs.data"),
        &include!("bn.rs.data"),
        &include!("ccp.rs.data"),
        &include!("en.rs.data"),
        &include!("en-001+en-ZA.rs.data"),
        &include!("en-001+en-ZA.rs.data"),
        &include!("es+es-AR.rs.data"),
        &include!("es+es-AR.rs.data"),
        &include!("fil.rs.data"),
        &include!("fr.rs.data"),
        &include!("ja.rs.data"),
        &include!("ru.rs.data"),
        &include!("sr+sr-Cyrl.rs.data"),
        &include!("sr+sr-Cyrl.rs.data"),
        &include!("sr-Latn.rs.data"),
        &include!("th.rs.data"),
        &include!("tr.rs.data"),
        &include!("und.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
