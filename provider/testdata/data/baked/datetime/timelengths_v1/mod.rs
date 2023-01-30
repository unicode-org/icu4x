// @generated
#![cfg(feature = "icu_datetime")]
#![allow(clippy::octal_escapes)]
type DataStruct = < :: icu_datetime :: provider :: calendar :: TimeLengthsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 19usize] = [
        "ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja",
        "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 19usize] = [
        &AR, &AR, &AR, &AR, &EN, &EN, &EN_ZA, &ES, &ES_AR, &EN, &EN_ZA, &JA, &EN_ZA, &EN_ZA,
        &EN_ZA, &EN_ZA, &TH, &TR, &UND,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static EN_ZA: DataStruct = include!("en-ZA.rs.data");
static EN: DataStruct = include!("en.rs.data");
static ES_AR: DataStruct = include!("es-AR.rs.data");
static ES: DataStruct = include!("es.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static TH: DataStruct = include!("th.rs.data");
static TR: DataStruct = include!("tr.rs.data");
static UND: DataStruct = include!("und.rs.data");
