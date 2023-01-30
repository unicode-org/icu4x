// @generated
#![cfg(feature = "icu_properties")]
#![allow(clippy::octal_escapes)]
type DataStruct = < :: icu_properties :: provider :: ExemplarCharactersNumbersV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 19usize] = [
        "ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja",
        "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 19usize] = [
        &AR, &AR, &BN, &CCP, &CCP, &CCP, &EN_ZA, &CCP, &CCP, &CCP, &FR, &CCP, &EN_ZA, &CCP, &CCP,
        &CCP, &CCP, &CCP, &CCP,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static CCP: DataStruct = include!("ccp.rs.data");
static EN_ZA: DataStruct = include!("en-ZA.rs.data");
static FR: DataStruct = include!("fr.rs.data");
