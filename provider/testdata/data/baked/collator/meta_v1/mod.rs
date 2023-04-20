// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationMetadataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 17usize] = [
        "ar",
        "ar-u-co-compat",
        "bn",
        "bn-u-co-trad",
        "es",
        "es-u-co-trad",
        "fil",
        "ja",
        "ja-u-co-unihan",
        "ru",
        "sr",
        "sr-Latn",
        "th",
        "tr",
        "und",
        "und-u-co-emoji",
        "und-u-co-eor",
    ];
    static DATA: [&DataStruct; 17usize] = [
        &AR, &AR, &AR, &AR, &ES, &ES, &ES, &AR, &AR, &RU, &AR, &AR, &TH, &ES, &UND, &UND, &UND,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static ES: DataStruct = include!("es.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static TH: DataStruct = include!("th.rs.data");
static UND: DataStruct = include!("und.rs.data");
