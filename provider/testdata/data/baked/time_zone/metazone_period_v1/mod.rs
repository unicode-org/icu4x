// @generated
#![cfg(feature = "icu_timezone")]
type DataStruct =
    <::icu_timezone::provider::MetazonePeriodV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static DATA: &DataStruct = &include!("und.rs.data");
    locale.is_empty().then(|| DATA)
}
