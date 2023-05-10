// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationDataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 16usize] = [
        "ar",
        "ar-u-co-compat",
        "bn",
        "bn-u-co-trad",
        "es",
        "es-u-co-trad",
        "fil",
        "ja",
        "ja-u-co-unihan",
        "sr",
        "sr-Latn",
        "th",
        "tr",
        "und",
        "und-u-co-emoji",
        "und-u-co-eor",
    ];
    static DATA: [&DataStruct; 16usize] = [
        &AR,
        &AR_U_CO_COMPAT,
        &BN,
        &BN_U_CO_TRAD,
        &ES,
        &ES_U_CO_TRAD,
        &FIL,
        &JA,
        &JA_U_CO_UNIHAN,
        &SR,
        &SR_LATN,
        &TH,
        &TR,
        &UND,
        &UND_U_CO_EMOJI,
        &UND_U_CO_EOR,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR_U_CO_COMPAT: DataStruct = include!("ar-u-co-compat.rs.data");
static AR: DataStruct = include!("ar.rs.data");
static BN_U_CO_TRAD: DataStruct = include!("bn-u-co-trad.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static ES_U_CO_TRAD: DataStruct = include!("es-u-co-trad.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FIL: DataStruct = include!("fil.rs.data");
static JA_U_CO_UNIHAN: DataStruct = include!("ja-u-co-unihan.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static SR_LATN: DataStruct = include!("sr-Latn.rs.data");
static SR: DataStruct = include!("sr.rs.data");
static TH: DataStruct = include!("th.rs.data");
static TR: DataStruct = include!("tr.rs.data");
static UND_U_CO_EMOJI: DataStruct = include!("und-u-co-emoji.rs.data");
static UND_U_CO_EOR: DataStruct = include!("und-u-co-eor.rs.data");
static UND: DataStruct = include!("und.rs.data");
