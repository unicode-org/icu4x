// @generated
#![cfg(feature = "icu_compactdecimal")]
#![allow(clippy::octal_escapes)]
type DataStruct = < :: icu_compactdecimal :: provider :: ShortCompactDecimalFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
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
        &JA, &EN, &SR_LATN, &TR, &BN, &EN, &SR, &CCP, &BN, &AR, &EN, &ES, &FR, &AR, &RU, &AR, &EN,
        &SR, &EN, &AR, &CCP, &ES_AR, &EN, &CCP,
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
    let i = perfect_hash_utils::compute_index((f0, f1), (d0 as u32, d1 as u32), 24u32)?;
    locale
        .strict_cmp(KEYS.get(i)?.as_bytes())
        .is_eq()
        .then(|| DATA.get(i).copied())
        .flatten()
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static CCP: DataStruct = include!("ccp.rs.data");
static EN: DataStruct = include!("en.rs.data");
static ES_AR: DataStruct = include!("es-AR.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FR: DataStruct = include!("fr.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static SR_LATN: DataStruct = include!("sr-Latn.rs.data");
static SR: DataStruct = include!("sr.rs.data");
static TR: DataStruct = include!("tr.rs.data");
