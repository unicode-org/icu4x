// @generated
#![cfg(feature = "icu_decimal")]
#![allow(clippy::octal_escapes)]
type DataStruct =
    <::icu_decimal::provider::DecimalSymbolsV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 24usize] = [
        "ja",
        "en-001",
        "sr-Latn",
        "tr",
        "bn-u-nu-latn",
        "en-ZA",
        "sr-Cyrl",
        "ccp",
        "bn",
        "ar",
        "th-u-nu-thai",
        "es",
        "fr",
        "ar-EG-u-nu-latn",
        "ru",
        "ar-EG",
        "en",
        "sr",
        "th",
        "ar-u-nu-latn",
        "und",
        "es-AR",
        "fil",
        "ccp-u-nu-latn",
    ];
    static DATA: [&DataStruct; 24usize] = [
        &EN,
        &EN,
        &ES_AR,
        &ES_AR,
        &BN_U_NU_LATN,
        &EN_ZA,
        &ES_AR,
        &CCP,
        &BN,
        &AR,
        &TH_U_NU_THAI,
        &ES,
        &FR,
        &AR_EG_U_NU_LATN,
        &EN_ZA,
        &AR,
        &EN,
        &ES_AR,
        &EN,
        &AR_EG_U_NU_LATN,
        &EN,
        &ES_AR,
        &EN,
        &BN_U_NU_LATN,
    ];
    static DISPLACEMENTS: [(u8, u8); 24usize] = [
        (0, 4),
        (0, 0),
        (0, 0),
        (0, 23),
        (0, 6),
        (0, 18),
        (0, 2),
        (0, 0),
        (0, 2),
        (0, 6),
        (0, 1),
        (0, 0),
        (0, 0),
        (0, 5),
        (0, 1),
        (0, 0),
        (0, 3),
        (0, 0),
        (0, 0),
        (0, 1),
        (0, 3),
        (0, 2),
        (0, 4),
        (0, 1),
    ];
    let (g, f0, f1) = perfect_hash_utils::compute_hash(locale, 24usize);
    let (d0, d1) = DISPLACEMENTS.get(g).copied()?;
    let i = perfect_hash_utils::compute_index((f0, f1), (d0 as u32, d1 as u32), 24usize)?;
    locale
        .strict_cmp(KEYS.get(i)?.as_bytes())
        .is_eq()
        .then(|| DATA.get(i).copied())
        .flatten()
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
