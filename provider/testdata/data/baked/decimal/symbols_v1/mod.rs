// @generated
#![cfg(feature = "icu_decimal")]
type DataStruct =
    <::icu_decimal::provider::DecimalSymbolsV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 24usize] = [
        "ar",
        "ar-EG",
        "ar-EG-u-nu-latn",
        "ar-u-nu-latn",
        "bn",
        "bn-u-nu-latn",
        "ccp",
        "ccp-u-nu-latn",
        "en",
        "en-001",
        "en-ZA",
        "es",
        "es-AR",
        "fil",
        "fr",
        "ja",
        "ru",
        "sr",
        "sr-Cyrl",
        "sr-Latn",
        "th",
        "th-u-nu-thai",
        "tr",
        "und",
    ];
    static DATA: [&DataStruct; 24usize] = [
        &include!("ar|ar-EG.rs.data"),
        &include!("ar|ar-EG.rs.data"),
        &include!("ar-EG-u-nu-latn|ar-u-nu-latn.rs.data"),
        &include!("ar-EG-u-nu-latn|ar-u-nu-latn.rs.data"),
        &include!("bn.rs.data"),
        &include!("bn-u-nu-latn|ccp-u-nu-latn.rs.data"),
        &include!("ccp.rs.data"),
        &include!("bn-u-nu-latn|ccp-u-nu-latn.rs.data"),
        &include!("en|en-001|fil|ja|th|und.rs.data"),
        &include!("en|en-001|fil|ja|th|und.rs.data"),
        &include!("en-ZA|ru.rs.data"),
        &include!("es.rs.data"),
        &include!("es-AR|sr|sr-Cyrl|sr-Latn|tr.rs.data"),
        &include!("en|en-001|fil|ja|th|und.rs.data"),
        &include!("fr.rs.data"),
        &include!("en|en-001|fil|ja|th|und.rs.data"),
        &include!("en-ZA|ru.rs.data"),
        &include!("es-AR|sr|sr-Cyrl|sr-Latn|tr.rs.data"),
        &include!("es-AR|sr|sr-Cyrl|sr-Latn|tr.rs.data"),
        &include!("es-AR|sr|sr-Cyrl|sr-Latn|tr.rs.data"),
        &include!("en|en-001|fil|ja|th|und.rs.data"),
        &include!("th-u-nu-thai.rs.data"),
        &include!("es-AR|sr|sr-Cyrl|sr-Latn|tr.rs.data"),
        &include!("en|en-001|fil|ja|th|und.rs.data"),
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
