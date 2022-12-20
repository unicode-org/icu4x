// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationReorderingV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 3usize] = ["bn", "ja", "th"];
    static DATA: [&DataStruct; 3usize] = [
        &include!("bn.rs.data"),
        &include!("ja.rs.data"),
        &include!("th.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
