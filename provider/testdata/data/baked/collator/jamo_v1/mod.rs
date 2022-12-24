// @generated
#![cfg(feature = "icu_collator")]
type DataStruct =
    <::icu_collator::provider::CollationJamoV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    locale.is_empty().then(|| &UND)
}
static UND: DataStruct = include!("und.rs.data");
