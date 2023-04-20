// @generated
#![cfg(feature = "icu_datetime")]
#![allow(clippy::octal_escapes)]
type DataStruct = <::icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 19usize] = [
        "ar",
        "ar-EG",
        "bn",
        "ccp",
        "en",
        "en-001",
        "en-ZA",
        "es",
        "es-AR",
        "fil",
        "fr",
        "ja",
        "ru",
        "sr",
        "sr-Cyrl",
        "sr-Latn",
        "th",
        "tr",
        "und",
    ];
    static DATA: [&DataStruct; 19usize] = [
        &AR,
        &AR,
        &BN,
        &BN,
        &EN,
        &EN_001,
        &EN_ZA,
        &ES,
        &ES_AR,
        &FIL,
        &FIL,
        &JA,
        &FIL,
        &ES,
        &ES,
        &ES,
        &FIL,
        &FIL,
        &FIL,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static EN_001: DataStruct = include!("en-001.rs.data");
static EN_ZA: DataStruct = include!("en-ZA.rs.data");
static EN: DataStruct = include!("en.rs.data");
static ES_AR: DataStruct = include!("es-AR.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FIL: DataStruct = include!("fil.rs.data");
static JA: DataStruct = include!("ja.rs.data");
