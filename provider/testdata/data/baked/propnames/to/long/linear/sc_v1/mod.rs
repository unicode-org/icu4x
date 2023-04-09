// @generated
#![cfg(feature = "icu_properties")]
#![allow(clippy::octal_escapes)]
type DataStruct = < :: icu_properties :: provider :: ScriptValueToLongNameV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    locale.is_empty().then(|| &UND)
}
static UND: DataStruct = include!("und.rs.data");
