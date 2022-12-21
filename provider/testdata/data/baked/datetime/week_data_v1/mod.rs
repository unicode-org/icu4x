// @generated
#![cfg(feature = "icu_calendar")]
type DataStruct =
    <::icu_calendar::provider::WeekDataV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 155usize] = [
        "und", "und-AD", "und-AE", "und-AF", "und-AG", "und-AI", "und-AL", "und-AM", "und-AN",
        "und-AR", "und-AS", "und-AT", "und-AU", "und-AX", "und-AZ", "und-BA", "und-BD", "und-BE",
        "und-BG", "und-BH", "und-BM", "und-BN", "und-BR", "und-BS", "und-BT", "und-BW", "und-BY",
        "und-BZ", "und-CA", "und-CH", "und-CL", "und-CM", "und-CN", "und-CO", "und-CR", "und-CY",
        "und-CZ", "und-DE", "und-DJ", "und-DK", "und-DM", "und-DO", "und-DZ", "und-EC", "und-EE",
        "und-EG", "und-ES", "und-ET", "und-FI", "und-FJ", "und-FO", "und-FR", "und-GB", "und-GE",
        "und-GF", "und-GG", "und-GI", "und-GP", "und-GR", "und-GT", "und-GU", "und-HK", "und-HN",
        "und-HR", "und-HU", "und-ID", "und-IE", "und-IL", "und-IM", "und-IN", "und-IQ", "und-IR",
        "und-IS", "und-IT", "und-JE", "und-JM", "und-JO", "und-JP", "und-KE", "und-KG", "und-KH",
        "und-KR", "und-KW", "und-KZ", "und-LA", "und-LB", "und-LI", "und-LK", "und-LT", "und-LU",
        "und-LV", "und-LY", "und-MC", "und-MD", "und-ME", "und-MH", "und-MK", "und-MM", "und-MN",
        "und-MO", "und-MQ", "und-MT", "und-MV", "und-MX", "und-MY", "und-MZ", "und-NI", "und-NL",
        "und-NO", "und-NP", "und-NZ", "und-OM", "und-PA", "und-PE", "und-PH", "und-PK", "und-PL",
        "und-PR", "und-PT", "und-PY", "und-QA", "und-RE", "und-RO", "und-RS", "und-RU", "und-SA",
        "und-SD", "und-SE", "und-SG", "und-SI", "und-SJ", "und-SK", "und-SM", "und-SV", "und-SY",
        "und-TH", "und-TJ", "und-TM", "und-TR", "und-TT", "und-TW", "und-UA", "und-UM", "und-US",
        "und-UY", "und-UZ", "und-VA", "und-VE", "und-VI", "und-VN", "und-WS", "und-XK", "und-YE",
        "und-ZA", "und-ZW",
    ];
    static DATA: [&DataStruct; 155usize] = [
        &UND, &UND_AD, &UND_AE, &UND_AE, &UND_AG, &UND, &UND, &UND, &UND_AD, &UND, &UND_AG,
        &UND_AD, &UND, &UND_AD, &UND, &UND, &UND_AG, &UND_AD, &UND_AD, &UND_AE, &UND, &UND,
        &UND_AG, &UND_AG, &UND_AG, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AD, &UND, &UND, &UND,
        &UND_AG, &UND, &UND, &UND_AD, &UND_AD, &UND_AE, &UND_AD, &UND_AG, &UND_AG, &UND_AE, &UND,
        &UND_AD, &UND_AE, &UND_AD, &UND_AG, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND,
        &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND_AD, &UND_AG, &UND_AG, &UND_AG, &UND_AG, &UND,
        &UND_AD, &UND_AG, &UND_AD, &UND_AG, &UND_AD, &UND_AG, &UND_AE, &UND_AE, &UND_AD, &UND_AD,
        &UND_AD, &UND_AG, &UND_AE, &UND_AG, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AE, &UND,
        &UND_AG, &UND, &UND_AD, &UND, &UND_AD, &UND_AD, &UND, &UND_AE, &UND_AD, &UND, &UND,
        &UND_AG, &UND, &UND_AG, &UND, &UND_AG, &UND_AD, &UND_AG, &UND_MV, &UND_AG, &UND, &UND_AG,
        &UND_AG, &UND_AD, &UND_AD, &UND_AG, &UND, &UND_AE, &UND_AG, &UND_AG, &UND_AG, &UND_AG,
        &UND_AD, &UND_AG, &UND_PT, &UND_AG, &UND_AE, &UND_AD, &UND, &UND, &UND_AD, &UND_AG,
        &UND_AE, &UND_AD, &UND_AG, &UND, &UND_AD, &UND_AD, &UND_AD, &UND_AG, &UND_AE, &UND_AG,
        &UND, &UND, &UND, &UND_AG, &UND_AG, &UND, &UND_AG, &UND_AG, &UND, &UND, &UND_AD, &UND_AG,
        &UND_AG, &UND, &UND_AG, &UND, &UND_AG, &UND_AG, &UND_AG,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static UND_AD: DataStruct = include!("und-AD.rs.data");
static UND_AE: DataStruct = include!("und-AE.rs.data");
static UND_AG: DataStruct = include!("und-AG.rs.data");
static UND_MV: DataStruct = include!("und-MV.rs.data");
static UND_PT: DataStruct = include!("und-PT.rs.data");
static UND: DataStruct = include!("und.rs.data");
