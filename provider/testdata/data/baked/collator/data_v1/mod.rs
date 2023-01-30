// @generated
#![cfg(feature = "icu_collator")]
#![allow(clippy::octal_escapes)]
type DataStruct =
    <::icu_collator::provider::CollationDataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 7usize] = ["bn", "es", "es-u-co-trad", "ja", "th", "tr", "und"];
    static DATA: [&DataStruct; 7usize] = [&BN, &ES, &ES_U_CO_TRAD, &JA, &TH, &TR, &UND];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static BN: DataStruct = include!("bn.rs.data");
static ES_U_CO_TRAD: DataStruct = include!("es-u-co-trad.rs.data");
static ES: DataStruct = include!("es.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static TH: DataStruct = include!("th.rs.data");
static TR: DataStruct = include!("tr.rs.data");
static UND: DataStruct = include!("und.rs.data");
