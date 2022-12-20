// @generated
#![cfg(feature = "icu_plurals")]
type DataStruct =
    <::icu_plurals::provider::CardinalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 12usize] = [
        "ar", "bn", "en", "es", "fil", "fr", "ja", "ru", "sr", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 12usize] = [
        &include!("ar.rs.data"),
        &include!("bn.rs.data"),
        &include!("en.rs.data"),
        &include!("es.rs.data"),
        &include!("fil.rs.data"),
        &include!("fr.rs.data"),
        &include!("ja+th+und.rs.data"),
        &include!("ru.rs.data"),
        &include!("sr.rs.data"),
        &include!("ja+th+und.rs.data"),
        &include!("tr.rs.data"),
        &include!("ja+th+und.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
