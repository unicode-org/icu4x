// @generated
/// Implement [`DataProvider<BidiControlV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_bidi_c_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_BIDI_C_V1: &<icu_properties::provider::BidiControlV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x1C\x06\0\0\x1D\x06\0\0\x0E \0\0\x10 \0\0* \0\0/ \0\0f \0\0j \0\0") }, 12usize)
            });
            #[doc(hidden)]
            pub fn lookup_props_bidi_c_v1(locale: &icu_provider::DataLocale) -> Result<&'static <icu_properties::provider::BidiControlV1Marker as icu_provider::DataMarker>::Yokeable, icu_provider::DataErrorKind> {
                if locale.is_empty() {
                    Ok(Self::SINGLETON_PROPS_BIDI_C_V1)
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale)
                }
            }
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::BidiControlV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BidiControlV1Marker>, icu_provider::DataError> {
                match Self::lookup_props_bidi_c_v1(&req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::BidiControlV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
