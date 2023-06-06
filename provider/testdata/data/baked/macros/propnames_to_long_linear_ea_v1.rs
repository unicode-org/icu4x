// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_to_long_linear_ea_v1 {
    () => {
        icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x07\0\x10\0\x19\0\"\0NeutralAmbiguousHalfwidthFullwidthNarrow") } }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_propnames_to_long_linear_ea_v1 {
    ($ locale : expr) => {{
        static SINGLETON: <icu_properties::provider::EastAsianWidthValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_to_long_linear_ea_v1!();
        if $locale.is_empty() {
            Ok(&SINGLETON)
        } else {
            Err(icu_provider::DataErrorKind::ExtraneousLocale)
        }
    }};
}
/// Implement [`DataProvider<EastAsianWidthValueToLongNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_ea_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::EastAsianWidthValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::EastAsianWidthValueToLongNameV1Marker>, icu_provider::DataError> {
                match lookup_propnames_to_long_linear_ea_v1!(req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::EastAsianWidthValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
