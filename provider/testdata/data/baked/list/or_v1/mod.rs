// @generated
#![cfg(feature = "icu_list")]
#![allow(clippy::octal_escapes)]
type DataStruct = <::icu_list::provider::OrListV1Marker as ::icu_provider::DataMarker>::Yokeable;
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
        &CCP,
        &CCP,
        &EN_001,
        &EN_001,
        &ES,
        &ES,
        &FIL,
        &FR,
        &JA,
        &RU,
        &RU,
        &RU,
        &SR_LATN,
        &TH,
        &TR,
        &CCP,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static CCP: DataStruct = include!("ccp.rs.data");
static EN_001: DataStruct = include!("en-001.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FIL: DataStruct = include!("fil.rs.data");
static FR: DataStruct = include!("fr.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static SR_LATN: DataStruct = include!("sr-Latn.rs.data");
static TH: DataStruct = include!("th.rs.data");
static TR: DataStruct = include!("tr.rs.data");
