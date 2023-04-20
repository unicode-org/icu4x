// @generated
#![cfg(feature = "icu_plurals")]
type DataStruct =
    <::icu_plurals::provider::CardinalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 12usize] = [
        "ar", "bn", "en", "es", "fil", "fr", "ja", "ru", "sr", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 12usize] =
        [&AR, &BN, &EN, &ES, &FIL, &FR, &JA, &RU, &SR, &JA, &TR, &JA];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static EN: DataStruct = include!("en.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FIL: DataStruct = include!("fil.rs.data");
static FR: DataStruct = include!("fr.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static SR: DataStruct = include!("sr.rs.data");
static TR: DataStruct = include!("tr.rs.data");
