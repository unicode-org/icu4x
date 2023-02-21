// @generated
#![allow(clippy::octal_escapes)]
type DataStruct =
    <::icu_provider::hello_world::HelloWorldV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 4usize] = ["bn", "en", "ja", "ru"];
    static DATA: [&DataStruct; 4usize] = [&BN, &EN, &JA, &RU];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static BN: DataStruct = include!("bn.rs.data");
static EN: DataStruct = include!("en.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static RU: DataStruct = include!("ru.rs.data");
