// @generated
type DataStruct =
    <::icu_provider::hello_world::HelloWorldV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 4usize] = ["bn", "en", "ja", "ru"];
    static DATA: [&DataStruct; 4usize] = [
        &include!("bn.rs.data"),
        &include!("en.rs.data"),
        &include!("ja.rs.data"),
        &include!("ru.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
