// @generated
type DataStruct =
    <::icu_provider::hello_world::HelloWorldV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 16usize] = [
        "bn", "cs", "de", "el", "en", "eo", "fa", "fi", "is", "ja", "la", "pt", "ro", "ru", "vi",
        "zh",
    ];
    static DATA: [&DataStruct; 16usize] = [
        &BN, &CS, &DE, &EL, &EN, &EO, &FA, &FI, &IS, &JA, &LA, &PT, &RO, &RU, &VI, &ZH,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static BN: DataStruct = include!("bn.rs.data");
static CS: DataStruct = include!("cs.rs.data");
static DE: DataStruct = include!("de.rs.data");
static EL: DataStruct = include!("el.rs.data");
static EN: DataStruct = include!("en.rs.data");
static EO: DataStruct = include!("eo.rs.data");
static FA: DataStruct = include!("fa.rs.data");
static FI: DataStruct = include!("fi.rs.data");
static IS: DataStruct = include!("is.rs.data");
static JA: DataStruct = include!("ja.rs.data");
static LA: DataStruct = include!("la.rs.data");
static PT: DataStruct = include!("pt.rs.data");
static RO: DataStruct = include!("ro.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static VI: DataStruct = include!("vi.rs.data");
static ZH: DataStruct = include!("zh.rs.data");
