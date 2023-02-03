// @generated
#![cfg(feature = "icu_decimal")]
#![allow(clippy::octal_escapes)]
type DataStruct =
    <::icu_decimal::provider::DecimalSymbolsV1Marker as ::icu_provider::DataMarker>::Yokeable;
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
        &AR,
        &AR,
        &AR_EG_U_NU_LATN,
        &AR_EG_U_NU_LATN,
        &BN,
        &BN_U_NU_LATN,
        &CCP,
        &BN_U_NU_LATN,
        &EN,
        &EN,
        &EN_ZA,
        &ES,
        &ES_AR,
        &EN,
        &FR,
        &EN,
        &EN_ZA,
        &ES_AR,
        &ES_AR,
        &ES_AR,
        &EN,
        &TH_U_NU_THAI,
        &ES_AR,
        &EN,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR_EG_U_NU_LATN: DataStruct = include!("ar-EG-u-nu-latn.rs.data");
static AR: DataStruct = include!("ar.rs.data");
static BN_U_NU_LATN: DataStruct = include!("bn-u-nu-latn.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static CCP: DataStruct = include!("ccp.rs.data");
static EN_ZA: DataStruct = include!("en-ZA.rs.data");
static EN: DataStruct = include!("en.rs.data");
static ES_AR: DataStruct = include!("es-AR.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FR: DataStruct = include!("fr.rs.data");
static TH_U_NU_THAI: DataStruct = include!("th-u-nu-thai.rs.data");
