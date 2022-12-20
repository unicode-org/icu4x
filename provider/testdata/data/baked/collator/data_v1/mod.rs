// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationDataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 7usize] = ["bn", "es", "es-u-co-trad", "ja", "th", "tr", "und"];
    static DATA: [&DataStruct; 7usize] = [
        &include!("bn.rs.data"),
        &include!("es.rs.data"),
        &include!("es-u-co-trad.rs.data"),
        &include!("ja.rs.data"),
        &include!("th.rs.data"),
        &include!("tr.rs.data"),
        &include!("und.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
