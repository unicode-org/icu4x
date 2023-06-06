// @generated
/// Implement [`DataProvider<WordBreakValueToShortNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_linear_wb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_SHORT_LINEAR_WB_V1: &<icu_properties::provider::WordBreakValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x16\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x18\0\x1A\0\x1C\0\x1E\0 \0\"\0$\0&\0(\0+\0-\x000\0XXLEFOKAMLMNNUEXCRExtendLFMBNLRIHLSQDQEBEBGEMGAZZWJ") } };
            #[doc(hidden)]
            pub fn lookup_propnames_to_short_linear_wb_v1(locale: &icu_provider::DataLocale) -> Result<&'static <icu_properties::provider::WordBreakValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable, icu_provider::DataErrorKind> {
                if locale.is_empty() {
                    Ok(Self::SINGLETON_PROPNAMES_TO_SHORT_LINEAR_WB_V1)
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale)
                }
            }
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::WordBreakValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::WordBreakValueToShortNameV1Marker>, icu_provider::DataError> {
                match Self::lookup_propnames_to_short_linear_wb_v1(&req.locale) {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_owned(icu_provider::prelude::zerofrom::ZeroFrom::zero_from(payload))) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::WordBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
