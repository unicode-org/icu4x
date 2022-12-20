// @generated
#![cfg(feature = "icu_compactdecimal")]
type DataStruct = < :: icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 24usize] = [
        "ar",
        "ar-EG",
        "ar-EG-u-nu-latn",
        "ar-u-nu-latn",
        "bn",
        "bn-u-nu-latn",
        "ccp",
        "ccp-u-nu-latn",
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
        "th-u-nu-thai",
        "tr",
        "und",
    ];
    static DATA: [&DataStruct; 24usize] = [
        &include!("ar+ar-EG+ar-EG-u-nu-latn+ar-u-nu-latn.rs.data"),
        &include!("ar+ar-EG+ar-EG-u-nu-latn+ar-u-nu-latn.rs.data"),
        &include!("ar+ar-EG+ar-EG-u-nu-latn+ar-u-nu-latn.rs.data"),
        &include!("ar+ar-EG+ar-EG-u-nu-latn+ar-u-nu-latn.rs.data"),
        &include!("bn+bn-u-nu-latn.rs.data"),
        &include!("bn+bn-u-nu-latn.rs.data"),
        &include!("ccp+ccp-u-nu-latn+und.rs.data"),
        &include!("ccp+ccp-u-nu-latn+und.rs.data"),
        &include!("en+en-001+en-ZA.rs.data"),
        &include!("en+en-001+en-ZA.rs.data"),
        &include!("en+en-001+en-ZA.rs.data"),
        &include!("es.rs.data"),
        &include!("es-AR.rs.data"),
        &include!("fil.rs.data"),
        &include!("fr.rs.data"),
        &include!("ja.rs.data"),
        &include!("ru.rs.data"),
        &include!("sr+sr-Cyrl.rs.data"),
        &include!("sr+sr-Cyrl.rs.data"),
        &include!("sr-Latn.rs.data"),
        &include!("th+th-u-nu-thai.rs.data"),
        &include!("th+th-u-nu-thai.rs.data"),
        &include!("tr.rs.data"),
        &include!("ccp+ccp-u-nu-latn+und.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
