// @generated
/// Implement [`DataProvider<HyphenV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_hyphen_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_HYPHEN_V1: &<icu_properties::provider::HyphenV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"-\0\0\0.\0\0\0\xAD\0\0\0\xAE\0\0\0\x8A\x05\0\0\x8B\x05\0\0\x06\x18\0\0\x07\x18\0\0\x10 \0\0\x12 \0\0\x17.\0\0\x18.\0\0\xFB0\0\0\xFC0\0\0c\xFE\0\0d\xFE\0\0\r\xFF\0\0\x0E\xFF\0\0e\xFF\0\0f\xFF\0\0") }, 11usize)
            });
            #[doc(hidden)]
            pub fn lookup_props_hyphen_v1(locale: &icu_provider::DataLocale) -> Result<&'static <icu_properties::provider::HyphenV1Marker as icu_provider::DataMarker>::Yokeable, icu_provider::DataErrorKind> {
                if locale.is_empty() {
                    Ok(Self::SINGLETON_PROPS_HYPHEN_V1)
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale)
                }
            }
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::HyphenV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::HyphenV1Marker>, icu_provider::DataError> {
                match Self::lookup_props_hyphen_v1(&req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::HyphenV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
