// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationReorderingV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 10usize] = [
        "ar",
        "ar-u-co-compat",
        "bn",
        "bn-u-co-trad",
        "ja",
        "ja-u-co-unihan",
        "ru",
        "sr",
        "sr-Latn",
        "th",
    ];
    static DATA: [&DataStruct; 10usize] = [&AR, &AR, &BN, &BN, &JA, &JA, &RU, &RU, &SR_LATN, &TH];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static SR_LATN: DataStruct = include!("sr-Latn.rs.data");
static TH: DataStruct = include!("th.rs.data");
